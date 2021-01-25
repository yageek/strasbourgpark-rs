use serde::Deserialize;

/// The url of the location endpoint
pub const LOCATION_URL: &'static str =
    "https://data.strasbourg.eu/api/records/1.0/search/?dataset=parkings";

/// The url of the status endpoint
pub const STATUS_URL: &'static str =
    "https://data.strasbourg.eu/api/records/1.0/search/?dataset=occupation-parkings-temps-reel";
pub trait TestableRecord {
    fn is_valid(&self) -> bool;
}
mod deserialize {

    use super::{Location, Record};
    use serde::de::{SeqAccess, Visitor};
    use serde::{Deserialize, Deserializer};

    pub(super) fn deserialize_record<'de, D, T>(deserializer: D) -> Result<Vec<Record<T>>, D::Error>
    where
        T: Deserialize<'de> + super::TestableRecord,
        D: Deserializer<'de>,
    {
        let elements: Vec<Record<T>> = Deserialize::deserialize(deserializer)?;
        Ok(elements
            .into_iter()
            .filter(|r| r.inner.is_valid())
            .collect())
    }

    pub(super) fn position_to_location<'de, D>(deserializer: D) -> Result<Location, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PositionScalarVisitor;
        impl<'de> Visitor<'de> for PositionScalarVisitor {
            type Value = Location;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting a 2 size length array")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Location, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let latitude = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("no latitude"))?;

                let longitude = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("no longitude"))?;

                Ok(Location {
                    latitude,
                    longitude,
                })
            }
        }

        let visitor = PositionScalarVisitor;
        deserializer.deserialize_seq(visitor)
    }

    pub(super) fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: i8 = i8::deserialize(deserializer)?;
        if value > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// A record represents an item of some data
/// with a specific id.
#[derive(Debug, Deserialize)]
pub struct Record<T> {
    /// The identifier of the record
    #[serde(rename(deserialize = "recordid"))]
    pub id: String,

    #[serde(rename(deserialize = "fields"))]
    pub(crate) inner: T,
}

/// The container for the API response
#[derive(Debug, Deserialize)]
pub struct OpenDataResponse<T> {
    /// The parameters relative to the response
    pub parameters: Parameters,

    /// The parameters relative to the pagination
    #[serde(flatten)]
    pub pagination: Pagination,

    /// The sets of records inside the response
    #[serde(bound = "T: TestableRecord + Deserialize<'de>")]
    #[serde(deserialize_with = "deserialize::deserialize_record")]
    pub records: Vec<Record<T>>,
}
/// A struct holding the information relative
/// to the pagination
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(rename(deserialize = "nhits"))]
    pub total: i32,
    #[serde(default)]
    pub start: i32,
}
#[derive(Debug, Deserialize)]
pub struct Parameters {
    pub timezone: String,
    #[serde(rename(deserialize = "rows"))]
    pub count: i32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct LocationOpenData {
    #[serde(rename(deserialize = "idsurfs"))]
    pub id: Option<String>,

    pub city: String,

    pub zipcode: String,
    pub street: String,
    pub address: String,
    #[serde(rename(deserialize = "position"))]
    #[serde(deserialize_with = "deserialize::position_to_location")]
    pub location: Location,

    #[serde(rename(deserialize = "friendlyurl"))]
    pub url: String,
    pub name: String,
    pub description: Option<String>,

    #[serde(rename(deserialize = "accessfordeaf"))]
    #[serde(deserialize_with = "deserialize::int_to_bool")]
    pub deaf_access: bool,

    #[serde(rename(deserialize = "accessfordeficient"))]
    #[serde(deserialize_with = "deserialize::int_to_bool")]
    pub deficient_access: bool,

    #[serde(rename(deserialize = "accessforelder"))]
    #[serde(deserialize_with = "deserialize::int_to_bool")]
    pub elder_access: bool,

    #[serde(rename(deserialize = "accessforwheelchair"))]
    #[serde(deserialize_with = "deserialize::int_to_bool")]
    pub wheelchair_access: bool,

    #[serde(rename(deserialize = "accessforblind"))]
    #[serde(deserialize_with = "deserialize::int_to_bool")]
    pub blind_access: bool,
}

impl TestableRecord for LocationOpenData {
    fn is_valid(&self) -> bool {
        true
    }
}
#[derive(Debug, Deserialize)]
pub struct StatusOpenData {
    #[serde(rename(deserialize = "idsurfs"))]
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    #[serde(rename(deserialize = "nom_parking"))]
    pub name: String,

    #[serde(rename(deserialize = "etat"))]
    pub status: i8,

    #[serde(rename(deserialize = "libre"))]
    pub free: u16,

    pub total: u16,

    #[serde(default)]
    #[serde(rename(deserialize = "etat_descriptif"))]
    pub users_info: Option<String>,
}

impl TestableRecord for StatusOpenData {
    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }
}
