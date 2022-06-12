pub fn r#unsafe() {
    /*
     * 1.解引用裸指针
     * 2.调用不安全的函数或方法
     * 3.访问或修改可变静态变量
     * 4.实现不安全 trait
     * 5.访问 union 的字段
     */

    {
        // 解引用裸指针
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    {
        // 创建不安全代码的安全抽象
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    std::slice::from_raw_parts_mut(ptr, mid),
                    std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        dbg!(split_at_mut(&mut [1, 4, 9, 16, 25], 3));
    }

    {
        // 使用 extern 函数调用外部代码
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
            dbg!(abs(-3));
        }
    }

    {
        // 访问或修改可变静态变量
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
        }
        unsafe {
            dbg!(COUNTER);
        }
    }

    {
        // 实现不安全 trait
        unsafe trait Foo {
            // methods go here
        }
        unsafe impl Foo for i32 {
            // method implementations go here
        }
    }

    {
        // 访问联合体中的字段
        union MyUnion {
            i: u32,
            f: f32,
        }
        let u = MyUnion { i: 1 };
        let i = unsafe { u.i };
    }
}
