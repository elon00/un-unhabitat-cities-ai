use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 23,
        agency_name: "United Nations Human Settlements Programme (UN-Habitat)".to_string(),
        project_name: "UNHabitat UrbanMesh AI".to_string(),
        portfolio: "Sustainable Urbanization & Housing".to_string(),
        strategy: "Soulbound Verifiable Credential -- Informal Settlement Land Titling and Municipal Rights".to_string(),
        target_chain: "Internet Computer (ICP) + Stellar".to_string(),
        is_active: true,
    }
}
