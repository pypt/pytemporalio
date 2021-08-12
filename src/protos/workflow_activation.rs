use std::collections::HashMap;

use pyo3::prelude::*;
use temporal_sdk_core::protos::coresdk::{
    workflow_activation::{
        StartWorkflow,
        FireTimer,
        UpdateRandomSeed,
        QueryWorkflow,
        CancelWorkflow,
        SignalWorkflow,
        ResolveActivity,
        WfActivationJob,
        wf_activation_job,
        WfActivation,
    },
    activity_result::ActivityResult,
};

use crate::protos::{
    common::WrappedPayload,
    activity_result::WrappedActivityResult,
};
use crate::utils::{
    prost_types_timestamp_to_u128,
    u128_to_prost_types_timestamp,
    vec_of_payloads_to_vec_of_wrapped_payloads,
    vec_of_wrapped_payloads_to_vec_of_payloads,
    hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads,
    hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads,
};

#[pyclass(name = "StartWorkflow")]
#[derive(Clone)]
pub struct WrappedStartWorkflow {
    pub workflow_type: String,
    pub workflow_id: String,
    pub arguments: Vec<WrappedPayload>,
    pub randomness_seed: u64,
    pub headers: HashMap<String, WrappedPayload>,
}

#[pymethods]
impl WrappedStartWorkflow {
    #[new]
    fn new(workflow_type: String,
           workflow_id: String,
           arguments: Vec<WrappedPayload>,
           randomness_seed: u64,
           headers: HashMap<String, WrappedPayload>) -> Self {
        WrappedStartWorkflow {
            workflow_type,
            workflow_id,
            arguments,
            randomness_seed,
            headers,
        }
    }
}


impl From<StartWorkflow> for WrappedStartWorkflow {
    fn from(i: StartWorkflow) -> Self {
        WrappedStartWorkflow {
            workflow_type: i.workflow_type,
            workflow_id: i.workflow_id,
            arguments: vec_of_payloads_to_vec_of_wrapped_payloads(i.arguments),
            randomness_seed: i.randomness_seed,
            headers: hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(i.headers),
        }
    }
}


impl From<WrappedStartWorkflow> for StartWorkflow {
    fn from(i: WrappedStartWorkflow) -> Self {
        StartWorkflow {
            workflow_type: i.workflow_type,
            workflow_id: i.workflow_id,
            arguments: vec_of_wrapped_payloads_to_vec_of_payloads(i.arguments),
            randomness_seed: i.randomness_seed,
            headers: hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(i.headers),
        }
    }
}


#[pyclass(name = "FireTimer")]
#[derive(Clone)]
pub struct WrappedFireTimer {
    pub timer_id: String,
}

#[pymethods]
impl WrappedFireTimer {
    #[new]
    fn new(timer_id: String) -> Self {
        WrappedFireTimer {
            timer_id,
        }
    }
}


impl From<FireTimer> for WrappedFireTimer {
    fn from(i: FireTimer) -> Self {
        WrappedFireTimer {
            timer_id: i.timer_id,
        }
    }
}

impl From<WrappedFireTimer> for FireTimer {
    fn from(i: WrappedFireTimer) -> Self {
        FireTimer {
            timer_id: i.timer_id,
        }
    }
}


#[pyclass(name = "UpdateRandomSeed")]
#[derive(Clone)]
pub struct WrappedUpdateRandomSeed {
    pub randomness_seed: u64,
}

#[pymethods]
impl WrappedUpdateRandomSeed {
    #[new]
    fn new(randomness_seed: u64) -> Self {
        WrappedUpdateRandomSeed {
            randomness_seed,
        }
    }
}


impl From<UpdateRandomSeed> for WrappedUpdateRandomSeed {
    fn from(i: UpdateRandomSeed) -> Self {
        WrappedUpdateRandomSeed {
            randomness_seed: i.randomness_seed,
        }
    }
}

impl From<WrappedUpdateRandomSeed> for UpdateRandomSeed {
    fn from(i: WrappedUpdateRandomSeed) -> Self {
        UpdateRandomSeed {
            randomness_seed: i.randomness_seed,
        }
    }
}


#[pyclass(name = "QueryWorkflow")]
#[derive(Clone)]
pub struct WrappedQueryWorkflow {
    pub query_id: String,
    pub query_type: String,
    pub arguments: Vec<WrappedPayload>,
}

