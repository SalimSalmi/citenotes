use std::io::Write;
use std::fs::{OpenOptions, File};
use biblatex::{Bibliography, Entry, DateValue, PermissiveType};
use citenotes::CiteNoteError;

fn get_year(entry: &Entry) -> citenotes::Result<String> {
    let dateresult = entry.date()
        .map_err(|_| CiteNoteError::new("Date parse failed."))?;

    match dateresult {
        PermissiveType::Typed(datevalue) => Ok(
            match datevalue.value {
                DateValue::At(date) |
                DateValue::After(date) |
                DateValue::Before(date) |
                DateValue::Between(_, date) => date.year,
            }.to_string()
        ),
        PermissiveType::Chunks(_) => Err(CiteNoteError::new("No date found."))
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

fn get_name(entry: &Entry) -> citenotes::Result<String> {
    match entry.author() {
        Ok(authors) => match authors.get(0) {
            Some(author) => Ok(remove_accented_letters(author.name.as_str())),
            None => Err(CiteNoteError::new("No authors found.")) 
        },
        Err(_) => Err(CiteNoteError::new("Author parse error.")) 
    }
}

fn get_bibkey(entry: &Entry) -> citenotes::Result<String> {
    let name = get_name(entry)?;
    let year = get_year(entry)?;
    Ok(format!("{}{}", name, year))
}

fn get_entry(bibliography: Bibliography) -> citenotes::Result<Entry> { 
    match bibliography.iter().next() {
        Some(entry) => Ok(entry.clone()),
        None => Err(CiteNoteError::new("No entry found.")),
    }
}

fn parse_bib(raw_bibtex: &str) -> citenotes::Result<Bibliography> {
    Bibliography::parse(raw_bibtex)
        .map_err(|_| CiteNoteError::new("Bibtex parse error")) 
}

pub fn entry_from_bibtext(raw_bibtex: &str) -> citenotes::Result<Entry> {    
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
    Error(CiteNoteError),
    NoKey,
    KeyExists,
    DoiExists,
}

impl DB {
    pub fn read() -> citenotes::Result<File> {
        let file_path = "bib.bib";
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|_| CiteNoteError::new("Failed to open bib file."))
    }

    pub fn load() -> citenotes::Result<DB> {
        let file = DB::read()?;
        let contents = std::io::read_to_string(file)
            .map_err(|_| CiteNoteError::new("Failed to read bib file."))?;        
        let bibliography = parse_bib(contents.as_str())?;
        
        Ok(DB { bibliography })
    }

    pub fn save(&self) -> citenotes::Result<()>{
        let output = self.bibliography.to_bibtex_string();
        let mut file = DB::read()?;
        file.write_all(output.as_bytes())
            .map_err(|_| CiteNoteError::new("Failed to load bib file."))?;

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
            Err(_) => return KeyDoiResult::Error(CiteNoteError::new("Error while parsing doi from Entry."))
        };

        if &doi_res == doi {
            KeyDoiResult::DoiExists
        } else {
            KeyDoiResult::KeyExists
        }
    }

    pub fn add(&mut self, entry: Entry) -> citenotes::Result<()> {
        let mut entry = entry.clone();
        let doi = entry.doi()
            .map_err(|_| CiteNoteError::new("Error while parsing doi from Entry."))?;
        
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





