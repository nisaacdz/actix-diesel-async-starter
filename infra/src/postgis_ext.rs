/// Extra PostGIS/SQL helper functions not provided by `postgis_diesel`.
///
/// `postgis_diesel` doesn't include `ST_Distance`. We define it here so it
/// can be used in Diesel query builder expressions — zero raw SQL.
use diesel::sql_types::Double;
use postgis_diesel::sql_types::GeoType;

diesel::define_sql_function! {
    /// Returns the distance between two geography/geometry values, in metres
    /// when used with the `Geography` SQL type.
    #[sql_name = "ST_Distance"]
    fn st_distance<G: GeoType>(left: G, right: G) -> Double;
}

diesel::define_sql_function! {
    /// Returns the greatest of two comparable float8 values.
    ///
    /// We use this for ranking geospatial matches by the worse of the two endpoint distances.
    #[sql_name = "GREATEST"]
    fn greatest_float8(left: Double, right: Double) -> Double;
}

diesel::define_sql_function! {
    /// Returns a geometry collection from a set of geometries.
    #[aggregate]
    #[sql_name = "ST_Collect"]
    fn st_collect<G: GeoType>(geom: G) -> postgis_diesel::sql_types::Geometry;
}

diesel::define_sql_function! {
    /// Returns the centroid of a geometry.
    #[sql_name = "ST_Centroid"]
    fn st_centroid(geom: postgis_diesel::sql_types::Geometry) -> postgis_diesel::sql_types::Geography;
}

diesel::define_sql_function! {
    /// Casts a Geography to a Geometry.
    #[sql_name = "geometry"]
    fn cast_to_geometry(geom: postgis_diesel::sql_types::Geography) -> postgis_diesel::sql_types::Geometry;
}
