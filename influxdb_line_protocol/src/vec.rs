use std::borrow::Borrow;
use std::ops::Index;

/// A fixed-size, stack allocated array that can spill to the heap
///
/// Previously this crate used smallvec, however, this has
/// covariance issues that cause problems with borrowed types
///
/// This is a hopefully temporary workaround until the upstream
/// bug is fixed -https://github.com/servo/rust-smallvec/issues/217
///
#[derive(Debug)]
pub enum NanoVec<T: Sized, const C: usize> {
    Inline(arrayvec::ArrayVec<T, C>),
    Heap(Vec<T>),
}

impl<T: Sized, const C: usize> Default for NanoVec<T, C> {
    fn default() -> Self {
        Self::Inline(Default::default())
    }
}

impl<T: Sized, const C: usize> NanoVec<T, C> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match &self {
            Self::Inline(data) => data.len(),
            Self::Heap(data) => data.len(),
        }
    }

    pub fn push(&mut self, value: T) {
        match self {
            Self::Inline(data) if data.len() == C => {
                // We are going to manually move data out of the ArrayVec so put into
                // ManuallyDrop to prevent `ArrayVec::drop()` from being called
                let data = std::mem::ManuallyDrop::new(data);

                let mut heap = Vec::with_capacity(C + 1);
                unsafe {
                    // SAFETY
                    // - data.as_ptr() is valid for C aligned reads of size T
                    // - heap.as_mut_ptr() is valid for C aligned writes of size T
                    std::ptr::copy_nonoverlapping(data.as_ptr(), heap.as_mut_ptr(), C);
                    // SAFETY
                    // - capacity is C+1 and just wrote indexes 0..=C
                    heap.set_len(C);
                }
                heap.push(value);

                *self = Self::Heap(heap)
            }
            // SAFETY - data.len() cannot be larger than C and verified that it doesn't equal C
            Self::Inline(data) => unsafe { data.push_unchecked(value) },
            Self::Heap(heap) => heap.push(value),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match &self {
            Self::Inline(data) => data.as_slice(),
            Self::Heap(data) => data.as_slice(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T: Sized, const C: usize> Index<usize> for NanoVec<T, C> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Inline(data) => &data[index],
            Self::Heap(data) => &data[index],
        }
    }
}

impl<T, const CAP: usize> std::ops::Deref for NanoVec<T, CAP> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Sized, const C: usize> Borrow<[T]> for NanoVec<T, C> {
    fn borrow(&self) -> &[T] {
        match self {
            Self::Inline(data) => data,
            Self::Heap(data) => data,
        }
    }
}

impl<T, const CAP: usize> AsRef<[T]> for NanoVec<T, CAP> {
    fn as_ref(&self) -> &[T] {
        match self {
            Self::Inline(data) => data,
            Self::Heap(data) => data,
        }
    }
}

impl<T: Sized, const C: usize> From<Vec<T>> for NanoVec<T, C> {
    fn from(heap: Vec<T>) -> Self {
        Self::Heap(heap)
    }
}

impl<'a, T: Sized, const C: usize> IntoIterator for &'a NanoVec<T, C> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Sized, const C: usize> IntoIterator for NanoVec<T, C> {
    type Item = T;
    type IntoIter = IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Inline(data) => IntoIter::Inline(data.into_iter()),
            Self::Heap(data) => IntoIter::Heap(data.into_iter()),
        }
    }
}

#[derive(Debug)]
pub enum IntoIter<T: Sized, const C: usize> {
    Inline(arrayvec::IntoIter<T, C>),
    Heap(std::vec::IntoIter<T>),
}

impl<T: Sized, const C: usize> Iterator for IntoIter<T, C> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Inline(data) => data.next(),
            Self::Heap(data) => data.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Inline(data) => data.size_hint(),
            Self::Heap(data) => data.size_hint(),
        }
    }
}
