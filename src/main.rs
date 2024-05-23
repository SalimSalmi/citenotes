mod doi;
mod http;
mod bib;
mod note;

use std::env::args;
use std::error::Error;
use std::process;

fn main() {
    let arg:String = get_arguments()
        .unwrap_or_else(|| {
            eprintln!("{:?}", "Not enough command line arguments.");
            process::exit(1);
        });
    new_note_from_doi(arg)
        .unwrap_or_else(|err| exit_on_error(err)); 
}

fn exit_on_error(err: Box<dyn Error>) {
    eprintln!("{}", err);
    process::exit(1);
}

fn new_note_from_doi(arg: String) -> Result<(), Box<dyn Error>> {
    // Format doi 
    let doi_name = doi::doi_from_url(&arg);
    let url = format!("http://dx.doi.org/{}", doi_name);

    // Http request for bib data 
    let response = http::request_bib_with_doi(&url)?
        .replace("DOI", "doi");
    // println!("{}",response.as_str());

    // Parse new entry from http reponse 
    let entry = bib::entry_from_bibtext(response.as_str())?;

    // Load bibliography 
    let mut db = bib::DB::load()?;

    // Print key
    println!("{}",&entry.key);

    // Add new entry 
    db.add(entry)?;

    // Create markdown notes 
    note::new(db.get_bibliography())?;

    // Save to .bib 
    db.save()?;

    Ok(())
}

// Handle command line arguments 
fn get_arguments() -> Option<String> {
    let args: Vec<String> = args().collect();
    println!("{}", args[0]);
    if args.len() > 1 { Some(String::from(&args[1])) }
    else { None } 
}

