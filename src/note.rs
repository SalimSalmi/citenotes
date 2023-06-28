use std::io;
use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use biblatex::{Bibliography, Entry};
use citenotes::CiteNoteError;

fn create_filename(key:&String) -> String {
    format!("{}.md",key)
}

fn file_exists(key: &String) -> bool {
    let filename = create_filename(key);
    let path = Path::new(filename.as_str());
    path.exists()
}

fn create_and_write_to_file(filename: &str, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn get_output_field(entry: &Entry) -> citenotes::Result<(String, String, String)>{
    let title = entry.title()
        .map_err(|_| CiteNoteError::new("Failed to get title field"))?;

    if title.len() == 0 {
        return Err(CiteNoteError::new("Title field is empty"));
    }

    let title =  String::from(title[0].v.get());
    let authors: String = entry.author()
        .map_err(|_| CiteNoteError::new("Failed to get author field"))?
        .iter()
        .map(|person| person.to_string())
        .collect::<Vec<String>>()
        .join(" and ");
    let url = entry.url()
        .map_err(|_| CiteNoteError::new("Failed to get url field"))?;

    Ok((title, authors, url))
}

fn create_md_output(entry: &Entry) -> citenotes::Result<String>{
    let (title, authors, url) = get_output_field(entry)?;
    Ok(format!("# {}\n\n*{}*\n{}\n---\n#no_notes", title, authors, url))
}

pub fn new(bibliography: &Bibliography) -> Result<(), Box<dyn Error>>{
    for entry in bibliography.iter() {
        if !file_exists(&entry.key) {
            let filename = create_filename(&entry.key);
            let output = create_md_output(entry)?;
            create_and_write_to_file(filename.as_str(), output.as_str())?;
        } 
    }
    Ok(())
}

