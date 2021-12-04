use red4ext_rs::prelude::*;

mod exports;
mod layout;
mod redscript;

#[ctor::ctor]
fn init() {
    rtti::on_register(exports::register, exports::post_register);
}
