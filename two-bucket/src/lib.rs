use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Debug)]
enum Action {
    Pouring,
    Emptying,
    Filling,
}

#[derive(Debug, Clone)]
struct BucketGroup {
    a: BucketItem,
    b: BucketItem,
    steps: HashSet<(u8, u8)>,
}

impl BucketGroup {
    pub fn new(capacity_a: u8, capacity_b: u8) -> Self {
        BucketGroup {
            a: BucketItem::new(capacity_a),
            b: BucketItem::new(capacity_b),
            steps: HashSet::new(),
        }
    }

    fn allowed_action(&self) -> (Vec<Action>, Vec<Action>) {
        (
            self.a.allowed_action(&self.b),
            self.b.allowed_action(&self.a),
        )
    }

    fn start(&mut self, goal: u8, start_bucket: &Bucket) -> Option<BucketGroup> {
        match *start_bucket {
            Bucket::One => {
                self.a = self.a.fill();
            }
            Bucket::Two => {
                self.b = self.b.fill();
            }
        }

        self.steps.insert((self.a.liters, self.b.liters));

        let not_allowed_steps: HashSet<(u8, u8)> = [
            (0, 0),
            (self.a.size, self.b.size),
            (self.a.size, 0),
            (0, self.b.size),
        ].iter()
            .map(|&s| s)
            .collect();

        if self.a.liters == goal || self.b.liters == goal {
            return Some(self.clone());
        }

        self.moves(goal, &not_allowed_steps)
    }

    pub fn solve(&mut self, goal: u8, start_bucket: &Bucket) -> BucketStats {
        let result = self.start(goal, start_bucket);
        let mut stats = BucketStats {
            moves: 0,
            goal_bucket: Bucket::One,
            other_bucket: 0,
        };

        match result {
            Some(g) => {
                stats.moves = g.steps.len() as u8;
                if g.a.liters == goal {
                    stats.other_bucket = g.b.liters;
                } else {
                    stats.other_bucket = g.a.liters;
                    stats.goal_bucket = Bucket::Two;
                }
            }
            None => (),
        }

        stats
    }

    fn moves(&mut self, goal: u8, not_allowed_steps: &HashSet<(u8, u8)>) -> Option<BucketGroup> {
        let (a_actions, b_actions) = self.allowed_action();

        if let Some(g) = self.do_actions(b_actions, false, goal, not_allowed_steps) {
            return Some(g);
        }

        if let Some(g) = self.do_actions(a_actions, true, goal, not_allowed_steps) {
            return Some(g);
        }

        None
    }

    fn do_actions(
        &mut self,
        actions: Vec<Action>,
        move_a: bool,
        goal: u8,
        not_allowed_steps: &HashSet<(u8, u8)>,
    ) -> Option<BucketGroup> {
        for action in actions {
            let (a, b) = if move_a {
                self.move_a(&action)
            } else {
                self.move_b(&action)
            };
            match Self::check_moves(&mut self.steps, &a, &b, goal, not_allowed_steps) {
                Some((matched, mut group)) => {
                    if matched {
                        return Some(group);
                    } else {
                        if let Some(g) = group.moves(goal, not_allowed_steps) {
                            return Some(g);
                        }
                    }
                }
                None => {
                    continue;
                }
            }
        }

        None
    }

    fn move_a(&self, action: &Action) -> (BucketItem, BucketItem) {
        self.a.moves(&action, &self.b)
    }

    fn move_b(&self, action: &Action) -> (BucketItem, BucketItem) {
        let (b, a) = self.b.moves(&action, &self.a);
        (a, b)
    }

    fn check_moves(
        steps: &mut HashSet<(u8, u8)>,
        a: &BucketItem,
        b: &BucketItem,
        goal: u8,
        not_allowed_steps: &HashSet<(u8, u8)>,
    ) -> Option<(bool, BucketGroup)> {
        if (not_allowed_steps.contains(&(a.liters, b.liters)) && a.liters != goal &&
                b.liters != goal) || steps.contains(&(a.liters, b.liters))
        {
            None
        } else {
            steps.insert((a.liters, b.liters));
            let new_group = BucketGroup {
                a: a.clone(),
                b: b.clone(),
                steps: steps.clone(),
            };
            if a.liters == goal || b.liters == goal {
                Some((true, new_group))
            } else {
                Some((false, new_group))
            }
        }
    }
}


#[derive(Clone, Debug)]
struct BucketItem {
    size: u8,
    liters: u8,
}

impl BucketItem {
    pub fn new(size: u8) -> Self {
        BucketItem {
            size: size,
            liters: 0,
        }
    }

    fn is_fill(&self) -> bool {
        self.size == self.liters
    }

    fn is_empty(&self) -> bool {
        self.liters == 0
    }

    fn surplus(&self) -> u8 {
        self.size - self.liters
    }

    fn empty(&self) -> Self {
        BucketItem {
            size: self.size,
            liters: 0,
        }
    }

    fn fill(&self) -> Self {
        BucketItem {
            size: self.size,
            liters: self.size,
        }
    }

    fn pouring_to_another(&self, other: &Self) -> (BucketItem, BucketItem) {
        let surp = other.surplus();
        if surp >= self.liters {
            (
                BucketItem {
                    size: self.size,
                    liters: 0,
                },
                BucketItem {
                    size: other.size,
                    liters: other.liters + self.liters,
                },
            )
        } else {
            (
                BucketItem {
                    size: self.size,
                    liters: self.liters - surp,
                },
                BucketItem {
                    size: other.size,
                    liters: other.size,
                },
            )
        }
    }

    pub fn moves(&self, action: &Action, other: &Self) -> (BucketItem, BucketItem) {
        match *action {
            Action::Emptying => (self.empty(), other.clone()),
            Action::Filling => (self.fill(), other.clone()),
            Action::Pouring => self.pouring_to_another(other),
        }
    }

    pub fn allowed_action(&self, other: &Self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        if !self.is_empty() {
            actions.push(Action::Emptying);
            if !other.is_fill() {
                actions.push(Action::Pouring);
            }
        }
        if !self.is_fill() {
            actions.push(Action::Filling);
        }

        actions
    }
}

/// Solve the bucket problem
/// Disgusting solution, if i have time， i will find some better solutions.
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    BucketGroup::new(capacity_1, capacity_2).solve(goal, start_bucket)
}
