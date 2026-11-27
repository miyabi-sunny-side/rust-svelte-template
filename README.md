# Rust + Svelte Template

An always-green starting point for a small web service. It combines an
[Axum](https://github.com/tokio-rs/axum) backend with a Svelte 5 and Vite frontend, while keeping
authentication, persistence, and deployment-provider policy out of the template.

In a production build, one Rust binary embeds and serves both the JSON API and the compiled frontend. The
starter UI is the Sumi-family app shell — an API-backed card list, a detail page behind a small
client-side router, and a light/dark theme system — giving a newly created repository a small
end-to-end baseline to change with confidence. [`DESIGN.md`](DESIGN.md) is the self-contained
design contract behind that shell.

## Prerequisites

- Rust 1.96.0 (the checked-in toolchain file selects it automatically through `rustup`)
- Node.js 24 LTS
- npm (included with Node.js)
- Python 3, for [`bin/browser`](bin/browser), which fetches the browser of the page tests
- Linux on x86-64 with the shared libraries Chrome needs, for the page tests
- Docker, optionally, for the container workflow

The only JavaScript lockfile is `client/package-lock.json`. Run npm commands from `client/` and use
`npm ci` for reproducible installs.

## Quick start

Create a repository with GitHub's **Use this template** button, then clone your new repository.
From its root:

```sh
cd client
npm ci
npm run build
cd ..
cargo run --locked
```

Open <http://127.0.0.1:3000>. The Rust server also exposes:

- `GET /healthz` — plain-text liveness response: `ok`
- `GET /api/health` — JSON service response: `{"status":"ok"}`
- `GET /api/items` — demo fixtures backing the starter card list

Stop the service with <kbd>Ctrl</kbd>+<kbd>C</kbd>.

## Development

Use two terminals so Vite can provide hot module replacement while Rust handles the API.

Build the frontend once before compiling Rust (the quick start already does this). Vite then
serves frontend edits immediately; rebuild the frontend and Rust to update the embedded UI.

Terminal 1, from the repository root:

```sh
npm --prefix client ci
npm --prefix client run build
cargo run
```

Terminal 2:

```sh
cd client
npm ci
npm run dev
```

Open <http://127.0.0.1:5173>. Vite proxies `/api` requests to
`http://127.0.0.1:3000`. The Rust server's `/healthz` endpoint remains available directly on port
3000.

## Live search example

[`Home.svelte`](client/src/pages/Home.svelte) demonstrates an editable search field
with a decorative search icon on its left. Typing filters the API-backed list by
item name immediately; clearing restores it, and no matches has its own message.
Copy the `query`/`matches` expression and labelled `.search-field` markup/styles
with the `search` entry from [`Icon.svelte`](client/src/lib/Icon.svelte), then adapt
them to the derived project's own data and `DESIGN.md`. There is no runtime
dependency or automatic synchronization with this template.

## Verify changes

Run the complete local verification set from the repository root:

```sh
npm --prefix client ci
npm --prefix client run format:check
npm --prefix client run check
npm --prefix client test
npm --prefix client run build
npm --prefix client run lint:design
python3 -B -m unittest discover -s bin -p 'test_*.py'
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release
```

`cargo test` includes the browser tests of the page (see [Page tests](#page-tests)); the first run
downloads their browser. Then run the [production smoke test](#production-build-and-smoke-test)
against that release binary. Run this set before every commit; GitHub Actions does not run these
checks.

`npm run lint:design` checks the design contract in [`DESIGN.md`](DESIGN.md). Keep that document in
sync when changing the starter interface's visual system or interaction states.

## Page tests

The page is tested in two layers. Put a test in the browser layer only when it needs a browser's
engine: layout, sizes, overlap, scrolling, actual colors, canvas drawing, real playback, or real
input. Everything else (text, which state shows, forms, requests after an action, roles and names,
key handling) belongs in jsdom.

- `npm --prefix client test` runs the Vitest + jsdom tests that sit next to the components as
  `client/src/**/*.test.ts`. Each test answers the API by replacing `fetch`.
- `cargo test --test page` runs [`tests/page.rs`](tests/page.rs) against the real binary in
  Chrome for Testing's `chrome-headless-shell`. One browser runs the tests one after another,
  driven through its matching `chromedriver` by W3C WebDriver commands sent over HTTP from Rust.
  The server and chromedriver listen on ports chosen by the OS, so runs of several repositories do
  not collide. [`tests/support/page.js`](tests/support/page.js) answers `/api/items` with the
  list a test lays out. Add words after `--` to run only the tests whose titles contain them.

[`bin/browser`](bin/browser) pins the browser version and the SHA-256 of both downloads. On first
use it downloads them into `${XDG_CACHE_HOME:-~/.cache}/chrome-for-testing/<version>/`, which
repositories pinning the same version share, and later runs reuse that directory without network
access.

### List layout regression checks

`client/src/global.sass` owns the `.content` spacing and the opt-in `.content-wide` /
`.table-scroll` recipes for dense tables. Copy the needed styles and the page tests into derived
products, then adapt columns and measurements to their own DESIGN.md. Cards and table rows scroll
with the document; wide tables scroll horizontally locally.

The browser tests measure content position, visible rows, overflow and wheel behavior of the Home
cards and of a dense table at four sizes in both themes, horizontal scrolling of the table by key
in narrow windows, and a long name and keyboard focus at 320px. The jsdom tests cover search,
loading, empty, failure recovery and the no-match message.

The Home measurements include its app header and search field. The table test replaces the page
with a table under the same styles; it has neither, so its top-position budget covers only content
padding and column headings. Derived products must measure their complete screen before setting a
budget.

## Production build and smoke test

Build the frontend before the Rust binary. Compilation embeds `client/dist` and fails with a
build instruction if `client/dist/index.html` is missing. Frontend rebuilds are tracked by Cargo;
recompile Rust after rebuilding the UI:

```sh
cd client
npm ci
npm run build
cd ..
cargo build --locked --release
binary_dir=$(mktemp -d)
cp target/release/rust-svelte-template "$binary_dir/"
cd "$binary_dir"
./rust-svelte-template
```

The executable runs from this isolated directory without any UI files. Only the binary needs to
be deployed; there is no runtime static-directory setting.

In another terminal:

```sh
curl --fail http://127.0.0.1:3000/healthz
curl --fail http://127.0.0.1:3000/api/health
curl --fail http://127.0.0.1:3000/
curl --fail http://127.0.0.1:3000/projects/example
```

The first request prints `ok`, the second returns `{"status":"ok"}`, and the third returns the
compiled app HTML. The final request proves that a client-side deep link also returns the app with a
successful HTTP status.

## Docker

Build and run the same frontend-plus-backend service in a non-root container:

```sh
docker build -t rust-svelte-template .
docker run --rm -p 127.0.0.1:3000:3000 rust-svelte-template
```

The build embeds the frontend after caching Rust dependencies. The runtime image contains only the
application binary on its base image; no separate UI directory is copied.

Then use the smoke-test requests above against <http://127.0.0.1:3000>. The image defaults to
port 3000. To change the container port, use `-e PORT=3100 -p 127.0.0.1:3100:3100`
and send requests to <http://127.0.0.1:3100>.

## Environment variables

Both application settings are optional and are read at startup.

| Variable | Default when unset | Purpose / invalid values |
| --- | --- | --- |
| `PORT` | `3000` | Decimal TCP port from `1` to `65535`. Empty, non-Unicode, signed, whitespace-padded, nonnumeric or out-of-range values fail startup with a `PORT` error. |
| `LOG_LEVEL` | `info` | Logging verbosity: exactly `off`, `error`, `warn`, `info`, `debug`, or `trace`. Empty, non-Unicode or invalid values (including uppercase and module filters) use `info`. |

The readers are [`src/port.rs`](src/port.rs) and [`src/logging.rs`](src/logging.rs).
Build-time Cargo/CI variables are not runtime application settings.

The listener uses `0.0.0.0:${PORT}` in native and container runs, so native runs
accept connections on all IPv4 interfaces. `APP_BIND_ADDR` is no longer read.
Use `PORT=3100 cargo run --locked` to serve a different port. Deployment owns
exposure through container publishing, Tailscale, or a proxy.

The template uses the shared `LOG_LEVEL` contract and ignores the former `RUST_LOG`.
Other services may still use their own logging configuration; consult their README.

No application secrets, database, authentication provider, or deployment-provider settings are
required. Add those explicitly when the project needs them rather than carrying unused template
configuration.

## Repository structure

```text
.
├── .github/workflows/  # Container release automation
├── bin/                # Fetches the pinned browser for the page tests
├── client/             # Svelte 5 app, Vite config, jsdom tests, and the npm lockfile
├── src/                # Axum router and executable entry point
├── tests/              # Rust integration tests, including the page tests in a browser
├── Cargo.toml          # Rust package and dependency configuration
├── DESIGN.md           # Starter UI design contract
├── Dockerfile          # Reproducible multi-stage production image
└── rust-toolchain.toml # Pinned Rust toolchain and components
```

The backend reserves `/api/*` for API routes. Unknown API paths return 404 instead of the frontend.
Other unknown paths fall back to the embedded `index.html`, allowing client-side routing.

## Rename the template

Choose the final Rust crate name before adding product code. Rust package names use hyphens, while
references in Rust source and log targets use underscores.

1. Replace `rust-svelte-template` in `Cargo.toml`, `Cargo.lock`, container metadata, and workflow
   configuration.
2. Replace `rust_svelte_template` in Rust source, commands, and logging examples.
3. Replace `rust-svelte-template-client` in `client/package.json` and `client/package-lock.json`.
4. Replace the starter title, description, accent color, and theme storage key
   (`rust-svelte-template:theme`) in `README.md`, `DESIGN.md`, `client/index.html`, and
   `client/src/`.
5. Review image names, badges, repository links, and package references for the new GitHub owner and
   repository.
6. Confirm that no old identifiers remain, then run the full verification set:

```sh
rg -n --hidden --glob '!.git/**' \
  'rust-svelte-template|rust_svelte_template|Rust \+ Svelte'
```

After renaming the Rust package, refresh `Cargo.lock` with `cargo check` before committing it.

## GitHub template and releases

Repository contents cannot enable GitHub's template flag. A repository administrator must open
**Settings → General**, enable **Template repository**, and verify that the **Use this template**
button appears on the repository page.

GitHub Actions runs only the release workflow. Formatting, linting, tests and the smoke test run
locally in the [verification set](#verify-changes) before each commit; pushes and pull requests
trigger no workflow.

This repository is a template, not a released product. Its container release workflow is manual by
default, and pushing a tag alone does not publish anything. The following release setup is for a
**derived product repository**.

For automatic product releases, uncomment the `push` / `tags` trigger in
[`.github/workflows/release.yml`](.github/workflows/release.yml). Leave `workflow_dispatch` in place
to allow manual runs with a `v`-prefixed tag selected as the ref. In the derived repository, update
`package.version` in `Cargo.toml`, run `cargo check` to refresh `Cargo.lock`, and commit both files.
After that commit passes the local verification set, create and push its matching SemVer tag (for example, package
version `1.2.3`):

```sh
git tag v1.2.3
git push origin v1.2.3
```

If you keep the manual default, select that tag when running **Release container** from the GitHub
Actions page. Use a new SemVer tag for corrections instead of moving an existing release tag.

Each release first verifies that the tag matches `Cargo.toml`'s package version, then builds the
tagged source using its actual `Cargo.toml` and `Cargo.lock`, pushes
`ghcr.io/<owner>/<repository>:<version>` and `latest`, and creates a GitHub Release with generated
notes and the published image reference (`ghcr.io/<owner>/<repository>:X.Y.Z@sha256:…`). It does
not depend on a previously published image. Re-running the workflow for a tag that already has a
GitHub Release leaves that Release as it is.

The release workflow imports and exports BuildKit's registry cache at
`ghcr.io/<owner>/<repository>:build-cache` with `mode=max`, retaining the existing Dockerfile's
cargo-chef dependency layers and frontend build layers between releases. No cache is required for
the first release; a missing cache causes a normal build. The workflow uses `GITHUB_TOKEN` with
`packages: write` for the image and cache, and `contents: write` for the GitHub Release. In a derived
repository, enable GitHub Actions and ensure its token can write to the GHCR package; if reusing an
existing package, grant that repository access in the package settings. Manage package visibility
and consumer access there too. If you change the image path, update both registry cache references
alongside the metadata image name in the release workflow.

The release profile uses `opt-level = 3`, `lto = false`, `codegen-units = 16`, and `strip = true` to
reduce build wait time while retaining optimized, stripped binaries. Derived products can measure
their own runtime requirements before choosing a different tradeoff.

## License

This template is available under the [MIT License](LICENSE).
