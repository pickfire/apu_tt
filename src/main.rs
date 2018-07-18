extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;

extern crate chrono;
extern crate dirs;
extern crate tabwriter;
extern crate yansi;

use chrono::prelude::*;
use reqwest::StatusCode;
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
};
use tabwriter::TabWriter;
use yansi::Paint;

const URL: &str = "http://s3-ap-southeast-1.amazonaws.com/open-ws/weektimetable";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Class {
    #[serde(skip_serializing, default = "String::new")]
    intake: String,
    modid: String,
    #[serde(skip_serializing, default = "String::new")]
    day: String,
    location: String,
    room: String,
    lectid: String,
    #[serde(skip_serializing, default = "String::new")]
    datestamp: String,
    datestamp_iso: String,
    time_from: String,
    time_to: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = dirs::cache_dir().unwrap().join("weektimetable");
    let mut request = reqwest::Client::new().get(URL);

    if cache.exists() {
        if let Ok(time) = fs::metadata(&cache)?.modified() {
            let time: DateTime<Utc> = DateTime::from(time);
            request = request.header("if-modified-since", time.to_rfc2822());
        }
    }

    let mut save = false;
    let classes: Vec<Class> = if let Ok(mut response) = request.send() {
        match response.status() {
            StatusCode::OK => {
                save = true;
                response
                    .json::<Vec<Class>>()?
                    .into_iter()
                    .filter(|c| c.intake == "UC2F1805CS(DA)" && !c.modid.starts_with("MPU"))
                    .map(|c| Class {
                        location: c.location.replace("NEW CAMPUS", "NEW"),
                        ..c
                    }).collect()
            }
            StatusCode::NOT_MODIFIED => {
                serde_cbor::from_reader(BufReader::new(File::open(&cache)?))?
            }
            s => panic!("Received response status: {:?}", s),
        }
    } else {
        serde_cbor::from_reader(BufReader::new(File::open(&cache)?))?
    };

    let mut tw = TabWriter::new(vec![]);
    for class in &classes {
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
            Paint::blue(&class.location).bold(),
            Paint::red(&class.room),
            Paint::yellow(&class.modid),
            Paint::cyan(&class.lectid),
        )?;
    }
    tw.flush()?;
    print!("{}", String::from_utf8(tw.into_inner()?)?);

    if save {
        serde_cbor::to_writer(&mut BufWriter::new(File::create(&cache)?), &classes)?;
    }

    Ok(())
}
