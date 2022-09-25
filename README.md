# prefix_calculator
Command line prefix calculator.

## Usage
```bash
USAGE:
    prefix_calculator [FLAGS] [OPTIONS]

FLAGS:
    -i, --int        Force interactive mode. Use with -e/--expr option to force interactive mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --expr <expr>    Evaluate expression. Use -i/--int to force interactive mode. Use semicolon ; to separate
                         multiple expressions.
```

## Supported features
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
  Note
  ```
  Variable names must start with an alpha character,
  and must contain only alphanumeric and underscore characters
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
  neg, not,
  asnum, asbool
  ```
- REPL

  Commands
  ```
  :quit - Exit calculator
  ctrl-d - Exit calculator

  :env - Show calculator environment
  :reset - Reset calculator environment
  :help - Print list of available operators and constants
  ```
  Variables
  ```
  last - stores result of last calculation
  ```
  Note
  ```
  Use semicolon ; to separate multiple expressions on the same line
  ```
- Example
  ```
  > var x 5
  5
  > + x 10
  15
  > * 2 + x 20
  50
  > sqrt + ^ 3 2 ^ 4 2
  5
  > / * 3.5 pi 2
  5.497787143782138
  > max x last
  5.497787143782138
  > and asbool 5 true
  true
  > + 5 asnum true
  6
  > :quit
  ```
