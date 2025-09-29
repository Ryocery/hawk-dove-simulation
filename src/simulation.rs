use crate::creature::{Creature, CreatureType};
use rand::seq::SliceRandom;
use rand::rng;

pub struct Simulation {
    population: Vec<Creature>,
    food_spots: Vec<Vec<Creature>>,
    food: u32,
    iterations: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            population: Vec::new(),
            food_spots: Vec::new(),
            food: 0,
            iterations: 0,
        }
    }

    pub fn set_creatures(mut self, doves: u32, hawks: u32) -> Self {
        for _ in 0..doves {
            self.population.push(Creature::new(CreatureType::Dove));
        }

        for _ in 0..hawks {
            self.population.push(Creature::new(CreatureType::Hawk));
        }

        self
    }

    pub fn set_food(mut self, food: u32) -> Self {
        self.food = food;
        self.food_spots = vec![Vec::new(); food as usize];

        self
    }

    fn clear_food_spots(&mut self) {
        for spot in &mut self.food_spots {
            spot.clear();
        }
    }

    pub fn set_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;

        self
    }

    pub fn iterate(&mut self) {
        let mut rng = rng();
        self.population.shuffle(&mut rng);

        for (index, creature) in self.population.drain(..).enumerate() {
            let spot_index = index % self.food_spots.len();
            self.food_spots[spot_index].push(creature);
        }

        for spot in &mut self.food_spots {
            match spot.len() {
                0 => {},
                1 => spot[0].feed(4),
                2 => Self::process_encounter(spot),
                _ => {
                    for creature in spot.iter_mut() {
                        creature.feed(0);
                    }
                },
            }

            for mut creature in spot.drain(..) {
                if !creature.alive { continue; }

                if creature.reproduced {
                    self.population.push(Creature::new(creature.creature_type.clone()));
                }

                creature.reset();
                self.population.push(creature);
            }
        }

        let dove_count = self.population.iter()
            .filter(|c| matches!(c.creature_type, CreatureType::Dove))
            .count();

        let hawk_count = self.population.iter()
            .filter(|c| matches!(c.creature_type, CreatureType::Hawk))
            .count();

        println!("Population - Doves: {}, Hawks: {}", dove_count, hawk_count)
    }

    fn process_encounter(spot: &mut [Creature]) {
        if let [creature1, creature2] = spot {
            match (&creature1.creature_type, &creature2.creature_type) {
                (CreatureType::Dove, CreatureType::Dove) => {
                    creature1.feed(2);
                    creature2.feed(2);
                },
                (CreatureType::Hawk, CreatureType::Dove) => {
                    creature1.feed(3);
                    creature2.feed(1);
                },
                (CreatureType::Dove, CreatureType::Hawk) => {
                    creature1.feed(1);
                    creature2.feed(3);
                },
                (CreatureType::Hawk, CreatureType::Hawk) => {
                    creature1.feed(0);
                    creature2.feed(0);
                },
            }
        }
    }

    pub fn start(&mut self) {
        for _ in 0..self.iterations {
            self.iterate();
        }
    }
}