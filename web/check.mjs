import { chromium } from "playwright";

const PAGE = process.argv[2] ?? "http://localhost:8080/";
const OTHER_PAGE = new URL("styles.css", PAGE).href;

const browser = await chromium.launch();
const page = await browser.newPage();
const problems = [];

page.on("pageerror", (error) => problems.push(`page error: ${error.message}`));
page.on("console", (message) => {
  if (message.type() === "error") problems.push(`console: ${message.text()}`);
});

const game = page.locator("idle-game");
const text = (selector) => game.locator(selector).textContent();
const number = async (selector) => Number(await text(selector));

await page.goto(PAGE);
await page.waitForFunction(() => Boolean(document.querySelector("idle-game")?.game));

const check = (label, condition) => {
  console.log(`${condition ? "ok  " : "FAIL"} ${label}`);
  if (!condition) problems.push(label);
};

check("a new game starts at stage 1", (await number("[data-stage]")) === 1);
check("the slime has 20 of 20 health", (await text("[data-enemy-health]")) === "20");

for (let i = 0; i < 4; i += 1) {
  await game.locator('[data-action="attack"]').click();
}

check("four clicks clear the first slime", (await number("[data-stage]")) === 2);
check("the player was paid", (await number("[data-gold]")) === 5);

// Leave the game page, which saves on the way out. From a page on the same
// server that is not the game, pretend that save is ten minutes old.
await page.goto(OTHER_PAGE);
await page.evaluate(() => {
  const save = JSON.parse(localStorage.getItem("rustbound-idle.save"));
  save.savedAt -= 10 * 60 * 1000;
  localStorage.setItem("rustbound-idle.save", JSON.stringify(save));
});

await page.goto(PAGE);
await page.waitForFunction(() =>
  document
    .querySelector("idle-game")
    ?.shadowRoot?.querySelector("[data-status]")
    ?.textContent.startsWith("Welcome back"),
);

check("the save was restored and ten minutes applied", (await number("[data-stage]")) >= 2);
check("the status says how long we were gone", (await text("[data-status]")).includes("10 minutes"));

await page.goto(OTHER_PAGE);
await page.evaluate(() => localStorage.setItem("rustbound-idle.save", "banana"));
await page.goto(PAGE);
await page.waitForFunction(() =>
  document
    .querySelector("idle-game")
    ?.shadowRoot?.querySelector("[data-status]")
    ?.textContent.includes("could not be read"),
);

check("a broken save starts a new game", (await number("[data-stage]")) === 1);

await browser.close();

if (problems.length > 0) {
  console.error("\nProblems:\n" + problems.join("\n"));
  process.exit(1);
}

console.log("\nAll checks passed.");
