use darklua_core::nodes::{
    AssignStatement, BinaryExpression, BinaryOperator, Block, DoStatement, Expression,
    FieldExpression, FunctionCall, Identifier, IfBranch, IfStatement, LocalAssignStatement, Prefix,
    Statement, StringExpression, TupleArguments, TypedIdentifier, Variable,
};
use darklua_core::process::{DefaultVisitor, NodeProcessor, NodeVisitor};
use darklua_core::rules::{Context, RuleConfiguration, RuleConfigurationError, RuleProperties};

use super::runtime_identifier::RuntimeIdentifierBuilder;
use darklua_core::rules::{Rule, RuleProcessResult};

const METATABLE_VARIABLE_NAME: &str = "m";
const GETMETATABLE_IDENTIFIER: &str = "__DALBIT_getmetatable_iter";

struct Processor {
    iterator_identifier: String,
    invariant_identifier: String,
    control_identifier: String,
}

fn get_type_condition(arg: Expression, type_name: &str) -> Box<BinaryExpression> {
    let type_call = Box::new(FunctionCall::new(
        Prefix::from_name("type"),
        TupleArguments::new(vec![arg]).into(),
        None,
    ));
    Box::new(BinaryExpression::new(
        BinaryOperator::Equal,
        Expression::Call(type_call),
        Expression::String(StringExpression::from_value(type_name)),
    ))
}

impl NodeProcessor for Processor {
    fn process_block(&mut self, block: &mut Block) {
        // Iterate through the statements in the block
        let mut result: Vec<(usize, Statement)> = Vec::new();
        for (i, stmt) in block.iter_mut_statements().enumerate() {
            if let Statement::GenericFor(generic_for) = stmt {
                let exps = generic_for.mutate_expressions();
                if exps.len() == 1 {
                    let mut stmts: Vec<Statement> = Vec::new();
                    let iterator_typed_identifier =
                        TypedIdentifier::new(self.iterator_identifier.as_str());
                    let iterator_identifier = iterator_typed_identifier.get_identifier().clone();

                    let invariant_typed_identifier =
                        TypedIdentifier::new(self.invariant_identifier.as_str());
                    let invariant_identifier = invariant_typed_identifier.get_identifier().clone();

                    let control_typed_identifier =
                        TypedIdentifier::new(self.control_identifier.as_str());
                    let control_identifier = control_typed_identifier.get_identifier().clone();

                    let iter_invar_control_local_assign = LocalAssignStatement::new(
                        vec![
                            iterator_typed_identifier,
                            invariant_typed_identifier,
                            control_typed_identifier,
                        ],
                        vec![exps[0].to_owned()],
                    );

                    let iterator_exp = Expression::Identifier(iterator_identifier.clone());
                    exps[0] = iterator_exp.clone();
                    let invariant_exp = Expression::Identifier(invariant_identifier.clone());
                    exps.push(invariant_exp);
                    let control_exp = Expression::Identifier(control_identifier.clone());
                    exps.push(control_exp);

                    let if_table_condition = get_type_condition(iterator_exp.clone(), "table");

                    let mt_typed_identifier = TypedIdentifier::new(METATABLE_VARIABLE_NAME);
                    let mt_identifier = mt_typed_identifier.get_identifier().clone();

                    let get_mt_call = FunctionCall::new(
                        Prefix::from_name(GETMETATABLE_IDENTIFIER),
                        TupleArguments::new(vec![iterator_exp.clone()]).into(),
                        None,
                    );
                    let mt_local_assign = LocalAssignStatement::new(
                        vec![mt_typed_identifier],
                        vec![get_mt_call.into()],
                    );

                    let if_mt_table_condition =
                        get_type_condition(mt_identifier.clone().into(), "table");
                    let mt_iter = FieldExpression::new(
                        Prefix::Identifier(mt_identifier.clone()),
                        Identifier::new("__iter"),
                    );
                    let if_mt_iter_function_condition =
                        get_type_condition(mt_iter.clone().into(), "function");

                    let mut mt_iter_call = FunctionCall::from_prefix(Box::new(mt_iter));
                    mt_iter_call = mt_iter_call
                        .with_argument(Expression::identifier(iterator_identifier.clone()));

                    let assign_from_iter = AssignStatement::new(
                        vec![
                            Variable::Identifier(iterator_identifier.clone()),
                            Variable::Identifier(invariant_identifier.clone()),
                            Variable::Identifier(control_identifier.clone()),
                        ],
                        vec![mt_iter_call.into()],
                    );

                    let assign_from_pairs = AssignStatement::new(
                        vec![
                            Variable::Identifier(iterator_identifier.clone()),
                            Variable::Identifier(invariant_identifier),
                            Variable::Identifier(control_identifier),
                        ],
                        vec![Identifier::new("next").into(), iterator_identifier.into()],
                    );

                    let if_mt_table_block = Block::new(vec![assign_from_iter.into()], None);
                    let if_not_mt_table_block = Block::new(vec![assign_from_pairs.into()], None);
                    let if_mt_table_branch = IfBranch::new(
                        Expression::Binary(Box::new(BinaryExpression::new(
                            BinaryOperator::And,
                            Expression::Binary(if_mt_table_condition),
                            Expression::Binary(if_mt_iter_function_condition),
                        ))),
                        if_mt_table_block,
                    );
                    let if_mt_table_stmt =
                        IfStatement::new(vec![if_mt_table_branch], Some(if_not_mt_table_block));

                    let if_table_block =
                        Block::new(vec![mt_local_assign.into(), if_mt_table_stmt.into()], None);
                    let if_table_branch =
                        IfBranch::new(Expression::Binary(if_table_condition), if_table_block);
                    let if_table_stmt = IfStatement::new(vec![if_table_branch], None);

                    stmts.push(iter_invar_control_local_assign.into());
                    stmts.push(if_table_stmt.into());
                    stmts.push(generic_for.clone().into());

                    result.push((i, DoStatement::new(Block::new(stmts, None)).into()))
                }
            }
        }

        for (i, stmt) in result {
            block.remove_statement(i);
            block.insert_statement(i, stmt);
        }
    }
}

