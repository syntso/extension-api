#[macro_export]
macro_rules! register_extension {
    ($extension:ty) => {
        type SyntExtensionAdapter = $crate::Adapter<$extension>;

        $crate::bindings::export!(SyntExtensionAdapter with_types_in $crate::bindings);
    };
}
