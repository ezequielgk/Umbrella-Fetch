//! Interactive TUI state for the U.S.S. command.

use super::roster::ROSTER;
use super::UssOperative;
use crate::shared::OperativeStatus;

/// In-memory state of the U.S.S. interactive panel.
pub struct UssAppState {
    pub selected_index: usize,
    pub show_detail: bool,
    pub filter_squad: Option<&'static str>,
    pub filter_status: Option<OperativeStatus>,
    pub sort_price_asc: Option<bool>,
}

impl UssAppState {
    /// Initializes a clean empty state.
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            show_detail: false,
            filter_squad: None,
            filter_status: None,
            sort_price_asc: None,
        }
    }

    /// Filters the immutable U.S.S. `ROSTER` with current configuration.
    pub fn get_filtered_roster(&self) -> Vec<&'static UssOperative> {
        let mut filtered: Vec<&'static UssOperative> = ROSTER.iter().filter(|op| {
            if let Some(sq) = self.filter_squad {
                // Not standard "squad" in USS, but they have alpha_id
                // We'll filter based on prefix or just dummy it out.
                // The prompt mentions `--squad alpha` -> filter by team.
                // UssOperative doesn't have a `squad` field, but has `alpha_id`.
                // Let's filter by checking if `alpha_id` starts with `sq`.
                if !op.alpha_id.starts_with(sq) { return false; }
            }
            if let Some(st) = self.filter_status {
                if op.status != st { return false; }
            }
            true
        }).collect();

        if let Some(asc) = self.sort_price_asc {
            filtered.sort_by(|a, b| {
                if asc {
                    a.price_usd.cmp(&b.price_usd)
                } else {
                    b.price_usd.cmp(&a.price_usd)
                }
            });
        }
        filtered
    }

    /// Moves the list cursor down.
    pub fn next(&mut self) {
        let max = self.get_filtered_roster().len();
        if max > 0 {
            self.selected_index = (self.selected_index + 1) % max;
        }
    }

    /// Moves the list cursor up.
    pub fn previous(&mut self) {
        let max = self.get_filtered_roster().len();
        if max > 0 {
            if self.selected_index == 0 {
                self.selected_index = max - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    /// Rotates squad filters.
    pub fn cycle_squad_filter(&mut self) {
        self.filter_squad = match self.filter_squad {
            None => Some("ALPHA"),
            Some("ALPHA") => Some("BRAVO"),
            Some("BRAVO") => Some("DELTA"),
            Some("DELTA") => None,
            _ => None,
        };
        self.reset_selection();
    }

    /// Rotates status filters.
    pub fn cycle_status_filter(&mut self) {
        self.filter_status = match self.filter_status {
            None => Some(OperativeStatus::Active),
            Some(OperativeStatus::Active) => Some(OperativeStatus::Kia),
            Some(OperativeStatus::Kia) => Some(OperativeStatus::Mia),
            Some(OperativeStatus::Mia) => Some(OperativeStatus::Retired),
            Some(OperativeStatus::Retired) => None,
        };
        self.reset_selection();
    }

    /// Toggles the salary sorting algorithm.
    pub fn toggle_price_sort(&mut self) {
        self.sort_price_asc = match self.sort_price_asc {
            None => Some(true),
            Some(true) => Some(false),
            Some(false) => None,
        };
        self.reset_selection();
    }

    fn reset_selection(&mut self) {
        self.selected_index = 0;
    }
}
