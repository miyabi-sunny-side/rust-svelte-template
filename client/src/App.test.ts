import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import App from "./App.svelte";

describe("App", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("shows a connected service and can retry after a failure", async () => {
    const fetchMock = vi
      .fn<typeof fetch>()
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: "ok" }), { status: 200 }),
      )
      .mockRejectedValueOnce(new Error("offline"))
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: "ok" }), { status: 200 }),
      );
    vi.stubGlobal("fetch", fetchMock);

    const view = render(App);

    await waitFor(() => expect(screen.getByText("connected")).toBeTruthy());
    expect(
      screen.getByText("Rust and Svelte are speaking the same language."),
    ).toBeTruthy();

    view.unmount();
    render(App);
    await waitFor(() => expect(screen.getByText("unavailable")).toBeTruthy());

    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    await waitFor(() => expect(screen.getByText("connected")).toBeTruthy());
    expect(fetchMock).toHaveBeenCalledTimes(3);
  });
});
