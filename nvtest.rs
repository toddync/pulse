#![allow(warnings)]
// use std::fmt::format;

#[derive(Debug)]
pub enum Value {
    Number(i128),
    Float(f64),
    String(String),
    Bool(bool),
    // List(Vec<Value>),
    // Map(HashMap<String, Value>),
    Undefined,
}

impl Value {
    // Example helper methods for type conversion
    pub fn as_int(&self) -> Option<i128> {
        if let Value::Number(i) = self {
            Some(*i)
        } else if let Value::Float(i) = self {
            Some(*i as i128)
        } else {
            None
        }
    }

    pub fn as_flt(&self) -> Option<f64> {
        if let Value::Number(i) = self {
            Some(*i as f64)
        } else if let Value::Float(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Number(a) => self.as_int().unwrap() != 0,
            Value::Float(a) => self.as_flt().unwrap() > f64::from(0) || self.as_flt().unwrap() < f64::from(0),
            Value::Bool(a)=> a == &true,
            Value::String(a)=> a.len() > 0,
            _ => false,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Value::Number(a) => format!("{}", a),
            Value::Float(a) => format!("{}", a),
            Value::Bool(a)=> format!("{}", a),
            Value::String(a)=> format!("{}", a),
            _ => format!(""),
        }
    }

    pub fn add(self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Number(a), Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Number(_)) => Value::Float(a + other.as_flt().unwrap()),
            (Value::String(a), Value::String(b)) => Value::String(a + b),
            _ => Value::Undefined,
        }
    }

    pub fn mul(self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Number(a), Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a * other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn modu(self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
            (Value::Number(a), Value::Float(b)) => Value::Float(a as f64 % b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a % other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn gt(self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a > other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a > other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool(a as f64 > other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a > other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn eq(self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a == other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a == other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool(a as f64 == other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a == other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn dif(self, other: &Value) -> bool {
        self.eq(other).not()
    }

    pub fn not(self) -> bool {
        !self.as_bool()
    }

    pub fn and(self, other: &Value) -> Value {
        match self.as_bool() {
            true => Value::Bool(other.as_bool()),
            false => Value::Bool(false)
        }
    }
}

fn main() {
    print(&vec![Value::String("Hello, World!".to_string())]);

    let mut x = Value::Number(10);
    let mut y = Value::Number(20);
    let mut z = x.add(&y);
    print(&vec![z]);

    let mut a = Value::Number(15);

    if a.gt(&Value::Number(10)).as_bool() {
        print(&vec![Value::String("Greater than 10".to_string())]);

    }

    let mut counter = Value::Number(3);
    let mut x = Value::Number(5);
    let mut y = Value::Number(3);
    let mut add = x.add(&y);
    let mut sub = x.sub(&y);
    let mut mul = x.mul(&y);
    let mut div = x.div(&y);
    let mut modu = x.modu(&y);
    print(&vec![add]);

    print(&vec![div]);

    let mut x = Value::Number(5);
    let mut y = Value::Number(10);
    print(&vec![x.lt(&y)]);

    print(&vec![x.gt(&y)]);

    print(&vec![x.eq(&y)]);

    print(&vec![x.dif(&y)]);


}

fn print(list: &[Value]) {
    for v in list {
        print!("{} ", v.as_str())
    }
    println!()
}
