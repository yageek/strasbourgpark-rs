use libc::{c_char, c_int};
use paste::paste;
use strasbourgpark::api::{Location as RLocation, LocationOpenData};

#[repr(C)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

impl From<RLocation> for Coordinate {
    fn from(location: RLocation) -> Coordinate {
        Coordinate {
            lat: location.latitude,
            lng: location.longitude,
        }
    }
}
// LocationOpenData
fn_get_string!(location, id, LocationOpenData);
fn_get_string!(location, city, LocationOpenData);
fn_get_string!(location, zipcode, LocationOpenData);
fn_get_string!(location, street, LocationOpenData);
fn_get_string!(location, address, LocationOpenData);
fn_get_string!(location, url, LocationOpenData);
fn_get_string!(location, name, LocationOpenData);
fn_get_bool!(location, deaf_access, LocationOpenData);
fn_get_bool!(location, elder_access, LocationOpenData);
fn_get_bool!(location, wheelchair_access, LocationOpenData);
fn_get_bool!(location, blind_access, LocationOpenData);

#[no_mangle]
pub unsafe extern "C" fn strasbourgpark_location_get_coordinate(
    ptr: *const LocationOpenData,
) -> Coordinate {
    debug_assert!(!ptr.is_null());
    let ptr = &*ptr;
    ptr.location.into()
}

#[no_mangle]
pub unsafe extern "C" fn strasbourgpark_location_get_description(
    ptr: *const LocationOpenData,
    description: *mut *const c_char,
    length: *mut c_int,
) {
    debug_assert!(!ptr.is_null());
    let ptr = &*ptr;
    if let Some(desc) = &ptr.description {
        *description = desc.as_ptr() as *const c_char;
        *length = desc.len() as c_int;
    } else {
        *length = 0;
        *description = std::ptr::null();
    }
}

#[repr(C)]
pub struct LocationOpenDataNative {
    
}
