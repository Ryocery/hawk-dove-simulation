use rand::Rng;

#[derive(Clone, Copy)]
pub enum CreatureType {
    Dove,
    Hawk,
}

#[derive(Clone)]
pub struct Creature {
    pub creature_type: CreatureType,
    pub food_eaten: u8,
    pub alive: bool,
    pub reproduced: bool,
}

impl Creature {
    pub fn new(creature_type: CreatureType) -> Self {
        Creature { creature_type, food_eaten: 0 , alive: true, reproduced: false}
    }

    pub fn feed(&mut self, food: u8) {
        self.food_eaten = food;
        self.process();
    }

    fn process (&mut self) {
        let mut rng = rand::rng();

        match self.food_eaten {
            0 => self.alive = false,
            1 => self.alive = rng.random_bool(0.5),
            2 => { /* Nothing happens, creature eats enough food to survive */ }
            3 => self.reproduced = rng.random_bool(0.5),
            4 => self.reproduced = true,
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.food_eaten = 0;
        self.reproduced = false;
    }
}