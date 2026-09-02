# Rust + Svelte Template

An always-green starting point for a small web service. It combines an
[Axum](https://github.com/tokio-rs/axum) backend with a Svelte 5 and Vite frontend, while keeping
authentication, persistence, and deployment-provider policy out of the template.

In a production build, one Rust process serves both the JSON API and the compiled frontend. The
starter UI is the Sumi-family app shell — an API-backed card list, a detail page behind a small
client-side router, and a light/dark theme system — giving a newly created repository a small
end-to-end baseline to change with confidence. [`DESIGN.md`](DESIGN.md) is the self-contained
design contract behind that shell.

## Prerequisites

- Rust 1.96.0 (the checked-in toolchain file selects it automatically through `rustup`)
- Node.js 24 LTS
- npm (included with Node.js)
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

Terminal 1, from the repository root:

```sh
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

## Verify changes

Run the complete local verification set from the repository root:

```sh
npm --prefix client ci
npm --prefix client run format:check
npm --prefix client run check
npm --prefix client test
npm --prefix client run build
npm --prefix client run lint:design
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release
```

`npm run lint:design` checks the design contract in [`DESIGN.md`](DESIGN.md). Keep that document in
sync when changing the starter interface's visual system or interaction states.

## Production build and smoke test

Build the frontend before the Rust binary because the server reads static assets from
`client/dist` at runtime:

```sh
cd client
npm ci
npm run build
cd ..
cargo build --locked --release
./target/release/rust-svelte-template
```

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
docker run --rm -p 3000:3000 rust-svelte-template
```

Then use the smoke-test requests above against <http://127.0.0.1:3000>. The image sets the bind
address for container networking and exposes port 3000; the application still defaults to loopback
when run directly on the host.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `APP_BIND_ADDR` | `127.0.0.1:3000` | Socket address used by the Rust HTTP listener. Use `0.0.0.0:3000` when the process must accept connections outside its own network namespace. |
| `RUST_LOG` | `info` | Logging filter consumed by `tracing-subscriber`, for example `rust_svelte_template=debug,tower_http=debug`. |

No application secrets, database, authentication provider, or deployment-provider settings are
required. Add those explicitly when the project needs them rather than carrying unused template
configuration.

## Repository structure

```text
.
├── .github/workflows/  # Continuous integration and container release automation
├── client/             # Svelte 5 app, Vite config, tests, and the npm lockfile
├── src/                # Axum router and executable entry point
├── Cargo.toml          # Rust package and dependency configuration
├── DESIGN.md           # Starter UI design contract
├── Dockerfile          # Reproducible multi-stage production image
└── rust-toolchain.toml # Pinned Rust toolchain and components
```

The backend reserves `/api/*` for API routes. Unknown API paths return 404 instead of the frontend.
Other unknown paths fall back to `client/dist/index.html`, allowing client-side routing.

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

Continuous integration checks pushes to `main` and pull requests. Every push to `main` also builds
and publishes the container image to `ghcr.io/<owner>/<repository>`, tagged `sha-<short-sha>` and
`edge`. Pull request runs build the same image but never push it, since forked PRs do not carry
`packages: write`.

The container release workflow is manual by default. Create and push a SemVer milestone tag, then
run **Release container** from the GitHub Actions page with that tag selected as the ref:

```sh
git tag v1.2.3
git push origin v1.2.3
```

Pushing the tag alone does not start the release workflow. To enable automatic releases for tag
pushes, follow the comments in [`.github/workflows/release.yml`](.github/workflows/release.yml).

Release does not rebuild the image. It looks for the `sha-<short-sha>` image that CI already pushed
for the tagged commit and, when found, retags it as `<version>` and `latest` with `docker buildx
imagetools create` — no compilation happens. If that image is missing (CI on `main` never ran for
that commit, failed, or its cache expired), the workflow falls back to building and pushing the image
itself. Manage package visibility and consumer access in the GitHub package settings. Only tag a
commit after its CI checks pass; use a new SemVer tag for corrections instead of moving an existing
release tag.

## License

This template is available under the [MIT License](LICENSE).
