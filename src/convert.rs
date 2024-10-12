use crate::Format;
use ron::Value;

impl Format {
    pub fn read(&self, input: &mut impl std::io::Read) -> Result<Value, String> {
        Ok(match self {
            Self::Rust => ron::de::from_reader(input)
                .map_err(|err| format!("Error while reading Rust object: {err}."))?,
            #[cfg(feature = "json")]
            Self::Json => json::from_reader(input)
                .map_err(|err| format!("Error while reading JSON: {err}."))?,
            #[cfg(feature = "json5")]
            Self::Json5 => from_reader(
                input,
                |s| json5::from_str(s),
                |err| format!("Error while reading JSON5: {err}."),
            )?,
            #[cfg(feature = "yaml")]
            Self::Yaml => yaml::from_reader(input)
                .map_err(|err| format!("Error while reading YAML: {err}."))?,
            #[cfg(feature = "toml")]
            Self::Toml => from_reader(input, toml::from_str, |err| {
                format!("Error while reading TOML: {err}.")
            })?,
            #[cfg(feature = "sexp")]
            Self::Sexp => sexp::from_reader(input)
                .map_err(|err| format!("Error while reading SEXP: {err}."))?,
            #[cfg(feature = "sexp")]
            Self::Elisp => sexp::from_reader_custom(input, sexp::parse::Options::elisp())
                .map_err(|err| format!("Error while reading ELISP: {err}."))?,
            #[cfg(feature = "url")]
            Self::Url => from_reader(
                input,
                |s| url::from_str(s),
                |err| format!("Error while reading URL: {err}."),
            )?,
            #[cfg(feature = "msgpack")]
            Self::MsgPack | Self::MsgPackNamed => msgpack::from_read(input)
                .map_err(|err| format!("Error while reading MessagePack: {err}."))?,
            #[cfg(feature = "cbor")]
            Self::Cbor => cbor::from_reader(input)
                .map_err(|err| format!("Error while reading CBOR: {err}."))?,
            #[cfg(feature = "bson")]
            Self::Bson => bson::from_reader(input)
                .map_err(|err| format!("Error while reading BSON: {err}."))?,
            #[cfg(feature = "bencode")]
            Self::Bencode => bencode::from_reader(input)
                .map_err(|err| format!("Error while reading BENCODE: {err}."))?,
            #[cfg(feature = "pickle")]
            Self::Pickle => pickle::de::from_reader(input, pickle::de::DeOptions::default())
                .map_err(|err| format!("Error while reading PICKLE: {err}."))?,
            #[cfg(feature = "pickle")]
            Self::Pickle2 => pickle::de::from_reader(
                input,
                pickle::de::DeOptions::default()
                    .decode_strings()
                    .replace_unresolved_globals(),
            )
            .map_err(|err| format!("Error while reading PICKLE: {err}."))?,
            #[cfg(feature = "dbus")]
            Self::DBusLe => from_reader_dbus(input, dbus::serialized::Format::DBus, dbus::LE),
            #[cfg(feature = "dbus")]
            Self::DBusBe => from_reader_dbus(input, dbus::serialized::Format::DBus, dbus::BE),
            #[cfg(feature = "dbus")]
            Self::DBusLeGVariant => {
                from_reader_dbus(input, dbus::serialized::Format::GVariant, dbus::LE)
            }
            #[cfg(feature = "dbus")]
            Self::DBusBeGVariant => {
                from_reader_dbus(input, dbus::serialized::Format::GVariant, dbus::BE)
            }
        })
    }

