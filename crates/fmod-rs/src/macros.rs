macro_rules! static_assert {
    ($cond:expr $(,)?) => {
        #[allow(deprecated)]
        #[allow(clippy::manual_range_contains)]
        const _: () = { ::std::assert!($cond) };
    };
    ($cond:expr, $msg:expr $(,)?) => {
        #[allow(deprecated)]
        #[allow(clippy::manual_range_contains)]
        const _: () = { ::std::assert!($cond, $msg) };
    };
}

macro_rules! yeet {
    ($err:expr) => {
        return Err($err)?
    };
}

#[cold]
#[track_caller]
pub(crate) fn index_out_of_bounds(ix: impl ::std::fmt::Display) -> ! {
    panic!("index out of bounds: {ix} is not a valid index");
}

macro_rules! ix {
    ($ix:expr) => {
        match $ix {
            ix => match usize::try_from(ix) {
                Ok(ix) => ix,
                Err(_) => ::fmod::macros::index_out_of_bounds(ix),
            },
        }
    };
}

macro_rules! doc_callout {
    ($text0:literal $(, $text1:literal)* $(,)?) => {
        concat!(
            "<div class=\"warning fmod-rs-callout\">\n\n",
            $text0,
            $(" ", $text1,)*
            "\n\n</div>",
        )
    };
}

macro_rules! whoops {
    {
        no_panic: $($args:tt)*
    } => {{
        log!(error, $($args)*);
    }};
    ($($args:tt)*) => {{
        // NB: assume panics get logged, don't double log
        if cfg!(debug_assertions) && !::std::thread::panicking() {
            panic!($($args)*);
        }
        whoops!(no_panic: $($args)*);
    }};
}

macro_rules! assert_unsafe_precondition {
    ($message:expr, ($($name:ident:$ty:ty = $arg:expr),*$(,)?) => $e:expr $(,)?) => {
        ::std::cfg_select! {
            feature = "unstable_ub_checks" => {
                ::core::assert_unsafe_precondition! { check_library_ub, $message, ($($name:$ty = $arg),*) => $e }
            }
            _ => {{
                #[inline]
                #[track_caller]
                const extern "C" fn precondition_check($($name:ty),*) {
                    if !($e) {
                        panic!(concat!("unsafe precondition(s) violated: ", $message,
                            "\n\nThis indicates a bug in the program. \
                            This Undefined Behavior check is optional, and cannot be relied on for safety."));
                    }
                }
                if cfg!(debug_assertions) {
                    precondition_check($($arg),*);
                }
            }}
        }
    }
}

macro_rules! log {
    ($level:ident, $fmt:expr $(, $($args:tt)*)?) => {
        ::std::cfg_select! {
            feature = "log" => {{
                ::log::$level!($fmt $(, $($args)*)?);
            }}
            _ => {{
                let _ = format_args!($fmt $(, $($args)*)?);
            }}
        }
    };
}

macro_rules! raw {
    ($(#[$meta:meta])* pub $($tt:tt)*) => {
        ::std::cfg_select! {
            feature = "raw" => {
                #[allow(clippy::missing_safety_doc, missing_docs)]
                #[cfg_attr(feature = "unstable_doc_cfg", doc(cfg(feature = "raw")))]
                $(#[$meta])* pub $($tt)*
            }
            _ => {
                #[allow(clippy::missing_safety_doc, unused, missing_docs)]
                $(#[$meta])* pub(crate) $($tt)*
            }
        }
    };
    ($mac:ident! { $(#[$meta:meta])* pub $($tt:tt)* }) => {
        ::std::cfg_select! {
            feature = "raw" => {
                $mac! {
                    #[allow(clippy::missing_safety_doc, missing_docs)]
                    #[cfg_attr(feature = "unstable_doc_cfg", doc(cfg(feature = "raw")))]
                    $(#[$meta])* pub $($tt)*
                }
            }
            _ => {
                $mac! {
                    #[allow(clippy::missing_safety_doc, unused, missing_docs)]
                    $(#[$meta])* pub(crate) $($tt)*
                }
            }
        }
    };
}

macro_rules! ffi {
    ($e:expr) => {{
        #[allow(unused_unsafe)]
        ::fmod::Error::from_raw(unsafe { $e })
    }};
}

#[macro_use]
mod flags;
#[macro_use]
mod class;
#[macro_use]
mod strict_enum;
#[macro_use]
mod typedef_enum;
#[macro_use]
mod structs;

#[allow(clippy::missing_safety_doc)]
unsafe trait TEq<U: ?Sized> {}
unsafe impl<T: ?Sized> TEq<T> for T {}

#[allow(dead_code, private_bounds)]
pub(crate) const fn assert_type_eq<A: TEq<B>, B>() {}

macro_rules! assert_type_eq {
    ($A:ty, $B:ty) => {
        const _: () = ::fmod::macros::assert_type_eq::<$A, $B>();
    };
}
