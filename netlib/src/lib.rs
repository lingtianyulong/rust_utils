use std::ffi::{ CStr, CString};
use libc::{c_char, c_int};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

// 发送 POST 请求, 参数为 url 和 json 字符串
#[unsafe(no_mangle)]
pub extern "C" fn http_post(url: *const c_char, body:* const c_char) -> *mut c_char {
    if url.is_null() {
        return std::ptr::null_mut();
    }

    let url = unsafe { CStr::from_ptr(url) };
    let body = unsafe { CStr::from_ptr(body) };

    let url_str = match url.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // 安全转换 json body
    let json_body = match body.to_str() {
        Ok(body) => body,
        Err(_) => return std::ptr::null_mut(),
    };

    let client = Client::new();
    let response = match client
        .post(url_str)
        .header(CONTENT_TYPE, "application/json")
        .body(json_body.to_string())
        .send()
    {
        Ok(response) => match response.text() {
            Ok(body) => body,
            Err(_) => return std::ptr::null_mut(),
        } ,
        Err(_) => return std::ptr::null_mut(),
    };

    let c_str = CString::new(response.as_str()).unwrap();
    c_str.into_raw()
}

// 释放返回的字符串
#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        return;
    }
    unsafe {
        _ = CString::from_raw(s)
    }
}
