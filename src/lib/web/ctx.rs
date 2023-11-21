use serde::Serialize; // all template contextst must be serialible
                      // so they can be turned inti a hashimap
use derive_more::Constructor;

// PageContext is used to get generic information about the context of every page
// and set global information like page titles
pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize, Default)]
pub struct Home {}

impl PageContext for Home {
    fn template_path(&self) -> &str {
        "home"
    }
    fn title(&self) -> &str {
        "Stash Your Clipboard!"
    }
    fn parent(&self) -> &str {
        "base" // includes page headers, footers, styles and scripts
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip,
}

impl PageContext for ViewClip {
    fn title(&self) -> &str {
        "View Clip"
    }

    fn template_path(&self) -> &str {
        "clip"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: crate::ShortCode,
}

impl PageContext for PasswordRequired {
    fn title(&self) -> &str {
        "Password Required"
    }

    fn template_path(&self) -> &str {
        "clip_need_password"
    }

    fn parent(&self) -> &str {
        "base"
    }
}
