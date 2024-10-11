macro_rules! format_impl {
    (
        $( $(#[$($meta:meta)*])* $format:ident $($name:literal)*, )*
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Format {
            $( $(#[$($meta)*])* $format, )*
        }

        impl Format {
            pub const ALL: ListOf<'static, Format> = ListOf(&[
                $( $(#[$($meta)*])* Self::$format, )*
            ]);
        }

        impl core::str::FromStr for Format {
            type Err = String;
            fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
                Ok(match s {
                    $( $(#[$($meta)*])* $($name)|* => Self::$format,)*
                    _ => return Err(format!("Unknown format {s}. Supported formats: {}", Format::ALL)),
                })
            }
        }

        impl AsRef<str> for Format {
            fn as_ref(&self) -> &str {
                match self {
                    $( $(#[$($meta)*])* Self::$format => format_impl!(@firstname $($name)*), )*
                }
            }
        }

        impl core::fmt::Display for Format {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.as_ref().fmt(f)
            }
        }
    };

    (@firstname $name:literal $($other:literal)*) => { $name };
}

format_impl! {
    Rust "rust" "ron" "r",
    #[cfg(feature = "json")]
    Json "json" "j",
    #[cfg(feature = "json5")]
    Json5 "json5" "J" "5",
    #[cfg(feature = "yaml")]
    Yaml "yaml" "y",
    #[cfg(feature = "toml")]
    Toml "toml" "t",
    #[cfg(feature = "sexp")]
    Sexp "sexp" "lisp" "scheme" "s",
    #[cfg(feature = "sexp")]
    Elisp "elisp" "el" "e",
    #[cfg(feature = "url")]
    Url "url" "u",
    #[cfg(feature = "msgpack")]
    MsgPack "msgpack" "mp" "m",
    #[cfg(feature = "msgpack")]
    MsgPackNamed "msgpack-named" "mpn" "n",
    #[cfg(feature = "cbor")]
    Cbor "cbor" "c",
    #[cfg(feature = "bson")]
    Bson "bson" "b",
    #[cfg(feature = "bencode")]
    Bencode "bencode" "B",
    #[cfg(feature = "pickle")]
    Pickle "pickle" "p",
    #[cfg(feature = "pickle")]
    Pickle2 "pickle2" "P",
    #[cfg(feature = "dbus")]
    DBusLe "dbus-le" "d",
    #[cfg(feature = "dbus")]
    DBusBe "dbus-be" "D",
    #[cfg(feature = "dbus")]
    DBusLeGVariant "dbus-le-gvariant" "g",
    #[cfg(feature = "dbus")]
    DBusBeGVariant "dbus-be-gvariant" "G",
}

pub struct ListOf<'a, T>(&'a [T]);

impl<'a, T> core::ops::Deref for ListOf<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> core::fmt::Display for ListOf<'a, T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0.is_empty() {
            return "<none>".fmt(f);
        }
        self.0[0].fmt(f)?;
        if self.0.len() > 1 {
            for x in &self.0[1..self.0.len() - 1] {
                ", ".fmt(f)?;
                x.fmt(f)?;
            }
            " and ".fmt(f)?;
            self.0[self.0.len() - 1].fmt(f)?;
        }
        Ok(())
    }
}
