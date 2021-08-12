use std::convert::TryFrom;
use std::collections::HashMap;

use pyo3::prelude::*;

use temporal_sdk_core::protos::coresdk::{
    common::{
        RetryPolicy,
        Payload,
        UserCodeFailure,
    },
    workflow_commands::{
        workflow_command,
        StartTimer,
        CancelTimer,
        ScheduleActivity,
        RequestCancelActivity,
        QuerySuccess,
        query_result,
        QueryResult,
        CompleteWorkflowExecution,
        FailWorkflowExecution,
        ContinueAsNewWorkflowExecution,
        CancelWorkflowExecution,
        WorkflowCommand,
    },
};


use crate::protos::common::{
    WrappedPayload,
    WrappedRetryPolicy,
    WrappedUserCodeFailure,
};

use crate::utils::{
    prost_duration_to_pyo3_chrono_duration,
    pyo3_chrono_duration_to_prost_duration,
    vec_of_payloads_to_vec_of_wrapped_payloads,
    vec_of_wrapped_payloads_to_vec_of_payloads,
    hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads,
    hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads,
};


#[pyclass(name = "StartTimer")]
#[derive(Clone)]
pub struct WrappedStartTimer {
    pub timer_id: String,
    pub start_to_fire_timeout: Option<pyo3_chrono::Duration>,
}

#[pymethods]
impl WrappedStartTimer {
    #[new]
    fn new(timer_id: String, start_to_fire_timeout: Option<pyo3_chrono::Duration>) -> Self {
        WrappedStartTimer {
            timer_id,
            start_to_fire_timeout,
        }
    }
}

impl TryFrom<StartTimer> for WrappedStartTimer {
    type Error = PyErr;

    fn try_from(i: StartTimer) -> Result<Self, Self::Error> {
        Ok(WrappedStartTimer {
            timer_id: i.timer_id,
            start_to_fire_timeout: prost_duration_to_pyo3_chrono_duration(i.start_to_fire_timeout)?,
        })
    }
}

impl TryFrom<WrappedStartTimer> for StartTimer {
    type Error = PyErr;

    fn try_from(i: WrappedStartTimer) -> Result<Self, Self::Error> {
        Ok(StartTimer {
            timer_id: i.timer_id,
            start_to_fire_timeout: pyo3_chrono_duration_to_prost_duration(i.start_to_fire_timeout)?,
        })
    }
}


#[pyclass(name = "CancelTimer")]
#[derive(Clone)]
pub struct WrappedCancelTimer {
    pub timer_id: String,
}

#[pymethods]
impl WrappedCancelTimer {
    #[new]
    fn new(timer_id: String) -> Self {
        WrappedCancelTimer {
            timer_id,
        }
    }
}

impl From<CancelTimer> for WrappedCancelTimer {
    fn from(i: CancelTimer) -> Self {
        WrappedCancelTimer {
            timer_id: i.timer_id
        }
    }
}

impl From<WrappedCancelTimer> for CancelTimer {
    fn from(i: WrappedCancelTimer) -> Self {
        CancelTimer {
            timer_id: i.timer_id
        }
    }
}


#[pyclass(name = "ScheduleActivity")]
#[derive(Clone)]
pub struct WrappedScheduleActivity {
    pub activity_id: String,
    pub activity_type: String,
    pub namespace: String,
    pub task_queue: String,
    pub header_fields: HashMap<String, WrappedPayload>,
    pub arguments: Vec<WrappedPayload>,
    pub schedule_to_close_timeout: Option<pyo3_chrono::Duration>,
    pub schedule_to_start_timeout: Option<pyo3_chrono::Duration>,
    pub start_to_close_timeout: Option<pyo3_chrono::Duration>,
    pub heartbeat_timeout: Option<pyo3_chrono::Duration>,
    pub retry_policy: Option<WrappedRetryPolicy>,
    // FIXME enum
    pub cancellation_type: i32,
}

