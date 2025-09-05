use copy_dir::copy_dir;
use log::info;
use roblox_install::RobloxStudio;
use std::path::{Path, PathBuf};
use std::{env::current_dir, fs::remove_dir_all};

const ROBLOX_PACKAGES_PATH: &str = "ExtraContent/LuaPackages/Packages";

pub fn install_roblox_packages(dest: &PathBuf) {
    let roblox_studio_content = RobloxStudio::locate()
        .map(|rs| rs.content_path().display().to_string())
        .unwrap();

    let roblox_studio_packages = format!("{}/../{}", roblox_studio_content, ROBLOX_PACKAGES_PATH);
    let roblox_studio_packages_path = Path::new(&roblox_studio_packages);

    let cwd = current_dir().unwrap();
    let dest_path = cwd.join(&dest);

    if roblox_studio_packages_path.exists() {
        if dest.exists() {
            info!("removing existing destination {:?}...", dest_path);
            remove_dir_all(&dest_path).unwrap();
        }

        info!("copying packages from {:?} ", roblox_studio_packages_path);
        copy_dir(roblox_studio_packages_path, &dest_path).unwrap();

        info!("successfully installed Roblox packages to {:?}", dest_path);
    }
}
