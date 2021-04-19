// 定义结构体
#[derive(Debug)]
struct User {
    name: String,
    age: u16,
    active: bool,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 结构体可以有多个 impl 块
impl Rect {
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // ----- struct 结构体 --------

    // 结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。

    // 创建 User 结构体实例
    let user = User {
        name: String::from("Jack"),
        age: 23,
        active: true,
    };
    println!("user: {:?}", user);
    println!("user name: {}", user.name);

    // 创建可变实例
    let mut user1 = User {
        name: String::from("Tom"),
        age: 25,
        active: false,
    };
    user1.age = 28;
    println!("user age: {}", user1.age);

    // 函数体中构建新的结构体实例
    let user2 = build_user(String::from("Ticker"), 32);
    println!("name: {}, age: {}", user2.name, user2.age);

    // 使用结构体更新语法从其他实例创建实例
    let user3 = User {
        name: String::from("Amils"),
        // ** 下面为结构体，不能在行尾添加逗号
        ..user2
    };
    println!("user3: {:#?}", user3);

    // 使用没有命名字段的元组结构体来创建不同的类型
    #[warn(dead_code)]
    struct Point(i32, i32, i32);
    #[warn(dead_code)]
    struct Color(u16, u16, u16);

    let color = Color(125, 100, 255);
    let point = Point(-100, 23, -8);
    println!("r: {}, g: {}, b: {}", color.0, color.1, color.2);
    println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);

    // 没有任何字段的类单元结构体
    // 我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，
    // 即 unit 类型。类单元结构体常常在你想要在某个类型上实现 trait
    // 但不需要在类型中存储数据的时候发挥作用。我们将在第十章介绍 trait。

    // 实例
    let width = 100;
    let height = 120;
    let rect = Rect { width, height };
    println!("(none) rect area: {}", area(width, height));
    println!("(tuple) rect area: {}", area_tuple((width, height)));
    println!(
        "(struct) rect area: {}",
        area_struct(Rect { width, height })
    );
    println!("(struct method) rect area: {}", rect.area());

    let rect2 = Rect { width, height: 150 };
    println!("rect can hold rect2 ?: {}", rect.can_hold(&rect2));
    println!("rect2 can hold rect ?: {}", rect2.can_hold(&rect));

    let square_rect = Rect::square(100);
    println!(
        "square rect width: {}, height: {}",
        square_rect.width, square_rect.height
    );

    // ---- 总结 ----
    // 结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，
    // 我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。
    // 方法允许为结构体实例指定行为，而关联函数将特定功能置于结构体的命名空间中并且无需一个实例。
}

fn area_struct(rect: Rect) -> u32 {
    rect.width * rect.height
}

fn area_tuple(react: (u32, u32)) -> u32 {
    react.0 * react.1
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn build_user(name: String, age: u16) -> User {
    User {
        name,
        age,
        active: true,
    }
}