#[pymethods]
impl WrappedScheduleActivity {
    #[new]
    fn new(activity_id: String,
           activity_type: String,
           namespace: String,
           task_queue: String,
           header_fields: HashMap<String, WrappedPayload>,
           arguments: Vec<WrappedPayload>,
           schedule_to_close_timeout: Option<pyo3_chrono::Duration>,
           schedule_to_start_timeout: Option<pyo3_chrono::Duration>,
           start_to_close_timeout: Option<pyo3_chrono::Duration>,
           heartbeat_timeout: Option<pyo3_chrono::Duration>,
           retry_policy: Option<WrappedRetryPolicy>,
           // FIXME enum
           cancellation_type: i32) -> Self {
        WrappedScheduleActivity {
            activity_id,
            activity_type,
            namespace,
            task_queue,
            header_fields,
            arguments,
            schedule_to_close_timeout,
            schedule_to_start_timeout,
            start_to_close_timeout,
            heartbeat_timeout,
            retry_policy,
            cancellation_type,
        }
    }
}

impl TryFrom<ScheduleActivity> for WrappedScheduleActivity {
    type Error = PyErr;

    fn try_from(i: ScheduleActivity) -> Result<Self, Self::Error> {
        Ok(WrappedScheduleActivity {
            activity_id: i.activity_id,
            activity_type: i.activity_type,
            namespace: i.namespace,
            task_queue: i.task_queue,
            header_fields: hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(i.header_fields),
            arguments: vec_of_payloads_to_vec_of_wrapped_payloads(i.arguments),
            schedule_to_close_timeout: prost_duration_to_pyo3_chrono_duration(i.schedule_to_close_timeout)?,
            schedule_to_start_timeout: prost_duration_to_pyo3_chrono_duration(i.schedule_to_start_timeout)?,
            start_to_close_timeout: prost_duration_to_pyo3_chrono_duration(i.start_to_close_timeout)?,
            heartbeat_timeout: prost_duration_to_pyo3_chrono_duration(i.heartbeat_timeout)?,
            retry_policy: match i.retry_policy {
                None => None,
                Some(retry_policy) => Some(WrappedRetryPolicy::try_from(retry_policy)?),
            },
            cancellation_type: i.cancellation_type,
        })
    }
}

impl TryFrom<WrappedScheduleActivity> for ScheduleActivity {
    type Error = PyErr;

    fn try_from(i: WrappedScheduleActivity) -> Result<Self, Self::Error> {
        Ok(ScheduleActivity {
            activity_id: i.activity_id,
            activity_type: i.activity_type,
            namespace: i.namespace,
            task_queue: i.task_queue,
            header_fields: hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(i.header_fields),
            arguments: vec_of_wrapped_payloads_to_vec_of_payloads(i.arguments),
            schedule_to_close_timeout: pyo3_chrono_duration_to_prost_duration(i.schedule_to_close_timeout)?,
            schedule_to_start_timeout: pyo3_chrono_duration_to_prost_duration(i.schedule_to_start_timeout)?,
            start_to_close_timeout: pyo3_chrono_duration_to_prost_duration(i.start_to_close_timeout)?,
            heartbeat_timeout: pyo3_chrono_duration_to_prost_duration(i.heartbeat_timeout)?,
            retry_policy: match i.retry_policy {
                None => None,
                Some(retry_policy) => Some(RetryPolicy::try_from(retry_policy)?),
            },
            cancellation_type: i.cancellation_type,
        })
    }
}


#[pyclass(name = "RequestCancelActivity")]
#[derive(Clone)]
pub struct WrappedRequestCancelActivity {
    pub activity_id: String,
}

#[pymethods]
impl WrappedRequestCancelActivity {
    #[new]
    fn new(activity_id: String) -> Self {
        WrappedRequestCancelActivity {
            activity_id,
        }
    }
}

impl From<RequestCancelActivity> for WrappedRequestCancelActivity {
    fn from(i: RequestCancelActivity) -> Self {
        WrappedRequestCancelActivity {
            activity_id: i.activity_id,
        }
    }
}

impl From<WrappedRequestCancelActivity> for RequestCancelActivity {
    fn from(i: WrappedRequestCancelActivity) -> Self {
        RequestCancelActivity {
            activity_id: i.activity_id,
        }
    }
}


#[pyclass(name = "QuerySuccess")]
#[derive(Clone)]
pub struct WrappedQuerySuccess {
    pub response: Option<WrappedPayload>,
}

#[pymethods]
impl WrappedQuerySuccess {
    #[new]
    fn new(response: Option<WrappedPayload>) -> Self {
        WrappedQuerySuccess {
            response,
        }
    }
}

