#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Profile {
    pub full_name: String,
    pub group_name: String,
}

impl Profile {
    pub fn new(full_name: String, group: String) -> Profile {
        Profile { full_name, group_name: group }
    }
}
