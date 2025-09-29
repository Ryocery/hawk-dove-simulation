/*
Simulation of food sharing.

Basically there is a set amount of food spots set in the Simulation struct. Each spot has 4 pieces of food.
If a creature takes 4 pieces, they survive and reproduce
If a creature takes 3 peices, they survive and have a 50% chance of reproduction
If a creature takes 2 pieces, they survive but don't reproduce
If a creature takes 1 piece, they have a 50% chance of survival
If a creature takes no pieces, they die

Each iteration a creature picks one random food spot, each food spot has max two slots (only two creatures can go on a food spot max)
Dove - Always shares 2 food pieces if another creature is present, making sure both survive albeit neither reproduce.
Hawk - Takes 3 pieces from Doves, leaving the dove with only 1 piece. If they encounter another Hawk they fight each other for the food waste the food completely, making both die.

Default Settings:
5 Doves and 5 Hawks
1000 food spots
5000 iterations
*/

mod simulation;
mod creature;

use simulation::Simulation;

fn main() {
    Simulation::new()
        .set_creatures(5, 5)
        .set_food(1000)
        .set_iterations(5000)
        .start();
}