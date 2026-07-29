<script lang="ts">
  type ConnectionState = "checking" | "connected" | "unavailable";

  let connection = $state<ConnectionState>("checking");
  let detail = $state("Opening a line to the Rust service…");

  async function checkConnection(signal?: AbortSignal) {
    connection = "checking";
    detail = "Opening a line to the Rust service…";

    try {
      const response = await fetch("/api/health", { signal });
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const body = (await response.json()) as { status?: string };
      if (body.status !== "ok") {
        throw new Error("Unexpected health response");
      }

      connection = "connected";
      detail = "Rust and Svelte are speaking the same language.";
    } catch (error) {
      if (error instanceof DOMException && error.name === "AbortError") {
        return;
      }

      connection = "unavailable";
      detail = "Start the Rust server, then try the connection again.";
    }
  }

  $effect(() => {
    const controller = new AbortController();
    void checkConnection(controller.signal);
    return () => controller.abort();
  });
</script>

<svelte:head>
  <title>Rust + Svelte Field Kit</title>
</svelte:head>

<main>
  <div class="registration" aria-hidden="true">RS / 01</div>

  <section class="hero" aria-labelledby="page-title">
    <p class="eyebrow">A field-tested starting point</p>
    <h1 id="page-title">Small surface.<br />Solid footing.</h1>
    <p class="lede">
      One Rust process, one Svelte interface, and just enough structure to turn
      a clean clone into a production service.
    </p>
  </section>

  <section class="status-panel" aria-labelledby="connection-title">
    <div class="status-heading">
      <p class="index">01 / CONNECTION</p>
      <span
        class:online={connection === "connected"}
        class:offline={connection === "unavailable"}
      >
        {connection}
      </span>
    </div>

    <div
      class="signal"
      class:online={connection === "connected"}
      aria-hidden="true"
    >
      <span></span><span></span><span></span>
    </div>

    <h2 id="connection-title">Service line</h2>
    <p aria-live="polite">{detail}</p>

    {#if connection === "unavailable"}
      <button type="button" onclick={() => void checkConnection()}
        >Try again</button
      >
    {/if}
  </section>

  <section class="principles" aria-labelledby="principles-title">
    <p class="index" id="principles-title">02 / LOADOUT</p>
    <ul>
      <li><strong>Axum</strong><span>quiet, typed HTTP</span></li>
      <li><strong>Svelte 5</strong><span>direct, reactive UI</span></li>
      <li><strong>One image</strong><span>the same build everywhere</span></li>
    </ul>
  </section>

  <footer>
    <span>READY TO RENAME</span>
    <span>MIT · 2026</span>
  </footer>
</main>
