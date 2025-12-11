## Sonic-Ring-Collector-Tycoon
Joshua Johnson 2025  
  
Idle game themed around collecting rings in the Sonic the Hedgehog universe!

How to start the game:
```rust
cd sonic_ring_tycoon
cargo run --release
```
## About the Code
Code is split between `main.rs` and `lib.rs`.
- `main.rs` contains UI code and logic.
- `lib.rs` contains game logic and maintains game state.

Run unit tests with `cargo test`.

## What Was Built, What Worked/Didn't Work, and Lessons Learned
Sonic Ring Collector Tycoon is an idle clicker game that begins with one button, "Collect Ring!", a counter that starts at zero, and a grayed out multiplier purchase button that becomes available when the player has collected enough rings. Purchasing this upgrade allows the player to collect rings a little bit faster, and future multiplier upgrades are slightly more expensive. After purchasing the first multiplier upgrade, the first auto-collector is made available for purchase. These collectors provide a certain amount of rings every second and can be upgraded to collect more and more. There are three types of collectors: Knuckles, Chili Dog Carts, and Tails' Flying Drones. Finally, the last items for purchase are Chaos Emeralds, which double the number of rings collected for every emerald purchased.

Thanks to progressive improvement over the weeks-long project, adding one feature at a time, each coded feature worked. If I were to do the project again, I would make the ring collection aesthetically pleasing by incrementing it constantly instead of once every second. I'd also add a prestige feature to restart the game with all collection rates doubled. This would give the game an end goal, whereas currently after chaos emeralds there is no end goal. Perhaps a Super Sonic upgrade available after acquiring seven chaos emeralds becomes the end goal and prestige button at the same time.

## Acknowledgements
### AI Use
- ChatGPT used to format and expand ideas for proposal
- ChatGPT used to learn egui and provide basic code framework (e.g., making a GUI window and adding a button)
- Cursor AI used to generate unit tests in `lib.rs`
- GitHub Copilot used periodically to help with pull request code reviews
