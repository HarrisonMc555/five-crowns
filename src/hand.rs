use crate::card::Card;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Hand { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn try_from(string: &str) -> Option<Self> {
        let string = string.trim();
        // Split will try to parse a single card from the empty string, we don't want
        // that. Currently, a blank string is a valid input for an empty hand.
        if string.is_empty() {
            return Some(Hand::new(Vec::new()));
        }
        Some(Hand::new(
            string
                .split(',')
                .map(|w| Card::try_from(w.trim()))
                .collect::<Option<Vec<_>>>()?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from() {
        // Valid
        assert_eq!(
            Some(Hand::new(vec![
                Card::try_from("3S").unwrap(),
                Card::try_from("5C").unwrap()
            ])),
            Hand::try_from("3S,5C")
        );
        // Leading/trailing spaces are ok
        assert_eq!(
            Some(Hand::new(vec![
                Card::try_from("10H").unwrap(),
                Card::try_from("QD").unwrap()
            ])),
            Hand::try_from("10H, QD")
        );
        assert_eq!(
            Some(Hand::new(vec![
                Card::try_from("10H").unwrap(),
                Card::try_from("QD").unwrap()
            ])),
            Hand::try_from("   10H   ,    QD   ")
        );
        // eprintln!("{:?}", "".split(',').collect::<Vec<_>>());
        // Single card is ok
        assert_eq!(
            Some(Hand::new(vec![
                Card::try_from("KR").unwrap(),
            ])),
            Hand::try_from("KR")
        );
        // Empty is ok
        assert_eq!(Some(Hand::new(Vec::new())), Hand::try_from(""));
        // No trailing commas
        assert_eq!(None, Hand::try_from("3S,"));
        // Invalid rank
        assert_eq!(None, Hand::try_from("11C"));
        // Invalid suit
        assert_eq!(None, Hand::try_from("7A"));
    }
}
