pub trait Render<T> {
    fn render(&self) -> T;
}

pub mod plain_text {
    use std::ops::Deref;

    pub struct PlainText(pub String);

    impl Deref for PlainText {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

pub mod markdown {
    use std::ops::Deref;

    pub struct Markdown(pub String);

    impl Deref for Markdown {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
