use serde::{Deserialize, Deserializer};

/// Defines an enum and automatically assigns a number to each variant of an enum, and creates an Unknown variant for values outside of the range.
macro_rules! define_enum {
    ($name:ident { $($variant:ident),* }) => {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum $name {
            $($variant),*,
            Unknown(i32)
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let variant_names = &[$(stringify!($variant)),*];
                let index = i32::deserialize(deserializer)?;
                match variant_names.get(index as usize) {
                    Some(&name) => {
                        match name {
                            $(stringify!($variant) => Ok($name::$variant),)*
                            _ => Ok($name::Unknown(index)),
                        }
                    }
                    None => Ok($name::Unknown(index)),
                }
            }
        }
    }
}

define_enum!(GroundSurfaceType {
    Concrete,
    Asphalt,
    DesertOrSandOrDirt,
    BareEarth,
    SnowOrIce,
    Water,
    GrassOrTurf,
    GravelOrCinders,
    PiercedSteelPlanks,
    Bitumen,
    Brick,
    Macadam,
    Stone,
    Coral,
    Clay,
    Laterite,
    LandingMats,
    Membrane,
    Wood
});

define_enum!(SurfaceType {
    ConcreteGrooved,
    ConcreteNoneGrooved,
    AsphaltGrooved,
    AsphaltNonGrooved,
    DesertOrSandOrDirt,
    BareEarth,
    SnowOrIce,
    Water,
    GrassOrTurf,
    AggregateFrictionSealCoat,
    GravelOrCinders,
    PorousFrictionCourses,
    PiercedSteelPlanks,
    RubberizedFrictionSealCoat,
    Bitumen,
    Brick,
    Macadam,
    Stone,
    Coral,
    Clay,
    Laterite,
    LandingMats,
    Membrane,
    Wood
});

define_enum!(Status { Closed, Open });

define_enum!(Availability {
    Unavailable,
    Available
});

define_enum!(LandingCategory {
    Npa,
    Cat1,
    Cat2,
    Cat3A,
    Cat3B,
    Cat3C
});

define_enum!(CatStop {
    None,
    Cat1,
    Cat2Or3
});

define_enum!(ThresholdType {
    Threshold,
    DisplacedThreshold
});

define_enum!(PapiVasi {
    None,
    Papi,
    Apapi,
    Vasis,
    Avasis
});

define_enum!(LineColour {
    Yellow,
    Orange,
    Blue,
    White
});

define_enum!(Style {
    Solid,
    Dashed,
    Dotted
});

define_enum!(Direction {
    Bidirectional,
    StartToEndpoint,
    EndToStartpoint
});

define_enum!(Bridge {
    None,
    Underpass,
    Overpass
});

define_enum!(LineStructureType {
    PowerLine,
    CableRailway,
    BushesOrTrees,
    Wall
});

define_enum!(PointStructureType {
    Smokestack,
    PowerlinePylon,
    Antenna,
    Windsock,
    Tree,
    Lightpole,
    LightStanchion
});

define_enum!(PolygonalStructureType {
    TerminalBuilding,
    Hangar,
    ControlTower,
    NonTerminalBuilding,
    Tank,
    Tree,
    Bush,
    Forest,
    EarthenWorks
});

define_enum!(Material {
    Concrete,
    Metal,
    StoneOrBrick,
    Composition,
    Rock,
    EarthenWorks,
    Wood
});

define_enum!(Conformance {
    NonConformant,
    Conformant
});
