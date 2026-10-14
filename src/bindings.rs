#![allow(clippy::same_length_and_capacity, clippy::too_many_arguments)]

wit_bindgen::generate!({
    path: "wit/0.2",
    world: "extension",
    pub_export_macro: true,
    export_macro_name: "export",
    default_bindings_module: "synt_extension_api::bindings",
    runtime_path: "synt_extension_api::bindings::rt",
});

pub use wit_bindgen::rt;
