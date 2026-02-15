use crate::i18n::tr;
use crate::i18n::use_zh_cn;
use codex_core::features::FEATURES;
use codex_protocol::account::PlanType;
use lazy_static::lazy_static;
use rand::Rng;

const ANNOUNCEMENT_TIP_URL: &str =
    "https://raw.githubusercontent.com/openai/codex/main/announcement_tip.toml";

const IS_MACOS: bool = cfg!(target_os = "macos");

const PAID_TOOLTIP: &str = "*New* Try the **Codex App** with 2x rate limits until *April 2nd*. Run 'codex app' or visit https://chatgpt.com/codex?app-landing-page=true";
const PAID_TOOLTIP_NON_MAC: &str = "*New* 2x rate limits until *April 2nd*.";
const OTHER_TOOLTIP: &str = "*New* Build faster with the **Codex App**. Run 'codex app' or visit https://chatgpt.com/codex?app-landing-page=true";
const OTHER_TOOLTIP_NON_MAC: &str = "*New* Build faster with Codex.";
const FREE_GO_TOOLTIP: &str =
    "*New* Codex is included in your plan for free through *March 2nd* – let’s build together.";

const RAW_TOOLTIPS: &str = include_str!("../tooltips.txt");

lazy_static! {
    static ref TOOLTIPS: Vec<&'static str> = RAW_TOOLTIPS
        .lines()
        .map(str::trim)
        .filter(|line| {
            if line.is_empty() || line.starts_with('#') {
                return false;
            }
            if !IS_MACOS && line.contains("codex app") {
                return false;
            }
            true
        })
        .collect();
    static ref ALL_TOOLTIPS: Vec<&'static str> = {
        let mut tips = Vec::new();
        tips.extend(TOOLTIPS.iter().copied());
        tips.extend(experimental_tooltips());
        tips
    };
}

fn experimental_tooltips() -> Vec<&'static str> {
    FEATURES
        .iter()
        .filter_map(|spec| spec.stage.experimental_announcement())
        .collect()
}

/// Pick a random tooltip to show to the user when starting Codex.
pub(crate) fn get_tooltip(plan: Option<PlanType>) -> Option<String> {
    let mut rng = rand::rng();

    if let Some(announcement) = announcement::fetch_announcement_tip() {
        return Some(announcement);
    }

    // Leave small chance for a random tooltip to be shown.
    if rng.random_ratio(8, 10) {
        match plan {
            Some(PlanType::Plus)
            | Some(PlanType::Business)
            | Some(PlanType::Team)
            | Some(PlanType::Enterprise)
            | Some(PlanType::Pro) => {
                let tooltip = if IS_MACOS {
                    tr(
                        PAID_TOOLTIP,
                        "*新功能* 在 *4 月 2 日* 前可体验 **Codex App**（速率上限 2 倍）。运行 `codex app` 或访问 https://chatgpt.com/codex?app-landing-page=true",
                    )
                } else {
                    tr(
                        PAID_TOOLTIP_NON_MAC,
                        "*新功能* 在 *4 月 2 日* 前享受 2 倍速率上限。",
                    )
                };
                return Some(tooltip.to_string());
            }
            Some(PlanType::Go) | Some(PlanType::Free) => {
                return Some(
                    tr(
                        FREE_GO_TOOLTIP,
                        "*新功能* 在 *3 月 2 日* 前，Codex 已免费包含在你的套餐中，一起构建吧。",
                    )
                    .to_string(),
                );
            }
            _ => {
                let tooltip = if IS_MACOS {
                    tr(
                        OTHER_TOOLTIP,
                        "*新功能* 用 **Codex App** 更快构建。运行 `codex app` 或访问 https://chatgpt.com/codex?app-landing-page=true",
                    )
                } else {
                    tr(OTHER_TOOLTIP_NON_MAC, "*新功能* 用 Codex 更快构建。")
                };
                return Some(tooltip.to_string());
            }
        }
    }

    pick_tooltip(&mut rng).map(localize_tooltip)
}

fn pick_tooltip<R: Rng + ?Sized>(rng: &mut R) -> Option<&'static str> {
    if ALL_TOOLTIPS.is_empty() {
        None
    } else {
        ALL_TOOLTIPS
            .get(rng.random_range(0..ALL_TOOLTIPS.len()))
            .copied()
    }
}

