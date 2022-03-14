pub trait Delta {
    fn set_delta(&mut self, delta: f32);
}

#[derive(Default)]
pub struct IdleDelta {
    pub value: f32,
}

impl Delta for IdleDelta {
    fn set_delta(&mut self, delta: f32) {
        self.value = delta;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameOver {
    Win,
    Lose,
}

#[derive(Default)]
pub struct PhysicsDelta {
    pub value: f32,
}

impl Delta for PhysicsDelta {
    fn set_delta(&mut self, delta: f32) {
        self.value = delta;
    }
}
