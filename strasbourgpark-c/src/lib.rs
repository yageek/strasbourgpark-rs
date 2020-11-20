#[macro_use]
mod macros;
mod api;

use libc::c_int;
use strasbourgpark::Client;

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
pub unsafe extern "C" strasbourg_park_client_free()
