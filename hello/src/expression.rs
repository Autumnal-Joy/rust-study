#![allow(unused)]

pub fn expression() {
    // 函数表达式
    {
        // 函数以 snake_case 命名
        fn add_i32(a: i32, b: i32) -> i32 {
            a + b
        }
        let fn_value = add_i32(1, 2);
    }

    // 块表达式
    {
        let block_value = {
            let x = 1;
            let y = 2;
            x + y
        };
    }

    // if 表达式
    {
        let score = 85i8;
        let if_value = if score >= 90 {
            'A'
        } else if score >= 80 {
            'B'
        } else {
            'C'
        };
    }

    // 循环表达式
    {
        // loop 循环
        loop {
            break;
        }

        // while 循环
        while false {
            break;
        }

        // for in 循环
        for v in ["a", "b", "c"] {
            dbg!(&v);
        }
        for i in (1..4).rev() {
            dbg!(&i);
        }

        // break/continue 指定循环标签
        'tag: loop {
            loop {
                break 'tag;
            }
        }

        // 循环表达式赋值
        let loop_value = loop {
            break 4;
        };
        dbg!(&loop_value);
    }

    // match 表达式
    {
        // 解构匹配
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        dbg!(&Some(5));
        dbg!(&plus_one(Some(5)));
        dbg!(&plus_one(None));

        // 通配模式
        fn fibonacci(x: u32) -> u32 {
            match x {
                0 => 1,
                1 => 1,
                other => fibonacci(other - 2) + fibonacci(other - 1),
            }
        }
        dbg!(&fibonacci(5));

        // 占位符
        let x = 10;
        let y = match x {
            0 => 10,
            10 => 20,
            _ => 0,
        };
        dbg!(&y);

        // if let 语法糖
        let some_u8_value = Some(5u8);
        // match some_u8_value {
        //     Some(x) => dbg!(&x),
        //     _ => println!("None"),
        // }
        if let Some(x) = some_u8_value {
            dbg!(&x);
        } else {
            println!("None");
        }
    }
}
