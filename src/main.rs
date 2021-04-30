use std::error::Error;
use std::io::{self, ErrorKind};

// Result<(), Box<dyn Error>>
// Box<dyn Error> 被称为 “trait 对象”（“trait object”）,可为函数返回任意类型错误
fn main() -> Result<(), Box<dyn Error>> {
    // ----- 错误处理 -----

    // panic!("panic(恐慌) 这是一个不可恢复的错误，会终止程序");

    // 试图访问炒超过v的长度，此时必须panic
    // let v = vec![1,2,3];
    // let value = v[100];

    // version 1

    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(f) => f,
    //     // 应对不同错误
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => {
    //             // 创建文件
    //             let f = File::create("hello.txt");
    //             let f = match f {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("创建文件[hello.txt]失败: {:?}", e),
    //             };
    //             f
    //         },
    //         other => panic!("读取文件[hello.txt]出错: {:?}", other),
    //     },
    // };

    // version 2

    // let file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("创建文件[hello.txt]失败: {:?}", error);
    //         })
    //     } else {
    //         panic!("读取文件[hello.txt]出错: {:?}", error);
    //     }
    // });

    // version 3

    // 如果找不到文件 unwrap，将直接panic
    // let file = File::open("hello.txt").unwrap();
    // 自定义错误内容 expect
    // let file = File::open("hello.txt").expect("读取文件[hello.txt]失败");

    // let username = match read_username_from_file() {
    //     Ok(name) => name,
    //     Err(err) => panic!("读取文件[hello.txt]失败: {:?}", err),
    // };
    // println!("读取文件内容：{}", username);

    // version 4
    File::fs::read_to_string("hello.txt")?;
    Ok(())
}

// 传播错误
// fn read_username_from_file() -> Result<String, io::Error> {
//     let file = File::open("hello.txt");

//     let mut file = match file {
//         Ok(file) => file,
//         Err(err) => return Err(err),
//     };

//     let mut s = String::from("");
//     match file.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(err) => Err(err),
//     }
// }

// 传播错误的简写：? 运算符
// ? 运算符可被用于返回 Result 的函数，所在的函数必须返回Result类型
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut file = File::open("hello.txt")?;
//     let mut s = String::from("");
//     file.read_to_string(&mut s)?;
//     Ok(s)
// }
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::from("hello.txt");
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
