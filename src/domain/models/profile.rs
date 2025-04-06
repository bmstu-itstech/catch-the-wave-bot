#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Profile {
    pub full_name: String,
    pub group_name: String,
}

impl Profile {
    pub fn new(full_name: impl Into<String>, group_name: impl Into<String>) -> Profile {
        Profile { 
            full_name: full_name.into(), 
            group_name: group_name.into(),
        }
    }
}
