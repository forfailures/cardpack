use crate::fluent::FluentName;

pub struct Standard52Rank {
    pub weight: u32,
    pub name: FluentName,
}

impl Standard52Rank {
    pub fn new(weight: u32, name: FluentName) -> Self {
        Standard52Rank { weight, name }
    }
}
