pub fn mutable() {
    // let 声明的变量默认不可变
    {
        let x = 5;
        /*
         * let x = 5;
         *     -
         *     |
         *     first assignment to `x`
         *     help: consider making this binding mutable: `mut x`
         * x = 6;
         * ^^^^^ cannot assign twice to immutable variable
         */
        // x = 6;
    }

    // 使用 mut 让变量可变
    {
        let mut x = 5;
        x = 6;
    }

    // 使用 const 声明常量
    {
        /*
         * 编译时确认取值
         * 值永远不可变
         * 需要注明类型
         * 可以全局声明
         */
        const SECONDS_OF_DAY: i32 = 24 * 60 * 60;
    }
}
