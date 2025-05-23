#![allow(unused)]

use std::ptr::{read, write};

pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    if slice.is_empty() {
        return;
    }
    let len = slice.len();
    let begin = &mut slice[0] as *mut T;
    for i in 1..len {
        unsafe {
            let right_hand = begin.add(i).read();
            let mut j = i;
            while j > 0 && slice[j - 1] > right_hand {
                write(begin.add(j), begin.add(j - 1).read());
                j -= 1;
            }
            write(begin.add(j), right_hand);
        }
    }
}

pub fn insertion_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut cmp = comp;
    use std::cmp::Ordering as O;
    if slice.is_empty() {
        return;
    }
    let len = slice.len();
    let begin = &mut slice[0] as *mut T;
    for i in 1..len {
        unsafe {
            let right_hand = begin.add(i).read();
            let mut j = i;
            while j > 0 && (O::Greater == cmp(&slice[j - 1], &right_hand)) {
                write(begin.add(j), begin.add(j - 1).read());
                j -= 1;
            }
            write(begin.add(j), right_hand);
        }
    }
}

#[cfg(test)]
mod tests {}
