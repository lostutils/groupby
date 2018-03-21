#[macro_use]
extern crate clap;
extern crate regex;

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

fn compare_with_option<T: PartialEq>(value: &T, opt: &Option<T>) -> bool {
    match *opt {
        Some(ref opt_value) => opt_value == value,
        _ => false,
    }
}

fn has_named_capture(regex: &Regex, name: &str) -> bool {
    regex.capture_names().any(|capture_name| compare_with_option(&name, &capture_name))
}

fn has_indexed_capture(regex: &Regex, index: usize) -> bool {
    index < regex.captures_len()
}

fn validate_group_id(group_id: &GroupId, re: &Regex) -> Result<(), String> {
    match *group_id {
        GroupId::Name(name) => {
            if !has_named_capture(&re, &name) {
                Err(format!("Group name unknown: {}", name))
            } else {
                Ok(())
            }
        }
        GroupId::Index(index) => {
            if !has_indexed_capture(&re, index) {
                Err(format!("Group index too large: {}", index))
            } else {
                Ok(())
            }
        }
        GroupId::None => Ok(()),
    }
}


fn main() {
    let matches = App::new("groupby (lostutils)")
        .about("Group lines based on a given regex.")
        .version(crate_version!())
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
                .long_help("The group-id to group by. Can be an index or a group name.")
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
