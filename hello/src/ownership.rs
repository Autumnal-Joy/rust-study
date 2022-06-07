pub fn ownership() {
    // 编译时大小确定的标量, 赋值时以值拷贝
    {
        let x = 5;
        let y = x;
        dbg!(&x, &y);
    }

    // 编译时大小未知的数据, 赋值时转移所有权
    {
        let x = String::from("hello");
        let y = x;
        dbg!(&y);
    }

    // 使用 clone 创建副本, 赋值时转移副本的所有权
    {
        let x = String::from("hello");
        let y = x.clone();
        dbg!(&x, &y);
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
