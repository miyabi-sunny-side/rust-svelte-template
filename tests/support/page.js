// What every document of the page tests (tests/page.rs) runs first. A test lays the list the
// page is to show in sessionStorage under `test:items` (JSON); a request for /api/items is then
// answered with it, as the server answers with its own items. Without it the request goes to
// the server.
{
  const fetchOfPage = window.fetch.bind(window);
  window.fetch = (input, init) => {
    const items = sessionStorage.getItem("test:items");
    const asked = new URL(
      input instanceof Request ? input.url : String(input),
      location.href,
    );
    if (items === null || asked.pathname !== "/api/items") {
      return fetchOfPage(input, init);
    }
    return Promise.resolve(
      new Response(items, { headers: { "Content-Type": "application/json" } }),
    );
  };
}
