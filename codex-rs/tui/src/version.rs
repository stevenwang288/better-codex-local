/// The current Codex CLI version as embedded at compile time.
///
/// `CODEX_CLI_VERSION_OVERRIDE` lets downstream builds pin an upstream-aligned
/// version string (for example `0.101.0`) while keeping workspace package
/// versions at `0.0.0` for local development.
pub const CODEX_CLI_VERSION: &str = match option_env!("CODEX_CLI_VERSION_OVERRIDE") {
    Some(value) if !value.is_empty() => value,
    _ => env!("CARGO_PKG_VERSION"),
};
