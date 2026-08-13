//! Contract test for the Makefile's dev-fast wiring.
//!
//! The estate's dev-fast profile (`tools/dev-fast/config.toml`) is the
//! standard development path, not an opt-in side path: every cargo
//! invocation in the `build`, `test`, `lint`, and `typecheck` Makefile
//! targets must pass `--config tools/dev-fast/config.toml`, while
//! `coverage` must never do so, because coverage requires the LLVM
//! codegen backend and the platform linker. See AGENTS.md's "Standard
//! development path" section for the full rationale. This test reads the
//! repository's own Makefile so an over-eager edit that drops the wiring
//! fails locally before the estate-wide DF-004 audit ever runs.
//!
//! Checks operate per cargo-invoking recipe line, not on a recipe's whole
//! text: a target whose recipe carries several cargo lines (the `doc`
//! step plus `clippy` in `lint`) would otherwise pass a whole-block match
//! even when only one of those lines is wired. A separate pair of tests
//! dry-runs `dev-build`/`dev-test` with a substituted `CARGO` value to
//! prove the Wave 1 block's `$(CARGO)` indirection actually reaches the
//! `--config` flag, without needing the nightly toolchain or `mold`.

use std::{path::Path, process::Command};

use rstest::rstest;

const MAKEFILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Makefile"));

/// Returns the tab-indented recipe lines that follow the first line
/// starting with `header_prefix`, or an empty string if no such rule
/// header exists.
///
/// This mirrors the convention documented in the dev-fast wiring brief:
/// a target's recipe block is the run of indented lines between its
/// header and the next non-indented line.
fn recipe_after(makefile: &str, header_prefix: &str) -> String {
    let Some(start) = makefile
        .lines()
        .position(|line| line.starts_with(header_prefix))
    else {
        return String::new();
    };
    makefile
        .lines()
        .skip(start + 1)
        .take_while(|line| line.starts_with('\t'))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns the recipe used for `make build`, following the delegation to
/// the `target/%/$(TARGET)` pattern rule when `build` itself carries no
/// recipe of its own (it depends on `target/debug/$(TARGET)` instead).
fn build_recipe(makefile: &str) -> String {
    let direct = recipe_after(makefile, "build:");
    if direct.is_empty() {
        recipe_after(makefile, "target/%/$(TARGET):")
    } else {
        direct
    }
}

/// Returns the lines within `recipe` that directly invoke `$(CARGO)`, as
/// distinct from lines that invoke some other tool (for example the
/// Whitaker binary, which reuses `$(CARGO_FLAGS)` but not `$(CARGO)`
/// itself).
fn cargo_invocation_lines(recipe: &str) -> Vec<&str> {
    recipe
        .lines()
        .filter(|line| line.contains("$(CARGO)"))
        .collect()
}

/// Reports whether `text` references the dev-fast configuration
/// fragment, matching `dev-fast` or `dev_fast` case-insensitively.
fn mentions_dev_fast(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("dev-fast") || lower.contains("dev_fast")
}

/// Asserts that every `$(CARGO)` invocation in `recipe` passes `--config`
/// and references the dev-fast fragment, checked line by line so a
/// partially-wired multi-line recipe fails rather than passing on the
/// strength of its other lines.
fn assert_uses_dev_fast_config(target: &str, recipe: &str) {
    let cargo_lines = cargo_invocation_lines(recipe);
    assert!(
        !cargo_lines.is_empty(),
        "the `{target}` Makefile target's recipe has no $(CARGO) invocation to check for dev-fast \
         wiring; see AGENTS.md's \"Standard development path\" section",
    );
    for line in cargo_lines {
        let wired = line.contains("--config") && mentions_dev_fast(line);
        assert!(
            wired,
            "the `{target}` Makefile target's cargo invocation `{line}` must pass `--config \
             tools/dev-fast/config.toml` (the DEV_FAST_CONFIG variable) so it runs on the \
             estate's dev-fast standard development path; see AGENTS.md's \"Standard development \
             path\" section",
        );
    }
}

/// Asserts that no `$(CARGO)` invocation in `recipe` references the
/// dev-fast fragment, checked line by line for the same reason as
/// [`assert_uses_dev_fast_config`].
fn assert_never_uses_dev_fast_config(target: &str, recipe: &str) {
    for line in cargo_invocation_lines(recipe) {
        assert!(
            !mentions_dev_fast(line),
            "the `{target}` Makefile target's cargo invocation `{line}` must never reference the \
             dev-fast configuration fragment: {target} requires the LLVM codegen backend and the \
             platform linker; see AGENTS.md's \"Fast development builds\" section",
        );
    }
}

#[rstest]
#[case::make_test("test")]
#[case::make_lint("lint")]
#[case::make_typecheck("typecheck")]
fn standard_target_uses_dev_fast_config(#[case] target: &str) {
    let header = format!("{target}:");
    let recipe = recipe_after(MAKEFILE, &header);
    assert_uses_dev_fast_config(target, &recipe);
}

#[test]
fn build_target_uses_dev_fast_config() {
    let recipe = build_recipe(MAKEFILE);
    assert_uses_dev_fast_config("build", &recipe);
}

#[test]
fn coverage_target_never_uses_dev_fast_config() {
    let recipe = recipe_after(MAKEFILE, "coverage:");
    assert_never_uses_dev_fast_config("coverage", &recipe);
}

#[test]
fn dev_fast_config_file_exists() {
    let path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tools/dev-fast/config.toml"
    ));
    assert!(
        path.exists(),
        "tools/dev-fast/config.toml must exist: the Makefile's DEV_FAST_CONFIG variable and the \
         standard development targets depend on it; see AGENTS.md's \"Fast development builds\" \
         section",
    );
}

/// Runs `make --dry-run <target> CARGO=<probe_cargo>` in the repository
/// root and returns the printed command, without building or testing
/// anything. Proves the Wave 1 block's `$(CARGO)` indirection actually
/// reaches the recipe, on any toolchain, with no nightly or `mold`
/// dependency.
///
/// Returns `Err` rather than panicking directly: this is a helper, not a
/// `#[test]` function itself, so `clippy::expect_used` still applies to
/// it even with `allow-expect-in-tests` set.
fn dry_run(target: &str, probe_cargo: &str) -> Result<String, String> {
    let output = Command::new("make")
        .arg("--dry-run")
        .arg(target)
        .arg(format!("CARGO={probe_cargo}"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .map_err(|error| {
            format!("failed to run `make --dry-run {target} CARGO={probe_cargo}`: {error}")
        })?;
    String::from_utf8(output.stdout)
        .map_err(|error| format!("make dry-run output for `{target}` was not valid UTF-8: {error}"))
}

/// Asserts that each string in `needles` occurs in `text`, in the given
/// order, by repeatedly splitting off everything before and including
/// the next needle. Names `target` when a needle is missing or
/// out of order.
fn assert_ordered(target: &str, text: &str, needles: &[&str]) {
    let mut remaining = text;
    for needle in needles {
        let Some((_, after)) = remaining.split_once(needle) else {
            panic!(
                "the `make {target}` dry-run output must contain {needles:?} in order; {needle:?} \
                 was missing after the previous marker; full output: {text:?}",
            );
        };
        remaining = after;
    }
}

#[rstest]
#[case::probe_dev_build("dev-build")]
#[case::probe_dev_test("dev-test")]
fn dev_fast_target_respects_cargo_substitution(#[case] target: &str) {
    let probe_cargo = "probe-cargo";
    let output = dry_run(target, probe_cargo).expect("make --dry-run must succeed");
    assert_ordered(target, &output, &[probe_cargo, "--config", "dev-fast"]);
}
