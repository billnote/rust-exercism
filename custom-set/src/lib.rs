#[derive(PartialEq)]
#[derive(Debug)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T>
where
    T: Ord + Eq + Clone,
{
    pub fn new(values: Vec<T>) -> Self {
        let mut tmp = values.clone();
        tmp.sort();

        let mut vec_set: Vec<T> = Vec::new();
        for i in tmp {
            if !vec_set.contains(&i) {
                vec_set.push(i.clone());
            }
        }

        CustomSet(vec_set)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.0.contains(item)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.len() > other.len() {
            false
        } else {
            let mut n: usize = 0;
            let mut m: usize = 0;

            while n < other.len() && m < self.len() && n >= m {
                if other.0[n] == self.0[m] {
                    m += 1;
                } else {
                    n -= m;
                    m = 0;
                }
                n += 1;
            }

            return m == self.len();
        }
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.iter().all(|i| !other.contains(i))
    }

    pub fn add(&mut self, item: T) {
        let mut insert_idx = 0;
        for i in 0..self.len() {
            if self.0[i] > item {
                insert_idx = i;
            } else if self.0[i] == item {
                return;
            }
        }

        self.0.insert(insert_idx, item);
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut inters: Vec<T> = Vec::new();
        self.0.iter().for_each(|i| if other.contains(i) {
            inters.push(i.clone());
        });

        CustomSet(inters)
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut inters: Vec<T> = Vec::new();
        self.0.iter().for_each(|i| if !other.contains(i) {
            inters.push(i.clone());
        });

        CustomSet(inters)
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = CustomSet(self.0.clone());
        for i in &other.0 {
            result.add(i.clone());
        }

        result
    }
}
