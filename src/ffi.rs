use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json::Value;
use serde::{Serialize, Deserialize};

use crate::jwt::JWT;

#[derive(Serialize, Deserialize)]
pub struct FfiResult {
    success: bool,
    data: serde_json::Value,
    error: String,
}

#[no_mangle]
pub extern "C" fn generate(secret: *const c_char, claims: *const c_char) -> *mut c_char {
    let secret = unsafe { CStr::from_ptr(secret).to_str().unwrap() };
    let claims = unsafe { CStr::from_ptr(claims).to_str().unwrap() };

    let jwt = JWT::new(secret.to_string());

    let value: Value = serde_json::from_str(claims).unwrap();
    let token = jwt.generate(&value);
    CString::new(token).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_claims(secret: *const c_char, value: *const c_char) -> *mut c_char {
    let secret = unsafe { CStr::from_ptr(secret).to_str().unwrap() };
    let value = unsafe { CStr::from_ptr(value).to_str().unwrap() };
    
    let jwt = JWT::new(secret.to_string());

    match jwt.get_claims::<serde_json::Value>(value) {
        Ok(claims) => {
            let result = FfiResult {
                success: true,
                data: claims,
                error: "".to_string(),
            };
            CString::new(
                serde_json::to_string(&result).unwrap(),
            ).unwrap().into_raw()
        },
        Err(error) => {
            let result = FfiResult { 
                success: false,
                data: serde_json::Value::Null,
                error,
            };
            CString::new(
                serde_json::to_string(&result).unwrap(),
            ).unwrap().into_raw()
        },
    }
}

#[no_mangle]
pub extern "C" fn validate(secret: *const c_char, token: *const c_char) -> bool {
    let secret = unsafe { CStr::from_ptr(secret).to_str().unwrap() };
    let token = unsafe { CStr::from_ptr(token).to_str().unwrap() };

    let jwt = JWT::new(secret.to_string());

    match jwt.validate(token) {
        Ok(_) => true,
        Err(_) => false,
    }
}
