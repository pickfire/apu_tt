extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;
extern crate tabwriter;
extern crate yansi;

use chrono::prelude::*;
use std::io::Write;
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
    let data: Vec<Class> = reqwest::get(URL)?.json()?;
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
