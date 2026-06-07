#[derive(Clone, Copy)]
pub struct BioAsset {
    pub name: &'static str,
    pub price: u32,
}

pub const BIO_ASSET_POOL: &[BioAsset] = &[
    BioAsset { name: "T-Virus Sample", price: 2450000 },
    BioAsset { name: "G-Virus Extract", price: 9800000 },
    BioAsset { name: "Veronica-X", price: 7620000 },
    BioAsset { name: "Las Plagas", price: 5980000 },
    BioAsset { name: "Nemesis-T", price: 8500000 },
    BioAsset { name: "Uroboros", price: 3310000 },
    BioAsset { name: "Hunter Alpha", price: 1250000 },
    BioAsset { name: "Hunter Beta", price: 1450000 },
    BioAsset { name: "Licker", price: 850000 },
    BioAsset { name: "Bandersnatch", price: 620000 },
    BioAsset { name: "Tyrant T-103", price: 4200000 },
    BioAsset { name: "C-Virus", price: 11200000 },
    BioAsset { name: "Progenitor", price: 25000000 },
    BioAsset { name: "Mold Sample", price: 1800000 },
    BioAsset { name: "Plaga Type-2", price: 2200000 },
];

#[derive(Clone, Copy)]
pub struct FieldLogEntry {
    pub date: &'static str,
    pub entry: &'static str,
}

pub const LOG_POOL: &[FieldLogEntry] = &[
    FieldLogEntry { date: "1998-05-11", entry: "Outbreak at Arklay Laboratory." },
    FieldLogEntry { date: "1998-07-24", entry: "Mansion Incident. S.T.A.R.S. engaged." },
    FieldLogEntry { date: "1998-09-23", entry: "T-Virus leak in Raccoon City." },
    FieldLogEntry { date: "1998-10-01", entry: "Sterilization operation (Code: XX)." },
    FieldLogEntry { date: "2003-02-18", entry: "Caucasus facility infiltrated." },
    FieldLogEntry { date: "2004-04-12", entry: "Las Plagas sample secured." },
    FieldLogEntry { date: "2009-03-05", entry: "Uroboros project terminated." },
    FieldLogEntry { date: "2012-12-24", entry: "C-Virus outbreak in Edonia." },
    FieldLogEntry { date: "2013-06-29", entry: "Tall Oaks bio-terror incident." },
    FieldLogEntry { date: "2017-07-20", entry: "Dulvey incident isolated." },
    FieldLogEntry { date: "2021-02-08", entry: "Megamycete activity recorded." },
];

#[derive(Clone)]
pub struct ThreatZone {
    pub name: &'static str,
    pub level: u8,
}

pub const REGION_POOL: &[&str] = &[
    "NORTH AMERICA", "EUROPE", "ASIA", "AFRICA", "ANTARCTICA", 
    "SOUTH AMERICA", "ARKLAY REGION", "RACCOON CITY", "EDONIA", 
    "LANSHIANG", "KIJUJU", "ROCKFORT ISL.", "TERRAGRIGIA",
    "CAUCASUS FAC", "PARIS LABS"
];

pub const TICKER_MESSAGES: &[&str] = &[
    "UMBRELLA CORP SECURE FEED >>>",
    "WARNING: CONTAINMENT BREACH DETECTED IN SECTOR 4 >>>",
    "ALL PERSONNEL EVACUATE TO DESIGNATED SAFE ZONES >>>",
    "U.B.C.S. DEPLOYMENT AUTHORIZED >>>",
    "OUR BUSINESS IS LIFE ITSELF >>>",
];

pub const CLEARANCE_POOL: &[&str] = &[
    "LEVEL-1 TECHNICIAN", "LEVEL-2 SECURITY", "LEVEL-3 SCIENTIST", 
    "LEVEL-4 RESEARCHER", "LEVEL-5 EXECUTIVE", "U.B.C.S. OPERATIVE", 
    "U.S.S. COMMANDER", "DIRECTOR", "HUMAN-UNIT-NEVER-KILLED"
];

pub const DIVISION_POOL: &[&str] = &[
    "APPLIED RESEARCH", "BIOWEAPON R&D", "VIROLOGY", "SECURITY (U.S.S.)", 
    "CONTAINMENT (U.B.C.S.)", "WHITE UMBRELLA", "RED UMBRELLA", 
    "PHARMACEUTICALS", "BLACK OPS", "INTELLIGENCE"
];

pub const FACILITY_POOL: &[&str] = &[
    "NEST-01", "NEST-02", "ARKLAY MANSION", "PARIS LABS", 
    "CAUCASUS FACILITY", "ROCKFORT ISLAND", "SPENCER ESTATE", 
    "AFRICA SECTOR", "ANTARCTICA BASE", "QUEEN ZENOBIA"
];

pub const PROTOCOL_POOL: &[&str] = &[
    "ALPHA-RED", "OMEGA-BLACK", "CODE: VERONICA", "STERILIZATION", 
    "CONTAINMENT", "EVACUATION", "OBSERVATION", "QUARANTINE", "PURGE"
];

pub const AUTH_POOL: &[&str] = &[
    "U-7734-X", "A-9921-Z", "X-0012-G", "C-8842-T", "S-1123-W", 
    "M-4421-V", "R-9988-B", "K-1029-P"
];

pub const PROJECT_POOL: &[&str] = &[
    "TYRANT T-103", "NEMESIS-ALPHA", "G-TYPE", "W-KERBEROS", 
    "HUNTER ALPHA", "LICKER", "BANDERSNATCH", "UROBOROS"
];

pub const STATUS_POOL: &[&str] = &[
    "▓ OPERATIONAL", "▓ COMPROMISED", "▓ LOCKDOWN", 
    "▓ CRITICAL", "▓ PURGE ACTIVE", "▓ EVACUATING"
];

pub const DIRECTIVE_POOL: &[&str] = &[
    "OBSERVE AND REPORT", "ERADICATE ALL SURVIVORS", "SECURE G-VIRUS", 
    "DEFEND FACILITY", "MAINTAIN QUARANTINE", "EXTRACT VIP"
];
