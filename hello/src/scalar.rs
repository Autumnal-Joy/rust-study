#![allow(unused)]

use std::mem;

pub fn scalar() {
    {
        // 类型推断
        let mut var = 0;
        var = 0i64;
    }

    // 标量类型
    {
        /*
         * 有符号数 i8, i16, i32, i64, i128, isize
         * isize 与系统结构相关, 32位或64位寻址
         */
        let int32: i32 = 1000000;
        let int32 = 1_000_000i32;
        let int32 = 100_0000;
        let arch: isize = 0;

        // 无符号数 u8, u16, u32, u64, u128, usize
        let ch: u8 = b'0';
        let uint32: u32 = 0;

        // 浮点数
        let float: f32 = 0.123456;
        let double = 0.123_456f64;
        let double = 0.1234_56;

        // 字符类型 (4字节)
        let ch: char = '0';

        // 布尔类型
        let boolean: bool = true;
        let boolean: bool = false;

        // 单元类型
        let unit: () = ();
    }

    // 复合类型
    {
        // 数组类型
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let arr: [i32; 5] = [1; 5];
        let len = arr.len();
        dbg!(mem::size_of_val(&arr));

        // 切片类型
        let slice = &arr;
        let slice = &arr[1..3];
        dbg!(arr[1], slice[0]);

        // 元组类型
        let tuple: (i32,) = (1,);
        let tuple = (1, 2, 3);
        dbg!(tuple.1);

        // 解构赋值
        let [inta, intb, intc, intd, inte] = arr;
        let (intx, inty, intz) = tuple;
    }
}
