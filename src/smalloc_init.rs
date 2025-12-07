use smalloc::Smalloc;
#[global_allocator]
static GLOBAL: Smalloc = Smalloc::new();

#[allow(unsafe_code)]

#[ctor::ctor]
unsafe fn init_smalloc() {
    #[cfg(feature = "smalloc")]
    GLOBAL.init();
}
