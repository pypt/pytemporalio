use pyo3::prelude::*;
use crate::errors::WorkflowUpdateError;

mod worker;
mod errors;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
pub fn pytemporalio(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add("WorkflowUpdateError", py.get_type::<WorkflowUpdateError>())?;

    Ok(())
}