impl From<QuerySuccess> for WrappedQuerySuccess {
    fn from(i: QuerySuccess) -> Self {
        WrappedQuerySuccess {
            response: match i.response {
                None => None,
                Some(payload) => Some(WrappedPayload::from(payload)),
            }
        }
    }
}

impl From<WrappedQuerySuccess> for QuerySuccess {
    fn from(i: WrappedQuerySuccess) -> Self {
        QuerySuccess {
            response: match i.response {
                None => None,
                Some(payload) => Some(Payload::from(payload)),
            }
        }
    }
}


// FIXME called just "Variant" in the original protobuf
#[pyclass(name = "QueryResultVariant")]
#[derive(Clone)]
pub struct WrappedQueryResultVariant {
    pub succeeded: Option<WrappedQuerySuccess>,
    pub failed: Option<WrappedUserCodeFailure>,
}

#[pymethods]
impl WrappedQueryResultVariant {
    #[new]
    fn new(succeeded: Option<WrappedQuerySuccess>,
           failed: Option<WrappedUserCodeFailure>) -> Self {
        WrappedQueryResultVariant {
            succeeded,
            failed,
        }
    }
}

impl From<query_result::Variant> for WrappedQueryResultVariant {
    fn from(i: query_result::Variant) -> Self {
        match i {
            query_result::Variant::Succeeded(success) => WrappedQueryResultVariant {
                succeeded: Some(WrappedQuerySuccess::from(success)),
                failed: None,
            },
            query_result::Variant::Failed(failure) => WrappedQueryResultVariant {
                succeeded: None,
                failed: Some(WrappedUserCodeFailure::from(failure)),
            }
        }
    }
}

impl From<WrappedQueryResultVariant> for query_result::Variant {
    fn from(i: WrappedQueryResultVariant) -> Self {
        if let Some(success) = i.succeeded {
            query_result::Variant::Succeeded(QuerySuccess::from(success))
        } else if let Some(failure) = i.failed {
            query_result::Variant::Failed(UserCodeFailure::from(failure))
        } else {
            panic!("Only one of query result variants must be set");
        }
    }
}


#[pyclass(name = "QueryResult")]
#[derive(Clone)]
pub struct WrappedQueryResult {
    pub query_id: String,
    pub variant: Option<WrappedQueryResultVariant>,
}

#[pymethods]
impl WrappedQueryResult {
    #[new]
    fn new(query_id: String,
           variant: Option<WrappedQueryResultVariant>) -> Self {
        WrappedQueryResult {
            query_id,
            variant,
        }
    }
}

impl From<QueryResult> for WrappedQueryResult {
    fn from(i: QueryResult) -> Self {
        WrappedQueryResult {
            query_id: i.query_id,
            variant: match i.variant {
                None => None,
                Some(variant) => Some(WrappedQueryResultVariant::from(variant)),
            },
        }
    }
}

impl From<WrappedQueryResult> for QueryResult {
    fn from(i: WrappedQueryResult) -> Self {
        QueryResult {
            query_id: i.query_id,
            variant: match i.variant {
                None => None,
                Some(variant) => Some(query_result::Variant::from(variant)),
            },
        }
    }
}


#[pyclass(name = "CompleteWorkflowExecution")]
#[derive(Clone)]
pub struct WrappedCompleteWorkflowExecution {
    pub result: Option<WrappedPayload>,
}

#[pymethods]
impl WrappedCompleteWorkflowExecution {
    #[new]
    fn new(result: Option<WrappedPayload>) -> Self {
        WrappedCompleteWorkflowExecution {
            result,
        }
    }
}

impl From<CompleteWorkflowExecution> for WrappedCompleteWorkflowExecution {
    fn from(i: CompleteWorkflowExecution) -> Self {
        WrappedCompleteWorkflowExecution {
            result: match i.result {
                None => None,
                Some(result) => Some(WrappedPayload::from(result)),
            },
        }
    }
}

impl From<WrappedCompleteWorkflowExecution> for CompleteWorkflowExecution {
    fn from(i: WrappedCompleteWorkflowExecution) -> Self {
        CompleteWorkflowExecution {
            result: match i.result {
                None => None,
                Some(result) => Some(Payload::from(result)),
            },
        }
    }
}


