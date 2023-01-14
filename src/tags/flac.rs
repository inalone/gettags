use crate::*;

pub struct FlacTags {}

impl TagRead for FlacTags {
    fn from_buffer(&self, buffer: &Vec<u8>) -> BoxedTags {
        todo!()
    }
}

impl TagWrite for FlacTags {}

impl_tags!(FlacTags);
