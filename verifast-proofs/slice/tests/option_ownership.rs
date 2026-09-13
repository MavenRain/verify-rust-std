pub fn shared_some<'a, T>(value: &'a T) -> Option<&'a T> {
    let result = Some(value);
    //@ close_ref_own::<'a, T>(value);
    //@ close std::option::Option_own::<&'a T>(_t, std::option::Option::Some(value));
    result
}
