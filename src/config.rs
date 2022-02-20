pub const CONFIG_PATH: &str = "miller";
pub const ARMA_CONFIG_PATH: &str = "arma_config.json";

pub trait Config {
    fn new() -> Self;

    // Opens config from configuration file
    fn from_file() -> Result<Self, String> where Self: Sized;

    // Save config
    fn save(&self) -> Result<(), std::io::Error>;

    // Validates the config and asks the user for input if there is any issue
    fn validate() -> Result<(), std::io::Error>;
}
