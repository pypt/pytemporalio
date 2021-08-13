use std::collections::HashMap;
use std::convert::TryFrom;

use pyo3::prelude::*;
use pyo3_chrono;

use temporal_sdk_core::protos::coresdk::common::{
    Payload,
    WorkflowExecution,
    UserCodeFailure,
    RetryPolicy,
};

use crate::utils::{
    prost_duration_to_pyo3_chrono_duration,
    pyo3_chrono_duration_to_prost_duration,
};


#[pyclass(name = "UserCodeFailure")]
#[derive(Clone)]
pub struct WrappedUserCodeFailure {
    pub message: String,
    pub r#type: String,
    pub source: String,
    pub stack_trace: String,
    pub non_retryable: bool,
    pub cause: Option<Box<WrappedUserCodeFailure>>,
}

#[pymethods]
impl WrappedUserCodeFailure {
    #[new]
    fn new(message: String,
           r#type: String,
           source: String,
           stack_trace: String,
           non_retryable: bool,
           cause: Option<WrappedUserCodeFailure>) -> Self {
        WrappedUserCodeFailure {
            message,
            r#type,
            source,
            stack_trace,
            non_retryable,
            cause: match cause {
                None => None,
                Some(cause) => Some(Box::new(cause)),
            },
        }
    }
}


impl From<UserCodeFailure> for WrappedUserCodeFailure {
    fn from(i: UserCodeFailure) -> Self {
        WrappedUserCodeFailure {
            message: i.message,
            r#type: i.r#type,
            source: i.source,
            stack_trace: i.stack_trace,
            non_retryable: i.non_retryable,
            cause: match i.cause {
                None => None,
                Some(cause) => Some(Box::new(WrappedUserCodeFailure::from(*cause))),
            },
        }
    }
}

impl From<WrappedUserCodeFailure> for UserCodeFailure {
    fn from(i: WrappedUserCodeFailure) -> Self {
        UserCodeFailure {
            message: i.message,
            r#type: i.r#type,
            source: i.source,
            stack_trace: i.stack_trace,
            non_retryable: i.non_retryable,
            cause: match i.cause {
                None => None,
                Some(cause) => Some(Box::new(UserCodeFailure::from(*cause))),
            },
        }
    }
}


#[pyclass(name = "Payload")]
#[derive(Clone)]
pub struct WrappedPayload {
    pub metadata: HashMap<String, Vec<u8>>,
    pub data: Vec<u8>,
}

#[pymethods]
impl WrappedPayload {
    #[new]
    fn new(metadata: HashMap<String, Vec<u8>>,
           data: Vec<u8>) -> Self {
        WrappedPayload {
            metadata,
            data,
        }
    }
}

impl From<Payload> for WrappedPayload {
    fn from(i: Payload) -> Self {
        WrappedPayload {
            metadata: i.metadata.clone(),
            data: i.data.clone(),
        }
    }
}


impl From<&Payload> for WrappedPayload {
    fn from(i: &Payload) -> Self {
        WrappedPayload {
            metadata: i.metadata.clone(),
            data: i.data.clone(),
        }
    }
}

impl From<WrappedPayload> for Payload {
    fn from(i: WrappedPayload) -> Self {
        Payload {
            metadata: i.metadata.clone(),
            data: i.data.clone(),
        }
    }
}

impl From<&WrappedPayload> for Payload {
    fn from(i: &WrappedPayload) -> Self {
        Payload {
            metadata: i.metadata.clone(),
            data: i.data.clone(),
        }
    }
}


#[pyclass(name = "WorkflowExecution")]
#[derive(Clone)]
pub struct WrappedWorkflowExecution {
    pub workflow_id: String,
    pub run_id: String,
}

#[pymethods]
impl WrappedWorkflowExecution {
    #[new]
    fn new(workflow_id: String, run_id: String) -> Self {
        WrappedWorkflowExecution {
            workflow_id,
            run_id,
        }
    }
}


impl From<WorkflowExecution> for WrappedWorkflowExecution {
    fn from(i: WorkflowExecution) -> Self {
        WrappedWorkflowExecution {
            workflow_id: i.workflow_id,
            run_id: i.run_id,
        }
    }
}

impl From<WrappedWorkflowExecution> for WorkflowExecution {
    fn from(i: WrappedWorkflowExecution) -> Self {
        WorkflowExecution {
            workflow_id: i.workflow_id,
            run_id: i.run_id,
        }
    }
}


#[pyclass(name = "RetryPolicy")]
#[derive(Clone)]
pub struct WrappedRetryPolicy {
    pub initial_interval: Option<pyo3_chrono::Duration>,
    pub backoff_coefficient: f64,
    pub maximum_interval: Option<pyo3_chrono::Duration>,
    pub maximum_attempts: i32,
    pub non_retryable_error_types: Vec<String>,
}

#[pymethods]
impl WrappedRetryPolicy {
    #[new]
    fn new(initial_interval: Option<pyo3_chrono::Duration>,
           backoff_coefficient: f64,
           maximum_interval: Option<pyo3_chrono::Duration>,
           maximum_attempts: i32,
           non_retryable_error_types: Vec<String>) -> Self {
        WrappedRetryPolicy {
            initial_interval,
            backoff_coefficient,
            maximum_interval,
            maximum_attempts,
            non_retryable_error_types,
        }
    }
}

impl TryFrom<RetryPolicy> for WrappedRetryPolicy {
    type Error = PyErr;

    fn try_from(i: RetryPolicy) -> Result<Self, Self::Error> {
        Ok(WrappedRetryPolicy {
            initial_interval: prost_duration_to_pyo3_chrono_duration(i.initial_interval)?,
            backoff_coefficient: i.backoff_coefficient,
            maximum_interval: prost_duration_to_pyo3_chrono_duration(i.maximum_interval)?,
            maximum_attempts: i.maximum_attempts,
            non_retryable_error_types: i.non_retryable_error_types,
        })
    }
}

impl TryFrom<WrappedRetryPolicy> for RetryPolicy {
    type Error = PyErr;

    fn try_from(i: WrappedRetryPolicy) -> Result<Self, Self::Error> {
        Ok(RetryPolicy {
            initial_interval: pyo3_chrono_duration_to_prost_duration(i.initial_interval)?,
            backoff_coefficient: i.backoff_coefficient,
            maximum_interval: pyo3_chrono_duration_to_prost_duration(i.maximum_interval)?,
            maximum_attempts: i.maximum_attempts,
            non_retryable_error_types: i.non_retryable_error_types,
        })
    }
}
