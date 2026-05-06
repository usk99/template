use std::{cmp::Reverse, collections::BinaryHeap, marker::PhantomData};

pub struct Max;
pub struct Min;

pub trait HeapOrder<T: Ord> {
    type Item: Ord;
    fn wrap(value: T) -> Self::Item;
    fn unwrap(item: Self::Item) -> T;
    fn unwrap_ref(item: &Self::Item) -> &T;
}

impl<T: Ord> HeapOrder<T> for Max {
    type Item = T;
    fn wrap(value: T) -> T {
        value
    }
    fn unwrap(item: T) -> T {
        item
    }
    fn unwrap_ref(item: &T) -> &T {
        item
    }
}

impl<T: Ord> HeapOrder<T> for Min {
    type Item = Reverse<T>;
    fn wrap(value: T) -> Reverse<T> {
        Reverse(value)
    }
    fn unwrap(Reverse(item): Reverse<T>) -> T {
        item
    }
    fn unwrap_ref(Reverse(item): &Reverse<T>) -> &T {
        item
    }
}

#[derive(Clone, Debug)]
pub struct Heap<T: Ord, Order: HeapOrder<T>> {
    heap: BinaryHeap<Order::Item>,
    _phantom: PhantomData<(T, Order)>,
}

pub type MaxHeap<T> = Heap<T, Max>;
pub type MinHeap<T> = Heap<T, Min>;

impl<T: Ord, Order: HeapOrder<T>> Heap<T, Order> {
    /// 空のヒープを作成
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            _phantom: PhantomData,
        }
    }
    /// 容量を指定して作成
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
            _phantom: PhantomData,
        }
    }
    /// 要素を追加
    pub fn push(&mut self, value: T) {
        self.heap.push(Order::wrap(value));
    }
    /// 先頭要素を取り出す（MaxHeap: 最大値, MinHeap: 最小値）
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(Order::unwrap)
    }
    /// 先頭要素を参照（取り出さない）
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(Order::unwrap_ref)
    }
    /// 要素数を返す
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    /// 空かどうか
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    /// 全要素を削除
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    /// 別のヒープの全要素を移動して結合（other は空になる）
    pub fn append(&mut self, other: &mut Self) {
        self.heap.append(&mut other.heap);
    }
    /// 全要素を順序不定で取り出す（ヒープは空になる）
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.heap.drain().map(Order::unwrap)
    }
    /// 全要素を順序不定で参照
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter().map(Order::unwrap_ref)
    }
}

impl<T: Ord, Order: HeapOrder<T>> From<Vec<T>> for Heap<T, Order> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            heap: vec.into_iter().map(Order::wrap).collect(),
            _phantom: PhantomData,
        }
    }
}
impl<T: Ord, Order: HeapOrder<T>> IntoIterator for Heap<T, Order> {
    type Item = T;
    type IntoIter =
        std::iter::Map<std::collections::binary_heap::IntoIter<Order::Item>, fn(Order::Item) -> T>;
    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter().map(Order::unwrap)
    }
}
impl<T: Ord, Order: HeapOrder<T>> FromIterator<T> for Heap<T, Order> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            heap: iter.into_iter().map(Order::wrap).collect(),
            _phantom: PhantomData,
        }
    }
}
impl<T: Ord, Order: HeapOrder<T>> Default for Heap<T, Order> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord, Order: HeapOrder<T>> Extend<T> for Heap<T, Order> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.heap.extend(iter.into_iter().map(Order::wrap));
    }
}
