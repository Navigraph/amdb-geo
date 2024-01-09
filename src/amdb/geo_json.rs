use std::hash::Hasher;

use serde::Deserialize;

use super::layers::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

impl std::hash::Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lat.to_bits().hash(state);
        self.lon.to_bits().hash(state);
    }
}

impl Eq for Coordinate {}

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Coordinate, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Lat and lon are reversed in the GeoJSON data
        let v: Vec<f64> = Deserialize::deserialize(deserializer)?;
        Ok(Coordinate { lat: v[1], lon: v[0] })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FeatureCollection<T> {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "features")]
    pub features: Vec<T>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Geometry<T> {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "coordinates")]
    pub coordinates: T,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Point<T> {
    #[serde(rename = "geometry")]
    pub geometry: Geometry<Coordinate>,
    #[serde(rename = "properties")]
    pub properties: T,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Polygon<T> {
    #[serde(rename = "geometry")]
    pub geometry: Geometry<Vec<Vec<Coordinate>>>,
    #[serde(rename = "properties")]
    pub properties: T,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LineString<T> {
    #[serde(rename = "geometry")]
    pub geometry: Geometry<Vec<Coordinate>>,
    #[serde(rename = "properties")]
    pub properties: T,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AirportMapData {
    #[serde(rename = "aerodromereferencepoint")]
    pub aerodrome_reference_point: FeatureCollection<Point<AerodromeReferencePoint>>,
    #[serde(rename = "apronelement")]
    pub apron_element: FeatureCollection<Polygon<ApronElement>>,
    #[serde(rename = "blastpad")]
    pub blastpad: FeatureCollection<Polygon<Blastpad>>,
    #[serde(rename = "constructionarea")]
    pub construction_area: FeatureCollection<Polygon<ConstructionArea>>,
    #[serde(rename = "deicingarea")]
    pub deicing_area: FeatureCollection<Polygon<DeicingArea>>,
    #[serde(rename = "finalapproachandtakeoffarea")]
    pub final_approach_and_takeoff_area: FeatureCollection<Polygon<FinalApproachAndTakeoffArea>>,
    #[serde(rename = "frequencyarea")]
    pub frequency_area: FeatureCollection<Polygon<FrequencyArea>>,
    #[serde(rename = "hotspot")]
    pub hotspot: FeatureCollection<Polygon<Hotspot>>,
    #[serde(rename = "landandholdshortoperationlocation")]
    pub land_and_hold_short_operation_location: FeatureCollection<LineString<LandAndHoldShortOperationLocation>>,
    #[serde(rename = "paintedcenterline")]
    pub painted_centerline: FeatureCollection<LineString<PaintedCenterline>>,
    #[serde(rename = "parkingstandarea")]
    pub parking_stand_area: FeatureCollection<Polygon<ParkingStandArea>>,
    #[serde(rename = "parkingstandlocation")]
    pub parking_stand_location: FeatureCollection<Point<ParkingStandLocation>>,
    #[serde(rename = "runwaydisplacedarea")]
    pub runway_displaced_area: FeatureCollection<Polygon<RunwayDisplacedArea>>,
    #[serde(rename = "runwayelement")]
    pub runway_element: FeatureCollection<Polygon<RunwayElement>>,
    #[serde(rename = "runwayexitline")]
    pub runway_exit_line: FeatureCollection<LineString<RunwayExitLine>>,
    #[serde(rename = "runwayintersection")]
    pub runway_intersection: FeatureCollection<Polygon<RunwayIntersection>>,
    #[serde(rename = "runwaymarking")]
    pub runway_marking: FeatureCollection<Polygon<RunwayMarking>>,
    #[serde(rename = "runwayshoulder")]
    pub runway_shoulder: FeatureCollection<Polygon<RunwayShoulder>>,
    #[serde(rename = "runwaythreshold")]
    pub runway_threshold: FeatureCollection<Point<RunwayThreshold>>,
    #[serde(rename = "serviceroad")]
    pub service_road: FeatureCollection<Polygon<ServiceRoad>>,
    #[serde(rename = "standguidanceline")]
    pub stand_guidance_line: FeatureCollection<LineString<StandGuidanceLine>>,
    #[serde(rename = "stopway")]
    pub stopway: FeatureCollection<Polygon<Stopway>>,
    #[serde(rename = "taxiwayelement")]
    pub taxiway_element: FeatureCollection<Polygon<TaxiwayElement>>,
    #[serde(rename = "taxiwayguidanceline")]
    pub taxiway_guidance_line: FeatureCollection<LineString<TaxiwayGuidanceLine>>,
    #[serde(rename = "taxiwayholdingposition")]
    pub taxiway_holding_position: FeatureCollection<LineString<TaxiwayHoldingPosition>>,
    #[serde(rename = "taxiwayintersectionmarking")]
    pub taxiway_intersection_marking: FeatureCollection<LineString<TaxiwayIntersectionMarking>>,
    #[serde(rename = "taxiwayshoulder")]
    pub taxiway_shoulder: FeatureCollection<Polygon<TaxiwayShoulder>>,
    #[serde(rename = "touchdownliftoffarea")]
    pub touchdown_liftoff_area: FeatureCollection<Polygon<TouchdownLiftoffArea>>,
    #[serde(rename = "verticallinestructure")]
    pub vertical_line_structure: FeatureCollection<LineString<VerticalLineStructure>>,
    #[serde(rename = "verticalpointstructure")]
    pub vertical_point_structure: FeatureCollection<Point<VerticalPointStructure>>,
    #[serde(rename = "verticalpolygonalstructure")]
    pub vertical_polygonal_structure: FeatureCollection<Polygon<VerticalPolygonalStructure>>,
    #[serde(rename = "water")]
    pub water: FeatureCollection<Polygon<Water>>,
}
