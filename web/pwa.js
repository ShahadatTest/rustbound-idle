if ("serviceWorker" in navigator) {
  window.addEventListener("load", async () => {
    try {
      const registration = await navigator.serviceWorker.register("./sw.js", {
        scope: "./",
      });

      console.info("Rustbound Idle service worker ready:", registration.scope);
    } catch (error) {
      console.warn("Rustbound Idle could not enable offline mode.", error);
    }
  });
}
