use std::{
    f64::INFINITY,
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn exception() {
    // 不可恢复错误
    // panic!("crash");

    // 可恢复错误
    {
        let f = match File::open("hello") {
            Ok(v) => v,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create("hello") {
                    Ok(v) => v,
                    Err(e) => panic!("{e:?}"),
                },
                other => panic!("{e:?}"),
            },
        };

        let f = File::open("hello").unwrap_or_else(|e| {
            if e.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|e| {
                    panic!("{e:?}");
                })
            } else {
                panic!("{e:?}");
            }
        });
    }

    // 传递错误
    {
        fn read_file1() -> Result<String, io::Error> {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
    }

    /*
     * 传递错误语法糖
     * "?" 解包 Ok 内的值, 或使函数返回 Err
     * "?" 解包 Some 内的值, 或使函数返回 None
     */
    {
        fn read_file2() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
}
