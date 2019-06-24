#[cfg(test)]
pub mod tests {

use crate::data::*;
use std::collections::HashMap;

    #[test]
    fn read_deployment_test() {

        let contents = include_str!("./test_deploy.yaml");
        /*fs::read_to_string("./tests/test_deploy.yaml")
            .expect("something went wrong reading the file");
        */
        let mut deployment = Deployment{
            version: "apps/v1".to_string(),
            kind: "Deployment".to_string(),
            metadata: MetaData{
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
                    spec: Spec{
                        containers: Vec::new()
                    },
                },
            },
        };

        deployment.metadata.labels.insert("app".to_string(), "nginx".to_string());
        deployment.spec.template.metadata.labels.insert("app".to_string(), "nginx".to_string());
        let container = Container {
            name: "nginx".to_string(),
            image: "nginx:1.7.9".to_string(),
            ports: vec!(Ports{container_port: 80}),
        };
        deployment.spec.template.spec.containers.insert(0, container);

        let file_deploy : Deployment = serde_yaml::from_str(&contents)
            .expect("not to fail");
        assert_eq!(deployment, file_deploy );
    }
}
