#![allow(unused)]

pub fn ternary_quick_sort<T: Ord>(slice: &mut [T]) {
    // implement 3-way split quick sort
}

pub fn ternary_quick_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    // implement 3-way split quick sort by comp
}

#[cfg(test)]
mod tests {}
