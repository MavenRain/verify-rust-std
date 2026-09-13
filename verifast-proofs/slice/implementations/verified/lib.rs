#![feature(rustc_attrs)]
#![rustc_coherence_is_core]
#![allow(internal_features)]

#[path = "../../lemmas.rs"]
mod lemmas;

impl<T> [T] {
    pub const fn first_chunk<'a, const N: usize>(&'a self) -> Option<&'a [T; N]> {
        if self.len() < N {
            //@ close std::option::Option_own::<&'a [T; N]>(_t, std::option::Option::None);
            None
        } else {
            //@ assert [_]slice_elements_share('a, _t, ?data, ?length);
            //@ lemmas::split_shared_elements('a, _t, data, length, usize_of_const(typeid(N)));
            //@ lemmas::slice_prefix_valid(data, length, usize_of_const(typeid(N)));
            //@ let array = data as &'a [T; N];
            //@ close array_share::<T, N>('a, _t, array);
            //@ leak array_share::<T, N>('a, _t, array);
            //@ close_ref_own::<'a, [T; N]>(array);
            //@ close std::option::Option_own::<&'a [T; N]>(_t, std::option::Option::Some(array));
            // SAFETY: We explicitly check for the correct number of elements,
            //   and do not let the reference outlive the slice.
            Some(unsafe { &*(self.as_ptr().cast_array()) })
        }
    }
}

impl<T> *const T {
    pub const fn cast_array<const N: usize>(self) -> *const [T; N] {
        self.cast()
    }
}
