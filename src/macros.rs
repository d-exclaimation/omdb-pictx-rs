macro_rules! f {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        res
    }}
}

pub(crate) use f;
