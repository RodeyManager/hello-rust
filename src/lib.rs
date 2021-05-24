pub mod tools {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn from(list: Vec<i32>) -> AveragedCollection {
            let mut instance = AveragedCollection { list, average: 0.0 };
            instance.update_average();
            instance
        }

        pub fn add(&mut self, item: i32) {
            self.list.push(item);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        // average getter
        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

pub mod gui {

    pub trait Draw {
        fn draw(&self);
    }

    // 屏幕
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Color {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
        pub alpha: u8,
    }
    // Button
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub color: Color,
        pub label: Label,
    }
    impl Button {
        pub fn click(&self) {
            println!("Button clicked");
        }
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("Button draw in to screen...");
        }
    }
    // Label
    pub struct Label {
        pub text: String,
        pub color: Color,
    }
    impl Draw for Label {
        fn draw(&self) {
            println!("Label draw in to screen...");
        }
    }

    // TextField
    pub struct TextField {
        pub label: Label,
        pub placeholder: String,
        pub text: String,
    }
    impl Draw for TextField {
        fn draw(&self) {
            println!("TextField draw in to screen...");
        }
    }

    // SelectBox
    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("SelectBox draw in to screen...");
        }
    }
}
