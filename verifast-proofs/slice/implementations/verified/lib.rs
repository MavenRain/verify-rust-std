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

    pub const fn first_chunk_mut<'a, const N: usize>(&'a mut self) -> Option<&'a mut [T; N]> {
        //@ assert full_borrow('a, slice_full_borrow_content::<T>(_t, ?data, ?length));
        if self.len() < N {
            //@ leak full_borrow('a, slice_full_borrow_content::<T>(_t, data, length));
            //@ close std::option::Option_own::<&'a mut [T; N]>(_t, std::option::Option::None);
            None
        } else {
            let pointer = self.as_mut_ptr().cast_array();
            //@ lemmas::slice_prefix_valid(data, length, usize_of_const(typeid(N)));
            //@ open_full_borrow_strong_('a, slice_full_borrow_content::<T>(_t, data, length));
            //@ lemmas::split_owned_elements(_t, data, length, usize_of_const(typeid(N)));
            //@ lemmas::slice_to_owned_array::<T, N>(_t, data);
            //@ open array_full_borrow_content::<T, N>(_t, pointer)();
            // SAFETY: We explicitly check for the correct number of elements,
            //   do not let the reference outlive the slice,
            //   and require exclusive access to the entire slice to mutate the chunk.
            let result: Option<&'a mut [T; N]> = Some(unsafe { &mut *pointer });
            /*@
            {
                assert result == std::option::Option::Some(?prefix);
                close array_full_borrow_content::<T, N>(_t, prefix)();
                {
                    pred Ctx() = ref_mut_end_token(prefix, pointer) &*&
                        slice_full_borrow_content::<T>(_t, data + usize_of_const(typeid(N)), length - usize_of_const(typeid(N)))();
                    produce_lem_ptr_chunk restore_full_borrow_(Ctx, array_full_borrow_content::<T, N>(_t, prefix), slice_full_borrow_content::<T>(_t, data, length))() {
                        open Ctx();
                        lemmas::restore_owned_prefix::<T, N>(_t, data, length, prefix);
                    } {
                        close Ctx();
                        close_full_borrow_strong_();
                    }
                }
                close_ref_mut_own::<'a, [T; N]>(_t, prefix);
                close std::option::Option_own::<&'a mut [T; N]>(_t, std::option::Option::Some(prefix));
            }
            @*/
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

impl<T> *mut T {
    pub const fn cast_array<const N: usize>(self) -> *mut [T; N]
//@ req true;
    //@ ens result == self as *mut [T; N];
    //@ on_unwind_ens false;
    {
        self.cast()
    }
}
