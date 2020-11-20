use crate::runtime::RUN_TIME;
use libc::{c_char, c_int, c_void};
use strasbourgpark::{api::LocationOpenData, Client};
pub trait CCallback: Send + 'static {
    type Item;
    fn on_success(&self, result: Self::Item);
    fn on_error(&self, result: &str);
}

pub trait CClient {
    fn get_all_locations<C>(&'static self, callback: C)
    where
        C: CCallback<Item = Vec<LocationOpenData>>;
}

impl CClient for Client {
    fn get_all_locations<C>(&'static self, callback: C)
    where
        C: CCallback<Item = Vec<LocationOpenData>>,
    {
        RUN_TIME.spawn(async move {
            match self.fetch_all_locations().await {
                Ok(root) => callback.on_success(root),
                Err(_e) => callback.on_error("error"),
            }
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_init(client: *mut *const Client) -> c_int {
    let value = match Client::new() {
        Ok(client) => Box::into_raw(Box::new(client)),
        Err(_) => return -1,
    };

    *client = value;
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_free(client: *mut Client) {
    Box::from_raw(client);
}

// Loading of the element

#[repr(C)]
pub struct LocationCallback {
    owner: *mut c_void,
    on_success: extern "C" fn(owner: *mut c_void, arg: *const c_char),
    on_error: extern "C" fn(owner: *mut c_void, arg: *const c_char),
}

#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_get_locations(
    client: *mut Client,
    callback: LocationCallback,
) {
    let client = client.as_ref().unwrap();
    client.get_all_locations(callback);
}
