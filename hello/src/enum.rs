#![allow(unused)]

pub fn r#enum() {
    // 枚举类型
    {
        enum WebEvent {
            PageLoad,
            PageUnload,
            KeyPress(char),
            Paste(String),
            Click { x: i64, y: i64 },
        }
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                // 解构
                WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}.", x, y);
                }
            }
        }
        inspect(WebEvent::PageLoad {});
        inspect(WebEvent::PageUnload);
        inspect(WebEvent::KeyPress('c'));
        inspect(WebEvent::Paste("abc".to_string()));
        inspect(WebEvent::Click { x: 1, y: 2 });
    }

    // 枚举类型别名
    {
        enum VeryVerboseEnumOfThingsToDoWithNumbers {
            Add,
            Subtract,
        }
        type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
        impl VeryVerboseEnumOfThingsToDoWithNumbers {
            fn run(&self, x: i32, y: i32) -> i32 {
                match self {
                    Self::Add => x + y,
                    Self::Subtract => x - y,
                }
            }
        }
        dbg!(Operations::Add.run(1, 3));
    }

    // 显式指定枚举类型的值
    {
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }
        dbg!(format!("#{:06x}", Color::Red as i32));
    }

    // 使用 use
    {
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }
        use Color::*;
        use Color::{Blue, Green, Red};
        dbg!(Blue as i32);
    }

    // enum 实现链表
    {
        use List::*;
        #[derive(Debug)]
        enum List {
            Cons(u32, Box<List>),
            Nil,
        }
        impl List {
            fn new() -> List {
                Nil
            }
            fn push_front(self, elem: u32) -> List {
                Cons(elem, Box::new(self))
            }
            fn size(&self) -> u32 {
                match self {
                    Cons(_, tail) => 1 + tail.size(),
                    Nil => 0,
                }
            }
            fn to_string(&self) -> String {
                match self {
                    Cons(head, tail) => format!("{}, {}", head, tail.to_string()),
                    Nil => format!("Nil"),
                }
            }
        }

        let list = List::new().push_front(1).push_front(2).push_front(3);
        dbg!(&list);
    }

    // Option 枚举
    {
        let x: Option<i32> = None;
        let y = Some(1);
        dbg!(&x.is_none());
        dbg!(&x.is_some());
        dbg!(&y.expect("is None"));
        dbg!(&y.unwrap_or(0));
    }
}
