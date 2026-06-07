pub mod roster;
pub mod state;
pub mod ui;

#[derive(Clone, Copy, PartialEq)]
pub enum OperativeStatus {
    Active,
    Kia,
    Mia,
}

pub struct OperativeStats {
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
    pub endurance: u8,
}

pub struct Operative {
    pub name: &'static str,
    pub rank: &'static str,
    pub squad: &'static str,
    pub origin: &'static str,
    pub speciality: &'static str,
    pub weapon: &'static str,
    pub status: OperativeStatus,
    pub price_usd: u32,
    pub stats: OperativeStats,
}
