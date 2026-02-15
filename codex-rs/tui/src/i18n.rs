pub(crate) fn tr<'a>(en: &'a str, zh_cn: &'a str) -> &'a str {
    if use_zh_cn() {
        zh_cn
    } else {
        en
    }
}

pub(crate) fn use_zh_cn() -> bool {
    if let Some(explicit) = std::env::var("CODEX_UI_LANG")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
    {
        match explicit.as_str() {
            "zh" | "zh-cn" | "zh_cn" | "zh-hans" | "zh_hans" => return true,
            "en" | "en-us" | "en_us" => return false,
            _ => {}
        }
    }

    ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .any(|value| value.to_ascii_lowercase().contains("zh"))
        || windows_user_locale_is_zh()
}

#[cfg(all(windows, not(test)))]
fn windows_user_locale_is_zh() -> bool {
    const LOCALE_NAME_CAPACITY: usize = 85;
    let mut buffer = [0u16; LOCALE_NAME_CAPACITY];
    // SAFETY: `buffer` is valid for writes of `LOCALE_NAME_CAPACITY` UTF-16 code units.
    let len = unsafe {
        GetUserDefaultLocaleName(buffer.as_mut_ptr(), LOCALE_NAME_CAPACITY as i32)
    };
    if len <= 1 {
        return false;
    }
    let locale = String::from_utf16_lossy(&buffer[..(len as usize - 1)]).to_ascii_lowercase();
    locale.starts_with("zh")
}

#[cfg(any(not(windows), test))]
fn windows_user_locale_is_zh() -> bool {
    false
}

#[cfg(all(windows, not(test)))]
#[link(name = "Kernel32")]
unsafe extern "system" {
    fn GetUserDefaultLocaleName(locale_name: *mut u16, locale_name_count: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::tr;
    use serial_test::serial;
    use std::ffi::OsString;

    struct EnvVarGuard {
        key: &'static str,
        original: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let original = std::env::var_os(key);
            unsafe {
                std::env::set_var(key, value);
            }
            Self { key, original }
        }

        fn remove(key: &'static str) -> Self {
            let original = std::env::var_os(key);
            unsafe {
                std::env::remove_var(key);
            }
            Self { key, original }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.original {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    #[serial]
    fn explicit_code_overrides_locale_detection() {
        let _lang = EnvVarGuard::set("LANG", "zh_CN.UTF-8");
        let _ui_lang = EnvVarGuard::set("CODEX_UI_LANG", "en");
        assert_eq!(tr("Hello", "你好"), "Hello");
    }

    #[test]
    #[serial]
    fn locale_detection_enables_chinese_when_no_explicit_override() {
        let _ui_lang = EnvVarGuard::remove("CODEX_UI_LANG");
        let _lang = EnvVarGuard::set("LANG", "zh_CN.UTF-8");
        assert_eq!(tr("Hello", "你好"), "你好");
    }
}
