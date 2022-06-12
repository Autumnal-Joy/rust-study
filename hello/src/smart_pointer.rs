use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub fn smart_pointer() {
    {
        /*
         * Box 在堆上存储数据
         * 使用情况:
         * 1.数据大小编译时未知
         * 2.避免大量数据被拷贝
         * 3.保存实现相同 trait 的不同类型
         */
        dbg!(Box::new(5));

        {
            /*
             * 递归定义无限长度
             * enum List {
             *     Cons(i32, List),
             *     Nil,
             * }
             */
            #[derive(Debug)]
            enum List {
                Cons(i32, Box<List>),
                Nil,
            }
            use List::{Cons, Nil};
            dbg!(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
        }
    }

    {
        use std::ops::Deref;

        #[derive(Debug)]
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }
        let x = MyBox::new(String::from("Rust"));

        {
            /*
             * Deref 解引用
             * *x = *(x.deref())
             */
            dbg!(&x);
            dbg!(&*x);
        }

        {
            /*
             * 参数不匹配时, 解引用强制转换
             * &x -> x.deref()
             * &MyBox<String> -> &String -> &str
             */
            fn hello(name: &str) -> String {
                format!("Hello, {}!", name)
            }
            dbg!(hello(&x));
        }
    }

    {
        // Drop
        struct SP {
            data: String,
        }
        impl Drop for SP {
            fn drop(&mut self) {
                println!("Drop `{}`!", self.data);
            }
        }
        let c = SP {
            data: String::from("Hello"),
        };
        let d = SP {
            data: String::from("World"),
        };
        drop(c);
    }

    {
        // Rc<T> 引用计数智能指针
        #[derive(Debug)]
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        use List::{Cons, Nil};
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
        dbg!(a);
        dbg!(b);
        dbg!(c);
    }

    {
        // RefCell<T> 和内部可变性模式
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }
        use List::{Cons, Nil};
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        dbg!(a);
        dbg!(b);
        dbg!(c);
    }

    {
        // Weak<T>
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        dbg!(&leaf);
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        dbg!(&branch);
    }
}
