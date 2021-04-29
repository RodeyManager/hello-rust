use std::fs::File;
use std::io::ErrorKind;

fn main() {

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





}
