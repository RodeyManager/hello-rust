use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // --------- 智能指针 ------------

    // 1.智能指针通常使用结构体(struct)实现, 智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait.
    // - Box<T>，用于在堆上分配值
    // - Rc<T>，一个引用计数类型，其数据可以有多个所有者
    // - Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问。（ RefCell<T> 是一个在运行时而不是在编译时执行借用规则的类型）

    // 使用Box <T>指向堆上的数据
    let b = Box::new(5);
    println!("b: {}", b);

    // Box 允许创建递归类型
    let cons = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("cons: {:?}", cons);

    // 一：通过 Deref trait 将智能指针当作常规引用处理

    // 1、通过解引用运算符追踪指针的值
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // 如果希望对 y 的值做出断言，必须使用 *y 来追踪引用所指向的值（也就是 解引用）
    assert_eq!(5, *y);

    // 2、像引用一样使用 Box<T>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 3、自定义智能指针
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 4、通过实现 Deref trait 将某类型像引用一样处理
    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 二、函数和方法的隐式解引用强制多态
    // 解引用强制多态（deref coercions）是 Rust 在函数或方法传参上的一种便利

    fn hello(name: &str) {
        println!("Hello {}", name);
    }

    let message = MyBox::new(String::from("Rust"));
    hello("Java");
    hello(&message);
    // 如果没有实现解引用强制多态，则需要像下面这样调用hello
    let message = MyBox::new(String::from("Python"));
    hello(&(*message)[..]);

    // 三、使用 Drop Trait 运行清理代码
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("释放结构体数据 `{}`", self.data);
        }
    }

    let _a = CustomSmartPointer {
        data: String::from("Rust 程序设计语言"),
    };
    // 提前清理
    drop(_a);
    let _b = CustomSmartPointer {
        data: String::from("C++ 程序设计语言"),
    };
    println!("CustomSmartPointer 创建");

    // 四、Rc<T> 引用计数智能指针
    let a = Rc::new(Mapping::Cons(
        5,
        Rc::new(Mapping::Cons(10, Rc::new(Mapping::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Mapping::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Mapping::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(KandList::Cons(Rc::clone(&value), Rc::new(KandList::Nil)));
    let b = KandList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = KandList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum Mapping {
    Cons(i32, Rc<Mapping>),
    Nil,
}

#[derive(Debug)]
enum KandList {
    Cons(Rc<RefCell<i32>>, Rc<KandList>),
    Nil,
}
