use js_sys::Math;

pub struct Sprite {
    id: String,
    name: String,
}

impl Sprite {
    pub fn new(name: &str) -> Sprite {
        Sprite {
            id: uid(name),
            name: String::from(name)
        }
    }
}

pub fn uid(prefix: &str) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}