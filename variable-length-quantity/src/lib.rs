/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
const FLAG: u8 = 0x80;
const HIGH_BYTE_MAX: u8 = 0x0F;
const VLQ_PAIR: &'static [(u32, usize)] = &[
    (0xF0000000, 28),
    (0xFE00000, 21),
    (0x1FC000, 14),
    (0x3F80, 7),
    (0x7F, 0),
];

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    values.iter().fold(&mut result, |acc, u| {
        if *u > 0 {
            VLQ_PAIR
                .iter()
                .skip_while(|pair| (pair.0 & u) == 0)
                .map(|pair| (pair.0 & u, pair.1))
                .for_each(|pair| if pair.1 > 0 {
                    acc.push((pair.0 >> pair.1) as u8 + FLAG);
                } else {
                    acc.push(pair.0 as u8);
                });
        } else {
            acc.push(0);
        }

        acc
    });

    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    if bytes[bytes.len() - 1] >= FLAG {
        return Err("error byte.");
    } else {
        let mut group: Vec<Vec<u8>> = Vec::new();
        let mut item: Vec<u8> = Vec::new();
        for b in bytes {
            if *b >= FLAG {
                item.push(b - FLAG);
            } else {
                item.push(*b);
                group.push(item);

                item = Vec::new();
            }
        }

        if group.iter().any(|items| {
            items.len() > 5 || (items.len() == 5 && *items.get(0).unwrap_or(&0xFF) > HIGH_BYTE_MAX)
        })
        {
            Err("overflow u32 ")
        } else {
            Ok(
                group
                    .iter()
                    .map(|items| {
                        items.iter().enumerate().fold(0u32, |acc, (idx, item)| {
                            acc + ((*item as u32) << VLQ_PAIR[VLQ_PAIR.len() - items.len() + idx].1)
                        })
                    })
                    .collect::<Vec<u32>>(),
            )
        }
    }
}
