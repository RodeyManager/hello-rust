fn main() {
    // 可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 常量定义
    const MAX_POINTS: u32 = 100_000;
    println!("max_points value is: {}", MAX_POINTS);

    // 可变变量
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length: {}", spaces);

    // 不可改变变量类型（可变变量、不可变变量）
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // Rust中的基本 标量类型（scalar，代表一个单独的值）：
    // 1、整型: i8/i16/i32(默认)/i64/i128/isize(有符号)、u8/u16/u32/u64/u128/usize(无符号)
    // 2、浮点型：f32(单精度)、f64（双精度，默认）
    // 3、布尔类型：true、false
    // 4、字符串类型

    // 加法
    let sum = 1 + 5;
    // 减法
    // let difference = 100.2 - 90.5; // difference: 9.700000000000003
    let difference = 95.2 - 4.3;
    // 乘法
    let product = 4 * 5;
    // 除法
    let quotient = 45.6 / 32.2;
    // 取余
    let remainder = 43 % 5;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}",
        sum, difference, product, quotient, remainder
    );

    let is_used = true;
    let is_complete: bool = false; // 显式指定类型注解
    println!("isUsed: {}, isComplete: {}", is_used, is_complete);

    let c = 'c';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heartEyedCat: {}", c, z, heart_eyed_cat);

    // 复合类型（Compound types）可以将多个值组合成一个类型。
    // Rust 有两个原生的复合类型：元组（tuple）和数组（array）

    // ---元组（多个不同类型的值，长度固定）
    let tup: (u8, i32, f64) = (1, -10, 5.6);
    println!("tup: {:?}", tup);
    // 元组模式匹配解构取值
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    // 元组索引取值
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // ---数组（多个相同类型的值，长度固定）
    // Rust 中的数组与一些其他语言中的数组不同，因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let arr = [1, 2, 3, 4, 5];
    println!("arr of value: {:?}", arr);
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [2; 10];
    println!("arr2 of value: {:?}", arr2);
    println!("arr3 of value: {:?}", arr3);

    // 函数
    let z = snak_cass(56, 123);
    println!("z: {}", z);
    let p = result_return(56, 12);
    println!("p: {:?}", p.unwrap());

    // *** Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。
    let java = 10;
    let rust = {
        let java = 5;
        java + 6 // 表达式结尾行没有分号，加了分号就是语句，不会返回值
    };
    println!("rust: {:?}, java: {}", rust, java);

    // ---控制流 branchs

    // if 表达式（if后面跟随的条件必须是bool值）
    let number = 16;
    if number < 5 {
        println!("The number is less");
    } else if number > 5 && number < 10 {
        println!("The number is middle");
    } else {
        println!("The number is granter");
    }

    // 在let语句右侧使用if表达式（必须返回相同类型）
    let number2 = if number > 10 { 100 } else { number };
    // 下面会报错，不能确定number2的类型
    // let number2 = if number > 10 { 100 } else { "hello" };
    println!("Number2: {}", number2);

    // ---循环语句
    let mut count = 0;

    loop {
        count += 1;
        println!("Current count: {}", count);
        if count >= 3 {
            break;
        };
    }

    while count != 0 {
        count -= 1;
        println!("Current count: {}", count);
    }

    let array = [1, 2, 3, 4, 5];
    let mut index = 0;

    // while 循环集合类型容易出错
    while index < 5 {
        println!("the value  is: {}", array[index]);
        index += 1;
    }

    // for 循环集合类型为最佳
    for element in array.iter() {
        println!("the element of value  is: {}", element);
    }

    // 使用Rang配合 for循环
    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!!!");
}

fn snak_cass(x: i32, y: i32) -> i32 {
    println!("function snak_cass, x: {} , y: {}!", x, y);
    x * y
}

fn result_return(x: u8, y: u8) -> std::io::Result<u8> {
    Ok(x + y)
}
