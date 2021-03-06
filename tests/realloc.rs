extern crate ralloc;

#[test]
fn test() {
    {
        let mut vec = Vec::new();

        vec.reserve(1);
        vec.reserve(2);
        vec.reserve(3);
        vec.reserve(100);
        vec.reserve(600);
        vec.reserve(1000);
        vec.reserve(2000);

        vec.push(1);
        vec.push(2);
    }

    ralloc::lock().debug_assert_no_leak();
}
