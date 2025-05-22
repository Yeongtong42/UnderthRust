#![allow(unused)]

pub fn binary_quick_sort<T: Ord>(slice: &mut [T]) {
    // implement basic quick sort
}

pub fn binary_quick_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    // implement basic quick sort by comp
}

#[cfg(test)]
mod tests {}
