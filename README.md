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
    -e, --expr <expr>    Evaluate expression. Use -i/--int to force interactive mode.
                         Use semicolon ; to separate multiple expressions.
                         Evaluated after -f/--file expression file
    -f, --file <file>    Evaluate expression file. Use -i/--int to force interactive mode.
                         Can use semicolon ; to separate multiple expressions on a single line.
                         Evaluated before -e/--expr expressions
```

## Supported features
- Numeric (e.g. 10.5) and boolean (e.g. true, false) values
- Constants:
  ```
  pi, tau, e, phi
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
- Control Flow
  ```
  If: if <condition> ? <true_code> fi
  If/Else: if <condition> ? <true_code> : <false_code> fi
  ```
- User Defined Functions
  ```
  define: def <name> <params> begin <body> end
  call: call <name> <args> cend
  ```
  Note
  ```
  Function names must start with an alpha character,
  and must contain only alphanumeric and underscore characters
  ```
- Special functions
  ```
  xprint - Execute and print expression
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
  :examples - Print examples
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
  > var z sqrt + ^ x 2 ^ y 2
  > xprint z
  5
  > :last
  5
  >
  ```
- Example 3
  ```
  > def f2c f
  >>> begin
  >>> / * - f 32 5 9
  >>> end
  true
  > 
  > call f2c 54 cend
  12.222222222222221
  > 
  > def temp_below f c
  >>> begin
  >>> < call f2c f cend c
  >>> end
  true
  > 
  > call temp_below 54 0 cend
  false
  > call temp_below 30 0 cend
  true
  > 
  ```
- Example 4
  ```
  > def dist x1 y1 x2 y2 
  >>> begin
  >>> var dx2 ^ - x2 x1 2
  >>> var dy2 ^ - y2 y1 2
  >>> sqrt + dx2 dy2
  >>> end
  true
  > 
  > call dist 3 4 6 8 cend
  5
  > 
  > def near x1 y1 x2 y2 begin < call dist x1 y1 x2 y2 cend 1.0 end
  true
  > 
  > call near 3 4 4 5 cend
  false
  > call near 3 4 3.5 4.5 cend
  true
  > 
  ```
- Example 5
  ```
  > var x 5
  5
  > var y 10
  10
  > if <= x 5 ? = x + x 1 : = y + y 1 fi
  6
  > x
  6
  > if > x 10 ? = x + x 1 : = y + y 1 fi
  11
  > y
  11
  > if < x 10 ? x fi
  6
  > if > x 10 ? x fi
  false
  >
  ```
