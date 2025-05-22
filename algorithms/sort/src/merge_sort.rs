#![allow(unused)]

pub fn merge_sort<T: Ord>(slice: &mut [T]) {
    // implement merge sort
}

pub fn merge_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    // implement merge sort by comp
}

#[cfg(test)]
mod tests {}
