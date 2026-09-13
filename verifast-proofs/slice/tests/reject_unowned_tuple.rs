/*@
lem unowned_tuple<T, U>(t: thread_id_t, pair: (T, U))
    req <T>.own(t, pair.0);
    ens <(T, U)>.own(t, pair);
{
    close_tuple_2_own::<T, U>(t, pair);
}
@*/
