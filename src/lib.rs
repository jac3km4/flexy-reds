use red4ext_rs::prelude::*;

mod exports;
mod layout;
mod redscript;

define_plugin! {
    name: "flexy-reds",
    author: "jac3km4",
    version: 0:0:2,
    on_register: {
        register_function!("Flexy.UI.RenderElem", exports::render_elem);
        register_function!("Flexy.Layout.ParseDim", exports::parse_dimension);
    }
}
