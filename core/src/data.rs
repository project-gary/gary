use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JoinCluster {
    pub machine_info: MachineInfo,
}

/*
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.7.9
        ports:
        - containerPort: 80
 * */

pub enum DeploymentCommand {
  // name, count, executor, params
  NewDeploy(Deployment),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeploymentType {
  Process,
  Docker,
}

#[derive(Debug, PartialEq)]
pub enum DeploymentReply {
  // name, result
  NewDeploy(String, Result<String, String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Spec {
  ProcessSpec(ProcessSpec),
  DockerSpec(DockerSpec),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[serde(alias = "apiVersion")]
    pub version: String,
    pub kind: DeploymentType,
    pub metadata: Metadata,
    pub spec: DeploymentSpec,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentSpec {
    pub replicas: usize,
    pub template: DeploymentTemplate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentTemplate {
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerSpec {
    pub containers: Vec<Container>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessSpec {
    pub cmd: String, // I'd much rather use &str here, but lifetimes...
    pub args: Option<String>, //TODO: this should be an iterable

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub ports: Vec<Ports>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    #[serde(alias = "containerPort")]
    pub container_port: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineInfo {
    pub fqdn: String,
    pub tags: Vec<String>,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_cpu: i64,
    pub used_cpu: i64,
    pub disk_avlible: i64,
}
