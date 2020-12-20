use serde::Deserialize;

/// The url of the location endpoint
pub const LOCATION_URL: &'static str =
    "https://data.strasbourg.eu/api/records/1.0/search/?dataset=parkings";

/// The url of the status endpoint
pub const STATUS_URL: &'static str =
    "https://data.strasbourg.eu/api/records/1.0/search/?dataset=occupation-parkings-temps-reel";

mod deserialize {

    use std::marker::PhantomData;

    use super::{Location, Record};
    use serde::de::{IgnoredAny, MapAccess, SeqAccess, Visitor};
    use serde::{Deserialize, Deserializer};

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for super::Record<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            enum Field {
                Id,
                Fields,
                Unknown,
            }

            impl<'de> Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl<'de> Visitor<'de> for FieldVisitor {
                        type Value = Field;

                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter,
                        ) -> std::fmt::Result {
                            formatter.write_str("`recordid` or `fields`")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "recordid" => Ok(Field::Id),
                                "fields" => Ok(Field::Fields),
                                _ => Ok(Field::Unknown),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }

            struct RecordVisitor<T> {
                marker: PhantomData<T>,
            }

            impl<'de, T> Visitor<'de> for RecordVisitor<T>
            where
                T: Deserialize<'de>,
            {
                type Value = Record<T>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("expecting a map size length array")
                }
                fn visit_map<V>(self, mut map: V) -> Result<Record<T>, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut ids = None;
                    let mut fields = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Id => {
                                if ids.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recordsid"));
                                }
                                ids = Some(map.next_value()?);
                            }
                            Field::Fields => {
                                if fields.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fields"));
                                }
                                fields = Some(map.next_value()?);
                            }
                            _ => {
                                let _elem = map.next_value::<IgnoredAny>()?;
                            }
                        }
                    }

                    let ids = ids.ok_or_else(|| serde::de::Error::missing_field("recordsid"))?;
                    let fields = fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?;
                    Ok(Record {
                        id: ids,
                        inner: fields,
                    })
                }
            }

            const FIELDS: &'static [&'static str] = &["idsurfs", "fields"];
            deserializer.deserialize_struct(
                "Record",
                FIELDS,
                RecordVisitor {
                    marker: PhantomData::default(),
                },
            )
        }
    }

    pub(super) fn failed_records<'de, D, T>(deserializer: D) -> Result<Vec<Record<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let elements: Vec<Option<Record<T>>> = Deserialize::deserialize(deserializer)?;

        let elements: Vec<Record<T>> = elements.into_iter().filter_map(|e| e).collect();
        Ok(elements)
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
#[derive(Debug)]
pub struct Record<T> {
    /// The identifier of the record
    pub id: String,

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

    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    #[serde(deserialize_with = "deserialize::failed_records")]
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

#[derive(Debug, Deserialize)]
pub struct StatusOpenData {
    #[serde(rename(deserialize = "idsurfs"))]
    pub id: String,

    #[serde(rename(deserialize = "nom_parking"))]
    pub name: String,

    #[serde(rename(deserialize = "etat"))]
    pub status: i8,

    #[serde(rename(deserialize = "libre"))]
    pub free: u16,

    pub total: u16,

    #[serde(rename(deserialize = "etat_descriptif"))]
    pub users_info: Option<String>,
}
