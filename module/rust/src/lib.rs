mod api;
mod binding;
#[doc(hidden)]
pub mod macros;
mod module;

pub use api::ZygiskApi;
pub use binding::{AppSpecializeArgs, ServerSpecializeArgs, StateFlags, ZygiskOption, API_VERSION};
pub use module::ZygiskModule;

mod test {
    use std::os::unix::io::RawFd;

    struct DummyModule;
    impl crate::ZygiskModule for DummyModule {}
    static MODULE: DummyModule = DummyModule;
    crate::zygisk_module!(&MODULE);

    fn companion(_socket: RawFd) {}
    crate::zygisk_companion!(companion);
}
