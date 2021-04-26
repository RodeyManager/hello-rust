use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // ---- 常见集合 ----

    // vector 允许我们一个挨着一个地储存一系列数量可变的值
    // 字符串（string）是字符的集合
    // 哈希 map（hash map）允许我们将值与一个特定的键（key）相关联。这是一个叫做 map 的更通用的数据结构的特定实现。

    // vector 用来储存一系列的值
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    println!("v: {:?}", v);

    {
        let v2 = vec![1, 2, 3];
        println!("v2: {:?}", v2);
    } // v2已离开作用域

    // 读取 vector 的元素
    let third: &i32 = &v[2];
    println!("get third: {}", third);

    match v.get(2) {
        Some(third) => println!("get third: {}", third),
        None => println!("There is no third element."),
    }

    // 处理超出vec范围的情况，此为更友好的处理方式
    match v.get(10) {
        Some(n) => println!("get v num: {}", n),
        None => println!("beyond v max length."),
    }

    // 以下编译报错：不能在相同作用域中同时存在可变和不可变引用的规则
    // let mut vec = vec![1, 2, 3];
    // let two = &vec[1]; // 引用不可变值
    // vec.push(4); // 引用可变值
    // println!("vec: {:?}", vec);
    // 以上原因：
    // 为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 vector 的工作方式：
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

    // 遍历 vector 中的元素
    let arr = vec![100, 101, 102];
    for element in &arr {
        println!("arr element: {}", element);
    }

    // 遍历可变 vector 的每一个元素的可变引用以便能改变其值
    let mut arr = vec![200, 201, 202];
    for element in &mut arr {
        // 在使用 += 运算符之前必须使用解引用运算符（*）
        *element += 10;
        println!("arr element: {}", element);
    }

    // 使用枚举来储存多种类型
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Cell::Int(5),
        Cell::Float(10.56),
        Cell::Text(String::from("Book Type")),
    ];
    println!("row: {:#?}", row);

    // 返回Option
    let p = arr.pop();
    println!("arr pop value: {:?}", p);

    // 使用字符串存储 UTF-8 编码的文本
    // Rust 倾向于确保暴露出可能的错误，字符串是比很多程序员所想象的要更为
    // 复杂的数据结构，以及 UTF-8

    // Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str。
    // String 和字符串 slice 都是 UTF-8 编码的

    let mut s = String::new();
    // 使用 push_str 和 push 附加字符串
    s.push_str("Hello");
    println!("s: {}", s);

    let s = "Hello World";
    println!("s: {}", s.to_string());
    let s = "Hello World".to_string();
    println!("s: {}", s.to_string());

    let s = String::from("Create String");
    println!("s: {}", s);

    // 字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    println!("hello: {}", hello);

    // 更新字符串
    let mut name = "Rodey".to_string();
    // 使用 push_str 方法向 String 附加字符串 slice
    name.push_str(" Luo");
    println!("my name is: {}", name);

    let last_name = " Luo";
    name.push_str(last_name);
    // 报错：push_str 方法获取了 s2 的所有权，就不能在最后一行打印出其值了
    // println!("my name is: {}", last_name);

    let s1 = "Hello ".to_string();
    let s2 = "World!".to_string();
    let s3 = s1 + &s2;
    // 注意： s1的所有权被释放了，后面不能再使用
    println!("s3 value: {}", s3);

    //     首先，s2 使用了 &，意味着我们使用第二个字符串的 引用 与第一个字符串相加。这是因为 add 函数的 s 参数：
    //     只能将 &str 和 String 相加，不能将两个 String 值相加。不过等一下 —— 正如 add 的第二个参数所指定的，&s2 的类型是 &String 而不是 &str。那么为什么示例 8-18 还能编译呢？

    // 之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。当add函数被调用时，
    // Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。第十五章会更深入的讨论解引用强制多态。因为 add 没有获取参数的所有权，所以 s2 在这个操作后仍然是有效的 String。

    // 其次，可以发现签名中 add 获取了 self 的所有权，因为 self 没有 使用 &。这意味着示例 8-18 中的 s1 的所有权将被移动到 add 调用中，之后就不再有效。
    // 所以虽然 let s3 = s1 + &s2; 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权。
    // 换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝要更高效。

    // 使用 format! 宏 (并且不会获取任何参数的所有权。)
    let year = String::from("2021");
    let month = String::from("04");
    let date = String::from("25");
    let current_date = format!("{}-{}-{}", year, month, date);
    println!("current date: {}", current_date);

    // 索引字符串
    // String 是一个 Vec<u8> 的封装。
    // let len = String::from("Jola").len();
    let len = String::from("Здравствуйте").len();
    println!("len of value: {}", len);
    // 当问及这个字符是多长的时候有人可能会说是 12。然而，Rust 的回答是 24。这是使用 UTF-8 编码 “Здравствуйте” 所需要的字节数，
    // 这是因为每个 Unicode 标量值需要两个字节存储。因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值。

    // 字符串 slice
    // 你应该小心谨慎的使用这个操作，因为这么做可能会使你的程序崩溃。
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1];  // panic
    println!("s value: {}", s);

    // 遍历字符串的方法
    for c in "你好，Rust".chars() {
        println!("{}", c);
    }

    // 返回原始字节
    for b in "你好，Rust".bytes() {
        println!("{}", b);
    }

    // 字符串并不简单

    // 哈希 map 储存键值对（要使用 HashMap必须先 use std::collections::HashMao）
    let mut map = HashMap::new();
    map.insert(String::from("Red"), 89);
    map.insert(String::from("Yellow"), 80);
    map.insert(String::from("blue"), 90);
    println!("map: {:#?}", map);

    // 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
    // HashMap<T, U>
    let teams = vec![
        String::from("Red"),
        String::from("Yellow"),
        String::from("Blue"),
    ];
    let coins = vec![90, 89, 100];
    let map: HashMap<_, _> = teams.iter().zip(coins.iter()).collect();
    // TODO: 转换后为什么顺序改变了???? { "Yellow": 89, "Blue": 100, "Red": 90 }
    println!("map: {:#?}", map);

    // 哈希 map 和所有权
    let first_name = String::from("Rodey");
    let last_name = String::from("Luo");
    let mut name = HashMap::new();
    name.insert(first_name, last_name);
    println!("name: {:?}", name);
    // 由于String拥有所有权，而map.insert后将所有权移动到insert中，所以first_name,last_name此后不再有效
    // println!("my name: {} {}", first_name, last_name);

    // 访问哈希 map 中的值
    let mut map = HashMap::new();
    map.insert(String::from("Red"), 255.0);
    map.insert(String::from("Green"), 55.0);
    map.insert(String::from("Blue"), 155.0);
    map.insert(String::from("Alpha"), 0.6);
    let red = map.get("Red"); // => Option
    println!("Red coins: {:?}", red);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 更新
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 78);
    scores.insert(String::from("Blue"), 98);
    println!("scores: {:?}", scores);

    // 只在键没有对应值时插入
    scores.entry(String::from("Red")).or_insert(98);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("scores: {:?}", scores);

    let s = "red greeb blue alpha red";
    let mut map = HashMap::new();

    for word in s.split_whitespace() {
        // ****以下为重点：
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）。这里我们将这个可变引用储存在 count 变量中，
        // 所以为了赋值必须首先使用星号（*）解引用 count。
        // 这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // map: {"greeb": 1, "alpha": 1, "red": 2, "blue": 1}
    println!("map: {:?}", map);

    // 练习----------
    // 1、给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。
    let numbers = vec![2, 1, 3, 5, 67, 32, 23, 5, 32];
    let mut sum = 0;
    let mut string = "".to_string();
    let len = numbers.len();
    for num in numbers.iter() {
        sum += num;
        string.push_str(&num.to_string());
    }
    let average = sum / len;
    println!("sum: {}", sum);
    println!("average: {}", average);
    println!("string: {}", string);

    let strings: Vec<&str> = string.split("").collect();
    let mut map = HashMap::new();
    for num in strings.iter() {
        // println!("num: {}", num);
        if !num.is_empty() {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
    }
    println!("map: {:?}", map);

    // 2、将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。
    // 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
    let consonant: Vec<&str> = "b,c,d,f,g,h,j,k,l,m,n,p,q,r,s,t,v,w,x,y,z"
        .split(",")
        .collect();
    let metes: Vec<&str> = "a,e,i,o,u".split(",").collect();
    let words = "first last middle big apple out is good".split_whitespace();
    let mut map = HashMap::new();

    for word in words {
        let word = Some(word).unwrap();

        // 辅音字母
        for letter in consonant.iter() {
            if word.starts_with(letter) {
                let text = map.entry(word).or_insert(String::from(""));
                *text = format!("{}-{}ay", word.strip_prefix(letter).unwrap(), letter);
            }
        }

        // 元音字母
        for letter in metes.iter() {
            if word.starts_with(letter) {
                let text = map.entry(word).or_insert(String::from(""));
                *text = format!("{}-hay", word);
            }
        }
    }

    println!("map: {:?}", map);

    // 3、使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
    // 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
    // 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

    Ok(())
}
