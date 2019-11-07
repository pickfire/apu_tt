use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufReader, BufWriter, Write},
};
use tabwriter::TabWriter;
use termion::{color, color::DetectColors, style};

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

fn fetch_timetable() -> Result<Vec<Class>, Box<dyn std::error::Error>> {
    Ok(reqwest::get(URL)?
        .json::<Vec<Class>>()?
        .into_iter()
        .filter(|c| c.intake == "UC3F1906CS(DA)")
        .map(|c| Class {
            location: c.location.trim_end_matches(" CAMPUS").to_owned(),
            ..c
        })
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = dirs::cache_dir().unwrap().join("weektimetable");

    let mut save = true;
    let classes: Vec<Class> = if cache.exists() {
        let classes: Vec<Class> = serde_cbor::from_reader(BufReader::new(File::open(&cache)?))?;
        let dates: HashSet<_> = classes.iter().map(|c| &c.datestamp_iso).collect();
        let today = Local::today();
        let days_from_mon = i64::from(Weekday::num_days_from_monday(&today.weekday()));
        let this_mon = today - Duration::days(days_from_mon);
        let needs_update = dates
            .iter()
            .any(|d| NaiveDate::parse_from_str(&d, "%F").unwrap() < this_mon.naive_local());
        if needs_update {
            fetch_timetable().unwrap_or(classes)
        } else {
            save = false;
            classes
        }
    } else {
        fetch_timetable()?
    };

    // generate days in week as iso format and filter classes for week
    let today = Local::today();
    let this_monday = i64::from(today.weekday().number_from_sunday());
    let mut find_week: Vec<_> = (1..=7)
        .map(|d| today - Duration::days(this_monday - d))
        .map(|d| d.format("%F").to_string())
        .collect();

    // initialize writers and check available colors
    let mut tw = TabWriter::new(io::stdout());
    let n_colors = tw.available_colors()?;
    let now = Local::now().naive_local();
    let mut next = false; // highlight current or next class once

    // display next week classes if no more classes this week
    let class_end_time = |c: &Class| {
        NaiveDateTime::new(
            NaiveDate::parse_from_str(&c.datestamp_iso, "%F").unwrap(),
            NaiveTime::parse_from_str(&c.time_to, "%I:%M %p").unwrap(),
        )
    };
    if !classes
        .iter()
        .filter(|c| find_week.contains(&c.datestamp_iso))
        .any(|c| now < class_end_time(c))
    {
        find_week = (1..=7)
            .map(|d| today - Duration::days(this_monday - d) + Duration::days(7))
            .map(|d| d.format("%F").to_string())
            .collect();
    }

    // display only relevant classes but classes filtered are cached
    for class in classes
        .iter()
        .filter(|c| find_week.contains(&c.datestamp_iso))
    {
        let date = NaiveDate::parse_from_str(&class.datestamp_iso, "%F")?;
        let time_since = NaiveTime::parse_from_str(&class.time_from, "%I:%M %p")?;
        let time_until = NaiveTime::parse_from_str(&class.time_to, "%I:%M %p")?;

        if !next && now < NaiveDateTime::new(date, time_until) {
            if n_colors >= 256 {
                let grey = color::Rgb(0x44, 0x44, 0x44);
                write!(&mut tw, "{}{}", color::Bg(grey), style::Bold)?;
            } else {
                write!(&mut tw, "{}{}", color::Bg(color::White), style::Bold)?;
            };
            next = true;
        }

        write!(
            &mut tw,
            "{}{}  {}{}-{}",
            color::Fg(color::Magenta),
            date.format("%a %b %d"),
            color::Fg(color::Green),
            time_since.format("%H%M"),
            time_until.format("%H%M")
        )?;
        write!(&mut tw, "\t{}{}", color::Fg(color::Blue), &class.location)?;
        write!(&mut tw, "\t{}{}", color::Fg(color::Red), &class.room)?;
        write!(&mut tw, "\t{}{}", color::Fg(color::Yellow), &class.modid)?;
        write!(&mut tw, "\t{}{}", color::Fg(color::Cyan), &class.lectid)?;
        writeln!(&mut tw, "{}", style::Reset)?;
    }
    tw.flush()?;

    if save {
        serde_cbor::to_writer(&mut BufWriter::new(File::create(&cache)?), &classes)?;
    }

    Ok(())
}
