/// Create a heap allocated macro from a string literal.
/// Optionally have it directly converted to upper or lowercase.
///
/// Example:
/// ```
/// let s = string!("Hello");
/// let uppercase = string!("Hello", UC);
/// let lowercase = string!("Hello", LC);
/// ```
#[macro_export]
macro_rules! string {
    ($e : literal, UC) => {
        String::from($e).to_uppercase()
    };
    ($e : literal, LC) => {
        String::from($e).to_lowercase()
    };
    ($e : literal) => {
        String::from($e)
    };
}
