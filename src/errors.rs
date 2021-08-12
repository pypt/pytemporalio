use pyo3::create_exception;

create_exception!(pytemporalio, WorkerRegistrationError, pyo3::exceptions::PyException);
create_exception!(pytemporalio, WorkerAlreadyRegisteredForQueue, WorkerRegistrationError);

create_exception!(pytemporalio, PollWfError, pyo3::exceptions::PyException);

create_exception!(pytemporalio, PollActivityError, pyo3::exceptions::PyException);

create_exception!(pytemporalio, CompleteWfError, pyo3::exceptions::PyException);
