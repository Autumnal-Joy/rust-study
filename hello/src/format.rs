pub fn r#format() {
    // 将格式化文本写到字符串
    dbg!(format!("Hello World"));
    // 向 io::stdout 打印格式化文本
    print!("Hello World\n");
    // 向 io::stdout 打印格式化文本, 并换行
    println!("Hello World");
    // 向 io::stderr 打印格式化文本
    eprint!("Hello World\n");
    // 向 io::stderr 打印格式化文本, 并换行
    eprintln!("Hello World");

    // 位置参数与具名参数
    {
        // 类似迭代器, {} 表示取下一个位置参数
        dbg!(format!("{1} {} {0} {}", 1, 2));

        /*
         * 模板字符串, 优先从具名参数中找, 其次从作用域中找
         * 具名参数位于位置参数之后
         */
        let world = "World";
        dbg!(format!("{value} {world}", value = "Hello"));
    }

    // 格式化最小宽度
    {
        /*
         * ":" 用于描述格式化信息
         * 打印第0个位置参数, 输出宽度至少为5
         */
        dbg!(format!("Hello {:5}!", "x"));

        /*
         * "0$" 表示用第0个位置参数替换
         * 打印第1个位置参数, 输出宽度至少为第0个位置参数
         */
        dbg!(format!("Hello {1:0$}!", 5, "x"));

        /*
         * "width$" 表示用具名参数或作用域变量替换
         * 打印第0个位置参数, 输出宽度至少为 width
         */
        dbg!(format!("Hello {:width$}!", "x", width = 5));
        let width = 5;
        dbg!(format!("Hello {:width$}!", "x"));
    }

    // 格式化对齐方式与填充字符
    {
        // 字符串默认居左对齐
        dbg!(format!("Hello {:5}!", "x"));

        // 数值默认居右对齐
        dbg!(format!("Hello {:5}!", 1));

        // "<" 显式表示居左对齐, 以空格填充右侧
        dbg!(format!("Hello {:<5}!", "x"));

        // ">" 显式表示居右对齐, 以空格填充左侧
        dbg!(format!("Hello {:>5}!", "x"));

        // "+^" 显式表示居中对齐, 以 "+" 填充两侧
        dbg!(format!("Hello {:+^5}!", "x"));

        // "-^" 显式表示居中对齐, 以 "-" 填充两侧, 右侧不少于左侧
        dbg!(format!("Hello {:-^6}!", "x"));
    }

    // 格式化使用 trait
    {
        // "+" 表示显示数值符号
        dbg!(format!("Hello {:+}!", 5));

        // "05" 表示宽度至少为5, 以0填充左侧
        dbg!(format!("Hello {:05}!", -5));

        // "#b" 表示以2进制显示数值, 显示前缀 0b
        dbg!(format!("Hello {:#b}!", 27));

        // "b" 表示以2进制显示数值, 不显示前缀
        dbg!(format!("Hello {:b}!", 27));

        // "#o" 表示以8进制显示数值, 显示前缀 0x
        dbg!(format!("Hello {:#o}!", 27));

        // "#010x" 表示以16进制显示数值, 宽度至少为10, 以0填充左侧
        dbg!(format!("Hello {:#010x}!", 27));

        // "?" 表示打印调试数据
        dbg!(format!("Hello {:?}", (100, 200)));

        // "#?" 表示多行打印调试数据
        dbg!(format!("Hello {:#?}", (100, 200)));
    }

    // 格式化控制精度
    {
        // ".5" 表示保留5位小数
        dbg!(format!("Hello {:.5}", 123.456));

        // 使用位置参数或具名参数指定格式
        dbg!(format!("Hello {1:.0$}", 5, 123.456));
        dbg!(format!("Hello {num:.prec$}", prec = 5, num = 123.456));

        // 未指定字符串时, ".*" 连续使用两个参数, 第一个代表宽度, 第二个代表字符串
        dbg!(format!("Hello {:.*}", 5, "123.456"));

        // 未指定数值时, ".*" 连续使用两个参数, 第一个代表精度, 第二个代表数值
        dbg!(format!("Hello {:.*}", 5, 123.456));

        // 指定字符串后, ".*" 使用字符串前一个参数表示宽度
        dbg!(format!("Hello {1:.*}", 5, "123.456"));

        // 指定数值后, ".*" 使用数值前一个参数表示精度
        dbg!(format!("Hello {1:.*}", 5, 123.456));
    }

    // 格式化转义
    {
        // "{" 和 "}" 的转义
        dbg!(format!("{{ Hello"));
        dbg!(format!("World }}"));
    }
}
