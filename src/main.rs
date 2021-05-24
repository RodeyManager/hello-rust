fn main() {
    // Rust 模式匹配

    // 模式是 Rust 中特殊的语法，它用来匹配类型中的结构，无论类型是简单还是复杂。结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。模式由如下一些内容组合而成：

    // - 字面值
    // - 解构的数组、枚举、结构体或者元组
    // - 变量
    // - 通配符
    // - 占位符

    // 1、match 分支 （match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到）
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    // 2、if let 条件表达式（多用于 Option、Result）
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // 3、while let 条件循环
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 4、for 循环
    let chars = vec!['a', 'b', 'c'];
    for (index, char) in chars.iter().enumerate() {
        println!("inex: {}, char: {}", index, char);
    }

    // 5、let 语句
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (10, 23, 1);
    let [first, .., last] = [1, 2, 3, 4, 5];
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("first: {}, last: {}", first, last);

    // 6、函数参数
    print_coords(&(1, 10));

    //  模式：Refutability（可反驳性）和 irrefutable（不可反驳）
    // 可反驳性：对某些可能的值进行匹配会失败的模式  let x = 5;
    // 不可反驳：能匹配任何传递的可能值的模式  if let Some(x) = a_value;

    // let 语句和 for 循环只能接受 不可反驳的模式
    // if let 和 while let 表达式被限制为只能接受可反驳的模式
    // match 匹配分支必须使用可反驳模式

    // 匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    // 匹配命名变量
    let x = Some(5);
    // y 位置1
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 此处y非位置1处的y
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Other case, x = {:?}", x),
    }

    // 多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // 通过 ..= 匹配值的范围
    let x = 5;
    match x {
        1..=5 => println!("1~5"),
        _ => println!("other"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("other"),
    }

    // 解构并分解值
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 10, y: 20 };
    let Point { x: a, y: b } = &point;
    let Point { x, y } = &point;
    println!("a: {}, b: {}", a, b);
    println!("x: {}, y: {}", x, y);

    // 解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }
    let m = Message::ChangeColor(0, 255, 0);
    match m {
        Message::Quit => println!("message quit"),
        Message::Move { x, y } => println!("messae move"),
        Message::Write(text) => println!("message write"),
        Message::ChangeColor(r, g, b) => {
            println!("message change color: r: {}, r: {}, b: {}", r, g, b)
        }
    }

    // 解构嵌套的结构体和枚举
    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u32, u32, u32),
    }
    enum Console {
        TextColor(Color),
    }

    let m = Console::TextColor(Color::Rgb(255, 12, 0));
    match m {
        Console::TextColor(Color::Rgb(r, g, b)) => {
            println!("Console text color is r: {}, g: {}, b: {}", r, g, b)
        }
        Console::TextColor(Color::Hsv(h, s, v)) => {
            println!("Console text color is h: {}, s: {}, v: {}", h, s, v)
        }
    }

    // 解构结构体和元组
    let ((width, height), Point { x, y }) = ((100, 120), Point { x: 3, y: 10 });
    println!("width: {}, height: {}", width, height);
    println!("Point x: {}, y: {}", x, y);

    // 忽略模式中的值  _
    // 使用 _ 忽略整个值
    let point = Point { x: 3, y: 5 };
    ignore_args(point.x, 45, 56);

    // 使用嵌套的 _ 忽略部分值
    let mut x = Some(5);
    let new_x = Some(10);
    match (x, new_x) {
        (Some(_), Some(_)) => println!("已匹配"),
        _ => x = new_x,
    }

    let numbers = (2, 4, 6, 8, 10);
    match numbers {
        (first, _, third, _, five) => println!("matched numbers: {}, {}, {}", first, third, five),
    }

    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let _ignore_var = 5;
    // 比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定
    let s = Some(String::from("Hello"));
    if let Some(_s) = s {
        println!("found a string");
    }

    // 用 .. 忽略剩余值
    let point = Point { x: 100, y: 200 };
    match point {
        Point { x, .. } => println!("point x: {}", x),
        _ => println!("not match point"),
    }

    let numbers = (1, 2, 3, 4, 5, 6);
    match numbers {
        (first, .., last) => println!("matched numbers: {}, {}", first, last),
    }

    // 匹配守卫提供的额外条件
    // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
    // 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
    let x = Some(4);
    match x {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    let x = 5;
    let saved = true;
    match x {
        4 | 5 | 6 if saved => println!("Save Ok"),
        _ => println!("Save No"),
    }

    // @ 绑定
    enum Log {
        Level { id: u8 },
    }
    let log = Log::Level { id: 5 };
    match log {
        Log::Level {
            id: temp_id @ 3..=7,
        } => println!("临时变量temp_id: {}", temp_id),
        Log::Level { id: 10..=12 } => println!("log level range"),
        Log::Level { id } => println!("log level id: {}", id),
    }
}

fn print_coords(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn ignore_args(_: i32, width: i32, height: i32) {
    println!("ignore first arg, width: {}, height: {}", width, height);
}
