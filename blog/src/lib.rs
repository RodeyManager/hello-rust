// 博文状态
trait State {
    // 请求审核
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // 批准
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 返回博文内容
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        println!("->新增博文");
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
        println!("->添加内容(草稿)");
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

// 草稿
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("->提交审核(待审核)");
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// 待审核
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("->审核通过(初审通过)");
        Box::new(Scheduled {})
    }
}

// 增加一个状态
// 待定
struct Scheduled {}

impl State for Scheduled {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("->待定公布");
        Box::new(Published {})
    }

    // 特意输出提示，审核中
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "博文是否公布，待定中..."
    }
}

// 公布
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // 只有审核通过的状态才返回博文内容
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        println!("->公布");
        &post.content
    }
}
