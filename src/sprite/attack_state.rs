#[derive(Debug, Default)]
pub struct AttackState {
    pub life: f64,
    pub damage: f64,
    pub attack_enabled: bool,
}

impl AttackState {
    pub fn new(life: f64, damage: f64) -> Self {
        AttackState {
            life,
            damage,
            attack_enabled: true,
        }
    }

    pub fn take_damage(&mut self, damage: f64) {
        self.life -= damage;
    }

    pub fn is_dead(&mut self) -> bool {
        self.life <= 0.0
    }

    pub fn mute(&mut self, enabled: bool) {
        self.attack_enabled = enabled;
    }
}
