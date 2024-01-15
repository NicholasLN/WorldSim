/// Power represents permission within a government: nationally, subnationally, or locally.
/// Powers are actions that can be taken by a government, and are granted to a government by the government above it.
/// Example Powers: declare war, make peace, raise taxes, lower taxes, etc.
/// Powers may be granted to a `Chamber` or a `Role`, as well as a `Division`.
/// When granted to a `Division`, the Division has full power to act upon that power as if it were a government.
///     Divisions may not perform diplomatic actions, however (yet, that's very complicated).
/// When granted to a `Chamber`, the Chamber may enact bills that are within the scope of that power
/// If two or more Chambers have the same power, they must both enact the same bill for it to pass, 
/// unless the enumerated power is granted an exclusive flag.
/// When granted to a `Role`, the Role may act upon that power as if it were a government.
/// For example if the President has the power to declare war, they may do so without the approval of the legislature.
/// If a Role has a power, and a Chamber has the same power, the Chamber's power is ignored.
pub struct Power {
    pub id: u8,
    pub name: String,
}

pub trait PowerBehavior {
    fn execute(&self); // To be implemented. Need to figure out how to do this.
}
pub trait OnPower {
    // Define two behaviors-- a behavior if the power is enacted upon by a Chamber, and a behavior if the power is enacted upon by a Role.
    // If enacted upon by a Chamber, the method will create a bill and add it to the Chamber's list of bills.
    // If enacted upon by a Role, the method will enact the power.
    fn on_chamber(&self, chamber: Chamber); // -> Result<Bill, ()>; this is called when a Chamber enacts a power. 
                                            // Example. If the power is "Declare War", the Chamber will create a bill that declares war 
                                            // after having collected the necessary information from the user. 
                                            ///(thats a ui thing for me to cry about later)
    fn on_role(&self, role: Role); // -> Result<(), ()>; this returns nothing, it just enacts execute().
}
 
// Used to extend the Power struct with additional information for a Chamber or Role.
pub struct EnumeratedPower {
    pub power: Power,

    pub exclusive: bool, // Whether or not this power is exclusive from needing approval from other legislatures.

    /*  +-+-+ Only for chambers +-+-+ */
    pub need_majority: Option<bool>, // 50% + 1
    pub need_supermajority: Option<bool>, // 2/3 or as defined by the ChamberVotingRules
    pub need_quorum: Option<bool>, // Whether or not the Chamber needs a quorum to vote on this power.
                                   // When would they not need a quorum? A power that is only used in emergencies, perhaps?
    pub need_abstain: Option<bool>, // Whether or not this power, if voted upon, may be abstained.
}

pub struct Role { 
    pub id: u8,
    pub name: String,
    pub powers: Vec<Power>,
}

pub struct ChamberPositions {
    pub role: Role,
    pub abstain: bool, // Whether or not this Role may abstain from voting.
    pub weight: f32, // How much a vote is worth. 
                     // If a Role has a weight of 0, they may not vote.
}

pub struct ChamberVotingRules {
    pub quorum: f32,        // Percentage of members that must be present to vote.
    pub majority: f32,      // Percentage of votes needed to pass a bill.
    pub supermajority: f32, // Percentage of votes needed to pass an important bill.
    pub positions: Vec<ChamberPositions>,
}

pub type ChamberCategory = u8; // Ex: "Legislative", "Executive", etc.
                               // Honestly useful for nothing but organization.
                               // Didn't want to make a whole struct for it, though.
pub struct Chamber {
    pub id: u8,
    pub name: String,
    pub category: ChamberCategory,
    pub powers: Vec<EnumeratedPower>,
    pub voting_rules: ChamberVotingRules,
}