    pub fn write(
        &self,
        pretty: bool,
        output: &mut impl std::io::Write,
        value: &Value,
    ) -> Result<(), String> {
        match self {
            Self::Rust => if pretty {
                ron::ser::to_writer_pretty(output, value, ron::ser::PrettyConfig::default())
            } else {
                ron::ser::to_writer(output, value)
            }
            .map_err(|err| format!("Error while writing Rust object: {err}."))?,
            #[cfg(feature = "json")]
            Self::Json => if pretty {
                json::to_writer_pretty(output, value)
            } else {
                json::to_writer(output, value)
            }
            .map_err(|err| format!("Error while writing JSON: {err}."))?,
            #[cfg(feature = "json5")]
            Self::Json5 => into_writer(output, value, json5::to_string, |err| {
                format!("Error while writing JSON5: {err}.")
            })?,
            #[cfg(feature = "yaml")]
            Self::Yaml => yaml::to_writer(output, value)
                .map_err(|err| format!("Error while writing YAML: {err}."))?,
            #[cfg(feature = "toml")]
            Self::Toml => into_writer(
                output,
                value,
                if pretty {
                    toml::to_string_pretty
                } else {
                    toml::to_string
                },
                |err| format!("Error while writing TOML: {err}."),
            )?,
            #[cfg(feature = "sexp")]
            Self::Sexp => sexp::to_writer(output, value)
                .map_err(|err| format!("Error while writing SEXP: {err}."))?,
            #[cfg(feature = "sexp")]
            Self::Elisp => sexp::to_writer_custom(output, value, sexp::print::Options::elisp())
                .map_err(|err| format!("Error while writing ELISP: {err}."))?,
            #[cfg(feature = "url")]
            Self::Url => url::to_writer(value, output)
                .map_err(|err| format!("Error while writing URL: {err}."))?,
            #[cfg(feature = "msgpack")]
            Self::MsgPack => msgpack::encode::write(output, value)
                .map_err(|err| format!("Error while writing MessagePack: {err}."))?,
            Self::MsgPackNamed => msgpack::encode::write_named(output, value)
                .map_err(|err| format!("Error while writing MessagePack named: {err}."))?,
            #[cfg(feature = "cbor")]
            Self::Cbor => cbor::ser::into_writer(value, output)
                .map_err(|err| format!("Error while writing CBOR: {err}."))?,
            #[cfg(feature = "bson")]
            Self::Bson => bson::to_document(value)
                .map_err(|err| format!("Error while formatting BSON: {err}."))?
                .to_writer(output)
                .map_err(|err| format!("Error while writing BSON: {err}."))?,
            #[cfg(feature = "bencode")]
            Self::Bencode => bencode::to_writer(output, value)
                .map_err(|err| format!("Error while writing BENCODE: {err}."))?,
            #[cfg(feature = "pickle")]
            Self::Pickle => {
                pickle::ser::to_writer(output, value, pickle::ser::SerOptions::default())
                    .map_err(|err| format!("Error while writing PICKLE: {err}."))?
            }
            #[cfg(feature = "pickle")]
            Self::Pickle2 => {
                pickle::ser::to_writer(output, value, pickle::ser::SerOptions::default().proto_v2())
                    .map_err(|err| format!("Error while writing PICKLE: {err}."))?
            }
            #[cfg(feature = "dbus")]
            Self::DBusLe => {
                into_writer_dbus(output, value, dbus::serialized::Format::DBus, dbus::LE)
            }
            #[cfg(feature = "dbus")]
            Self::DBusBe => {
                into_writer_dbus(output, value, dbus::serialized::Format::DBus, dbus::BE)
            }
            #[cfg(feature = "dbus")]
            Self::DBusLeGVariant => {
                into_writer_dbus(output, value, dbus::serialized::Format::GVariant, dbus::LE)
            }
            #[cfg(feature = "dbus")]
            Self::DBusBeGVariant => {
                into_writer_dbus(output, value, dbus::serialized::Format::GVariant, dbus::BE)
            }
        }
        Ok(())
    }
}

#[cfg(any(feature = "json5", feature = "toml", feature = "url"))]
fn from_reader<T, E>(
    reader: &mut impl std::io::Read,
    from_str: fn(&str) -> Result<T, E>,
    from_err: fn(E) -> String,
) -> Result<T, String> {
    let mut buf = String::default();
    reader
        .read_to_string(&mut buf)
        .map_err(|err| format!("Error while reading input: {err}."))?;
    from_str(&buf).map_err(from_err)
}

#[cfg(any(feature = "json5", feature = "toml"))]
fn into_writer<T, E>(
    writer: &mut impl std::io::Write,
    value: &T,
    to_string: fn(&T) -> Result<String, E>,
    from_err: fn(E) -> String,
) -> Result<(), String> {
    let buf = to_string(value).map_err(from_err)?;
    writer
        .write_all(buf.as_bytes())
        .map_err(|err| format!("Error while writing output: {err}."))
}

#[cfg(feature = "dbus")]
fn from_reader_dbus<'de, T: serde::Deserialize<'de> + dbus::Type>(
    reader: &mut impl std::io::Read,
    format: dbus::serialized::Format,
    endian: dbus::Endian,
) -> Result<T, String> {
    let ctx = dbus::serialized::Context::new(format, endian, 0);
    let mut buf = Vec::default();
    reader
        .read_to_end(&mut buf)
        .map_err(|err| format!("Error while reading input: {err}."))?;
    let cur = std::io::Cursor::new(buf);
    let data = dbus::serialized::Data::new(cur.get_ref(), ctx);
    Ok(data
        .deserialize()
        .map_err(|err| format!("Error while reading DBus: {err}."))?
        .0)
}

#[cfg(feature = "dbus")]
fn into_writer_dbus<'de, T: serde::Serialize + dbus::Type>(
    writer: &mut impl std::io::Write,
    value: &T,
    format: dbus::serialized::Format,
    endian: dbus::Endian,
) -> Result<T, String> {
    let ctx = dbus::serialized::Context::new(format, endian, 0);
    dbus::to_writer(writer, ctx, value).map_err(|err| format!("Error while writing DBus: {err}."))
}
