#![allow(warnings)]

#[macro_use]
extern crate error_chain;

extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate tabwriter;

use std::io::Write;

const URL: &str = "https://ws.apiit.edu.my/web-services/index.php/open/weektimetable";

error_chain! {
    foreign_links {
        TabWriterError(tabwriter::IntoInnerError<tabwriter::TabWriter<Vec<u8>>>);
        Utf8Error(std::string::FromUtf8Error);
        JsonError(serde_json::error::Error);
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

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

fn run() -> Result<()> {
    let data: Vec<Class> = reqwest::get(URL)?.json()?;
    let classes: Vec<_> = data.into_iter()
        .filter(|c| c.INTAKE == "UC1F1705CS(DA)")
        .map(|c| Class {
            LOCATION: c.LOCATION.replace("NEW CAMPUS", "NEW"),
            ..c
        })
        .collect();

    let mut tw = tabwriter::TabWriter::new(vec![]);
    for class in classes {
        writeln!(
            &mut tw,
            "{}\t{}\t{}\t{}\t{}\t{}-{}",
            class.DAY,
            class.LOCATION,
            class.ROOM,
            class.MODID,
            class.LECTID,
            class.TIME_FROM,
            class.TIME_TO
        )?;
    }
    tw.flush()?;
    print!("{}", String::from_utf8(tw.into_inner()?)?);

    Ok(())
}

quick_main!(run);
