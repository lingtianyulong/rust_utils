#[cfg(test)]
mod tests {
    // use super::*;
    use std::ffi::CString;
    use crate::string_util::*;
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

    #[test]
    fn test_to_uppercase() {
        let s = CString::new("Hello World").unwrap();
        let result = unsafe { CString::from_raw(to_uppercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "HELLO WORLD");

        let s = CString::new("hello123").unwrap();
        let result = unsafe { CString::from_raw(to_uppercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "HELLO123");

        let s = CString::new("你好世界").unwrap();
        let result = unsafe { CString::from_raw(to_uppercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "你好世界");

        let s = CString::new("").unwrap();
        let result = unsafe { CString::from_raw(to_uppercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "");

        assert!(to_uppercase(std::ptr::null()).is_null());
    }

    #[test]
    fn test_to_lowercase() {
        let s = CString::new("HELLO WORLD").unwrap();
        let result = unsafe { CString::from_raw(to_lowercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "hello world");

        let s = CString::new("HELLO123").unwrap();
        let result = unsafe { CString::from_raw(to_lowercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "hello123");

        let s = CString::new("你好世界").unwrap();
        let result = unsafe { CString::from_raw(to_lowercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "你好世界");

        let s = CString::new("").unwrap();
        let result = unsafe { CString::from_raw(to_lowercase(s.as_ptr())) };
        assert_eq!(result.to_str().unwrap(), "");

        assert!(to_lowercase(std::ptr::null()).is_null());
    }
}