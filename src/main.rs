use std::{fs, io::{self, Error}};
use biblatex::{Bibliography, Chunk};

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
    
//    println!("{:?}",bib);
    let mut i=0;

    for  bib1 in bib {
        let fields = bib1.fields;
        if let Some(title) = fields.get("title") {
            // println!("{:?}",&title);
            match &title[0] {
                Chunk::Normal(s) =>  {
                    println!("N:{}",&s);
                },
                Chunk::Verbatim(s) => {
                    println!("V:{}",&s);
                    if s == "Black hole excision for dynamic black holes" {
                        i += 10;
                    }
                }
            }


            i += 1;

        }
    

    }


    Ok(())
}
