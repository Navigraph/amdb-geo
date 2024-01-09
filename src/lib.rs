use crate::amdb::geo_json::AirportMapData;

mod amdb;
pub mod enums;
pub mod implementations;
pub mod output_types;

pub struct Airport {
    pub aerodrome_reference_point: output_types::AerodromeReferencePoint,
    pub apron_elements: Vec<output_types::ApronElement>,
    pub painted_centerlines: Vec<output_types::PaintedCenterline>,
    pub parking_stand_areas: Vec<output_types::ParkingStandArea>,
    pub parking_stand_locations: Vec<output_types::ParkingStandLocation>,
    pub runway_displaced_areas: Vec<output_types::RunwayDisplacedArea>,
    pub runway_elements: Vec<output_types::RunwayElement>,
    pub runway_exit_lines: Vec<output_types::RunwayExitLine>,
    pub runway_markings: Vec<output_types::RunwayMarking>,
    pub runway_shoulders: Vec<output_types::RunwayShoulder>,
    pub runway_thresholds: Vec<output_types::RunwayThreshold>,
    pub stand_guidance_lines: Vec<output_types::StandGuidanceLine>,
    pub taxiway_elements: Vec<output_types::TaxiwayElement>,
    pub taxiway_guidance_lines: Vec<output_types::TaxiwayGuidanceLine>,
    pub taxiway_holding_positions: Vec<output_types::TaxiwayHoldingPosition>,
    pub taxiway_shoulders: Vec<output_types::TaxiwayShoulder>,
}

fn map_vec<T, O: From<T>>(input: Vec<T>) -> Vec<O> { input.into_iter().map(Into::into).collect() }

pub fn parse_airport(data: &str) -> Result<Airport, Box<dyn std::error::Error>> {
    let mut airport: AirportMapData = serde_json::from_str(data)?;

    let airport = Airport {
        aerodrome_reference_point: airport.aerodrome_reference_point.features.remove(0).into(),
        apron_elements: map_vec(airport.apron_element.features),
        runway_elements: map_vec(airport.runway_element.features),
        painted_centerlines: map_vec(airport.painted_centerline.features),
        parking_stand_areas: map_vec(airport.parking_stand_area.features),
        parking_stand_locations: map_vec(airport.parking_stand_location.features),
        runway_displaced_areas: map_vec(airport.runway_displaced_area.features),
        runway_exit_lines: map_vec(airport.runway_exit_line.features),
        runway_markings: map_vec(airport.runway_marking.features),
        runway_shoulders: map_vec(airport.runway_shoulder.features),
        runway_thresholds: map_vec(airport.runway_threshold.features),
        stand_guidance_lines: map_vec(airport.stand_guidance_line.features),
        taxiway_elements: map_vec(airport.taxiway_element.features),
        taxiway_guidance_lines: map_vec(airport.taxiway_guidance_line.features),
        taxiway_holding_positions: map_vec(airport.taxiway_holding_position.features),
        taxiway_shoulders: map_vec(airport.taxiway_shoulder.features),
    };

    Ok(airport)
}
