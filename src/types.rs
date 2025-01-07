// #![allow(warnings)]

// #[derive(Debug)]
// pub enum Value {
//     Number(i64),
//     Float(f64),
//     String(String),
//     Bool(bool),
//     // List(Vec<Value>),
//     // Map(HashMap<String, Value>),
//     Undefined,
// }

// impl Value {
//     // Example helper methods for type conversion
//     pub fn as_int(&self) -> Option<i64> {
//         if let Value::Number(i) = self {
//             Some(*i)
//         } else if let Value::Float(i) = self {
//             Some(*i as i64)
//         } else {
//             None
//         }
//     }

//     pub fn as_flt(&self) -> Option<f64> {
//         if let Value::Number(i) = self {
//             Some(*i as f64)
//         } else if let Value::Float(i) = self {
//             Some(*i)
//         } else {
//             None
//         }
//     }

//     pub fn as_bool(&self) -> bool {
//         match self {
//             Value::Number(a) => self.as_int().unwrap() != 0,
//             Value::Float(a) => self.as_flt().unwrap() > f64::from(0) || self.as_flt().unwrap() < f64::from(0),
//             Value::Bool(a)=> a == &true,
//             Value::String(a)=> a.len() > 0,
//             _ => false,
//         }
//     }

//     pub fn as_str(&self) -> String {
//         match self {
//             Value::Number(a) => format!("{}", a),
//             Value::Float(a) => format!("{}", a),
//             Value::Bool(a)=> format!("{}", a),
//             Value::String(a)=> format!("{}", a),
//             _ => format!(""),
//         }
//     }

//     pub fn add(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
//             (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
//             (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 + b),
//             (Value::Float(a), Value::Number(_)) => Value::Float(a + other.as_flt().unwrap()),
//             (Value::String(a), Value::String(b)) => Value::String(a.to_owned() + b),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn sub(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
//             (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
//             (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 - b),
//             (Value::Float(a), Value::Number(_)) => Value::Float(a - other.as_flt().unwrap()),
//             // (Value::String(a), Value::String(b)) => Value::String(a - b),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn mul(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
//             (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
//             (Value::Number(a), Value::Float(b)) => Value::Float(a.to_owned() as f64 * b),
//             (Value::Float(a), Value::Number(b)) => Value::Float(a * other.as_flt().unwrap()),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn div(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
//             (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
//             (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 / b),
//             (Value::Float(a), Value::Number(b)) => Value::Float(a / other.as_flt().unwrap()),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn modu(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
//             (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
//             (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 % b),
//             (Value::Float(a), Value::Number(b)) => Value::Float(a % other.as_flt().unwrap()),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn gt(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_int().unwrap()),
//             (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),
//             (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 > other.as_flt().unwrap()),
//             (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn lt(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_int().unwrap()),
//             (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),
//             (Value::Number(a), Value::Float(b)) => Value::Bool((a.to_owned() as f64) < other.as_flt().unwrap()),
//             (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn eq(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_int().unwrap()),
//             (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),
//             (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 == other.as_flt().unwrap()),
//             (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),
//             _ => Value::Undefined,
//         }
//     }

//     pub fn dif(&self, other: &Value) -> Value {
//         Value::Bool(self.eq(other).not())
//     }

//     pub fn not(self) -> bool {
//         !self.as_bool()
//     }

//     pub fn and(&self, other: &Value) -> Value {
//         match self.as_bool() {
//             true => Value::Bool(other.as_bool()),
//             false => Value::Bool(false)
//         }
//     }
// }

pub static VALUE: &str = r#"#![allow(warnings)]\n\n#[derive(Debug)]\npub enum Value {\n    Number(i64),\n    Float(f64),\n    String(String),\n    Bool(bool),\n    // List(Vec<Value>),\n    // Map(HashMap<String, Value>),\n    Undefined,\n}\n\nimpl Value {\n    // Example helper methods for type conversion\n    pub fn as_int(&self) -> Option<i64> {\n        if let Value::Number(i) = self {\n            Some(*i)\n        } else if let Value::Float(i) = self {\n            Some(*i as i64)\n        } else {\n            None\n        }\n    }\n\n    pub fn as_flt(&self) -> Option<f64> {\n        if let Value::Number(i) = self {\n            Some(*i as f64)\n        } else if let Value::Float(i) = self {\n            Some(*i)\n        } else {\n            None\n        }\n    }\n\n    pub fn as_bool(&self) -> bool {\n        match self {\n            Value::Number(a) => self.as_int().unwrap() != 0,\n            Value::Float(a) => self.as_flt().unwrap() > f64::from(0) || self.as_flt().unwrap() < f64::from(0),\n            Value::Bool(a)=> a == &true,\n            Value::String(a)=> a.len() > 0,\n            _ => false,\n        }\n    }\n\n    pub fn as_str(&self) -> String {\n        match self {\n            Value::Number(a) => format!(\"{}\", a),\n            Value::Float(a) => format!(\"{}\", a),\n            Value::Bool(a)=> format!(\"{}\", a),\n            Value::String(a)=> format!(\"{}\", a),\n            _ => format!(\"\"),\n        }\n    }\n\n    pub fn add(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),\n            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),\n            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 + b),\n            (Value::Float(a), Value::Number(_)) => Value::Float(a + other.as_flt().unwrap()),\n            (Value::String(a), Value::String(b)) => Value::String(a.to_owned() + b),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn sub(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),\n            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),\n            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 - b),\n            (Value::Float(a), Value::Number(_)) => Value::Float(a - other.as_flt().unwrap()),\n            // (Value::String(a), Value::String(b)) => Value::String(a - b),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn mul(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),\n            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),\n            (Value::Number(a), Value::Float(b)) => Value::Float(a.to_owned() as f64 * b),\n            (Value::Float(a), Value::Number(b)) => Value::Float(a * other.as_flt().unwrap()),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn div(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),\n            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),\n            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 / b),\n            (Value::Float(a), Value::Number(b)) => Value::Float(a / other.as_flt().unwrap()),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn modu(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),\n            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),\n            (Value::Number(a), Value::Float(b)) => Value::Float(*a as f64 % b),\n            (Value::Float(a), Value::Number(b)) => Value::Float(a % other.as_flt().unwrap()),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn gt(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_int().unwrap()),\n            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),\n            (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 > other.as_flt().unwrap()),\n            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() > other.as_flt().unwrap()),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn lt(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_int().unwrap()),\n            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),\n            (Value::Number(a), Value::Float(b)) => Value::Bool((a.to_owned() as f64) < other.as_flt().unwrap()),\n            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() < other.as_flt().unwrap()),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn eq(&self, other: &Value) -> Value {\n        match (self, other) {\n            (Value::Number(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_int().unwrap()),\n            (Value::Float(a), Value::Float(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),\n            (Value::Number(a), Value::Float(b)) => Value::Bool(a.to_owned() as f64 == other.as_flt().unwrap()),\n            (Value::Float(a), Value::Number(b)) => Value::Bool(a.to_owned() == other.as_flt().unwrap()),\n            _ => Value::Undefined,\n        }\n    }\n\n    pub fn dif(&self, other: &Value) -> Value {\n        Value::Bool(self.eq(other).not())\n    }\n\n    pub fn not(self) -> bool {\n        !self.as_bool()\n    }\n\n    pub fn and(&self, other: &Value) -> Value {\n        match self.as_bool() {\n            true => Value::Bool(other.as_bool()),\n            false => Value::Bool(false)\n        }\n    }\n}"#;