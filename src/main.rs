#![allow(warnings)]

extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate tabwriter;
extern crate yansi;

use std::io::Write;
use tabwriter::TabWriter;
use yansi::Paint;

const URL: &str = "https://ws.apiit.edu.my/web-services/index.php/open/weektimetable";

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
    time_from: String,
    time_to: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let data: Vec<Class> = reqwest::get(URL)?.json()?;
    let classes: Vec<_> = data.into_iter()
        .filter(|c| {
            c.intake == "UC2F1805CS(DA)"
                && (c.modid.contains("T-1") || c.modid.contains("L") || c.modid.contains("(LS)"))
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
            Paint::purple(class.day),
            Paint::green(format!("{}-{}", class.time_from, class.time_to)),
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
