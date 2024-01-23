use crate::modules::game::power_behavior::PowerBehavior;

pub struct Power {
    pub id: u8,
    pub name: String,
    pub behavior: Option<PowerBehavior>,
}