use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// 判断字符串是否以某个字符串开头
// 如果整个字符串, 以 start 为起始,则返回true, 否则, 返回 false
// 如 src 为 "Hello World", start 为 "Hello" 则返回 true
#[unsafe(no_mangle)]
pub extern "C"  fn starts_with(src: *const c_char, start: *const c_char) -> bool {
    let s_cstr = unsafe {
        assert!(!src.is_null());
        CStr::from_ptr(src)
    };
    let prefix_cstr = unsafe {
        assert!(!start.is_null());
        CStr::from_ptr(start)
    };

    // 将 CStr 转换为 Rust 字符串切片
    let s_str = match s_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return false, // 如果不是有效的 UTF-8 字符串，返回 false
    };
    let prefix_str = match prefix_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };

    // 使用 starts_with 方法判断前缀
    s_str.starts_with(prefix_str)
}

// 判断字符串是否以某个字符串为结尾
// 若整个字符串, 以 end 结尾, 则返回 true, 否则, 返回 false
#[unsafe(no_mangle)]
pub extern "C" fn ends_with(src: *const c_char, end: *const c_char) -> bool {
    let s_cstr = unsafe {
        assert!(!src.is_null());
        CStr::from_ptr(src)
    };
    let end_cstr = unsafe {
        assert!(!end.is_null());
        CStr::from_ptr(end)
    };

    let s_str = match s_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };
    let end_str = match end_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };

    s_str.ends_with(end_str)
}

// 判断输入字符串中是否包含某个字符串
// 若包含, 则返回 true, 否则, 返回 false
#[unsafe(no_mangle)]
pub extern "C" fn contains(src: *const c_char, contain: *const c_char) -> bool {
    let s_cstr = unsafe {
        assert!(!src.is_null());
        CStr::from_ptr(src)
    };
    let contain_cstr = unsafe {
        assert!(!contain.is_null());
        CStr::from_ptr(contain)
    };

    let s_str = match s_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };
    let contain_str = match contain_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };

    s_str.contains(contain_str)   // 区分大小写
}

// 将字符串转换为大写形式
// 若转换成功, 返回转换后的字符串指针, 否则返回 NULL
#[unsafe(no_mangle)]
pub extern "C" fn to_uppercase(src: *const c_char) -> *mut c_char {
    let s_cstr = unsafe {
        if src.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(src)
    };

    let s_str = match s_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    let upper = s_str.to_uppercase();
    
    match CString::new(upper) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null_mut()
    }
}

// 将字符串转换为小写形式
// 若转换成功, 返回转换后的字符串指针, 否则返回 NULL
#[unsafe(no_mangle)]
pub extern "C" fn to_lowercase(src: *const c_char) -> *mut c_char {
    let s_cstr = unsafe {
        if src.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(src)
    };

    let s_str = match s_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    let upper = s_str.to_lowercase();
    
    match CString::new(upper) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null_mut()
    }
}
