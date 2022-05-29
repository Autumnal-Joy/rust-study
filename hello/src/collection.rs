#![allow(unused)]

pub fn collection() {
    // vector
    {
        // 新建空 vector
        let mut vec: Vec<i32> = Vec::new();

        // 从数组新建 vector
        let mut vec = vec![1, 2, 3];

        // 更新 vector
        vec.push(4);
        vec.pop();

        // 访问 vector
        let v = &vec[2];
        let v = vec.get(2).expect("out of range");

        // 遍历 vector
        for v in &mut vec {
            *v += 1;
        }
        dbg!(&vec);
    }

    // String
    {
        // 新建空 String
        let mut s = String::new();

        // 从字面量新建 String
        let mut s = "Hello".to_string();
        let mut s = String::from("Hello");

        // 追加 String
        s.push(' ');
        s.push_str("World");
        s = s + "!";
        dbg!(&s);

        // 访问 String
        let sub_str = &s[0..5];
        dbg!(&sub_str);

        // 逐字符遍历 String
        for c in s.chars() {
            dbg!(&c);
        }

        // 逐字节遍历 String
        for c in s.bytes() {
            dbg!(&c);
        }
    }

    // hash map
    {
        use std::collections::HashMap;

        // 新建空 HashMap
        let mut m: HashMap<String, i32> = HashMap::new();

        // 从数组创建 HashMap
        let arr1 = vec![String::from("Blue"), String::from("Yellow")];
        let arr2 = vec![10, 50];
        let mut m: HashMap<_, _> = arr1.iter().zip(arr2.iter()).collect();

        // 更新 HashMap
        let mut m = HashMap::new();
        m.insert(String::from("Blue"), 10);
        m.remove(&String::from("Blue"));
        let value = m.entry(String::from("Blue")).or_insert(10);
        m.insert(String::from("Yellow"), 25);
        m.insert(String::from("Yellow"), 50);

        // 访问 HashMap
        let v = m.get(&String::from("Yellow")).expect("Not Found");

        // 遍历 HashMap
        for (key, value) in &m {
            dbg!(key, value);
        }
    }
}
