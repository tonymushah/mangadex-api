macro_rules! cfg_multi_thread{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "multi-thread")]
            #[cfg_attr(docrs, doc(cfg(feature = "multi-thread")))]
            $item
        )*
    }
}

macro_rules! cfg_tokio_multi_thread{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tokio-multi-thread")]
            #[cfg_attr(docrs, doc(cfg(feature = "tokio-multi-thread")))]
            $item
        )*
    }
}

macro_rules! cfg_rw_multi_thread{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "rw-multi-thread")]
            #[cfg_attr(docrs, doc(cfg(feature = "rw-multi-thread")))]
            $item
        )*
    }
}

macro_rules! cfg_legacy_auth{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "legacy-auth")]
            #[cfg_attr(docrs, doc(cfg(feature = "legacy-auth")))]
            $item
        )*
    }
}

macro_rules! cfg_legacy_account {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "legacy-account")]
            #[cfg_attr(docrs, doc(cfg(feature = "legacy-account")))]
            $item
        )*
    }
}

macro_rules! cfg_utils{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "utils")]
            #[cfg_attr(docrs, doc(cfg(feature = "utils")))]
            $item
        )*
    }
}

macro_rules! cfg_deserializable_endpoint{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "deserializable_endpoint")]
            #[cfg_attr(docrs, doc(cfg(feature = "deserializable_endpoint")))]
            $item
        )*
    }
}

macro_rules! cfg_serialize{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "serialize")]
            #[cfg_attr(docrs, doc(cfg(feature = "serialize")))]
            $item
        )*
    }
}

macro_rules! cfg_oauth{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "oauth")]
            #[cfg_attr(docrs, doc(cfg(feature = "oauth")))]
            $item
        )*
    }
}

macro_rules! cfg_custom_list_v2{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "custom_list_v2")]
            #[cfg_attr(docrs, doc(cfg(feature = "custom_list_v2")))]
            $item
        )*
    }
}
