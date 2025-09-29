# Hawk-Dove Simulation

A simple Rust implementation of the Chicken game.

## Rules

| Food Pieces | Outcome |
|-------------|---------|
| 4 pieces | Survive + reproduce |
| 3 pieces | Survive + 50% chance to reproduce |
| 2 pieces | Survive (no reproduction) |
| 1 piece | 50% chance of survival |
| 0 pieces | Death |

### Behavioral Interactions

- **Dove meets Dove**: Share equally (2 pieces each) - both survive, neither reproduces
- **Hawk meets Dove**: Hawk takes 3 pieces, Dove gets 1 - Hawk thrives, Dove risks death
- **Hawk meets Hawk**: Fight over food, both get nothing - both die

## Running the Simulation

```bash
cargo run --release
```

## Configuration

Default settings (can be modified in `main.rs`):
- Starting population: 5 Doves, 5 Hawks
- Food spots: 1000
- Iterations: 5000

```rust
Simulation::new()
    .set_creatures(5, 5)
    .set_food(1000)
    .set_iterations(5000)
    .start();
```