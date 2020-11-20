
macro_rules! fn_get_string {
     ($fn_name:ident, $ptr:ty, $var:ident) => {

    #[no_mangle]
    /// Retrieve the identifier from a location
    pub unsafe extern "C" fn $fn_name (
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
}