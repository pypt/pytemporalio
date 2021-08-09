use pyo3::prelude::*;


use std::collections::HashMap;


#[pyclass(name = "Payload")]
#[derive(Clone)]
pub struct WrappedPayload {
    pub metadata: HashMap<String, Vec<u8>>,
    pub data: Vec<u8>,
}

#[pymethods]
impl WrappedPayload {}


#[pyclass(name = "WfActivationJob", subclass)]
#[derive(Clone)]
pub struct WrappedWfActivationJob {}

#[pymethods]
impl WrappedWfActivationJob {}


#[pyclass(name = "StartWorkflow", extends = WrappedWfActivationJob)]
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


#[pyclass(name = "FireTimer", extends = WrappedWfActivationJob)]
#[derive(Clone)]
pub struct WrappedFireTimer {
    pub timer_id: String,
}

#[pymethods]
impl WrappedFireTimer {}


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
