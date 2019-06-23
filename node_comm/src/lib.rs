use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait ClusterCommunication {
    /*
     * Cluster management
     * */
    fn join_cluster(info: MachineInfo);
    fn gossip(gossip: Vec<MachineInfo>);
    fn sync_request();
    fn sync_response(info: MachineInfo);

    /*
     * Deployment management
     * */
    fn request_run_deployment(deployment: Deployment);
}

pub trait ClusterApi {
    fn my_fn();
}

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub version: String,
    pub kind: String,
    pub metadata: MetaData,
    pub spec: DeploymentSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentSpec {
    pub replicas: i32,
    pub template: DeploymentTemplate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentTemplate {
    pub metadata: MetaData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    pub containers: Vec<Containers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Containers {
    pub name: String,
    pub image: String,
    pub ports: Ports,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ports {
    pub container_ports: String;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineInfo {
    pub fqdn: String,
    pub tags: Vec<String>,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_cpu: i64,
    pub used_cpu: i64,
    pub disk_avlible: i64,
}
