//! The page of the real binary in a real browser, for what only a browser's engine shows:
//! where things lie and how big they are, what overflows, scrolling, and the keys as a person
//! presses them. The rest of the page is tested in jsdom (client/src/**/*.test.ts).
//!
//! One browser and one server: the tests run one after another, in the order they are
//! written, each on a page of no state in a window of 1280 by 720 in the light. The list the
//! page shows is laid by the test (tests/support/page.js). `cargo test --test page --
//! <words>` runs the tests whose titles hold any of the words; `--skip <words>` leaves those
//! out.
#![allow(clippy::unwrap_used)] // helpers outside #[test] fns
use serde_json::{Value, json};
use std::{future::Future, pin::Pin, sync::Arc, time::Instant};

use browser::Page;
use server::Server;

#[path = "support/browser.rs"]
mod browser;
#[path = "support/server.rs"]
mod server;

type Run = Pin<Box<dyn Future<Output = ()> + Send>>;
/// A test: its title, and what it does on the page of the server.
type Test = (&'static str, fn(Arc<Page>, Arc<Server>) -> Run);

macro_rules! test {
    ($title:literal, $test:ident) => {
        ($title, |page, server| Box::pin($test(page, server)))
    };
}

/// The tests, in the order they run.
const TESTS: &[Test] = &[
    test!(
        "cards and the table flow with the page in four windows, dark and light",
        lists_flow_with_the_page
    ),
    test!(
        "a long name stays in a narrow window and Tab reaches the card with its outline",
        long_name_and_keyboard
    ),
];

/// What every document of the page runs first: the stand-in for the list of the server.
const PAGE_JS: &str = include_str!("support/page.js");

// ---- the tests ----

/// `n` items for the page to list.
fn items(n: usize) -> Value {
    let items: Vec<Value> = (0..n)
        .map(|i| json!({"id": i.to_string(), "name": format!("Item {i}"), "updated_at": "2026-09-11"}))
        .collect();
    Value::Array(items)
}

/// Has the page list `items` from now on, in place of the items of the server.
async fn lay(page: &Page, items: Value) {
    page.js(
        "sessionStorage.setItem('test:items', JSON.stringify(arguments[0]));",
        json!([items]),
    )
    .await;
}

/// A list of the page and what it is held to: its rows, what holds them, where the rows begin
/// to be in sight (below the header), how far down the first row may begin, and how many rows
/// are wholly in sight at least.
struct List {
    rows: &'static str,
    holder: &'static str,
    below: f64,
    top: f64,
    visible: u64,
}

/// The cards of Home, below its header of 48px and its search field.
const CARDS: List = List {
    rows: ".card",
    holder: ".cards",
    below: 48.0,
    top: 160.0,
    visible: 7,
};
/// A dense table in `.content-wide` > `.table-scroll` (global.sass), with no header above it.
const TABLE: List = List {
    rows: "tbody tr",
    holder: ".table-scroll",
    below: 0.0,
    top: 65.0,
    visible: 15,
};

/// Puts a table of 100 rows in place of the page, under the styles of the page.
const TABLE_JS: &str = r#"
document.body.innerHTML = `<main class="content content-wide">
  <div class="table-scroll" tabindex="0" role="region" aria-label="作品一覧">
    <table>
      <thead><tr><th scope="col">名前</th><th scope="col">更新日</th></tr></thead>
      <tbody></tbody>
    </table>
  </div>
</main>`;
const body = document.querySelector("tbody");
for (let i = 1; i <= 100; i++) {
  const row = body.insertRow();
  row.insertCell().textContent = `作品 ${i}`;
  row.insertCell().textContent = "2026-09-11";
}
scrollTo(0, 0);
"#;

/// Where the rows of a list lie, and what overflows.
const MEASURE_JS: &str = "
const [rows, holder, below] = arguments;
const boxes = [...document.querySelectorAll(rows)].map((row) => row.getBoundingClientRect());
const held = document.querySelector(holder);
return {
  firstTop: boxes[0].top,
  visible: boxes.filter((box) => box.top >= below && box.bottom <= innerHeight).length,
  innerScroll: held.scrollHeight - held.clientHeight,
  documentWidth: document.documentElement.scrollWidth,
  maxHeight: getComputedStyle(held).maxHeight,
};";

/// The list lies in the page as `list` holds it to, and the wheel scrolls it with the page.
async fn flows(page: &Page, look: &str, list: &List, width: u32, height: u32) {
    let rows = list.rows;
    let at = page
        .js(MEASURE_JS, json!([rows, list.holder, list.below]))
        .await;
    assert_eq!(
        at["documentWidth"], width,
        "{look} {rows}: the page is wider than the window: {at}"
    );
    assert!(
        at["innerScroll"].as_f64().unwrap() <= 1.0,
        "{look} {rows}: the list scrolls inside itself: {at}"
    );
    assert_eq!(
        at["maxHeight"], "none",
        "{look} {rows}: the list is held to a height: {at}"
    );
    assert!(
        at["firstTop"].as_f64().unwrap() <= list.top,
        "{look} {rows}: the first row begins below {}px: {at}",
        list.top
    );
    assert!(
        at["visible"].as_u64().unwrap() >= list.visible,
        "{look} {rows}: fewer than {} rows are in sight: {at}",
        list.visible
    );
    let first = "return document.querySelector(arguments[0]).getBoundingClientRect().y;";
    let before = page.js(first, json!(rows)).await.as_f64().unwrap();
    page.wheel(f64::from(width) / 2.0, f64::from(height) / 2.0, 600.0)
        .await;
    page.until(
        &format!("{look} {rows}: the wheel scrolls the page"),
        "return scrollY > 500;",
        Value::Null,
    )
    .await;
    let after = page.js(first, json!(rows)).await.as_f64().unwrap();
    assert!(
        after < before - 500.0,
        "{look} {rows}: the first row stayed at {after} from {before}"
    );
}

async fn lists_flow_with_the_page(page: Arc<Page>, server: Arc<Server>) {
    lay(&page, items(100)).await;
    for (width, height) in [(1440, 900), (1280, 720), (375, 812), (320, 640)] {
        for theme in ["dark", "light"] {
            let look = format!("{width}x{height} {theme}");
            page.look(theme, width, height).await;
            page.open(&server.url("/")).await;
            page.until(
                &format!("{look}: 100 cards"),
                "return document.querySelectorAll('.card').length === 100;",
                Value::Null,
            )
            .await;
            flows(&page, &look, &CARDS, width, height).await;

            page.js(TABLE_JS, Value::Null).await;
            flows(&page, &look, &TABLE, width, height).await;
            let heading = page
                .js(
                    "const th = document.querySelector('th'); \
                     return [th.getBoundingClientRect().y, getComputedStyle(th).position];",
                    Value::Null,
                )
                .await;
            assert!(
                heading[0].as_f64().unwrap() < 0.0,
                "{look}: the column headings stay in sight: {heading}"
            );
            assert_eq!(
                heading[1], "static",
                "{look}: the column headings are not in the flow: {heading}"
            );
            if width < 768 {
                page.js(
                    "document.querySelector('[role=region]').focus();",
                    Value::Null,
                )
                .await;
                page.key("ArrowRight").await;
                page.until(
                    &format!("{look}: the wide table scrolls sideways by the key"),
                    "return document.querySelector('[role=region]').scrollLeft > 0;",
                    Value::Null,
                )
                .await;
            }
        }
    }
}

async fn long_name_and_keyboard(page: Arc<Page>, server: Arc<Server>) {
    page.look("light", 320, 640).await;
    let name = "LongName".repeat(30);
    lay(
        &page,
        json!([{"id": "0", "name": name, "updated_at": "2026-09-11"}]),
    )
    .await;
    page.open(&server.url("/")).await;
    page.until(
        "one card",
        "return document.querySelectorAll('.card').length === 1;",
        Value::Null,
    )
    .await;
    let width = page
        .js("return document.documentElement.scrollWidth;", Value::Null)
        .await;
    assert_eq!(width, 320, "the long name widens the page to {width}");
    page.js(
        "document.querySelector('input[type=search]').focus();",
        Value::Null,
    )
    .await;
    page.key("Tab").await;
    let outline = page
        .until(
            "Tab moves the focus from the search field to the card",
            "const card = document.querySelector('.card'); \
             return document.activeElement === card && getComputedStyle(card).outlineStyle;",
            Value::Null,
        )
        .await;
    assert_eq!(outline, "solid", "the focused card shows no outline");
}

// ---- running them ----

/// Which tests a run takes, as `cargo test` passes its words on: titles that hold any of the
/// words (all, when there are none), and none that hold a word after `--skip`; with
/// `--exact`, a word is a whole title. Other options are left alone.
fn chosen(args: &[String]) -> impl Fn(&str) -> bool {
    let mut words = Vec::new();
    let mut skipped = Vec::new();
    let mut exact = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--skip" => skipped.extend(args.next().cloned()),
            "--exact" => exact = true,
            option if option.starts_with('-') => {}
            word => words.push(word.to_owned()),
        }
    }
    move |title: &str| {
        let holds = |word: &String| {
            if exact {
                title == word
            } else {
                title.contains(word.as_str())
            }
        };
        (words.is_empty() || words.iter().any(holds)) && !skipped.iter().any(holds)
    }
}

