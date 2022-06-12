pub fn r#trait() {
    {
        /*
         * 关联类型
         * type TypeName = Type;
         * 与泛型的区别: 关联类型在一种数据类型上只能实现一次
         *
         */
        #[derive(Debug)]
        struct Counter {
            count: u32,
            max: u32,
        };
        impl Counter {
            fn new(max: u32) -> Counter {
                Counter { count: 0, max }
            }
        }
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < self.max {
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        dbg!(Counter::new(5).sum::<u32>());
    }

    {
        // 默认泛型类型和运算符重载
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl std::ops::Add for Point {
            type Output = Self;
            fn add(self, other: Point) -> Self {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        dbg!((Point { x: 1, y: 0 } + Point { x: 2, y: 3 }));

        impl std::ops::Add<i32> for Point {
            type Output = Self;
            fn add(self, other: i32) -> Self {
                Point {
                    x: self.x + other,
                    y: self.y + other,
                }
            }
        }
        dbg!((Point { x: 1, y: 0 } + 2));
    }

    {
        // 完全限定语法与消歧义
        struct Human;
        impl Human {
            fn fly(&self) {
                println!("waving arms");
            }
            fn name() {
                println!("Human");
            }
        }
        trait Pilot {
            fn fly(&self);
            fn name();
        }
        impl Pilot for Human {
            fn fly(&self) {
                println!("flying a plane");
            }
            fn name() {
                println!("Pilot");
            }
        }
        trait Wizard {
            fn fly(&self);
            fn name();
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("floating in the sky");
            }
            fn name() {
                println!("Wizard");
            }
        }
        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);
        Human::name();
        <Human as Pilot>::name();
        <Human as Wizard>::name();
    }

    {
        // trait 继承
        trait OutlinePrint: std::fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "─".repeat(len + 4));
                println!("│{}│", " ".repeat(len + 2));
                println!("│ {} │", output);
                println!("│{}│", " ".repeat(len + 2));
                println!("{}", "─".repeat(len + 4));
            }
        }
        let x = 10;
        impl OutlinePrint for i32 {}
        x.outline_print();
    }
}
