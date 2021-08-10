use pyo3::prelude::*;


use std::collections::HashMap;

use crate::protos::WrappedActivityResult;
use crate::protos::WrappedPayload;

#[pyclass(name = "StartWorkflow")]
#[derive(Clone)]
pub struct WrappedStartWorkflow {
    pub workflow_type: String,
    pub workflow_id: String,
    pub arguments: Vec<WrappedPayload>,
    pub randomness_seed: u64,
    pub headers: HashMap<String, WrappedPayload>,
}

#[pymethods]
impl WrappedStartWorkflow {}


#[pyclass(name = "FireTimer")]
#[derive(Clone)]
pub struct WrappedFireTimer {
    pub timer_id: String,
}

#[pymethods]
impl WrappedFireTimer {}


#[pyclass(name = "UpdateRandomSeed")]
#[derive(Clone)]
pub struct WrappedUpdateRandomSeed {
    pub randomness_seed: u64,
}

#[pymethods]
impl WrappedUpdateRandomSeed {}


#[pyclass(name = "QueryWorkflow")]
#[derive(Clone)]
pub struct WrappedQueryWorkflow {
    pub query_id: String,
    pub query_type: String,
    pub arguments: Vec<WrappedPayload>,
}

#[pymethods]
impl WrappedQueryWorkflow {}


#[pyclass(name = "CancelWorkflow")]
#[derive(Clone)]
pub struct WrappedCancelWorkflow {
    pub details: Vec<WrappedPayload>,
}

#[pymethods]
impl WrappedCancelWorkflow {}


#[pyclass(name = "SignalWorkflow")]
#[derive(Clone)]
pub struct WrappedSignalWorkflow {
    pub signal_name: String,
    pub input: Vec<WrappedPayload>,
    pub identity: String,
}

#[pymethods]
impl WrappedSignalWorkflow {}


#[pyclass(name = "ResolveActivity")]
#[derive(Clone)]
pub struct WrappedResolveActivity {
    pub activity_id: String,
    pub result: Option<WrappedActivityResult>,
}

#[pymethods]
impl WrappedResolveActivity {}


// FIXME validate that only one of them is set somehow (but then all of them could be empty)
#[pyclass(name = "WfActivationJob")]
#[derive(Clone)]
pub struct WrappedWfActivationJob {
    pub start_workflow: Option<WrappedStartWorkflow>,
    pub fire_timer: Option<WrappedFireTimer>,
    pub update_random_seed: Option<WrappedUpdateRandomSeed>,
    pub query_workflow: Option<WrappedQueryWorkflow>,
    pub cancel_workflow: Option<WrappedCancelWorkflow>,
    pub signal_workflow: Option<WrappedSignalWorkflow>,
    pub resolve_activity: Option<WrappedResolveActivity>,
    pub remove_from_cache: Option<bool>,
}

#[pymethods]
impl WrappedWfActivationJob {}


#[pyclass(name = "WfActivation")]
#[derive(Clone)]
pub struct WrappedWfActivation {
    pub run_id: String,
    pub timestamp: Option<u128>,
    pub is_replaying: bool,
    pub jobs: Vec<Option<WrappedWfActivationJob>>,
}

#[pymethods]
impl WrappedWfActivation {}
