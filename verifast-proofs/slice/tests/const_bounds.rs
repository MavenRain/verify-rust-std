pub fn const_bounds<const N: usize>() -> usize {
    //@ assert 0 <= usize_of_const(typeid(N));
    //@ assert usize_of_const(typeid(N)) <= usize::MAX;
    N
}
