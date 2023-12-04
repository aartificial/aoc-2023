#[derive(Debug, Default)]
pub struct Score {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Score {
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
