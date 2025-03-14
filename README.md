# [Dalbit](https://crates.io/crates/dalbit)

<a href="https://discord.gg/ATVVsNNv3u"><img alt="Discord" src="https://img.shields.io/discord/385151591524597761?style=plastic&logo=discord&color=%235865F2" /></a>

Dalbit(달빛) is a Luau-to-Lua transpiler, designed specifically for `Lua 5.3`.

## Installation

### Using [pesde](https://pesde.dev/packages/caveful_games/dalbit)
To manage/install with pesde manifest:
```sh
pesde add caveful_games/dalbit --dev --target lune
pesde install
```
To run directly from anywhere:
```sh
pesde x caveful_games/dalbit
```

### [From Releases](https://github.com/CavefulGames/dalbit/releases)

### Using Cargo (build from source)
```sh
cargo install dalbit --locked
```

## Usage
You can print more options and informations by running `dalbit help [command]`

### `init`
Initializes dalbit manifest file in the current path.
```sh
dalbit init [path]
```

### `fetch`
Fetches and updates lua polyfills.
* This polyfill can be found [here](https://github.com/CavefulGames/dalbit-polyfill).
```sh
dalbit fetch --config <config-path>
```

### `transpile`
Transpiles luau code to lua code.
```sh
dalbit transpile --config <config-path>
```

### `clean`
Cleans polyfill caches from disk.
```sh
dalbit clean
```

## Example
### `dalbit.toml`
```toml
input = "input.luau"
output = "output.lua"
file_extension = "lua"
target_version = "lua53"
minify = true

[modifiers]

[polyfill]
repository = "https://github.com/CavefulGames/dalbit-polyfill"
injection_path = "__polyfill__"

```

### `inputs/input.luau`
```luau
local obj = { items = {1, 4, 9} }
setmetatable(obj, { __iter = function(o) return next, o.items end })

for k, v in obj do
    print(k * k)
end

```

### `outputs/output.luau`
```lua
local setmetatable=require'./__polyfill__'.setmetatable local __DALBIT_getmetatable_iter=require'./__polyfill__'.__DALBIT_getmetatable_iter local type=require'./__polyfill__'.type local next=require'./__polyfill__'.next local io=nil local module=nil local package=nil local dofile=nil local loadfile=nil local load=nil local obj={items={1,4,9}}
setmetatable(obj,{__iter=function(o)return next,o.items end})do local _DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c, _DALBIT_REMOVE_GENERALIZED_ITERATION_invare234e8bef135bb4c, _DALBIT_REMOVE_GENERALIZED_ITERATION_controle234e8bef135bb4c=

obj if type(_DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c)=='table'then local m=__DALBIT_getmetatable_iter(_DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c)if type(m)=='table'and type(m.__iter)=='function'then _DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c, _DALBIT_REMOVE_GENERALIZED_ITERATION_invare234e8bef135bb4c, _DALBIT_REMOVE_GENERALIZED_ITERATION_controle234e8bef135bb4c=m.__iter(_DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c)else _DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c, _DALBIT_REMOVE_GENERALIZED_ITERATION_invare234e8bef135bb4c, _DALBIT_REMOVE_GENERALIZED_ITERATION_controle234e8bef135bb4c=next, _DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c end end for k,v in _DALBIT_REMOVE_GENERALIZED_ITERATION_itere234e8bef135bb4c,_DALBIT_REMOVE_GENERALIZED_ITERATION_invare234e8bef135bb4c,_DALBIT_REMOVE_GENERALIZED_ITERATION_controle234e8bef135bb4c do
print(k*k)
end end
```

## How does it work?
- Dalbit utilizes darklua and full-moon to transform lua scripts and injects polyfills.

## Real-world use cases
- [Kaledis](https://github.com/orpos/kaledis) - A tool that enables Luau to work with Love2D, simplifying project management, transpiling, and configuration.
- [overblox](https://github.com/pesde-pkg/tooling/tree/main/toolchainlib) - A tool that can transpile Roblox scripts to OVERDARE scripts using Dalbit.

## Contributions
Any issues, advices, and PRs for contribution are welcome!

## Special Thanks
- [seaofvoices/darklua](https://github.com/seaofvoices/darklua) - Providing important and cool lua mutating rules.
- [Kampfkarren/full-moon](https://github.com/Kampfkarren/full-moon) - A lossless Lua parser.

## Trivia
The name of this project, Dalbit, translates to "moonshine" in Korean.
