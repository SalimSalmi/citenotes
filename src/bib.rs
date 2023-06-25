use std::fs::{OpenOptions, File};
use std::io::Write;
use std::error::Error;
use std::fmt;
// use std::env;
use biblatex::{Bibliography, Entry, DateValue, PermissiveType};

#[derive(Debug)]
pub struct BibError {
    message: String,
}

type BibResult<T> = Result<T, BibError>;

impl Error for BibError {}

impl fmt::Display for BibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = format!("BibError: {}", self.message);
        write!(f, "{}", message)
    }
}

fn create_biberror(message: &str) -> BibError {
    BibError { message: String::from(message) }
}


fn get_year(entry: &Entry) -> BibResult<String> {
    let dateresult = entry.date()
        .map_err(|_| create_biberror("Date parse failed."))?;

    match dateresult {
        PermissiveType::Typed(datevalue) => Ok(
            match datevalue.value {
                DateValue::At(date) |
                DateValue::After(date) |
                DateValue::Before(date) |
                DateValue::Between(_, date) => date.year,
            }.to_string()
        ),
        PermissiveType::Chunks(_) => Err(create_biberror("No date found."))
    }
}

fn remove_accented_letters(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for c in input.chars() {
        let normalized_char = match c {
            'á' | 'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            _ => c,
        };

        output.push(normalized_char);
    }

    output
}

fn get_name(entry: &Entry) -> BibResult<String> {
    match entry.author() {
        Ok(authors) => match authors.get(0) {
            Some(author) => Ok(remove_accented_letters(author.name.as_str())),
            None => Err(create_biberror("No authors found.")) 
        },
        Err(_) => Err(create_biberror("Author parse error.")) 
    }
}

fn get_bibkey(entry: &Entry) -> BibResult<String> {
    let name = get_name(entry)?;
    let year = get_year(entry)?;
    Ok(format!("{}{}", name, year))
}

fn get_entry(bibliography: Bibliography) -> BibResult<Entry> { 
    match bibliography.iter().next() {
        Some(entry) => Ok(entry.clone()),
        None => Err(create_biberror("No entry found.")),
    }
}

fn parse_bib(raw_bibtex: &str) -> BibResult<Bibliography> {
    Bibliography::parse(raw_bibtex)
        .map_err(|_| create_biberror("Bibtex parse error")) 
}

pub fn entry_from_bibtext(raw_bibtex: &str) -> BibResult<Entry> {    
    let bibliography = parse_bib(raw_bibtex)?;
    let mut entry = get_entry(bibliography)?;
    let key = get_bibkey(&entry)?;
    entry.key = key.clone();
    Ok(entry)
}

pub struct DB {
    bibliography: Bibliography
}

enum KeyDoiResult {
    Error(BibError),
    NoKey,
    KeyExists,
    DoiExists,
}

impl DB {

    pub fn read() -> BibResult<File> {
        let file_path = "bib.bib";
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|_| create_biberror("Failed to open bib file."))
    }

    pub fn load() -> BibResult<DB> {
        let file = DB::read()?;
        let contents = std::io::read_to_string(file)
            .map_err(|_| create_biberror("Failed to read bib file."))?;        
        let bibliography = parse_bib(contents.as_str())?;
        
        Ok(DB { bibliography })
    }

    pub fn save(&self) -> BibResult<()>{
        let output = self.bibliography.to_bibtex_string();
        let mut file = DB::read()?;
        file.write_all(output.as_bytes())
            .map_err(|_| create_biberror("Failed to load bib file."))?;

        Ok(())
    }

    pub fn get_bibliography(&self) -> &Bibliography {
        &self.bibliography
    }
    
    fn lookup_entry_exists(&self, key: &String, doi: &String) -> KeyDoiResult {
        let entry = match self.bibliography.get(key) {
            Some(entry) => entry,
            None => return KeyDoiResult::NoKey
        };

        let doi_res = match entry.doi() {
            Ok(doi_res) => doi_res,
            Err(_) => return KeyDoiResult::Error(create_biberror("Error while parsing doi from Entry."))
        };

        if &doi_res == doi {
            KeyDoiResult::DoiExists
        } else {
            KeyDoiResult::KeyExists
        }
    }

    pub fn add(&mut self, entry: Entry) -> BibResult<()> {
        let mut entry = entry.clone();
        let doi = entry.doi()
            .map_err(|_| create_biberror("Error while parsing doi from Entry."))?;
        
        for ch in 'a'..'z' {
            match self.lookup_entry_exists(&entry.key, &doi) {
                KeyDoiResult::KeyExists => entry.key.push(ch),
                KeyDoiResult::DoiExists | KeyDoiResult::NoKey => break,
                KeyDoiResult::Error(err) => return Err(err) 
            } 
        }

        self.bibliography.insert(entry);
        Ok(())
    }
}





