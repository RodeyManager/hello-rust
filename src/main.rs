use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("密码数字: {}", secret_number);
        println!("请输入一个数字: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("当前数字为: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
