# Rustbound Idle

A proof of concept for a twelve-part series that builds a Rust idle game one article at a time.

This checkout is the end of **article 9: Give the Game a Memory**.

Versioned JSON saves, restore validation, and browser snapshots with derived display values.

## Run this checkpoint

Install stable Rust with Cargo. The early articles use a terminal demo so readers can run the rules before a browser exists.

```sh
cargo run --locked --example demo
cargo test --locked
```

`Game::save_json()` stores engine state; `Game::from_saved_json()` restores it. `Game::state_json()` adds calculated values for rendering. The demo also checks a save round trip.

## Article checkpoints

Each article is one buildable commit, tagged `article-01` through `article-12`. Later features are introduced in their own article: modules and tests in 8, serialization in 9, WASM in 10, the browser in 11, and deployment in 12.

| # | Article | Reader learns | Game addition |
| --- | --- | --- | --- |
| 1 | Skip Hello World. Let's Make Damage. | `fn`, parameters, return values, basic types, expressions | Three tiny damage, health, and death calculations in `src/main.rs`. |
| 2 | Tiny Functions, Bigger Rules | Function composition, `if`, booleans | An `attack()` calculation composed from the tiny functions, including a defeated-enemy guard. |
| 3 | Give the Game a Shape | `struct`, `impl`, constructors, methods | `Player` and `Enemy` structs with constructors and methods; attacks consume and return the enemy. |
| 4 | Borrowing Without the Lecture | `&`, `&mut`, ownership | Attacks borrow a reusable player and mutate the same enemy through references. |
| 5 | One Click, One Turn | Mutable state and orchestration | `Game::click()` handles damage, defeat, a fixed gold reward, and the next enemy. |
| 6 | Gold, XP, Levels, and Upgrades | Composition, progression state, integer math | XP, levels, scaled enemies and bosses, gold spending, and click upgrades. |
| 7 | Make Time Do the Work | Time as input, deterministic updates | Idle upgrades, `tick(seconds)`, and a shared damage pool that can defeat several enemies. |
| 8 | You Accidentally Built a Game Engine | Modules, visibility, organization, unit tests | The same rules move into six modules with a public `Game` library API, unit tests, and a runnable example. |
| 9 | Give the Game a Memory | Serialization, `serde`, snapshots, boundaries | Versioned JSON saves, restore validation, and browser snapshots with derived display values. |
| 10 | Rust Meets WebAssembly | WASM exports and browser compilation | The existing engine gets a `wasm-bindgen` wrapper exported to JavaScript as `Game`. |
| 11 | Put a Web Component Around It | Native Web Components, JS/WASM calls, events, rendering | The playable `<idle-game>` UI adds rendering, controls, local saves, offline progress, and development servers. |
| 12 | Ship It | Production builds, WASM assets, static hosting, deployment | A release build assembles `dist/`; a Render blueprint and CI prepare the static site for deployment. |

Inspect the sequence or compare adjacent articles:

```sh
git log --reverse --oneline final-stage
git diff article-05 article-06
```

With a clean working tree, check out an article and return to the finished game:

```sh
git switch --detach article-03
cargo run --locked
git switch final-stage
```

Articles 1-7 keep their code in `src/main.rs`. Article 8 moves the demo to `examples/demo.rs` and the rules to `src/{combat,player,enemy,progression,upgrades,game}.rs`. Article 9 adds save boundaries; article 10 adds the browser API in `src/lib.rs`.
