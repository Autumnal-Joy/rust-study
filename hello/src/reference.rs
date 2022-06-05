pub fn reference() {
    // 引用变量, 不获取所有权
    {
        fn func(s: &String) -> String {
            format!("{s} World")
        }
        let x = String::from("Hello");
        let y = func(&x);
        dbg!(&y);
    }

    // 使用可变引用修改变量的值
    {
        fn func(s: &mut String) {
            s.push_str(" World");
        }
        let mut x = String::from("Hello");
        let y = &mut x;
        func(y);
        dbg!(&x);
    }

    // 同时使用多个引用
    {
        fn func(s: &mut String) {
            s.push_str(" World");
        }
        let mut x = String::from("Hello");
        let y = &mut x;
        // let z = &x;
        func(y);
        dbg!(&x);
        /*
         * 用可变引用访问数据时, 不能存在其它引用
         * let y = &mut x;
         *         ------ mutable borrow occurs here
         * let z = &x;
         *         ^^ immutable borrow occurs here
         * func(y);
         *      - mutable borrow later used here
         */
    }

    // 使用切片来部分引用
    {
        let s = String::from("Hello World");
        let hello = dbg!(&s[..5]);
        let world = dbg!(&s[6..]);
    }
}
