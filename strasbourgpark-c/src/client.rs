use std::sync::Arc;

use crate::runtime::RUN_TIME;
use libc::{c_char, c_int, c_void};
use strasbourgpark::{api::LocationOpenData, Client, ClientError};

pub struct WrapperClient {
    inner: Arc<Client>,
}

impl WrapperClient {
    fn new() -> Result<WrapperClient, ClientError> {
        let client = Client::new()?;
        let inner = Arc::new(client);
        Ok(WrapperClient { inner })
    }
}
pub trait CCallback: Send {
    type Item;
    fn on_success(&self, result: Self::Item);
    fn on_error(&self, result: &str);
}

impl WrapperClient {
    fn get_all_locations<F>(&self, callback: F)
    where
        F: FnOnce(Result<Vec<LocationOpenData>, ClientError>) -> () + Send + 'static,
    {
        let value = Arc::clone(&self.inner);

        RUN_TIME.spawn(async move {
            let value = value.fetch_all_locations().await;
            callback(value);
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_init(client: *mut *const WrapperClient) -> c_int {
    let value = match WrapperClient::new() {
        Ok(client) => Box::into_raw(Box::new(client)),
        Err(_) => return -1,
    };

    *client = value;
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_free(client: *mut WrapperClient) {
    Box::from_raw(client);
}

// Loading of the element

#[repr(C)]
pub struct LocationCallback {
    owner: *mut c_void,
    on_success: extern "C" fn(owner: *const c_void, arg: *const c_char),
    on_error: extern "C" fn(owner: *const c_void, arg: *const c_char),
}

unsafe impl Send for LocationCallback {}
#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_get_locations(
    client: *mut WrapperClient,
    callback: LocationCallback,
) {
    let client = client.as_ref().unwrap();
    client.get_all_locations(move |result| match result {
        Ok(r) => {
            (callback.on_success)(std::ptr::null(), std::ptr::null());
        }
        Err(e) => {
            (callback.on_success)(std::ptr::null(), std::ptr::null());
        }
    })
}
