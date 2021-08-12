// FIXME move to coresdk

use pyo3::prelude::*;

use temporal_sdk_core::protos::coresdk::{
    ActivityTaskCompletion,
    activity_result::ActivityResult,
};

use activity_result::WrappedActivityResult;

pub(crate) mod activity_result;
pub(crate) mod activity_task;
pub(crate) mod common;
pub(crate) mod workflow_activation;
pub(crate) mod workflow_commands;
pub(crate) mod workflow_completion;

#[pyclass(name = "ActivityTaskCompletion")]
#[derive(Clone)]
pub struct WrappedActivityTaskCompletion {
    pub task_token: Vec<u8>,
    pub task_queue: String,
    pub result: Option<WrappedActivityResult>,
}

#[pymethods]
impl WrappedActivityTaskCompletion {
    #[new]
    fn new(task_token: Vec<u8>,
           task_queue: String,
           result: Option<activity_result::WrappedActivityResult>) -> Self {
        WrappedActivityTaskCompletion {
            task_token,
            task_queue,
            result,
        }
    }
}

impl From<ActivityTaskCompletion> for WrappedActivityTaskCompletion {
    fn from(i: ActivityTaskCompletion) -> Self {
        WrappedActivityTaskCompletion {
            task_token: i.task_token,
            task_queue: i.task_queue,
            result: match i.result {
                None => None,
                Some(result) => Some(WrappedActivityResult::from(result)),
            },
        }
    }
}

impl From<WrappedActivityTaskCompletion> for ActivityTaskCompletion {
    fn from(i: WrappedActivityTaskCompletion) -> Self {
        ActivityTaskCompletion {
            task_token: i.task_token,
            task_queue: i.task_queue,
            result: match i.result {
                None => None,
                Some(result) => Some(ActivityResult::from(result)),
            },
        }
    }
}
