#![allow(unused_imports)]
use super::{super::types::Value, Res};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<'a> Value<'a> {
    fn type_name(&self) -> &'static str {
        match self {
            Value::Undefined => "Undefined",
            Value::Num(_) => "Num",
            Value::Str(_) => "Str",
            Value::Bool(_) => "Bool",
            Value::Vec(_) => "Vec",
            Value::Obj(_) => "Obj",
            Value::Fn(_, _) => "Function",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Undefined => false,
            Value::Fn(_, _) => true,
            Value::Bool(b) => *b,
            Value::Num(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Vec(v) => !v.is_empty(),
            Value::Obj(o) => !o.is_empty(),
        }
    }
}

impl<'a> Add for Value<'a> {
    type Output = Res<'a>;

    fn add(self, other: Self) -> Res<'a> {
        // Destructure once for all 25 possible (LHS, RHS) pairs
        match (self, other) {
            // — Numeric + numeric
            (Value::Num(a), Value::Num(b)) => Res::V(Value::Num(a + b)),

            // — String/number, string/bool concatenations
            (Value::Num(a), Value::Str(b)) => Res::V(Value::Str(format!("{}{}", a, b))),
            (Value::Str(a), Value::Num(b)) => Res::V(Value::Str(format!("{}{}", a, b))),
            (Value::Bool(a), Value::Str(b)) => Res::V(Value::Str(format!("{}{}", a, b))),
            (Value::Str(a), Value::Bool(b)) => Res::V(Value::Str(format!("{}{}", a, b))),
            (Value::Str(a), Value::Str(b)) => Res::V(Value::Str(format!("{}{}", a, b))),

            // — Boolean acts like 0(false) or 1(true):
            (Value::Num(mut a), Value::Bool(b)) => {
                if b {
                    a += 1.0
                }
                Res::V(Value::Num(a))
            }

            (Value::Bool(a), Value::Num(mut b)) => {
                if a {
                    b += 1.0
                }
                Res::V(Value::Num(b))
            }

            // — Undefined acts like additive zero or empty:
            (Value::Undefined, Value::Undefined) => Res::V(Value::Undefined),
            (Value::Undefined, Value::Num(n)) | (Value::Num(n), Value::Undefined) => {
                Res::V(Value::Num(n))
            }
            (Value::Undefined, Value::Str(s)) | (Value::Str(s), Value::Undefined) => {
                Res::V(Value::Str(s))
            }
            (Value::Undefined, Value::Vec(v)) | (Value::Vec(v), Value::Undefined) => {
                Res::V(Value::Vec(v))
            }
            (Value::Undefined, Value::Obj(o)) | (Value::Obj(o), Value::Undefined) => {
                Res::V(Value::Obj(o))
            }

            // — Vector concatenation
            (Value::Vec(mut a), Value::Vec(b)) => {
                a.extend(b);
                Res::V(Value::Vec(a))
            }

            // — Object (map) merge: RHS entries override LHS on key conflict
            (Value::Obj(mut a), Value::Obj(b)) => {
                a.extend(b);
                Res::V(Value::Obj(a))
            }

            // — Everything else is unsupported
            (lhs, rhs) => Res::E(format!(
                "Addition not supported: {} + {}",
                lhs.type_name(),
                rhs.type_name(),
            )),
        }
    }
}

impl<'a> Sub for Value<'a> {
    type Output = Res<'a>;

