# Plugin: Numeric

Numeric operations

## Components

| Name              | Property | Data Type | Socket Type |
|-------------------|----------|-----------|-------------|
|                   |
| numeric_operation | lhs      | number    | input       |
|                   | result   | number    | output      |
|                   |
| numeric_gate      | lhs      | number    | input       |
|                   | rhs      | number    | input       |
|                   | result   | number    | output      |

## Entity Types

| Name       | Components        | Description                                                                                    |
|------------|-------------------|------------------------------------------------------------------------------------------------|
| abs        | numeric_operation | Computes the absolute value                                                                    |
| acos       | numeric_operation | Computes the arccosine of a number                                                             |
| acosh      | numeric_operation | Inverse hyperbolic cosine function                                                             |
| asin       | numeric_operation | Computes the arcsine of a number                                                               |
| asinh      | numeric_operation | Inverse hyperbolic sine function                                                               |
| atan       | numeric_operation | Computes the arctangent of a number                                                            |
| atan2      | numeric_gate      | Computes the four quadrant arctangent in radians                                               |
| atanh      | numeric_operation | Inverse hyperbolic tangent function                                                            |
| cbrt       | numeric_operation | Returns the cube root of a number                                                              |
| ceil       | numeric_operation | Returns the smallest integer greater than or equal to a number                                 |
| cos        | numeric_operation | Computes the cosine of a number (in radians)                                                   |
| cosh       | numeric_operation | Hyperbolic cosine function                                                                     |
| exp        | numeric_operation | Returns e^(input), (the exponential function)                                                  |
| exp2       | numeric_operation | Returns 2^(input)                                                                              |
| floor      | numeric_operation | Returns the largest integer less than or equal to a number                                     |
| fract      | numeric_operation | Returns the fractional part of a number                                                        |
| hypot      | numeric_gate      | Calculates the length of the hypotenuse of a right-angle triangle given legs of length x and y |
| ln         | numeric_operation | Returns the natural logarithm of the number                                                    |
| log        | numeric_gate      | Returns the logarithm of the number with respect to an arbitrary base                          |
| log2       | numeric_operation | Returns the base 2 logarithm of the number                                                     |
| log10      | numeric_operation | Returns the base 10 logarithm of the number                                                    |
| pow        | numeric_gate      | Raises a number to a power                                                                     |
| recip      | numeric_operation | Takes the reciprocal (inverse) of a number, 1/x                                                |
| round      | numeric_operation | Returns the nearest integer to a number. Round half-way cases away from 0.0                    |
| signum     | numeric_operation | Returns a number that represents the sign of the input                                         |
| sin        | numeric_operation | Computes the sine of a number (in radians)                                                     |
| sinh       | numeric_operation | Hyperbolic sine function                                                                       |
| sqrt       | numeric_operation | Returns the square root of a number                                                            |
| tan        | numeric_operation | Computes the tangent of a number (in radians)                                                  |
| tanh       | numeric_operation | Hyperbolic tangent function                                                                    |
| to_degrees | numeric_operation | Converts radians to degrees                                                                    |
| to_radians | numeric_operation | Converts degrees to radians                                                                    |
| trunc      | numeric_operation | Returns the integer part of a number                                                           |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                      | Repository                                                                                                                                           |
|---------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-numeric | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/numeric](https://github.com/reactive-graph/plugins-core/tree/main/plugins/numeric) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/numeric/tabs.json") }}
