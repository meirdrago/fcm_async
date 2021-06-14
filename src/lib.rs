
#![allow(unused)]

use std::collections::HashMap;
use std::time::Duration;
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_asyncio;
use log::{warn, debug};
use pyo3_log::{Caching, Logger};
use fcm;
use tokio;


#[pyclass]
struct FcmClient {
    client:     Arc<fcm::Client>,
    fcm_key:    String,
    result_ok:  PyObject,
    result_err: PyObject,
}

#[pymethods]
impl FcmClient {
    #[new]
    fn new(ppy: Python, fcm_key: String) -> Self {
        FcmClient { 
            client:     Arc::new(fcm::Client::new()), 
            fcm_key:    fcm_key,
            result_ok:  (true).to_object(ppy),
            result_err: (false).to_object(ppy),
        }
    }

    #[text_signature = "($self, map, token)"]
    fn send_message(&self, ppy: Python, map: HashMap<String, String>, token: String) -> PyResult<PyObject>{
        debug!("FcmClient::send_message title  {:?}", map);
        let fcm_key = self.fcm_key.clone();
        let client  = Arc::clone(&self.client);

        let res_ok  = self.result_ok.clone();
        let res_err = self.result_err.clone();

        pyo3_asyncio::tokio::into_coroutine(ppy, async move {
            let mut message_builder = fcm::MessageBuilder::new(fcm_key.as_str(), token.as_str());
            message_builder.data(&map).unwrap();
            let res: PyObject = match client.send(message_builder.finalize()).await{
                Ok(_) => res_ok,
                Err(e) => {
                    warn!("FcmClient ERROR SENDING: {:?}", e);
                    res_err
                }
            };
            
            Ok(Python::with_gil(|py| res))
        })
    }


    #[text_signature = "($self, title, body, token)"]
    fn send_notification(&self, ppy: Python, title: String, body: String, token: String) -> PyResult<PyObject>{
        debug!("FcmClient::send_notification title: {}, body: {}", title, body);

        let fcm_key = self.fcm_key.clone();
        let client  = Arc::clone(&self.client);

        let res_ok  = self.result_ok.clone();
        let res_err = self.result_err.clone();

        pyo3_asyncio::tokio::into_coroutine(ppy, async move {
            let mut notification = fcm::NotificationBuilder::new();
            notification.title(title.as_str());
            notification.body(body.as_str());

            let mut message_builder = fcm::MessageBuilder::new(fcm_key.as_str(), token.as_str());
            message_builder.notification(notification.finalize());

            let res: PyObject = match client.send(message_builder.finalize()).await{
                Ok(_) => res_ok,
                Err(e) => {
                    warn!("FcmClient ERROR SENDING: {:?}", e);
                    res_err
                }
            };
            
            Ok(Python::with_gil(|py| res))
        })
    }
}


#[pymodule]
fn fcm_async(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_asyncio::try_init(py)?;
    pyo3_asyncio::tokio::init_multi_thread_once();
    Logger::new(py, Caching::LoggersAndLevels)?
                                        .install();

    m.add_class::<FcmClient>()?;
    Ok(())
}


