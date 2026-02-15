use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

use crate::i18n::tr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Approvals,
    Permissions,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    #[strum(serialize = "sandbox-add-read-dir")]
    SandboxReadRoot,
    Experimental,
    Skills,
    Review,
    Rename,
    New,
    Resume,
    Fork,
    Init,
    Compact,
    Plan,
    Collab,
    Agent,
    // Undo,
    Diff,
    Mention,
    Status,
    DebugConfig,
    Statusline,
    Mcp,
    Apps,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    Clean,
    Personality,
    TestApproval,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Feedback => tr("send logs to maintainers", "发送日志给维护者"),
            SlashCommand::New => tr("start a new chat during a conversation", "在当前会话中开启新对话"),
            SlashCommand::Init => tr(
                "create an AGENTS.md file with instructions for Codex",
                "创建 AGENTS.md 指令文件",
            ),
            SlashCommand::Compact => tr(
                "summarize conversation to prevent hitting the context limit",
                "压缩总结对话，避免触发上下文上限",
            ),
            SlashCommand::Review => tr("review my current changes and find issues", "审查当前改动并找问题"),
            SlashCommand::Rename => tr("rename the current thread", "重命名当前会话"),
            SlashCommand::Resume => tr("resume a saved chat", "恢复已保存会话"),
            SlashCommand::Fork => tr("fork the current chat", "从当前会话分叉"),
            // SlashCommand::Undo => "ask Codex to undo a turn",
            SlashCommand::Quit | SlashCommand::Exit => tr("exit Codex", "退出 Codex"),
            SlashCommand::Diff => tr("show git diff (including untracked files)", "显示 git diff（含未跟踪文件）"),
            SlashCommand::Mention => tr("mention a file", "引用文件"),
            SlashCommand::Skills => tr(
                "use skills to improve how Codex performs specific tasks",
                "使用技能来提升特定任务效果",
            ),
            SlashCommand::Status => tr("show current session configuration and token usage", "显示当前会话配置与 token 使用"),
            SlashCommand::DebugConfig => tr(
                "show config layers and requirement sources for debugging",
                "显示配置层级与来源（调试用）",
            ),
            SlashCommand::Statusline => tr("configure which items appear in the status line", "配置状态栏显示项"),
            SlashCommand::Ps => tr("list background terminals", "列出后台终端"),
            SlashCommand::Clean => tr("stop all background terminals", "停止所有后台终端"),
            SlashCommand::MemoryDrop => tr("DO NOT USE", "请勿使用"),
            SlashCommand::MemoryUpdate => tr("DO NOT USE", "请勿使用"),
            SlashCommand::Model => tr("choose what model and reasoning effort to use", "选择模型与推理强度"),
            SlashCommand::Personality => tr("choose a communication style for Codex", "选择 Codex 的表达风格"),
            SlashCommand::Plan => tr("switch to Plan mode", "切换到 Plan 模式"),
            SlashCommand::Collab => tr("change collaboration mode (experimental)", "切换协作模式（实验）"),
            SlashCommand::Agent => tr("switch the active agent thread", "切换活动代理线程"),
            SlashCommand::Approvals => tr("choose what Codex is allowed to do", "设置 Codex 允许执行的操作"),
            SlashCommand::Permissions => tr("choose what Codex is allowed to do", "设置 Codex 允许执行的操作"),
            SlashCommand::ElevateSandbox => tr("set up elevated agent sandbox", "设置高权限智能体沙盒"),
            SlashCommand::SandboxReadRoot => {
                tr(
                    "let sandbox read a directory: /sandbox-add-read-dir <absolute_path>",
                    "允许沙盒读取目录：/sandbox-add-read-dir <absolute_path>",
                )
            }
            SlashCommand::Experimental => tr("toggle experimental features", "切换实验功能"),
            SlashCommand::Mcp => tr("list configured MCP tools", "列出已配置的 MCP 工具"),
            SlashCommand::Apps => tr("manage apps", "管理应用"),
            SlashCommand::Logout => tr("log out of Codex", "退出登录"),
            SlashCommand::Rollout => tr("print the rollout file path", "输出 rollout 文件路径"),
            SlashCommand::TestApproval => tr("test approval request", "测试审批请求"),
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::Plan
                | SlashCommand::SandboxReadRoot
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Resume
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            // | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Personality
            | SlashCommand::Approvals
            | SlashCommand::Permissions
            | SlashCommand::ElevateSandbox
            | SlashCommand::SandboxReadRoot
            | SlashCommand::Experimental
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Status
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Clean
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Collab => true,
            SlashCommand::Agent => true,
            SlashCommand::Statusline => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::SandboxReadRoot => cfg!(target_os = "windows"),
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}
