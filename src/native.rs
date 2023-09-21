use std::time::{SystemTime, UNIX_EPOCH};
use crate::gc::Gc;

use crate::value::Value;

pub fn clock_native(_arg_count: usize, _values: &[Value], _gc: &mut Gc) -> Value {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        .into()
}

pub fn list_append(_arg_count: usize, values: &[Value], gc: &mut Gc) -> Value {
    if let Value::RoxList(list) = values[0] {
        let vec: &mut Vec<Value> = gc.deref_mut(list);
        for &elem in values[1..].into_iter() {
            vec.push(elem);
        }
        Value::RoxList(list)
    } else {
        panic!("Append target is not List");
    }
}
