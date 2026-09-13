pub unsafe fn reject_raw_mut_ref<T>(pointer: *mut T)
//@ req pointer != 0 &*& pointer as usize % std::mem::align_of::<T>() == 0;
//@ ens true;
//@ on_unwind_ens false;
{
    let _reference = unsafe { &mut *pointer };
}