fn localize_tooltip(tip: &str) -> String {
    if !use_zh_cn() {
        return tip.to_string();
    }

    match tip {
        "Use /compact when the conversation gets long to summarize history and free up context." => {
            "当对话变长时，使用 /compact 总结历史并释放上下文。".to_string()
        }
        "Start a fresh idea with /new; the previous session stays in history." => {
            "用 /new 开启新话题；上一会话会保留在历史中。".to_string()
        }
        "Use /feedback to send logs to the maintainers when something looks off." => {
            "出现异常时，用 /feedback 把日志发给维护者。".to_string()
        }
        "Switch models or reasoning effort quickly with /model." => {
            "用 /model 可快速切换模型或推理强度。".to_string()
        }
        "Use /permissions to control when Codex asks for confirmation." => {
            "用 /permissions 控制 Codex 何时请求确认。".to_string()
        }
        "Run /review to get a code review of your current changes." => {
            "运行 /review 获取当前改动的代码审查。".to_string()
        }
        "Use /skills to list available skills or ask Codex to use one." => {
            "用 /skills 查看可用技能，或让 Codex 使用某个技能。".to_string()
        }
        "Use /status to see the current model, approvals, and token usage." => {
            "用 /status 查看当前模型、审批策略和 Token 使用情况。".to_string()
        }
        "Use /fork to branch the current chat into a new thread." => {
            "用 /fork 将当前聊天分叉为新线程。".to_string()
        }
        "Use /init to create an AGENTS.md with project-specific guidance." => {
            "用 /init 创建带项目指引的 AGENTS.md。".to_string()
        }
        "Use /mcp to list configured MCP tools." => {
            "用 /mcp 查看已配置的 MCP 工具。".to_string()
        }
        "Run `codex app` to open Codex Desktop (it installs on macOS if needed)." => {
            "运行 `codex app` 打开 Codex 桌面端（macOS 缺失时会自动安装）。".to_string()
        }
        "Use /personality to customize how Codex communicates." => {
            "用 /personality 自定义 Codex 的沟通风格。".to_string()
        }
        "Use /rename to rename your threads for easier thread resuming." => {
            "用 /rename 重命名线程，便于后续恢复。".to_string()
        }
        "Use the OpenAI docs MCP for API questions; enable it with `codex mcp add openaiDeveloperDocs --url https://developers.openai.com/mcp`." => {
            "遇到 API 问题可用 OpenAI Docs MCP；可通过 `codex mcp add openaiDeveloperDocs --url https://developers.openai.com/mcp` 启用。".to_string()
        }
        "Join the OpenAI community Discord: http://discord.gg/openai" => {
            "加入 OpenAI 社区 Discord：http://discord.gg/openai".to_string()
        }
        "Visit the Codex community forum: https://community.openai.com/c/codex/37" => {
            "访问 Codex 社区论坛：https://community.openai.com/c/codex/37".to_string()
        }
        "You can run any shell command from Codex using `!` (e.g. `!ls`)" => {
            "你可以在 Codex 中用 `!` 运行任意 shell 命令（例如 `!ls`）。".to_string()
        }
        "Type / to open the command popup; Tab autocompletes slash commands." => {
            "输入 / 打开命令弹窗；Tab 可自动补全斜杠命令。".to_string()
        }
        "When the composer is empty, press Esc to step back and edit your last message; Enter confirms." => {
            "当输入框为空时，按 Esc 可回退并编辑上一条消息；Enter 确认。".to_string()
        }
        "Press Tab to queue a message when a task is running; otherwise it sends immediately (except `!`)." => {
            "任务运行中按 Tab 可将消息入队；否则会立即发送（`!` 例外）。".to_string()
        }
        "Paste an image with Ctrl+V to attach it to your next message." => {
            "按 Ctrl+V 粘贴图片，可将其附加到下一条消息。".to_string()
        }
        "You can resume a previous conversation by running `codex resume`" => {
            "运行 `codex resume` 可恢复之前的对话。".to_string()
        }
        _ => tip.to_string(),
    }
}

pub(crate) mod announcement {
    use crate::tooltips::ANNOUNCEMENT_TIP_URL;
    use crate::version::CODEX_CLI_VERSION;
    use chrono::NaiveDate;
    use chrono::Utc;
    use regex_lite::Regex;
    use serde::Deserialize;
    use std::sync::OnceLock;
    use std::thread;
    use std::time::Duration;

    static ANNOUNCEMENT_TIP: OnceLock<Option<String>> = OnceLock::new();

    /// Prewarm the cache of the announcement tip.
    pub(crate) fn prewarm() {
        let _ = thread::spawn(|| ANNOUNCEMENT_TIP.get_or_init(init_announcement_tip_in_thread));
    }

    /// Fetch the announcement tip, return None if the prewarm is not done yet.
    pub(crate) fn fetch_announcement_tip() -> Option<String> {
        ANNOUNCEMENT_TIP
            .get()
            .cloned()
            .flatten()
            .and_then(|raw| parse_announcement_tip_toml(&raw))
    }

    #[derive(Debug, Deserialize)]
    struct AnnouncementTipRaw {
        content: String,
        from_date: Option<String>,
        to_date: Option<String>,
        version_regex: Option<String>,
        target_app: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct AnnouncementTipDocument {
        announcements: Vec<AnnouncementTipRaw>,
    }

    #[derive(Debug)]
    struct AnnouncementTip {
        content: String,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        version_regex: Option<Regex>,
        target_app: String,
    }

    fn init_announcement_tip_in_thread() -> Option<String> {
        thread::spawn(blocking_init_announcement_tip)
            .join()
            .ok()
            .flatten()
    }

