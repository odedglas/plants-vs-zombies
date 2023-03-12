use crate::model::AttackEffect;

#[derive(Debug, Default)]
pub struct AttackState {
    pub life: f64,
    pub damage: f64,
    pub attack_enabled: bool,
    pub effect: Option<AttackEffect>,
}

impl AttackState {
    pub fn new(life: f64, damage: f64, effect: Option<AttackEffect>) -> Self {
        AttackState {
            life,
            damage,
            effect,
            attack_enabled: true,
        }
    }

    pub fn get_damage(&self) -> f64 {
        match self.attack_enabled {
            true => self.damage,
            false => 0.0,
        }
    }

    pub fn mutate_damage(&mut self, damage: f64) {
        self.damage += damage
    }

    pub fn take_damage(&mut self, damage: f64) {
        self.life -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.life <= 0.0
    }

    pub fn mute(&mut self, enabled: bool) {
        self.attack_enabled = enabled;
    }
}
