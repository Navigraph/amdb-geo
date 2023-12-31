use serde::Deserialize;

use crate::enums::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct AerodromeReferencePoint {
    pub id: u64,
    pub idarpt: String,
    pub iata: String,
    pub name: String,
    pub elev: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ApronElement {
    pub id: u64,
    pub idapron: Option<String>,
    pub gsurftyp: GroundSurfaceType,
    pub status: Status,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Blastpad {
    pub id: u64,
    pub idthr: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConstructionArea {
    pub id: u64,
    pub pstdate: String,
    pub pendate: String,
    pub piocdate: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DeicingArea {
    pub id: u64,
    pub idbase: Option<String>,
    pub gsurftyp: GroundSurfaceType,
    pub ident: String,
    pub status: Status,
    pub restacn: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FinalApproachAndTakeoffArea {
    pub id: u64,
    pub idrwy: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FrequencyArea {
    pub id: u64,
    pub frq: f64,
    pub station: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Hotspot {
    pub id: u64,
    pub idhot: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LandAndHoldShortOperationLocation {
    pub id: u64,
    pub idp: String,
    pub idthr: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PaintedCenterline {
    pub id: u64,
    pub idrwy: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ParkingStandArea {
    pub id: u64,
    pub idstd: Option<String>,
    pub gsurftyp: GroundSurfaceType,
    pub idapron: Option<String>,
    pub jetway: Availability,
    pub fuel: String,
    pub restacn: Option<String>,
    pub towing: Availability,
    pub gndpower: Availability,
    pub termref: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ParkingStandLocation {
    pub id: u64,
    pub idstd: Option<String>,
    pub acn: Option<String>,
    pub termref: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayDisplacedArea {
    pub id: u64,
    pub idthr: String,
    pub status: Status,
    pub surftype: SurfaceType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayElement {
    pub id: u64,
    pub idrwy: String,
    pub width: f64,
    pub length: f64,
    pub surftype: SurfaceType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayExitLine {
    pub id: u64,
    pub idlin: String,
    pub status: Status,
    pub direc: Direction,
    pub color: LineColour,
    pub style: Style,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayIntersection {
    pub id: u64,
    pub idrwi: String,
    pub surftype: SurfaceType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayMarking {
    pub id: u64,
    pub idrwy: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayShoulder {
    pub id: u64,
    pub idrwy: String,
    pub status: Status,
    pub gsurftyp: GroundSurfaceType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RunwayThreshold {
    pub id: u64,
    pub idthr: String,
    pub tdze: f64,
    pub tdzslope: f64,
    pub brngtrue: f64,
    pub brngmag: f64,
    pub rwyslope: f64,
    pub tora: f64,
    pub toda: f64,
    pub asda: f64,
    pub lda: f64,
    pub cat: LandingCategory,
    pub status: Status,
    pub thrtype: ThresholdType,
    pub vasis: PapiVasi,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ServiceRoad {
    pub id: u64,
    pub gsurftyp: GroundSurfaceType,
    pub idbase: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StandGuidanceLine {
    pub id: u64,
    pub idstd: Option<String>,
    pub status: Status,
    pub direc: Direction,
    pub color: LineColour,
    pub style: Style,
    pub termref: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Stopway {
    pub id: u64,
    pub idthr: String,
    pub status: f64,
    pub surftype: SurfaceType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TaxiwayElement {
    pub id: u64,
    pub idlin: Option<String>,
    pub idapron: Option<String>,
    pub gsurftyp: GroundSurfaceType,
    pub bridge: Bridge,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TaxiwayGuidanceLine {
    pub id: u64,
    pub idlin: Option<String>,
    pub status: Status,
    pub direc: Direction,
    pub color: LineColour,
    pub style: Style,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TaxiwayHoldingPosition {
    pub id: u64,
    pub idp: Option<String>,
    pub idlin: Option<String>,
    pub status: Status,
    pub catstop: CatStop,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TaxiwayIntersectionMarking {
    pub id: u64,
    pub idlin: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TaxiwayShoulder {
    pub id: u64,
    pub gsurftyp: GroundSurfaceType,
    pub status: Status,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TouchdownLiftoffArea {
    pub id: u64,
    pub idrwy: Option<String>,
    pub surftype: SurfaceType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VerticalLineStructure {
    pub id: u64,
    pub linsttyp: LineStructureType,
    pub material: Material,
    pub height: f64,
    pub elev: f64,
    pub lighting: Conformance,
    pub marking: Conformance,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VerticalPointStructure {
    pub id: u64,
    pub pntsttyp: PointStructureType,
    pub material: Material,
    pub height: f64,
    pub elev: f64,
    pub lighting: Conformance,
    pub radius: f64,
    pub marking: Conformance,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VerticalPolygonalStructure {
    pub id: u64,
    pub ident: Option<String>,
    pub plysttyp: PolygonalStructureType,
    pub material: Material,
    pub height: f64,
    pub elev: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Water {}