    fn blocking_init_announcement_tip() -> Option<String> {
        // Avoid system proxy detection to prevent macOS system-configuration panics (#8912).
        let client = reqwest::blocking::Client::builder()
            .no_proxy()
            .build()
            .ok()?;
        let response = client
            .get(ANNOUNCEMENT_TIP_URL)
            .timeout(Duration::from_millis(2000))
            .send()
            .ok()?;
        response.error_for_status().ok()?.text().ok()
    }

    pub(crate) fn parse_announcement_tip_toml(text: &str) -> Option<String> {
        let announcements = toml::from_str::<AnnouncementTipDocument>(text)
            .map(|doc| doc.announcements)
            .or_else(|_| toml::from_str::<Vec<AnnouncementTipRaw>>(text))
            .ok()?;

        let mut latest_match = None;
        let today = Utc::now().date_naive();
        for raw in announcements {
            let Some(tip) = AnnouncementTip::from_raw(raw) else {
                continue;
            };
            if tip.version_matches(CODEX_CLI_VERSION)
                && tip.date_matches(today)
                && tip.target_app == "cli"
            {
                latest_match = Some(tip.content);
            }
        }
        latest_match
    }

    impl AnnouncementTip {
        fn from_raw(raw: AnnouncementTipRaw) -> Option<Self> {
            let content = raw.content.trim();
            if content.is_empty() {
                return None;
            }

            let from_date = match raw.from_date {
                Some(date) => Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()?),
                None => None,
            };
            let to_date = match raw.to_date {
                Some(date) => Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()?),
                None => None,
            };
            let version_regex = match raw.version_regex {
                Some(pattern) => Some(Regex::new(&pattern).ok()?),
                None => None,
            };

            Some(Self {
                content: content.to_string(),
                from_date,
                to_date,
                version_regex,
                target_app: raw.target_app.unwrap_or("cli".to_string()).to_lowercase(),
            })
        }

        fn version_matches(&self, version: &str) -> bool {
            self.version_regex
                .as_ref()
                .is_none_or(|regex| regex.is_match(version))
        }

        fn date_matches(&self, today: NaiveDate) -> bool {
            if let Some(from) = self.from_date
                && today < from
            {
                return false;
            }
            if let Some(to) = self.to_date
                && today >= to
            {
                return false;
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tooltips::announcement::parse_announcement_tip_toml;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn random_tooltip_returns_some_tip_when_available() {
        let mut rng = StdRng::seed_from_u64(42);
        assert!(pick_tooltip(&mut rng).is_some());
    }

    #[test]
    fn random_tooltip_is_reproducible_with_seed() {
        let expected = {
            let mut rng = StdRng::seed_from_u64(7);
            pick_tooltip(&mut rng)
        };

        let mut rng = StdRng::seed_from_u64(7);
        assert_eq!(expected, pick_tooltip(&mut rng));
    }

    #[test]
    fn announcement_tip_toml_picks_last_matching() {
        let toml = r#"
[[announcements]]
content = "first"
from_date = "2000-01-01"

[[announcements]]
content = "latest match"
version_regex = ".*"
target_app = "cli"

[[announcements]]
content = "should not match"
to_date = "2000-01-01"
        "#;

        assert_eq!(
            Some("latest match".to_string()),
            parse_announcement_tip_toml(toml)
        );

        let toml = r#"
[[announcements]]
content = "first"
from_date = "2000-01-01"
target_app = "cli"

[[announcements]]
content = "latest match"
version_regex = ".*"

[[announcements]]
content = "should not match"
to_date = "2000-01-01"
        "#;

        assert_eq!(
            Some("latest match".to_string()),
            parse_announcement_tip_toml(toml)
        );
    }

    #[test]
    fn announcement_tip_toml_picks_no_match() {
        let toml = r#"
[[announcements]]
content = "first"
from_date = "2000-01-01"
to_date = "2000-01-05"

[[announcements]]
content = "latest match"
version_regex = "invalid_version_name"

[[announcements]]
content = "should not match either "
target_app = "vsce"
        "#;

        assert_eq!(None, parse_announcement_tip_toml(toml));
    }

    #[test]
    fn announcement_tip_toml_bad_deserialization() {
        let toml = r#"
[[announcements]]
content = 123
from_date = "2000-01-01"
        "#;

        assert_eq!(None, parse_announcement_tip_toml(toml));
    }

    #[test]
    fn announcement_tip_toml_parse_comments() {
        let toml = r#"
# Example announcement tips for Codex TUI.
# Each [[announcements]] entry is evaluated in order; the last matching one is shown.
# Dates are UTC, formatted as YYYY-MM-DD. The from_date is inclusive and the to_date is exclusive.
# version_regex matches against the CLI version (env!("CARGO_PKG_VERSION")); omit to apply to all versions.
# target_app specify which app should display the announcement (cli, vsce, ...).

[[announcements]]
content = "Welcome to Codex! Check out the new onboarding flow."
from_date = "2024-10-01"
to_date = "2024-10-15"
target_app = "cli"
version_regex = "^0\\.0\\.0$"

[[announcements]]
content = "This is a test announcement"
        "#;

        assert_eq!(
            Some("This is a test announcement".to_string()),
            parse_announcement_tip_toml(toml)
        );
    }
}
