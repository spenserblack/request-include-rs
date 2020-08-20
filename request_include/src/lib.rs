use proc_macro_hack::proc_macro_hack;

/// Use a request response to include a string.
///
/// # Examples
///
/// *__NOTE__ The below examples fail to compile because `<website>` is not a valid URL.*
///
/// ## No User Agent
///
/// ```rust,compile_fail
/// let response = request_include::include_str!("<website>");
/// ```
///
/// ## With User Agent
///
/// Many websites will reject certain user agents. The default user agent
/// is likely to be blocked by many websites, so you should probably specify
/// a user agent.
///
/// ```rust,compile_fail
/// let response = request_include::include_str!("<website>", "<my user agent (contact info)>");
/// ```
#[proc_macro_hack]
pub use request_include_impl::include_str;
