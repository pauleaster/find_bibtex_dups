use std::{fs, io::{self, Error}};
use biblatex::Bibliography;

fn read_bibliography_file(path: String) -> Result<Bibliography, Error>{
    let bibtex_file = fs::read_to_string(path)?;
    let opt_bib = Bibliography::parse(&bibtex_file);
    match opt_bib {
        Some(bib) => Ok(bib),
        None => Err(io::Error::new(io::ErrorKind::Other,"Invalid Bib file")),
    }

}

fn main() -> Result<(),Error>{
    let bib = read_bibliography_file("FinalBib.bib".to_string())?;
    
   println!("{:?}",bib);
   Ok(())
}
