#![allow(unused)]

pub fn structure() {
    // 类单元结构体
    #[derive(Debug)]
    struct Unit;
    dbg!(format!("{:?}", Unit {}));
    dbg!(format!("{:?}", Unit));

    // 元组结构体
    #[derive(Debug)]
    struct Point(f32, f32);

    // 类 C 结构体
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // 嵌套结构体
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    /*
     * 展开运算符 ".."
     * 与 JavaScript 不同, 不替换已存在的属性
     */
    {
        let p = Person {
            name: "person1".to_string(),
            ..Person {
                name: "person2".to_string(),
                age: 10,
            }
        };
        dbg!(&p);
    }

    // 解构赋值
    {
        let rec = Rectangle {
            top_left: Point(1.0, 2.0),
            bottom_right: Point(3.0, 0.0),
        };
        let Rectangle {
            top_left: Point(x1, y1),
            bottom_right: Point(x2, y2),
        } = rec;
    }

    // 结构体的关联函数
    {
        impl Rectangle {
            fn area(&self) -> f32 {
                let Point(x1, y1) = self.top_left;
                let Point(x2, y2) = self.bottom_right;
                (x2 - x1) * (y1 - y2)
            }
        }
        let rec = Rectangle {
            top_left: Point(1.0, 2.0),
            bottom_right: Point(3.0, 0.0),
        };
        println!("{:?}", rec.area());
    }
}
