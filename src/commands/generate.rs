use crate::Shell;
use crate::error::Error;
use clap::CommandFactory;
use std::io;

pub fn run(shell: Shell) -> Result<(), Error> {
    let mut cmd = crate::Cli::command();
    let bin_name = cmd.get_name().to_string();

    match shell {
        Shell::Bash => {
            clap_complete::generate(
                clap_complete::shells::Bash,
                &mut cmd,
                bin_name,
                &mut io::stdout(),
            );
        }
        Shell::Zsh => {
            clap_complete::generate(
                clap_complete::shells::Zsh,
                &mut cmd,
                bin_name,
                &mut io::stdout(),
            );
        }
        Shell::Fish => {
            clap_complete::generate(
                clap_complete::shells::Fish,
                &mut cmd,
                bin_name,
                &mut io::stdout(),
            );
        }
        Shell::Powershell => {
            clap_complete::generate(
                clap_complete::shells::PowerShell,
                &mut cmd,
                bin_name,
                &mut io::stdout(),
            );
        }
    }
    Ok(())
}
