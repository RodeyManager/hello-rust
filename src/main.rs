use hello_rust::gui::{Button, Color, Label, Screen, SelectBox, TextField};
use hello_rust::tools::AveragedCollection;

fn main() {
    let collection = vec![10, 11, 1, 2, 4, 2, 5];
    let mut average_collection = AveragedCollection::from(collection);
    println!("collection average value: {}", average_collection.average());
    average_collection.add(3);
    println!(
        "add after collection average value: {}",
        average_collection.average()
    );
    average_collection.remove();
    println!(
        "remove after collection average value: {}",
        average_collection.average()
    );

    let color = Color {
        red: 255,
        green: 0,
        blue: 0,
        alpha: 1,
    };
    let btn_label = Label {
        text: String::from("Click Me"),
        color,
    };
    let email_label = Label {
        text: String::from("Username: "),
        color: color.clone(),
    };
    let button = Box::new(Button {
        width: 100,
        height: 30,
        label: btn_label,
        color: color.clone(),
    });
    let text_field = Box::new(TextField {
        text: String::from(""),
        label: email_label,
        placeholder: String::from("Please input email"),
    });
    let select_box = Box::new(SelectBox {
        width: 100,
        height: 30,
        options: vec![String::from("Rust languge"), String::from("Java languge")],
    });
    let screen = Screen {
        components: vec![text_field, select_box, button],
    };
    screen.run();
    println!("\n");

    // trait 对象安全规则：
    // - 返回值类型不为 Self
    // - 方法没有任何泛型类型参数

    // 面向对象设计模式的实现 实例
    use blog::Post;

    let mut post = Post::new();
    post.add_text("沉重哀悼袁隆平院士");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("博文是否公布，待定中...", post.content());

    post.approve();
    assert_eq!("沉重哀悼袁隆平院士", post.content());
}
