use rand::Rng;
use std::io;

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen::<u32>();

    println!("密码数字: {}", secret_number);
    println!("请输入一个数字: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取失败");

    println!("当前数字为: {}", guess);
}
