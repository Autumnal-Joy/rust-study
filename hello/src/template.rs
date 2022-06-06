use std::fmt::Display;

pub fn template() {
    {
        // 定义泛型结构
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl Point<f32, f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, p: Point<V, W>) -> Point<T, W> {
                Point { x: self.x, y: p.y }
            }
        }
    }

    {
        pub struct NewsArticle {
            pub author: String,
            pub content: String,
        }
        let article = NewsArticle {
            author: String::from("Author"),
            content: String::from("Paragraph"),
        };

        {
            // 定义 trait 与实现 trait
            trait Summary {
                fn summarize(&self) -> String;
            }
            impl Summary for NewsArticle {
                fn summarize(&self) -> String {
                    format!("{} (by {})", self.content, self.author)
                }
            }
            dbg!(article.summarize());
        }

        {
            // trait 的默认实现
            trait Summary {
                fn summarize(&self) -> String {
                    String::from("(Read more...)")
                }
            }
            impl Summary for NewsArticle {}
            dbg!(article.summarize());
        }

        {
            // trait 方法内部调用
            trait Summary {
                fn summarize_author(&self) -> String;
                fn summarize(&self) -> String {
                    format!("(Read more from {}...)", self.summarize_author())
                }
            }
            impl Summary for NewsArticle {
                fn summarize_author(&self) -> String {
                    format!("@{}", self.author)
                }
            }
            dbg!(article.summarize());
        }

        {
            // 在参数中使用 trait
            trait Summary {
                fn summarize(&self) -> String;
            }
            // trait bound
            fn notify1<T: Summary>(item: &T) {
                dbg!(item.summarize());
            }
            // 简化写法
            fn notify2(item: &impl Summary) {
                dbg!(item.summarize());
            }
            // 多个 trait bound
            fn notify3<T: Summary + Display>(item: &T) {
                dbg!(item.summarize());
            }
            // 简化写法
            fn notify4(item: &(impl Summary + Display)) {
                dbg!(item.summarize());
            }
            // 后置 trait bound
            fn notify5<T>(item: &T)
            where
                T: Summary + Display,
            {
                dbg!(item.summarize());
            }
        }

        {
            // 在返回值中使用 trait
            trait Summary {
                fn summarize(&self) -> String;
            }
            impl Summary for NewsArticle {
                fn summarize(&self) -> String {
                    format!("{} (by {})", self.content, self.author)
                }
            }
            fn summarizable() -> impl Summary {
                NewsArticle {
                    author: String::from("Author"),
                    content: String::from("Paragraph"),
                }
            }
        }

        {
            // 综合实现 max 函数
            fn max<T: PartialOrd + Copy>(list: &[T]) -> T {
                let mut max = list[0];
                for &item in list.iter() {
                    if item > max {
                        max = item;
                    }
                }
                max
            }
            let arr = vec![3, 1, 4, 2, 5];
            dbg!(max(&arr));
        }
    }
}
