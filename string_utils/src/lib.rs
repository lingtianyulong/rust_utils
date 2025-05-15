use std::ffi::CStr;
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

    s_str.contains(contain_str)
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    #[test]
    fn test_starts_with() {
        let s = CString::new("Hello World").unwrap();
        let prefix = CString::new("Hello").unwrap();
        assert!(starts_with(s.as_ptr(), prefix.as_ptr()));

        let s = CString::new("Hello World").unwrap();
        let prefix = CString::new("World").unwrap(); 
        assert!(!starts_with(s.as_ptr(), prefix.as_ptr()));

        let s = CString::new("你好世界").unwrap();
        let prefix = CString::new("你好").unwrap();
        assert!(starts_with(s.as_ptr(), prefix.as_ptr()));

        let s = CString::new("").unwrap();
        let prefix = CString::new("").unwrap();
        assert!(starts_with(s.as_ptr(), prefix.as_ptr()));
    }
    
    #[test]
    fn test_ends_with() {
        let s = CString::new("Hello World").unwrap();
        let end = CString::new("World").unwrap();
        assert!(ends_with(s.as_ptr(), end.as_ptr()));

        let s = CString::new("Hello World").unwrap();
        let end = CString::new("Hello").unwrap();
        assert!(!ends_with(s.as_ptr(), end.as_ptr()));  

        let s = CString::new("你好世界").unwrap();
        let end = CString::new("世界").unwrap();
        assert!(ends_with(s.as_ptr(), end.as_ptr()));   

        let s = CString::new("").unwrap();
        let end = CString::new("").unwrap();
        assert!(ends_with(s.as_ptr(), end.as_ptr()));
    }

    #[test]
    fn test_contains() {
        let s = CString::new("Hello World").unwrap();
        let contain = CString::new("World").unwrap();
        assert!(contains(s.as_ptr(), contain.as_ptr()));
    }
    

}
