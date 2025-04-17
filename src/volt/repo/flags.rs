
pub struct Flags {
    pub has_zettels: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags { has_zettels: false }
    }

    pub fn with_zettels(mut self) -> Self {
        self.has_zettels = true;
        self
    }
}
