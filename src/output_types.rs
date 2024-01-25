use geo::{Coord, LineString, Polygon};

use crate::enums::{
    Availability,
    Bridge,
    CatStop,
    Direction,
    GroundSurfaceType,
    LandingCategory,
    LineColour,
    PapiVasi,
    Status,
    Style,
    SurfaceType,
    ThresholdType,
};

trait AmdbElement {
    fn id(&self) -> u64;
}

macro_rules! implement {
    ($type:ty) => {
        impl AmdbElement for $type {
            fn id(&self) -> u64 { self.id }
        }

        impl std::hash::Hash for $type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
        }
        impl Eq for $type {}
        impl PartialEq for $type {
            fn eq(&self, other: &$type) -> bool { self.id == other.id }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunwayId(pub String, pub String);

impl From<String> for RunwayId {
    fn from(s: String) -> Self {
        let mut split = s.split('.');
        let runway1_id = split.next().unwrap().to_string();
        let runway2_id = split.next().unwrap().to_string();
        RunwayId(runway1_id, runway2_id)
    }
}

impl ToString for RunwayId {
    fn to_string(&self) -> String { format!("{}.{}", self.0, self.1) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HoldingPointTarget {
    Taxiway(String),
    Runway(RunwayId),
}

impl From<String> for HoldingPointTarget {
    fn from(s: String) -> Self {
        if s.contains(".") {
            HoldingPointTarget::Runway(s.into())
        } else {
            HoldingPointTarget::Taxiway(s)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApronElement {
    pub id: u64,
    pub surface_type: GroundSurfaceType,
    pub apron_id: Option<String>,
    pub geometry: Polygon,
}

implement!(ApronElement);

#[derive(Debug, Clone)]
pub struct RunwayElement {
    pub id: u64,
    pub runway_id: RunwayId,
    pub width: f64,
    pub length: f64,
    pub surface_type: SurfaceType,
    pub geometry: Polygon,
}

implement!(RunwayElement);

#[derive(Debug, Clone)]
pub struct RunwayMarking {
    pub id: u64,
    pub runway_id: RunwayId,
    pub geometry: Polygon,
}

implement!(RunwayMarking);

#[derive(Debug, Clone)]
pub struct RunwayShoulder {
    pub id: u64,
    pub runway_id: RunwayId,
    pub status: Status,
    pub surface_type: GroundSurfaceType,
    pub geometry: Polygon,
}

implement!(RunwayShoulder);

#[derive(Debug, Clone)]
pub struct RunwayDisplacedArea {
    pub id: u64,
    pub threshold_id: String,
    pub status: Status,
    pub surface_type: SurfaceType,
    pub geometry: Polygon,
}

implement!(RunwayDisplacedArea);

#[derive(Debug, Clone)]
pub struct ParkingStandArea {
    pub id: u64,
    pub stand_id: Option<String>,
    pub apron_id: Option<String>,
    pub surface_type: GroundSurfaceType,
    pub jetway: Availability,
    pub fuel: String,
    pub towing: Availability,
    pub ground_power: Availability,
    pub terminal_name: Option<String>,
    pub geometry: Polygon,
}

implement!(ParkingStandArea);

#[derive(Debug, Clone)]
pub struct TaxiwayElement {
    pub id: u64,
    pub taxiway_id: Option<String>,
    pub apron_id: Option<String>,
    pub surface_type: GroundSurfaceType,
    pub bridge: Bridge,
    pub geometry: Polygon,
}

implement!(TaxiwayElement);

#[derive(Debug, Clone)]
pub struct TaxiwayShoulder {
    pub id: u64,
    pub surface_type: GroundSurfaceType,
    pub geometry: Polygon,
}

implement!(TaxiwayShoulder);

#[derive(Debug, Clone)]
pub struct StandGuidanceLine {
    pub id: u64,
    pub color: LineColour,
    pub direction: Direction,
    pub style: Style,
    pub stand_id: Option<String>,
    pub terminal_name: Option<String>,
    pub geometry: LineString,
}

implement!(StandGuidanceLine);

#[derive(Debug, Clone)]
pub struct TaxiwayGuidanceLine {
    pub id: u64,
    pub color: LineColour,
    pub direction: Direction,
    pub style: Style,
    pub status: Status,
    pub taxiway_id: Option<String>,
    pub geometry: LineString,
}

implement!(TaxiwayGuidanceLine);

#[derive(Debug, Clone)]
pub struct RunwayExitLine {
    pub id: u64,
    pub color: LineColour,
    pub direction: Direction,
    pub style: Style,
    pub status: Status,
    pub taxiway_id: Option<String>,
    pub geometry: LineString,
}

implement!(RunwayExitLine);

#[derive(Debug, Clone)]
pub struct TaxiwayHoldingPosition {
    pub id: u64,
    pub status: Status,
    pub taxiway_id: Option<String>,
    pub category: CatStop,
    pub holding_point_target: Option<HoldingPointTarget>,
    pub geometry: LineString,
}

implement!(TaxiwayHoldingPosition);

#[derive(Debug, Clone)]
pub struct AerodromeReferencePoint {
    pub id: u64,
    pub airport_id: String,
    pub iata_id: String,
    pub airport_name: String,
    pub elevation: f64,
    pub location: Coord,
}

implement!(AerodromeReferencePoint);

#[derive(Debug, Clone)]
pub struct RunwayThreshold {
    pub id: u64,
    pub threshold_id: String,
    pub touch_down_zone_elevation: f64,
    pub touch_down_zone_slope: f64,
    pub true_bearing: f64,
    pub magnetic_bearing: f64,
    pub runway_slope: f64,
    pub category: LandingCategory,
    pub papivasi: PapiVasi,
    pub status: Status,
    pub threshold_type: ThresholdType,
    pub location: Coord,
}

implement!(RunwayThreshold);

#[derive(Debug, Clone)]
pub struct ParkingStandLocation {
    pub id: u64,
    pub stand_id: Option<String>,
    pub aircraft_types: Vec<String>,
    pub location: Coord,
}

implement!(ParkingStandLocation);

#[derive(Debug, Clone)]
pub struct PaintedCenterline {
    pub id: u64,
    pub runway_id: RunwayId,
    pub geometry: LineString,
}

implement!(PaintedCenterline);
