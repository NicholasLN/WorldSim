use crate::modules::game::power::Power;

// Used to extend the Power struct with additional information for a Chamber or Role.
pub struct EnumeratedPower {
    pub power: Power,

    pub exclusive: bool, // Whether or not this power is exclusive from needing approval from other legislatures.

    /*  +-+-+ Only for chambers +-+-+ */
    pub need_majority: Option<bool>,
    // 50% + 1
    pub need_supermajority: Option<bool>,
    // 2/3 or as defined by the ChamberVotingRules
    pub need_quorum: Option<bool>,
    // Whether or not the Chamber needs a quorum to vote on this power.
    // When would they not need a quorum? A power that is only used in emergencies, perhaps?
    pub need_abstain: Option<bool>, // Whether or not this power, if voted upon, may be abstained.
}
