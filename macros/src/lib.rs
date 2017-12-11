#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)+) => {
        hashmap!($($key => $value),+) 
    };
    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut _map = HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}
