use crate::pcalc_value::{Value, ValueResult};
use crate::pcalc_keywords as keywords;

#[inline(always)]
pub fn square_root(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.sqrt()))
}

#[inline(always)]
pub fn exponential(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.exp()))
}

#[inline(always)]
pub fn exponential2(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.exp2()))
}

#[inline(always)]
pub fn natural_logarithm(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.ln()))
}

#[inline(always)]
pub fn logarithm2(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.log2()))
}

#[inline(always)]
pub fn logarithm10(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.log10()))
}

#[inline(always)]
pub fn trig_sin(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.sin()))
}

#[inline(always)]
pub fn trig_cos(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.cos()))
}

#[inline(always)]
pub fn trig_tan(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.tan()))
}

#[inline(always)]
pub fn trig_sinh(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.sinh()))
}

#[inline(always)]
pub fn trig_cosh(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.cosh()))
}

#[inline(always)]
pub fn trig_tanh(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.tanh()))
}

#[inline(always)]
pub fn trig_asin(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.asin()))
}

#[inline(always)]
pub fn trig_acos(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.acos()))
}

#[inline(always)]
pub fn trig_atan(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.atan()))
}

#[inline(always)]
pub fn trig_asinh(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.asinh()))
}

#[inline(always)]
pub fn trig_acosh(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.acosh()))
}

#[inline(always)]
pub fn trig_atanh(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.atanh()))
}

#[inline(always)]
pub fn sign(val: &Value) -> ValueResult {
    Ok(Value::from_num(if val.to_num()? < 0.0 { -1.0 } else { 1.0 }))
}

#[inline(always)]
pub fn absolute(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.abs()))
}

#[inline(always)]
pub fn reciprocal(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.recip()))
}

#[inline(always)]
pub fn fraction(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.fract()))
}

#[inline(always)]
pub fn truncate(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.trunc()))
}

#[inline(always)]
pub fn ceiling(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.ceil()))
}

#[inline(always)]
pub fn floor(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.floor()))
}

#[inline(always)]
pub fn round(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.to_num()?.round()))
}

#[inline(always)]
pub fn negate(val: &Value) -> ValueResult {
    Ok(Value::from_num(-val.to_num()?))
}

#[inline(always)]
pub fn logical_not(val: &Value) -> ValueResult {
    Ok(Value::from_bool(!val.to_bool()?))
}

#[inline(always)]
pub fn num_cast(val: &Value) -> ValueResult {
    Ok(Value::from_num(val.as_num()))
}

#[inline(always)]
pub fn bool_cast(val: &Value) -> ValueResult {
    Ok(Value::from_bool(val.as_bool()))
}

// --------------------------------------------------------------------------------

pub type UnaryFtn = fn(&Value) -> ValueResult;

