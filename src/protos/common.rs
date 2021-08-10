use pyo3::prelude::*;

use std::collections::HashMap;

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
