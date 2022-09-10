use std::path::Path;

#[cfg(windows)]
#[inline]
pub(crate) fn format_pattern_to_expression(pattern: &str) -> String {
    pattern.replace("\\", "/")
}

#[cfg(not(windows))]
#[inline]
pub(crate) fn format_pattern_to_expression(pattern: &str) -> String {
    pattern.to_string()
}

#[cfg(windows)]
#[inline]
pub(crate) fn format_expression_to_path(expression: &str) -> String {
    expression.replace("/", "\\")
}

#[cfg(not(windows))]
#[inline]
pub(crate) fn format_expression_to_path(expression: &str) -> String {
    expression.to_string()
}

#[cfg(windows)]
#[inline]
pub(crate) fn trim_last_separator(path: &String) -> &str {
    path.trim_end_matches('\\')
}

#[cfg(not(windows))]
#[inline]
pub(crate) fn trim_last_separator(path: &String) -> &str {
    path.trim_end_matches('/')
}

#[inline]
pub(crate) fn path_to_string(path: &Path) -> String {
    path.display().to_string()
}
