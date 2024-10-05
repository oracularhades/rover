use serde_json::Value;

pub fn merge(a: &Value, b: &Value) -> Value {
    let mut base = a.clone();
    if let (Some(a_map), Some(b_map)) = (base.as_object_mut(), b.as_object()) {
        a_map.extend(b_map.clone().into_iter());
    }

    return base;
}