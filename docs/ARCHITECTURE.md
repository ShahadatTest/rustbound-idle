# Architecture

Rustbound Idle has four layers with deliberately narrow jobs.

```text
Rust engine → WebAssembly border → Web Component → service worker
   rules          messages          screen/save       offline files
```

## Rust engine

`src/game.rs`, `player.rs`, `enemy.rs`, `rules.rs`, and `save.rs` own game behavior. They do not know
about HTML, browser storage, wall clocks, installation, or the network.

## WebAssembly border

`src/web.rs` exposes a small `Game` API to JavaScript and turns Rust state and errors into JSON and text.
It does not draw the game.

## Browser shell

`web/idle-game.js` draws the snapshot, converts clicks into engine calls, measures elapsed browser time,
and stores saves in `localStorage`. It asks Rust for every game rule.

## Offline layer

`web/pwa.js`, `sw.js`, and `manifest.webmanifest` install and cache the application files. They do not
own game state or rules.

Read [CONTRIBUTING.md](../CONTRIBUTING.md) before proposing a change that crosses these boundaries.
