mod args;
mod convert;
mod format;

use args::Args;
use format::Format;

fn main() -> Result<(), String> {
    let args = Args::from_env();

    let mut input = args.open_input()?;
    let mut output = args.open_output()?;

    let value = args.input_format().read(&mut input)?;
    args.output_format().write(args.pretty, &mut output, &value)
}
