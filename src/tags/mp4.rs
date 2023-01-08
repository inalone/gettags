use crate::*;

pub struct MP4Tags {}

impl TagRead for MP4Tags {}

impl TagWrite for MP4Tags {}

impl_tags!(MP4Tags);
