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


}