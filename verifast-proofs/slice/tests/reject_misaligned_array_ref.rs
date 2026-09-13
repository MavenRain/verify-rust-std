pub unsafe fn reject_misaligned_array_ref<T, const N: usize>(pointer: *mut [T; N])
//@ req *pointer |-> ?array &*& pointer != 0 &*& pointer as usize % std::mem::align_of::<[T; N]>() != 0;
//@ ens true;
//@ on_unwind_ens false;
{
    let _reference = unsafe { &mut *pointer };
}
