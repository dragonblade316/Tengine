macro_rules! unsafe_singleton {
    ($type: ty) => {
        static mut INSTANCE: *mut $type = 0 as *mut $type;

        impl $type {
            pub fn set_instance(instance: Self) {
                unsafe {
                    INSTANCE = std::mem::transmute(Box::new(instance));
                }
            }

            pub fn get_instance() -> &'static mut Self {
                unsafe {
                    assert!(INSTANCE != 0 as *mut Self, "tengine is not initialized");
                    &mut *INSTANCE
                }
            }
        }
    };
}
