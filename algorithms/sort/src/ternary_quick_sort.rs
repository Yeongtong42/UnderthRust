#![allow(unused)]

/// partition slice in 3 part and return it's delimeter
/// based on Dijkstra's Dutch national flag algorithm
/// two pivot's are at front and back
fn ternary_partition<T: Ord>(slice: &mut [T]) -> (usize, usize) {
    use std::cmp::Ordering as O;
    let end = slice.len() - 1;

    if slice[0] > slice[end] {
        slice.swap(0, end);
    }

    // [0, i) : smaller, equal pivot 1
    // [i, j) : big than pivot 1 and smaller than pivot2
    // [j.. : big, equal pivot 2
    let mut i = 1usize;
    let mut j = 1usize;
    let mut k = end - 1;
    while j <= k {
        if slice[j].cmp(&slice[0]).is_le() {
            // left
            slice.swap(i, j);
            i += 1;
            j += 1;
        } else if slice[j].cmp(&slice[end]).is_ge() {
            // right
            slice.swap(j, k);
            k -= 1;
        } else {
            // mid
            j += 1;
        }
    }
    slice.swap(0, i - 1);
    slice.swap(j, end);
    (i, j)
}

/// sort slice with 3-way partition quick-sort algorithm
/// use Ord trait for sorting
pub fn ternary_quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let (pivot1, pivot2) = ternary_partition(slice);

    ternary_quick_sort(&mut slice[0..pivot1 - 1]);
    ternary_quick_sort(&mut slice[pivot1..pivot2]);
    ternary_quick_sort(&mut slice[pivot2 + 1..]);
}

/// partition slice in 3 part and return it's delimeter
/// based on Dijkstra's Dutch national flag algorithm
/// two pivot's are at front and back
/// use comp to identify it's order
fn ternary_partition_by<T, F>(slice: &mut [T], comp: &mut F) -> (usize, usize)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering as O;
    let end = slice.len() - 1;

    if comp(&slice[0], &slice[end]).is_gt() {
        slice.swap(0, end);
    }

    // [0, i) : smaller, equal pivot 1
    // [i, j) : big than pivot 1 and smaller than pivot2
    // [j.. : big, equal pivot 2
    let mut i = 1usize;
    let mut j = 1usize;
    let mut k = end - 1;
    while j <= k {
        if comp(&slice[j], &slice[0]).is_le() {
            // left
            slice.swap(i, j);
            i += 1;
            j += 1;
        } else if comp(&slice[j], &slice[end]).is_ge() {
            // right
            slice.swap(j, k);
            k -= 1;
        } else {
            // mid
            j += 1;
        }
    }
    slice.swap(0, i - 1);
    slice.swap(j, end);
    (i, j)
}

fn ternary_quick_by<T, F>(slice: &mut [T], comp: &mut F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if slice.len() <= 1 {
        return;
    }

    let (pivot1, pivot2) = ternary_partition_by(slice, comp);

    ternary_quick_by(&mut slice[0..pivot1 - 1], comp);
    ternary_quick_by(&mut slice[pivot1..pivot2], comp);
    ternary_quick_by(&mut slice[pivot2 + 1..], comp);
}

/// sort slice with 3-way partition quick-sort algorithm
/// use custom comparator 'comp' for sorting
pub fn ternary_quick_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut cmp = comp;
    ternary_quick_by(slice, &mut cmp);
}

#[cfg(test)]
mod tests {}
