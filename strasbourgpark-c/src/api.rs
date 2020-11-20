use strasbourgpark::api::{LocationOpenData, Record};
use libc::{c_char, c_int};

fn_get_string!(strasbourgpark_location_get_id, LocationOpenData, id);

