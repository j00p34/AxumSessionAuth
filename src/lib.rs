#![doc = include_str!("../README.md")]
#[cfg(not(any(feature = "postgres", feature = "mysql", feature = "sqlite",)))]
compile_error!("one of the features ['postgres', 'mysql', 'sqlite'] must be enabled");

#[cfg(any(
    all(feature = "postgres", feature = "mysql"),
    all(feature = "postgres", feature = "sqlite"),
    all(feature = "mysql", feature = "sqlite"),
))]
compile_error!("only one of ['postgres', 'mysql', 'sqlite'] can be enabled");

mod auth;
///This Library Requires that AxumDatabaseSessions is used as an active layer.
mod layer;
mod service;
mod session;

pub use auth::{Auth, HasPermission, Rights};
pub use layer::AuthSessionLayer;
pub use service::AuthSessionService;
pub use session::{AuthSession, Authentication};
