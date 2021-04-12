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
    // 3、布尔类型
    // 4、字符串类型

    // 加法
    let sum = 1 + 5;
    // 减法
    let difference = 100.2 - 90.5;
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
}
