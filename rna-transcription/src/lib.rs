#[derive(PartialEq)]
#[derive(Debug)]
pub struct RNA {
    nucleotides: String,
}

pub struct DNA {
    nucleotides: String,
}

impl RNA {
    pub fn new(nucleotides: &str) -> Self {
        RNA { nucleotides: String::from(nucleotides) }
    }
}

impl DNA {
    pub fn new(nucleotides: &str) -> Self {
        DNA { nucleotides: String::from(nucleotides) }
    }

    pub fn to_rna(&self) -> Result<RNA, String> {
        if self.nucleotides.chars().all(|c| match c {
            'G' | 'C' | 'T' | 'A' => true,
            _ => false,
        })
        {
            Ok(RNA {
                nucleotides: self.nucleotides
                    .chars()
                    .map(|c| match c {
                        'G' => 'C',
                        'C' => 'G',
                        'T' => 'A',
                        'A' => 'U',
                        _ => ' ',
                    })
                    .collect::<String>(),
            })
        } else {
            Err(String::from("Error symbol!"))
        }
    }
}
