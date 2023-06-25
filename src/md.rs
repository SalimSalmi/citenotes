use std::{error, fmt};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use biblatex::{Bibliography, Entry, RetrievalError};

#[derive(Debug)]
pub struct MDError {
    message: String,
}

impl error::Error for MDError {}

impl fmt::Display for MDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = format!("BibError: {}", self.message);
        write!(f, "{}", message)
    }
}

fn create_mderror(message: &str) -> MDError {
    MDError { message: String::from(message) }
}

fn create_filename(key:&String) -> String {
    format!("{}.md",key)
}

fn file_exists(key: &String) -> bool {
    let filename = create_filename(key);
    let path = Path::new(filename.as_str());
    path.exists()
}


fn create_and_write_to_file(filename: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_md_output(entry: &Entry) -> Result<String,  RetrievalError>{
    let title = entry.title()?[0].v.get();
    let authors: String = entry.author()?
        .iter()
        .map(|person| person.to_string())
        .collect::<Vec<String>>()
        .join(" and ");
    let url = entry.url()?;

    Ok(format!("# {}\n\n*{}*\n{}\n---\n", title, authors, url))
}

pub fn create_notes(bibliography: &Bibliography) -> Result<(), MDError>{
    for entry in bibliography.iter() {
        if !file_exists(&entry.key) {
            let filename = create_filename(&entry.key);
            let output = create_md_output(entry)
                .map_err(|_| create_mderror("Failed to create markdown formatted output."))?;
            create_and_write_to_file(filename.as_str(), output.as_str())
                .map_err(|_| create_mderror("Failed to create markdown file."))?;
        } 
    }
    Ok(())
}

