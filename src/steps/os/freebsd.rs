use crate::command::CommandExt;
use crate::execution_context::ExecutionContext;
use crate::sudo::Sudo;
use crate::terminal::print_separator;
use crate::utils::require_option;
use crate::Step;
use color_eyre::eyre::Result;
use std::process::Command;

pub fn upgrade_freebsd(ctx: &ExecutionContext) -> Result<()> {
    let sudo = require_option(ctx.sudo().as_ref(), String::from("No sudo detected"))?;
    print_separator("FreeBSD Update");
    ctx.run_type()
        .execute(sudo)
        .args(["/usr/sbin/freebsd-update", "fetch", "install"])
        .status_checked()
}

pub fn upgrade_packages(ctx: &ExecutionContext) -> Result<()> {
    let sudo = require_option(ctx.sudo().as_ref(), String::from("No sudo detected"))?;
    print_separator("FreeBSD Packages");

    let mut command = ctx.run_type().execute(sudo);

    command.args(["/usr/sbin/pkg", "upgrade"]);
    if ctx.config().yes(Step::System) {
        command.arg("-y");
    }
    command.status_checked()
}

pub fn audit_packages(sudo: Option<&Sudo>) -> Result<()> {
    if let Some(sudo) = sudo {
        println!();
        Command::new(sudo)
            .args(["/usr/sbin/pkg", "audit", "-Fr"])
            .status_checked()?;
    }
    Ok(())
}
