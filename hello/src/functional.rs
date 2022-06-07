pub fn functional() {
    {
        // 闭包 closure, 本质是自动捕获外界变量的 lambda 表达式
        let x = 10;
        let equal_to_x = |y| x == y;
    }

    {
        // Fn trait
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }

        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
    }

    {
        // 迭代器
        let vi = vec![1, 2, 3];
        let it = vi.iter();
        for i in it {
            dbg!(&i);
        }
    }

    {
        // 消费适配器: sum
        let it = vec![1, 2, 3].into_iter();
        let total: i32 = it.sum();
        dbg!(&total);
    }

    {
        let v = vec!["1", "12", "123"];

        // 迭代器适配器: map
        let v1: Vec<String> = v
            .iter()
            .map(|s| s.to_string())
            .map(|mut x| {
                x.push('x');
                x
            })
            .collect();
        dbg!(&v);

        // 迭代器适配器: filter
        let v2: Vec<&str> = v
            .iter()
            .filter(|s| s.len() > 1)
            .map(|s| s.clone())
            .collect();
    }

    {
        // 自定义迭代器
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        let sum: u32 = Counter::new()
            .skip(1)
            .zip(Counter::new())
            .map(|pair| pair.0 * pair.1)
            .filter(|x| x % 3 == 0)
            .sum();
        dbg!(&sum);
    }
}
