use crate::Format;
use argp::FromArgs;
use std::path::PathBuf;

/// Convert between data formats.
#[derive(FromArgs)]
pub struct Args {
    /// Show version info and exit
    #[argp(switch, short = 'V')]
    pub version: bool,

    /// Input format
    #[argp(
        option,
        short = 'f',
        arg_name = "format",
        from_str_fn(core::str::FromStr::from_str)
    )]
    pub from: Option<Format>,

    /// Output format
    #[argp(
        option,
        short = 't',
        arg_name = "format",
        from_str_fn(core::str::FromStr::from_str)
    )]
    pub into: Option<Format>,

    /// Pretty output if supported
    #[argp(switch, short = 'p')]
    pub pretty: bool,

    /// Input file ('-' to read from stdin)
    #[argp(positional)]
    pub input: PathBuf,

    /// Output file (skip to write to stdout)
    #[argp(positional)]
    pub output: Option<PathBuf>,
}

impl Args {
    pub fn from_env() -> Self {
        let mut args: Self = argp::parse_args_or_exit(argp::DEFAULT);

        if args.version {
            eprintln!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }

        if let Err(error) = args.validate() {
            eprintln!("{}\nRun --help for more information.", error);
            std::process::exit(1);
        }

        args
    }

    pub fn input_format(&self) -> &Format {
        self.from.as_ref().unwrap()
    }

    pub fn output_format(&self) -> &Format {
        self.into.as_ref().unwrap()
    }

    pub fn input_file(&self) -> Option<&PathBuf> {
        if self.input.as_os_str() == "-" {
            None
        } else {
            Some(&self.input)
        }
    }

    pub fn open_input(&self) -> Result<Box<dyn std::io::Read>, String> {
        Ok(if let Some(input) = self.input_file() {
            Box::new(
                std::fs::File::open(input)
                    .map_err(|err| format!("Error while openning input file: {err}."))?,
            ) as _
        } else {
            Box::new(std::io::stdin()) as _
        })
    }

    pub fn output_file(&self) -> Option<&PathBuf> {
        self.output.as_ref().and_then(|output| {
            if output.as_os_str() == "-" {
                None
            } else {
                Some(output)
            }
        })
    }

    pub fn open_output(&self) -> Result<Box<dyn std::io::Write>, String> {
        Ok(if let Some(output) = self.output_file() {
            Box::new(
                std::fs::File::create(output)
                    .map_err(|err| format!("Error while openning output file: {err}."))?,
            ) as _
        } else {
            Box::new(std::io::stdout()) as _
        })
    }

    pub fn validate(&mut self) -> Result<(), String> {
        if self.from.is_none() {
            if let Some(input) = self.input_file() {
                if let Some(ext) = input.extension().and_then(|ext| ext.to_str()) {
                    self.from = Some(ext.parse().map_err(|err| {
                        format!("Cannot determine input format from file extension: {err}.")
                    })?);
                } else {
                    return Err("Cannot determine input format because file extension is missing.".to_string());
                }
            } else {
                return Err("Input format has not specified and cannot be determined from file extension.".to_string());
            }
        }

        if self.into.is_none() {
            if let Some(output) = self.output_file() {
                if let Some(ext) = output.extension().and_then(|ext| ext.to_str()) {
                    self.into = Some(ext.parse().map_err(|err| {
                        format!("Cannot determine output format from file extension: {err}.")
                    })?);
                } else {
                    return Err(
                        "Cannot determine output format because file extension is missing.".to_string()
                    );
                }
            } else {
                return Err("Please specify input format (-f)".to_string());
            }
        }

        Ok(())
    }
}
