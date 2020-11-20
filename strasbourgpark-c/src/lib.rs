use libc::{c_char, c_int};
use strasbourgpark::api::{LocationOpenData, Record};

#[macro_use]
mod macros;
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

fn_get_string!(strasbourgpark_location_get_id, LocationOpenData, id);

