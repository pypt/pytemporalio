use std::collections::HashMap;
use std::convert::TryFrom;

use pyo3::prelude::*;
use pyo3_chrono;


use temporal_sdk_core::protos::coresdk::{
    activity_task::{
        Start,
        Cancel,
        ActivityTask,
        activity_task,
    },
    common::{
        WorkflowExecution,
        RetryPolicy,
    },
};

use crate::protos::coresdk::common::{
    WrappedPayload,
    WrappedWorkflowExecution,
    WrappedRetryPolicy,
};

use crate::utils::{
    hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads,
    hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads,
    vec_of_payloads_to_vec_of_wrapped_payloads,
    vec_of_wrapped_payloads_to_vec_of_payloads,
    prost_types_timestamp_to_u128,
    u128_to_prost_types_timestamp,
    prost_duration_to_pyo3_chrono_duration,
    pyo3_chrono_duration_to_prost_duration,
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
impl WrappedStart {
    #[new]
    fn new(workflow_namespace: String,
           workflow_type: String,
           workflow_execution: Option<WrappedWorkflowExecution>,
           activity_type: String,
           header_fields: HashMap<String, WrappedPayload>,
           input: Vec<WrappedPayload>,
           heartbeat_details: Vec<WrappedPayload>,
           scheduled_time: Option<u128>,
           current_attempt_scheduled_time: Option<u128>,
           started_time: Option<u128>,
           attempt: i32,
           schedule_to_close_timeout: Option<pyo3_chrono::Duration>,
           start_to_close_timeout: Option<pyo3_chrono::Duration>,
           heartbeat_timeout: Option<pyo3_chrono::Duration>,
           retry_policy: Option<WrappedRetryPolicy>) -> Self {
        WrappedStart {
            workflow_namespace,
            workflow_type,
            workflow_execution,
            activity_type,
            header_fields,
            input,
            heartbeat_details,
            scheduled_time,
            current_attempt_scheduled_time,
            started_time,
            attempt,
            schedule_to_close_timeout,
            start_to_close_timeout,
            heartbeat_timeout,
            retry_policy,
        }
    }
}


impl TryFrom<Start> for WrappedStart {
    type Error = PyErr;

    fn try_from(i: Start) -> Result<Self, Self::Error> {
        Ok(WrappedStart {
            workflow_namespace: i.workflow_namespace,
            workflow_type: i.workflow_type,
            workflow_execution: match i.workflow_execution {
                None => None,
                Some(workflow_execution) => Some(WrappedWorkflowExecution::from(workflow_execution)),
            },
            activity_type: i.activity_type,
            header_fields: hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(i.header_fields),
            input: vec_of_payloads_to_vec_of_wrapped_payloads(i.input),
            heartbeat_details: vec_of_payloads_to_vec_of_wrapped_payloads(i.heartbeat_details),
            scheduled_time: prost_types_timestamp_to_u128(i.scheduled_time),
            current_attempt_scheduled_time: prost_types_timestamp_to_u128(i.current_attempt_scheduled_time),
            started_time: prost_types_timestamp_to_u128(i.started_time),
            attempt: i.attempt,
            schedule_to_close_timeout: prost_duration_to_pyo3_chrono_duration(i.schedule_to_close_timeout)?,
            start_to_close_timeout: prost_duration_to_pyo3_chrono_duration(i.start_to_close_timeout)?,
            heartbeat_timeout: prost_duration_to_pyo3_chrono_duration(i.heartbeat_timeout)?,
            retry_policy: match i.retry_policy {
                None => None,
                Some(retry_policy) => Some(WrappedRetryPolicy::try_from(retry_policy)?),
            },
        })
    }
}


impl TryFrom<WrappedStart> for Start {
    type Error = PyErr;

    fn try_from(i: WrappedStart) -> Result<Self, Self::Error> {
        Ok(Start {
            workflow_namespace: i.workflow_namespace,
            workflow_type: i.workflow_type,
            workflow_execution: match i.workflow_execution {
                None => None,
                Some(workflow_execution) => Some(WorkflowExecution::from(workflow_execution)),
            },
            activity_type: i.activity_type,
            header_fields: hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(i.header_fields),
            input: vec_of_wrapped_payloads_to_vec_of_payloads(i.input),
            heartbeat_details: vec_of_wrapped_payloads_to_vec_of_payloads(i.heartbeat_details),
            scheduled_time: u128_to_prost_types_timestamp(i.scheduled_time),
            current_attempt_scheduled_time: u128_to_prost_types_timestamp(i.current_attempt_scheduled_time),
            started_time: u128_to_prost_types_timestamp(i.started_time),
            attempt: i.attempt,
            schedule_to_close_timeout: pyo3_chrono_duration_to_prost_duration(i.schedule_to_close_timeout)?,
            start_to_close_timeout: pyo3_chrono_duration_to_prost_duration(i.start_to_close_timeout)?,
            heartbeat_timeout: pyo3_chrono_duration_to_prost_duration(i.heartbeat_timeout)?,
            retry_policy: match i.retry_policy {
                None => None,
                Some(retry_policy) => Some(RetryPolicy::try_from(retry_policy)?),
            },
        })
    }
}

#[pyclass(name = "Cancel")]
#[derive(Clone)]
pub struct WrappedCancel {
    // FIXME ActivityCancelReason
    pub reason: i32,
}

#[pymethods]
impl WrappedCancel {
    #[new]
    fn new(reason: i32) -> Self {
        WrappedCancel {
            reason,
        }
    }
}


impl From<Cancel> for WrappedCancel {
    fn from(i: Cancel) -> Self {
        WrappedCancel {
            reason: i.reason
        }
    }
}


impl From<WrappedCancel> for Cancel {
    fn from(i: WrappedCancel) -> Self {
        Cancel {
            reason: i.reason
        }
    }
}


#[pyclass(name = "Variant")]
#[derive(Clone)]
pub struct WrappedVariant {
    pub start: Option<WrappedStart>,
    pub cancel: Option<WrappedCancel>,
}

#[pymethods]
impl WrappedVariant {
    #[new]
    fn new(start: Option<WrappedStart>,
           cancel: Option<WrappedCancel>) -> Self {
        WrappedVariant {
            start,
            cancel,
        }
    }
}


impl TryFrom<activity_task::Variant> for WrappedVariant {
    type Error = PyErr;

    fn try_from(i: activity_task::Variant) -> Result<Self, Self::Error> {
        Ok(match i {
            activity_task::Variant::Start(task) => {
                WrappedVariant {
                    start: Some(WrappedStart::try_from(task)?),
                    cancel: None,
                }
            }
            activity_task::Variant::Cancel(task) => {
                WrappedVariant {
                    start: None,
                    cancel: Some(WrappedCancel::from(task)),
                }
            }
        })
    }
}

impl TryFrom<WrappedVariant> for activity_task::Variant {
    type Error = PyErr;

    fn try_from(i: WrappedVariant) -> Result<Self, Self::Error> {
        Ok(
            if let Some(start) = i.start {
                activity_task::Variant::Start(Start::try_from(start)?)
            } else if let Some(cancel) = i.cancel {
                activity_task::Variant::Cancel(Cancel::try_from(cancel)?)
            } else {
                panic!("Only one of 'start' and 'cancel' must be set")
            }
        )
    }
}


#[pyclass(name = "ActivityTask")]
#[derive(Clone)]
pub struct WrappedActivityTask {
    pub task_token: Vec<u8>,
    pub activity_id: String,
    pub variant: Option<WrappedVariant>,
}

#[pymethods]
impl WrappedActivityTask {
    #[new]
    fn new(task_token: Vec<u8>,
           activity_id: String,
           variant: Option<WrappedVariant>) -> Self {
        WrappedActivityTask {
            task_token,
            activity_id,
            variant,
        }
    }
}


impl TryFrom<ActivityTask> for WrappedActivityTask {
    type Error = PyErr;

    fn try_from(i: ActivityTask) -> Result<Self, Self::Error> {
        Ok(WrappedActivityTask {
            task_token: i.task_token,
            activity_id: i.activity_id,
            variant: match i.variant {
                None => None,
                Some(variant) => Some(WrappedVariant::try_from(variant)?),
            },
        })
    }
}

impl TryFrom<WrappedActivityTask> for ActivityTask {
    type Error = PyErr;

    fn try_from(i: WrappedActivityTask) -> Result<Self, Self::Error> {
        Ok(ActivityTask {
            task_token: i.task_token,
            activity_id: i.activity_id,
            variant: match i.variant {
                None => None,
                Some(variant) => Some(activity_task::Variant::try_from(variant)?),
            },
        })
    }
}
