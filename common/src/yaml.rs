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