#[pyclass(name = "FailWorkflowExecution")]
#[derive(Clone)]
pub struct WrappedFailWorkflowExecution {
    pub failure: Option<WrappedUserCodeFailure>,
}

#[pymethods]
impl WrappedFailWorkflowExecution {
    #[new]
    fn new(failure: Option<WrappedUserCodeFailure>) -> Self {
        WrappedFailWorkflowExecution {
            failure,
        }
    }
}

impl From<FailWorkflowExecution> for WrappedFailWorkflowExecution {
    fn from(i: FailWorkflowExecution) -> Self {
        WrappedFailWorkflowExecution {
            failure: match i.failure {
                None => None,
                Some(failure) => Some(WrappedUserCodeFailure::from(failure)),
            },
        }
    }
}

impl From<WrappedFailWorkflowExecution> for FailWorkflowExecution {
    fn from(i: WrappedFailWorkflowExecution) -> Self {
        FailWorkflowExecution {
            failure: match i.failure {
                None => None,
                Some(failure) => Some(UserCodeFailure::from(failure)),
            },
        }
    }
}


#[pyclass(name = "ContinueAsNewWorkflowExecution")]
#[derive(Clone)]
pub struct WrappedContinueAsNewWorkflowExecution {
    pub workflow_type: String,
    pub task_queue: String,
    pub arguments: Vec<WrappedPayload>,
    pub workflow_run_timeout: Option<pyo3_chrono::Duration>,
    pub workflow_task_timeout: Option<pyo3_chrono::Duration>,
    pub memo: HashMap<String, WrappedPayload>,
    pub header: HashMap<String, WrappedPayload>,
    pub search_attributes: HashMap<String, WrappedPayload>,
}

#[pymethods]
impl WrappedContinueAsNewWorkflowExecution {
    #[new]
    fn new(workflow_type: String,
           task_queue: String,
           arguments: Vec<WrappedPayload>,
           workflow_run_timeout: Option<pyo3_chrono::Duration>,
           workflow_task_timeout: Option<pyo3_chrono::Duration>,
           memo: HashMap<String, WrappedPayload>,
           header: HashMap<String, WrappedPayload>,
           search_attributes: HashMap<String, WrappedPayload>) -> Self {
        WrappedContinueAsNewWorkflowExecution {
            workflow_type,
            task_queue,
            arguments,
            workflow_run_timeout,
            workflow_task_timeout,
            memo,
            header,
            search_attributes,
        }
    }
}

impl TryFrom<ContinueAsNewWorkflowExecution> for WrappedContinueAsNewWorkflowExecution {
    type Error = PyErr;

    fn try_from(i: ContinueAsNewWorkflowExecution) -> Result<Self, Self::Error> {
        Ok(WrappedContinueAsNewWorkflowExecution {
            workflow_type: i.workflow_type,
            task_queue: i.task_queue,
            arguments: vec_of_payloads_to_vec_of_wrapped_payloads(i.arguments),
            workflow_run_timeout: prost_duration_to_pyo3_chrono_duration(i.workflow_run_timeout)?,
            workflow_task_timeout: prost_duration_to_pyo3_chrono_duration(i.workflow_task_timeout)?,
            memo: hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(i.memo),
            header: hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(i.header),
            search_attributes: hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(i.search_attributes),
        })
    }
}


impl TryFrom<WrappedContinueAsNewWorkflowExecution> for ContinueAsNewWorkflowExecution {
    type Error = PyErr;

    fn try_from(i: WrappedContinueAsNewWorkflowExecution) -> Result<Self, Self::Error> {
        Ok(ContinueAsNewWorkflowExecution {
            workflow_type: i.workflow_type,
            task_queue: i.task_queue,
            arguments: vec_of_wrapped_payloads_to_vec_of_payloads(i.arguments),
            workflow_run_timeout: pyo3_chrono_duration_to_prost_duration(i.workflow_run_timeout)?,
            workflow_task_timeout: pyo3_chrono_duration_to_prost_duration(i.workflow_task_timeout)?,
            memo: hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(i.memo),
            header: hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(i.header),
            search_attributes: hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(i.search_attributes),
        })
    }
}


