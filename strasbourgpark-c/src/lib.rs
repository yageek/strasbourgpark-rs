use libc::{c_char, c_int};
use strasbourgparkapi::api::{LocationOpenData, Record};

mod api;
pub unsafe extern "C" fn record_get_id<T>(
    record: *const Record<T>,
    id: *mut *const c_char,
    length: *mut c_int,
) {
    debug_assert!(!record.is_null());
    let record = &*record;
    *id = record.id.as_ptr() as *const c_char;
    *length = record.id.len() as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn strasbourgparkapi_location_get_id(
    location: *const LocationOpenData,
    id: *mut *const c_char,
    length: *mut c_int,
) {
    debug_assert!(!location.is_null());
    let location = &*location;
    *id = location.id.as_ptr() as *const c_char;
    *length = location.id.len() as c_int;
}
