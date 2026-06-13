use std::path::PathBuf;

const APP_QUALIFIER: &str = "";
const APP_ORGANIZATION: &str = "";
const APP_APPLICATION: &str = "arctic-spa-dc-rust";

pub const DB_DEFAULT_FILE_NAME: &str = "asdc.db";
pub const CONFIG_DEFAULT_FILE_NAME: &str = "config.json";

pub fn default_proj_dir() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_APPLICATION)
}

fn proj_dir_or_fallback() -> directories::ProjectDirs {
    default_proj_dir().unwrap_or_else(|| {
        log::warn!("could not determine project directory, falling back to current directory");
        directories::ProjectDirs::from("", "", "").expect("fallback project dir should always resolve")
    })
}

pub fn default_data_dir() -> PathBuf {
    proj_dir_or_fallback().data_dir().to_path_buf()
}
pub fn default_config_dir() -> PathBuf {
    proj_dir_or_fallback().config_dir().to_path_buf()
}

pub fn default_database_path() -> PathBuf {
    default_data_dir().join(DB_DEFAULT_FILE_NAME)
}
pub fn default_config_path() -> PathBuf {
    default_config_dir().join(CONFIG_DEFAULT_FILE_NAME)
}

pub fn initialize_path(path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let is_new_file = !path.exists();

    // create directory if it doesn't exist
    let path_dir = path.parent().ok_or_else(|| format!("invalid path: {:?}", path))?;
    if !path_dir.exists() {
        log::debug!("creating directory: {:#?}", path_dir.display());
        std::fs::create_dir_all(path_dir)?;
    }

    // create file if it doesn't exist
    if !path.exists() {
        log::debug!("file not found at {:#?}, creating empty file", path.display());

        let file = std::fs::File::create(&path)?;

        log::info!("created file {:#?}, size={}", path.display(), file.metadata()?.len());
    }

    log::trace!("initialized path {:#?} (new file: {})", path.display(), is_new_file);

    Ok(is_new_file)
}
