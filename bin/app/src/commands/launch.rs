use std::path::Path;

use clap::{ArgAction, ArgMatches, Command};
use hemtt_bin_error::Error;
use hemtt_bin_project::config::Configuration;
use steamlocate::SteamDir;

use crate::utils::create_link;

#[must_use]
pub fn cli() -> Command {
    Command::new("launch")
        .about("Launch Arma 3 with your mod and dependencies")
        .arg(
            clap::Arg::new("binarize")
                .long("binarize")
                .short('b')
                .help("Use BI's binarize on supported files")
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    let config = Configuration::from_file(Path::new("hemtt.toml"))?;
    let Some(mainprefix) = config.mainprefix() else {
        return Err(Error::MainPrefixNotFound(String::from("Required for launch")));
    };
    let Some(arma3dir) = SteamDir::locate().and_then(|mut s| s.app(&107_410).map(std::borrow::ToOwned::to_owned)) else {
        return Err(Error::Arma3NotFound);
    };

    println!("Arma 3 found at: {:?}", arma3dir.path);

    let mut mods = config.hemtt().launch().parameters().to_vec();

    mods.push({
        let mut path = std::env::current_dir()?;
        path.push(".hemtt/dev");
        path.display().to_string()
    });

    // climb to the workshop folder
    if !config.hemtt().launch().mods().is_empty() {
        let Some(common) = arma3dir.path.parent() else {
            return Err(Error::WorkshopNotFound);
        };
        let Some(root) = common.parent() else {
            return Err(Error::WorkshopNotFound);
        };
        let workshop = root.join("workshop").join("content").join("107410");
        if !workshop.exists() {
            return Err(Error::WorkshopNotFound);
        };
        for load_mod in config.hemtt().launch().mods() {
            let mod_path = workshop.join(load_mod);
            if !mod_path.exists() {
                return Err(Error::WorkshopModNotFound(load_mod.to_string()));
            };
            mods.push(mod_path.display().to_string());
        }
    }

    let ctx = super::dev::execute(matches)?;

    let prefix_folder = arma3dir.path.join(mainprefix);
    if !prefix_folder.exists() {
        std::fs::create_dir_all(&prefix_folder)?;
    }

    let link = prefix_folder.join(ctx.config().prefix());
    if !link.exists() {
        create_link(
            link.display().to_string().as_str(),
            ctx.hemtt_folder().display().to_string().as_str(),
        )?;
    }

    let args = vec![format!(
        "-mod=\"{}\" -skipIntro -noSplash -showScriptErrors -debug -filePatching",
        mods.join(";")
    )];

    println!(
        "Launching {} with: {}",
        arma3dir.path.display(),
        args.join(" ")
    );

    std::process::Command::new({
        let mut path = arma3dir.path;
        path.push("arma3_x64.exe");
        path.display().to_string()
    })
    .args(args)
    .spawn()?;

    Ok(())
}
