use pyo3::prelude::*;

use std::collections::HashMap;

use pyo3_chrono;

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
impl WrappedUserCodeFailure {}


#[pyclass(name = "Payload")]
#[derive(Clone)]
pub struct WrappedPayload {
    pub metadata: HashMap<String, Vec<u8>>,
    pub data: Vec<u8>,
}

#[pymethods]
impl WrappedPayload {}


#[pyclass(name = "WorkflowExecution")]
#[derive(Clone)]
pub struct WrappedWorkflowExecution {
    pub workflow_id: String,
    pub run_id: String,
}

#[pymethods]
impl WrappedWorkflowExecution {}


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
impl WrappedRetryPolicy {}
