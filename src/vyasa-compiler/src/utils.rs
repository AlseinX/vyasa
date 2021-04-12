pub fn call_replace<T>(target: &mut T, func: impl FnOnce(T) -> T) {
    let target = target as *mut T;
    unsafe {
        target.write(func(target.read()));
    }
}
