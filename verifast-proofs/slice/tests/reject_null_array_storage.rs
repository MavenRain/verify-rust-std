/*@
lem reject_null_array_storage<T, N>()
    req usize_of_const(typeid(N)) == 0;
    ens false;
{
    close array::<T>(0, 0, nil);
    array_to_Array::<T, N>(0 as *[T; N]);
}
@*/