/// What a panic said.
fn said(panic: &(dyn std::any::Any + Send)) -> String {
    panic
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| panic.downcast_ref::<&str>().map(|&said| said.to_owned()))
        .unwrap_or_default()
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let chosen = chosen(&args);
    let tests: Vec<&Test> = TESTS.iter().filter(|test| chosen(test.0)).collect();
    if args.iter().any(|arg| arg == "--list") {
        for (title, _) in &tests {
            println!("{title}: test");
        }
        return std::process::ExitCode::SUCCESS;
    }
    println!("\nrunning {} tests", tests.len());
    if tests.is_empty() {
        return std::process::ExitCode::SUCCESS;
    }
    let began = Instant::now();
    let server = Arc::new(Server::start().await);
    let page = Arc::new(Page::start(&[PAGE_JS]).await);
    let mut failed = Vec::new();
    for (title, test) in &tests {
        // each test begins on a page of no state, in a window of 1280 by 720 in the light
        page.open(&server.url("/healthz")).await;
        page.js("localStorage.clear(); sessionStorage.clear();", Value::Null)
            .await;
        page.look("light", 1280, 720).await;
        let began = Instant::now();
        let ran = tokio::spawn(test(page.clone(), server.clone())).await;
        let took = began.elapsed().as_secs_f64();
        match ran {
            Ok(()) => println!("test {title} ... ok ({took:.2}s)"),
            Err(error) => {
                println!("test {title} ... FAILED: {}", said(&*error.into_panic()));
                failed.push(*title);
            }
        }
    }
    let page = Arc::try_unwrap(page).ok().unwrap();
    page.close().await;
    let seconds = began.elapsed().as_secs_f64();
    if failed.is_empty() {
        println!(
            "\ntest result: ok. {} passed; 0 failed; finished in {seconds:.2}s\n",
            tests.len()
        );
        std::process::ExitCode::SUCCESS
    } else {
        println!("\nfailures:");
        for title in &failed {
            println!("    {title}");
        }
        println!(
            "\ntest result: FAILED. {} passed; {} failed; finished in {seconds:.2}s\n",
            tests.len() - failed.len(),
            failed.len()
        );
        std::process::ExitCode::from(101)
    }
}
