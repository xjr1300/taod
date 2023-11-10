use geo_types::Geometry;
use geozero::wkb;

pub type GeometryF64 = Geometry<f64>;

pub type WkbGeometryF64 = wkb::Decode<GeometryF64>;
