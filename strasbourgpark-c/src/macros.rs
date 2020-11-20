macro_rules! fn_get_string {
    ($fn_suffix:ident, $var:ident, $ptr:ty) => {
        paste! {
        #[no_mangle]
        #[doc = "Retrieve the `" $var "` as const char."]
        pub unsafe extern "C" fn [< strasbourgpark_ $fn_suffix _get_ $var >](
            ptr: *const $ptr,
            $var: *mut *const c_char,
            length: *mut c_int,
        ) {
            debug_assert!(!ptr.is_null());
            let ptr = &*ptr;
            *$var = ptr.$var.as_ptr() as *const c_char;
            *length = ptr.$var.len() as c_int;
        }
        }
    };
}

macro_rules! fn_get_bool {
    ($fn_suffix:ident, $var:ident, $ptr:ty) => {
        paste! {
        #[no_mangle]
        #[doc = "Retrieve the `" $var "` as const int."]
        pub unsafe extern "C" fn [< strasbourgpark_ $fn_suffix _get_ $var >](
            ptr: *const $ptr) -> c_int {
            debug_assert!(!ptr.is_null());
            let ptr = &*ptr;
            match ptr.$var {
                true => 0,
                false => -1
            }

        }
        }
    };
}
