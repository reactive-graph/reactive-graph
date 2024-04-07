# Plugin: String

## Components

| Name                      | Property | Data Type | Socket Type |
|---------------------------|----------|-----------|-------------|
|                           |
| string_bool_operation     | lhs      | string    | input       |
|                           | result   | bool      | output      |
|                           |
| string_comparison         | lhs      | string    | input       |
|                           | rhs      | string    | input       |
|                           | result   | bool      | output      |
|                           |
| string_gate               | lhs      | string    | input       |
|                           | rhs      | string    | input       |
|                           | result   | string    | output      |
|                           |
| string_number_operation   | lhs      | string    | input       |
|                           | result   | number    | output      |
|                           |
| string_operation          | lhs      | string    | input       |
|                           | result   | string    | output      |
|                           |
| string_string_number_gate | lhs      | string    | input       |
|                           | rhs      | string    | input       |
|                           | result   | number    | output      |

## Entity Types / Behaviours

| Name                 | Component                 | Description                                                                                                  |
|----------------------|---------------------------|--------------------------------------------------------------------------------------------------------------|
| camel_case           | string_operation          | Converts the input to camel case                                                                             |
| capitalize           | string_operation          | Converts the first character of the input to upper case and convert the rest of the input to lower case      |
| char_count           | string_number_operation   | Counts the characters                                                                                        |
| char_count_graphemes | string_number_operation   | Counts the graphemes in the input string taking care of surrogate pairs and combining marks                  |
| chop_after           | string_operation          | Returns everything after the given search                                                                    |
| chop_after_last      | string_operation          | Returns everything after the last given search                                                               |
| chop_before          | string_operation          | Returns everything before the given search                                                                   |
| chop_before_last     | string_operation          | Returns everything before the last given search                                                              |
| chop_remove_prefix   | string_operation          | Extracts the prefix from the input                                                                           |
| chop_remove_suffix   | string_operation          | Extracts the suffix from the input                                                                           |
| concat               | string_gate               | Concatenate lhs with rhs                                                                                     |
| contains             | string_comparison         | Returns true, if lhs contains rhs                                                                            |
| count_substrings     | string_string_number_gate | Counts the number of substring appearances in the input string                                               |
| count_unique_words   | string_string_number_gate | Counting occurrences of unique words in the input string. This function respects unicode                     |
| count_words          | string_string_number_gate | Counts the number of words in the input string                                                               |
| decapitalize         | string_operation          | Converts the first character of the input to lower case and convert the rest of the input to lower case      |
| ends_with            | string_comparison         | Returns true, if lhs ends with rhs                                                                           |
| escape_html          | string_operation          | Escapes HTML special characters                                                                              |
| escape_regexp        | string_operation          | Escapes the regular expression special characters                                                            |
| is_alpha             | string_bool_operation     | Checks whether the input string contains only alpha characters                                               |
| is_alpha_digit       | string_bool_operation     | Checks whether the input string contains contains only alpha and digit characters                            |
| is_blank             | string_bool_operation     | Checks whether the input string is empty or contains only whitespaces                                        |
| is_camel_case        | string_bool_operation     | Checks whether the input string is camelCased                                                                |
| is_capitalize        | string_bool_operation     | Checks whether the input string is capitalized and the rest of the input string is lower case                |
| is_decapitalize      | string_bool_operation     | Checks whether the input string is decapitalized and the rest of the input string is converted to lower case |
| is_digit             | string_bool_operation     | Checks whether the input string contains only digit characters                                               |
| is_empty             | string_bool_operation     | Checks whether the input string is empty                                                                     |
| is_kebab_case        | string_bool_operation     | Checks whether the input string is kebab-cased                                                               |
| is_lower_first       | string_bool_operation     | Checks whether the input string has the first character in lower case                                        |
| is_lowercase         | string_bool_operation     | Checks whether the input string has only lower case characters                                               |
| is_numeric           | string_bool_operation     | Checks whether the input string is numeric                                                                   |
| is_pascal_case       | string_bool_operation     | Checks whether the input string is PascalCased                                                               |
| is_shouty_kebab_case | string_bool_operation     | Checks whether the input string is SHOUTY-KEBAB-CASED                                                        |
| is_shouty_snake_case | string_bool_operation     | Checks whether the input string is SHOUTY_SNAKE_CASED                                                        |
| is_snake_case        | string_bool_operation     | Checks whether the input string is snake_cased                                                               |
| is_train_case        | string_bool_operation     | Checks whether the input string is Train-Cased                                                               |
| is_title_case        | string_bool_operation     | Checks whether the input string is a titlecased string and there is at least one character                   |
| is_upper_first       | string_bool_operation     | Checks whether the input string has the first character in upper case                                        |
| is_uppercase         | string_bool_operation     | Checks whether the input string has only upper case characters                                               |
| kebab_case           | string_operation          | Converts the input to kebab case                                                                             |
| lower_first          | string_operation          | Converts the first character of the input to lower case                                                      |
| lowercase            | string_operation          | Converts the input to lower case                                                                             |
| pascal_case          | string_operation          | Converts the input to pascal case                                                                            |
| shouty_kebab_case    | string_operation          | Converts the input to SHOUTY kebab case                                                                      |
| shouty_snake_case    | string_operation          | Converts the input to SHOUTY snake case                                                                      |
| snake_case           | string_operation          | Converts the input to snake case                                                                             |
| starts_with          | string_comparison         | Returns true, if lhs starts with rhs                                                                         |
| string_length        | string_number_operation   | Returns the length of the input string                                                                       |
| strip_html_tags      | string_operation          | Strips all HTML tags                                                                                         |
| swap_case            | string_operation          | Converts the input to swap case                                                                              |
| templating           |                           | Renders a template                                                                                           |
| title_case           | string_operation          | Converts the input to title case                                                                             |
| train_case           | string_operation          | Converts the input to train case                                                                             |
| trim                 | string_operation          | Removes whitespace at the beginning and end of a string                                                      |
| trim_end             | string_operation          | Removes whitespace at the end of a string                                                                    |
| trim_start           | string_operation          | Removes whitespace at the beginning of a string                                                              |
| unescape_html        | string_operation          | Unescapes HTML special characters                                                                            |
| upper_first          | string_operation          | Converts the first character of the input to upper case                                                      |
| uppercase            | string_operation          | Converts the input to upper case                                                                             |

## Rust Crate / Rust Reference

* https://doc.rust-lang.org/std/string/struct.String.html
* https://docs.rs/voca_rs/latest/voca_rs/index.html

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                     | Repository                                                                                                                                         |
|--------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-string | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/string](https://github.com/reactive-graph/plugins-core/tree/main/plugins/string) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/string/concat/tabs.json") }}

{{ graphql_playground(config="/examples/graphql/plugins/string/pascal_case/tabs.json") }}

{{ graphql_playground(config="/examples/graphql/plugins/string/trim/tabs.json") }}
