#[macro_export]
macro_rules! Prefix_Sum_Map {
    ($( $x:expr => $y:expr ),*) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                if temp_map.contains_key(&$x) {
                    temp_map.insert($x, temp_map[&$x] + $y);
                } else {
                    temp_map.insert($x, $y);
                }
            )*
            temp_map
        }
    };
}
