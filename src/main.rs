extern crate regex;
extern crate clap;

use clap::{App, Arg};
use regex::Regex;
use std::collections::{BTreeSet, BTreeMap};
use std::io::BufRead;

fn groupby_unique(re: &Regex, group_id: Option<usize>) -> BTreeMap<String, BTreeSet<String>> {
    let mut grouping: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();


        let capture = match re.captures(&line) {
            Some(captures) => captures.get(group_id.unwrap_or(0)).unwrap().as_str(),
            None => "***NO-MATCH***",
        };

        grouping.entry(capture.to_string()).or_insert(BTreeSet::new()).insert(line.clone());
    }

    return grouping;
}

fn print_groupby_unique(re: &Regex, group_id: Option<usize>) {
    for (group, members) in groupby_unique(&re, group_id) {
        println!("{}", group);
        for line in members {
            println!("    {}", line);
        }
    }
}


fn groupby_all(re: &Regex, group_id: Option<usize>) -> BTreeMap<String, Vec<String>> {
    let mut grouping: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();


        let capture = match re.captures(&line) {
            Some(captures) => captures.get(group_id.unwrap_or(0)).unwrap().as_str(),
            None => "***NO-MATCH***",
        };

        grouping.entry(capture.to_string()).or_insert(Vec::new()).push(line.clone());
    }

    return grouping;
}

fn print_groupby_all(re: &Regex, group_id: Option<usize>) {
    for (group, members) in groupby_all(&re, group_id) {
        println!("{}", group);
        for line in members {
            println!("    {}", line);
        }
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
        Some(n) => match n.parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => None,
        },
        None => None,
    };

    let is_unique = matches.is_present("unique");

    if is_unique {
        print_groupby_unique(&re, group_id);
    } else {
        print_groupby_all(&re, group_id);
    }
}
