pub trait TagRead {}

pub trait TagWrite {}

pub trait Tags: TagRead + TagWrite {}

#[macro_export]
macro_rules! impl_tags {
    ($type:ty) => {
        impl Tags for $type {}
    };
}
