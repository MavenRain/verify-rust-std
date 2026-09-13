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
            let pointer = self.as_ptr().cast_array();
            //@ close array_share::<T, N>('a, _t, pointer);
            //@ leak array_share::<T, N>('a, _t, pointer);
            //@ let p = precreate_ref(pointer);
            //@ produce_type_interp::<[T; N]>();
            //@ init_ref_share::<[T; N]>('a, _t, p);
            //@ leak type_interp::<[T; N]>();
            //@ open_frac_borrow('a, ref_initialized_::<[T; N]>(p), _q_a / 2);
            //@ open [?f]ref_initialized_::<[T; N]>(p)();
            // SAFETY: We explicitly check for the correct number of elements,
            //   and do not let the reference outlive the slice.
            let result: Option<&'a [T; N]> = Some(unsafe { &*pointer });
            //@ close [f]ref_initialized_::<[T; N]>(p)();
            //@ close_frac_borrow(f, ref_initialized_::<[T; N]>(p));
            //@ let array = p as &'a [T; N];
            //@ close_ref_own::<'a, [T; N]>(array);
            //@ close std::option::Option_own::<&'a [T; N]>(_t, std::option::Option::Some(array));
            result
        }
    }
}

impl<T> *const T {
    pub const fn cast_array<const N: usize>(self) -> *const [T; N]
//@ req true;
    //@ ens result == self as *const [T; N];
    //@ on_unwind_ens false;
    {
        self.cast()
    }
}
