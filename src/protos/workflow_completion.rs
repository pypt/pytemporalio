use std::convert::TryFrom;

use pyo3::prelude::*;

use temporal_sdk_core::protos::coresdk::{
    common::UserCodeFailure,
    workflow_commands::WorkflowCommand,
    workflow_completion::{
        WfActivationCompletion,
        wf_activation_completion,
        Success,
        Failure,
    },
};

use crate::protos::{
    common::WrappedUserCodeFailure,
    workflow_commands::{
        WrappedWorkflowCommand,
    },
};


#[pyclass(name = "Success")]
#[derive(Clone)]
pub struct WrappedSuccess {
    pub commands: Vec<WrappedWorkflowCommand>,
}

#[pymethods]
impl WrappedSuccess {
    #[new]
    fn new(commands: Vec<WrappedWorkflowCommand>) -> Self {
        WrappedSuccess {
            commands,
        }
    }
}

impl TryFrom<Success> for WrappedSuccess {
    type Error = PyErr;

    fn try_from(i: Success) -> Result<Self, Self::Error> {
        let wrapped_command_results = i.commands.iter().map(|x| WrappedWorkflowCommand::try_from(x));
        let mut wrapped_commands: Vec<WrappedWorkflowCommand> = Vec::new();
        for result in wrapped_command_results {
            wrapped_commands.push(result?);
        }
        Ok(WrappedSuccess {
            commands: wrapped_commands,
        })
    }
}


impl TryFrom<WrappedSuccess> for Success {
    type Error = PyErr;

    fn try_from(i: WrappedSuccess) -> Result<Self, Self::Error> {
        let command_results = i.commands.iter().map(|x| WorkflowCommand::try_from(x));
        let mut commands: Vec<WorkflowCommand> = Vec::new();
        for result in command_results {
            commands.push(result?);
        }
        Ok(Success {
            commands: commands,
        })
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
                Some(failure) => Some(WrappedUserCodeFailure::from(failure)),
            }
        }
    }
}


impl From<WrappedFailure> for Failure {
    fn from(i: WrappedFailure) -> Self {
        Failure {
            failure: match i.failure {
                None => None,
                Some(failure) => Some(UserCodeFailure::from(failure)),
            }
        }
    }
}


#[pyclass(name = "Status")]
#[derive(Clone)]
pub struct WrappedStatus {
    pub successful: Option<WrappedSuccess>,
    pub failed: Option<WrappedFailure>,
}


#[pymethods]
impl WrappedStatus {
    #[new]
    fn new(successful: Option<WrappedSuccess>, failed: Option<WrappedFailure>) -> Self {
        WrappedStatus {
            successful,
            failed,
        }
    }
}

impl TryFrom<wf_activation_completion::Status> for WrappedStatus {
    type Error = PyErr;

    fn try_from(i: wf_activation_completion::Status) -> Result<Self, Self::Error> {
        Ok(match i {
            wf_activation_completion::Status::Successful(success) => WrappedStatus {
                successful: Some(WrappedSuccess::try_from(success)?),
                failed: None,
            },
            wf_activation_completion::Status::Failed(failure) => WrappedStatus {
                successful: None,
                failed: Some(WrappedFailure::from(failure)),
            },
        })
    }
}


impl TryFrom<WrappedStatus> for wf_activation_completion::Status {
    type Error = PyErr;

    fn try_from(i: WrappedStatus) -> Result<Self, Self::Error> {
        Ok(
            if let Some(success) = i.successful {
                wf_activation_completion::Status::Successful(Success::try_from(success)?)
            } else if let Some(failure) = i.failed {
                wf_activation_completion::Status::Failed(Failure::try_from(failure)?)
            } else {
                panic!("Only one of workflow completion statuses must be set");
            }
        )
    }
}


#[pyclass(name = "WfActivationCompletion")]
#[derive(Clone)]
pub struct WrappedWfActivationCompletion {
    pub run_id: String,
    pub status: Option<WrappedStatus>,
}

#[pymethods]
impl WrappedWfActivationCompletion {
    #[new]
    fn new(run_id: String, status: Option<WrappedStatus>) -> Self {
        WrappedWfActivationCompletion {
            run_id,
            status,
        }
    }
}


impl TryFrom<WfActivationCompletion> for WrappedWfActivationCompletion {
    type Error = PyErr;

    fn try_from(i: WfActivationCompletion) -> Result<Self, Self::Error> {
        Ok(WrappedWfActivationCompletion {
            run_id: i.run_id,
            status: match i.status {
                None => None,
                Some(status) => Some(WrappedStatus::try_from(status)?),
            },
        })
    }
}

impl TryFrom<WrappedWfActivationCompletion> for WfActivationCompletion {
    type Error = PyErr;

    fn try_from(i: WrappedWfActivationCompletion) -> Result<Self, Self::Error> {
        Ok(WfActivationCompletion {
            run_id: i.run_id,
            status: match i.status {
                None => None,
                Some(status) => Some(wf_activation_completion::Status::try_from(status)?),
            },
        })
    }
}
