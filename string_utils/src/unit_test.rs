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


    #[test]
    fn test_split_string_rust() {
        let result = split_string_rust("apple,banana,cherry", ',');
        assert_eq!(result, vec!["apple", "banana", "cherry"]);

        let result2 = split_string_rust("hello world test", ' ');
        assert_eq!(result2, vec!["hello", "world", "test"]);

        // 测试空字符串
        let result3 = split_string_rust("", ',');
        assert_eq!(result3, Vec::<String>::new());

        // 测试只有分隔符
        let result4 = split_string_rust(",,,", ',');
        assert_eq!(result4, vec!["", "", "", ""]);

        // 测试没有分隔符
        let result5 = split_string_rust("no_delimiter", ',');
        assert_eq!(result5, vec!["no_delimiter"]);
    }


    #[test]
    fn test_starts_with() {
        use std::ffi::CString;

        let src = CString::new("Hello World").unwrap();
        let prefix = CString::new("Hello").unwrap();

        assert!(starts_with(src.as_ptr(), prefix.as_ptr()));

        let wrong_prefix = CString::new("World").unwrap();
        assert!(!starts_with(src.as_ptr(), wrong_prefix.as_ptr()));
    }

    #[test]
    fn test_performance_comparison() {
        let large_string = "word1,word2,word3,".repeat(1000);

        // 测试我们的 memchr 实现
        let start = std::time::Instant::now();
        let result1 = split_string_rust(&large_string, ',');
        let memchr_duration = start.elapsed();

        // 测试标准库实现
        let start = std::time::Instant::now();
        let result2: Vec<String> = large_string.split(',').map(|s| s.to_string()).collect();
        let std_duration = start.elapsed();

        println!("memchr 实现耗时: {:?}", memchr_duration);
        println!("标准库实现耗时: {:?}", std_duration);
        println!("结果长度: {} vs {}", result1.len(), result2.len());

        // 验证结果一致性（除了最后的空字符串）
        assert_eq!(result1.len(), result2.len() - 1); // 标准库会多一个空字符串
    }
}