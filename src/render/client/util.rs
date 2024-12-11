use crate::RdbcValue;
use std::collections::HashMap;

pub fn extract_map_params(params: &HashMap<String, RdbcValue>) -> HashMap<String, RdbcValue> {
    let mut params_new = HashMap::new();
    for (key, value) in params {
        match value {
            RdbcValue::Object(object_map) => {
                // 递归处理对象类型
                let nested_params = extract_map_params(object_map);
                for (nested_key, nested_value) in nested_params {
                    params_new.insert(format!("{}.{}", key, nested_key), nested_value);
                }
            }
            RdbcValue::Array(array) => {
                // 递归处理数组类型
                let nested_params = extract_vec_params(array);
                for (nested_key, nested_value) in nested_params {
                    params_new.insert(format!("{}.{}", key, nested_key), nested_value);
                }
                params_new.insert(key.clone(), value.clone());
            }
            _ => {
                // 处理基本类型
                params_new.insert(key.clone(), value.clone());
            }
        }
    }
    params_new
}

pub fn extract_vec_params(array: &[RdbcValue]) -> HashMap<String, RdbcValue> {
    let mut params_new = HashMap::new();
    for (index, item) in array.iter().enumerate() {
        match item {
            RdbcValue::Object(object_map) => {
                // 递归处理对象类型
                let nested_params = extract_map_params(object_map);
                for (nested_key, nested_value) in nested_params {
                    params_new.insert(format!("{}.{}", index, nested_key), nested_value);
                }
            }
            RdbcValue::Array(nested_array) => {
                // 递归处理数组类型
                let nested_params = extract_vec_params(nested_array);
                for (nested_key, nested_value) in nested_params {
                    params_new.insert(format!("{}.{}", index, nested_key), nested_value);
                }
            }
            _ => {
                // 处理基本类型
                params_new.insert(index.to_string(), item.clone());
            }
        }
    }
    params_new
}
