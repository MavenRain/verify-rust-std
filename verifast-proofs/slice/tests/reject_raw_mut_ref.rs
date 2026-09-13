pub unsafe fn reject_raw_mut_ref<T>(pointer: *mut T)
//@ req true;
//@ ens true;
//@ on_unwind_ens false;
{
    let _reference = unsafe { &mut *pointer };
}
