use clap::Parser;
use std::process::Command;
use url::Url;

mod args;
mod send_status_message;

use send_status_message::send_message;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = args::Arguments::parse();
    let svn_url = Url::parse(args.svn_url.as_str()).unwrap();
    let folder_path = svn_url.path_segments().unwrap().next().unwrap();

    send_message(
        args.run_code.clone(),
        format!("START | SUCCESS | NOT REQUIRED"),
    )
    .await;

    // check if folder exists
    if !std::path::Path::new(folder_path).exists() {
        // create folder
        std::fs::create_dir(folder_path).unwrap();

        match Command::new("svn")
            .arg("checkout")
            .arg(args.svn_url)
            .arg(folder_path)
            .arg("--username")
            .arg("priyadarshan.giri")
            .arg("--password")
            .arg("second1forAFS")
            .output()
        {
            Ok(_) => {}
            Err(e) => {
                send_message(
                    args.run_code.clone(),
                    format!("SVN CHECKOUT | FAILED | {}", e),
                )
                .await;
                return Ok(());
            }
        }
    } else {
        match Command::new("svn")
            .arg("update")
            .arg("--username")
            .arg("priyadarshan.giri")
            .arg("--password")
            .arg("this1sforAFS")
            .current_dir(folder_path)
            .output()
        {
            Ok(_) => {}
            Err(e) => {
                send_message(
                    args.run_code.clone(),
                    format!("SVN UPDATE | FAILED | {}", e),
                )
                .await;
                return Ok(());
            }
        }
    }

    match Command::new("afs-cli")
        .arg(args.run_code.clone())
        .current_dir(folder_path)
        .output()
    {
        Ok(_) => {}
        Err(e) => {
            send_message(args.run_code, format!("AFS CLI | FAILED | {}", e)).await;
            return Ok(());
        }
    }

    Ok(())
}
