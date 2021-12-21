#[allow(dead_code)]
pub fn as_groups(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut grouped = Vec::new();
    let mut groups = Vec::new();
    for line in lines {
        if line.is_empty() && !grouped.is_empty() {
            groups.push(grouped);
            grouped = Vec::new();
        } else {
            grouped.push(line);
        }
    }

    if !grouped.is_empty() {
        groups.push(grouped);
    }

    groups
}

#[allow(unused_macros)]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        use std::collections::HashMap;
        Iterator::collect::<HashMap<_, _>>(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        use std::collections::HashSet;
        Iterator::collect::<HashSet<_>>(IntoIterator::into_iter([$($v,)*]))
    }};
}
