use crate::pcalc_environment::Environment;
use crate::pcalc_function::FunctionPtr;
use crate::pcalc_value::ValueError;
use std::collections::HashSet;
use std::fmt;

// --------------------------------------------------------------------------------
// Check Error

#[derive(Debug, Clone)]
pub struct CheckError {
    error_msg: String
}

impl CheckError {
    pub fn self_recursive(name: &str) -> Self {
        CheckError {
            error_msg: format!("Self recursive function '{}'", name)
        }
    }

    pub fn dual_recursive(name1: &str, name2: &str) -> Self {
        CheckError {
            error_msg: format!("Dual recursive functions '{}' and '{}'", name1, name2)
        }
    }

    pub fn cross_recursive(name1: &str, name2: &str) -> Self {
        CheckError {
            error_msg: format!("Cross recursive functions '{}' and '{}'", name1, name2)
        }
    }
}

impl From<CheckError> for ValueError {
    fn from(item: CheckError) -> Self {
        ValueError::new(&item.error_msg)
    }
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

// --------------------------------------------------------------------------------

pub type CheckResult = Result<(), CheckError>;

// --------------------------------------------------------------------------------
pub fn check_self_recursive(name: &str, func: &FunctionPtr) -> CheckResult {
    let call_cnt = func
        .body()
        .iter()
        .filter(|c| c.is_funcall())
        .map(|c| c.get_name().unwrap_or(""))
        .filter(|n| *n == name)
        .count();
    if call_cnt > 0 {
        Err(CheckError::self_recursive(name))
    } else {
        Ok(())
    }
}

// --------------------------------------------------------------------------------
pub fn check_dual_recursive(name: &str, func: &FunctionPtr, env: &Environment) -> CheckResult {
    let fcalls: Vec<&str> = func
        .body()
        .iter()
        .filter(|c| c.is_funcall())
        .map(|c| c.get_name().unwrap_or(""))
        .filter(|n| !n.is_empty() && *n != name)
        .collect();
    for nm in &fcalls {
        if let Ok(f) = env.get_func(nm) {
            let call_cnt = f
                .body()
                .iter()
                .filter(|c| c.is_funcall())
                .map(|c| c.get_name().unwrap_or(""))
                .filter(|n| *n == name)
                .count();
            if call_cnt > 0 {
                return Err(CheckError::dual_recursive(name, nm));
            }
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------------
pub fn check_cross_recursive(name: &str, func: &FunctionPtr, env: &Environment) -> CheckResult {
    let mut to_visit: HashSet<&str> = func
        .body()
        .iter()
        .filter(|c| c.is_funcall())
        .map(|c| c.get_name().unwrap_or(""))
        .filter(|n| !n.is_empty() && *n != name)
        .collect();
    let mut visited: HashSet<&str> = Default::default();
    while !to_visit.is_empty() {
        let nm = *to_visit.iter().next().unwrap();
        to_visit.remove(nm);
        visited.insert(nm);

        if let Ok(f) = env.get_func(nm) {
            for fc in f.body().iter().filter(|c| c.is_funcall()) {
                if let Some(fcn) = fc.get_name() {
                    if fcn == name {
                        return Err(CheckError::cross_recursive(name, nm));
                    }

                    if !visited.contains(fcn) {
                        to_visit.insert(fcn);
                    }
                }
            }
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcalc_code::Funcall;
    use crate::pcalc_function::*;

    #[test]
    fn test_check_self_recursive() {
        let fptr = make_func("foobar");
        match check_self_recursive("foobar", &fptr) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(format!("{}", e), "Self recursive function 'foobar'")
        };
    }

    #[test]
    fn test_check_not_self_recursive() {
        let fptr = make_func("wahoo");
        assert!(check_self_recursive("foobar", &fptr).is_ok());
    }

    #[test]
    fn test_check_dual_recursive() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func("foo"));

        let foo = make_func("bar");
        match check_dual_recursive("foo", &foo, &env) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(format!("{}", e), "Dual recursive functions 'foo' and 'bar'")
        };
    }

    #[test]
    fn test_check_not_dual_recursive() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func("tar"));

        let foo = make_func("bar");
        assert!(check_dual_recursive("foo", &foo, &env).is_ok());
    }

    #[test]
    fn test_check_cross_recursive() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func("foo"));

        let foo = make_func("bar");
        match check_cross_recursive("foo", &foo, &env) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(format!("{}", e), "Cross recursive functions 'foo' and 'bar'")
        };
    }

    #[test]
    fn test_check_cross_recursive2() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func("tar"));
        env.def_func("tar", &make_func("foo"));

        let foo = make_func("bar");
        match check_cross_recursive("foo", &foo, &env) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(format!("{}", e), "Cross recursive functions 'foo' and 'tar'")
        };
    }

    #[test]
    fn test_check_cross_recursive3() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func2("tar", "zar"));
        env.def_func("tar", &make_func2("foo", "zar"));
        env.def_func("zar", &make_func("car"));

        let foo = make_func2("zar", "bar");
        match check_cross_recursive("foo", &foo, &env) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(format!("{}", e), "Cross recursive functions 'foo' and 'tar'")
        };
    }

    #[test]
    fn test_check_not_cross_recursive1() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func("tar"));

        let foo = make_func("bar");
        assert!(check_cross_recursive("foo", &foo, &env).is_ok());
    }

    #[test]
    fn test_check_not_cross_recursive2() {
        let mut env = Environment::new();

        env.def_func("bar", &make_func("tar"));
        env.def_func("tar", &make_func("zar"));
        env.def_func("zar", &make_func("car"));

        let foo = make_func2("bar", "zar");
        assert!(check_cross_recursive("foo", &foo, &env).is_ok());
    }

    fn make_func(call: &str) -> FunctionPtr {
        let mut exprs = Expressions::new();
        exprs.push(Box::new(Funcall::new(call.to_string(), Arguments::new())));
        FunctionPtr::new(Function::new(Parameters::new(), exprs))
    }

    fn make_func2(call: &str, call2: &str) -> FunctionPtr {
        let mut exprs = Expressions::new();
        exprs.push(Box::new(Funcall::new(call.to_string(), Arguments::new())));
        exprs.push(Box::new(Funcall::new(call2.to_string(), Arguments::new())));
        FunctionPtr::new(Function::new(Parameters::new(), exprs))
    }
}
