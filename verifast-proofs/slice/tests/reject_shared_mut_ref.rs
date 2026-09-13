pub unsafe fn reject_shared_mut_ref<T>(pointer: *mut T)
//@ req [1/2](*pointer |-> ?value);
//@ ens true;
//@ on_unwind_ens false;
{
    let _reference = unsafe { &mut *pointer };
}
