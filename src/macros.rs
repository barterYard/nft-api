#[macro_export]
macro_rules! get_str {
    ($var:ident, $a:expr) => {
        let $var = $a.to_string();
        let $var = $var.as_str();
    };
}
