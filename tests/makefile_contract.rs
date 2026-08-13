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

use std::path::Path;

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

/// Reports whether `text` references the dev-fast configuration
/// fragment, matching `dev-fast` or `dev_fast` case-insensitively.
fn mentions_dev_fast(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("dev-fast") || lower.contains("dev_fast")
}

/// Asserts that a Makefile recipe wires cargo through the dev-fast
/// configuration fragment, naming the convention and where it is
/// documented when the assertion fails.
fn assert_uses_dev_fast_config(target: &str, recipe: &str) {
    let wired = recipe.contains("--config") && mentions_dev_fast(recipe);
    assert!(
        wired,
        "the `{target}` Makefile target must invoke cargo with `--config \
         tools/dev-fast/config.toml` (the DEV_FAST_CONFIG variable) so it runs on the estate's \
         dev-fast standard development path; see AGENTS.md's \"Standard development path\" section",
    );
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
    assert!(
        !mentions_dev_fast(&recipe),
        "the `coverage` Makefile target must never reference the dev-fast configuration fragment: \
         coverage requires the LLVM codegen backend and the platform linker; see AGENTS.md's \
         \"Fast development builds\" section",
    );
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
