use clap::{App, ArgMatches};

use prelude::*;
use config::Config;

use commands;

macro_rules! each_subcommand {
    ($mac:ident) => {
        $mac!(react_native_gradle);
        $mac!(react_native_codepush);
        #[cfg(target_os="macos")]
        $mac!(react_native_xcode);
    }
}

pub fn make_app<'a, 'b: 'a>(mut app: App<'a, 'b>) -> App<'a, 'b> {
    macro_rules! add_subcommand {
        ($name:ident) => {{
            app = app.subcommand(commands::$name::make_app(
                App::new(&stringify!($name)[13..])));
        }}
    }

    app = app.about("provides helpers for react-native.");
    each_subcommand!(add_subcommand);
    app
}

pub fn execute<'a>(matches: &ArgMatches<'a>, config: &Config) -> Result<()> {
    macro_rules! execute_subcommand {
        ($name:ident) => {{
            if let Some(sub_matches) = matches.subcommand_matches(&stringify!($name)[13..]) {
                return Ok(commands::$name::execute(&sub_matches, &config)?);
            }
        }}
    }
    each_subcommand!(execute_subcommand);
    unreachable!();
}
