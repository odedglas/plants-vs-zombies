#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum ResourceKind {
    Card,
    Interface,
    Plant,
    Zombie,
    Level,
}

impl ResourceKind {
    pub fn value(&self) -> &str {
        match *self {
            ResourceKind::Card => "card",
            ResourceKind::Interface => "interface",
            ResourceKind::Plant => "plant",
            ResourceKind::Zombie => "zombie",
            ResourceKind::Level => "level",
        }
    }
}

pub enum ResourceDataType {
    Cell,
    Data,
    Image,
}

impl ResourceDataType {
    pub fn value(&self) -> &str {
        match *self {
            ResourceDataType::Cell => "cell",
            ResourceDataType::Data => "data",
            ResourceDataType::Image => "image",
        }
    }
}
