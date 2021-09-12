// --------------------------------------------------------------------------------
// NameList

type NameList<'a> = Vec<&'a str>;

// --------------------------------------------------------------------------------
// Keywords

// Misc
pub const DEFVAR: &str = "var";
pub const SETVAR: &str = "=";
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const PI: &str = "pi";
pub const TAU: &str = "tau";
pub const E: &str = "e";

// Binary Ops
pub const ADD: &str = "+";
pub const SUBTRACT: &str = "-";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";
pub const REMAINDER: &str = "%";
pub const POWER: &str = "^";
pub const MAX: &str = "max";
pub const MIN: &str = "min";
pub const EQUAL: &str = "==";
pub const NOT_EQUAL: &str = "!=";
pub const LESS: &str = "<";
pub const LESS_EQUAL: &str = "<=";
pub const GREATER: &str = ">";
pub const GREATER_EQUAL: &str = ">=";
pub const AND: &str = "and";
pub const OR: &str = "or";

// Unary Ops
pub const SQRT: &str = "sqrt";
pub const EXP: &str = "exp";
pub const EXP2: &str = "exp2";
pub const LN: &str = "ln";
pub const LOG2: &str = "log2";
pub const LOG10: &str = "log10";
pub const SIN: &str = "sin";
pub const COS: &str = "cos";
pub const TAN: &str = "tan";
pub const SINH: &str = "sinh";
pub const COSH: &str = "cosh";
pub const TANH: &str = "tanh";
pub const ASIN: &str = "asin";
pub const ACOS: &str = "acos";
pub const ATAN: &str = "atan";
pub const ASINH: &str = "asinh";
pub const ACOSH: &str = "acosh";
pub const ATANH: &str = "atanh";
pub const SIGN: &str = "sign";
pub const ABS: &str = "abs";
pub const RECIP: &str = "recip";
pub const FRACT: &str = "fract";
pub const TRUNC: &str = "trunc";
pub const CEIL: &str = "ceil";
pub const FLOOR: &str = "floor";
pub const ROUND: &str = "round";
pub const NEG: &str = "neg";
pub const NOT: &str = "not";

// --------------------------------------------------------------------------------
// Keyword Functions

#[inline(always)]
pub fn binary_ops() -> NameList<'static> {
    vec![ADD, SUBTRACT, MULTIPLY, DIVIDE, REMAINDER, POWER,
         MAX, MIN,
         EQUAL, NOT_EQUAL, LESS, LESS_EQUAL, GREATER, GREATER_EQUAL,
         AND, OR]
}

#[inline(always)]
pub fn unary_ops() -> NameList<'static> {
    vec![SQRT, EXP, EXP2, LN, LOG2, LOG10,
         SIN, COS, TAN, SINH, COSH, TANH,
         ASIN, ACOS, ATAN, ASINH, ACOSH, ATANH,
         SIGN, ABS, RECIP, FRACT, TRUNC,
         CEIL, FLOOR, ROUND,
         NEG, NOT]
}

#[inline(always)]
pub fn constants() -> NameList<'static> {
    vec![PI, TAU, E]
}
