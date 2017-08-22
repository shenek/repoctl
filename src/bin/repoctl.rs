#[macro_use]
extern crate clap;
extern crate walkdir;
extern crate repoctl;

use clap::{App,AppSettings};
use walkdir::WalkDir;
use repoctl::repository::*;

fn main() {
	// Loading the arguments description & parse it
	let yaml = load_yaml!("repoctl.yml");
	let matches = App::from_yaml(yaml)
			.setting(AppSettings::SubcommandRequiredElseHelp)
			.setting(AppSettings::VersionlessSubcommands)
			.get_matches();

	let mut verbosity = matches.occurrences_of("verbose");

	match matches.subcommand() {
		("show",Some(show_matches)) => {
			let mut all = false;
			verbosity += show_matches.occurrences_of("verbose");
			match show_matches.subcommand() {
				("all", Some(all_matches)) => {
					all = true;
					verbosity += all_matches.occurrences_of("verbose");
				},
				(boh,_) => {
					println!("command {} unknown", boh);
					println!("{}", show_matches.usage()); // it's not working :(
				},
			}
			let repodirs = [ "/etc/pkg", "/usr/local/etc/pkg/repos" ];
			let mut repos: Vec<Repo> = Vec::new();
			for repodir in repodirs.into_iter() {
				for repofile in WalkDir::new(repodir)
					.into_iter()
					.filter_map( |e| e.ok())
					.filter( |e| e.file_type().is_file())
					.filter( |e| e.path().extension().unwrap_or( std::ffi::OsStr::new("")) == "conf" ) {
					if verbosity > 0 {
						println!("Parsing file: {:?}", repofile.path());
					}
					for r in multi_parse_filename( repofile.path() ) {
						merge_repo( &mut repos, r);
					}
				}
			}
			if all {
				println!("Available repos:");
				for r in repos.iter() {
					println!("\trepo: {}", r.name);
					println!("\turl: {}", r.url);
					println!("\tenabled: {}", r.enabled);
					println!("");
				}
			} else {
				println!("Enabled repos:");
				for r in repos.iter().filter(|x| x.enabled) {
					println!("\trepo: {}", r.name);
					println!("\turl: {}", r.url);
				}
			}
		},
		(boh,_) => {
			println!("command {} unknown", boh);
			println!("{}", matches.usage()); // it's not working :(
		},
	}
}
