use std::ffi::{ CString, CStr} ;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn starts_with(src: *const c_char, start: *const c_char) -> bool {
    if src.is_null() || start.is_null() {
        return false;
    }

    let src_str = unsafe { CStr::from_ptr(src) };
    let start_str = unsafe { CStr::from_ptr(start) };

    let src_str = match src_str.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };

    let start_str = match start_str.to_str() {
        Ok(str) => str,
        Err(_) => return false,
    };

    src_str.starts_with(start_str)
}

// 判断字符串是否以某个字符串为结尾
// 若整个字符串, 以 end 结尾, 则返回 true, 否则, 返回 false
#[unsafe(no_mangle)]
pub extern "C" fn ends_with(src: *const c_char, end: *const c_char) -> bool {
    if src.is_null() || end.is_null() {
        return false;
    }

    let s_cstr = unsafe { CStr::from_ptr(src) };
    let end_cstr = unsafe { CStr::from_ptr(end) };

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
    if src.is_null() || contain.is_null() {
        return false;
    }
    
    let s_cstr = unsafe { CStr::from_ptr(src) };
    let contain_cstr = unsafe { CStr::from_ptr(contain) };

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

    if src.is_null() {
        return std::ptr::null_mut();
    }
    
    let s_cstr = unsafe { CStr::from_ptr(src) };
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
    if src.is_null() {
        return std::ptr::null_mut();
    }
    
    let s_cstr = unsafe { CStr::from_ptr(src) };
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

// 使用指定分隔符分割字符串
// 返回一个以 NULL 结尾的字符串数组，调用者需要使用 free_split_result 释放内存
// 如果分割失败，返回 NULL
#[unsafe(no_mangle)]
pub extern "C" fn split(src: *const c_char, delimiter: *const c_char) -> *mut *mut c_char {
    if src.is_null() || delimiter.is_null() {
        return std::ptr::null_mut();
    }
    
    let s_cstr = unsafe { CStr::from_ptr(src) };
    let delimiter_cstr = unsafe { CStr::from_ptr(delimiter) };

    let s_str = match s_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    let delimiter_str = match delimiter_cstr.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    // 使用 split 方法分割字符串
    let parts: Vec<&str> = s_str.split(delimiter_str).collect();

    // 创建 C 字符串数组，需要额外一个位置存放 NULL 终止符
    let mut c_strings = Vec::with_capacity(parts.len() + 1);

    // 转换每个部分为 C 字符串
    for part in parts {
        match CString::new(part) {
            Ok(cstring) => c_strings.push(cstring.into_raw()),
            Err(_) => {
                // 如果转换失败，需要清理已分配的内存
                for ptr in c_strings {
                    unsafe {
                        let _ = CString::from_raw(ptr);
                    }
                }
                return std::ptr::null_mut();
            }
        }
    }

    // 添加 NULL 终止符
    c_strings.push(std::ptr::null_mut());

    // 将 Vec 转换为原始指针数组
    let boxed_array = c_strings.into_boxed_slice();
    Box::into_raw(boxed_array) as *mut *mut c_char
}

// 释放 split 函数返回的字符串数组内存
// 必须与 split 函数配对使用，否则会造成内存泄漏
#[unsafe(no_mangle)]
pub extern "C" fn free_split_result(result: *mut *mut c_char) {
    if result.is_null() {
        return;
    }

    unsafe {
        // 计算数组长度（直到遇到 NULL）
        let mut len = 0;
        let mut ptr = result;
        while !(*ptr).is_null() {
            len += 1;
            ptr = ptr.add(1);
        }

        // 释放每个字符串
        for i in 0..len {
            let string_ptr = *result.add(i);
            if !string_ptr.is_null() {
                let _ = CString::from_raw(string_ptr);
            }
        }

        // 释放数组本身
        let _ = Box::from_raw(std::slice::from_raw_parts_mut(result, len + 1));
    }
}