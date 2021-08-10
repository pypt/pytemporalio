use pyo3::prelude::*;

use crate::protos::{
    WrappedPayload,
    WrappedUserCodeFailure,
};

#[pyclass(name = "Cancelation")]
#[derive(Clone)]
pub struct WrappedCancelation {
    pub details: Option<WrappedPayload>,
}

#[pymethods]
impl WrappedCancelation {}


#[pyclass(name = "Success")]
#[derive(Clone)]
pub struct WrappedSuccess {
    pub result: Option<WrappedPayload>,
}

#[pymethods]
impl WrappedSuccess {}


#[pyclass(name = "Failure")]
#[derive(Clone)]
pub struct WrappedFailure {
    pub failure: Option<WrappedUserCodeFailure>,
}

#[pymethods]
impl WrappedFailure {}


// FIXME ensure that only one gets set somehow
#[pyclass(name = "ActivityResult")]
#[derive(Clone)]
pub struct WrappedActivityResult {
    pub completed: Option<WrappedSuccess>,
    pub failed: Option<WrappedFailure>,
    pub canceled: Option<WrappedCancelation>,
}

#[pymethods]
impl WrappedActivityResult {}
