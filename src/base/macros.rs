#[macro_export]
macro_rules! define_action {
    {$($it:item)*} => {
        $(
            #[ob_types_macro::__data]
            #[derive(ob_types_macro::OBAction)]
            #[__oba_crate_path(crate)]
            $it
        )*
    };
}
