use std::collections::HashMap;

pub struct Biology<'a> {
    genetic_code: HashMap<&'a str, &'a str>,
}

impl<'a> Biology<'a> {
    pub fn name_for(&self, codon: &'a str) -> Result<&'a str, &'static str> {
        match self.genetic_code.get(codon) {
            Some(name) => Ok(name),
            None => Err("error codon."),
        }
    }

    pub fn of_rna(&self, rna: &str) -> Result<Vec<&'a str>, &'static str> {
        if rna.len() % 3 != 0 {
            Err("error rna lenght.")
        } else {
            let mut stop = 0;
            let names = rna.chars()
                .collect::<Vec<char>>()
                .chunks(3)
                .enumerate()
                .filter_map(|(idx, chars)| {
                    let codon = chars.iter().collect::<String>();
                    match self.genetic_code.get(codon.as_str()) {
                        Some(name) => {
                            if *name == "stop codon" && stop == 0 {
                                stop = idx;
                            }
                            Some(*name)
                        }
                        None => None,
                    }
                })
                .collect::<Vec<&'a str>>();

            if names.len() != rna.len() / 3 {
                Err("error rna.")
            } else {
                Ok(if stop > 0 {
                    names.get(..stop).unwrap().to_vec()
                } else {
                    names
                })
            }
        }
    }
}


pub fn parse<'a>(paris: Vec<(&'a str, &'a str)>) -> Biology<'a> {
    let mut codes = HashMap::<&'a str, &'a str>::new();
    for (codon, name) in paris {
        codes.insert(codon, name);
    }

    Biology { genetic_code: codes }
}
