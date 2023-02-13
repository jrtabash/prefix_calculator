# prefix_calculator
Command line prefix calculator.

## Usage
```bash
USAGE:
    prefix_calculator [FLAGS] [OPTIONS]

FLAGS:
    -b, --batch      Enable batch mode
    -i, --int        Force interactive mode. Use with -e/--expr option to force interactive mode
    -h, --help       Prints help information
    -q, --quiet      Disable startup message
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
- Special functions
  ```
  print
  ```
- REPL

  Commands
  ```
  :quit - Exit calculator
  ctrl-d - Exit calculator

  :env - Show calculator environment
  :reset - Reset calculator environment
  :batch - Toggle batch mode
  :last - Show last value
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
- Example 1
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
- Example 2
  ```
  > :batch
  batch mode on
  > var x 3
  > var y 4
  > sqrt + ^ x 2 ^ y 2
  > :last
  5
  >
  ```
