#[macro_use]
extern crate common;
extern crate bollard;
extern crate futures;

use bollard::container::{
    Config, CreateContainerOptions, HostConfig, LogOutput, LogsOptions, StartContainerOptions,
    StatsOptions, StopContainerOptions,
};
use bollard::Docker;
use common::plugins::runtime::*;
use futures::Future;
use tokio::prelude::*;
use tokio::runtime::*;

#[derive(Debug)]
pub struct ContainerdRuntimePlugin {
    docker: Docker,
    runner: tokio::runtime::Runtime,
}

impl ContainerdRuntimePlugin {
    pub fn new() -> Self {
        let mut doc = Docker::connect_with_local_defaults().unwrap();
        let mut rt = Runtime::new().unwrap();
        return ContainerdRuntimePlugin {
            runner: rt,
            docker: doc,
        };
    }
}

declare_plugin!(ContainerdRuntimePlugin, ContainerdRuntimePlugin::new);

impl RuntimePlugin for ContainerdRuntimePlugin {
    /// The name of the plugin used to identify it.
    fn name(&self) -> String {
        return "docker".to_string();
    }
    /// A callback fired immediately after the plugin is loaded. Usually used
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}

    fn get_features(&self) -> Vec<RuntimeFeatures> {
        return vec![RuntimeFeatures::WorkloadRunner, RuntimeFeatures::Container];
    }
    fn get_version(&self) -> i32 {
        0
    }

    //Required for feature WorkloadRunner
    fn create_workload(
        &self,
        id: String,
        config: &RuntimeConfig,
        options: &Option<SandboxConfig>,
    ) -> Result<String, RuntimeError> {
        return Err(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    fn start_workload(&mut self, id: String) -> Option<RuntimeError> {
        let nginx_config = Config {
            image: Some("nginx"),
            env: Some(vec![]),
            ..Default::default()
        };
        let res = self.runner.block_on(
            self.docker
                .create_container(Some(CreateContainerOptions { name: "nginx" }), nginx_config),
        );
        let results = self.runner.block_on(
            self.docker
                .start_container("nginx", None::<StartContainerOptions<String>>),
        );
        match results {
            Ok(_) => return None,
            Err(_) => return Some(RuntimeError::new(RuntimeErrorType::Unknown)),
        }
        return Some(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    fn stop_workload(&mut self, id: String, timeout: i64) -> Option<RuntimeError> {
        let options = Some(StopContainerOptions { t: timeout });
        let results = self
            .runner
            .block_on(self.docker.stop_container("nginx", options));
        match results {
            Ok(_) => return None,
            Err(_) => return Some(RuntimeError::new(RuntimeErrorType::Unknown)),
        }
    }

    fn remove_workload(&mut self, id: String) -> Option<RuntimeError> {
        return None;
    }

    fn status_workload(&mut self, id: String) -> Result<WorkloadStatus, RuntimeError> {
        let res = self.runner.block_on(
            self.docker
                .stats(
                    &id,
                    Some(StatsOptions {
                        stream: true,
                        ..Default::default()
                    }),
                )
                .map(|stat| {
                    println!(
                        "{} - mem total: {:?} | mem usage: {:?}",
                        stat.name, stat.memory_stats.max_usage, stat.memory_stats.usage
                    );
                    WorkloadStatus {
                        current_memory: stat.memory_stats.usage.unwrap(),
                        max_memory: stat.memory_stats.max_usage.unwrap(),
                        workload_status: CurrentWorkloadStatus::Running,
                    }
                })
                .collect(),
        );

        if let Ok(res) = res {
            if res.len() > 0 {
                return Ok(res[0].clone());
            }
        }

        return Err(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    //Required for feature container && vm
    fn update_workload_resources(
        &self,
        id: String,
        rez: WorkloadResources,
    ) -> Option<RuntimeError> {
        return None;
    }

    fn exec_sync(
        &self,
        id: String,
        cmd: &[String],
        timeout: i32,
    ) -> (&[u8], &[u8], Option<RuntimeError>) {
        return (&[0], &[0], None);
    }
}
