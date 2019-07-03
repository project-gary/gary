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
        let mut deployment = Deployment {
            version: "apps/v1".to_string(),
            kind: DeploymentType::Docker,
            metadata: Metadata {
                name: Some("nginx-deployment".to_string()),
                labels: Some(HashMap::new()),
            },
            spec: DeploymentSpec {
                replicas: 3,
                template: DeploymentTemplate {
                    metadata: Metadata {
                        name: None,
                        labels: Some(HashMap::new()),
                    },
                    spec: Spec::DockerSpec(DockerSpec {
                        containers: Vec::new(),
                    }),
                },
            },
        };

        //deployment
        //    .metadata
        //    .labels.unwrap()
        //    .insert("app".to_string(), "nginx".to_string());
        deployment
            .metadata
            .labels
            .as_mut()
            .map(|l| l.insert("app".to_string(), "nginx".to_string()));
        deployment
            .spec
            .template
            .metadata
            .labels
            .as_mut()
            .map(|l| l.insert("app".to_string(), "nginx".to_string()));
        if let Spec::DockerSpec(ref mut spec) = deployment.spec.template.spec {
            let container = Container {
                name: "nginx".to_string(),
                image: "nginx:1.7.9".to_string(),
                ports: vec![Ports { container_port: 80 }],
            };
            spec.containers.insert(0, container);
        } //TODO: else?

        let file_deploy: Deployment = serde_yaml::from_str(&contents).expect("not to fail");
        assert_eq!(deployment, file_deploy);
    }
}
