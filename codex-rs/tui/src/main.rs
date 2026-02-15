use clap::Parser;
use codex_arg0::arg0_dispatch_or_else;
use codex_tui::Cli;
use codex_tui::run_main;
use codex_utils_cli::CliConfigOverrides;

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|codex_linux_sandbox_exe| async move {
        // Default this fork to Chinese UI unless the user explicitly overrides it.
        // Users can still force English via `CODEX_UI_LANG=en`.
        if std::env::var_os("CODEX_UI_LANG").is_none() {
            // SAFETY: We set this once at process start before spawning any worker threads.
            unsafe {
                std::env::set_var("CODEX_UI_LANG", "zh-CN");
            }
        }

        let top_cli = TopCli::parse();
        let mut inner = top_cli.inner;
        inner
            .config_overrides
            .raw_overrides
            .splice(0..0, top_cli.config_overrides.raw_overrides);
        let exit_info = run_main(inner, codex_linux_sandbox_exe).await?;
        let token_usage = exit_info.token_usage;
        if !token_usage.is_zero() {
            println!("{}", codex_core::protocol::FinalOutput::from(token_usage),);
        }
        Ok(())
    })
}
