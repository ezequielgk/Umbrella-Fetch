use crate::lore::{
    FieldLogEntry, ThreatZone, BioAsset, REGION_POOL, LOG_POOL, 
    CLEARANCE_POOL, DIVISION_POOL, FACILITY_POOL, PROTOCOL_POOL, 
    AUTH_POOL, PROJECT_POOL, STATUS_POOL, DIRECTIVE_POOL, BIO_ASSET_POOL
};
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::RngExt;

pub const UMBRELLA_NETWORK: &str = "UMBRELLA-INTRANET-CORE";

#[derive(Clone)]
pub struct AppConfig {
    pub clearance: String,
    pub division: String,
    pub facility_id: String,
    pub project_codename: String,
    pub security_protocol: String,
    pub current_directive: String,
    pub auth_code: String,
    pub system_status: String,
}

pub struct Config {
    pub app: AppConfig,
    pub active_threat_zones: Vec<ThreatZone>,
    pub active_field_logs: Vec<FieldLogEntry>,
    pub active_bio_assets: Vec<BioAsset>,
}

impl Config {
    pub fn load() -> Self {
        let mut rng = rand::rng();
        
        let app_config = AppConfig {
            clearance: CLEARANCE_POOL.choose(&mut rng).unwrap().to_string(),
            division: std::env::var("UMBRELLA_DIVISION").unwrap_or_else(|_| DIVISION_POOL.choose(&mut rng).unwrap().to_string()),
            facility_id: FACILITY_POOL.choose(&mut rng).unwrap().to_string(),
            project_codename: PROJECT_POOL.choose(&mut rng).unwrap().to_string(),
            security_protocol: PROTOCOL_POOL.choose(&mut rng).unwrap().to_string(),
            current_directive: DIRECTIVE_POOL.choose(&mut rng).unwrap().to_string(),
            auth_code: AUTH_POOL.choose(&mut rng).unwrap().to_string(),
            system_status: STATUS_POOL.choose(&mut rng).unwrap().to_string(),
        };

        let mut regions = REGION_POOL.to_vec();
        regions.shuffle(&mut rng);
        let active_threat_zones = regions.into_iter().take(5).map(|name| {
            ThreatZone { name, level: rng.random_range(1..=5) }
        }).collect();
        
        let mut logs = LOG_POOL.to_vec();
        logs.shuffle(&mut rng);
        let active_field_logs = logs.into_iter().take(5).collect();
        
        let mut assets = BIO_ASSET_POOL.to_vec();
        assets.shuffle(&mut rng);
        let active_bio_assets = assets.into_iter().take(5).collect();

        Self {
            app: app_config,
            active_threat_zones,
            active_field_logs,
            active_bio_assets,
        }
    }
}
