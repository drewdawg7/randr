
pub struct AttackResult {
    pub attacker: String,
    pub defender: String,
    pub damage_to_target: i32,
    pub target_health_before: i32,
    pub target_health_after: i32,
    pub target_died: bool,
}
