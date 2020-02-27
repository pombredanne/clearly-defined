use crate::Coordinate;
use bytes::Bytes;
use http::Request;

/// Gets the definitions for the supplied coordinates, note that this method
/// is limited to a maximum of 1000 coordinates per request, which is why
/// the return is actually an iterator
pub fn get(coordinates: I) -> impl Iterator<Request<Bytes>>
where
    I: IntoIterator<Item = CA>,
    CA: AsRef<C>,
    C: crate::Coord,
{
    let mut requests = Vec::new();
    let mut coords = Vec::with_capacity(1000);
    for coord in coordinates.into_iter() {}
}
