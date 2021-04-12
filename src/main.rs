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
}
