pub fn add_two(a: i32) -> i32 {
    a + 3
}

// 单元测试
#[cfg(test)]
mod test {
    use super::*;

    // 检测 panic
    #[test]
    fn it_adds_two() {
        assert!(4 == add_two(2), "check the add_two");
        assert_eq!(4, add_two(2), "check the add_two");
    }

    // 检测非 panic
    #[test]
    #[should_panic(expected = "Testing")]
    fn test_panic() {
        println!("123");
        panic!("Testing panic");
    }

    // 使用 Result
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // 忽略测试
    #[test]
    #[ignore]
    fn high_cost() {
        for i in 0..10000000 {
            continue;
        }
    }
}
