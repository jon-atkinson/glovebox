//! `glovebox` — sandboxed Claude Code environments driven from a zellij plugin.
//!
//! Development happens at <https://github.com/jon-atkinson/glovebox>.

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_set() {
        assert_eq!(super::VERSION, "0.0.1");
    }
}
