use geo::{Coord, LineString, Polygon};

use crate::{
    amdb::{
        geo_json::{self, Coordinate},
        layers,
    },
    output_types::{
        AerodromeReferencePoint, ApronElement, PaintedCenterline, ParkingStandArea,
        RunwayDisplacedArea, RunwayElement, RunwayExitLine, RunwayMarking, RunwayShoulder,
        RunwayThreshold, StandGuidanceLine, TaxiwayElement, TaxiwayGuidanceLine,
        TaxiwayHoldingPosition, TaxiwayShoulder,
    },
};

impl Into<Coord> for Coordinate {
    fn into(self) -> Coord {
        Coord::from((self.lon, self.lat))
    }
}

impl Into<LineString> for geo_json::Geometry<Vec<Coordinate>> {
    fn into(self) -> LineString {
        LineString::from(self.coordinates.clone())
    }
}

impl Into<Polygon> for geo_json::Geometry<Vec<Vec<Coordinate>>> {
    fn into(self) -> Polygon {
        // TODO: Force to be clockwise
        Polygon::new(LineString::from(self.coordinates[0].clone()), vec![])
    }
}

/// Replaces empty strings or $UNK strings with None
fn normalize_string(s: Option<String>) -> Option<String> {
    match s {
        Some(x) if x == String::from("") || x == String::from("$UNK") => None,
        x => x,
    }
}

