use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use pyo3::types::PyDict;
use pyo3::exceptions::PyOSError;
use pyo3_asyncio;

mod errors;
mod pollers;
mod worker;

// use errors::*;
use pollers::WrappedServerGatewayOptions;
use worker::WrappedWorkerConfig;


// use prost::Message;

// use std::{fmt::Display, future::Future, sync::Arc};

use temporal_sdk_core::{
    init,
    // protos::coresdk::workflow_completion::WfActivationCompletion,
    // protos::coresdk::ActivityHeartbeat,
    // protos::coresdk::ActivityTaskCompletion,
    // tracing_init,
    // ClientTlsConfig,
    // CompleteActivityError,
    // CompleteWfError,
    Core,
    // CoreInitError,
    CoreInitOptions,
    // PollActivityError,
    // PollWfError,
    // ServerGatewayOptions,
    // TlsConfig,
    // Url,
    // WorkerConfig,
};
// use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
// use std::collections::HashMap;


#[pyclass(name = "CoreInitOptions")]
#[derive(Clone)]
struct WrappedCoreInitOptions {
    pub(crate) internal: CoreInitOptions,
}

#[pymethods]
impl WrappedCoreInitOptions {
    // FIXME set default value of max_cached_workflows here
    #[new]
    fn new(gateway_opts: WrappedServerGatewayOptions, max_cached_workflows: usize) -> Self {
        WrappedCoreInitOptions {
            internal: CoreInitOptions {
                gateway_opts: gateway_opts.internal,
                max_cached_workflows,
            }
        }
    }
}


#[pyclass(name = "Core")]
#[derive(Clone)]
struct WrappedCore {
    pub(crate) internal: Box<dyn Core>,
}

#[pymethods]
impl WrappedCore {
    fn register_worker<'p>(&mut self, py: Python<'p>, config: WrappedWorkerConfig) -> PyResult<&'p PyAny> {
        let mut this = self.clone();

        pyo3_asyncio::tokio::local_future_into_py(py, async move {
            match this.internal.register_worker(config.internal).await {

                // FIXME custom error
                Err(err) => Err(PyOSError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(()) => {
                    Python::with_gil(|py| Ok(py.None()))
                }
            }
        })
    }
}

#[pyfunction(name = "init")]
fn wrapped_init(py: Python, opts: WrappedCoreInitOptions) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::local_future_into_py(py, async move {
        match init(opts.internal).await {
            Err(err) => return Err(PyOSError::new_err(format!(
                "{}",
                err.to_string()
            ))),
            Ok(initialized_core) => {
                Python::with_gil(|py| {
                    let wrapped_core = WrappedCore { internal: Box::new(initialized_core) };
                    Ok(wrapped_core.into_py(py))
                })
            }
        }
    })
}


#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
pub fn pytemporalio(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    // m.add_class::<WrappedClientTlsConfig>()?;
    // m.add_class::<WrappedTlsConfig>()?;

    // m.add("WorkflowUpdateError", py.get_type::<WorkflowUpdateError>())?;

    Ok(())
}
