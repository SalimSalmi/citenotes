mod doi;
mod http;
mod bib;
mod md;
use std::env::args;
use std::error::Error;

fn main() {
    let arg:String = get_arguments()
        .unwrap_or_else(|| panic!("{:?}", "Not enough command line arguments."));
    new_note_from_doi(arg)
        .unwrap_or_else(|err| panic!("{:?}", err)); 
}

fn new_note_from_doi(arg: String) -> Result<(), Box<dyn Error>> {
    // Format doi 
    let doi_name = doi::doi_from_url(&arg);
    let url = format!("http://dx.doi.org/{}", doi_name);

    // Http request for bib data 
    let response = http::request_bib_with_doi(&url)?
        .replace("DOI", "doi");

    // Parse new entry from http reponse 
    let entry = bib::entry_from_bibtext(response.as_str())?;

    // Load bibliography, add new entry and save 
    let mut db = bib::DB::load()?;
    db.add(entry)?;
    db.save()?;

    // Create markdown notes 
    md::create_notes(db.get_bibliography())?;

    Ok(())
}

// Handle command line arguments 
fn get_arguments() -> Option<String> {
    let args: Vec<String> = args().collect();
    println!("{}", args[0]);
    if args.len() > 1 { Some(String::from(&args[1])) }
    else { None } 
}

