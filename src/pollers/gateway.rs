use std::convert::TryFrom;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3_chrono;
use temporal_sdk_core::{
    ClientTlsConfig,
    ServerGatewayOptions,
    TlsConfig,
    Url,
};

use crate::utils::{
    pyo3_chrono_duration_to_std_duration,
    std_duration_to_pyo3_chrono_duration,
};

#[pyclass(name = "ClientTlsConfig")]
#[derive(Clone)]
pub struct WrappedClientTlsConfig {
    pub client_cert: Vec<u8>,
    pub client_private_key: Vec<u8>,
}

#[pymethods]
impl WrappedClientTlsConfig {
    #[new]
    fn new(client_cert: Vec<u8>, client_private_key: Vec<u8>) -> Self {
        WrappedClientTlsConfig {
            client_cert,
            client_private_key,
        }
    }
}

impl From<ClientTlsConfig> for WrappedClientTlsConfig {
    fn from(i: ClientTlsConfig) -> Self {
        WrappedClientTlsConfig {
            client_cert: i.client_cert,
            client_private_key: i.client_private_key,
        }
    }
}

impl From<WrappedClientTlsConfig> for ClientTlsConfig {
    fn from(i: WrappedClientTlsConfig) -> Self {
        ClientTlsConfig {
            client_cert: i.client_cert,
            client_private_key: i.client_private_key,
        }
    }
}


#[pyclass(name = "TlsConfig")]
#[derive(Clone)]
pub struct WrappedTlsConfig {
    server_root_ca_cert: Option<Vec<u8>>,
    domain: Option<String>,
    client_tls_config: Option<WrappedClientTlsConfig>,
}

#[pymethods]
impl WrappedTlsConfig {
    #[new]
    fn new(server_root_ca_cert: Option<Vec<u8>>,
           domain: Option<String>,
           client_tls_config: Option<WrappedClientTlsConfig>) -> Self {
        WrappedTlsConfig {
            server_root_ca_cert,
            domain,
            client_tls_config,
        }
    }
}


impl From<TlsConfig> for WrappedTlsConfig {
    fn from(i: TlsConfig) -> Self {
        WrappedTlsConfig {
            server_root_ca_cert: i.server_root_ca_cert,
            domain: i.domain,
            client_tls_config: match i.client_tls_config {
                None => None,
                Some(client_tls_config) => Some(WrappedClientTlsConfig::from(client_tls_config)),
            },
        }
    }
}


impl From<WrappedTlsConfig> for TlsConfig {
    fn from(i: WrappedTlsConfig) -> Self {
        TlsConfig {
            server_root_ca_cert: i.server_root_ca_cert,
            domain: i.domain,
            client_tls_config: match i.client_tls_config {
                None => None,
                Some(client_tls_config) => Some(ClientTlsConfig::from(client_tls_config)),
            },
        }
    }
}


#[pyclass(name = "ServerGatewayOptions")]
#[derive(Clone)]
pub struct WrappedServerGatewayOptions {
    target_url: String,
    namespace: String,
    identity: String,
    worker_binary_id: String,
    long_poll_timeout: pyo3_chrono::Duration,
    tls_cfg: Option<WrappedTlsConfig>,
}


#[pymethods]
impl WrappedServerGatewayOptions {
    #[new]
    fn new(target_url: String,
           namespace: String,
           identity: String,
           worker_binary_id: String,
           long_poll_timeout: pyo3_chrono::Duration,
           tls_cfg: Option<WrappedTlsConfig>) -> Self {
        WrappedServerGatewayOptions {
            target_url,
            namespace,
            identity,
            worker_binary_id,
            long_poll_timeout,
            tls_cfg,
        }
    }
}


impl TryFrom<ServerGatewayOptions> for WrappedServerGatewayOptions {
    type Error = PyErr;

    fn try_from(i: ServerGatewayOptions) -> Result<Self, Self::Error> {
        Ok(WrappedServerGatewayOptions {
            target_url: i.target_url.as_str().to_string(),
            namespace: i.namespace,
            identity: i.identity,
            worker_binary_id: i.worker_binary_id,
            long_poll_timeout: std_duration_to_pyo3_chrono_duration(i.long_poll_timeout)?,
            tls_cfg: match i.tls_cfg {
                None => None,
                Some(tls_config) => Some(WrappedTlsConfig::from(tls_config)),
            },
        })
    }
}


impl TryFrom<WrappedServerGatewayOptions> for ServerGatewayOptions {
    type Error = PyErr;

    fn try_from(i: WrappedServerGatewayOptions) -> Result<Self, Self::Error> {
        let parsed_target_url = match Url::parse(&i.target_url) {
            Ok(url) => { url }
            Err(e) => return Err(PyValueError::new_err(format!(
                "{}",
                e.to_string()
            ))),
        };

        Ok(ServerGatewayOptions {
            target_url: parsed_target_url,
            namespace: i.namespace,
            identity: i.identity,
            worker_binary_id: i.worker_binary_id,
            long_poll_timeout: pyo3_chrono_duration_to_std_duration(i.long_poll_timeout)?,
            tls_cfg: match i.tls_cfg {
                None => None,
                Some(wrapped_tls_config) => Some(TlsConfig::from(wrapped_tls_config)),
            },
        })
    }
}
