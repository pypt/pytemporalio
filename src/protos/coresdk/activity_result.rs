use pyo3::prelude::*;

use temporal_sdk_core::protos::coresdk::{
    activity_result::{
        ActivityResult,
        activity_result,
        Cancelation,
        Success,
        Failure,
    },
    common::{
        Payload,
        UserCodeFailure,
    },
};

use crate::protos::coresdk::common::{
    WrappedPayload,
    WrappedUserCodeFailure,
};

#[pyclass(name = "Cancelation")]
#[derive(Clone)]
pub struct WrappedCancelation {
    pub details: Option<WrappedPayload>,
}

#[pymethods]
impl WrappedCancelation {
    #[new]
    fn new(details: Option<WrappedPayload>) -> Self {
        WrappedCancelation {
            details,
        }
    }
}


impl From<Cancelation> for WrappedCancelation {
    fn from(i: Cancelation) -> Self {
        WrappedCancelation {
            details: match i.details {
                None => None,
                Some(details) => Some(WrappedPayload::from(details)),
            }
        }
    }
}

impl From<WrappedCancelation> for Cancelation {
    fn from(i: WrappedCancelation) -> Self {
        Cancelation {
            details: match i.details {
                None => None,
                Some(details) => Some(Payload::from(details)),
            }
        }
    }
}


#[pyclass(name = "Success")]
#[derive(Clone)]
pub struct WrappedSuccess {
    pub result: Option<WrappedPayload>,
}

#[pymethods]
impl WrappedSuccess {
    #[new]
    fn new(result: Option<WrappedPayload>) -> Self {
        WrappedSuccess {
            result,
        }
    }
}


impl From<Success> for WrappedSuccess {
    fn from(i: Success) -> Self {
        WrappedSuccess {
            result: match i.result {
                None => None,
                Some(result) => Some(WrappedPayload::from(result))
            },
        }
    }
}


impl From<WrappedSuccess> for Success {
    fn from(i: WrappedSuccess) -> Self {
        Success {
            result: match i.result {
                None => None,
                Some(result) => Some(Payload::from(result))
            },
        }
    }
}


#[pyclass(name = "Failure")]
#[derive(Clone)]
pub struct WrappedFailure {
    pub failure: Option<WrappedUserCodeFailure>,
}

#[pymethods]
impl WrappedFailure {
    #[new]
    fn new(failure: Option<WrappedUserCodeFailure>) -> Self {
        WrappedFailure {
            failure,
        }
    }
}


impl From<Failure> for WrappedFailure {
    fn from(i: Failure) -> Self {
        WrappedFailure {
            failure: match i.failure {
                None => None,
                Some(failure) => Some(WrappedUserCodeFailure::from(failure))
            }
        }
    }
}

impl From<WrappedFailure> for Failure {
    fn from(i: WrappedFailure) -> Self {
        Failure {
            failure: match i.failure {
                None => None,
                Some(failure) => Some(UserCodeFailure::from(failure))
            }
        }
    }
}


#[pyclass(name = "Status")]
#[derive(Clone)]
pub struct WrappedStatus {
    pub completed: Option<WrappedSuccess>,
    pub failed: Option<WrappedFailure>,
    pub canceled: Option<WrappedCancelation>,
}

#[pymethods]
impl WrappedStatus {
    #[new]
    fn new(completed: Option<WrappedSuccess>,
           failed: Option<WrappedFailure>,
           canceled: Option<WrappedCancelation>) -> Self {
        WrappedStatus {
            completed,
            failed,
            canceled,
        }
    }
}


impl From<activity_result::Status> for WrappedStatus {
    fn from(i: activity_result::Status) -> Self {
        match i {
            activity_result::Status::Completed(completed_status) => {
                WrappedStatus {
                    completed: Some(WrappedSuccess::from(completed_status)),
                    failed: None,
                    canceled: None,
                }
            }
            activity_result::Status::Failed(failed_status) => {
                WrappedStatus {
                    completed: None,
                    failed: Some(WrappedFailure::from(failed_status)),
                    canceled: None,
                }
            }
            activity_result::Status::Canceled(canceled_status) => {
                WrappedStatus {
                    completed: None,
                    failed: None,
                    canceled: Some(WrappedCancelation::from(canceled_status)),
                }
            }
        }
    }
}


impl From<WrappedStatus> for activity_result::Status {
    fn from(i: WrappedStatus) -> Self {
        if let Some(completed) = i.completed {
            activity_result::Status::Completed(Success::from(completed))
        } else if let Some(failed) = i.failed {
            activity_result::Status::Failed(Failure::from(failed))
        } else if let Some(canceled) = i.canceled {
            activity_result::Status::Canceled(Cancelation::from(canceled))
        } else {
            panic!("Only one of 'completed', 'failed' and 'canceled' must be set")
        }
    }
}


#[pyclass(name = "ActivityResult")]
#[derive(Clone)]
pub struct WrappedActivityResult {
    pub status: Option<WrappedStatus>,
}


#[pymethods]
impl WrappedActivityResult {
    #[new]
    fn new(status: Option<WrappedStatus>) -> Self {
        WrappedActivityResult {
            status
        }
    }
}


impl From<ActivityResult> for WrappedActivityResult {
    fn from(i: ActivityResult) -> Self {
        WrappedActivityResult {
            status: match i.status {
                None => None,
                Some(status) => Some(WrappedStatus::from(status))
            }
        }
    }
}

impl From<WrappedActivityResult> for ActivityResult {
    fn from(i: WrappedActivityResult) -> Self {
        ActivityResult {
            status: match i.status {
                None => None,
                Some(status) => Some(activity_result::Status::from(status))
            }
        }
    }
}
