extern crate ralloc;

fn alloc_box() -> Box<u32> {
    Box::new(0xDEADBEAF)
}

#[test]
fn test() {
    {
        let mut a = Box::new(1);
        let mut b = Box::new(2);
        let mut c = Box::new(3);

        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
        assert_eq!(*c, 3);
        assert_eq!(*alloc_box(), 0xDEADBEAF);

        *a = 0;
        *b = 0;
        *c = 0;
        assert_eq!(*a, 0);
        assert_eq!(*b, 0);
        assert_eq!(*c, 0);
    }

    ralloc::lock().debug_assert_no_leak();
}
