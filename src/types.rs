#![allow(unused)]
#[derive(Debug)]
pub enum Value {
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    // List(Vec<Value>),
    // Map(HashMap<String, Value>),
    Undefined,
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Number(i) = self {
            Some(*i)
        } else if let Value::Float(i) = self {
            Some(*i as i64)
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

    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 + b),
            (Value::Float(a), Value::Number(_)) => Value::Float(a + other.as_flt().unwrap()),
            (Value::String(a), Value::String(b)) => Value::String(a.to_owned() + b),
            _ => Value::Undefined,
        }
    }

    pub fn sub(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 - b),
            (Value::Float(a), Value::Number(_)) => Value::Float(a - other.as_flt().unwrap()),
            // (Value::String(a), Value::String(b)) => Value::String(a - b),
            _ => Value::Undefined,
        }
    }

    pub fn mul(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Number(a), Value::Float(b)) => Value::Float(a.to_owned() as f64 * b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a * other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 / b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a / other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn modu(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 % b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a % other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn gt(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 > other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn lt(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool((a.to_owned() as f64) < other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn eq(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 == other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn dif(&self, other: &Value) -> Value {
        Value::Bool(self.eq(other).not())
    }

    pub fn not(self) -> bool {
        !self.as_bool()
    }

    pub fn and(&self, other: &Value) -> Value {
        match self.as_bool() {
            true => Value::Bool(other.as_bool()),
            false => Value::Bool(false)
        }
    }
}

pub static VALUE: &str = r#"#![allow(warnings)]
#[derive(Debug)]
pub enum Value {
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Undefined,
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Number(i) = self {
            Some(*i)
        } else if let Value::Float(i) = self {
            Some(*i as i64)
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

    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 + b),
            (Value::Float(a), Value::Number(_)) => Value::Float(a + other.as_flt().unwrap()),
            (Value::String(a), Value::String(b)) => Value::String(a.to_owned() + b),
            _ => Value::Undefined,
        }
    }

    pub fn sub(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 - b),
            (Value::Float(a), Value::Number(_)) => Value::Float(a - other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn mul(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Number(a), Value::Float(b)) => Value::Float(a.to_owned() as f64 * b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a * other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 / b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a / other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn modu(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 % b),
            (Value::Float(a), Value::Number(b)) => Value::Float(a % other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn gt(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 > other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn lt(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool((a.to_owned() as f64) < other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn eq(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_int().unwrap()),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),
            (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 == other.as_flt().unwrap()),
            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),
            _ => Value::Undefined,
        }
    }

    pub fn dif(&self, other: &Value) -> Value {
        Value::Bool(self.eq(other).not())
    }

    pub fn not(self) -> bool {
        !self.as_bool()
    }

    pub fn and(&self, other: &Value) -> Value {
        match self.as_bool() {
            true => Value::Bool(other.as_bool()),
            false => Value::Bool(false)
        }
    }
}"#;