pub mod roster;
pub mod state;
pub mod ui;

use crate::shared::{OperativeStatus, OperativeStats};

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
