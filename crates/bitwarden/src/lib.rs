//! # Bitwarden
//!
//! A Rust client SDK to interact with the Bitwarden Secrets Manager.
//! This is a beta release and might be missing some functionality.
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```ini
//! [dependencies]
//! bitwarden = { "*", features = ["secrets"] }
//! ```
//!
//! # Basic setup
//!
//! All operations in this crate are done via a [Client]:
//!
//! ```rust
//! use bitwarden::{
//!     auth::login::AccessTokenLoginRequest,
//!     client::client_settings::{ClientSettings, DeviceType},
//!     error::Result,
//!     secrets_manager::secrets::SecretIdentifiersRequest,
//!     Client,
//! };
//! use uuid::Uuid;
//!
//! async fn test() -> Result<()> {
//!     // Use the default values
//!     let mut client = Client::new(None);
//!
//!     // Or set your own values
//!     let settings = ClientSettings {
//!         identity_url: "https://identity.bitwarden.com".to_string(),
//!         api_url: "https://api.bitwarden.com".to_string(),
//!         user_agent: "Bitwarden Rust-SDK".to_string(),
//!         device_type: DeviceType::SDK,
//!     };
//!     let mut client = Client::new(Some(settings));
//!
//!     // Before we operate, we need to authenticate with a token
//!     let token = AccessTokenLoginRequest { access_token: String::from(""), state_file: None };
//!     client.auth().login_access_token(&token).await.unwrap();
//!
//!     let org_id = SecretIdentifiersRequest { organization_id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() };
//!     println!("Stored secrets: {:#?}", client.secrets().list(&org_id).await.unwrap());
//!     Ok(())
//! }
//! ```
//!

#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

#[cfg(feature = "internal")]
pub mod admin_console;
pub mod auth;
pub mod client;
pub mod error;
#[cfg(feature = "mobile")]
pub mod mobile;
#[cfg(feature = "internal")]
pub mod platform;
#[cfg(feature = "secrets")]
pub mod secrets_manager;
#[cfg(feature = "mobile")]
pub mod tool;
#[cfg(feature = "mobile")]
pub(crate) mod uniffi_support;
mod util;
#[cfg(feature = "internal")]
pub mod vault;

pub use client::Client;

// Ensure the readme docs compile
#[doc = include_str!("../README.md")]
mod readme {}
