#![feature(rustc_attrs)]
#![rustc_coherence_is_core]
#![allow(internal_features)]

#[path = "../../lemmas.rs"]
mod lemmas;

impl<T> [T] {
    pub const fn first_chunk<const N: usize>(&self) -> Option<&[T; N]> {
        if self.len() < N {
            None
        } else {
            // SAFETY: We explicitly check for the correct number of elements,
            //   and do not let the reference outlive the slice.
            Some(unsafe { &*(self.as_ptr().cast_array()) })
        }
    }

    pub const fn first_chunk_mut<const N: usize>(&mut self) -> Option<&mut [T; N]> {
        if self.len() < N {
            None
        } else {
            // SAFETY: We explicitly check for the correct number of elements,
            //   do not let the reference outlive the slice,
            //   and require exclusive access to the entire slice to mutate the chunk.
            Some(unsafe { &mut *(self.as_mut_ptr().cast_array()) })
        }
    }
}

impl<T> *const T {
    pub const fn cast_array<const N: usize>(self) -> *const [T; N] {
        self.cast()
    }
}

impl<T> *mut T {
    pub const fn cast_array<const N: usize>(self) -> *mut [T; N] {
        self.cast()
    }
}
