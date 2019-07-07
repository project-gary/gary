use crate::config::ClusterConfig;
use serde_yaml::Value;

pub fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Mapping(ref mut a), &Value::Mapping(ref b)) => {
            for (k, v) in b {
                merge(a.get_mut(&k.clone()).unwrap(), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub fn get_config_or_default(contents: String) -> ClusterConfig {
    let mut result = serde_yaml::to_value(&contents).unwrap();
    let input = serde_yaml::to_value(&ClusterConfig::new_default()).unwrap();
    merge(&mut result, &input);
    return serde_yaml::from_value(result).unwrap();
}