pub const REMOVE_GENERALIZED_ITERATION_MODIFIER_NAME: &str = "remove_generalized_iteration";

/// A rule that removes generalized iteration.
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveGeneralizedIteration {
    runtime_identifier_format: String,
}

impl Default for RemoveGeneralizedIteration {
    fn default() -> Self {
        Self {
            runtime_identifier_format: "_DALBIT_REMOVE_GENERALIZED_ITERATION_{name}{hash}"
                .to_string(),
        }
    }
}

impl Rule for RemoveGeneralizedIteration {
    fn process(&self, block: &mut Block, _: &Context) -> RuleProcessResult {
        let var_builder = RuntimeIdentifierBuilder::new(
            self.runtime_identifier_format.as_str(),
            format!("{block:?}").as_bytes(),
            Some(vec![METATABLE_VARIABLE_NAME.to_string()]),
        )?;
        let mut processor = Processor {
            iterator_identifier: var_builder.build("iter")?,
            invariant_identifier: var_builder.build("invar")?,
            control_identifier: var_builder.build("control")?,
        };
        DefaultVisitor::visit_block(block, &mut processor);
        Ok(())
    }
}

impl RuleConfiguration for RemoveGeneralizedIteration {
    fn configure(&mut self, _: RuleProperties) -> Result<(), RuleConfigurationError> {
        Ok(())
    }

    fn get_name(&self) -> &'static str {
        REMOVE_GENERALIZED_ITERATION_MODIFIER_NAME
    }

    fn serialize_to_properties(&self) -> RuleProperties {
        RuleProperties::new()
    }
}
