use std::fmt;
use std::f64::{INFINITY, NEG_INFINITY};



#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Value {
    Str(::std::string::String),
    Number(f64),
    Bool(bool),
    Nil
}
use self::Value::*;

impl Value {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Str(_) | Number(_) => true,
            Bool(b) => b,
            Nil => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Number(a), &Number(b)) => a == b,
            (&Bool(a), &Bool(b)) => a == b,
            (&Nil, &Nil) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Str(ref s) => write!(f, "{}", s),
            Number(n) => if n == INFINITY {
                write!(f, "Infinity")
            } else if n == NEG_INFINITY {
                write!(f, "-Infinity")
            } else if n == 0.0 && n.is_sign_negative() {
                write!(f, "-{}", n)
            } else {
                write!(f, "{}", n)
            },
            Bool(b) => write!(f, "{}", b),
            Nil => write!(f, "nil")
        }
    }
}