use std::env;
/**
 * Read env and preset configurations
 */
pub fn preset_envs() {
    let log_level = env::var("LOG_LEVEL").unwrap_or("".to_string());
    if !log_level.is_empty() && (vec!["Error", "Warn", "Info", "Debug"].contains(&log_level.as_str())) {
        env::set_var("LOG_LEVEL", log_level);
    } else {
        env::set_var("LOG_LEVEL", "Info");
    }

    let no_html = env::var("NO_HTML").unwrap_or("".to_string());
    if !no_html.is_empty() && (vec!["true", "false"].contains(&no_html.as_str())) {
        env::set_var("NO_HTML", no_html);
    } else {
        env::set_var("NO_HTML", "true");
    }

    let no_html = env::var("NOCLIENTCACHE").unwrap_or("".to_string());
    if !no_html.is_empty() && (vec!["true", "false"].contains(&no_html.as_str())) {
        env::set_var("NOCLIENTCACHE", no_html);
    } else {
        env::set_var("NOCLIENTCACHE", "true");
    }
}