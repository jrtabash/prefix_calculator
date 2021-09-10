# prefix_calculator
Command line prefix calculator. Supported features:
- Numeric (e.g. 10.5) and boolean (e.g. true, false) values
- Constants:
  ```
  pi, tau, e
  ```
- User variables
  ```
  define: var <name> <init>
  set: = <name> <value>
  get: <name>
  ```
- Binary operations
  ```
  +, -, *, /, %, ^,
  max, min,
  ==, !=, <, <=, >, >=,
  and, or
  ```
- Unary operations
  ```
  sqrt, exp, exp2, ln, log2, log10,
  sin, cos, tan, sinh, cosh, tanh,
  asin, acos, atan, asinh, acosh, atanh,
  sign, abs, recip, fract, trunc,
  ceil, floor, round,
  neg, not
  ```
- Example Inputs
  ```
  var x 5
  + x 10
  * 2 + x 20
  sqrt + ^ 3 2 ^ 4 2
  max 1 2
  ```
- Example Corresponding Outputs
  ```
  5
  15
  50
  5
  2
  ```
