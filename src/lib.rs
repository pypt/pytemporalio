use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use pyo3::types::PyDict;
use pyo3::exceptions::PyOSError;
use pyo3_asyncio;

mod errors;
mod pollers;
mod protos;
mod worker;

use errors::{WorkerRegistrationError, PollWfError};
use pollers::WrappedServerGatewayOptions;
use protos::{
    WrappedPayload,
    WrappedWfActivation,
    WrappedWfActivationJob,
    WrappedStartWorkflow,
    WrappedFireTimer,
};
use worker::WrappedWorkerConfig;


use std::sync::Arc;

use temporal_sdk_core::{
    init,
    Core,
    CoreInitOptions,
    protos::coresdk::workflow_activation::wf_activation_job,
};


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
struct WrappedCore {
    pub(crate) internal: Arc<Box<dyn Core>>,
}

#[pymethods]
impl WrappedCore {
    fn register_worker<'p>(&self, py: Python<'p>, config: WrappedWorkerConfig) -> PyResult<&'p PyAny> {
        let internal = self.internal.clone();
        pyo3_asyncio::tokio::local_future_into_py(py, async move {
            match internal.register_worker(config.internal).await {
                Err(err) => Err(WorkerRegistrationError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(()) => {
                    Python::with_gil(|py| Ok(py.None()))
                }
            }
        })
    }

    fn poll_workflow_task<'p>(&self, py: Python<'p>, task_queue: String) -> PyResult<&'p PyAny> {
        let internal = self.internal.clone();
        pyo3_asyncio::tokio::local_future_into_py(py, async move {
            match internal.poll_workflow_task(task_queue.as_str()).await {
                Err(err) => Err(PollWfError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(wf_activation) => {
                    // FIXME return the activation
                    Python::with_gil(|py| {

                        // FIXME make sure duration since epoch works fine
                        let nanos_since_epoch = match wf_activation.timestamp {
                            Some(ts) => Some(ts.seconds as u128 * 1000 + ts.nanos as u128),
                            None => None,
                        };

                        let wrapped_jobs: Vec<Option<WrappedWfActivationJob>> = wf_activation.jobs.iter().map(|&x| match x.variant {
                            None => None,

                            // FIXME wrap around 120 characters
                            Some(job) => Some(match job {
                                wf_activation_job::Variant::StartWorkflow(matched_job) => {
                                    WrappedStartWorkflow {
                                        workflow_type: matched_job.workflow_type,
                                        workflow_id: matched_job.workflow_id,
                                        arguments: matched_job.arguments.iter().map(|&x| WrappedPayload {
                                            metadata: x.metadata,
                                            data: x.data,
                                        }).collect::<Vec<_>>(),
                                        randomness_seed: matched_job.randomness_seed,

                                        // FIXME we could probably do less copying here
                                        headers: matched_job.headers.iter().map(|(k, v)| (
                                            String::from(k),
                                            WrappedPayload { metadata: v.metadata, data: v.data }
                                        )).collect(),
                                    }
                                }
                                wf_activation_job::Variant::FireTimer(matched_job) => {
                                    WrappedFireTimer {
                                        timer_id: matched_job.timer_id,
                                    }
                                }
                            }),
                        }).collect::<Vec<_>>();

                        let wrapped_wf_activation = WrappedWfActivation {
                            run_id: wf_activation.run_id,
                            // FIXME is it optional by any chance?
                            timestamp: nanos_since_epoch,
                            is_replaying: wf_activation.is_replaying,
                            jobs: wrapped_jobs,
                        };

                        Ok(wrapped_wf_activation.into_py(py))
                    })
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
                    let wrapped_core = WrappedCore { internal: Arc::new(Box::new(initialized_core)) };
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
