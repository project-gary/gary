#[cfg(test)]
pub mod tests {

    use crate::config::*;
    use crate::data::*;
    use crate::yaml::*;

    use std::collections::HashMap;

    #[test]
    fn test_merge() {
        let contents = include_str!("./test_config.yaml");
        let input: serde_yaml::Value = serde_yaml::from_str(&contents).unwrap();
        let mut result = serde_yaml::to_value(&ClusterConfig::new_default()).unwrap();
        merge(&mut result, &input);
        let mut other_result = ClusterConfig::new_default();

        other_result.gossip_config.interval = 6;
        other_result.gossip_config.port = 9876;
        other_result.deployment_config.port = 991122;
        other_result.node_info.node_name = "Bobby".to_string();

        let a: ClusterConfig = serde_yaml::from_value(result).unwrap();
        assert_eq!(a, other_result);
        assert_ne!(a, ClusterConfig::new_default());
    }

    #[test]
    fn read_deployment_test() {
        let contents = include_str!("./test_deploy.yaml");
        /*fs::read_to_string("./tests/test_deploy.yaml")
            .expect("something went wrong reading the file");
        */
        let mut deployment = Deployment {
            version: "apps/v1".to_string(),
            kind: "Deployment".to_string(),
            metadata: MetaData {
                name: Some("nginx-deployment".to_string()),
                labels: HashMap::new(),
            },
            spec: DeploymentSpec {
                replicas: 3,
                template: DeploymentTemplate {
                    metadata: MetaData {
                        name: None,
                        labels: HashMap::new(),
                    },
                    spec: Spec {
                        containers: Vec::new(),
                    },
                },
            },
        };

        deployment
            .metadata
            .labels
            .insert("app".to_string(), "nginx".to_string());
        deployment
            .spec
            .template
            .metadata
            .labels
            .insert("app".to_string(), "nginx".to_string());
        let container = Container {
            name: "nginx".to_string(),
            image: "nginx:1.7.9".to_string(),
            ports: vec![Ports { container_port: 80 }],
        };
        deployment
            .spec
            .template
            .spec
            .containers
            .insert(0, container);

        let file_deploy: Deployment = serde_yaml::from_str(&contents).expect("not to fail");
        assert_eq!(deployment, file_deploy);
    }
}
