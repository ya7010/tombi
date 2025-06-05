use crate::{Value, ValueArena, ValueId};

/// Convert a tombi-json-arena Value to serde_json::Value
pub fn to_value(value_id: &ValueId, arena: &ValueArena) -> serde_json::Value {
    match arena.get(value_id).expect("Invalid ValueId") {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => serde_json::Value::Number(
            serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        Value::String(str_id) => {
            let s = arena.str_arena().get(*str_id).unwrap_or("");
            serde_json::Value::String(s.to_string())
        }
        Value::Array(array_id) => {
            let arr = arena.array_arena().get(*array_id).unwrap();
            serde_json::Value::Array(arr.iter().map(|vid| to_value(vid, arena)).collect())
        }
        Value::Object(obj_id) => {
            let obj = arena.object_arena().get(*obj_id).unwrap();
            let mut map = serde_json::Map::new();
            for (k, v) in obj.iter() {
                let key = arena.str_arena().get(*k).unwrap_or("");
                map.insert(key.to_string(), to_value(v, arena));
            }
            serde_json::Value::Object(map)
        }
    }
}
