pub unsafe fn reject_null_array_ref<T, const N: usize>(pointer: *mut [T; N])
//@ req *pointer |-> ?array &*& pointer == 0;
//@ ens true;
//@ on_unwind_ens false;
{
    let _reference = unsafe { &mut *pointer };
}
