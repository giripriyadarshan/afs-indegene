use clap::Parser;
use std::process::Command;
use url::Url;

mod args;

fn main() {
    let args = args::Arguments::parse();
    let svn_url = Url::parse(args.svn_url.as_str()).unwrap();
    let folder_path = svn_url.path_segments().unwrap().next().unwrap();

    // check if folder exists
    if !std::path::Path::new(folder_path).exists() {
        // create folder
        std::fs::create_dir(folder_path).unwrap();

        Command::new("svn")
            .arg("checkout")
            .arg(args.svn_url)
            .arg(folder_path)
            .arg("--username")
            .arg("priyadarshan.giri")
            .arg("--password")
            .arg("this1sforAFS")
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("svn")
            .arg("update")
            .arg("--username")
            .arg("priyadarshan.giri")
            .arg("--password")
            .arg("this1sforAFS")
            .current_dir(folder_path)
            .output()
            .expect("failed to execute process");
    }

    Command::new("afs-cli")
        .arg(args.run_code)
        .current_dir(folder_path)
        .output()
        .expect("failed to execute process");
}
