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
struct Class {
    INTAKE: String,
    MODID: String,
    DAY: String,
    LOCATION: String,
    ROOM: String,
    LECTID: String,
    DATESTAMP: String,
    TIME_FROM: String,
    TIME_TO: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let data: Vec<Class> = reqwest::get(URL)?.json()?;
    let classes: Vec<_> = data.into_iter()
        .filter(|c| {
            c.INTAKE == "UC2F1805CS(DA)"
                && (c.MODID.contains("T-1") || c.MODID.contains("L") || c.MODID.contains("(LS)"))
        })
        .map(|c| Class {
            LOCATION: c.LOCATION.replace("NEW CAMPUS", "NEW"),
            ..c
        })
        .collect();

    let mut tw = TabWriter::new(vec![]);
    for class in classes {
        writeln!(
            &mut tw,
            "{}\t{}\t{}\t{}\t{}\t{}",
            Paint::purple(class.DAY),
            Paint::blue(class.LOCATION).bold(),
            Paint::yellow(class.ROOM),
            Paint::cyan(class.MODID),
            Paint::green(class.LECTID),
            Paint::red(format!("{}-{}", class.TIME_FROM, class.TIME_TO))
        )?;
    }
    tw.flush()?;
    print!("{}", String::from_utf8(tw.into_inner()?)?);

    Ok(())
}
