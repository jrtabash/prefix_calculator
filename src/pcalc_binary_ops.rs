use crate::pcalc_value::{Value, ValueResult};

#[inline(always)]
pub fn add(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()? + rhs.to_num()?))
}

#[inline(always)]
pub fn subtract(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()? - rhs.to_num()?))
}

#[inline(always)]
pub fn multiply(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()? * rhs.to_num()?))
}

#[inline(always)]
pub fn divide(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()? / rhs.to_num()?))
}

#[inline(always)]
pub fn remainder(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()? % rhs.to_num()?))
}

#[inline(always)]
pub fn power(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(f64::powf(lhs.to_num()?, rhs.to_num()?)))
}

#[inline(always)]
pub fn maximum(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()?.max(rhs.to_num()?)))
}

#[inline(always)]
pub fn minimum(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_num(lhs.to_num()?.min(rhs.to_num()?)))
}

#[inline(always)]
pub fn equal(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs == rhs))
}

#[inline(always)]
pub fn not_equal(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs != rhs))
}

#[inline(always)]
pub fn less(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs < rhs))
}

#[inline(always)]
pub fn less_equal(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs <= rhs))
}

#[inline(always)]
pub fn greater(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs > rhs))
}

#[inline(always)]
pub fn greater_equal(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs >= rhs))
}

#[inline(always)]
pub fn logical_and(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs.to_bool()? && rhs.to_bool()?))
}

#[inline(always)]
pub fn logical_or(lhs: &Value, rhs: &Value) -> ValueResult {
    Ok(Value::from_bool(lhs.to_bool()? || rhs.to_bool()?))
}

// --------------------------------------------------------------------------------

pub type BinaryFtn = fn(&Value, &Value) -> ValueResult;

