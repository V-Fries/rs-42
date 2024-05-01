#[macro_export]
macro_rules! unwrap_or_return {
    ($result:expr, $return_closure:expr) => {
        match $result {
            Ok(inner) => inner,
            Err(why) => {
                return $return_closure(why);
            },
        }
    };
}

