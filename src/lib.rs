use std::sync::Arc;
use std::convert::TryFrom;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyOSError;
use pyo3_asyncio;
use temporal_sdk_core::{
    init,
    Core,
    CoreInitOptions,
    ServerGatewayOptions,
    WorkerConfig,
    protos::coresdk::{
        ActivityTaskCompletion,
        ActivityHeartbeat,
        workflow_completion::WfActivationCompletion,
    },
};

mod errors;
mod pollers;
mod protos;
mod utils;
mod worker;

use errors::{
    WorkerRegistrationError,
    PollWfError,
    PollActivityError,
    CompleteWfError,
    CompleteActivityError,
};

use pollers::{
    gateway::{
        WrappedServerGatewayOptions,
        WrappedClientTlsConfig,
        WrappedTlsConfig,
    },
};

use protos::{
    WrappedActivityTaskCompletion,
    WrappedActivityHeartbeat,
    activity_result::{
        WrappedActivityResult,
        WrappedStatus,
        WrappedSuccess,
        WrappedCancelation,
        WrappedFailure,
    },
    activity_task::{
        WrappedActivityTask,
        WrappedVariant as WrappedActivityTaskVariant,
        WrappedStart,
        WrappedCancel,
    },
    common::{
        WrappedPayload,
        WrappedUserCodeFailure,
        WrappedWorkflowExecution,
        WrappedRetryPolicy,
    },
    workflow_activation::{
        WrappedWfActivation,
        WrappedWfActivationJob,
        WrappedStartWorkflow,
        WrappedFireTimer,
        WrappedUpdateRandomSeed,
        WrappedQueryWorkflow,
        WrappedCancelWorkflow,
        WrappedSignalWorkflow,
        WrappedResolveActivity,
        WrappedVariant as WrappedWorkflowActivationVariant,
    },
    workflow_commands::{
        WrappedStartTimer,
        WrappedCancelTimer,
        WrappedScheduleActivity,
        WrappedRequestCancelActivity,
        WrappedQuerySuccess,
        WrappedQueryResultVariant,
        WrappedQueryResult,
        WrappedCompleteWorkflowExecution,
        WrappedFailWorkflowExecution,
        WrappedContinueAsNewWorkflowExecution,
        WrappedCancelWorkflowExecution,
        WrappedVariant as WrappedWorkflowCommandsVariant,
        WrappedWorkflowCommand,
    },
    workflow_completion::{
        WrappedSuccess as WrappedWorkflowCompletionSuccess,
        WrappedFailure as WrappedWorkflowCompletionFailure,
        WrappedStatus as WrappedWorkflowCompletionStatus,
        WrappedWfActivationCompletion,
    },
};

use worker::config::WrappedWorkerConfig;


#[pyclass(name = "CoreInitOptions")]
#[derive(Clone)]
struct WrappedCoreInitOptions {
    pub(crate) internal: CoreInitOptions,
}

#[pymethods]
impl WrappedCoreInitOptions {
    // FIXME set default value of max_cached_workflows here
    #[new]
    fn new(gateway_opts: WrappedServerGatewayOptions, max_cached_workflows: usize) -> PyResult<Self> {
        Ok(WrappedCoreInitOptions {
            internal: CoreInitOptions {
                gateway_opts: ServerGatewayOptions::try_from(gateway_opts)?,
                max_cached_workflows,
            }
        })
    }
}


#[pyclass(name = "Core")]
struct WrappedCore {
    // FIXME rename to something more sensible
    pub(crate) internal: Arc<dyn Core>,
}

