# prefix_calculator
Work in Progress ... Command line prefix calculator, supporting:
- Numeric (e.g. 10.5) and boolean (e.g. true, false) values
- Constants:
  ```
  pi, tau, e
  ```
- User variables (define, set, get)
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
- Example Input
  ```
  var x 5
  + x 10
  * 2 + x 20
  ```
- Example Output
  ```
  5
  15
  50
  ```
