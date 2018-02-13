#![allow(warnings)]

#[macro_use]
extern crate error_chain;

extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate prettytable;

// extern crate rmp_serde;

// use std::fs::File;

error_chain! {
    foreign_links {
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
    let body =
        reqwest::get("https://ws.apiit.edu.my/web-services/index.php/open/weektimetable")?.text()?;
    let data: Vec<Class> = serde_json::from_str(&body).unwrap();

    // let mut buffer = File::create("cache").expect("serialize");

    // TODO: Caching
    // rmp_serde::to_writer(&mut buffer, &data, false);

    // TODO: Config
    let classes: Vec<Class> = data.into_iter()
        .filter(|c| c.INTAKE == "UC1F1705CS(DA)")
        .collect();

    let mut table = prettytable::Table::new();
    for class in classes {
        table.add_row(row![
            class.DAY,
            class.LOCATION.replace("NEW CAMPUS", "NEW"),
            class.ROOM,
            class.MODID,
            class.LECTID,
            format!("{}-{}", class.TIME_FROM, class.TIME_TO)
        ]);
    }

    let format = prettytable::format::FormatBuilder::new()
        .column_separator(' ')
        .build();
    table.set_format(format);
    table.printstd();

    Ok(())
}

quick_main!(run);
