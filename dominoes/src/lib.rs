struct Domino {
    current_stone: (usize, usize),
    remain_stones: Vec<(usize, usize)>,
}

impl Domino {
    pub fn new(dominoes: &Vec<(usize, usize)>) -> Option<Self> {
        if dominoes.is_empty() {
            None
        } else {
            Some(Domino {
                current_stone: dominoes[0],
                remain_stones: dominoes[1..].to_vec(),
            })
        }
    }

    pub fn play(&mut self) -> Option<Vec<(usize, usize)>> {
        let mut chain = Vec::<(usize, usize)>::new();
        self.playing(&mut chain)
    }

    fn playing(&self, chain: &mut Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
        chain.push(self.current_stone);
        if self.remain_stones.len() == 0 {
            if chain[0].0 == self.current_stone.1 {
                Some(chain.clone())
            } else {
                None
            }
        } else {
            match self.allowed_next() {
                Some(next_stones) => {
                    for domino in next_stones {
                        match domino.playing(&mut chain.clone()) {
                            Some(c) => {
                                return Some(c);
                            }
                            None => (),
                        }
                    }
                }
                None => (),
            }

            None
        }
    }

    fn allowed_next(&self) -> Option<Vec<(Self)>> {
        let mut allowed_vec = Vec::<(Self)>::new();
        for i in 0..self.remain_stones.len() {
            let stone = self.remain_stones[i];
            if self.current_stone.1 == stone.0 {
                let mut new_stones = self.remain_stones.clone();
                new_stones.remove(i);
                allowed_vec.push(Domino {
                    current_stone: stone,
                    remain_stones: new_stones,
                });
            } else if self.current_stone.1 == stone.1 {
                let mut new_stones = self.remain_stones.clone();
                new_stones.remove(i);
                allowed_vec.push(Domino {
                    current_stone: (stone.1, stone.0),
                    remain_stones: new_stones,
                });
            }
        }

        if allowed_vec.len() > 0 {
            Some(allowed_vec)
        } else {
            None
        }
    }
}

pub fn chain(dominoes: &Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    if dominoes.is_empty() {
        return Some(vec![]);
    }

    if let Some(mut domino) = Domino::new(dominoes) {
        domino.play()
    } else {
        None
    }
}
