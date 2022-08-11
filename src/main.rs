use chrono::Datelike;
use chrono::NaiveDateTime;
use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Copies the (raw) files from [srcdir] to [todir] and");
        println!("organizes them into folders by date yyyy/mm/dd\n\n");
        println!("Usage: ./cmd [srcdir] [todir]\n");
        return Ok(());
    }

    let srcdir_path = &args[1];
    let todir_path = &args[2];

    let mut copied_count = 0;

    let dir1 = fs::read_dir(srcdir_path)?;
    for file in dir1 {
        let f = &file?;
        let meta = f.metadata()?;
        if meta.is_file() {
            let name = f.file_name();
            let date_time = NaiveDateTime::from_timestamp(meta.ctime(), 0);
            let year = date_time.date().year();
            let month = date_time.date().month();
            let day = date_time.date().day();

            let final_dest = format!(
                "{}/{}/{}/{}",
                &todir_path,
                &year.to_string(),
                &month.to_string(),
                &day.to_string()
            );

            // try create folders in the destination: year/month/day.
            fs::create_dir_all(&final_dest)?;

            // copy the file to the destination folders.
            fs::copy(
                f.path().to_str().unwrap(),
                format!("{}/{}", &final_dest, name.to_str().unwrap()),
            )?;

            println!("Copied {}", name.to_str().unwrap());
            copied_count += 1;
        }
    }

    println!("==============");
    println!("Copied {} files", copied_count);

    Ok(())
}
