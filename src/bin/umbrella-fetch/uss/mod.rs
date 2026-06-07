pub mod roster;
pub mod state;
pub mod ui;
pub mod unit_info;

use crate::shared::{OperativeStats, OperativeStatus};

pub struct UssOperative {
    pub codename: &'static str,
    pub real_name: Option<&'static str>,
    pub alpha_id: &'static str,
    pub ops_total: u8,
    pub ops_survived: u8,
    pub stats: OperativeStats,
    pub weapon: &'static str,
    pub speciality: &'static str,
    pub status: OperativeStatus,
    pub price_usd: u32,
    pub origin: &'static str,
}
