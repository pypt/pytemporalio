use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyOSError;
use pyo3_asyncio;

mod errors;
mod pollers;
mod protos;
mod utils;
mod worker;

use errors::{
    WorkerRegistrationError,
    PollWfError,
    PollActivityError,
};
use pollers::WrappedServerGatewayOptions;

// FIXME make them more hierarchical or something
use protos::{
    WrappedPayload,
    WrappedWfActivation,
    WrappedWfActivationJob,
    WrappedStartWorkflow,
    WrappedFireTimer,
    WrappedUpdateRandomSeed,
    WrappedQueryWorkflow,
    WrappedCancelWorkflow,
    WrappedSignalWorkflow,
    WrappedResolveActivity,
    WrappedActivityResult,
    WrappedSuccess,
    WrappedCancelation,
    WrappedFailure,
    WrappedUserCodeFailure,
    WrappedActivityTask,
    WrappedStart,
    WrappedCancel,
    WrappedWorkflowExecution,
};
use worker::WrappedWorkerConfig;


use std::sync::Arc;

use temporal_sdk_core::{
    init,
    Core,
    CoreInitOptions,
    protos::coresdk::workflow_activation::wf_activation_job,
    protos::coresdk::activity_result::activity_result,
    protos::coresdk::activity_task::activity_task,
};

