use std::convert::TryFrom;

use pyo3::prelude::*;
use pyo3_chrono;
use temporal_sdk_core::WorkerConfig;

use crate::utils::{
    std_duration_to_pyo3_chrono_duration,
    pyo3_chrono_duration_to_std_duration,
};

#[pyclass(name = "WorkerConfig")]
#[derive(Clone)]
pub struct WrappedWorkerConfig {
    pub task_queue: String,
    pub max_outstanding_workflow_tasks: usize,
    pub max_outstanding_activities: usize,
    pub max_concurrent_wft_polls: usize,
    pub nonsticky_to_sticky_poll_ratio: f32,
    pub max_concurrent_at_polls: usize,
    pub no_remote_activities: bool,
    pub sticky_queue_schedule_to_start_timeout: pyo3_chrono::Duration,
}


#[pymethods]
impl WrappedWorkerConfig {
    // FIXME defaults
    #[new]
    fn new(task_queue: String,
           max_outstanding_workflow_tasks: usize,
           max_outstanding_activities: usize,
           max_concurrent_wft_polls: usize,
           nonsticky_to_sticky_poll_ratio: f32,
           max_concurrent_at_polls: usize,
           no_remote_activities: bool,
           sticky_queue_schedule_to_start_timeout: pyo3_chrono::Duration) -> Self {
        WrappedWorkerConfig {
            task_queue,
            max_outstanding_workflow_tasks,
            max_outstanding_activities,
            max_concurrent_wft_polls,
            nonsticky_to_sticky_poll_ratio,
            max_concurrent_at_polls,
            no_remote_activities,
            sticky_queue_schedule_to_start_timeout,
        }
    }
}

impl TryFrom<WorkerConfig> for WrappedWorkerConfig {
    type Error = PyErr;

    fn try_from(i: WorkerConfig) -> Result<Self, Self::Error> {
        Ok(WrappedWorkerConfig {
            task_queue: i.task_queue,
            max_outstanding_workflow_tasks: i.max_outstanding_workflow_tasks,
            max_outstanding_activities: i.max_outstanding_activities,
            max_concurrent_wft_polls: i.max_concurrent_wft_polls,
            nonsticky_to_sticky_poll_ratio: i.nonsticky_to_sticky_poll_ratio,
            max_concurrent_at_polls: i.max_concurrent_at_polls,
            no_remote_activities: i.no_remote_activities,
            sticky_queue_schedule_to_start_timeout: std_duration_to_pyo3_chrono_duration(i.sticky_queue_schedule_to_start_timeout)?,
        })
    }
}


impl TryFrom<WrappedWorkerConfig> for WorkerConfig {
    type Error = PyErr;

    fn try_from(i: WrappedWorkerConfig) -> Result<Self, Self::Error> {
        Ok(WorkerConfig {
            task_queue: i.task_queue,
            max_outstanding_workflow_tasks: i.max_outstanding_workflow_tasks,
            max_outstanding_activities: i.max_outstanding_activities,
            max_concurrent_wft_polls: i.max_concurrent_wft_polls,
            nonsticky_to_sticky_poll_ratio: i.nonsticky_to_sticky_poll_ratio,
            max_concurrent_at_polls: i.max_concurrent_at_polls,
            no_remote_activities: i.no_remote_activities,
            sticky_queue_schedule_to_start_timeout: pyo3_chrono_duration_to_std_duration(i.sticky_queue_schedule_to_start_timeout)?,
        })
    }
}
