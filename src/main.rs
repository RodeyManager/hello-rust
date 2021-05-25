// use std::slice;

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    fn prev(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
    step: u32,
}
impl Counter {
    fn new(begin: u32, step: u32) -> Counter {
        Counter { count: begin, step }
    }

    fn count(&self) -> Option<u32> {
        Some(self.count)
    }
}
impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += self.step;
        self.count()
    }

    fn prev(&mut self) -> Option<Self::Item> {
        self.count -= self.step;
        self.count()
    }
}


// 默认泛型类型参数和运算符重载
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// RHS=SELF 默认类型参数
// trait Add<RHS=Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

// 自定义 RHS 类型
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 完全限定语法与消歧义：调用相同名称的方法
trait Audi {
    fn racing(&self);
}
trait Mercedes {
    fn racing(&self);
}

struct SportCar;
impl SportCar {
    fn racing(&self) {
        println!("sport car racing");
    }
}
impl Audi for SportCar {
    fn racing(&self) {
        println!("audi sport car racing");
    }
}
impl Mercedes for SportCar {
    fn racing(&self) {
        println!("mercedes sport car racing")
    }
}

// 一个带有关联函数的trait 和 一个带有同名关联函数并实行了此trait的类型
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// 父 trait 用于在另一个 trait 中使用某 trait 的功能
use std::fmt;

trait OutLinePrint: fmt::Display {
    fn print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct PointFmt {
    x: i32,
    y: i32,
}

impl OutLinePrint for PointFmt {}
impl fmt::Display for PointFmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newtype 模式用以在外部类型上实现外部 trait
// 绕开 孤儿原则（orphan rule）
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // Rust 高级特性

    // 一、不安全的 Rust

    // 可以通过 unsafe 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块。
    // 这里有五类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作，它们称之为 “不安全的超级力量。” 这些超级力量是：

    // - 解引用裸指针
    // - 调用不安全的函数或方法
    // - 访问或修改可变静态变量
    // - 实现不安全 trait
    // - 访问 union 的字段

    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut counter = Counter::new(0, 2);
    assert_eq!(Some(2), counter.next());
    assert_eq!(Some(4), counter.next());
    assert_eq!(Some(2), counter.prev());

    // 二、高级 trait

    let point1 = Point { x: 1, y: 1 };
    let point2 = Point { x: 2, y: 2 };
    assert_eq!(Point { x: 3, y: 3 }, point1 + point2);

    println!("Millimeters(0) + Meters(2) = {:?}", Millimeters(0) + Meters(2));

    // 完全限定语法与消歧义：调用相同名称的方法
    let sport_car = SportCar;
    sport_car.racing();
    SportCar::racing(&sport_car);
    Audi::racing(&sport_car);
    Mercedes::racing(&sport_car);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let point_fmt = PointFmt{ x: 100, y: 101 };
    point_fmt.print();

    let w = Wrapper(vec![
        String::from("Hello"),
        String::from("world")
    ]);
    println!("w = {}", w);

    // 三、高级类型

    // 1、类型别名 (类型别名的主要用途是减少重复)
    type Kilometers = i32;

    let x: i32 = 2;
    let y: Kilometers = 3;
    assert_eq!(5, x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}

    // 2、使用类型别名
    // type Thunk = Box<dyn Fn() + Send + 'static>;
    // let f: Thunk = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Thunk) {}
    // fn returns_long_type() -> Thunk {}

    // 3、从不返回的 never type
    // Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
    // 我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。
    // continue 的值是 !
    // never type 的另一个用途是 panic!

