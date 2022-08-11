# photoorganizer
A very simple tool to copy photos (or any file) from an SD to a folder while organizing the files by year/month/day 

I'm learning Rust, so it's a good excuse to write something simple as this.

Usage:

>\> cargo run from_dir/ to_dir/

The tool copies all the files from `from_dir` to `to_dir/yyyy/mm/dd/` where `yyyy/m/dd/` is the creation date of the file.
