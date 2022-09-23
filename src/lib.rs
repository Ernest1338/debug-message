/// Print the given message if the DEBUG environment
/// variable is set.
///
/// # Example
///
/// ```no_run
/// use debug-message::debug;
///
/// println!("this will be printed every time");
/// debug("this will only be printed if the DEBUG env var is set");
/// ```
/// for more code examples check out the usage.rs example
///
/// # CLI Usage
///
/// WONT print the debug messages:
/// ```no_run
/// cargo r --example usage
/// ```
/// WILL print the debug messages:
/// ```no_run
/// DEBUG=1 cargo r --example usage
/// ```
pub fn debug(message: &str) {
    if std::env::var("DEBUG").is_ok() {
        println!("\x1b[32m[DEBUG]\x1b[00m {}", message);
    }
}
