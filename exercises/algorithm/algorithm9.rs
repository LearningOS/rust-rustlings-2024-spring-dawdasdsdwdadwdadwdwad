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
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }
    pub fn add(&mut self, value: T) {
        //TODO
        if self.count == 0 {
            self.items.remove(0);
            println!("成功移除0!");
        }
        self.items.push(value);
        self.count = self.count + 1;
        let mut index = self.count;
        while index > 1 {
            let parent = self.parent_idx(index);
            if (self.comparator)(&self.items[index - 1], &self.items[parent - 1]) {
                self.items.swap(index - 1, parent - 1);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if right_idx <= self.count
            && (self.comparator)(&self.items[right_idx], &self.items[left_idx])
        {
            right_idx
        } else {
            left_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        println!("now count is {}", self.count);
        let mut idx = 1;
        self.items.swap(0, self.count - 1);
        println!("in function next before remove count is {}", self.count);
        let mut root = self.items.remove(self.count - 1);
        self.count -= 1;
        println!("remove success!");
        println!("in function next after remove count is {}", self.count);
        let mut circle = self.count - 1;
        let mut countcir = 0;
        while circle != countcir {
            self.items.swap(countcir, countcir + 1);
            countcir += 1;
        }
        Some(root)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
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

        println!("count is {}", heap.count);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        let mut count = 1;
        for i in heap.items.iter() {
            println!("第{count}个是{i}");
            count += 1;
        }
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        println!("count is {}", heap.count);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        let mut count = 1;
        for i in heap.items.iter() {
            println!("第{count}个是{i}");
            count += 1;
        }
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
