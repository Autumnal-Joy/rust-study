use std::fmt;

pub fn debug_display() {
    {
        //  "fmt::Debug" 特质与 "fmt::Display" 特质在基本类型中效果不完全一致
        dbg!(format!("{:?}", "3"));
        dbg!(format!("{}", "3"));
    }

    {
        // 使用 "#[derive(Debug)]" 推导 "Complex" 的 "fmt::Debug" 特质的实现
        #[derive(Debug)]
        struct Complex {
            real: i64,
            imag: i64,
        }
        dbg!(format!("{:?}", Complex { real: 3, imag: 6 }));

        // 为 "Complex" 结构体实现 "fmt::Display" 特质
        impl fmt::Display for Complex {
            // 这个 trait 要求 "fmt" 使用与下面的函数完全一致的函数签名
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                /*
                 * 仅将 "self" 的第一个元素写入到给定的输出流 "f"
                 * 返回 "fmt:Result", 此结果表明操作成功或失败
                 * 注意 "write!" 的用法和 "print!" 相似
                 */
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }
        dbg!(format!("{}", Complex { real: 3, imag: 6 }));
    }

    {
        // 定义一个包含单个 "Vec" 的结构体 "List"
        #[derive(Debug)]
        struct List(Vec<i32>);
        dbg!(format!("{:?}", List(vec![1, 2, 3])));

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 使用元组的下标获取值，并创建一个 "vec" 的引用
                let vec = &self.0;

                write!(f, "[")?;

                // 使用 "v" 对 "vec" 进行迭代，并用 "count" 记录迭代次数
                for (count, v) in vec.iter().enumerate() {
                    // 使用 "?" 或 "try!" 来返回错误
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{count}: {}", v)?;
                }

                // 加上配对中括号，并返回一个 fmt::Result 值。
                write!(f, "]")
            }
        }
        dbg!(format!("{}", List(vec![1, 2, 3])));
    }
}
