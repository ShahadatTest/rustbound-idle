import init, { Game } from "./pkg/rustbound_idle.js";

const SAVE_KEY = "rustbound-idle.save.v1";
const MAX_OFFLINE_SECONDS = 60 * 60 * 4;
const TICK_MS = 1000;
const SAVE_EVERY_MS = 5000;

let wasmReady;

function loadWasm() {
  wasmReady ??= init();
  return wasmReady;
}

function formatNumber(value) {
  return new Intl.NumberFormat(undefined, { maximumFractionDigits: 0 }).format(value);
}

function clampPercent(value, max) {
  if (!max) return 0;
  return Math.max(0, Math.min(100, (value / max) * 100));
}

class IdleGame extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.game = null;
    this.timer = null;
    this.lastSaveAt = 0;
    this.statusTimer = null;
    this.lastTickAt = 0;

    this.handleVisibilityChange = this.handleVisibilityChange.bind(this);
    this.handleBeforeUnload = this.handleBeforeUnload.bind(this);
  }

  async connectedCallback() {
    this.renderShell();

    try {
      await loadWasm();
      this.restoreGame();
      this.bindEvents();
      this.renderState();
      this.startLoop();
      document.addEventListener("visibilitychange", this.handleVisibilityChange);
      window.addEventListener("beforeunload", this.handleBeforeUnload);
    } catch (error) {
      console.error(error);
      this.showFatalError("The WebAssembly game engine could not start.");
    }
  }

  disconnectedCallback() {
    this.stopLoop();
    this.saveGame();
    document.removeEventListener("visibilitychange", this.handleVisibilityChange);
    window.removeEventListener("beforeunload", this.handleBeforeUnload);
  }

  renderShell() {
    this.shadowRoot.innerHTML = `
      <link rel="stylesheet" href="./styles.css" />
      <section class="app" aria-labelledby="game-title">
        <header class="hero">
          <p class="eyebrow">Rust + WebAssembly + Web Components</p>
          <div class="hero-row">
            <div>
              <h1 id="game-title">Rustbound Idle</h1>
              <p class="lede">Tiny functions. Bigger systems. One very persistent monster problem.</p>
            </div>
            <button class="ghost" data-action="reset" type="button">Reset save</button>
          </div>
        </header>

        <p class="status" data-status role="status" aria-live="polite"></p>

        <section class="stats" aria-label="Player stats">
          <article><span>Level</span><strong data-level>1</strong></article>
          <article><span>Gold</span><strong data-gold>0</strong></article>
          <article><span>Click</span><strong><span data-click-damage>1</span> dmg</strong></article>
          <article><span>Idle</span><strong><span data-idle-dps>0</span> DPS</strong></article>
        </section>

        <section class="panel enemy-panel" aria-labelledby="enemy-name">
          <div class="stage-row">
            <p>Stage <strong data-stage>1</strong></p>
            <span class="boss-badge" data-boss hidden>Boss</span>
          </div>
          <div class="enemy-mark" aria-hidden="true">⚙</div>
          <h2 id="enemy-name" data-enemy-name>Moss Slime</h2>
          <p class="health-label">
            <span data-enemy-health>0</span> / <span data-enemy-max-health>0</span> HP
          </p>
          <div class="meter" role="progressbar" aria-label="Enemy health" data-health-meter>
            <span data-health-bar></span>
          </div>
          <button class="attack" data-action="attack" type="button">
            Attack
            <small>+<span data-attack-value>1</span> damage</small>
          </button>
          <p class="reward">Reward: <strong><span data-reward-gold>0</span> gold</strong> · <strong><span data-reward-xp>0</span> XP</strong></p>
        </section>

        <section class="two-column">
          <article class="panel progression" aria-labelledby="progress-title">
            <h2 id="progress-title">Progress</h2>
            <div class="row"><span>XP</span><strong><span data-xp>0</span> / <span data-xp-next>20</span></strong></div>
            <div class="meter xp-meter" role="progressbar" aria-label="Experience" data-xp-meter>
              <span data-xp-bar></span>
            </div>
            <div class="row"><span>Enemies defeated</span><strong data-kills>0</strong></div>
            <p class="hint">Every tenth stage is a boss. Defeat enemies, earn gold, and make the machine stronger.</p>
          </article>

          <article class="panel upgrades" aria-labelledby="upgrade-title">
            <h2 id="upgrade-title">Upgrades</h2>
            <button class="upgrade" data-action="buy-click" type="button">
              <span>
                <strong>Sharper clicks</strong>
                <small>+2 click damage · Lv. <span data-click-level>0</span></small>
              </span>
              <b><span data-click-cost>10</span>g</b>
            </button>
            <button class="upgrade" data-action="buy-idle" type="button">
              <span>
                <strong>Clockwork helper</strong>
                <small>Deals damage every second · Lv. <span data-idle-level>0</span></small>
              </span>
              <b><span data-idle-cost>25</span>g</b>
            </button>
          </article>
        </section>

        <footer>
          <span>Game rules: Rust</span>
          <span>Runtime: WebAssembly</span>
          <span>UI: native Web Component</span>
          <span data-save-state>Save ready</span>
        </footer>
      </section>
    `;
  }

  bindEvents() {
    this.shadowRoot.querySelector('[data-action="attack"]').addEventListener("click", () => {
      this.game.click();
      this.afterAction();
    });

    this.shadowRoot.querySelector('[data-action="buy-click"]').addEventListener("click", () => {
      const bought = this.game.buyClickUpgrade();
      this.afterAction();
      this.flashStatus(bought ? "Click damage upgraded." : "Not enough gold yet.");
    });

    this.shadowRoot.querySelector('[data-action="buy-idle"]').addEventListener("click", () => {
      const bought = this.game.buyIdleUpgrade();
      this.afterAction();
      this.flashStatus(bought ? "Your clockwork helper got stronger." : "Not enough gold yet.");
    });

    this.shadowRoot.querySelector('[data-action="reset"]').addEventListener("click", () => {
      const confirmed = window.confirm("Reset Rustbound Idle and erase this local save?");
      if (!confirmed) return;

      localStorage.removeItem(SAVE_KEY);
      this.game?.free?.();
      this.game = new Game();
      this.saveGame();
      this.renderState();
      this.flashStatus("New game started.");
    });
  }

  restoreGame() {
    const raw = localStorage.getItem(SAVE_KEY);
    if (!raw) {
      this.game = new Game();
      return;
    }

    try {
      const save = JSON.parse(raw);
      const gameJson = typeof save.game === "string" ? save.game : JSON.stringify(save.game);
      this.game = Game.fromJson(gameJson);

      const savedAt = Number(save.savedAt) || Date.now();
      const elapsedSeconds = Math.max(0, Math.floor((Date.now() - savedAt) / 1000));
      const offlineSeconds = Math.min(elapsedSeconds, MAX_OFFLINE_SECONDS);

      if (offlineSeconds > 0) {
        this.game.tick(offlineSeconds);
        this.flashStatus(`Welcome back — ${this.describeDuration(offlineSeconds)} of idle progress applied.`);
      }
    } catch (error) {
      console.warn("Could not restore save; starting fresh.", error);
      localStorage.removeItem(SAVE_KEY);
      this.game = new Game();
      this.flashStatus("The old save could not be read, so a new game was started.");
    }
  }

  startLoop() {
    this.stopLoop();
    this.lastTickAt = Date.now();

    this.timer = window.setInterval(() => {
      const now = Date.now();
      const elapsedSeconds = Math.floor((now - this.lastTickAt) / 1000);
      if (elapsedSeconds < 1) return;

      this.lastTickAt = now;
      this.game.tick(Math.min(elapsedSeconds, MAX_OFFLINE_SECONDS));
      this.renderState();

      if (Date.now() - this.lastSaveAt >= SAVE_EVERY_MS) {
        this.saveGame();
      }
    }, TICK_MS);
  }

  stopLoop() {
    if (this.timer) {
      window.clearInterval(this.timer);
      this.timer = null;
    }
  }

  afterAction() {
    this.renderState();
    this.saveGame();
  }

  saveGame() {
    if (!this.game) return;

    try {
      localStorage.setItem(
        SAVE_KEY,
        JSON.stringify({
          savedAt: Date.now(),
          game: JSON.parse(this.game.saveJson()),
        }),
      );
      this.lastSaveAt = Date.now();
      const saveState = this.shadowRoot.querySelector("[data-save-state]");
      if (saveState) saveState.textContent = "Saved locally";
    } catch (error) {
      console.warn("Could not save game.", error);
      this.flashStatus("Saving is unavailable in this browser session.");
    }
  }

  renderState() {
    const state = JSON.parse(this.game.stateJson());

    this.setText("[data-level]", state.player.level);
    this.setText("[data-gold]", formatNumber(state.player.gold));
    this.setText("[data-click-damage]", formatNumber(state.clickDamage));
    this.setText("[data-idle-dps]", formatNumber(state.idleDps));
    this.setText("[data-stage]", formatNumber(state.stage));
    this.setText("[data-enemy-name]", state.enemy.name);
    this.setText("[data-enemy-health]", formatNumber(state.enemy.health));
    this.setText("[data-enemy-max-health]", formatNumber(state.enemy.maxHealth));
    this.setText("[data-attack-value]", formatNumber(state.clickDamage));
    this.setText("[data-reward-gold]", formatNumber(state.enemy.goldReward));
    this.setText("[data-reward-xp]", formatNumber(state.enemy.xpReward));
    this.setText("[data-xp]", formatNumber(state.player.xp));
    this.setText("[data-xp-next]", formatNumber(state.xpToNext));
    this.setText("[data-kills]", formatNumber(state.player.totalKills));
    this.setText("[data-click-level]", state.player.clickUpgradeLevel);
    this.setText("[data-idle-level]", state.player.idleUpgradeLevel);
    this.setText("[data-click-cost]", formatNumber(state.clickUpgradeCost));
    this.setText("[data-idle-cost]", formatNumber(state.idleUpgradeCost));

    const boss = this.shadowRoot.querySelector("[data-boss]");
    boss.hidden = !state.enemy.isBoss;

    const healthPercent = clampPercent(state.enemy.health, state.enemy.maxHealth);
    const healthBar = this.shadowRoot.querySelector("[data-health-bar]");
    healthBar.style.width = `${healthPercent}%`;
    this.setProgress("[data-health-meter]", state.enemy.health, state.enemy.maxHealth);

    const xpPercent = clampPercent(state.player.xp, state.xpToNext);
    const xpBar = this.shadowRoot.querySelector("[data-xp-bar]");
    xpBar.style.width = `${xpPercent}%`;
    this.setProgress("[data-xp-meter]", state.player.xp, state.xpToNext);

    const clickButton = this.shadowRoot.querySelector('[data-action="buy-click"]');
    const idleButton = this.shadowRoot.querySelector('[data-action="buy-idle"]');
    clickButton.disabled = state.player.gold < state.clickUpgradeCost;
    idleButton.disabled = state.player.gold < state.idleUpgradeCost;
  }

  setText(selector, value) {
    this.shadowRoot.querySelector(selector).textContent = String(value);
  }

  setProgress(selector, value, max) {
    const meter = this.shadowRoot.querySelector(selector);
    meter.setAttribute("aria-valuemin", "0");
    meter.setAttribute("aria-valuemax", String(max));
    meter.setAttribute("aria-valuenow", String(value));
  }

  flashStatus(message) {
    const status = this.shadowRoot.querySelector("[data-status]");
    if (!status) return;

    window.clearTimeout(this.statusTimer);
    status.textContent = message;
    status.classList.add("visible");
    this.statusTimer = window.setTimeout(() => status.classList.remove("visible"), 5000);
  }

  describeDuration(seconds) {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return minutes ? `${hours}h ${minutes}m` : `${hours}h`;
  }

  handleVisibilityChange() {
    if (document.visibilityState === "hidden") {
      this.saveGame();
    }
  }

  handleBeforeUnload() {
    this.saveGame();
  }

  showFatalError(message) {
    const status = this.shadowRoot.querySelector("[data-status]");
    status.textContent = message;
    status.classList.add("visible", "fatal");
  }
}

customElements.define("idle-game", IdleGame);
