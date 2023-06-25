pub fn doi_from_url(arg: &str) -> String {
    let patterns = [ 
        "doi.org/",
        "doi:",
    ];
    let doi: String = arg.split_whitespace().collect();
    
    for pattern in patterns {
        if let Some(index) = doi.find(pattern) {
            let trimmed = doi[(index + pattern.len())..].to_string();
            return trimmed;
        }
    }

    doi
}
