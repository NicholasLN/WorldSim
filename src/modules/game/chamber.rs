use crate::modules::game::enumerated_power::EnumeratedPower;
use crate::modules::game::role::Role;

pub struct ChamberPositions {
    pub role: Role,
    pub abstain: bool,
    // Whether or not this Role may abstain from voting.
    pub weight: f32, // How much a vote is worth.
    // If a Role has a weight of 0, they may not vote.
}

pub struct ChamberVotingRules {
    pub quorum: f32,
    // Percentage of members that must be present to vote.
    pub majority: f32,
    // Percentage of votes needed to pass a bill.
    pub supermajority: f32,
    // Percentage of votes needed to pass an important bill.
    pub positions: Vec<ChamberPositions>,
}

pub type ChamberCategory = u8;

// Ex: "Legislative", "Executive", etc.
// Honestly useful for nothing but organization.
// Didn't want to make a whole struct for it, though.
pub struct Chamber {
    pub id: u8,
    pub name: String,
    pub category: ChamberCategory,
    pub powers: Vec<EnumeratedPower>,
    pub voting_rules: ChamberVotingRules,
}