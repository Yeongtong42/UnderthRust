#![allow(unused)]

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot = len - 1;
    let mut cur_left_pos = 0usize;

    for i in 0..len {
        if slice[i] <= slice[pivot] {
            slice.swap(cur_left_pos, i);
            cur_left_pos += 1;
        }
    }
    cur_left_pos - 1
}

pub fn binary_quick_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    // partition
    let pivot_pos = partition(slice);

    // recurse two part
    binary_quick_sort(&mut slice[0..pivot_pos]);
    binary_quick_sort(&mut slice[pivot_pos + 1..len]);
}

fn partition_by<T, F>(slice: &mut [T], comp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering as O;
    let mut cmp = comp;
    let len = slice.len();
    let pivot = len - 1;
    let mut cur_left_pos = 0usize;

    for i in 0..len {
        if O::Greater != cmp(&slice[i], &slice[pivot]) {
            slice.swap(cur_left_pos, i);
            cur_left_pos += 1;
        }
    }
    cur_left_pos - 1
}

fn quick_sort_by_comp<T, F>(slice: &mut [T], comp: &mut F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let len = slice.len();
    if len <= 1 {
        return;
    }

    // partition
    let pivot_pos = partition_by(slice, comp);

    // recurse two part
    quick_sort_by_comp(&mut slice[0..pivot_pos], comp);
    quick_sort_by_comp(&mut slice[pivot_pos + 1..len], comp)
}

pub fn binary_quick_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut cmp = comp;
    quick_sort_by_comp(slice, &mut cmp);
}

#[cfg(test)]
mod tests {}
