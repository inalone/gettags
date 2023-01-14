pub type BoxedTags = Box<dyn Tags>;

pub trait TagRead {
    fn from_buffer(&self, buffer: &Vec<u8>) -> BoxedTags;
}

pub trait TagWrite {}

pub trait Tags: TagRead + TagWrite {}

#[macro_export]
macro_rules! impl_tags {
    ($type:ty) => {
        impl Tags for $type {}

        impl $type {
            pub fn new() -> Self {
                Self {}
            }
        }
    };
}
