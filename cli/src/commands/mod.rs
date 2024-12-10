mod node;
pub use node::*;

use anstyle::{AnsiColor, Color, Style};
use anyhow::Result;
use clap::{builder::Styles, Parser};

const PRIMARY_COLOR: Option<Color> = Some(Color::Ansi(AnsiColor::Blue));
// const SECONDARY_COLOR: Option<Color> = Some(Color::Ansi(AnsiColor::Cyan));
const ERROR_COLOR: Option<Color> = Some(Color::Ansi(AnsiColor::Red));
const STYLES: Styles = Styles::plain()
    .header(Style::new().bold().fg_color(PRIMARY_COLOR))
    .usage(Style::new().bold().fg_color(PRIMARY_COLOR))
    .error(Style::new().bold().fg_color(ERROR_COLOR))
    .literal(Style::new().bold());

#[derive(Debug, Parser)]
#[clap(version, about, author, styles = STYLES)]
pub struct Cli {
    /// Verbosity level [options: 0, 1, 2, 3]
    #[clap(default_value = "2", short, long)]
    pub verbosity: u8,
    /// Specify a subcommand
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[clap(name = "node")]
    Node(Node),
}

impl Command {
    /// Parses the command
    pub fn parse(self) -> Result<String> {
        match self {
            Self::Node(command) => command.parse(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sanity check as official clap recommendation
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
