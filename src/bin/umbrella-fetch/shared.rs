#[derive(Clone, Copy, PartialEq)]
pub enum OperativeStatus {
    Active,
    Kia,
    Mia,
    Retired,
}

pub struct OperativeStats {
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
    pub endurance: u8,
}
