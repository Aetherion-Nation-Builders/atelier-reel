//! Nation and project links (Help menu, About dialog, header button, Home screen).
//!
//! Atelier Reel is part of Atelier, the Nation's creative suite. The command ids keep their
//! upstream names (`help.discord`, `help.website`) so menus and shortcuts stay untouched.

/// The app's short name, used in the website and repository URLs.
pub const APP: &str = "reel";

/// The Nation (the `help.discord` command: where people gather).
pub const DISCORD: &str = "https://www.aetherionnnation.com";
/// The Atelier suite.
pub const WEBSITE: &str = "https://www.aetherionnnation.com/atelier";
/// This app's page.
pub const APP_PAGE: &str = "https://www.aetherionnnation.com/atelier/reel";
/// This app's source repository.
pub const GITHUB: &str = "https://github.com/Aetherion-Nation-Builders/atelier-reel";
/// New issue on this app's repository.
pub const ISSUES: &str = "https://github.com/Aetherion-Nation-Builders/atelier-reel/issues";

/// (command id, label, url) for every link, in menu order.
pub const ALL: [(&str, &str, &str); 5] = [
    ("help.discord", "Join the Nation…", DISCORD),
    ("help.website", "Atelier Suite", WEBSITE),
    ("help.appPage", "Atelier Reel Website", APP_PAGE),
    ("help.github", "Atelier Reel on GitHub", GITHUB),
    ("help.reportIssue", "Report an Issue…", ISSUES),
];

/// The URL a `help.*` link command opens.
pub fn url_for(command: &str) -> Option<&'static str> {
    ALL.iter().find(|(id, _, _)| *id == command).map(|(_, _, u)| *u)
}

/// Open `url` in the system browser (a new tab on the web).
pub fn open(ctx: &egui::Context, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_follow_the_atelier_scheme() {
        assert_eq!(APP_PAGE, format!("{WEBSITE}/{APP}"));
        assert_eq!(GITHUB, format!("https://github.com/Aetherion-Nation-Builders/atelier-{APP}"));
        assert!(ALL.iter().all(|(id, _, u)| id.starts_with("help.") && u.starts_with("https://")));
        assert_eq!(url_for("help.discord"), Some(DISCORD));
        assert_eq!(url_for("help.nope"), None);
    }
}
