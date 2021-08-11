use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3_chrono;
use temporal_sdk_core::WorkerConfig;

#[pyclass(name = "WorkerConfig")]
#[derive(Clone)]
pub struct WrappedWorkerConfig {
    pub(crate) internal: WorkerConfig,
}


#[pymethods]
impl WrappedWorkerConfig {
    // FIXME defaults
    #[new]
    fn new(task_queue: String,
           max_outstanding_workflow_tasks: usize,
           max_outstanding_activities: usize,
           max_concurrent_wft_polls: usize,
           nonsticky_to_sticky_poll_ratio: f32,
           max_concurrent_at_polls: usize,
           no_remote_activities: bool,
           sticky_queue_schedule_to_start_timeout: pyo3_chrono::Duration) -> PyResult<Self> {

        // FIXME where does ".0" point to?
        let converted_sticky_queue_schedule_to_start_timeout = match sticky_queue_schedule_to_start_timeout.0.to_std() {
            Ok(timeout) => { timeout }
            Err(e) => return Err(PyValueError::new_err(format!(
                "{}",
                e.to_string()
            ))),
        };

        Ok(WrappedWorkerConfig {
            internal: WorkerConfig {
                task_queue,
                max_outstanding_workflow_tasks,
                max_outstanding_activities,
                max_concurrent_wft_polls,
                nonsticky_to_sticky_poll_ratio,
                max_concurrent_at_polls,
                no_remote_activities,
                sticky_queue_schedule_to_start_timeout: converted_sticky_queue_schedule_to_start_timeout,
            }
        })
    }
}
