use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3_chrono;
use temporal_sdk_core::{
    ClientTlsConfig,
    ServerGatewayOptions,
    TlsConfig,
    Url,
};

use crate::utils::pyo3_chrono_duration_to_std_duration;

#[pyclass(name = "ClientTlsConfig")]
#[derive(Clone)]
pub struct WrappedClientTlsConfig {
    pub(crate) internal: ClientTlsConfig,
}

#[pymethods]
impl WrappedClientTlsConfig {
    #[new]
    fn new(client_cert: Vec<u8>, client_private_key: Vec<u8>) -> Self {
        WrappedClientTlsConfig {
            internal: ClientTlsConfig {
                client_cert,
                client_private_key,
            }
        }
    }
}


#[pyclass(name = "TlsConfig")]
#[derive(Clone)]
pub struct WrappedTlsConfig {
    pub(crate) internal: TlsConfig,
}

#[pymethods]
impl WrappedTlsConfig {
    #[new]
    fn new(server_root_ca_cert: Option<Vec<u8>>,
           domain: Option<String>,
           client_tls_config: Option<WrappedClientTlsConfig>) -> Self {
        WrappedTlsConfig {
            internal: TlsConfig {
                server_root_ca_cert,
                domain,
                client_tls_config: match client_tls_config {
                    None => None,
                    Some(i) => Some(i.internal),
                },
            }
        }
    }
}


#[pyclass(name = "ServerGatewayOptions")]
#[derive(Clone)]
pub struct WrappedServerGatewayOptions {
    pub(crate) internal: ServerGatewayOptions,
}


#[pymethods]
impl WrappedServerGatewayOptions {
    #[new]
    fn new(target_url: String,
           namespace: String,
           identity: String,
           worker_binary_id: String,
           long_poll_timeout: pyo3_chrono::Duration,
           tls_cfg: Option<WrappedTlsConfig>) -> PyResult<Self> {
        let parsed_target_url = match Url::parse(&target_url) {
            Ok(url) => { url }
            Err(e) => return Err(PyValueError::new_err(format!(
                "{}",
                e.to_string()
            ))),
        };

        Ok(WrappedServerGatewayOptions {
            internal: ServerGatewayOptions {
                target_url: parsed_target_url,
                namespace,
                identity,
                worker_binary_id,
                long_poll_timeout: pyo3_chrono_duration_to_std_duration(long_poll_timeout)?,
                tls_cfg: match tls_cfg {
                    None => None,
                    Some(i) => Some(i.internal),
                },
            }
        })
    }
}
