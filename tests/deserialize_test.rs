use strasbourgpark::api::{LocationOpenData, OpenDataResponse, StatusOpenData};

#[test]
fn deserialize_location() {
    let content = include_str!("samples/data.strasbourg.eu_locations.json");

    let value: OpenDataResponse<LocationOpenData> =
        serde_json::from_str(content).expect("Should parse correctly");
    assert_eq!(
        "b6585e4c0238ccde0a5ae0dc4e3f654fbc2d0ad1",
        value.records[0].id
    );
}

#[test]
fn deserialize_status() {
    let content = include_str!("samples/data.strasbourg.eu_status.json");

    let value: OpenDataResponse<StatusOpenData> =
        serde_json::from_str(content).expect("Should parse correctly");
    assert_eq!(
        "614bdf0f7be2d406ab7dde7fab6165745bf8d836",
        value.records[0].id
    );
}

#[test]
fn deserialize_bad_payload() {
    let content = include_str!("samples/status_error.json");

    let value: OpenDataResponse<StatusOpenData> =
        serde_json::from_str(content).expect("should take errors into account");

    assert_eq!(29, value.records.len());
}
