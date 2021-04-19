fn main() {
    // ----- 枚举和模式匹配 -----

    let localhost = IPAdrr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let remote = IPAdrr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };

    println!("localhost address: {}", localhost.address);
    println!("remote address: {}", remote.address);

    // 直接将类型赋值到枚举的成员上
    let localhost = IPAddrType::V4(String::from("127.0.0.1"));
    let remote = IPAddrType::V6(String::from("::1"));

    println!("localhost address: {:?}", localhost);
    println!("remote address: {:?}", remote);

    let localhost = IPAddress::V4(127, 0, 0, 1);
    let remote = IPAddress::V6(String::from("::2"));

    println!("localhost address: {:?}", localhost);
    println!("remote address: {:?}", remote);

    let message_writer = Message::Write(String::from("message write..."));
    message_writer.call();

    // Option
    let some_number = Some(1);
    let some_string = Some(String::from("some string"));
    let absent_number: Option<i32> = None;
    println!(
        "some number: {:?}, some string: {:?}",
        some_number, some_string
    );
    println!("absent number: {:?}", absent_number);
    let len1 = some_string.clone().map(|s| s.len());
    let len2 = some_string.clone().unwrap().len();
    println!("some string length: {:?}", len1);
    println!("some string length: {}", len2);

    value_of_cents(Coin::Penny);
    value_of_cents(Coin::Quarter(UsState::Alaska));

    let six = plus_one(Some(5));
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn value_of_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => 100,
    }
}

#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IPAddrType {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IPAdrr {
    kind: IPAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u16, u16, u16),
}

impl Message {
    // 同时也可以为枚举定义方法
    fn call(&self) {
        println!("print enum self");
    }
}