#[pymethods]
impl WrappedQueryWorkflow {
    #[new]
    fn new(query_id: String,
           query_type: String,
           arguments: Vec<WrappedPayload>) -> Self {
        WrappedQueryWorkflow {
            query_id,
            query_type,
            arguments,
        }
    }
}

impl From<QueryWorkflow> for WrappedQueryWorkflow {
    fn from(i: QueryWorkflow) -> Self {
        WrappedQueryWorkflow {
            query_id: i.query_id,
            query_type: i.query_type,
            arguments: vec_of_payloads_to_vec_of_wrapped_payloads(i.arguments),
        }
    }
}

impl From<WrappedQueryWorkflow> for QueryWorkflow {
    fn from(i: WrappedQueryWorkflow) -> Self {
        QueryWorkflow {
            query_id: i.query_id,
            query_type: i.query_type,
            arguments: vec_of_wrapped_payloads_to_vec_of_payloads(i.arguments),
        }
    }
}


#[pyclass(name = "CancelWorkflow")]
#[derive(Clone)]
pub struct WrappedCancelWorkflow {
    pub details: Vec<WrappedPayload>,
}

#[pymethods]
impl WrappedCancelWorkflow {
    #[new]
    fn new(details: Vec<WrappedPayload>) -> Self {
        WrappedCancelWorkflow {
            details
        }
    }
}


impl From<CancelWorkflow> for WrappedCancelWorkflow {
    fn from(i: CancelWorkflow) -> Self {
        WrappedCancelWorkflow {
            details: vec_of_payloads_to_vec_of_wrapped_payloads(i.details),
        }
    }
}


impl From<WrappedCancelWorkflow> for CancelWorkflow {
    fn from(i: WrappedCancelWorkflow) -> Self {
        CancelWorkflow {
            details: vec_of_wrapped_payloads_to_vec_of_payloads(i.details),
        }
    }
}


#[pyclass(name = "SignalWorkflow")]
#[derive(Clone)]
pub struct WrappedSignalWorkflow {
    pub signal_name: String,
    pub input: Vec<WrappedPayload>,
    pub identity: String,
}

#[pymethods]
impl WrappedSignalWorkflow {
    #[new]
    fn new(signal_name: String,
           input: Vec<WrappedPayload>,
           identity: String) -> Self {
        WrappedSignalWorkflow {
            signal_name,
            input,
            identity,
        }
    }
}

impl From<SignalWorkflow> for WrappedSignalWorkflow {
    fn from(i: SignalWorkflow) -> Self {
        WrappedSignalWorkflow {
            signal_name: i.signal_name,
            input: vec_of_payloads_to_vec_of_wrapped_payloads(i.input),
            identity: i.identity,
        }
    }
}

impl From<WrappedSignalWorkflow> for SignalWorkflow {
    fn from(i: WrappedSignalWorkflow) -> Self {
        SignalWorkflow {
            signal_name: i.signal_name,
            input: vec_of_wrapped_payloads_to_vec_of_payloads(i.input),
            identity: i.identity,
        }
    }
}


#[pyclass(name = "ResolveActivity")]
#[derive(Clone)]
pub struct WrappedResolveActivity {
    pub activity_id: String,
    pub result: Option<WrappedActivityResult>,
}

#[pymethods]
impl WrappedResolveActivity {
    #[new]
    fn new(activity_id: String,
           result: Option<WrappedActivityResult>) -> Self {
        WrappedResolveActivity {
            activity_id,
            result,
        }
    }
}


impl From<ResolveActivity> for WrappedResolveActivity {
    fn from(i: ResolveActivity) -> Self {
        WrappedResolveActivity {
            activity_id: i.activity_id,
            result: match i.result {
                None => None,
                Some(result) => Some(WrappedActivityResult::from(result)),
            },
        }
    }
}

impl From<WrappedResolveActivity> for ResolveActivity {
    fn from(i: WrappedResolveActivity) -> Self {
        ResolveActivity {
            activity_id: i.activity_id,
            result: match i.result {
                None => None,
                Some(result) => Some(ActivityResult::from(result)),
            },
        }
    }
}


#[pyclass(name = "Variant")]
#[derive(Clone)]
pub struct WrappedVariant {
    pub start_workflow: Option<WrappedStartWorkflow>,
    pub fire_timer: Option<WrappedFireTimer>,
    pub update_random_seed: Option<WrappedUpdateRandomSeed>,
    pub query_workflow: Option<WrappedQueryWorkflow>,
    pub cancel_workflow: Option<WrappedCancelWorkflow>,
    pub signal_workflow: Option<WrappedSignalWorkflow>,
    pub resolve_activity: Option<WrappedResolveActivity>,
    pub remove_from_cache: Option<bool>,
}

