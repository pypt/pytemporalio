use pyo3::prelude::*;

use std::collections::HashMap;

use pyo3_chrono;

use crate::protos::{
    WrappedPayload,
    WrappedWorkflowExecution,
    WrappedRetryPolicy,
};


#[pyclass(name = "Start")]
#[derive(Clone)]
pub struct WrappedStart {
    pub workflow_namespace: String,
    pub workflow_type: String,
    pub workflow_execution: Option<WrappedWorkflowExecution>,
    pub activity_type: String,
    pub header_fields: HashMap<String, WrappedPayload>,
    pub input: Vec<WrappedPayload>,
    pub heartbeat_details: Vec<WrappedPayload>,
    pub scheduled_time: Option<u128>,
    pub current_attempt_scheduled_time: Option<u128>,
    pub started_time: Option<u128>,
    pub attempt: i32,
    pub schedule_to_close_timeout: Option<pyo3_chrono::Duration>,
    pub start_to_close_timeout: Option<pyo3_chrono::Duration>,
    pub heartbeat_timeout: Option<pyo3_chrono::Duration>,
    pub retry_policy: Option<WrappedRetryPolicy>,
}

#[pymethods]
impl WrappedStart {}


#[pyclass(name = "Cancel")]
#[derive(Clone)]
pub struct WrappedCancel {
    // FIXME ActivityCancelReason
    pub reason: i32,
}

#[pymethods]
impl WrappedCancel {}


#[pyclass(name = "Variant")]
#[derive(Clone)]
pub struct WrappedVariant {
    pub start: Option<WrappedStart>,
    pub cancel: Option<WrappedCancel>,
}

#[pymethods]
impl WrappedVariant {}


#[pyclass(name = "ActivityTask")]
#[derive(Clone)]
pub struct WrappedActivityTask {
    pub task_token: Vec<u8>,
    pub activity_id: String,
    pub variant: Option<WrappedVariant>,
}

#[pymethods]
impl WrappedActivityTask {}
