use std::error::Error;
use std::ffi::{CStr, CString};
use std::time::Duration;
use libc::{c_char};
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, CONTENT_TYPE, USER_AGENT};

// 发送 POST 请求, 参数为 url 和 json 字符串
#[unsafe(no_mangle)]
pub extern "C" fn http_post(url: *const c_char, body:* const c_char) -> *mut c_char {
    println!("In http post");

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

    println!("url_str: {}", url_str);
    println!("json_body: {}", json_body);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("curl/7.88.0"));

    let client = Client::builder()
        .no_proxy()
        .default_headers(headers)
        .timeout(Duration::from_secs(20))
        .build().unwrap();

    let res = client
        .post(url_str)
        .header("Content-Type", "application/json")
        .body(json_body.to_string())
        .send();

    match res {
        Ok(resp) => {
            let text = resp.text();
            match text {
                Ok(body) => CString::new(body).unwrap().into_raw(),
                Err(e) => {
                    eprintln!("Failed to read body: {}", e);
                    std::ptr::null_mut()
                }
            }
        }
        Err(e) => {
            // 打印详细错误链
            eprintln!("POST failed: {}", e);
            let mut source = e.source();
            while let Some(inner) = source {
                eprintln!("Caused by: {}", inner);
                source = inner.source();
            }
            std::ptr::null_mut()
        }
    }
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
