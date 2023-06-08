use rand::{distributions::Standard, prelude::Distribution};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Speed(pub usize);
