import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";

import Header from "./Header.svelte";
import { THEME_STORAGE_KEY } from "./theme";

describe("Header theme flow", () => {
  afterEach(() => {
    cleanup();
    window.localStorage.clear();
    delete document.documentElement.dataset.theme;
  });

  it("menu leads with テーマ設定 and choices apply without closing", async () => {
    render(Header);

    await fireEvent.click(screen.getByRole("button", { name: "メニュー" }));
    const menu = screen.getByRole("dialog", { name: "メニュー" });
    const firstItem = menu.querySelector("nav > :first-child");
    expect(firstItem?.textContent?.trim()).toBe("テーマ設定");

    await fireEvent.click(firstItem as HTMLElement);
    const dialog = screen.getByRole("dialog", { name: "テーマ設定" });
    expect(dialog).toBeTruthy();
    expect(screen.getAllByRole("radio")).toHaveLength(3);

    await fireEvent.click(screen.getByRole("radio", { name: "ライト" }));
    expect(document.documentElement.dataset.theme).toBe("light");
    expect(window.localStorage.getItem(THEME_STORAGE_KEY)).toBe("light");
    expect(
      screen
        .getByRole("radio", { name: "ライト" })
        .getAttribute("aria-checked"),
    ).toBe("true");
    expect(screen.getByRole("radiogroup")).toBeTruthy();

    await fireEvent.click(screen.getByRole("radio", { name: "ダーク" }));
    expect(document.documentElement.dataset.theme).toBe("dark");
    expect(window.localStorage.getItem(THEME_STORAGE_KEY)).toBe("dark");
    expect(screen.getByRole("radiogroup")).toBeTruthy();

    await fireEvent.click(screen.getByRole("radio", { name: "自動" }));
    expect(document.documentElement.dataset.theme).toBeUndefined();
    expect(window.localStorage.getItem(THEME_STORAGE_KEY)).toBeNull();
    expect(screen.getByRole("radiogroup")).toBeTruthy();
  });

  it("Escape closes the modal and focus returns to the hamburger", async () => {
    render(Header);
    const hamburger = screen.getByRole("button", { name: "メニュー" });

    await fireEvent.click(hamburger);
    await fireEvent.click(screen.getByRole("button", { name: "テーマ設定" }));
    expect(screen.getByRole("radiogroup")).toBeTruthy();

    await fireEvent.keyDown(window, { key: "Escape" });
    expect(screen.queryByRole("radiogroup")).toBeNull();
    expect(document.activeElement).toBe(hamburger);
  });
});
