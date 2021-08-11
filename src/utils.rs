use std::convert::TryInto;
use std::collections::HashMap;
use std::time::Duration as StdDuration;

use chrono;
use prost_types::{
    Duration as ProstDuration,
    Timestamp as ProstTimestamp,
};
use pyo3::exceptions::PyValueError;
use pyo3_chrono;
use temporal_sdk_core::protos::coresdk::common::Payload;

use crate::protos::common::WrappedPayload;

pub(crate) fn pyo3_chrono_duration_to_std_duration(duration: pyo3_chrono::Duration) -> Result<StdDuration, crate::PyErr> {
    // FIXME where does ".0" point to?
    match duration.0.to_std() {
        Ok(std_duration) => { Ok(std_duration) }
        Err(e) => Err(PyValueError::new_err(format!(
            "{}",
            e.to_string()
        ))),
    }
}


pub(crate) fn std_duration_to_pyo3_chrono_duration(duration: StdDuration) -> Result<pyo3_chrono::Duration, crate::PyErr> {
    let chrono_duration = match chrono::Duration::from_std(duration) {
        Ok(result_duration) => result_duration,
        Err(err) => return Err(PyValueError::new_err(format!(
            "{}",
            err
        ))),
    };

    Ok(pyo3_chrono::Duration::from(chrono_duration))
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


pub(crate) fn pyo3_chrono_duration_to_prost_duration(duration: Option<pyo3_chrono::Duration>) -> Result<Option<ProstDuration>, crate::PyErr> {
    match duration {
        None => Ok(None),
        Some(d) => {
            let std_duration = pyo3_chrono_duration_to_std_duration(d)?;
            Ok(Some(ProstDuration::from(std_duration)))
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


// FIXME untested at all
pub(crate) fn u128_to_prost_types_timestamp(timestamp: Option<u128>) -> Option<ProstTimestamp> {
    match timestamp {
        None => None,
        Some(ts) => {
            let seconds = ts / 1000;
            let nanos = ts - (seconds * 1000);
            Some(ProstTimestamp {
                seconds: seconds.try_into().unwrap(),
                nanos: nanos.try_into().unwrap(),
            })
        }
    }
}


// FIXME rename to a shorter name
pub(crate) fn vec_of_payloads_to_vec_of_wrapped_payloads(payloads: Vec<Payload>) -> Vec<WrappedPayload> {
    payloads.iter().map(|x| WrappedPayload::from(x)).collect::<Vec<_>>()
}


// FIXME rename to a shorter name
pub(crate) fn vec_of_wrapped_payloads_to_vec_of_payloads(payloads: Vec<WrappedPayload>) -> Vec<Payload> {
    payloads.iter().map(|x| Payload::from(x)).collect::<Vec<_>>()
}


// FIXME rename to a shorter name
pub(crate) fn hashmap_of_string_payloads_to_hashmap_of_string_wrapped_payloads(payloads: HashMap<String, Payload>) -> HashMap<String, WrappedPayload> {
    // FIXME we could probably do less copying here
    payloads.iter().map(|(k, v)| (
        String::from(k),
        WrappedPayload::from(v)
    )).collect()
}


// FIXME rename to a shorter name
pub(crate) fn hashmap_of_string_wrapped_payloads_to_hashmap_of_string_payloads(payloads: HashMap<String, WrappedPayload>) -> HashMap<String, Payload> {
    // FIXME we could probably do less copying here
    payloads.iter().map(|(k, v)| (
        String::from(k),
        Payload::from(v)
    )).collect()
}