    fn sub(self, other: Self) -> Res<'a> {
        match (self, other) {
            // — Numeric subtraction
            (Value::Num(a), Value::Num(b)) => Res::V(Value::Num(a - b)),

            // — Boolean acts like 0(false) or 1(true):
            (Value::Num(mut a), Value::Bool(b)) => {
                if b {
                    a -= 1.0
                }
                Res::V(Value::Num(a))
            }
            (Value::Bool(a), Value::Num(mut b)) => {
                if a {
                    b -= 1.0
                }
                Res::V(Value::Num(b))
            }

            // — Undefined as zero/empty
            (Value::Undefined, Value::Undefined) => Res::V(Value::Undefined),
            (Value::Undefined, Value::Num(n)) | (Value::Num(n), Value::Undefined) => {
                Res::V(Value::Num(n))
            } // 0 - n = n, n - 0 = n
            (Value::Undefined, Value::Str(s)) | (Value::Str(s), Value::Undefined) => {
                Res::V(Value::Str(s))
            } // "" - s = s, s - "" = s
            (Value::Undefined, Value::Vec(v)) | (Value::Vec(v), Value::Undefined) => {
                Res::V(Value::Vec(v))
            } // [] - v = v, v - [] = v
            (Value::Undefined, Value::Obj(o)) | (Value::Obj(o), Value::Undefined) => {
                Res::V(Value::Obj(o))
            } // {} - o = o, o - {} = o

            // — Vec element‑wise subtraction
            /* (Value::Vec(a), Value::Vec(b)) => {
                if a.len() != b.len() {
                    return Res::E(format!(
                        "Cannot subtract Vec(len={}) - Vec(len={}) due to length mismatch",
                        a.len(),
                        b.len(),
                    ));
                }
                let diff = a
                    .into_iter()
                    .zip(b)
                    .map(|(x, y)| {
                        match x.sub(y) {
                            Res::V(v) => v,
                            Res::E(e) => return Value::Undefined, // placeholder
                        }
                    })
                    .collect();
                Res::V(Value::Vec(diff)) // element‑wise via zip/map :contentReference[oaicite:5]{index=5} :contentReference[oaicite:6]{index=6}
            } */
            // — Obj key‑removal: drop any key in `b` from `a`
            (Value::Obj(mut a), Value::Obj(b)) => {
                for key in b.keys() {
                    a.remove(key); // HashMap::remove :contentReference[oaicite:7]{index=7}
                }
                Res::V(Value::Obj(a))
            }

            // — All other combinations are unsupported
            (lhs, rhs) => Res::E(format!(
                "Subtraction not supported: {} - {}",
                lhs.type_name(),
                rhs.type_name(),
            )),
        }
    }
}

impl<'a> Mul for Value<'a> {
    type Output = Res<'a>;

    fn mul(self, other: Self) -> Res<'a> {
        match (self, other) {
            // — Num × Num
            (Value::Num(a), Value::Num(b)) => Res::V(Value::Num(a * b)),

            // — Str × Num or Num × Str: repeat string n times
            (Value::Str(s), Value::Num(n)) => {
                let count = if n >= 0.0 { n as usize } else { 0 };
                Res::V(Value::Str(s.repeat(count)))
            }
            (Value::Num(n), Value::Str(s)) => {
                let count = if n >= 0.0 { n as usize } else { 0 };
                Res::V(Value::Str(s.repeat(count)))
            }

            // — Undefined as multiplicative identity
            (Value::Undefined, Value::Undefined) => Res::V(Value::Undefined),
            (Value::Undefined, rhs) => Res::V(rhs),
            (lhs, Value::Undefined) => Res::V(lhs),

            // — Vec × Vec: element‑wise (Hadamard) product
            /* (Value::Vec(a), Value::Vec(b)) => {
                if a.len() != b.len() {
                    return Res::E(format!(
                        "Cannot multiply Vec(len={}) × Vec(len={}): length mismatch",
                        a.len(),
                        b.len(),
                    ));
                }
                let prod = a
                    .into_iter()
                    .zip(b)
                    .map(|(x, y)| match x.mul(y) {
                        Res::V(v) => v,
                        Res::E(e) => return Value::Undefined, // will be ignored
                    })
                    .collect();
                Res::V(Value::Vec(prod))
            } */

            // — Vec × Num or Num × Vec: scalar multiply each element
            /* (Value::Vec(v), Value::Num(n)) | (Value::Num(n), Value::Vec(v)) => {
                let scaled = v
                    .into_iter()
                    .map(|x| match x.mul(Value::Num(n)) {
                        Res::V(v2) => v2,
                        Res::E(_) => Value::Undefined,
                    })
                    .collect();
                Res::V(Value::Vec(scaled))
            } */
            // — All other cases unsupported
            (lhs, rhs) => Res::E(format!(
                "Multiplication not supported: {} × {}",
                lhs.type_name(),
                rhs.type_name(),
            )),
        }
    }
}

