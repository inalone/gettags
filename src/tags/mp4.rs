use crate::*;

pub struct MP4Tags {}

impl TagRead for MP4Tags {
    fn from_buffer(&self, buffer: &Vec<u8>) -> BoxedTags {
        todo!()
    }
}

impl TagWrite for MP4Tags {}

impl_tags!(MP4Tags);
