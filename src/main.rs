fn main() {
    // -------- 所有权（ownership）重点 ----------
    /* 所有权（系统）是 Rust 最为与众不同的特性，它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全。因此，理解 Rust 中所有权如何工作是十分重要的。 */

    // 栈（Stack）和堆（Heap）
    // 栈：后进先出（last in, first out），固定大小; 进栈：增加数据；出栈：移出数据
    // 堆：先进先出（first in, first out），不固定大小;

    // 规则：
    // 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // 2. 值在任一时刻有且只有一个所有者。
    // 3. 当所有者（变量）离开作用域，这个值将被丢弃

    let mut name = String::from("Jack");
    name.insert_str(0, "Hello, ");
    name.push_str(", what are you doing?");
    println!("name: {}", name);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // 变量与数据交互的方式（一）：移动
    let s1 = String::from("Hello");
    let s2 = s1;
    // 运行这段代码会报错，因为s1已经不再有效了
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    // 变量与数据交互的方式（二）：克隆
    let s1 = String::from("World");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    // 只在栈上的数据：拷贝
    // Copy类型实例：
    // - 所有整数类型，比如 u32。
    // - 布尔类型，bool，它的值是 true 和 false。
    // - 所有浮点数类型，比如 f64。
    // - 字符类型，char。
    // - 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

    // 所有权与函数
    let name = String::from("Java");

    taskes_move(name);
    // name已被移动到taskes_move函数中，此后不再有效，
    // 下面代码会报错
    // println!("after name: {}", name);

    let number = 5;
    takes_copy(number);
    // 由于number是copy类型，所以 takes_copy 后可继续使用
    println!("after number: {}", number);

    // 返回值与作用域 （返回值也可以转移所有权）
    let t1 = gives_owerships();
    let t2 = String::from(" Language");
    let t3 = takes_and_gives_back(t2);
    println!("t1: {}", t1);
    // t2已移动，之后不可用
    // println!("t2: {}", t2);
    println!("t3: {}", t3);

    // 使用元组来返回多个值
    let d = String::from("Hello");
    let (s, l) = calculate_length(d);
    println!("s: {}, len: {}", s, l);

    // 引用与借用
    let kiss = String::from("Kiss");
    let len = calculate_length_refence(&kiss);
    println!("kiss: {}, kiss length: {}", kiss, len);

    // 默认引用不可变
    let javascript = String::from("JavaScript");
    // 不可变引用
    immutable_language(&javascript);
    // 可变引用
    let mut javascript = String::from("JavaScript");
    changeble_language(&mut javascript);
    println!("javascript: {}", javascript);
    // ***不过可变引用有一个很大的限制：在特定作用域中的特定数据只能有一个可变引用。
    let mut r = String::from("redirect");
    // 下面代码将报错： ^^^^^^ second mutable borrow occurs here
    // let r1 = &mut r;
    // let r2 = &mut r;
    // println!("r1: {}, r2: {}", r1, r2);
    // 以下代码是可行的
    {
        let r1 = &mut r;
        println!("r1: {}", r1);
    }
    let r2 = &mut r;
    println!("r2: {}", r2);

    let r1 = &r; // 没问题
    let r2 = &r; // 没问题
                 // let r3 = &mut r; // 大问题
                 // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    println!("r1: {}, r2: {}", r1, r2);

    // 悬垂引用（Dangling References）
    // 下满代码执行会报错：^ expected named lifetime parameter
    // let reference_to_nothing = dangle();

    // 总结
    // 引用的规则：
    //  - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //  - 引用必须总是有效的。

    // Slice 类型 （另一个没有所有权的数据类型）
    let str = String::from("hello rust");
    let word_len = find_word_len(&str);
    println!("word_len: {}", word_len);

    let hello = &str[0..5];
    let world = &str[6..10];
    let worlds = &str[..10];
    let worlds2 = &str[..str.len()];
    println!("slice: {} {}", world, hello);
    println!("words: {}", worlds);
    println!("worlds2: {}", worlds2);

    let find_word = find_first_word(&str);
    println!("find_word: {}", find_word);

    // 字符串字面值就是 slice
    // &str 是一个不可变引用
    let string = "Hello World";
    println!("string: {}", string);

    // 字符串 slice 作为参数 (&str[..])
    let find_word_silce = find_first_word_slice(&str[..]);
    println!("find_word_silce: {}", find_word_silce);

    // 其他类型的 slice
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let array_slice = &array[3..(array.len() - 1)];
    println!("array_slice: {:?}", array_slice);
}

fn find_first_word_slice(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}

fn find_first_word(str: &String) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..str.len()]
}

fn find_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("s");
//     &s
// }

fn changeble_language(str: &mut String) {
    str.push_str(" language");
}

fn immutable_language(str: &String) {
    // 报错，默认引用不可变
    println!("str: {}", str);
    // str.push_str(" language");
}

fn calculate_length_refence(str: &String) -> usize {
    str.len()
}

fn calculate_length(str: String) -> (String, usize) {
    let len = str.len();
    (str, len)
}

fn taskes_move(name: String) {
    println!("name: {}", name);
}

fn takes_copy(number: i32) {
    println!("number: {}", number);
}

fn gives_owerships() -> String {
    let str = String::from("Rust");
    str
}

fn takes_and_gives_back(str: String) -> String {
    println!("t2: {}", str);
    str
}
