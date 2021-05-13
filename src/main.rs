fn main() {
  // Rust 中的函数式语言功能：迭代器与闭包
  // 什么是函数式编程风格：函数式编程风格通常包含将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行等等。
  // 闭包（Closures），一个可以储存在变量里的类似函数的结构
  // 迭代器（Iterators），一种处理元素序列的方式

  // 闭包：可以捕获环境的匿名函数

  let intensity = 10;
  let random = 6;
  generate_workout(intensity, random);
}

fn generate_workout(intensity: u32, random: u32) {
  // 1、以下多次调用 simulated_expensive_calculation(intensity)
  // if intensity < 25 {
  //   println!(
  //     "Today, do {} pushups",
  //     simulated_expensive_calculation(intensity)
  //   );
  //   println!(
  //     "Next, do {} situps",
  //     simulated_expensive_calculation(intensity)
  //   );
  // } else {
  //   if random == 3 {
  //     println!("Take a break today! Remember to stay hydrated!");
  //   } else {
  //     println!(
  //       "Today, run for {} minutes",
  //       simulated_expensive_calculation(intensity)
  //     );
  //   }
  // }

  // 2、使用变量提取函数调用，保证只调用一次(注意：任何情况下都会执行一次，
  // 但有某些条件下不需要执行（比如intensity > 25 && random == 3），继续优化)
  // let expensive_result = simulated_expensive_calculation(intensity);
  // if intensity < 25 {
  //   println!("Today, do {} pushups", expensive_result);
  //   println!("Next, do {} situps", expensive_result);
  // } else {
  //   if random == 3 {
  //     println!("Take a break today! Remember to stay hydrated!");
  //   } else {
  //     println!("Today, run for {} minutes", expensive_result);
  //   }
  // }

  // 3、重构使用闭包储存代码
  // let expensive_closure = |num| {
  //   println!("calculating slowly...");
  //   std::thread::sleep(std::time::Duration::from_secs(2));
  //   num
  // };

  // if intensity < 25 {
  //   let expensive_result = expensive_closure(intensity);
  //   println!("Today, do {} pushups", expensive_result);
  //   println!("Next, do {} situps", expensive_result);
  // } else {
  //   if random == 3 {
  //     println!("Take a break today! Remember to stay hydrated!");
  //   } else {
  //     println!("Today, run for {} minutes", expensive_closure(intensity));
  //   }
  // }

  // 4、利用结构体保存闭包，优化多次调用(建立缓存)
  // 闭包不要求像 fn 函数那样在参数和返回值上注明类型。
  let mut cache = Cacher::new(|num| {
    println!("calculating slowly...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Today, do {} pushups", cache.value(intensity));
    println!("Next, do {} situps", cache.value(intensity));
  } else {
    if random == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!("Today, run for {} minutes", cache.value(intensity));
    }
  }
  // 请尝试引入更多泛型参数来增加 Cacher 功能的灵活性。【作业】

  // 闭包会捕获其环境
  let x = 4;
  let equal_to_x = |z| z == x;
  let y = 4;
  assert!(equal_to_x(y));

  // 迭代器Iterator

  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();
  println!("total: {}", total);

  // 迭代器适配器
  let v1 = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  println!("v2: {:?}", v2);

  // 实现 Iterator trait 来创建自定义迭代器
  let mut counter = Counter::new();
  println!("counter first value: {:?}", counter.next().unwrap());

  // 使用自定义迭代器中其他 Iterator trait 方法
  let counter1 = Counter::new();
  let counter2 = Counter::new();
  let sum: u32 = counter1
    .zip(counter2.skip(1))
    .map(|(x, y)| x * y)
    .filter(|x| x % 3 == 0)
    .sum();

  println!("The sum: {}", sum);
  // zip 在任一输入迭代器返回 None 时也返回 None
}

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;

    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[test]
fn counter_test() {
  let mut counter = Counter::new();

  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  // into_iter 生成获取Vec所有权的Iterator
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
  let shoes = vec![
    Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
    Shoe {
      size: 13,
      style: String::from("sandal"),
    },
    Shoe {
      size: 10,
      style: String::from("boot"),
    },
  ];

  let in_my_size = shoes_in_my_size(shoes, 10);

  assert_eq!(
    in_my_size,
    vec![
      Shoe {
        size: 10,
        style: String::from("sneaker")
      },
      Shoe {
        size: 10,
        style: String::from("boot")
      },
    ]
  );
}

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calcutation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calcutation: T) -> Cacher<T> {
    Cacher {
      calcutation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calcutation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("calculating slowly...");
  std::thread::sleep(std::time::Duration::from_secs(2));
  intensity
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn call_with_diff_value() {
    let mut cache = Cacher::new(|a| a);

    let v1 = cache.value(1);
    let v2 = cache.value(2);

    assert_eq!(v2, 2);
  }

  #[test]
  fn iterator_next() {
    let list = vec![1, 2, 3];
    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    // 使用next方法时，iter必须时可变的，原因时next会消费每一项
    // 使用for时，iter无需是可变的，因为for会获取每一行的所有权
  }
}
