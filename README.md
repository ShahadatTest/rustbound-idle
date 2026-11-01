# Rustbound Idle

This is the companion project for the [Rustbound series](https://journeytocode.io/series/rustbound-build-your-first-rust-app/). Each checkpoint is copied from the complete program at the end of its lesson and tested before it is published here.

## Follow one lesson

Clone the repository, then switch to the checkpoint named in the article:

```sh
git clone https://github.com/journey-to-code-org/rustbound-idle.git
cd rustbound-idle
git switch article_9
cargo test
```

The branches `article_1` through `article_15` hold the finished code for each lesson. Matching immutable tags use `article-01` through `article-15`.

| Checkpoint | Lesson |
| --- | --- |
| `article_1` | Skip Hello World. |
| `article_2` | One Attack Is Not a Fight. |
| `article_3` | Three Numbers Are Not a Player. |
| `article_4` | Borrowing Without the Lecture. |
| `article_5` | One Fight Is Not a Game. |
| `article_6` | Winning Is Not Progress. |
| `article_7` | False Is Not an Answer. |
| `article_8` | Idle Is Not a Timer. |
| `article_9` | We Accidentally Built a Game Engine. |
| `article_10` | Closing the Terminal Is Not Game Over. |
| `article_11` | JavaScript Does Not Get the Rules. |
| `article_12` | A Button Is Not a Rule. |
| `article_13` | Localhost Is Not a Launch. |
| `article_14` | A Bookmark Is Not an App. |
| `article_15` | Cheaters Are Going to Cheat. |

## Run the current checkpoint

```sh
cargo test
cargo run
```

Articles 11 through 14 also compile the library for the browser:

```sh
wasm-pack build --target web --out-dir web/pkg
python3 -m http.server 8080 --directory web
```

Article 12 adds a Playwright check. After the server is running:

```sh
cd web
npm install
npm run check
```

Article 13 adds repeatable development, release, and Render build scripts:

```sh
bash ./scripts/dev.sh
bash ./scripts/build.sh
```

At the Article 13 checkpoint, the release script writes the five files that ship to `dist/`. The Render Blueprint runs
`scripts/render-build.sh` and publishes that directory as a static site.

Article 14 turns that release into an installable PWA. It adds the manifest, service worker,
icons, and an offline browser check. Its release folder contains ten files:

```sh
bash ./scripts/build.sh
cd web
npm run check:offline
```
