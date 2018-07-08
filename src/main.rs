extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;
extern crate tabwriter;
extern crate yansi;

use chrono::prelude::*;
use reqwest::{header::IfModifiedSince, StatusCode};
use std::{
    env, fs::{self, File}, io::{BufReader, BufWriter, Write},
};
use tabwriter::TabWriter;
use yansi::Paint;

const URL: &str = "http://s3-ap-southeast-1.amazonaws.com/open-ws/weektimetable";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Class {
    intake: String,
    modid: String,
    day: String,
    location: String,
    room: String,
    lectid: String,
    datestamp: String,
    datestamp_iso: String,
    time_from: String,
    time_to: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = env::home_dir().unwrap().join(".cache/weektimetable");
    let mut request = reqwest::Client::new().get(URL);

    if cache.exists() {
        if let Ok(time) = fs::metadata(&cache)?.modified() {
            request.header(IfModifiedSince(time.into()));
        }
    }

    let data: Vec<Class> = if let Ok(mut response) = request.send() {
        match response.status() {
            StatusCode::Ok => {
                let mut buf = vec![];
                response.copy_to(&mut buf)?;
                BufWriter::new(File::create(&cache)?).write_all(&buf)?;
                serde_json::from_slice(&buf)?
            }
            StatusCode::NotModified => {
                serde_json::from_reader(BufReader::new(File::open(&cache)?))?
            }
            s => panic!("Received response status: {:?}", s),
        }
    } else {
        serde_json::from_reader(BufReader::new(File::open(&cache)?))?
    };

    let classes: Vec<_> = data.into_iter()
        .filter(|c| {
            c.intake == "UC2F1805CS(DA)"
                && (c.modid.contains("T-1") || c.modid.contains('L') || c.modid.contains("(LS)"))
        })
        .map(|c| Class {
            location: c.location.replace("NEW CAMPUS", "NEW"),
            ..c
        })
        .collect();

    let mut tw = TabWriter::new(vec![]);
    for class in classes {
        writeln!(
            &mut tw,
            "{}\t{}\t{}\t{}\t{}\t{}",
            Paint::purple(
                NaiveDate::parse_from_str(&*class.datestamp_iso, "%F")?.format("%a %b %d")
            ),
            Paint::green(format!(
                "{}-{}",
                NaiveTime::parse_from_str(&*class.time_from, "%I:%M %p")?.format("%H%M"),
                NaiveTime::parse_from_str(&*class.time_to, "%I:%M %p")?.format("%H%M")
            )),
            Paint::blue(class.location).bold(),
            Paint::red(class.room),
            Paint::yellow(class.modid),
            Paint::cyan(class.lectid),
        )?;
    }
    tw.flush()?;
    print!("{}", String::from_utf8(tw.into_inner()?)?);

    Ok(())
}