#[pymethods]
impl WrappedVariant {
    #[new]
    fn new(start_workflow: Option<WrappedStartWorkflow>,
           fire_timer: Option<WrappedFireTimer>,
           update_random_seed: Option<WrappedUpdateRandomSeed>,
           query_workflow: Option<WrappedQueryWorkflow>,
           cancel_workflow: Option<WrappedCancelWorkflow>,
           signal_workflow: Option<WrappedSignalWorkflow>,
           resolve_activity: Option<WrappedResolveActivity>,
           remove_from_cache: Option<bool>) -> Self {
        WrappedVariant {
            start_workflow,
            fire_timer,
            update_random_seed,
            query_workflow,
            cancel_workflow,
            signal_workflow,
            resolve_activity,
            remove_from_cache,
        }
    }
}

impl From<wf_activation_job::Variant> for WrappedVariant {
    fn from(i: wf_activation_job::Variant) -> Self {
        match i {
            wf_activation_job::Variant::StartWorkflow(start_workflow_job) => WrappedVariant {
                start_workflow: Some(WrappedStartWorkflow::from(start_workflow_job)),
                fire_timer: None,
                update_random_seed: None,
                query_workflow: None,
                cancel_workflow: None,
                signal_workflow: None,
                resolve_activity: None,
                remove_from_cache: None,
            },
            wf_activation_job::Variant::FireTimer(fire_timer_job) => WrappedVariant {
                start_workflow: None,
                fire_timer: Some(WrappedFireTimer::from(fire_timer_job)),
                update_random_seed: None,
                query_workflow: None,
                cancel_workflow: None,
                signal_workflow: None,
                resolve_activity: None,
                remove_from_cache: None,
            },
            wf_activation_job::Variant::UpdateRandomSeed(update_random_seed_job) => WrappedVariant {
                start_workflow: None,
                fire_timer: None,
                update_random_seed: Some(WrappedUpdateRandomSeed::from(update_random_seed_job)),
                query_workflow: None,
                cancel_workflow: None,
                signal_workflow: None,
                resolve_activity: None,
                remove_from_cache: None,
            },
            wf_activation_job::Variant::QueryWorkflow(query_workflow_job) => WrappedVariant {
                start_workflow: None,
                fire_timer: None,
                update_random_seed: None,
                query_workflow: Some(WrappedQueryWorkflow::from(query_workflow_job)),
                cancel_workflow: None,
                signal_workflow: None,
                resolve_activity: None,
                remove_from_cache: None,
            },
            wf_activation_job::Variant::CancelWorkflow(cancel_workflow_job) => WrappedVariant {
                start_workflow: None,
                fire_timer: None,
                update_random_seed: None,
                query_workflow: None,
                cancel_workflow: Some(WrappedCancelWorkflow::from(cancel_workflow_job)),
                signal_workflow: None,
                resolve_activity: None,
                remove_from_cache: None,
            },
            wf_activation_job::Variant::SignalWorkflow(signal_workflow_job) => WrappedVariant {
                start_workflow: None,
                fire_timer: None,
                update_random_seed: None,
                query_workflow: None,
                cancel_workflow: None,
                signal_workflow: Some(WrappedSignalWorkflow::from(signal_workflow_job)),
                resolve_activity: None,
                remove_from_cache: None,
            },
            wf_activation_job::Variant::ResolveActivity(resolve_activity_job) => WrappedVariant {
                start_workflow: None,
                fire_timer: None,
                update_random_seed: None,
                query_workflow: None,
                cancel_workflow: None,
                signal_workflow: None,
                resolve_activity: Some(WrappedResolveActivity::from(resolve_activity_job)),
                remove_from_cache: None,
            },
            wf_activation_job::Variant::RemoveFromCache(remove_from_cache) => WrappedVariant {
                start_workflow: None,
                fire_timer: None,
                update_random_seed: None,
                query_workflow: None,
                cancel_workflow: None,
                signal_workflow: None,
                resolve_activity: None,
                remove_from_cache: Some(remove_from_cache),
            },
        }
    }
}

