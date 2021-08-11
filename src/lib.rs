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
use pollers::{
    WrappedServerGatewayOptions,
    WrappedClientTlsConfig,
    WrappedTlsConfig,
};

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
    WrappedRetryPolicy,
    WrappedVariant,
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
    vec_of_payloads_to_vec_of_wrapped_payloads,
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
    pub(crate) internal: Arc<dyn Core>,
}

#[pymethods]
impl WrappedCore {
    fn register_worker<'p>(&self, py: Python<'p>, config: WrappedWorkerConfig) -> PyResult<&'p PyAny> {
        let internal = self.internal.clone();
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
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
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
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
                                            arguments: vec_of_payloads_to_vec_of_wrapped_payloads(matched_job.arguments),
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
                                                arguments: vec_of_payloads_to_vec_of_wrapped_payloads(matched_job.arguments),
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
                                                details: vec_of_payloads_to_vec_of_wrapped_payloads(matched_job.details),
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
                                                input: vec_of_payloads_to_vec_of_wrapped_payloads(matched_job.input),
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
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
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
                                                    input: vec_of_payloads_to_vec_of_wrapped_payloads(task.input),
                                                    heartbeat_details: vec_of_payloads_to_vec_of_wrapped_payloads(task.heartbeat_details),
                                                    scheduled_time: prost_types_timestamp_to_u128(task.scheduled_time),
                                                    current_attempt_scheduled_time: prost_types_timestamp_to_u128(task.current_attempt_scheduled_time),
                                                    started_time: prost_types_timestamp_to_u128(task.started_time),
                                                    attempt: task.attempt,
                                                    schedule_to_close_timeout: prost_duration_to_pyo3_chrono_duration(task.schedule_to_close_timeout)?,
                                                    start_to_close_timeout: prost_duration_to_pyo3_chrono_duration(task.start_to_close_timeout)?,
                                                    heartbeat_timeout: prost_duration_to_pyo3_chrono_duration(task.heartbeat_timeout)?,
                                                    retry_policy: match task.retry_policy {
                                                        None => None,
                                                        Some(retry_policy) => Some(WrappedRetryPolicy {
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
                                        }
                                        activity_task::Variant::Cancel(task) => {
                                            WrappedVariant {
                                                start: None,
                                                cancel: Some(WrappedCancel {
                                                    reason: task.reason,
                                                }),
                                            }
                                        }
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
    let current_loop = pyo3_asyncio::get_running_loop(py)?;
    pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
        match init(opts.internal).await {
            Err(err) => return Err(PyOSError::new_err(format!(
                "{}",
                err.to_string()
            ))),
            Ok(initialized_core) => {
                Python::with_gil(|py| {
                    let wrapped_core = WrappedCore { internal: Arc::new(initialized_core) };
                    Ok(wrapped_core.into_py(py))
                })
            }
        }
    })
}


#[pymodule]
pub fn pytemporalio(py: Python<'_>, root_module: &PyModule) -> PyResult<()> {
    let errors_module = PyModule::new(py, "errors")?;
    root_module.add_submodule(errors_module)?;
    errors_module.add("WorkerRegistrationError", py.get_type::<WorkerRegistrationError>())?;
    errors_module.add("PollWfError", py.get_type::<PollWfError>())?;

    let pollers_module = PyModule::new(py, "pollers")?;
    root_module.add_submodule(pollers_module)?;
    root_module.add_function(wrap_pyfunction!(wrapped_init, root_module)?)?;
    root_module.add_class::<WrappedCore>()?;
    root_module.add_class::<WrappedCoreInitOptions>()?;

    // FIXME iterate over Vec of traits here and elsewhere somehow
    let pollers_gateway_module = PyModule::new(py, "gateway")?;
    pollers_module.add_submodule(pollers_gateway_module)?;
    pollers_gateway_module.add_class::<WrappedClientTlsConfig>()?;
    pollers_gateway_module.add_class::<WrappedServerGatewayOptions>()?;
    pollers_gateway_module.add_class::<WrappedTlsConfig>()?;

    let protos_module = PyModule::new(py, "protos")?;
    root_module.add_submodule(protos_module)?;

    let protos_activity_result_module = PyModule::new(py, "activity_result")?;
    protos_module.add_submodule(protos_activity_result_module)?;
    protos_activity_result_module.add_class::<WrappedActivityResult>()?;
    protos_activity_result_module.add_class::<WrappedSuccess>()?;
    protos_activity_result_module.add_class::<WrappedCancelation>()?;
    protos_activity_result_module.add_class::<WrappedFailure>()?;

    let protos_activity_task_module = PyModule::new(py, "activity_task")?;
    protos_module.add_submodule(protos_activity_task_module)?;
    protos_activity_task_module.add_class::<WrappedActivityTask>()?;
    protos_activity_task_module.add_class::<WrappedVariant>()?;
    protos_activity_task_module.add_class::<WrappedStart>()?;
    protos_activity_task_module.add_class::<WrappedCancel>()?;

    let protos_common_module = PyModule::new(py, "common")?;
    protos_module.add_submodule(protos_common_module)?;
    protos_common_module.add_class::<WrappedPayload>()?;
    protos_common_module.add_class::<WrappedUserCodeFailure>()?;
    protos_common_module.add_class::<WrappedWorkflowExecution>()?;
    protos_common_module.add_class::<WrappedRetryPolicy>()?;

    let protos_workflow_activation_module = PyModule::new(py, "workflow_activation")?;
    protos_module.add_submodule(protos_workflow_activation_module)?;
    protos_workflow_activation_module.add_class::<WrappedWfActivation>()?;
    protos_workflow_activation_module.add_class::<WrappedWfActivationJob>()?;
    protos_workflow_activation_module.add_class::<WrappedStartWorkflow>()?;
    protos_workflow_activation_module.add_class::<WrappedFireTimer>()?;
    protos_workflow_activation_module.add_class::<WrappedUpdateRandomSeed>()?;
    protos_workflow_activation_module.add_class::<WrappedQueryWorkflow>()?;
    protos_workflow_activation_module.add_class::<WrappedCancelWorkflow>()?;
    protos_workflow_activation_module.add_class::<WrappedSignalWorkflow>()?;
    protos_workflow_activation_module.add_class::<WrappedResolveActivity>()?;

    let worker_module = PyModule::new(py, "worker")?;
    root_module.add_submodule(worker_module)?;

    let worker_config_module = PyModule::new(py, "config")?;
    worker_module.add_submodule(worker_config_module)?;
    worker_config_module.add_class::<WrappedWorkerConfig>()?;

    Ok(())
}
