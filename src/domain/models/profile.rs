#[derive(Clone, Debug)]
pub struct Profile {
    full_name: String,
    group_name: String,
}

impl Profile {
    pub fn new(full_name: impl Into<String>, group_name: impl Into<String>) -> Profile {
        Profile { 
            full_name: full_name.into(), 
            group_name: group_name.into(),
        }
    }
    
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
    
    pub fn group_name(&self) -> &str {
        &self.group_name
    }
}
