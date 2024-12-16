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

#[macro_export(local_inner_macros)]
macro_rules! err_handle {
    ($err_name:ident) => {
        return Err($err_name.into())
    };
    ($err_name:ident, $err_rename:ident, $handler:block) => {{
        let $err_rename = $err_name;
        $handler
    }};
}

#[macro_export]
/// Macro to select a message type and execute the corresponding handler.
macro_rules! select_msg {
    ($input:expr, {
        $(
        $msg_ty:ty = $var_name:ident => $handler:block
        ),*
    }) => {
        match $input.r#type.as_str() {
            $(<$msg_ty as $crate::OBMessage>::TYPE => {
                let $var_name: $msg_ty = $input.parse()?;
                $handler
            },)*
            _ => {
                Default::default()
            }
        }
    };
    ($input:expr, {
        $(
        $msg_ty:ty = $var_name:ident => $handler:block,
        )*
        _ => $else_block:block $(,)?
    }) => {
        match $input.r#type.as_str() {
            $(<$msg_ty as $crate::OBMessage>::TYPE => {
                let $var_name: $msg_ty = $input.parse()?;
                $handler
            },)*
            _ => $else_block,
        }
    };
    ($input:expr, {
        $(
        $msg_ty:path = $var_name:ident => $handler:block,
        )*
        _ => $else_block:block,
        Err = $err_name:ident => $err_block:block $(,)?
    }) => {
        match $input.r#type.as_str() {
            $(<$msg_ty as $crate::OBMessage>::TYPE => {
                match $input.parse::<$msg_ty>() {
                    Ok($var_name) => $handler,
                    Err($err_name) => $err_block,
                }
            },)*
            _ => $else_block,
        }
    };
    ($input:expr, {
        $(
        $msg_ty:path = $var_name:ident => $handler:block,
        )*
        Err = $err_name:ident => $err_block:block $(,)?
    }) => {
        match $input.r#type.as_str() {
            $(<$msg_ty as $crate::OBMessage>::TYPE => {
                match $input.parse::<$msg_ty>() {
                    Ok($var_name) => $handler,
                    Err($err_name) => $err_block,
                }
            },)*
            _ => {},
        }
    };
}

#[cfg(test)]
mod test {
    use crate::base::RawMessageSeg;

    #[test]
    fn test_select_msg() {
        use crate::ob12::message;
        let input: RawMessageSeg = message::MessageSeg::Text(message::Text {
            text: "sadaw".into(),
            extra: Default::default(),
        })
        .try_into()
        .unwrap();
        select_msg!(input, {
            message::Text = msg => {
                println!("{:?}", msg);
            }
        });
    }
}

pub(crate) use {define_action, trait_alias};
