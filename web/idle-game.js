import init, { Game } from "./pkg/rustbound_idle.js";

const SAVE_KEY = "rustbound-idle.save";
const SAVE_EVERY_MS = 5000;
const MAX_AWAY_SECONDS = 8 * 60 * 60;

class IdleGame extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.game = null;
    this.timer = null;
    this.lastTickAt = 0;
    this.lastSaveAt = 0;
  }

  async connectedCallback() {
    this.renderShell();

    await init();

    this.restoreGame();
    this.bindEvents();
    this.renderState();
    this.startClock();

    window.addEventListener("beforeunload", () => this.saveGame());
  }

  renderShell() {
    this.shadowRoot.innerHTML = `
      <link rel="stylesheet" href="./styles.css" />

      <section class="game">
        <h1>Rustbound Idle</h1>
        <p class="status" data-status role="status" aria-live="polite"></p>

        <section class="stats">
          <p>Stage <strong data-stage>1</strong></p>
          <p>Level <strong data-level>1</strong></p>
          <p>Gold <strong data-gold>0</strong></p>
        </section>

        <section class="enemy">
          <h2 data-enemy-name>Enemy</h2>
          <p><span data-enemy-health>0</span> / <span data-enemy-max-health>0</span> HP</p>
          <div class="meter"><span data-health-bar></span></div>
          <button data-action="attack" type="button">
            Attack for <span data-click-damage>0</span>
          </button>
        </section>

        <section class="progress">
          <p>XP <span data-xp>0</span> / <span data-xp-needed>0</span></p>
          <div class="meter xp"><span data-xp-bar></span></div>
        </section>

        <section class="upgrades">
          <button data-action="buy-click" type="button">
            Sharper clicks (level <span data-click-upgrade-level>0</span>)
            for <span data-click-upgrade-cost>0</span> gold
          </button>
        </section>

        <button class="reset" data-action="reset" type="button">Reset save</button>
      </section>
    `;
  }

  bindEvents() {
    this.on("attack", () => {
      this.game.click();
      this.afterAction();
    });

    this.on("buy-click", () => {
      const outcome = this.game.buyClickUpgrade();
      this.afterAction();
      this.showStatus(
        outcome === "purchased" ? "Click damage upgraded." : "Not enough gold yet.",
      );
    });

    this.on("reset", () => {
      localStorage.removeItem(SAVE_KEY);
      this.game.free();
      this.game = new Game();
      this.renderState();
      this.showStatus("New game started.");
    });
  }

  afterAction() {
    this.renderState();
    this.saveGame();
  }

  on(action, handler) {
    this.shadowRoot
      .querySelector(`[data-action="${action}"]`)
      .addEventListener("click", handler);
  }

  renderState() {
    const state = JSON.parse(this.game.stateJson());

    this.setText("[data-stage]", state.stage);
    this.setText("[data-level]", state.level);
    this.setText("[data-gold]", state.gold);
    this.setText("[data-enemy-name]", state.enemyName);
    this.setText("[data-enemy-health]", state.enemyHealth);
    this.setText("[data-enemy-max-health]", state.enemyMaxHealth);
    this.setText("[data-click-damage]", state.clickDamage);
    this.setText("[data-xp]", state.xp);
    this.setText("[data-xp-needed]", state.xpNeeded);
    this.setText("[data-click-upgrade-level]", state.clickUpgradeLevel);
    this.setText("[data-click-upgrade-cost]", state.clickUpgradeCost);

    this.setMeter("[data-health-bar]", state.enemyHealth, state.enemyMaxHealth);
    this.setMeter("[data-xp-bar]", state.xp, state.xpNeeded);

    this.shadowRoot.querySelector('[data-action="buy-click"]').disabled =
      state.gold < state.clickUpgradeCost;
  }

  setText(selector, value) {
    this.shadowRoot.querySelector(selector).textContent = String(value);
  }

  setMeter(selector, value, max) {
    const percent = max === 0 ? 0 : Math.min(100, (value / max) * 100);
    this.shadowRoot.querySelector(selector).style.width = `${percent}%`;
  }

  showStatus(message) {
    this.shadowRoot.querySelector("[data-status]").textContent = message;
  }

  startClock() {
    this.lastTickAt = Date.now();

    this.timer = setInterval(() => {
      const now = Date.now();
      const elapsedSeconds = Math.floor((now - this.lastTickAt) / 1000);

      if (elapsedSeconds < 1) return;

      this.lastTickAt += elapsedSeconds * 1000;
      this.game.tick(elapsedSeconds);
      this.renderState();

      if (now - this.lastSaveAt >= SAVE_EVERY_MS) {
        this.saveGame();
      }
    }, 1000);
  }

  saveGame() {
    localStorage.setItem(
      SAVE_KEY,
      JSON.stringify({ savedAt: Date.now(), game: this.game.saveJson() }),
    );
    this.lastSaveAt = Date.now();
  }

  restoreGame() {
    const raw = localStorage.getItem(SAVE_KEY);

    if (raw === null) {
      this.game = new Game();
      return;
    }

    try {
      const save = JSON.parse(raw);

      this.game = Game.fromJson(save.game);

      const awaySeconds = Math.floor((Date.now() - save.savedAt) / 1000);
      const appliedSeconds = Math.max(0, Math.min(awaySeconds, MAX_AWAY_SECONDS));

      this.game.tick(appliedSeconds);

      if (appliedSeconds > 0) {
        this.showStatus(`Welcome back. ${describe(appliedSeconds)} of progress applied.`);
      }
    } catch (error) {
      console.warn("Could not restore the save:", error);
      localStorage.removeItem(SAVE_KEY);
      this.game = new Game();
      this.showStatus("The old save could not be read, so a new game was started.");
    }
  }
}

function describe(seconds) {
  if (seconds < 60) return plural(seconds, "second");
  if (seconds < 3600) return plural(Math.floor(seconds / 60), "minute");
  return plural(Math.floor(seconds / 3600), "hour");
}

function plural(count, unit) {
  return count === 1 ? `1 ${unit}` : `${count} ${unit}s`;
}

customElements.define("idle-game", IdleGame);
