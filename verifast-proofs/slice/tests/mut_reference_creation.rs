pub unsafe fn from_raw<'a, T>(pointer: *mut T) -> Option<&'a mut T>
//@ req thread_token(?t) &*& [?q]lifetime_token('a) &*& full_borrow('a, <T>.full_borrow_content(t, pointer)) &*& pointer != 0 &*& pointer as usize % std::mem::align_of::<T>() == 0;
//@ ens thread_token(t) &*& [q]lifetime_token('a) &*& std::option::Option_own::<&'a mut T>(t, result);
//@ on_unwind_ens false;
{
    //@ open_full_borrow_strong_('a, <T>.full_borrow_content(t, pointer));
    //@ open_full_borrow_content::<T>(t, pointer);
    let result: Option<&'a mut T> = Some(unsafe { &mut *pointer });
    /*@
    {
        assert result == std::option::Option::Some(?reference);
        close_full_borrow_content::<T>(t, reference);
        {
            pred Ctx() = ref_mut_end_token(reference, pointer);
            produce_lem_ptr_chunk restore_full_borrow_(Ctx, <T>.full_borrow_content(t, reference), <T>.full_borrow_content(t, pointer))() {
                open Ctx();
                open_full_borrow_content::<T>(t, reference);
                end_ref_mut(reference);
                close_full_borrow_content::<T>(t, pointer);
            } {
                close Ctx();
                close_full_borrow_strong_();
            }
        }
        close_ref_mut_own::<'a, T>(t, reference);
        close std::option::Option_own::<&'a mut T>(t, std::option::Option::Some(reference));
    }
    @*/
    result
}
