pub struct HasName {
    pub name: String
}

impl HasName {
    pub fn new(name: &str) -> HasName { HasName { name: name.to_string() } }
}
