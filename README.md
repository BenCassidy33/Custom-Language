## Language Design:

A custom version of python. Staticly Typed Python/Lua

Variables: `x: type = value`
Functions: `fun name(arg: type?, ...): return_type? do ... end`
Custom Types(enums, structs, ...):

```
    let x: int = 5
    // type aliases
    type typealias = type

    // structures
    type typename = struct {
        field1: type1,
        field2: type2,
        field3: struct {
            ...
        },
        field4: enum {
            ...
        }
    }

    // enums, typevalue is optional
    type typename = enum : typevalue {
        field1 = 0,
        field2 = 1,
        ...
    }

    type typename = union {
        Int,
        String
    }
```

Modules and Imports: `from module_name import { function1, function2, ... }`
Maps and Interfaces:

```
    //type: 'map[Any: Any]'
    let map = { field: value }

    let map: Map[String: Int] = { "age": 25 }

    // interfaces are types.
    type name = interface {
        field: type,

        // functions are types!
        field_fun = fun(type1, type2): return_type
    }

    //WARNING: DuckTypes are checked at runtime, Not compile time! Use 'Interface' for static type checking. DuckTypes are commonly used for maps where the type of a variable may not be entirely static and can have more fields than required but not less fields.
    type name = ducktype {
        ... // same syntax as interfaces
    }
```

Example of ducktype:

```
// Note: Int|String will get converted to an "anonmyous" union at compile time.
let system_info: Map[String: Int|Float|String] = {}

type required_info = ducktype {
    host_name: String,
    os_name: String,
    os_version: Float,
}

// will error with traits because the fields of the map may change during runtime! Essentially, traits can only be used on types whose fields are declared at compile type (like structs) but not whose can change during runtime (like maps).
fun print_required_info(info: required_info) do
    ...
end
```
