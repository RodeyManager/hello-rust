#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 * 10, 20);
    }

    // panic!就会失败
    // #[test]
    // fn another() {
    //     panic!("测试失败");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Reactangle{ width: 3, height: 10 };
        let smaller = Reactangle{ width: 2, height: 8 };

        // assert!宏为true表示通过，总是接受一个bool值
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let large = Reactangle{ width: 10, height: 20 };
        let smaller = Reactangle{ width: 5, height: 15 };

        assert!(!smaller.can_hold(&large));
    }

    #[test]
    fn is_add_two () {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn no_add_two () {
        assert_ne!(5, add_two(2));
    }

    #[test]
    #[should_panic]
    fn greater_than_100 () {
        Guess::new(120);
    }

    // 将 Result<T, E> 用于测试
    // 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解
    #[test]
    fn is_work() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2加2不等于4"))
        }
    }

    // 并行或连续的运行测试, --test-thread来设置并行运行线程数
    // cargo test -- --test-threads=1

    // 显示函数输出 --nocapture来设置测试通过时也输出println!
    // cargo test -- --nocapture

    // 通过指定名字来运行部分测试
    // cargo test <名称>

    // 过滤运行多个测试，将会运行以is开头的所有测试案例
    // cargo test is

    // 忽略某些测试
    // 在 #[test] 之后增加了 #[ignore] 行
    // --ignored 只忽略被标记为#[ignore]的测试案例
    // cargo test -- --ignored


    // 测试的组织结构(单元测试 & 集成测试)

    // 单元测试：
    //  单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。

    // 测试模块和 #[cfg(test)]
    // 单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定他们不应该被包含进编译结果中。

    // 集成测试
    // 为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级
    // cargo test --test integration_test 运行特定的集成测试文件

}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("请输入1到100之间的任何数字");
        }
        Guess { value }
    }
}


pub fn add_two (a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
    struct Reactangle {
        width: u32,
        height: u32,
    }

    impl Reactangle {
        fn can_hold(&self, other: &Reactangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }