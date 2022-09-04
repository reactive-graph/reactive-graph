# Plugin: String

## Components

| Name             | Property | Data Type | Socket Type |
|------------------|----------|-----------|-------------|
| StringOperation  | lhs      | string    | input       |
|                  | result   | string    | output      |
| StringGate       | lhs      | string    | input       |
|                  | rhs      | string    | input       |
|                  | result   | string    | output      |
| StringComparison | lhs      | string    | input       |
|                  | rhs      | string    | input       |
|                  | result   | bool      | output      |

## Entity Types / Behaviours

| Name       | Component        | Description                                             |
|------------|------------------|---------------------------------------------------------|
| Trim       | StringOperation  | Removes whitespace at the beginning and end of a string |
| TrimStart  | StringOperation  | Removes whitespace at the beginning of a string         |
| TrimEnd    | StringOperation  | Removes whitespace at the end of a string               |
| Uppercase  | StringOperation  |                                                         |
| Lowercase  | StringOperation  |                                                         |
| StartsWith | StringComparison |                                                         |
| EndsWith   | StringComparison |                                                         |
| Contains   | StringComparison |                                                         |
| ...        |                  |                                                         |
| Split      |                  | lhs (str), rhs (str) -> result (array of str)           |
| Join       |                  | lhs (arr of str) -> result (str)                        |
| Replace    |                  | lhs (str), search (str), replace (str) -> result (str)  |
| Chars      |                  | lhs (str) -> result (array of str)                      |
| Len        |                  | lhs (str) -> result (i64)                               |
| Lines      |                  | lhs (str) -> result (array of str)                      |

## Rust Crate / Rust Reference

* https://doc.rust-lang.org/std/string/struct.String.html


## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                     | Repository                                                                                                       |
|--------------------------|------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-string | [https://github.com/inexorgame/inexor-rgf-plugin-string](https://github.com/inexorgame/inexor-rgf-plugin-string) |