#[pymethods]
impl WrappedCore {
    fn register_worker<'p>(&self, py: Python<'p>, config: WrappedWorkerConfig) -> PyResult<&'p PyAny> {
        let worker_config = WorkerConfig::try_from(config)?;
        let internal = self.internal.clone();
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
            match internal.register_worker(worker_config).await {
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
                    Python::with_gil(|py| {
                        let wrapped_wf_activation = WrappedWfActivation::from(wf_activation);
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
                Err(err) => Err(PollActivityError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(activity_task) => {
                    Python::with_gil(|py| {
                        let wrapped_activity_task = WrappedActivityTask::try_from(activity_task)?;
                        Ok(wrapped_activity_task.into_py(py))
                    })
                }
            }
        })
    }

    fn complete_activity_task<'p>(&self, py: Python<'p>, completion: WrappedActivityTaskCompletion) -> PyResult<&'p PyAny> {
        let internal = self.internal.clone();
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
            match internal.complete_activity_task(ActivityTaskCompletion::try_from(completion)?).await {
                Err(err) => Err(CompleteActivityError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(()) => {
                    Python::with_gil(|py| Ok(py.None()))
                }
            }
        })
    }

    fn complete_workflow_task<'p>(&self, py: Python<'p>, completion: WrappedWfActivationCompletion) -> PyResult<&'p PyAny> {
        let internal = self.internal.clone();
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        pyo3_asyncio::tokio::future_into_py_with_loop(current_loop, async move {
            match internal.complete_workflow_task(WfActivationCompletion::try_from(completion)?).await {
                Err(err) => Err(CompleteWfError::new_err(format!(
                    "{}",
                    err.to_string()
                ))),
                Ok(()) => {
                    Python::with_gil(|py| Ok(py.None()))
                }
            }
        })
    }

    fn record_activity_heartbeat(&self, details: WrappedActivityHeartbeat) {
        self.internal.record_activity_heartbeat(ActivityHeartbeat::from(details))
    }

    fn request_workflow_eviction(&self, run_id: String) {
        self.internal.request_workflow_eviction(run_id.as_str())
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
    protos_module.add_class::<WrappedActivityTaskCompletion>()?;
    protos_module.add_class::<WrappedActivityHeartbeat>()?;

    let protos_activity_result_module = PyModule::new(py, "activity_result")?;
    protos_module.add_submodule(protos_activity_result_module)?;
    protos_activity_result_module.add_class::<WrappedActivityResult>()?;
    protos_activity_result_module.add_class::<WrappedStatus>()?;
    protos_activity_result_module.add_class::<WrappedSuccess>()?;
    protos_activity_result_module.add_class::<WrappedCancelation>()?;
    protos_activity_result_module.add_class::<WrappedFailure>()?;

    let protos_activity_task_module = PyModule::new(py, "activity_task")?;
    protos_module.add_submodule(protos_activity_task_module)?;
    protos_activity_task_module.add_class::<WrappedActivityTask>()?;
    protos_activity_task_module.add_class::<WrappedActivityTaskVariant>()?;
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
    protos_workflow_activation_module.add_class::<WrappedWorkflowActivationVariant>()?;

    let protos_workflow_commands_module = PyModule::new(py, "workflow_commands")?;
    protos_module.add_submodule(protos_workflow_commands_module)?;
    protos_workflow_commands_module.add_class::<WrappedStartTimer>()?;
    protos_workflow_commands_module.add_class::<WrappedCancelTimer>()?;
    protos_workflow_commands_module.add_class::<WrappedScheduleActivity>()?;
    protos_workflow_commands_module.add_class::<WrappedRequestCancelActivity>()?;
    protos_workflow_commands_module.add_class::<WrappedQuerySuccess>()?;
    protos_workflow_commands_module.add_class::<WrappedQueryResultVariant>()?;
    protos_workflow_commands_module.add_class::<WrappedQueryResult>()?;
    protos_workflow_commands_module.add_class::<WrappedCompleteWorkflowExecution>()?;
    protos_workflow_commands_module.add_class::<WrappedFailWorkflowExecution>()?;
    protos_workflow_commands_module.add_class::<WrappedContinueAsNewWorkflowExecution>()?;
    protos_workflow_commands_module.add_class::<WrappedCancelWorkflowExecution>()?;
    protos_workflow_commands_module.add_class::<WrappedWorkflowCommandsVariant>()?;
    protos_workflow_commands_module.add_class::<WrappedWorkflowCommand>()?;

    let protos_workflow_completion_module = PyModule::new(py, "workflow_completion")?;
    protos_module.add_submodule(protos_workflow_completion_module)?;
    protos_workflow_completion_module.add_class::<WrappedWorkflowCompletionSuccess>()?;
    protos_workflow_completion_module.add_class::<WrappedWorkflowCompletionFailure>()?;
    protos_workflow_completion_module.add_class::<WrappedWorkflowCompletionStatus>()?;
    protos_workflow_completion_module.add_class::<WrappedWfActivationCompletion>()?;

    let worker_module = PyModule::new(py, "worker")?;
    root_module.add_submodule(worker_module)?;

    let worker_config_module = PyModule::new(py, "config")?;
    worker_module.add_submodule(worker_config_module)?;
    worker_config_module.add_class::<WrappedWorkerConfig>()?;

    Ok(())
}
