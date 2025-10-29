/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    /// 创建新堆，传入比较函数
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引从1开始，0位置占位
            comparator,
        }
    }

    /// 返回堆中元素个数
    pub fn len(&self) -> usize {
        self.count
    }

    /// 判断堆是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 插入一个值
    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.count < self.items.len() {
            self.items[self.count] = value;
        } else {
            self.items.push(value);
        }

        // 上浮：从最后一个位置开始，与父节点比较
        self.bubble_up(self.count);
    }

    /// 上浮第 i 个元素以维护堆性质
    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    /// 获取父节点索引
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    /// 判断是否有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /// 左子节点索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    /// 右子节点索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 返回较小（或较大）的子节点索引，依据 comparator
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right > self.count {
            return left; // 只有左子节点
        }

        // 根据 comparator 决定哪个子节点更“优先”
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }

    // /// 弹出堆顶元素
    // pub fn pop(&mut self) -> Option<T> {
    //     if self.is_empty() {
    //         return None;
    //     }

    //     let ret = self.items.swap_remove(1); // 移除第一个有效元素
    //     self.count -= 1;

    //     if self.count > 0 {
    //         // 将最后一个元素移到顶部，并下沉
    //         let last = self.items.swap_remove(self.count + 1);
    //         self.items.insert(1, last);
    //         self.bubble_down(1);
    //     }

    //     Some(ret)
    // }
    /// 弹出堆顶元素
pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
        return None;
    }

    self.count -= 1;
    let ret = self.items.swap_remove(1); // 自动用最后一个元素填充位置1

    if self.count > 0 {
        self.bubble_down(1);
    }

    Some(ret)
}

    /// 下沉第 i 个元素以维护堆性质
    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建最小堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建最大堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    /// 每次返回堆顶元素（最小或最大），并移除它
    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.pop()
        }
    }
}

/// 包装类型：MinHeap
pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_min()
    }
}

/// 包装类型：MaxHeap
pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}