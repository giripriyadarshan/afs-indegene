use std::{
    collections::HashSet,
    io::{Read, Write},
    path::Path,
    process::Command,
};
use xml::reader::{EventReader, XmlEvent};

use crate::models::revision;

pub fn get_key_messages(all_key_messages: HashSet<String>) -> Option<HashSet<String>> {
    // run svn command to check latest revision number
    let output = Command::new("svn")
        .args(["log", "--xml", "-l", "1"])
        .arg("--username")
        .arg("priyadarshan.giri")
        .arg("--password")
        .arg("this1sforAFS")
        .output()
        .expect("Failed to execute command");

    let mut latest_revision_number: usize = 0;

    if output.status.success() {
        let parser = EventReader::new(output.stdout.as_slice());

        for event in parser {
            match event {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == "logentry" {
                        for attr in attributes {
                            if attr.name.local_name == "revision" {
                                latest_revision_number = attr.value.parse::<usize>().unwrap();
                            }
                        }
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {}
                Ok(XmlEvent::Characters(..)) => {}
                Err(e) => {
                    // Handle any parsing errors
                    eprintln!("Error: {}", e);
                }
                _ => {}
            }
        }
    } else {
        eprintln!("Error: svn command failed");
    }

    // check if revision.toml exists
    if !std::path::Path::new("revision.toml").exists() {
        // create a file named as revision.toml and add default values
        std::fs::File::create("revision.toml").unwrap();

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open("revision.toml")
            .unwrap();
        file.write_all(b"[revision]").unwrap();
        let lrn = format!("\nrevision_number = {}", latest_revision_number);
        file.write_all(lrn.as_bytes()).unwrap();
        Some(all_key_messages)
    } else {
        // read revision number from revision.toml
        let mut file = std::fs::File::open("revision.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file");
        let config: revision::RevisionConfig = toml::from_str(contents.as_str()).unwrap();
        let revision_number = config.revision.revision_number;

        // check if revision number is same as latest revision number
        if revision_number == latest_revision_number {
            None
        } else {
            // edge case: if revision number is greater than latest revision number
            if revision_number > latest_revision_number {
                // update revision number in revision.toml
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .open("revision.toml")
                    .unwrap();
                file.write_all(b"[revision]").unwrap();
                let lrn = format!("\nrevision_number = {}", latest_revision_number);
                file.write_all(lrn.as_bytes()).unwrap();
                // propagate error through afs daemon
                None
            } else {
                let mut changed_keymessages: HashSet<String> = HashSet::new();

                for i in (revision_number + 1)..=latest_revision_number {
                    let paths = read_paths_from_svn(i);
                    for path in paths {
                        let path = Path::new(&path);
                        let path = path.strip_prefix(get_relative_path()).unwrap();
                        let mut ancestors = path.ancestors();
                        changed_keymessages.insert(
                            ancestors
                                .nth(ancestors.count() - 2)
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_owned(),
                        );
                    }
                }

                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .open("revision.toml")
                    .unwrap();
                file.write_all(b"[revision]").unwrap();
                let lrn = format!("\nrevision_number = {}", latest_revision_number);
                file.write_all(lrn.as_bytes()).unwrap();

                let returned_key_messages = changed_keymessages
                    .intersection(&all_key_messages)
                    .cloned()
                    .collect::<HashSet<String>>();

                Some(returned_key_messages)
            }
        }
    }
}

fn read_paths_from_svn(revision_number: usize) -> Vec<String> {
    let output = Command::new("svn")
        .args([
            "log",
            "--verbose",
            "--xml",
            "-r",
            revision_number.to_string().as_str(),
        ])
        .arg("--username")
        .arg("priyadarshan.giri")
        .arg("--password")
        .arg("this1sforAFS")
        .output()
        .expect("Failed to execute command");

    let mut paths = Vec::new();

    if output.status.success() {
        let parser = EventReader::new(output.stdout.as_slice());

        let mut within_paths = false;
        let mut within_path = false;

        // Iterate over the XML events
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    if name.local_name == "paths" {
                        within_paths = true;
                    } else if name.local_name == "path" && within_paths {
                        within_path = true;
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "paths" {
                        within_paths = false;
                    } else if name.local_name == "path" {
                        within_path = false;
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    if within_path {
                        // Store the path
                        paths.push(text.clone());
                    }
                }
                Err(e) => {
                    // Handle any parsing errors
                    eprintln!("Error: {}", e);
                }
                _ => {}
            }
        }
    } else {
        eprintln!("Error: svn command failed");
    }
    paths
}

fn get_relative_path() -> String {
    let output = Command::new("svn")
        .args(["info", "--xml"])
        .arg("--username")
        .arg("priyadarshan.giri")
        .arg("--password")
        .arg("this1sforAFS")
        .output()
        .expect("Failed to execute command");

    let mut relative_path = String::new();

    if output.status.success() {
        let parser = EventReader::new(output.stdout.as_slice());

        let mut within_entry = false;
        let mut within_relative_url = false;

        // Iterate over the XML events
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    if name.local_name == "entry" {
                        within_entry = true;
                    } else if name.local_name == "relative-url" && within_entry {
                        within_relative_url = true;
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "entry" {
                        within_entry = false;
                    } else if name.local_name == "relative-url" {
                        within_relative_url = false;
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    if within_relative_url {
                        // Store the path
                        relative_path = text;
                    }
                }
                Err(e) => {
                    // Handle any parsing errors
                    eprintln!("Error: {}", e);
                }
                _ => {}
            }
        }
    } else {
        eprintln!("Error: svn command failed");
    }

    relative_path.replace('^', "")
}
