// 程序运行
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let contents = std::fs::read_to_string(config.filename)?;

  let lines = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  if lines.len() <= 0 {
    println!("<提示> 未找到相关内容\n");
    std::process::exit(0);
  }

  for line in lines {
    println!("{}", line);
  }

  Ok(())
}

// 搜索字符串
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();

  // 1、读取每一行数据
  // 2、过滤首尾空格
  for line in contents.lines() {
    if line.contains(&query) {
      result.push(line);
    }
  }

  result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("请输入完整参数\x1b[31m(exp: minigrep <search-string> <local-file>)\x1b[39m");
    }
    let query = args[0].clone();
    let filename = args[1].clone();
    // 检查环境变量是否设置（不关心其值）
    // powershell:
    // 设置： $Env:CASE_INSENSITIVE=1
    // 删除：$Env:CASE_INSENSITIVE=$null
    let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }

  pub fn get_case_text(&self) -> String {
    if self.case_sensitive {
      String::from("否")
    } else {
      String::from("是")
    }
  }
}

pub struct Console {}

impl Console {
  pub fn log(message: &str) {
    println!("{}", message);
  }

  pub fn error(message: &str) {
    eprintln!("\x1b[31m{}\x1b[39m", message);
  }

  pub fn warn(message: &str) {
    println!("\x1b[33m{}\x1b[39m", message);
  }

  pub fn info(message: &str) {
    println!("\x1b[36m{}\x1b[39m", message);
  }

  pub fn success(message: &str) {
    println!("\x1b[32m{}\x1b[39m", message);
  }
}

// ------------------------------- tests ----------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "productive";
    let contents = "\n
一个总是会返回空 vector 的 search 函数定义\n
空 vector 并不匹配一个包含一行\n
\"safe, fast, productive.\" 的 vector 而失败。";

    assert_eq!(
      vec!["\"safe, fast, productive.\" 的 vector 而失败。"],
      search_case_insensitive(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "SaFE";
    let contents = "\n
一个总是会返回空 vector 的 search 函数定义\n
空 vector 并不匹配一个包含一行\n
\"safe, fast, productive.\" 的 vector 而失败。";

    assert_eq!(
      vec!["\"safe, fast, productive.\" 的 vector 而失败。"],
      search_case_insensitive(query, contents)
    );
  }
}
