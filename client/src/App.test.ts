import { cleanup, render, screen, waitFor } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import App from "./App.svelte";

const ITEM = {
  id: "sumi",
  name: "Sumi ダークテーマ",
  summary: "概要テキスト",
  status: "stable",
  updated_at: "2026-08-01",
  body: "本文テキスト",
};

describe("App", () => {
  afterEach(() => {
    cleanup();
    vi.unstubAllGlobals();
    window.history.replaceState(null, "", "/");
  });

  it("keeps the invariant header and restores a deep detail URL", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn<typeof fetch>().mockImplementation((input) => {
        const payload = String(input) === "/api/items" ? [ITEM] : ITEM;
        return Promise.resolve(
          new Response(JSON.stringify(payload), { status: 200 }),
        );
      }),
    );
    window.history.replaceState(null, "", "/items/sumi");

    render(App);

    const header = screen.getByRole("banner");
    expect(header.textContent).toContain("rust-svelte-template");
    expect(screen.getByRole("button", { name: "メニュー" })).toBeTruthy();
    expect(header.querySelector("a")).toBeNull();

    await waitFor(() =>
      expect(screen.getByRole("heading", { name: ITEM.name })).toBeTruthy(),
    );
  });
});