impl From<WrappedVariant> for wf_activation_job::Variant {
    fn from(i: WrappedVariant) -> Self {
        if let Some(start_workflow_job) = i.start_workflow {
            return wf_activation_job::Variant::StartWorkflow(StartWorkflow::from(start_workflow_job));
        } else if let Some(fire_timer_job) = i.fire_timer {
            return wf_activation_job::Variant::FireTimer(FireTimer::from(fire_timer_job));
        } else if let Some(update_random_seed_job) = i.update_random_seed {
            return wf_activation_job::Variant::UpdateRandomSeed(UpdateRandomSeed::from(update_random_seed_job));
        } else if let Some(query_workflow_job) = i.query_workflow {
            return wf_activation_job::Variant::QueryWorkflow(QueryWorkflow::from(query_workflow_job));
        } else if let Some(cancel_workflow_job) = i.cancel_workflow {
            return wf_activation_job::Variant::CancelWorkflow(CancelWorkflow::from(cancel_workflow_job));
        } else if let Some(signal_workflow_job) = i.signal_workflow {
            return wf_activation_job::Variant::SignalWorkflow(SignalWorkflow::from(signal_workflow_job));
        } else if let Some(resolve_activity_job) = i.resolve_activity {
            return wf_activation_job::Variant::ResolveActivity(ResolveActivity::from(resolve_activity_job));
        } else if let Some(remove_from_cache) = i.remove_from_cache {
            return wf_activation_job::Variant::RemoveFromCache(remove_from_cache);
        } else {
            panic!("Only one of job types must be set");
        }
    }
}

#[pyclass(name = "WfActivationJob")]
#[derive(Clone)]
pub struct WrappedWfActivationJob {
    pub variant: Option<WrappedVariant>,
}

#[pymethods]
impl WrappedWfActivationJob {
    #[new]
    fn new(variant: Option<WrappedVariant>) -> Self {
        WrappedWfActivationJob {
            variant,
        }
    }
}


impl From<WfActivationJob> for WrappedWfActivationJob {
    fn from(i: WfActivationJob) -> Self {
        WrappedWfActivationJob {
            variant: match i.variant {
                None => None,
                Some(variant) => Some(WrappedVariant::from(variant)),
            },
        }
    }
}

impl From<&WfActivationJob> for WrappedWfActivationJob {
    fn from(i: &WfActivationJob) -> Self {
        let variant = i.variant.clone();
        WrappedWfActivationJob {
            variant: match variant {
                None => None,
                Some(variant) => Some(WrappedVariant::from(variant)),
            },
        }
    }
}

impl From<WrappedWfActivationJob> for WfActivationJob {
    fn from(i: WrappedWfActivationJob) -> Self {
        WfActivationJob {
            variant: match i.variant {
                None => None,
                Some(variant) => Some(wf_activation_job::Variant::from(variant)),
            },
        }
    }
}

impl From<&WrappedWfActivationJob> for WfActivationJob {
    fn from(i: &WrappedWfActivationJob) -> Self {
        let variant = i.variant.clone();
        WfActivationJob {
            variant: match variant {
                None => None,
                Some(variant) => Some(wf_activation_job::Variant::from(variant)),
            },
        }
    }
}

#[pyclass(name = "WfActivation")]
#[derive(Clone)]
pub struct WrappedWfActivation {
    pub run_id: String,
    pub timestamp: Option<u128>,
    pub is_replaying: bool,
    pub jobs: Vec<WrappedWfActivationJob>,
}

#[pymethods]
impl WrappedWfActivation {
    #[new]
    fn new(run_id: String,
           timestamp: Option<u128>,
           is_replaying: bool,
           jobs: Vec<WrappedWfActivationJob>) -> Self {
        WrappedWfActivation {
            run_id,
            timestamp,
            is_replaying,
            jobs,
        }
    }
}

impl From<WfActivation> for WrappedWfActivation {
    fn from(i: WfActivation) -> Self {
        let wrapped_jobs = i.jobs.iter().map(|x| WrappedWfActivationJob::from(x)).collect::<Vec<_>>();

        WrappedWfActivation {
            run_id: i.run_id,
            timestamp: prost_types_timestamp_to_u128(i.timestamp),
            is_replaying: i.is_replaying,
            jobs: wrapped_jobs,
        }
    }
}

impl From<WrappedWfActivation> for WfActivation {
    fn from(i: WrappedWfActivation) -> Self {
        let unwrapped_jobs = i.jobs.iter().map(|x| WfActivationJob::from(x)).collect::<Vec<_>>();

        WfActivation {
            run_id: i.run_id,
            timestamp: u128_to_prost_types_timestamp(i.timestamp),
            is_replaying: i.is_replaying,
            jobs: unwrapped_jobs,
        }
    }
}