    // 4、动态大小类型和 Sized trait
    // str 是一个 DST；直到运行时我们都不知道字符串有多长
    // let s1: str = "Hello there!"; // error
    // *** 动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。
    // 为了处理 DST，Rust 有一个特定的 trait 来决定一个类型的大小是否在编译时可知：这就是 Sized trait。
    // 这个 trait 自动为编译器在编译时就知道大小的类型实现。另外，Rust 隐式的为每一个泛型函数增加了 Sized bound。
    // 对于如下泛型函数定义：
    // fn generic<T>(t: T) {}
    // 实际上被当作如下处理：
    // fn generic<T: Sized>(t: T) {}
    // 泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制：
    // fn generic<T: ?Sized>(t: &T) {}
    // ?Sized trait bound 与 Sized 相对；也就是说，它可以读作 “T 可能是也可能不是 Sized 的”。这个语法只能用于 Sized ，而不能用于其他 trait。
    // 另外注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后。在这个例子中选择了引用。

    // 四、高级函数与闭包

    // 1、函数指针
    // 可以向函数传递闭包；也可以向函数传递常规函数；
    // 通过函数指针允许我们使用函数作为另一个函数的参数（函数的类型是 fn （使用小写的 “f” ）以免与 Fn 闭包 trait 相混淆）
    // fn 被称为 函数指针（function pointer）
    // 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    // 作为一个既可以使用内联定义的闭包又可以使用命名函数的例子
    let numbers = vec![1,2,3,4,5];
    let numbers_string: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();
    println!("numbers to strings: {:?}", numbers_string);
    // 或者可以将函数作为 map 的参数来代替闭包，像是这样：
    let numbers = vec![1,2,3,4,5];
    let numbers_string: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("numbers to strings: {:?}", numbers_string);

    #[derive(Debug)]
    enum Status {
        Value(u32),
    }
    let list_of_status: Vec<Status> = (0u32..6).map(Status::Value).collect();
    println!("list of status value: {:?}", list_of_status);

    // 2、返回闭包
    // - 闭包表现为 trait，这意味着不能直接返回闭包
    // 以下代码不能编译
    // fn return_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    // 但是我们可以使用 trait 对象
    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    println!("return closure value: {}", return_closure()(1));

    // 五、宏（Macro）

    // 使用 macro_rules! 的 声明（Declarative）宏，和三种 过程（Procedural）宏：

    // - 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
    // - 类属性（Attribute-like）宏定义可用于任意项的自定义属性
    // - 类函数宏看起来像函数不过作用于作为参数传递的 token

    // 1、宏和函数的区别
    // 宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）
    // 一个函数标签必须声明函数参数个数和类型。相比之下，宏能够接受不同数量的参数
    // 宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用

    // 2、使用 macro_rules! 的声明宏用于通用元编程
    // vec! 宏简单定义参考：
    // #[macro_export]
    // macro_rules! vec {
    //     ( $( $x:expr ),* ) => {
    //         {
    //             let mut temp_vec = Vec::new();
    //             $(
    //                 temp_vec.push($x);
    //             )*
    //             temp_vec
    //         }
    //     };
    // }
    // #[macro_export] 注解说明宏应该是可用的。 如果没有该注解，这个宏不能被引入作用域。

    // 3、用于从属性生成代码的过程宏
    // 有三种类型的过程宏（自定义派生（derive），类属性和类函数），不过它们的工作方式都类似
    // use proc_macro;
    // #[some_attribute] // `some_attribute` 是一个使用特定宏的占位符
    // pub fn some_name(input: TokenStream) -> TokenStream {}
    // 定义过程宏的函数接受一个 TokenStream 作为输入并产生一个 TokenStream 作为输出。
    // 这也就是宏的核心：宏所处理的源代码组成了输入 TokenStream，同时宏生成的代码是输出 TokenStream

    // 4、如何编写自定义 derive 宏（#[proc_macro_derive]）
    // #[derive(HelloMacro)] 注解他们的类型来得到 hello_macro 函数的默认实现
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;
    Pancakes::hello_macro();

    #[derive(HelloMacro)]
    struct KindBound;
    KindBound::hello_macro();

    // 5、类属性宏（#[proc_macro_attribute]）
    // 类属性宏与自定义派生宏相似，不同于为 derive 属性生成代码，它们允许你创建新的属性。
    // 它们也更为灵活；derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数。

    // 6、类函数宏（#[proc_macro]）
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
}