#[pyclass(name = "CancelWorkflowExecution")]
#[derive(Clone)]
pub struct WrappedCancelWorkflowExecution {}

#[pymethods]
impl WrappedCancelWorkflowExecution {
    #[new]
    fn new() -> Self {
        WrappedCancelWorkflowExecution {}
    }
}

impl From<CancelWorkflowExecution> for WrappedCancelWorkflowExecution {
    fn from(_i: CancelWorkflowExecution) -> Self {
        WrappedCancelWorkflowExecution {}
    }
}

impl From<WrappedCancelWorkflowExecution> for CancelWorkflowExecution {
    fn from(_i: WrappedCancelWorkflowExecution) -> Self {
        CancelWorkflowExecution {}
    }
}


#[pyclass(name = "Variant")]
#[derive(Clone)]
pub struct WrappedVariant {
    pub start_timer: Option<WrappedStartTimer>,
    pub schedule_activity: Option<WrappedScheduleActivity>,
    pub respond_to_query: Option<WrappedQueryResult>,
    pub request_cancel_activity: Option<WrappedRequestCancelActivity>,
    pub cancel_timer: Option<WrappedCancelTimer>,
    pub complete_workflow_execution: Option<WrappedCompleteWorkflowExecution>,
    pub fail_workflow_execution: Option<WrappedFailWorkflowExecution>,
    pub continue_as_new_workflow_execution: Option<WrappedContinueAsNewWorkflowExecution>,
    pub cancel_workflow_execution: Option<WrappedCancelWorkflowExecution>,
}

#[pymethods]
impl WrappedVariant {
    #[new]
    fn new(start_timer: Option<WrappedStartTimer>,
           schedule_activity: Option<WrappedScheduleActivity>,
           respond_to_query: Option<WrappedQueryResult>,
           request_cancel_activity: Option<WrappedRequestCancelActivity>,
           cancel_timer: Option<WrappedCancelTimer>,
           complete_workflow_execution: Option<WrappedCompleteWorkflowExecution>,
           fail_workflow_execution: Option<WrappedFailWorkflowExecution>,
           continue_as_new_workflow_execution: Option<WrappedContinueAsNewWorkflowExecution>,
           cancel_workflow_execution: Option<WrappedCancelWorkflowExecution>) -> Self {
        WrappedVariant {
            start_timer,
            schedule_activity,
            respond_to_query,
            request_cancel_activity,
            cancel_timer,
            complete_workflow_execution,
            fail_workflow_execution,
            continue_as_new_workflow_execution,
            cancel_workflow_execution,
        }
    }
}

impl TryFrom<workflow_command::Variant> for WrappedVariant {
    type Error = PyErr;

    fn try_from(i: workflow_command::Variant) -> Result<Self, Self::Error> {
        Ok(match i {
            workflow_command::Variant::StartTimer(start_timer) => WrappedVariant {
                start_timer: Some(WrappedStartTimer::try_from(start_timer)?),
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::ScheduleActivity(schedule_activity) => WrappedVariant {
                start_timer: None,
                schedule_activity: Some(WrappedScheduleActivity::try_from(schedule_activity)?),
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::RespondToQuery(respond_to_query) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: Some(WrappedQueryResult::try_from(respond_to_query)?),
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::RequestCancelActivity(request_cancel_activity) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: Some(WrappedRequestCancelActivity::try_from(request_cancel_activity)?),
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::CancelTimer(cancel_timer) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: Some(WrappedCancelTimer::try_from(cancel_timer)?),
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::CompleteWorkflowExecution(complete_workflow_execution) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: Some(WrappedCompleteWorkflowExecution::try_from(complete_workflow_execution)?),
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::FailWorkflowExecution(fail_workflow_execution) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: Some(WrappedFailWorkflowExecution::try_from(fail_workflow_execution)?),
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::ContinueAsNewWorkflowExecution(continue_as_new_workflow_execution) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: Some(WrappedContinueAsNewWorkflowExecution::try_from(continue_as_new_workflow_execution)?),
                cancel_workflow_execution: None,
            },
            workflow_command::Variant::CancelWorkflowExecution(cancel_workflow_execution) => WrappedVariant {
                start_timer: None,
                schedule_activity: None,
                respond_to_query: None,
                request_cancel_activity: None,
                cancel_timer: None,
                complete_workflow_execution: None,
                fail_workflow_execution: None,
                continue_as_new_workflow_execution: None,
                cancel_workflow_execution: Some(WrappedCancelWorkflowExecution::try_from(cancel_workflow_execution)?),
            },
        })
    }
}


impl TryFrom<WrappedVariant> for workflow_command::Variant {
    type Error = PyErr;