impl<'a> Div for Value<'a> {
    type Output = Res<'a>;

    fn div(self, other: Self) -> Res<'a> {
        match (self, other) {
            // — Num ÷ Num, with zero‑check
            (Value::Num(a), Value::Num(b)) => {
                if b == 0.0 {
                    return Res::E("Division by zero".into()); // clear runtime error
                }
                Res::V(Value::Num(a / b))
            }

            // — Undefined as division‑identity
            (Value::Undefined, Value::Undefined) => Res::V(Value::Undefined),
            (lhs, Value::Undefined) => Res::V(lhs), // X ÷ Undefined = X
            (Value::Undefined, _) => Res::V(Value::Undefined), // Undefined ÷ X = Undefined

            // — Vec ÷ Vec: element‑wise, require equal length
            /* (Value::Vec(a), Value::Vec(b)) => {
                if a.len() != b.len() {
                    return Res::E(format!(
                        "Cannot divide Vec(len={}) ÷ Vec(len={}): length mismatch",
                        a.len(),
                        b.len(),
                    ));
                }
                let divided = a
                    .into_iter()
                    .zip(b)
                    .map(|(x, y)| match x.div(y) {
                        Res::V(v) => v,
                        Res::E(_) => Value::Undefined, // propagate inner error as Undefined
                    })
                    .collect();
                Res::V(Value::Vec(divided))
            } */

            // — Vec ÷ Num: scalar divide each element
            /* (Value::Vec(v), Value::Num(n)) => {
                if n == 0.0 {
                    return Res::E("Division by zero");
                }
                let scaled = v
                    .into_iter()
                    .map(|x| match x.div(Value::Num(n)) {
                        Res::V(v2) => v2,
                        Res::E(_) => Value::Undefined,
                    })
                    .collect();
                Res::V(Value::Vec(scaled))
            } */

            // — Num ÷ Vec: divide scalar by each element
            /* (Value::Num(n), Value::Vec(v)) => {
                let inv = v
                    .into_iter()
                    .map(|x| match Value::Num(n).div(x) {
                        Res::V(v2) => v2,
                        Res::E(_) => Value::Undefined,
                    })
                    .collect();
                Res::V(Value::Vec(inv))
            } */
            // — All other combinations unsupported
            (lhs, rhs) => Res::E(format!(
                "Division not supported: {} ÷ {}",
                lhs.type_name(),
                rhs.type_name(),
            )),
        }
    }
}

impl<'a> Neg for Value<'a> {
    type Output = Res<'a>;

    fn neg(self) -> Res<'a> {
        match self {
            // — Numeric: just negate the f64
            Value::Num(n) => Res::V(Value::Num(-n)),

            // — Undefined stays undefined (acts like 0)
            Value::Undefined => Res::V(Value::Undefined),

            // — Vec: element‑wise negation via map/neg
            /* Value::Vec(v) => {
                let negated = v
                    .into_iter()
                    .map(|elem| match elem.neg() {
                        Res::V(val) => val,
                        Res::E(_) => Value::Undefined,
                    })
                    .collect();
                Res::V(Value::Vec(negated))
            } */
            // — Str and Obj can’t be meaningfully negated
            other @ _ => Res::E(format!("Negation not supported: {}", other.type_name())),
        }
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> PartialOrd for Value<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Numeric comparison
            (Value::Num(a), Value::Num(b)) => a.partial_cmp(b),

            // String lexicographical
            (Value::Str(a), Value::Str(b)) => a.partial_cmp(b),

            // Vec: lexicographic (first non‑equal element decides)
            /* (Value::Vec(a), Value::Vec(b)) => {
                let min_len = a.len().min(b.len());
                for i in 0..min_len {
                    match a[i].partial_cmp(&b[i]) {
                        Some(Ordering::Equal) => continue,
                        non_eq => return non_eq;
                    }
                }
                // all shared elements equal ⇒ shorter vec is “less”
                a.len().partial_cmp(&b.len())
            } */
            // Undefined only compares equal to Undefined
            (Value::Undefined, Value::Undefined) => Some(Ordering::Equal),

            // Disallow ordering between different variants or e.g. Obj
            _ => None,
        }
    }
}
