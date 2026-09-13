pub unsafe fn unsupported_raw_mut_slice<T>(pointer: *mut [T])
//@ req true;
//@ ens true;
//@ on_unwind_ens false;
{
    let _reference = unsafe { &mut *pointer };
}
