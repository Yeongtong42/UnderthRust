#![allow(unused)]

pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    // implement insertion sort
}

pub fn insertion_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    // implement insertion sort by comp
}

#[cfg(test)]
mod tests {}
