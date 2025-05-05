use super::super::types::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment<'a> {
    values: HashMap<&'a str, Value<'a>>,
    parent: Option<EnvPtr<'a>>,
}
pub type EnvPtr<'a> = Rc<RefCell<Environment<'a>>>;

pub fn new<'a>(parent: Option<EnvPtr<'a>>) -> EnvPtr<'a> {
    Rc::new(RefCell::new(Environment {
        values: HashMap::new(),
        parent,
    }))
}

pub fn get<'a>(env: EnvPtr<'a>, name: &str) -> Option<Value<'a>> {
    if let Some(val) = env.borrow().values.get(name) {
        Some(val.clone())
    } else if let Some(parent) = env.borrow().parent.clone() {
        get(parent, name)
    } else {
        None
    }
}

pub fn set<'a>(env: &EnvPtr<'a>, name: &'a str, value: Value<'a>) -> Option<String> {
    if env.borrow().values.contains_key(name) {
        env.borrow_mut().values.insert(name, value);
        None
    } else if let Some(ref parent) = env.borrow().parent {
        set(parent, name, value)
    } else {
        Some(format!("Undefined variable '{}'.", name))
    }
}

pub fn define<'a>(env: EnvPtr<'a>, name: &'a str, value: Value<'a>) -> Option<String> {
    if env.borrow().values.get(&name).is_none() {
        env.borrow_mut().values.insert(name, value);
        None
    } else {
        Some(format!("Variable '{}' is already declared.", name))
    }
}
