use std::fmt;
use std::cmp;

// --------------------------------------------------------------------------------
// Value Error

#[derive(Debug, Clone)]
pub struct ValueError {
    error_msg: String
}

impl ValueError {
    pub fn new(err_msg: &str) -> ValueError {
        ValueError {
            error_msg: String::from(err_msg)
        }
    }

    pub fn from_string(err_msg: String) -> ValueError {
        ValueError {
            error_msg: err_msg
        }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

// --------------------------------------------------------------------------------
// Value

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Num(f64),    // Number
    Bool(bool),  // Boolean
}

impl Value {
    #[inline(always)]
    pub fn from_num(n: f64) -> Value {
        Value::Num(n)
    }

    #[inline(always)]
    pub fn from_bool(b: bool) -> Value {
        Value::Bool(b)
    }

    #[inline(always)]
    pub fn is_num(&self) -> bool {
        if let Value::Num(_) = self { true } else { false }
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        if let Value::Bool(_) = self { true } else { false }
    }

    pub fn to_num(&self) -> Result<f64, ValueError> {
        match self {
            Value::Num(n) => Ok(*n),
            Value::Bool(_) => Err(ValueError::from_string(format!("{} not a number", self)))
        }
    }

    pub fn to_bool(&self) -> Result<bool, ValueError> {
        match self {
            Value::Num(_) => Err(ValueError::from_string(format!("{} not a boolean", self))),
            Value::Bool(b) => Ok(*b)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Num(n) => format!("{}", n),
            Value::Bool(b) => format!("{}", b),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Num(n) => other.is_num() && *n == other.to_num().unwrap(),
            Value::Bool(b) => other.is_bool() && *b == other.to_bool().unwrap(),
        }
    }
}

impl cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.is_num() && other.is_num() {
            self.to_num().unwrap().partial_cmp(&other.to_num().unwrap())
        }
        else if self.is_bool() && other.is_bool() {
            self.to_bool().unwrap().partial_cmp(&other.to_bool().unwrap())
        }
        else {
            None
        }
    }
}

// --------------------------------------------------------------------------------
// Value Result

pub type ValueResult = Result<Value, ValueError>;

// --------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_error() {
        assert_eq!(format!("{}", ValueError::new("foobar")), "foobar");

        let five = Value::from_num(5.0);
        let yes = Value::from_bool(true);

        assert_eq!(format!("{}", five.to_bool().unwrap_err()), "5 not a boolean");
        assert_eq!(format!("{}", yes.to_num().unwrap_err()), "true not a number");
    }

    #[test]
    fn test_value_num() {
        let five = Value::from_num(5.0);
        assert!(five.is_num());
        assert!(!five.is_bool());

        assert_eq!(five.to_num().unwrap(), 5.0);
        assert!(five.to_bool().is_err());

        assert_eq!(five.to_string(), "5");
    }

    #[test]
    fn test_value_bool() {
        let flag = Value::from_bool(true);
        assert!(!flag.is_num());
        assert!(flag.is_bool());

        assert!(flag.to_num().is_err());
        assert_eq!(flag.to_bool().unwrap(), true);

        assert_eq!(flag.to_string(), "true");
    }

   #[test]
    fn test_value_equal() {
        let five1 = Value::from_num(5.0);
        let five2 = Value::from_num(5.0);
        let six = Value::from_num(6.0);
        let yes1 = Value::from_bool(true);
        let yes2 = Value::from_bool(true);
        let no = Value::from_bool(false);

        assert!(five1 == five2);
        assert!(five1 != six);
        assert!(five1 != yes1);

        assert!(yes1 == yes2);
        assert!(yes1 != no);
        assert!(yes1 != five1);
    }

    #[test]
    fn test_value_ordering() {
        let five1 = Value::from_num(5.0);
        let five2 = Value::from_num(5.0);
        let six = Value::from_num(6.0);
        let yes1 = Value::from_bool(true);
        let yes2 = Value::from_bool(true);
        let no = Value::from_bool(false);

        assert!(five1 < six);
        assert!(five1 <= five2);
        assert!(six > five1);
        assert!(five1 >= five2);

        assert!(no < yes1);
        assert!(yes1 <= yes2);
        assert!(yes1 > no);
        assert!(yes1 >= yes2);
    }
}
