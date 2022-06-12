pub fn oop() {
    {
        // 状态模式
        struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
            fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }
            fn reject(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.reject())
                }
            }
            fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(self)
            }
        }

        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn reject(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
        }

        struct Draft {}
        struct PendingReview {}
        struct Published {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                Box::new(Published {})
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                Box::new(Draft {})
            }
        }
        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
        }
    }

    {
        // 类型模式
        struct Post {
            content: String,
        }
        struct DraftPost {
            content: String,
        }
        struct PendingReviewPost {
            content: String,
        }

        impl Post {
            fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }
            fn content(&self) -> &str {
                &self.content
            }
        }
        impl DraftPost {
            fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }
            fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }
        impl PendingReviewPost {
            fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }
    }
}