pub fn bop2ftn(name: &str) -> Option<BinaryFtn> {
    match name {
        "+" => Some(add),
        "-" => Some(subtract),
        "*" => Some(multiply),
        "/" => Some(divide),
        "%" => Some(remainder),
        "^" => Some(power),
        "max" => Some(maximum),
        "min" => Some(minimum),
        "==" => Some(equal),
        "!=" => Some(not_equal),
        "<" => Some(less),
        "<=" => Some(less_equal),
        ">" => Some(greater),
        ">=" => Some(greater_equal),
        "and" => Some(logical_and),
        "or" => Some(logical_or),
        _ => None
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binop_add() {
        let five = Value::from_num(5.0);
        let three = Value::from_num(3.0);
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        assert_eq!(add(&five, &three).unwrap(), Value::from_num(8.0));
        assert_eq!(add(&five, &yes).unwrap(), Value::from_num(6.0));
        assert_eq!(add(&five, &no).unwrap(), Value::from_num(5.0));
    }

    #[test]
    fn test_binop_subtract() {
        let five = Value::from_num(5.0);
        let three = Value::from_num(3.0);
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        assert_eq!(subtract(&five, &three).unwrap(), Value::from_num(2.0));
        assert_eq!(subtract(&five, &yes).unwrap(), Value::from_num(4.0));
        assert_eq!(subtract(&five, &no).unwrap(), Value::from_num(5.0));
    }

    #[test]
    fn test_binop_multiply() {
        let five = Value::from_num(5.0);
        let three = Value::from_num(3.0);
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        assert_eq!(multiply(&five, &three).unwrap(), Value::from_num(15.0));
        assert_eq!(multiply(&five, &yes).unwrap(), Value::from_num(5.0));
        assert_eq!(multiply(&five, &no).unwrap(), Value::from_num(0.0));
    }

    #[test]
    fn test_binop_divide() {
        let six = Value::from_num(6.0);
        let two = Value::from_num(2.0);
        let yes = Value::from_bool(true);
        assert_eq!(divide(&six, &two).unwrap(), Value::from_num(3.0));
        assert_eq!(divide(&six, &yes).unwrap(), Value::from_num(6.0));
    }

    #[test]
    fn test_binop_remainder() {
        let six = Value::from_num(6.0);
        let five = Value::from_num(5.0);
        let two = Value::from_num(2.0);
        let yes = Value::from_bool(true);
        assert_eq!(remainder(&six, &two).unwrap(), Value::from_num(0.0));
        assert_eq!(remainder(&five, &two).unwrap(), Value::from_num(1.0));
        assert_eq!(remainder(&five, &yes).unwrap(), Value::from_num(0.0));
    }

    #[test]
    fn test_binop_power() {
        let six = Value::from_num(6.0);
        let two = Value::from_num(2.0);
        let yes = Value::from_bool(true);
        assert_eq!(power(&six, &two).unwrap(), Value::from_num(36.0));
        assert_eq!(power(&six, &yes).unwrap(), Value::from_num(6.0));
    }

    #[test]
    fn test_binop_maximum() {
        let six = Value::from_num(6.0);
        let two = Value::from_num(2.0);
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        assert_eq!(maximum(&six, &two).unwrap(), Value::from_num(6.0));
        assert_eq!(maximum(&two, &six).unwrap(), Value::from_num(6.0));
        assert_eq!(maximum(&yes, &no).unwrap(), Value::from_num(1.0));
    }

    #[test]
    fn test_binop_minimum() {
        let six = Value::from_num(6.0);
        let two = Value::from_num(2.0);
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        assert_eq!(minimum(&six, &two).unwrap(), Value::from_num(2.0));
        assert_eq!(minimum(&two, &six).unwrap(), Value::from_num(2.0));
        assert_eq!(minimum(&yes, &no).unwrap(), Value::from_num(0.0));
    }

    #[test]
    fn test_binop_equal() {
        let one1 = Value::from_num(1.0);
        let one2 = Value::from_num(1.0);
        let two = Value::from_num(2.0);
        assert_eq!(equal(&one1, &one2).unwrap(), Value::from_bool(true));
        assert_eq!(equal(&one1, &two).unwrap(), Value::from_bool(false));
    }

    #[test]
    fn test_binop_not_equal() {
        let one1 = Value::from_num(1.0);
        let one2 = Value::from_num(1.0);
        let two = Value::from_num(2.0);
        assert_eq!(not_equal(&one1, &one2).unwrap(), Value::from_bool(false));
        assert_eq!(not_equal(&one1, &two).unwrap(), Value::from_bool(true));
    }

    #[test]
    fn test_binop_less() {
        let one1 = Value::from_num(1.0);
        let one2 = Value::from_num(1.0);
        let two = Value::from_num(2.0);
        assert_eq!(less(&one1, &one2).unwrap(), Value::from_bool(false));
        assert_eq!(less(&one1, &two).unwrap(), Value::from_bool(true));
    }

    #[test]
    fn test_binop_less_equal() {
        let one1 = Value::from_num(1.0);
        let one2 = Value::from_num(1.0);
        let two = Value::from_num(2.0);
        assert_eq!(less_equal(&one1, &one2).unwrap(), Value::from_bool(true));
        assert_eq!(less_equal(&one1, &two).unwrap(), Value::from_bool(true));
        assert_eq!(less_equal(&two, &one1).unwrap(), Value::from_bool(false));
    }

    #[test]
    fn test_binop_greater() {
        let one1 = Value::from_num(1.0);
        let one2 = Value::from_num(1.0);
        let two = Value::from_num(2.0);
        assert_eq!(greater(&one1, &one2).unwrap(), Value::from_bool(false));
        assert_eq!(greater(&two, &one1).unwrap(), Value::from_bool(true));
    }

    #[test]
    fn test_binop_greater_equal() {
        let one1 = Value::from_num(1.0);
        let one2 = Value::from_num(1.0);
        let two = Value::from_num(2.0);
        assert_eq!(greater_equal(&one1, &one2).unwrap(), Value::from_bool(true));
        assert_eq!(greater_equal(&two, &one1).unwrap(), Value::from_bool(true));
        assert_eq!(greater_equal(&one1, &two).unwrap(), Value::from_bool(false));
    }

    #[test]
    fn test_binop_logical_and() {
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        let one = Value::from_num(1.0);
        let zero = Value::from_num(0.0);

        assert_eq!(logical_and(&yes, &yes).unwrap(), yes);
        assert_eq!(logical_and(&yes, &no).unwrap(), no);
        assert_eq!(logical_and(&no, &yes).unwrap(), no);
        assert_eq!(logical_and(&no, &no).unwrap(), no);

        assert_eq!(logical_and(&one, &one).unwrap(), yes);
        assert_eq!(logical_and(&one, &zero).unwrap(), no);
        assert_eq!(logical_and(&zero, &one).unwrap(), no);
        assert_eq!(logical_and(&zero, &zero).unwrap(), no);
    }

    #[test]
    fn test_binop_logical_or() {
        let yes = Value::from_bool(true);
        let no = Value::from_bool(false);
        let one = Value::from_num(1.0);
        let zero = Value::from_num(0.0);

        assert_eq!(logical_or(&yes, &yes).unwrap(), yes);
        assert_eq!(logical_or(&yes, &no).unwrap(), yes);
        assert_eq!(logical_or(&no, &yes).unwrap(), yes);
        assert_eq!(logical_or(&no, &no).unwrap(), no);

        assert_eq!(logical_or(&one, &one).unwrap(), yes);
        assert_eq!(logical_or(&one, &zero).unwrap(), yes);
        assert_eq!(logical_or(&zero, &one).unwrap(), yes);
        assert_eq!(logical_or(&zero, &zero).unwrap(), no);
    }
}