use utils::{
    prost_types_timestamp_to_u128,
    prost_duration_to_pyo3_chrono_duration,
};
use crate::protos::{WrappedRetryPolicy, WrappedVariant};


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

                        let wrapped_jobs: Vec<Option<WrappedWfActivationJob>> = wf_activation.jobs.iter().map(|x| match x.variant.clone() {
                            None => None,

                            // FIXME wrap around 120 characters
                            // FIXME use a builder pattern or something like that to avoid the Nones
                            Some(job) => Some(match job {
                                wf_activation_job::Variant::StartWorkflow(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: Some(WrappedStartWorkflow {
                                            workflow_type: matched_job.workflow_type,
                                            workflow_id: matched_job.workflow_id,
                                            arguments: matched_job.arguments.iter().map(|x| WrappedPayload {
                                                metadata: x.metadata.clone(),
                                                data: x.data.clone(),
                                            }).collect::<Vec<_>>(),
                                            randomness_seed: matched_job.randomness_seed,

                                            // FIXME we could probably do less copying here
                                            headers: matched_job.headers.iter().map(|(k, v)| (
                                                String::from(k),
                                                WrappedPayload { metadata: v.metadata.clone(), data: v.data.clone() }
                                            )).collect(),
                                        }),
                                        fire_timer: None,
                                        update_random_seed: None,
                                        query_workflow: None,
                                        cancel_workflow: None,
                                        signal_workflow: None,
                                        resolve_activity: None,
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::FireTimer(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: Some(
                                            WrappedFireTimer {
                                                timer_id: matched_job.timer_id,
                                            }
                                        ),
                                        update_random_seed: None,
                                        query_workflow: None,
                                        cancel_workflow: None,
                                        signal_workflow: None,
                                        resolve_activity: None,
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::UpdateRandomSeed(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: None,
                                        update_random_seed: Some(
                                            WrappedUpdateRandomSeed {
                                                randomness_seed: matched_job.randomness_seed,
                                            }
                                        ),
                                        query_workflow: None,
                                        cancel_workflow: None,
                                        signal_workflow: None,
                                        resolve_activity: None,
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::QueryWorkflow(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: None,
                                        update_random_seed: None,
                                        query_workflow: Some(
                                            WrappedQueryWorkflow {
                                                query_id: matched_job.query_id,
                                                query_type: matched_job.query_type,
                                                arguments: matched_job.arguments.iter().map(|x| WrappedPayload {
                                                    metadata: x.metadata.clone(),
                                                    data: x.data.clone(),
                                                }).collect::<Vec<_>>(),
                                            }
                                        ),
                                        cancel_workflow: None,
                                        signal_workflow: None,
                                        resolve_activity: None,
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::CancelWorkflow(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: None,
                                        update_random_seed: None,
                                        query_workflow: None,
                                        cancel_workflow: Some(
                                            WrappedCancelWorkflow {
                                                details: matched_job.details.iter().map(|x| WrappedPayload {
                                                    metadata: x.metadata.clone(),
                                                    data: x.data.clone(),
                                                }).collect::<Vec<_>>(),
                                            }
                                        ),
                                        signal_workflow: None,
                                        resolve_activity: None,
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::SignalWorkflow(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: None,
                                        update_random_seed: None,
                                        query_workflow: None,
                                        cancel_workflow: None,
                                        signal_workflow: Some(
                                            WrappedSignalWorkflow {
                                                signal_name: matched_job.signal_name,
                                                input: matched_job.input.iter().map(|x| WrappedPayload {
                                                    metadata: x.metadata.clone(),
                                                    data: x.data.clone(),
                                                }).collect::<Vec<_>>(),
                                                identity: matched_job.identity,
                                            }
                                        ),
                                        resolve_activity: None,
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::ResolveActivity(matched_job) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: None,
                                        update_random_seed: None,
                                        query_workflow: None,
                                        cancel_workflow: None,
                                        signal_workflow: None,
                                        resolve_activity: Some(
                                            WrappedResolveActivity {
                                                activity_id: matched_job.activity_id,
                                                result: match matched_job.result {
                                                    None => None,
                                                    Some(result) => match result.status {
                                                        // FIXME it should then be a ResolveActivity with empty status
                                                        None => None,
                                                        Some(status) => Some(match status {
                                                            activity_result::Status::Completed(matched_status) => {
                                                                WrappedActivityResult {
                                                                    completed: Some(WrappedSuccess {
                                                                        result: match matched_status.result {
                                                                            None => None,
                                                                            Some(result) => Some(WrappedPayload {
                                                                                metadata: result.metadata,
                                                                                data: result.data,
                                                                            })
                                                                        },
                                                                    }),
                                                                    failed: None,
                                                                    canceled: None,
                                                                }
                                                            }
                                                            activity_result::Status::Failed(matched_status) => {
                                                                WrappedActivityResult {
                                                                    completed: None,
                                                                    failed: Some(WrappedFailure {
                                                                        failure: match matched_status.failure {
                                                                            None => None,
                                                                            Some(failure) => Some(WrappedUserCodeFailure {
                                                                                message: failure.message,
                                                                                r#type: failure.r#type,
                                                                                source: failure.source,
                                                                                stack_trace: failure.stack_trace,
                                                                                non_retryable: failure.non_retryable,

                                                                                // FIXME recursively convert UserCodeFailure to WrappedUserCodeFailure
                                                                                cause: None,
                                                                            })
                                                                        },
                                                                    }),
                                                                    canceled: None,
                                                                }
                                                            }
                                                            activity_result::Status::Canceled(matched_status) => {
                                                                WrappedActivityResult {
                                                                    completed: None,
                                                                    failed: None,
                                                                    canceled: Some(WrappedCancelation {
                                                                        details: match matched_status.details {
                                                                            None => None,
                                                                            Some(details) => Some(WrappedPayload {
                                                                                metadata: details.metadata,
                                                                                data: details.data,
                                                                            })
                                                                        },
                                                                    }),
                                                                }
                                                            }
                                                        }),
                                                    }
                                                },
                                            }
                                        ),
                                        remove_from_cache: None,
                                    }
                                }
                                wf_activation_job::Variant::RemoveFromCache(remove_from_cache) => {
                                    WrappedWfActivationJob {
                                        start_workflow: None,
                                        fire_timer: None,
                                        update_random_seed: None,
                                        query_workflow: None,
                                        cancel_workflow: None,
                                        signal_workflow: None,
                                        resolve_activity: None,
                                        remove_from_cache: Some(remove_from_cache),
                                    }
                                }
                            }),
                        }).collect::<Vec<_>>();

                        let wrapped_wf_activation = WrappedWfActivation {
                            run_id: wf_activation.run_id,
                            // FIXME is it optional by any chance?
                            timestamp: prost_types_timestamp_to_u128(wf_activation.timestamp),
                            is_replaying: wf_activation.is_replaying,
                            jobs: wrapped_jobs,
                        };

                        Ok(wrapped_wf_activation.into_py(py))
                    })
                }
            }
        })
    }

    fn poll_activity_task<'p>(&self, py: Python<'p>, task_queue: String) -> PyResult<&'p PyAny> {
        let internal = self.internal.clone();
        pyo3_asyncio::tokio::local_future_into_py(py, async move {
            match internal.poll_activity_task(task_queue.as_str()).await {

                // FIXME PollActivityError
                Err(err) => Err(PollActivityError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(activity_task) => {
                    // FIXME WrappedActivityTask
                    Python::with_gil(|py| {
                        let wrapped_activity_task = WrappedActivityTask {
                            task_token: activity_task.task_token,
                            activity_id: activity_task.activity_id,
                            variant: match activity_task.variant {
                                None => None,
                                Some(variant) => Some(
                                    match variant {
                                        activity_task::Variant::Start(task) => {
                                            WrappedVariant {
                                                start: Some(WrappedStart {
                                                    workflow_namespace: task.workflow_namespace,
                                                    workflow_type: task.workflow_type,
                                                    workflow_execution: match task.workflow_execution {
                                                        None => None,
                                                        Some(workflow_execution) => Some(WrappedWorkflowExecution {
                                                            workflow_id: workflow_execution.workflow_id,
                                                            run_id: workflow_execution.run_id,
                                                        }),
                                                    },
                                                    activity_type: task.activity_type,
                                                    header_fields: task.header_fields.iter().map(|(k, v)| (
                                                        String::from(k),
                                                        WrappedPayload { metadata: v.metadata.clone(), data: v.data.clone() }
                                                    )).collect(),
                                                    input: task.input.iter().map(|x| WrappedPayload {
                                                        metadata: x.metadata.clone(),
                                                        data: x.data.clone(),
                                                    }).collect::<Vec<_>>(),
                                                    heartbeat_details: task.heartbeat_details.iter().map(|x| WrappedPayload {
                                                        metadata: x.metadata.clone(),
                                                        data: x.data.clone(),
                                                    }).collect::<Vec<_>>(),
                                                    scheduled_time: prost_types_timestamp_to_u128(task.scheduled_time),
                                                    current_attempt_scheduled_time: prost_types_timestamp_to_u128(task.current_attempt_scheduled_time),
                                                    started_time: prost_types_timestamp_to_u128(task.started_time),
                                                    attempt: task.attempt,
                                                    schedule_to_close_timeout: prost_duration_to_pyo3_chrono_duration(task.schedule_to_close_timeout)?,
                                                    start_to_close_timeout: prost_duration_to_pyo3_chrono_duration(task.start_to_close_timeout)?,
                                                    heartbeat_timeout: prost_duration_to_pyo3_chrono_duration(task.heartbeat_timeout)?,
                                                    retry_policy: match task.retry_policy {
                                                        None => None,
                                                        Some(retry_policy) => Some( WrappedRetryPolicy{
                                                            initial_interval: prost_duration_to_pyo3_chrono_duration(retry_policy.initial_interval)?,
                                                            backoff_coefficient: retry_policy.backoff_coefficient,
                                                            maximum_interval: prost_duration_to_pyo3_chrono_duration(retry_policy.maximum_interval)?,
                                                            maximum_attempts: retry_policy.maximum_attempts,
                                                            non_retryable_error_types: retry_policy.non_retryable_error_types,
                                                        }),
                                                    },
                                                }),
                                                cancel: None,
                                            }
                                        },
                                        activity_task::Variant::Cancel(task) => {
                                            WrappedVariant {
                                                start: None,
                                                cancel: Some(WrappedCancel {
                                                    reason: task.reason,
                                                })
                                            }
                                        },
                                    }
                                )
                            },
                        };

                        Ok(wrapped_activity_task.into_py(py))
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


#[pymodule]
pub fn pytemporalio(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wrapped_init, m)?)?;

    m.add_class::<WrappedActivityResult>()?;
    m.add_class::<WrappedCancelation>()?;
    m.add_class::<WrappedCancelWorkflow>()?;
    m.add_class::<WrappedCore>()?;
    m.add_class::<WrappedCoreInitOptions>()?;
    m.add_class::<WrappedFailure>()?;
    m.add_class::<WrappedFireTimer>()?;
    m.add_class::<WrappedPayload>()?;
    m.add_class::<WrappedQueryWorkflow>()?;
    m.add_class::<WrappedResolveActivity>()?;
    m.add_class::<WrappedServerGatewayOptions>()?;
    m.add_class::<WrappedSignalWorkflow>()?;
    m.add_class::<WrappedStartWorkflow>()?;
    m.add_class::<WrappedSuccess>()?;
    m.add_class::<WrappedUpdateRandomSeed>()?;
    m.add_class::<WrappedUserCodeFailure>()?;
    m.add_class::<WrappedWfActivation>()?;
    m.add_class::<WrappedWfActivationJob>()?;
    m.add_class::<WrappedWorkerConfig>()?;


    m.add("WorkerRegistrationError", py.get_type::<WorkerRegistrationError>())?;
    m.add("PollWfError", py.get_type::<PollWfError>())?;

    Ok(())
}
