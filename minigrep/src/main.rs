/**
 *
 * 一个命令行小程序（实例训练）
 *
*/
use minigrep::{Config, Console};
use std::process;

fn main() {
    // ---- 1、
    // collect 是一个经常需要注明类型的函数，因为 Rust 不能推断出你想要什么类型的集合。
    // let args: Vec<String> = env::args().skip(1).collect();
    // let query = &args[0];
    // let filename = &args[1];

    // println!("query: {}, filename: {}", config.query, config.filename);

    // let contents = match read_file_content(&config.filename) {
    //     Ok(c) => c,
    //     Err(err) => {
    //         panic!("读取文件失败：{:?}", err);
    //     }
    // };

    // println!("contents: {}", contents);

    // ---- 2、
    // 重构改进模块性和错误处理
    let args: Vec<String> = std::env::args().skip(1).collect();

    // let config = Config::new(&args);
    // 非零的退出状态是一个惯例信号，用来告诉调用程序的进程：该程序以错误状态退出了。
    let config = Config::new(&args).unwrap_or_else(|err| {
        Console::error(format!("× {}", err).as_str());
        process::exit(0);
    });
    Console::warn(
        format!(
            "::<关键字: {}, 当前文件: {}, 大小写匹配: {}>\n",
            config.query,
            config.filename,
            config.get_case_text()
        )
        .as_str(),
    );

    // 处理
    if let Err(err) = minigrep::run(config) {
        Console::error(format!("× {}", err).as_str());
        process::exit(0);
    }
}
