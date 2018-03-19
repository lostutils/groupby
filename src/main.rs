extern crate regex;
extern crate clap;

use clap::{App, Arg};
use regex::Regex;
use std::collections::{BTreeSet, BTreeMap};
use std::io::BufRead;

#[derive(Debug)]
enum GroupId<'a> {
    Name(&'a str),
    Index(usize),
    None,
}

impl<'a> From<&'a str> for GroupId<'a> {
    fn from(s: &'a str) -> Self {
        match s.parse::<usize>() {
            Ok(n) => GroupId::Index(n),
            Err(_) => GroupId::Name(s),
        }
    }
}


trait Group: Default + IntoIterator<Item=String> {
    fn add(&mut self, line: String);
}

impl Group for BTreeSet<String> {
    fn add(&mut self, line: String) {
        self.insert(line);
    }
}

impl Group for Vec<String> {
    fn add(&mut self, line: String) {
        self.push(line);
    }
}

fn groupby<G: Group>(re: &Regex, group_id: GroupId) -> BTreeMap<String, G> {
    let mut grouping: BTreeMap<String, G> = BTreeMap::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();


        let capture = match re.captures(&line) {
            Some(captures) => {
                match group_id {
                    GroupId::Name(name) => captures.name(name).unwrap().as_str(),
                    GroupId::Index(index) => captures.get(index).unwrap().as_str(),
                    GroupId::None => captures.get(0).unwrap().as_str(),
                }
            }
            None => "***NO-MATCH***",
        };

        grouping.entry(capture.to_string()).or_insert_with(Default::default).add(line.clone());
    }

    return grouping;
}

fn print_groupby<G: Group>(re: &Regex, group_id: GroupId) {
    for (group, members) in groupby::<G>(&re, group_id) {
        println!("{}", group);
        for line in members {
            println!("    {}", line);
        }
    }
}

fn print_groupby_unique(re: &Regex, group_id: GroupId) {
    print_groupby::<BTreeSet<String>>(re, group_id);
}

fn print_groupby_all(re: &Regex, group_id: GroupId) {
    print_groupby::<Vec<String>>(re, group_id);
}

fn validate_group_id(group_id: &GroupId, re: &Regex) -> Result<(), String> {
    match group_id {
        &GroupId::Name(name) => {
            if re.capture_names().find(|capture_name| capture_name.unwrap_or("") == name).is_none() {
                Err(format!("Group name unknown: {}", name))
            } else {
                Ok(())
            }
        }
        &GroupId::Index(index) => {
            if index >= re.captures_len() {
                Err(format!("Group index too large: {}", index))
            } else {
                Ok(())
            }
        }
        &GroupId::None => Ok(()),
    }
}


fn main() {
    let matches = App::new("groupby (lostutils)")
        .about("Group lines based on a given regex.")
        .arg(
            Arg::with_name("regex")
                .help("The regex to group by.")
                .long_help("The regex to group by. \
                 The match will use the entire expression, unless a group-id is provided.")
                .required(true)
        )
        .arg(
            Arg::with_name("group-id")
                .short("g")
                .value_name("group-id")
                .help("The group-id to group by.")
        )
        .arg(
            Arg::with_name("unique")
                .short("u")
                .takes_value(false)
                .help("Remove duplicate lines in the same group")
        )
        .get_matches();


    let pat = matches.value_of("regex").unwrap();

    let re = Regex::new(pat).unwrap();

    let group_id = match matches.value_of("group-id") {
        Some(value) => GroupId::from(value),
        None => GroupId::None,
    };

    if let Err(message) = validate_group_id(&group_id, &re) {
        println!("{}", message);
        std::process::exit(1);
    }

    let is_unique = matches.is_present("unique");

    if is_unique {
        print_groupby_unique(&re, group_id);
    } else {
        print_groupby_all(&re, group_id);
    }
}
