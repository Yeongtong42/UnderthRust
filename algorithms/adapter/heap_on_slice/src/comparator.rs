//! 이진힙을 구성하기 위해 필요한 비교함수 관련 trait과 utility를 제공하는 모듈
//! heap_on_slice 모듈에서 pub use comparator::* 하므로 모든 public trait과 struct가 노출된다.

/// 이진힙을 구성하기 위해 필요한 비교함수를 정의하는 trait
/// `cmp` 메소드를 구현해야 한다.
/// `comp.cmp(a, b)`의 의미는 `std::cmp::Ord::cmp`와 동일하게 해석한다.
/// Heap을 구성하는 과정에서 a, b가 부모자식 관계일 때 comp.cmp(a, b)의 결과가 std::cmp::Ordering::Less를 반환하는 경우,
/// MinHeap의 경우 a가 b의 부모가 되고, MaxHeap의 경우 b가 a의 부모가 된다.
pub trait Comparator<T> {
    fn cmp(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

/// Ord trait을 구현한 타입 T에 대해 기본으로 제공되는 Comparator implementation. Unit struct이므로
/// ```
/// use heap_on_slice::comparator::DefaultComparator;
/// let comp = DefaultComparator;
/// ```
/// 을 통해 생성할 수 있다.
#[derive(Debug, Copy, Clone, Default)]
pub struct DefaultComparator;
impl<T: Ord> Comparator<T> for DefaultComparator {
    fn cmp(&self, a: &T, b: &T) -> std::cmp::Ordering {
        a.cmp(b)
    }
}

/// Ord trait을 구현한 타입 T에 대해 기본으로 제공되는 Reversed Comparator implementation.
/// ```
/// use heap_on_slice::comparator::ReverseComparator;
/// let comp = ReverseComparator;
/// ```
/// 을 통해 생성할 수 있다.
/// ReverseComparator를 통해 MinHeap trait을 사용하는 것은 DefaultComparator를 통해 MaxHeap trait을 사용하는 것과 동등하다.
#[derive(Debug, Copy, Clone, Default)]
pub struct ReverseComparator;
impl<T: Ord> Comparator<T> for ReverseComparator {
    fn cmp(&self, a: &T, b: &T) -> std::cmp::Ordering {
        b.cmp(a)
    }
}

/// # Reverse
/// Comparator trait을 구현한 타입 C에 대해 order를 반전시켜 Comparator trait을 구현하는 wrapper struct.
/// heap_on_slice모듈에서는 instance dependent comparator를 지원하기 위해, Reverse wrapper는 원본 comparator의 인스턴스에 대한 참조를 wrapping한다.
/// Reverse struct을 통해 MinHeap trait을 사용하는 것은 MaxHeap trait을 사용하는 것과 동등하다.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Reverse<'c, C: ?Sized>(pub &'c C);

impl<T, C: Comparator<T> + ?Sized> Comparator<T> for Reverse<'_, C> {
    fn cmp(&self, a: &T, b: &T) -> std::cmp::Ordering {
        self.0.cmp(b, a)
    }
}

#[cfg(test)]
mod unit_test {
    //! implementation of unit tests for the heap_on_slice::comparator module

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn default_comparator_orders() {
        let comp = DefaultComparator;
        assert_eq!(comp.cmp(&1, &2), Ordering::Less);
        assert_eq!(comp.cmp(&2, &1), Ordering::Greater);
        assert_eq!(comp.cmp(&3, &3), Ordering::Equal);
    }

    #[test]
    fn reverse_comparator_orders() {
        let comp = ReverseComparator;
        assert_eq!(comp.cmp(&1, &2), Ordering::Greater);
        assert_eq!(comp.cmp(&2, &1), Ordering::Less);
        assert_eq!(comp.cmp(&3, &3), Ordering::Equal);
    }

    #[test]
    fn reverse_wrapper_reverses_order() {
        let dc = DefaultComparator;
        let comp = Reverse(&dc);
        assert_eq!(comp.cmp(&1, &2), Ordering::Greater);
        assert_eq!(comp.cmp(&2, &1), Ordering::Less);
        assert_eq!(comp.cmp(&3, &3), Ordering::Equal);

        let rc = ReverseComparator;
        let comp2 = Reverse(&rc);
        assert_eq!(comp2.cmp(&1, &2), Ordering::Less);
        assert_eq!(comp2.cmp(&2, &1), Ordering::Greater);
        assert_eq!(comp2.cmp(&3, &3), Ordering::Equal);
    }

    #[test]
    fn test_unsized_comparator() {
        // Use a trait object for Comparator<i32>
        let comp: &dyn Comparator<i32> = &DefaultComparator;
        // DefaultComparator should order 1 < 2
        assert_eq!(comp.cmp(&1, &2), Ordering::Less);
        // Wrap the trait object in Reverse to invert ordering
        let rev = Reverse(comp);
        assert_eq!(rev.cmp(&1, &2), Ordering::Greater);
    }
}