impl From<geo_json::Polygon<layers::ApronElement>> for ApronElement {
    fn from(apron: geo_json::Polygon<layers::ApronElement>) -> Self {
        Self {
            id: apron.properties.id,
            surface_type: apron.properties.gsurftyp,
            apron_id: normalize_string(apron.properties.idapron),
            geometry: apron.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::RunwayMarking>> for RunwayMarking {
    fn from(marking: geo_json::Polygon<layers::RunwayMarking>) -> Self {
        Self {
            id: marking.properties.id,
            runway_id: marking.properties.idrwy.into(),
            geometry: marking.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::RunwayShoulder>> for RunwayShoulder {
    fn from(shoulder: geo_json::Polygon<layers::RunwayShoulder>) -> Self {
        Self {
            id: shoulder.properties.id,
            runway_id: shoulder.properties.idrwy.into(),
            status: shoulder.properties.status,
            surface_type: shoulder.properties.gsurftyp,
            geometry: shoulder.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::RunwayDisplacedArea>> for RunwayDisplacedArea {
    fn from(area: geo_json::Polygon<layers::RunwayDisplacedArea>) -> Self {
        Self {
            id: area.properties.id,
            threshold_id: area.properties.idthr,
            status: area.properties.status,
            surface_type: area.properties.surftype,
            geometry: area.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::RunwayElement>> for RunwayElement {
    fn from(element: geo_json::Polygon<layers::RunwayElement>) -> Self {
        Self {
            id: element.properties.id,
            runway_id: element.properties.idrwy.into(),
            width: element.properties.width,
            length: element.properties.length,
            surface_type: element.properties.surftype,
            geometry: element.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::ParkingStandArea>> for ParkingStandArea {
    fn from(area: geo_json::Polygon<layers::ParkingStandArea>) -> Self {
        Self {
            id: area.properties.id,
            stand_id: normalize_string(area.properties.idstd),
            apron_id: normalize_string(area.properties.idapron),
            surface_type: area.properties.gsurftyp,
            jetway: area.properties.jetway,
            fuel: area.properties.fuel,
            towing: area.properties.towing,
            ground_power: area.properties.gndpower,
            terminal_name: normalize_string(area.properties.termref),
            geometry: area.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::TaxiwayElement>> for TaxiwayElement {
    fn from(element: geo_json::Polygon<layers::TaxiwayElement>) -> Self {
        Self {
            id: element.properties.id,
            taxiway_id: normalize_string(element.properties.idlin),
            apron_id: normalize_string(element.properties.idapron),
            surface_type: element.properties.gsurftyp,
            bridge: element.properties.bridge,
            geometry: element.geometry.into(),
        }
    }
}

impl From<geo_json::Polygon<layers::TaxiwayShoulder>> for TaxiwayShoulder {
    fn from(shoulder: geo_json::Polygon<layers::TaxiwayShoulder>) -> Self {
        Self {
            id: shoulder.properties.id,
            surface_type: shoulder.properties.gsurftyp,
            geometry: shoulder.geometry.into(),
        }
    }
}

impl From<geo_json::LineString<layers::StandGuidanceLine>> for StandGuidanceLine {
    fn from(guidance_line: geo_json::LineString<layers::StandGuidanceLine>) -> Self {
        Self {
            id: guidance_line.properties.id,
            color: guidance_line.properties.color,
            direction: guidance_line.properties.direc,
            style: guidance_line.properties.style,
            stand_id: normalize_string(guidance_line.properties.idstd),
            terminal_name: normalize_string(guidance_line.properties.termref),
            geometry: guidance_line.geometry.into(),
        }
    }
}

impl From<geo_json::LineString<layers::TaxiwayGuidanceLine>> for TaxiwayGuidanceLine {
    fn from(guidance_line: geo_json::LineString<layers::TaxiwayGuidanceLine>) -> Self {
        Self {
            id: guidance_line.properties.id,
            color: guidance_line.properties.color,
            direction: guidance_line.properties.direc,
            style: guidance_line.properties.style,
            status: guidance_line.properties.status,
            taxiway_id: normalize_string(guidance_line.properties.idlin),
            geometry: guidance_line.geometry.into(),
        }
    }
}

impl From<geo_json::LineString<layers::RunwayExitLine>> for RunwayExitLine {
    fn from(exit_line: geo_json::LineString<layers::RunwayExitLine>) -> Self {
        Self {
            id: exit_line.properties.id,
            color: exit_line.properties.color,
            direction: exit_line.properties.direc,
            style: exit_line.properties.style,
            status: exit_line.properties.status,
            taxiway_id: exit_line.properties.idlin,
            geometry: exit_line.geometry.into(),
        }
    }
}

impl From<geo_json::LineString<layers::TaxiwayHoldingPosition>> for TaxiwayHoldingPosition {
    fn from(position: geo_json::LineString<layers::TaxiwayHoldingPosition>) -> Self {
        Self {
            id: position.properties.id,
            status: position.properties.status,
            taxiway_id: normalize_string(position.properties.idlin),
            category: position.properties.catstop,
            holding_point_target: normalize_string(position.properties.idp).map(Into::into),
            geometry: position.geometry.into(),
        }
    }
}

impl From<geo_json::LineString<layers::PaintedCenterline>> for PaintedCenterline {
    fn from(centerline: geo_json::LineString<layers::PaintedCenterline>) -> Self {
        Self {
            id: centerline.properties.id,
            runway_id: centerline.properties.idrwy.into(),
            geometry: centerline.geometry.into(),
        }
    }
}

impl From<geo_json::Point<layers::AerodromeReferencePoint>> for AerodromeReferencePoint {
    fn from(reference_point: geo_json::Point<layers::AerodromeReferencePoint>) -> Self {
        Self {
            id: reference_point.properties.id,
            airport_id: reference_point.properties.idarpt,
            iata_id: reference_point.properties.iata,
            airport_name: reference_point.properties.name,
            elevation: reference_point.properties.elev,
            location: reference_point.geometry.coordinates.into(),
        }
    }
}

impl From<geo_json::Point<layers::RunwayThreshold>> for RunwayThreshold {
    fn from(threshold: geo_json::Point<layers::RunwayThreshold>) -> Self {
        Self {
            id: threshold.properties.id,
            threshold_id: threshold.properties.idthr,
            touch_down_zone_elevation: threshold.properties.tdze,
            touch_down_zone_slope: threshold.properties.tdzslope,
            true_bearing: threshold.properties.brngtrue,
            magnetic_bearing: threshold.properties.brngmag,
            runway_slope: threshold.properties.rwyslope,
            category: threshold.properties.cat,
            papivasi: threshold.properties.vasis,
            status: threshold.properties.status,
            threshold_type: threshold.properties.thrtype,

            location: threshold.geometry.coordinates.into(),
        }
    }
}
