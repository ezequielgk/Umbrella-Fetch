pub struct UssUnitInfo {
    pub commander:    &'static str,
    pub codename:     &'static str,
    pub division:     &'static str,
    pub founded:      &'static str,
    pub mission:      &'static str,
    pub clearance:    &'static str,
    pub deniability:  &'static str,
    pub contract:     &'static str,
    pub total_ops:    u8,
    pub ops_success:  u8,
    pub ops_failed:   u8,
    pub survival_avg: &'static str,
    pub last_op:      &'static str,
    pub board_quote:  &'static str,
    pub board_date:   &'static str,
}

pub struct ActiveTarget {
    pub number:    u8,
    pub name:      &'static str,
    pub redacted:  bool,
    pub priority:  TargetPriority,
}

pub enum TargetPriority { High, Medium, Low }

pub struct CovertOp {
    pub date:    &'static str,
    pub result:  OpResult,
    pub summary: &'static str,
}

pub enum OpResult { Success, Failed, Ongoing }

pub const UNIT_INFO: UssUnitInfo = UssUnitInfo {
    commander:    "HUNK — Mr. Death",
    codename:     "USS ALPHA-02",
    division:     "Covert Operations",
    founded:      "1990",
    mission:      "High-Value Asset Retrieval & Black Ops",
    clearance:    "LEVEL 5 — ABOVE UBCS",
    deniability:  "FULL — no official records",
    contract:     "Classified / Indefinite",
    total_ops:    104,
    ops_success:  103,
    ops_failed:   1,
    survival_avg: "12% avg",
    last_op:      "RACCOON — T-002",
    board_quote:  "\"Failure is not an option. Survive at all costs.\"",
    board_date:   "1998-09-28",
};

pub const ACTIVE_TARGETS: &[ActiveTarget] = &[
    ActiveTarget { number: 1, name: "William Birkin", redacted: false, priority: TargetPriority::High },
    ActiveTarget { number: 2, name: "████████████", redacted: true, priority: TargetPriority::Medium },
    ActiveTarget { number: 3, name: "James Marcus (Residue)", redacted: false, priority: TargetPriority::Low },
    ActiveTarget { number: 4, name: "███████████████", redacted: true, priority: TargetPriority::High },
    ActiveTarget { number: 5, name: "Alyssa Ashcroft (Journalist)", redacted: false, priority: TargetPriority::Medium },
];

pub const COVERT_OPS_LOG: &[CovertOp] = &[
    CovertOp { date: "1998-05", result: OpResult::Success, summary: "Arklay Lab Secure" },
    CovertOp { date: "1998-07", result: OpResult::Success, summary: "Ecliptic Express Cleanup" },
    CovertOp { date: "1998-09", result: OpResult::Ongoing, summary: "G-Virus Retrieval" },
    CovertOp { date: "1998-10", result: OpResult::Failed, summary: "Containment of Raccoon City" },
    CovertOp { date: "1998-12", result: OpResult::Ongoing, summary: "Rockfort Island Infiltration" },
];
