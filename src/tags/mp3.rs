use crate::*;

pub struct MP3Tags {}

impl TagRead for MP3Tags {}

impl TagWrite for MP3Tags {}

impl_tags!(MP3Tags);
