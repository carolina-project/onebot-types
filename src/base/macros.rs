macro_rules! define_action {
    {$(
        #[resp($resp:ty)]
        $it:item
    )*} => {
        $(
            #[ob_types_macro::__data]
            #[derive(ob_types_macro::OBAction)]
            #[action(__crate_path = crate, resp = $resp)]
            $it
        )*
    };
}

macro_rules! trait_alias {
    ($vis:vis $alias:ident($trait0:path $(, $traits:path)*)) => {
        $vis trait $alias: $trait0 $(+ $traits)* {}
        impl<T: $trait0 $(+ $traits)*> $alias for T {}
    };
}

pub(crate) use {define_action, trait_alias};
