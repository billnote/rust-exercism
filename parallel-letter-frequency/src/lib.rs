use std::collections::HashMap;

use std::thread;
use std::sync::mpsc;

pub fn frequency(text: &[&str], thread_size: usize) -> HashMap<char, usize> {
    let mut result = HashMap::<char, usize>::new();
    let (tx, rx): (mpsc::Sender<HashMap<char, usize>>, mpsc::Receiver<HashMap<char, usize>>) =
        mpsc::channel();
    let mut data_group = (0..thread_size)
        .map(|_| Vec::<String>::new())
        .collect::<Vec<Vec<String>>>();
    let mut thread_count = 0;

    for i in 0..text.len() {
        data_group[i % thread_size].push(String::from(text[i]));
    }

    data_group.iter().filter(|s| !s.is_empty()).for_each(|s| {
        let thread_tx = tx.clone();
        let data = s.clone();
        thread_count += 1;

        thread::spawn(move || {
            let mut sub_frequency = HashMap::<char, usize>::new();
            data.iter().for_each(|s| {
                s.to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .for_each(|c| {
                        let entry = sub_frequency.entry(c).or_insert(0);
                        *entry += 1;
                    })
            });

            thread_tx.send(sub_frequency).unwrap();
        });
    });

    while thread_count > 0 {
        let sub_frequency = rx.recv().unwrap();
        sub_frequency.iter().for_each(|(k, v)| {
            let entry = result.entry(*k).or_insert(0);
            *entry += v;
        });
        thread_count -= 1;
    }

    result
}
