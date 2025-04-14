use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WeekId {
    pub year: i32,
    pub week: u32,
}

impl WeekId {
    pub fn new(year: i32, week: u32) -> Self {
        Self { year, week }
    }
}

impl Into<(i32, u32)> for WeekId {
    fn into(self) -> (i32, u32) {
        (self.year, self.week)
    }
}

impl Display for WeekId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.year, self.week)
    }
}
