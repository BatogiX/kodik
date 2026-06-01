mod api_v1;
mod api_v2;
mod error;
mod graphql;
mod related;
mod shared;

pub use api_v1::{ShikiApiAnimes, ShikiApiUsersWhoami};
pub use api_v2::{ShikiApiUserRates, UserRatesTargetType};
pub use error::{Error, Result};
pub use graphql::Related;
pub use related::fetch_not_anime_ids;
pub(crate) use shared::deserialize_usize_from_string_or_number;
pub use shared::{AnimeKind, AnimeStatus, UserRate, UserRateStatus};
