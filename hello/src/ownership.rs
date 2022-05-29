#![allow(unused)]

pub fn ownership() {
    // 保存在栈上的标量, 赋值时以值拷贝
    {
        let x = 5;
        let y = x;
        dbg!(x, y);
    }

    // 保存在堆上的数据, 赋值时转移所有权
    {
        let x = String::from("hello");
        let y = x;
        dbg!(&y);
    }

    // 使用 clone 创建副本, 赋值时转移副本的所有权
    {
        let x = String::from("hello");
        let y = x.clone();
        println!("{x}, {y}");
    }

    // 传参时转移所有权, 函数表达式赋值时转移所有权
    {
        fn func(s: String) -> String {
            format!("{s}")
        }
        let x = String::from("hello");
        let y = func(x);
        dbg!(&y);
    }
}
