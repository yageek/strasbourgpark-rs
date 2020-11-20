use crate::runtime::RUN_TIME;
use libc::c_int;
use strasbourgpark::{api::LocationOpenData, Client};
trait CCallback {
    type Item;
    fn on_success(&self, result: Self::Item);
    fn on_error(&self, result: &str);
}

trait CClient {
    fn get_all_locations<C>(&self, callback: C)
    where
        C: CCallback<Item = Vec<LocationOpenData>>;
}

impl CClient for Client {
    fn get_all_locations<C>(&self, callback: C)
    where
        C: CCallback<Item = Vec<LocationOpenData>>,
    {
        RUN_TIME.spawn(async {
            let res = self.fetch_all_locations().await;
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

#[no_mangle]
pub unsafe extern "C" fn strasbourg_park_client_get_locations(client: *mut Client) {}