    fn try_from(i: WrappedVariant) -> Result<Self, Self::Error> {
        Ok(
            if let Some(start_timer) = i.start_timer {
                workflow_command::Variant::StartTimer(StartTimer::try_from(start_timer)?)
            } else if let Some(schedule_activity) = i.schedule_activity {
                workflow_command::Variant::ScheduleActivity(ScheduleActivity::try_from(schedule_activity)?)
            } else if let Some(respond_to_query) = i.respond_to_query {
                workflow_command::Variant::RespondToQuery(QueryResult::from(respond_to_query))
            } else if let Some(request_cancel_activity) = i.request_cancel_activity {
                workflow_command::Variant::RequestCancelActivity(RequestCancelActivity::from(request_cancel_activity))
            } else if let Some(cancel_timer) = i.cancel_timer {
                workflow_command::Variant::CancelTimer(CancelTimer::from(cancel_timer))
            } else if let Some(complete_workflow_execution) = i.complete_workflow_execution {
                workflow_command::Variant::CompleteWorkflowExecution(CompleteWorkflowExecution::from(complete_workflow_execution))
            } else if let Some(fail_workflow_execution) = i.fail_workflow_execution {
                workflow_command::Variant::FailWorkflowExecution(FailWorkflowExecution::from(fail_workflow_execution))
            } else if let Some(continue_as_new_workflow_execution) = i.continue_as_new_workflow_execution {
                workflow_command::Variant::ContinueAsNewWorkflowExecution(ContinueAsNewWorkflowExecution::try_from(continue_as_new_workflow_execution)?)
            } else if let Some(cancel_workflow_execution) = i.cancel_workflow_execution {
                workflow_command::Variant::CancelWorkflowExecution(CancelWorkflowExecution::from(cancel_workflow_execution))
            } else {
                panic!("Only one of workflow command variants must be set");
            }
        )
    }
}


#[pyclass(name = "WorkflowCommand")]
#[derive(Clone)]
pub struct WrappedWorkflowCommand {
    pub variant: Option<WrappedVariant>,
}

#[pymethods]
impl WrappedWorkflowCommand {
    #[new]
    fn new(variant: Option<WrappedVariant>) -> Self {
        WrappedWorkflowCommand {
            variant,
        }
    }
}


impl TryFrom<WorkflowCommand> for WrappedWorkflowCommand {
    type Error = PyErr;

    fn try_from(i: WorkflowCommand) -> Result<Self, Self::Error> {
        Ok(WrappedWorkflowCommand {
            variant: match i.variant {
                None => None,
                Some(variant) => Some(WrappedVariant::try_from(variant)?),
            }
        })
    }
}

impl TryFrom<WrappedWorkflowCommand> for WorkflowCommand {
    type Error = PyErr;

    fn try_from(i: WrappedWorkflowCommand) -> Result<Self, Self::Error> {
        Ok(WorkflowCommand {
            variant: match i.variant {
                None => None,
                Some(variant) => Some(workflow_command::Variant::try_from(variant)?),
            }
        })
    }
}


impl TryFrom<&WorkflowCommand> for WrappedWorkflowCommand {
    type Error = PyErr;

    fn try_from(i: &WorkflowCommand) -> Result<Self, Self::Error> {
        Ok(WrappedWorkflowCommand {
            variant: match i.variant.clone() {
                None => None,
                Some(variant) => Some(WrappedVariant::try_from(variant)?),
            }
        })
    }
}

impl TryFrom<&WrappedWorkflowCommand> for WorkflowCommand {
    type Error = PyErr;

    fn try_from(i: &WrappedWorkflowCommand) -> Result<Self, Self::Error> {
        Ok(WorkflowCommand {
            variant: match i.variant.clone() {
                None => None,
                Some(variant) => Some(workflow_command::Variant::try_from(variant)?),
            }
        })
    }
}
