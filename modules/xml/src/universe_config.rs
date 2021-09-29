#![cfg(feature = "serialize")]

//! Data returned from `UniverseConfig.svc`

use serde::{Deserialize, Serialize};

/// Information on account registration
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    /// Unknown
    #[serde(rename = "SendPasswordUrl")]
    send_password_url: String,
    /// URL to log into an account
    #[serde(rename = "SignInUrl")]
    pub sign_in_url: String,
    /// URL to sign up for an account
    #[serde(rename = "SignUpUrl")]
    pub sign_up_url: String,
}

/// Information on the game
#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    /// URL for login to subscription management
    #[serde(rename = "AuthenticationUrl")]
    pub authentication_url: String,
    /// URL for registration as a user
    #[serde(rename = "ClientUrl")]
    pub client_url: String,
    /// URL to submit crash logs
    #[serde(rename = "CrashLogUrl")]
    pub crash_log_url: String,
    /// Old launcher URL?
    #[serde(rename = "LauncherUrl")]
    pub launcher_url: String,
    /// URL of the website shown in the patcher
    #[serde(rename = "LauncherUrl2")]
    pub launcher_url2: String,
}

/// Information on downloading the game
#[derive(Debug, Deserialize, Serialize)]
pub struct PatcherInfo {
    /// Manifest of files specific to running Windows programs on Mac
    #[serde(rename = "CiderUrl")]
    pub cider_url: String,
    /// Configuration for the patching process
    #[serde(rename = "ConfigUrl")]
    pub config_url: String,
    /// URL of the installer for major updates
    #[serde(rename = "InstallUrl")]
    pub install_url: String,
}

/// Information on the CDN client
#[derive(Debug, Deserialize, Serialize)]
pub struct CdnInfo {
    /// An ID for the Akamai CDN
    #[serde(rename = "CpCode")]
    pub cp_code: u32,
    /// The patcher subdirectory
    #[serde(rename = "PatcherDir")]
    pub patcher_dir: String,
    /// The patch server
    #[serde(rename = "PatcherUrl")]
    pub patcher_url: String,
    /// Whether to use https (?)
    #[serde(rename = "Secure")]
    pub secure: bool,
    /// Whether to use the Akamai download manager
    #[serde(rename = "UseDlm")]
    pub use_dlm: bool,
}

/// A single server (*Universe*) that can be selected
#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    /// URL of the auth server
    #[serde(rename = "AuthenticationIP")]
    pub authentication_ip: String,
    /// Information for the CDN client
    #[serde(rename = "CdnInfo")]
    pub cdn_info: CdnInfo,
    /// Info for moderation (?)
    #[serde(rename = "CrispInfo")]
    pub crisp_info: String,
    /// ID of the data center
    #[serde(rename = "DataCenterId")]
    pub data_center_id: u32,
    /// URL for the game API
    #[serde(rename = "GameApiUrl")]
    pub game_api_url: String,
    /// URL for game content
    #[serde(rename = "GameContentApiUrl")]
    pub game_content_api_url: String,
    /// Language Tag of the server
    #[serde(rename = "Language")]
    pub language: String,
    /// Log level
    #[serde(rename = "LogLevel")]
    pub log_level: u32,
    /// URL of the metrics server
    #[serde(rename = "MetricsDataServiceUrl")]
    pub metrics_data_service_url: String,
    /// Display name of the server
    #[serde(rename = "Name")]
    pub name: String,
    /// Whether this server is available
    #[serde(rename = "Online")]
    pub online: bool,
    /// Whether this server is selected by default
    #[serde(rename = "Suggested")]
    pub suggested: bool,
    /// URL for the UGC controller
    #[serde(rename = "UGCControllerServicesUrl")]
    pub ugc_controller_services_url: String,
    /// CDN info for user generated content
    #[serde(rename = "UgcCdnInfo")]
    pub ugc_cdn_info: CdnInfo,
    /// Whether to use online model conversion
    #[serde(rename = "Use3DServices")]
    pub use3d_services: bool,
    /// Current version of the game
    #[serde(rename = "Version")]
    pub version: String,
    /// Type of version dir (default 0)
    #[serde(rename = "VersionDirType")]
    pub version_dir_type: String,
    /// API url (?)
    #[serde(rename = "WebApiUrl")]
    pub web_api_url: String,
}

/// The list of servers
#[derive(Debug, Deserialize, Serialize)]
pub struct Servers {
    /// The list of servers
    #[serde(rename = "Server")]
    pub servers: Vec<Server>,
}

/// The root of the `EnvironmentInfo` endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct Environment {
    /// Information on accounts
    #[serde(rename = "AccountInfo")]
    pub account_info: AccountInfo,
    /// Information on the game
    #[serde(rename = "GameInfo")]
    pub game_info: GameInfo,
    /// Information on the general patcher process
    #[serde(rename = "PatcherInfo")]
    pub patcher_info: PatcherInfo,
    /// Information on individual servers/universes
    #[serde(rename = "Servers")]
    pub servers: Servers,
}
