macro_rules! cfg_legacy_auth{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "legacy-auth")]
            #[cfg_attr(docsrs, doc(cfg(feature = "legacy-auth")))]
            $item
        )*
    }
}

macro_rules! cfg_utils{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "utils")]
            #[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
            $item
        )*
    }
}

macro_rules! cfg_oauth{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "oauth")]
            #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
            $item
        )*
    }
}

macro_rules! cfg_custom_list_v2{
    ($($item:item)*) => {
        $(
            #[cfg(feature = "custom_list_v2")]
            #[cfg_attr(docsrs, doc(cfg(feature = "custom_list_v2")))]
            $item
        )*
    }
}
