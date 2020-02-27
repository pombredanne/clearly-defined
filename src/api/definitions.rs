use crate::{ApiResponse, Error};
use bytes::Bytes;
use http::Request;
use serde::Deserialize;
use std::{collections::BTreeMap, convert::TryFrom};

#[derive(Deserialize)]
pub struct DefCoords {
    #[serde(rename = "type")]
    pub shape: crate::Shape,
    pub provider: crate::Provider,
    pub name: String,
    pub revision: crate::CoordVersion,
}

#[derive(Deserialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha256: String,
}

#[derive(Deserialize)]
pub struct Scores {
    pub total: u32,
    pub date: u32,
    pub source: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub release_date: chrono::Date<chrono::Utc>,
    pub project_website: url::Url,
    pub urls: BTreeMap<String, url::Url>,
    pub hashes: Hashes,
    pub files: u32,
    pub tools: Vec<String>,
    pub tool_score: Scores,
    pub score: Scores,
}

#[derive(Deserialize)]
pub struct LicenseScore {
    pub total: u32,
    pub declared: u32,
    pub discovered: u32,
    pub consistency: u32,
    pub spdx: u32,
    pub texts: u32,
}

#[derive(Deserialize)]
pub struct Attribution {
    pub unknown: u32,
    pub parties: Vec<String>,
}

#[derive(Deserialize)]
pub struct Discovered {
    pub unknown: u32,
    pub expressions: Vec<String>,
}

#[derive(Deserialize)]
pub struct Facet {
    pub attribution: Attribution,
    pub discovered: Discovered,
    pub files: u32,
}

#[derive(Deserialize)]
pub struct Facets {
    pub core: Facet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub declared: String,
    pub facets: Facets,
    pub tool_score: LicenseScore,
    pub score: LicenseScore,
}

#[derive(Deserialize)]
pub struct File {
    pub path: String,
    pub hashes: Hashes,
    pub license: Option<String>,
    #[serde(default)]
    pub attributions: Vec<String>,
    #[serde(default)]
    pub natures: Vec<String>,
    pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct Definition {
    pub coordinates: DefCoords,
    /// The description of the component, won't be present if the coordinate
    /// has not been harvested
    pub described: Option<Description>,
    pub licensed: Option<License>,
    #[serde(default)]
    pub files: Vec<File>,
}

/// Gets the definitions for the supplied coordinates, note that this method
/// is limited to a maximum of 1000 coordinates per request, which is why
/// the return is actually an iterator
pub fn get<I, CA, C>(coordinates: I) -> impl Iterator<Item = Request<Bytes>>
where
    I: IntoIterator<Item = CA>,
    CA: AsRef<C>,
    C: crate::Coord,
{
    let mut requests = Vec::new();
    let mut coords = Vec::with_capacity(1000);
    for coord in coordinates.into_iter() {
        coords.push(serde_json::Value::String(format!(
            "{}",
            coord.as_ref().display()
        )));

        if coords.len() == 1000 {
            requests.push(std::mem::replace(&mut coords, Vec::with_capacity(1000)));
        }
    }

    if coords.len() > 0 {
        requests.push(coords);
    }

    requests.into_iter().map(|req| {
        let rb = http::Request::builder()
            .method(http::Method::POST)
            .uri(format!("{}/definitions", crate::ROOT_URI))
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::ACCEPT, "application/json");

        // This..._shouldn't_? fail
        let json = serde_json::to_vec(&serde_json::Value::Array(req))
            .expect("failed to serialize coordinates");

        rb.body(Bytes::from(json)).expect("failed to build request")
    })
}

pub struct GetResponse {
    /// The list of objects matching the query
    pub definitions: Vec<Definition>,
}

impl ApiResponse<&[u8]> for GetResponse {}
impl ApiResponse<bytes::Bytes> for GetResponse {}

impl<B> TryFrom<http::Response<B>> for GetResponse
where
    B: AsRef<[u8]>,
{
    type Error = Error;

    fn try_from(response: http::Response<B>) -> Result<Self, Self::Error> {
        let (_parts, body) = response.into_parts();

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawGetResponse {
            #[serde(flatten)]
            items: BTreeMap<String, Definition>,
        }

        let res: RawGetResponse = serde_json::from_slice(body.as_ref())?;

        let mut v = Vec::with_capacity(res.items.len());
        for (_, val) in res.items {
            v.push(val);
        }

        Ok(Self { definitions: v })
    }
}
