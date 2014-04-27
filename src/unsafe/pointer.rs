let raw_p: *u32 = &10;

unsafe {
    assert!(*raw_p == 10);
}