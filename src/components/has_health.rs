pub struct HasHealth {
    pub health: usize
}

impl HasHealth {
    pub fn new(health: usize) -> HasHealth { HasHealth { health: health } }
}
