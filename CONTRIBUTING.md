# Contributing to Rustbound Idle

Thank you for being here. You do not need to write Rust to contribute. Ideas, testing,
documentation, design, accessibility feedback, and welcoming other contributors all count.

## Where to start

- Have an idea or question? Open a [Discussion](https://github.com/journey-to-code-org/rustbound-idle/discussions).
- Found a bug? Use the bug-report issue form.
- Want something small? Look for the [`good first issue`](https://github.com/journey-to-code-org/rustbound-idle/labels/good%20first%20issue) label.
- Stuck? Ask publicly in the issue or Discussion. Your question may help the next person.

Please do not open a large pull request without an issue or Discussion first. A short conversation can
prevent two people from solving different versions of the same problem.

## Run it locally

You need Rust, the `wasm32-unknown-unknown` target, `wasm-pack`, Node, and Python 3.

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cd web
npm install
cd ..
bash ./scripts/dev.sh
```

The game will be available at <http://localhost:8080/>.

## Three boundaries that protect the project

1. The Rust engine never learns about the browser. No DOM, `localStorage`, clocks, or service workers.
2. The browser never calculates game rules. Damage, rewards, prices, XP, and enemy strength come from Rust.
3. The service worker only makes application files available offline. It owns no game logic or saves.

If a useful change seems to cross one of these boundaries, open a Discussion. That is a design
conversation, not a rejection.

## Before opening a pull request

On Windows, keep Cargo at two build jobs to avoid exhausting linker memory.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test -j 2 --all-targets
```

If you changed browser code, build and serve the release, then run both browser checks:

```bash
bash ./scripts/build.sh
python3 -m http.server 8080 --directory dist
```

In another terminal:

```bash
cd web
npm install
npx playwright install chromium
node check.mjs http://localhost:8080/
npm run check:offline
```

## Pull requests

Small is welcome. One test, one CSS fix, or one clarified sentence is a real contribution.
Explain what changed, why it helps, and how you checked it. Review may ask for changes; that is a normal
part of making the project better together.

By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
