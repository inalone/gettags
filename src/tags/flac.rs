pub use crate::*;

pub struct FlacTags {}

impl TagRead for FlacTags {}

impl TagWrite for FlacTags {}

impl Tags for FlacTags {}
