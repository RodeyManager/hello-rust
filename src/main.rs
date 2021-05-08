use std::fmt::Debug;


fn main() {


    // ------------- 泛型、trait 和生命周期 ----------------

    // 概念
    // 泛型（generics）。泛型是具体类型或其他属性的抽象替代。
    // trait，一个定义泛型行为的方法。trait 可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型。
    // 生命周期（lifetimes），它是一类允许我们向编译器提供引用如何相互关联的泛型。Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。


    // - 提取函数来减少重复

    // let list = vec![1,2,54,2,6,67,255];

    // let mut largest = list[0];
    // for number in list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let list = vec![100, 250, 1000, 23, 500];
    let max_number = largest_number(&list);
    println!("Largest Number: {}", max_number);

    let list = vec!['a', 'b', 'A', 'd', 'k', 'M'];
    let max_char = largest_char(&list);
    println!("Largest Char: {}", max_char);



    // 实现了 PartialOrd 和 Copy trait 的泛型的 largest 函数
    let list = vec![100, 250, 1000, 23, 500];
    let max_number = largest(&list);
    println!("[PartialOrd + Copy] Largest Number: {}", max_number);

    let list = vec![100.00, 250.23, 100.02, 23.12, 500.55];
    let max_number = largest(&list);
    println!("[PartialOrd + Copy] Largest Number: {}", max_number);

    let list = vec!['a', 'b', 'A', 'd', 'k', 'M'];
    let max_char = largest(&list);
    println!("[PartialOrd + Copy] Largest Char: {}", max_char);


    let list = vec![100, 250, 1000, 23, 500];
    let max_number = largest_ord(&list);
    println!("[Ord] Largest Number: {:?}", max_number);

    // let list = vec![100.00, 250.23, 100.02, 23.12, 500.55];
    // let max_number = largest_ord(&list);
    // println!("[Ord] Largest Number: {}", max_number);

    let list = vec!['a', 'b', 'A', 'd', 'k', 'M'];
    let max_char = largest_ord(&list);
    println!("[Ord] Largest Char: {}", max_char);




    // 结构体定义中的泛型
    let point1 = Point { x: 2, y: 5 };
    // let point1 = Point { x: 2.08, y: 5 };
    // let point2 = Point { x: 10, y: 20.8 };
    println!("Point 1: {:?}", point1);
    // println!("Point 2: {:?}", point2);

    // 枚举定义中的泛型
    let rust = Language::Rust(String::from("Rust程序设计"));
    let java = Language::Java(String::from("Java Web大型企业应用开发"));
    let cplus = Language::CPlus(String::from("C++程序设计"));
    println!("rust book: {:?}", rust);
    println!("java book: {:?}", java);
    println!("cplus book: {:?}", cplus);

    // 方法定义中的泛型
    let point =  Point { x: 11.05, y: 23.01 };
    println!("point x: {}, y: {}", point.x, point.y);
    println!("point calc area: {:?}", point.distance_from_origin());

    let react1 = Rectangle { width: 25.35, height: 18 };
    let react2 = Rectangle { width: 15, height: 20.65 };
    let react = react1.mixup(react2);
    println!("React: {:?}", react);

    // trait：定义共享的行为

    let article = NewsArticle {
        headline: String::from("奖金50万元,2021年度深圳市“鹏城工匠”开启申报"),
        location: String::from("广东省深圳市"),
        author: String::from("今日头条"),
        content: String::from("深圳市“鹏城工匠”是深圳技能人才的最高荣誉"),
    };

    let tweet = Tweet {
        username: String::from("Rodey Luo"),
        content: String::from("中国人民银行自5月9日起发行2021吉祥文化金银纪念币一套。"),
        reply: true,
        retweet: true,
    };

    println!("今日新闻：{}", article.summarize());
    println!("今日圈事：{}", tweet.summarize_author());


    // trait 作为参数
    notify(&article);
    notify(&tweet);

    // Trait Bound 语法
    notify_bound(&article);
    notify_bound(&tweet);

    // 通过 + 指定多个 trait bound
    notify_bound_more(&tweet);

    // 通过 where 简化 trait bound
    notify_bound_where(&tweet);

    // 返回实现了 trait 的类型
    // impl Trait 允许你简单的指定函数返回一个 Iterator 而无需写出实际的冗长的类型。
    let return_tweet = return_summarizable();
    println!("今日圈闻：{}", return_tweet.summarize_author());


    // 生命周期（lifetime）与引用有效性

    // Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。

    // 函数中的泛型生命周期
    let string1 = String::from("abcdef");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The long string is: {}", result);

    // 结构体定义中的生命周期注解
    let string = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = string.split('.').next().expect("not found a '.'");
    let imet = ImportantExcerpt { part: first_sentence };
    println!("imet: {:?}", imet);
    println!("imet part: {}", imet.announce_and_return_part("Call me Ishmael. Some years ago..."));

    // 生命周期省略（Lifetime Elision）
    // 三条规则来判断引用何时不需要明确的注解(这些规则适用于 fn 定义，以及 impl 块):
    // 1、每一个是引用的参数都有它自己的生命周期参数，例如： fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    // 2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32
    // 3、如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)

    // 静态生命周期
    // 所有的字符串字面值都拥有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";
    println!("s: {}", s);


}

// 意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}


// 方法定义中的生命周期注解

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开。
// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用
// 就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中。
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 > len2 {
        s1
    } else if len1 < len2 {
        s2
    } else {
        "=="
    }
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Jack Li"),
        content: String::from("三大运营商将从美国退市。"),
        reply: true,
        retweet: true,
    }
}

fn notify_bound_where<T>(item: &T)
where T: Summary + Sort {
    println!("Breaking news! {}", item.summarize());
    item.up();
}

fn notify_bound_more(item: &(impl Summary + Sort)) {
    println!("Breaking news! {}", item.summarize());
    item.up();
}

// =

// fn notify_bound_more<T: Summary + Sort>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
//     item.up();
// }

fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// &impl trait 可以指定引用
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

trait Summary {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

trait Sort {
    fn up(&self) {
        println!("Sort Up");
    }
}


#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    // fn summrize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Sort for Tweet {}


// 结构体

// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn mixup<V, W>(self, other: Rectangle<V, W>) -> Rectangle<T, W> {
        Rectangle { width: self.width, height: other.height }
    }
}

// 枚举
#[derive(Debug)]
enum Language<T> {
    Java(T),
    Rust(T),
    CPlus(T),
}



// 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。
// 如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，
// 我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配。尝试自己实现这种替代解决方式吧！

// 以下不支持 float 浮点类型
fn largest_ord<T: Ord>(list: &[T]) -> &T {
    list.iter().max().unwrap()
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest_number(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}