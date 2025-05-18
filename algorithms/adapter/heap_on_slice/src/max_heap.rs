//! MaxHeap trait을 제공하는 모듈
use super::*;

/// # MaxHeap on Slice
/// MaxHeap의 기능을 제공하는 trait
/// MaxHeap은 기본적으로 큰 값이 루트에 위치하는(i.e. comp.cmp의 리턴이 std::cmp::Ordering::Greater인) 이진힙을 의미한다.
/// MaxHeap은 `cmp`메서드를 구현해 Comparator trait을 구현하면 autoimplement된다.
/// MinHeap trait과 method이름이 같으므로, 두 trait을 모두 use하는 경우 method를 구분하기 위해 명시적으로 trait을 지정해야 하는 불편함이 발생할 수 있음.
///
/// # Methods
/// heap property를 유지할 수 있도록 돕는 메서드 5개
/// - `test_heap_property`: slice가 maxheap property를 만족하는지 확인한다.
/// - `heapify`: 주어진 slice의 모든 원소가 heap property를 만족시키도록 변환한다.
/// - `move_upward`: heap property를 만족하는 array에서 idx번째 원소의 값이 커진 경우 호출하는 메서드. heap property를 만족할 때까지 위로 이동시킨다.
/// - `move_downward`: heap property를 만족하는 array에서 idx번째 원소의 값이 작아진 경우 호출하는 메서드. heap property를 만족할 때까지 아래로 이동시킨다.
/// - `adjust_heap`: heap property를 만족하는 array에서 idx번째 원소의 값이 커지거나 작아진 경우 호출하는 메서드. 부모또는 자식과 비교해 move_upward, move_downward를 호출한다.
///
/// heap을 priority queue로 사용할 수 있도록 돕는 메서드 2개
/// - `heap_pushpop`: queue에 새로운 원소를 추가하고, 가장 큰 원소를 pop한다.
/// - `heap_pop`: queue에서 가장 큰 원소를 pop하기 위한 메서드. slice의 마지막 위치로 pop된 원소를 이동시키고, 해당 원소를 포함하지 않도록 길이가 1 줄어든 slice를 반환한다.
///
/// in-place sort를 위한 메서드 1개
/// - `heap_sort`: slice를 in-place sort하는 메서드. MinHeap trait의 경우 heap_reverse_sort메서드를 제공한다.
///
/// # Heap push
/// MaxHeap trait은 heap_push 메서드를 제공하지 않는다.
/// 이는 `&mut [T]` slice는 컨테이너가 아니며, 참조하는 공간의 길이를 늘릴 수 없기 때문이다.
/// 따라서 사용자가 heap_push 기능을 구현하고자 한다면,
/// - 해당 슬라이스의 내용 뒤에 새로운 원소를 추가한다.
/// - `arr: &mut [T]`가 해당 원소를 포함하도록 인자를 전달한다.
/// - `move_upward` 메서드를 호출하여 heap property를 만족하도록 한다.
///
/// 이 과정에서 인자로 전달할 slice의 새로이 설정하기 위해 실제 메모리상의 위치를 reallocate해야 할 수 있다.
pub trait MaxHeap<T>: Comparator<T> {
    ///test_heap_property 메서드는 slice가 maxheap property를 만족하는지 확인한다.
    fn test_heap_property(&self, arr: &[T]) -> bool {
        let rev_comp = Reverse(self);
        test_heap_property(&rev_comp, arr)
    }

    /// heapify 메서드는 slice의 모든 원소가 maxheap property를 만족하도록 위치를 조정한다.
    fn heapify(&self, arr: &mut [T]) {
        let rev_comp: Reverse<'_, Self> = Reverse(self);
        heapify(&rev_comp, arr);
    }

    /// heap property를 만족하는 array에서 idx번째 원소의 값이 커진 경우 호출하는 메서드. heap property를 만족할 때까지 위로 이동시킨다.
    /// 실제로 힙 내부의 값 이동이 일어날 경우 true를 반환한다.
    /// 그렇지 않은 경우 false를 반환한다.
    fn move_upward(&self, arr: &mut [T], idx: usize) -> bool {
        let rev_comp = Reverse(self);
        move_upward(&rev_comp, arr, idx)
    }

    /// heap property를 만족하는 array에서 idx번째 원소의 값이 작아진 경우 호출하는 메서드. heap property를 만족할 때까지 아래로 이동시킨다.
    /// 실제로 힙 내부의 값 이동이 일어날 경우 true를 반환한다.
    /// 그렇지 않은 경우 false를 반환한다.
    fn move_downward(&self, arr: &mut [T], idx: usize) -> bool {
        let rev_comp = Reverse(self);
        move_downward(&rev_comp, arr, idx)
    }

    /// heap property를 만족하는 array에서 idx번째 원소의 값이 커지거나 작아진 경우 호출하는 메서드. 부모또는 자식과 비교해 move_upward, move_downward를 호출한다.
    /// 실제로 힙 내부의 값 이동이 일어날 경우 true를 반환한다.
    /// 그렇지 않은 경우 false를 반환한다.
    fn adjust_heap(&self, arr: &mut [T], idx: usize) -> bool {
        let rev_comp = Reverse(self);
        adjust_heap(&rev_comp, arr, idx)
    }

    /// queue에 새로운 원소를 추가하고, 가장 큰 원소를 pop한다.
    /// 힙의 Root값과 비교결과가 같은 경우 최적화를 위해 힙을 조정하지 않고 x를 반환한다.
    fn heap_pushpop(&self, arr: &mut [T], x: T) -> T {
        let rev_comp = Reverse(self);
        heap_pushpop(&rev_comp, arr, x)
    }

    /// queue에서 가장 큰 원소를 pop하기 위한 메서드. slice의 마지막 위치로 pop된 원소를 이동시키고, 해당 원소를 포함하지 않도록 길이가 1 줄어든 slice를 반환한다.
    /// 힙이 비어있는 경우 None을 반환한다.
    /// 그렇지 않은 경우, pop된 원소를 포함하지 않는 slice를 반환한다.
    fn heap_pop<'arr>(&self, arr: &'arr mut [T]) -> Option<&'arr mut [T]> {
        let rev_comp = Reverse(self);
        heap_pop(&rev_comp, arr)
    }

    /// slice를 in-place sort하는 메서드. MinHeap trait의 경우 heap_reverse_sort메서드를 제공한다.
    fn heap_sort(&self, arr: &mut [T]) {
        let rev_comp = Reverse(self);
        heap_reverse_sort(&rev_comp, arr);
    }
}

/// MaxHeap trait을 구현한 타입 C에 대해 MinHeap trait을 autoimplement한다.
impl<T, C: Comparator<T>> MaxHeap<T> for C {}
