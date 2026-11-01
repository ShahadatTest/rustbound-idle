import { createReadStream, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, resolve } from "node:path";
import { chromium } from "playwright";

const DIST = resolve(process.argv[2] ?? "../dist");
const PORT = 8081;
const ORIGIN = `http://localhost:${PORT}/`;

const TYPES = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".css": "text/css",
  ".wasm": "application/wasm",
  ".webmanifest": "application/manifest+json",
  ".png": "image/png",
};

function serve() {
  const server = createServer((req, res) => {
    const path = new URL(req.url, ORIGIN).pathname;
    const file = join(DIST, path.endsWith("/") ? `${path}index.html` : path);

    if (!statSync(file, { throwIfNoEntry: false })?.isFile()) {
      res.writeHead(404).end();
      return;
    }

    res.writeHead(200, {
      "content-type": TYPES[extname(file)] ?? "application/octet-stream",
    });
    createReadStream(file).pipe(res);
  });

  return new Promise((done) => server.listen(PORT, () => done(server)));
}

let failures = 0;

function check(name, ok) {
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}`);
  if (!ok) failures += 1;
}

const server = await serve();
const browser = await chromium.launch();

try {
  const page = await browser.newPage();
  const gameStarted = () => Boolean(document.querySelector("idle-game")?.game);

  await page.goto(ORIGIN);
  await page.waitForFunction(gameStarted);
  check("the game starts while the server is up", true);

  await page.locator("idle-game").locator('[data-action="attack"]').click();

  await page.waitForFunction(() => navigator.serviceWorker.controller, null, {
    timeout: 10000,
  });
  check("a service worker controls the page", true);

  server.close();
  server.closeAllConnections();

  await page.reload();
  await page.waitForFunction(gameStarted);
  check("the game starts again with the server gone", true);

  const stage = await page.locator("idle-game").locator("[data-stage]").textContent();
  check("the game draws its state", stage.trim() !== "");

  const saved = await page.evaluate(() =>
    localStorage.getItem("rustbound-idle.save"),
  );
  check("the save is still there", saved !== null);
} catch (error) {
  console.error(error.message);
  failures += 1;
} finally {
  await browser.close();
  server.close();
}

process.exit(failures === 0 ? 0 : 1);
