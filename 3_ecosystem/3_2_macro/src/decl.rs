macro_rules! btreemap {
    ( $( ($k:expr, $v:expr) ),* ) => {
        {
            use std::collections::BTreeMap;

            let mut buf = BTreeMap::new();
            $(
                buf.insert($k, $v);
            )*
            buf
        }
    };
}
pub(crate) use btreemap;
