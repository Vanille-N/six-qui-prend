use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Points(u64);

impl Points {
    pub fn null() -> Self {
        Self(0)
    }
}

impl std::ops::Add for Points {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::AddAssign for Points {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<f64> for &Points {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Card(u8);

impl Card {
    pub fn new(num: u8) -> Self {
        Self(num)
    }

    pub fn number(&self) -> u8 {
        self.0
    }

    pub fn score(&self) -> Points {
        if self.0 == 55 {
            Points(7)
        } else if self.0 % 11 == 0 {
            Points(5)
        } else if self.0 % 10 == 0 {
            Points(3)
        } else if self.0 % 5 == 0 {
            Points(2)
        } else {
            Points(1)
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{: >3}]", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Stack {
    top: Card,
    cards: Vec<Card>,
    size: usize,
    points: Points,
}

impl Stack {
    pub fn reset(&mut self) {
        self.top = Card(0);
        self.cards.clear();
        self.size = 0;
        self.points = Points::null();
    }

    pub fn incr(&mut self, c: Card) {
        self.top = c;
        self.cards.push(c);
        self.size += 1;
        self.points += c.score();
    }

    pub fn new() -> Self {
        Self {
            top: Card(0),
            cards: Vec::new(),
            size: 0,
            points: Points::null(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn top(&self) -> Card {
        self.top
    }

    pub fn points(&self) -> Points {
        self.points
    }

    pub fn push(&mut self, c: Card) -> Points {
        let ans = if self.size == 5 || self.top >= c {
            let pts = self.points;
            self.reset();
            pts
        } else {
            Points(0)
        };
        self.incr(c);
        ans
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.cards.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_until_six() {
        let mut stk = Stack::new();
        // 1st card
        assert_eq!(stk.push(Card(3)), Points(0));
        assert_eq!(stk.top(), Card(3));
        assert_eq!(stk.size(), 1);
        assert_eq!(stk.points(), Points(1));
        // 2nd card
        assert_eq!(stk.push(Card(5)), Points(0));
        assert_eq!(stk.points(), Points(3));
        // 3rd, 4th, 5th cards
        assert_eq!(stk.push(Card(10)), Points(0));
        assert_eq!(stk.push(Card(11)), Points(0));
        assert_eq!(stk.push(Card(15)), Points(0));
        assert_eq!(stk.top(), Card(15));
        assert_eq!(stk.size(), 5);
        assert_eq!(stk.points(), Points(13));
        // 6th card resets the stack
        let res = stk.push(Card(55));
        assert_eq!(res, Points(13));
        assert_eq!(stk.top(), Card(55));
        assert_eq!(stk.size(), 1);
        assert_eq!(stk.points(), Points(7));
    }

    #[test]
    fn underflow() {
        let mut stk = Stack::new();
        assert_eq!(stk.push(Card(15)), Points(0));
        assert_eq!(stk.push(Card(10)), Points(2));
        assert_eq!(stk.push(Card(3)), Points(3));
        assert_eq!(stk.push(Card(4)), Points(0));
        assert_eq!(stk.push(Card(1)), Points(2));
    }
}
