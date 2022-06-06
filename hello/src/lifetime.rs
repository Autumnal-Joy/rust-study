pub fn lifetime() {
    {
        /*
         * 无法判断返回的引用的存活作用域
         *
         * fn longest(x: &str, y: &str) -> &str {
         *     if x.len() > y.len() {
         *         x
         *     } else {
         *         y
         *     }
         * }
         */
    }

    {
        // 引用类型的泛型生命周期标注
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let str1 = "1234";
        {
            let str2 = "123456";
            dbg!(longest(str1, str2));
        }
    }

    {
        // 结构体中的生命周期标注
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        // 生命周期推测规则
        {
            // 每一个引用参数都有自己的生命周期参数
            fn foo(x: &i32, y: &i32) {}
            fn bar<'a, 'b>(x: &'a i32, y: &'b i32) {}
        }

        {
            // 如果只有一个输入生命周期参数, 那么它被赋予所有输出生命周期参数
            fn foo(x: &i32) -> (&i32, &i32) {
                (&0, &0)
            }
            fn bar<'a>(x: &'a i32) -> (&'a i32, &'a i32) {
                (&0, &0)
            }
        }

        {
            // 如果参数包含 &self 或 &mut self, 那么所有输出生命周期参数被赋予 self 的生命周期
            struct Structure<'a> {
                v: &'a str,
            }
            impl<'a> Structure<'a> {
                fn foo(&self) -> &str {
                    self.v
                }
                fn bar(&'a self) -> &'a str {
                    self.v
                }
            }
        }
    }

    {
        // 静态生命周期
        let mut x;
        let mut y;
        {
            let s: &'static str = "I have a static lifetime.";
            x = s;
            y = "I have a static lifetime.";
        }
        dbg!(&x, &y);
    }
}
