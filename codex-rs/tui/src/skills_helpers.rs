use codex_core::skills::model::SkillMetadata;
use codex_utils_fuzzy_match::fuzzy_match;

use crate::i18n::use_zh_cn;
use crate::text_formatting::truncate_text;

pub(crate) const SKILL_NAME_TRUNCATE_LEN: usize = 21;

pub(crate) fn skill_display_name(skill: &SkillMetadata) -> &str {
    skill
        .interface
        .as_ref()
        .and_then(|interface| interface.display_name.as_deref())
        .unwrap_or(&skill.name)
}

pub(crate) fn skill_description(skill: &SkillMetadata) -> &str {
    skill
        .interface
        .as_ref()
        .and_then(|interface| interface.short_description.as_deref())
        .or(skill.short_description.as_deref())
        .unwrap_or(&skill.description)
}

pub(crate) fn localized_skill_description(skill: &SkillMetadata) -> String {
    let description = skill_description(skill).trim();
    if description.is_empty() {
        return String::new();
    }
    if !use_zh_cn() {
        return description.to_string();
    }

    let translated = match skill.name.as_str() {
        "agent-browser" => "基于 Rust 的高速无头浏览器自动化 CLI（含 Node.js 回退），可用于导航、点击、输入和页面快照。",
        "atlas" => "仅限 macOS 的 ChatGPT Atlas 桌面端 AppleScript 控制技能；仅在明确需要控制 Atlas 标签/书签/历史时使用。",
        "autofillin" => "基于 Playwright 的自动填表与文件上传技能，支持登录态保持、表单识别与提交前人工确认。",
        "browser-daemon" => "基于 Playwright 守护进程的持久浏览器自动化（无 MCP/无 Puppeteer），可持续接收导航与脚本命令。",
        "browser-max-automation" => "使用 Playwright MCP 的浏览器自动化，适用于网页测试、表单操作、截图与 iframe/复杂 JS 场景。",
        "clean-code-auditor" => "用于 JS/TS 代码的 Clean Code + SOLID + TDD 审核与重构，重点处理命名、职责划分与错误模型问题。",
        "cloudflare-deploy" => "将应用与基础设施部署到 Cloudflare（Workers/Pages 等）。",
        "codex-cli" => "编排 OpenAI Codex CLI 的并行任务执行：协调子任务、上下文注入与会话复用。",
        "dev-browser" => "带持久页面状态的浏览器自动化技能，适用于导航、填表、截图、抓取和 Web 测试。",
        "develop-web-game" => "用于 Web 游戏迭代开发：小步修改、Playwright 测试、截图检查和控制台错误回归。",
        "disk-growth-forensics" => "Windows C 盘突增空间取证流程，支持按时间窗口扫描并给出分级清理建议。",
        "doc" => "用于 .docx 文档读取/创建/编辑，重视排版与布局一致性。",
        "e2e-playwright" => "Playwright 端到端测试专家技能，覆盖跨浏览器、视觉回归、移动模拟、可访问性与 CI 集成。",
        "figma" => "通过 Figma MCP 获取设计上下文、截图、变量与资源，并辅助设计到代码实现。",
        "figma-implement-design" => "将 Figma 节点高保真实现为生产代码（1:1 视觉还原）。",
        "find-skills" => "帮助发现并安装可用技能，适合用户询问“有没有技能可做某事”时使用。",
        "gh-address-comments" => "使用 gh CLI 处理当前分支 PR 的 review/issue 评论，并先校验 gh 登录状态。",
        "gh-fix-ci" => "排查并修复 GitHub Actions 失败检查：先收集失败上下文和计划，再在获批后实施修复。",
        "imagegen" => "通过 OpenAI Image API 生成或编辑图片（含修补、换背景、透明底与批量变体）。",
        "jupyter-notebook" => "创建和维护 Jupyter Notebook（.ipynb），优先使用模板与脚本快速生成。",
        "linear" => "管理 Linear 工单、项目与团队流程。",
        "netlify-deploy" => "使用 Netlify CLI 部署网站，支持预览与生产发布。",
        "notion-knowledge-capture" => "把对话与决策沉淀到结构化 Notion 页面（知识库/FAQ/决策记录）。",
        "notion-meeting-intelligence" => "结合 Notion 上下文与研究结果，准备会议议程、预读和参会人定制材料。",
        "notion-research-documentation" => "跨 Notion 做信息检索与综合，输出结构化文档并附引用。",
        "notion-spec-to-implementation" => "将 Notion 规格转为实施计划、任务拆分与进度跟踪。",
        "openai-docs" => "当用户询问 OpenAI 产品/API 开发时，提供带引用的官方最新文档答案。",
        "pdf" => "用于 PDF 的读取、生成与版面校验（重视渲染结果）。",
        "playwright" => "在终端中自动化真实浏览器（导航、填表、截图、抓取、UI 流调试）。",
        "playwright-cli" => "使用 playwright-cli 做网页交互自动化、截图与信息提取。",
        "pretty-table" => "渲染对齐良好的终端表格（兼容 CJK/emoji 宽度），并支持 Markdown 表格格式化。",
        "proxmox" => "通过 REST API 管理 Proxmox 集群（VM/LXC 启停、快照、任务与节点状态）。",
        "proxmox-vm" => "与 Proxmox 虚拟机交互：截图、按键注入与网络信息查看。",
        "remote-chrome" => "通过 CDP 连接已运行的 Chrome/Edge，复用已登录配置进行自动化。",
        "render-deploy" => "分析代码并生成 Render 部署蓝图（render.yaml）与控制台直达链接。",
        "screenshot" => "执行系统级截图（全屏/窗口/区域），用于工具内截图能力不足的场景。",
        "security-best-practices" => "按语言/框架执行安全最佳实践审查，并给出加固建议。",
        "security-ownership-map" => "基于 Git 历史构建安全责任拓扑，分析敏感代码归属与 bus factor 风险。",
        "security-threat-model" => "基于仓库进行威胁建模：资产、边界、攻击路径与缓解措施。",
        "selenium" => "Selenium 跨浏览器自动化测试（Chrome/Firefox/Safari/Edge）。",
        "sentry" => "查询 Sentry 问题与事件，汇总近期线上错误（只读）。",
        "skill-scanner" => "安装前扫描技能包中的恶意代码风险（挖矿、后门、外传等）。",
        "skill-vetter" => "以安全优先方式评估技能来源、权限范围与可疑模式。",
        "skillguard" => "对 AgentSkill 做安全扫描（凭据窃取、注入、越权、数据外传等）。",
        "skills-audit" => "使用 SkillLens 审计本地技能目录，输出风险导向报告。",
        "skills-search" => "在 skills.sh 注册表中搜索并发现技能。",
        "sora" => "通过 OpenAI 视频 API 生成/混剪/下载/删除 Sora 视频内容。",
        "speech" => "通过 OpenAI Audio API 执行文本转语音（旁白、无障碍朗读、批量语音）。",
        "spreadsheet" => "处理电子表格（xlsx/csv/tsv）的创建、编辑、分析与格式保持。",
        "transcribe" => "音视频转写技能，支持说话人区分与已知说话人提示。",
        "uisheji" => "编排并演示 UI 交付闭环（概念稿→选择→落地页面→自动打开）。",
        "vercel-deploy" => "使用脚本与 Vercel 流程完成预览与生产部署。",
        "vercel-react-best-practices" => "Vercel 的 React/Next.js 性能优化实践指南。",
        "web-design-guidelines" => "按 Web 界面规范审查 UI/UX 与可访问性。",
        "yeet" => "仅在用户明确要求时，执行一体化流程：stage + commit + push + 创建 PR。",
        _ => description,
    };

    translated.to_string()
}

pub(crate) fn truncate_skill_name(name: &str) -> String {
    truncate_text(name, SKILL_NAME_TRUNCATE_LEN)
}

pub(crate) fn match_skill(
    filter: &str,
    display_name: &str,
    skill_name: &str,
) -> Option<(Option<Vec<usize>>, i32)> {
    if let Some((indices, score)) = fuzzy_match(display_name, filter) {
        return Some((Some(indices), score));
    }
    if display_name != skill_name
        && let Some((_indices, score)) = fuzzy_match(skill_name, filter)
    {
        return Some((None, score));
    }
    None
}
