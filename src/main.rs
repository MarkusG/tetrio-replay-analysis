use std::fs;

use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize)]
struct Replay {
    _id: String,
    shortid: String,
    // ismulti: bool, // multiplayer?
    endcontext: Vec<EndContext>,
    // gametype: String, // tl, blitz, 4l, TODO make an enum?
    // verified: bool, // no idea
    data: i32, // TODO
    // back: String, // no idea
    ts: String // timestamp
}

#[derive(Debug, Deserialize)]
struct EndContext {
    naturalorder: i32,
    user: EndContextUser,
    active: bool,
    wins: u8, // number of round wins
    points: EndContextPoints, // osk why
    inputs: i32, // total inputs
    piecesplaced: i32 // pieces placed
}

#[derive(Debug, Deserialize)]
struct EndContextUser {
    _id: String,
    username: String
}

#[derive(Debug, Deserialize)]
struct EndContextPoints {
    #[serde(rename(deserialize = "primary"))]
    wins: u8, // round wins
    #[serde(rename(deserialize = "secondary"))]
    apm: f64,
    #[serde(rename(deserialize = "tertiary"))]
    pps: f64,
    extra: EndContextPointsExtra, // contains the overall vs score
    #[serde(rename(deserialize = "secondaryAvgTracking"))]
    round_apm: Vec<f64>, // per-round apm
    #[serde(rename(deserialize = "tertiaryAvgTracking"))]
    round_pps: Vec<f64>, // per-round pps
    extraAvgTracking: EndContextPointsExtraAvgTracking, // per-round apm osk wtf
}

#[derive(Debug, Deserialize)]
struct EndContextPointsExtra {
    vs: f64
}

#[derive(Debug, Deserialize)]
struct EndContextPointsExtraAvgTracking {
#[serde(rename(deserialize = "aggregatestats___vsscore"))]
    vs: Vec<f64> // what in the hell
}

fn main() {
    let raw_data = fs::read_to_string("replay.ttrm").unwrap();
    let parsed_data: Replay = serde_json::from_str(&raw_data).unwrap();
    for e in parsed_data.endcontext {
        println!("{:#?}", e);
    }
}
