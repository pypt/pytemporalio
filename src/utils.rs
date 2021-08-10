use pyo3::exceptions::PyValueError;

use std::time::Duration as StdDuration;

use chrono;
use pyo3_chrono;
use prost_types::{
    Duration as ProstDuration,
    Timestamp as ProstTimestamp,
};
use temporal_sdk_core::protos::coresdk::common::Payload;
use crate::protos::WrappedPayload;

// FIXME where does ".0" point to?
pub(crate) fn pyo3_chrono_duration_to_std_duration(duration: pyo3_chrono::Duration) -> Result<StdDuration, crate::PyErr> {
    match duration.0.to_std() {
        Ok(std_duration) => { Ok(std_duration) }
        Err(e) => Err(PyValueError::new_err(format!(
            "{}",
            e.to_string()
        ))),
    }
}


pub(crate) fn prost_duration_to_pyo3_chrono_duration(duration: Option<ProstDuration>) -> Result<Option<pyo3_chrono::Duration>, crate::PyErr> {
    match duration {
        None => Ok(None),
        Some(d) => {
            let seconds = chrono::Duration::seconds(d.seconds);
            let nanos = chrono::Duration::nanoseconds(d.nanos as i64);

            let zero_duration = chrono::Duration::zero();

            match zero_duration.checked_add(&seconds) {
                None => Err(PyValueError::new_err(format!(
                    "Out of bounds for seconds {}",
                    seconds
                ))),
                Some(duration_with_seconds) => match duration_with_seconds.checked_add(&nanos) {
                    None => Err(PyValueError::new_err(format!(
                        "Out of bounds for nanos {}",
                        nanos
                    ))),
                    Some(full_duration) => Ok(Some(pyo3_chrono::Duration::from(full_duration)))
                }
            }
        }
    }
}


// FIXME make sure duration since epoch works fine
pub(crate) fn prost_types_timestamp_to_u128(timestamp: Option<ProstTimestamp>) -> Option<u128> {
    match timestamp {
        None => None,
        Some(ts) => Some(ts.seconds as u128 * 1000 + ts.nanos as u128),
    }
}


pub(crate) fn vec_of_payloads_to_vec_of_wrapped_payloads(payloads: Vec<Payload>) -> Vec<WrappedPayload> {
    payloads.iter().map(|x| WrappedPayload {
        metadata: x.metadata.clone(),
        data: x.data.clone(),
    }).collect::<Vec<_>>()
}
