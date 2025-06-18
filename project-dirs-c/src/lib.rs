use std::ffi::CString;
use std::os::raw::c_char;

#[repr(u8)]
pub enum FromBuilderError {
    NoError = 0,
    NullInput = 1,
    NonStrInput = 2,
    BuilderParsingFailed = 3,
    ResultSerializationFailed = 4,
}

unsafe fn fail<T, R>(error: T, error_ptr: *mut u8) -> *mut R
where
    T: Into<u8>,
{
    unsafe {
        *error_ptr = error.into();
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn project_dirs__from_builder(
    s: *const c_char,
    error: *mut FromBuilderError,
) -> *mut c_char {
    unsafe {
        let error_msg: *mut c_char = std::ptr::null_mut();
        let result = project_dirs__from_builder_with_msg(s, error, error_msg, 0);

        if !error_msg.is_null() {
            let _error_msg = CString::from_raw(error_msg); // we need to drop it
        }

        result
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn project_dirs__project_dirs(
    application: *const c_char,
    organization: *const c_char,
    qualifier: *const c_char,
) -> *mut c_char {
    unsafe {
        let application = std::ffi::CStr::from_ptr(application).to_str().unwrap_or("");
        let organization = std::ffi::CStr::from_ptr(organization)
            .to_str()
            .unwrap_or("");
        let qualifier = std::ffi::CStr::from_ptr(qualifier).to_str().unwrap_or("");

        let project = project_dirs::Project::new(application, organization, qualifier);
        let dirs = project.project_dirs();

        let result = CString::new(serde_json::to_string(&dirs).unwrap()).unwrap();
        let result_ptr = result.as_ptr() as *mut c_char;
        std::mem::forget(result);
        result_ptr
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn project_dirs__from_builder_with_msg(
    s: *const c_char,
    error: *mut FromBuilderError,
    buf_error_msg: *mut c_char,
    buf_error_len: usize,
) -> *mut c_char {
    unsafe {
        if s.is_null() {
            return fail(FromBuilderError::NullInput as u8, error as *mut u8);
        }

        let s = s as *const i8;
        let s = std::ffi::CStr::from_ptr(s).to_str();

        if !s.is_ok() {
            return fail(FromBuilderError::NonStrInput as u8, error as *mut u8);
        }
        let s = s.unwrap();

        let builder: Result<project_dirs_builder::Builder, _> = serde_json::from_str(s);

        if let Err(err) = builder {
            if !buf_error_msg.is_null() {
                let err_msg = err.to_string();
                let err_msg_ptr = err_msg.as_ptr() as *mut c_char;
                let bytes_to_copy = if err_msg.len() > buf_error_len {
                    buf_error_len
                } else {
                    err_msg.len()
                };
                std::ptr::copy_nonoverlapping(err_msg_ptr, buf_error_msg, bytes_to_copy);
            }
            return fail(
                FromBuilderError::BuilderParsingFailed as u8,
                error as *mut u8,
            );
        }

        let builder = builder.unwrap();

        let result_str = serde_json::to_string(&builder.build());
        if !result_str.is_ok() {
            return fail(
                FromBuilderError::ResultSerializationFailed as u8,
                error as *mut u8,
            );
        }
        let result_str = CString::new(result_str.unwrap()).unwrap();
        let result_ptr = result_str.as_ptr() as *mut c_char;
        std::mem::forget(result_str);

        result_ptr
    }
}
