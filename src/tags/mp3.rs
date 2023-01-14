use crate::*;

pub struct MP3Tags {}

impl TagRead for MP3Tags {
    fn from_buffer(&self, buffer: &Vec<u8>) -> BoxedTags {
        todo!()
    }
}

impl TagWrite for MP3Tags {}

impl_tags!(MP3Tags);