pub fn uop2ftn(name: &str) -> Option<UnaryFtn> {
    match name {
        keywords::SQRT => Some(square_root),
        keywords::EXP => Some(exponential),
        keywords::EXP2 => Some(exponential2),
        keywords::LN => Some(natural_logarithm),
        keywords::LOG2 => Some(logarithm2),
        keywords::LOG10 => Some(logarithm10),
        keywords::SIN => Some(trig_sin),
        keywords::COS => Some(trig_cos),
        keywords::TAN => Some(trig_tan),
        keywords::SINH => Some(trig_sinh),
        keywords::COSH => Some(trig_cosh),
        keywords::TANH => Some(trig_tanh),
        keywords::ASIN => Some(trig_asin),
        keywords::ACOS => Some(trig_acos),
        keywords::ATAN => Some(trig_atan),
        keywords::ASINH => Some(trig_asinh),
        keywords::ACOSH => Some(trig_acosh),
        keywords::ATANH => Some(trig_atanh),
        keywords::SIGN => Some(sign),
        keywords::ABS => Some(absolute),
        keywords::RECIP => Some(reciprocal),
        keywords::FRACT => Some(fraction),
        keywords::TRUNC => Some(truncate),
        keywords::CEIL => Some(ceiling),
        keywords::FLOOR => Some(floor),
        keywords::ROUND => Some(round),
        keywords::NEG => Some(negate),
        keywords::NOT => Some(logical_not),
        keywords::ASNUM => Some(num_cast),
        keywords::ASBOOL => Some(bool_cast),
        _ => None
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn check_equal(lhs: Value, rhs: f64) -> bool {
        (lhs.to_num().unwrap() - rhs).abs() < 0.0001
    }

    #[test]
    fn test_unop_square_root() {
        let four = Value::from_num(4.0);
        let yes = Value::from_bool(true);
        assert!(check_equal(square_root(&four).unwrap(), 2.0));
        assert!(square_root(&yes).is_err());
    }

    #[test]
    fn test_unop_exponential() {
        let v0 = Value::from_num(0.0);
        let v1 = Value::from_num(1.0);
        let v2 = Value::from_num(2.0);
        assert!(check_equal(exponential(&v0).unwrap(), 1.0));
        assert!(check_equal(exponential(&v1).unwrap(), 2.7182));
        assert!(check_equal(exponential(&v2).unwrap(), 7.3890));
    }

    #[test]
    fn test_unop_exponential2() {
        let v0 = Value::from_num(0.0);
        let v1 = Value::from_num(1.0);
        let v2 = Value::from_num(2.0);
        assert!(check_equal(exponential2(&v0).unwrap(), 1.0));
        assert!(check_equal(exponential2(&v1).unwrap(), 2.0));
        assert!(check_equal(exponential2(&v2).unwrap(), 4.0));
    }

    #[test]
    fn test_unop_natural_logarithm() {
        let v1 = Value::from_num(1.0);
        let v2 = Value::from_num(2.0);
        assert!(check_equal(natural_logarithm(&v1).unwrap(), 0.0));
        assert!(check_equal(natural_logarithm(&v2).unwrap(), 0.6931));
    }

    #[test]
    fn test_unop_logarithm2() {
        let v2 = Value::from_num(2.0);
        let v4 = Value::from_num(4.0);
        assert!(check_equal(logarithm2(&v2).unwrap(), 1.0));
        assert!(check_equal(logarithm2(&v4).unwrap(), 2.0));
    }

    #[test]
    fn test_unop_logarithm10() {
        let v10 = Value::from_num(10.0);
        let v100 = Value::from_num(100.0);
        assert!(check_equal(logarithm10(&v10).unwrap(), 1.0));
        assert!(check_equal(logarithm10(&v100).unwrap(), 2.0));
    }

    #[test]
    fn test_unop_trig_sin() {
        let v0 = Value::from_num(0.0);
        let half_pi = Value::from_num(1.5707963267948966);
        let pi = Value::from_num(3.141592653589793);
        assert!(check_equal(trig_sin(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_sin(&half_pi).unwrap(), 1.0));
        assert!(check_equal(trig_sin(&pi).unwrap(), 0.0));
    }

    #[test]
    fn test_unop_trig_cos() {
        let v0 = Value::from_num(0.0);
        let half_pi = Value::from_num(1.5707963267948966);
        let pi = Value::from_num(3.141592653589793);
        assert!(check_equal(trig_cos(&v0).unwrap(), 1.0));
        assert!(check_equal(trig_cos(&half_pi).unwrap(), 0.0));
        assert!(check_equal(trig_cos(&pi).unwrap(), -1.0));
    }

    #[test]
    fn test_unop_trig_tan() {
        let v0 = Value::from_num(0.0);
        let quarter_pi = Value::from_num(0.7853981633974483);
        let pi = Value::from_num(3.141592653589793);
        assert!(check_equal(trig_tan(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_tan(&quarter_pi).unwrap(), 1.0));
        assert!(check_equal(trig_tan(&pi).unwrap(), 0.0));
    }

    #[test]
    fn test_unop_trig_sinh() {
        let v0 = Value::from_num(0.0);
        let half_pi = Value::from_num(1.5707963267948966);
        let pi = Value::from_num(3.141592653589793);
        assert!(check_equal(trig_sinh(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_sinh(&half_pi).unwrap(), 2.3012));
        assert!(check_equal(trig_sinh(&pi).unwrap(), 11.5487));
    }

    #[test]
    fn test_unop_trig_cosh() {
        let v0 = Value::from_num(0.0);
        let half_pi = Value::from_num(1.5707963267948966);
        let pi = Value::from_num(3.141592653589793);
        assert!(check_equal(trig_cosh(&v0).unwrap(), 1.0));
        assert!(check_equal(trig_cosh(&half_pi).unwrap(), 2.5091));
        assert!(check_equal(trig_cosh(&pi).unwrap(), 11.5919));
    }

    #[test]
    fn test_unop_trig_asin() {
        let v0 = Value::from_num(0.0);
        let quarter_pi = Value::from_num(0.7853981633974483);
        let v1 = Value::from_num(1.0);
        assert!(check_equal(trig_asin(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_asin(&quarter_pi).unwrap(), 0.9033));
        assert!(check_equal(trig_asin(&v1).unwrap(), 1.5707));
    }

    #[test]
    fn test_unop_trig_acos() {
        let v0 = Value::from_num(0.0);
        let quarter_pi = Value::from_num(0.7853981633974483);
        let v1 = Value::from_num(1.0);
        assert!(check_equal(trig_acos(&v0).unwrap(), 1.5707));
        assert!(check_equal(trig_acos(&quarter_pi).unwrap(), 0.6674));
        assert!(check_equal(trig_acos(&v1).unwrap(), 0.0));
    }

    #[test]
    fn test_unop_trig_atan() {
        let v0 = Value::from_num(0.0);
        let quarter_pi = Value::from_num(0.7853981633974483);
        let v1 = Value::from_num(1.0);
        assert!(check_equal(trig_atan(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_atan(&quarter_pi).unwrap(), 0.6657));
        assert!(check_equal(trig_atan(&v1).unwrap(), 0.7853));
    }

    #[test]
    fn test_unop_trig_asinh() {
        let v0 = Value::from_num(0.0);
        let quarter_pi = Value::from_num(0.7853981633974483);
        let v1 = Value::from_num(1.0);
        assert!(check_equal(trig_asinh(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_asinh(&quarter_pi).unwrap(), 0.7212));
        assert!(check_equal(trig_asinh(&v1).unwrap(), 0.8813));
    }

    #[test]
    fn test_unop_trig_acosh() {
        let v1 = Value::from_num(1.0);
        let half_pi = Value::from_num(1.5707963267948966);
        let pi = Value::from_num(3.141592653589793);
        assert!(check_equal(trig_acosh(&v1).unwrap(), 0.0));
        assert!(check_equal(trig_acosh(&half_pi).unwrap(), 1.0232));
        assert!(check_equal(trig_acosh(&pi).unwrap(), 1.8115));
    }

    #[test]
    fn test_unop_trig_atanh() {
        let v0 = Value::from_num(0.0);
        let quarter_pi = Value::from_num(0.7853981633974483);
        assert!(check_equal(trig_atanh(&v0).unwrap(), 0.0));
        assert!(check_equal(trig_atanh(&quarter_pi).unwrap(), 1.0593));
    }

    #[test]
    fn test_unop_sign() {
        let v0 = Value::from_num(0.0);
        let v2 = Value::from_num(2.0);
        let minus2 = Value::from_num(-2.0);
        assert!(check_equal(sign(&v0).unwrap(), 1.0));
        assert!(check_equal(sign(&v2).unwrap(), 1.0));
        assert!(check_equal(sign(&minus2).unwrap(), -1.0));
    }

    #[test]
    fn test_unop_absolute() {
        let v0 = Value::from_num(0.0);
        let v2 = Value::from_num(2.0);
        let minus2 = Value::from_num(-2.0);
        assert!(check_equal(absolute(&v0).unwrap(), 0.0));
        assert!(check_equal(absolute(&v2).unwrap(), 2.0));
        assert!(check_equal(absolute(&minus2).unwrap(), 2.0));
    }

    #[test]
    fn test_unop_reciprocal() {
        let v2 = Value::from_num(2.0);
        let minus2 = Value::from_num(-2.0);
        assert!(check_equal(reciprocal(&v2).unwrap(), 0.5));
        assert!(check_equal(reciprocal(&minus2).unwrap(), -0.5));
    }

    #[test]
    fn test_unop_fraction() {
        let v2 = Value::from_num(2.0053);
        let minus2 = Value::from_num(-2.1234);
        assert!(check_equal(fraction(&v2).unwrap(), 0.0053));
        assert!(check_equal(fraction(&minus2).unwrap(), -0.1234));
    }

    #[test]
    fn test_unop_truncate() {
        let v2 = Value::from_num(2.0053);
        let minus2 = Value::from_num(-2.1234);
        assert!(check_equal(truncate(&v2).unwrap(), 2.0));
        assert!(check_equal(truncate(&minus2).unwrap(), -2.0));
    }

    #[test]
    fn test_unop_ceiling() {
        let v2 = Value::from_num(2.0053);
        let minus2 = Value::from_num(-2.1234);
        assert!(check_equal(ceiling(&v2).unwrap(), 3.0));
        assert!(check_equal(ceiling(&minus2).unwrap(), -2.0));
    }

    #[test]
    fn test_unop_floor() {
        let v2 = Value::from_num(2.0053);
        let minus2 = Value::from_num(-2.1234);
        assert!(check_equal(floor(&v2).unwrap(), 2.0));
        assert!(check_equal(floor(&minus2).unwrap(), -3.0));
    }

    #[test]
    fn test_unop_round() {
        let v2 = Value::from_num(2.0053);
        let minus2 = Value::from_num(-2.1234);
        assert!(check_equal(round(&v2).unwrap(), 2.0));
        assert!(check_equal(round(&minus2).unwrap(), -2.0));
    }

    #[test]
    fn test_unop_negate() {
        let v2 = Value::from_num(2.0);
        let minus2 = Value::from_num(-2.0);
        assert!(check_equal(negate(&v2).unwrap(), -2.0));
        assert!(check_equal(negate(&minus2).unwrap(), 2.0));
    }

    #[test]
    fn test_unop_logical_not() {
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        assert_eq!(logical_not(&yes).unwrap(), no);
        assert_eq!(logical_not(&no).unwrap(), yes);
    }

    #[test]
    fn test_type_cast() {
        let one = Value::from_num(1.0);
        let zero = Value::from_num(0.0);
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);

        assert_eq!(num_cast(&one).unwrap(), one);
        assert_eq!(num_cast(&zero).unwrap(), zero);
        assert_eq!(num_cast(&yes).unwrap(), one);
        assert_eq!(num_cast(&no).unwrap(), zero);

        assert_eq!(bool_cast(&one).unwrap(), yes);
        assert_eq!(bool_cast(&zero).unwrap(), no);
        assert_eq!(bool_cast(&yes).unwrap(), yes);
        assert_eq!(bool_cast(&no).unwrap(), no);
    }
}
