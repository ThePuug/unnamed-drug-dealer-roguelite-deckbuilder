// Cards module - Manages deck, hand, and played cards for each Owner

use super::card::Card;
use rand::seq::SliceRandom;

/// Card collections for a single Owner (Narc, Player, or Buyer)
#[derive(Clone)]
pub struct Cards {
    pub deck: Vec<Card>,
    pub hand: [Option<Card>; 3],
    pub played: Vec<Card>, // Only Buyer uses this for tracking
}

impl Cards {
    /// Create new Cards with a deck
    pub fn new(deck: Vec<Card>) -> Self {
        Self {
            deck,
            hand: [None, None, None],
            played: Vec::new(),
        }
    }

    /// Create empty Cards (for Buyer before persona is selected)
    pub fn empty() -> Self {
        Self {
            deck: Vec::new(),
            hand: [None, None, None],
            played: Vec::new(),
        }
    }

    /// Draw cards from deck into empty hand slots
    pub fn draw_to_hand(&mut self) {
        for slot in &mut self.hand {
            if slot.is_none() && !self.deck.is_empty() {
                *slot = Some(self.deck.remove(0));
            }
        }
    }

    /// Shuffle unplayed hand cards back into deck
    pub fn shuffle_back(&mut self) {
        self.collect_unplayed();
        self.shuffle_deck();
    }

    /// Shuffle the deck
    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }

    /// Collect unplayed cards from hand into deck (doesn't shuffle)
    pub fn collect_unplayed(&mut self) {
        for slot in &mut self.hand {
            if let Some(card) = slot.take() {
                self.deck.push(card);
            }
        }
    }

    /// Collect all cards (unplayed hand + played) into deck (doesn't shuffle)
    pub fn collect_all(&mut self) {
        self.collect_unplayed();
        self.deck.append(&mut self.played);
    }
}

impl From<&Cards> for Vec<Card> {
    fn from(cards: &Cards) -> Vec<Card> {
        cards.hand.iter().filter_map(|s| s.clone()).collect()
    }
}
// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::card::CardType;

    #[test]
    fn test_draw_to_hand() {
        let mut cards = Cards::new(vec![
            Card { id: 1, name: "Card1".to_string(), card_type: CardType::Product { price: 10, heat: 0 } },
            Card { id: 2, name: "Card2".to_string(), card_type: CardType::Product { price: 20, heat: 0 } },
            Card { id: 3, name: "Card3".to_string(), card_type: CardType::Product { price: 30, heat: 0 } },
            Card { id: 4, name: "Card4".to_string(), card_type: CardType::Product { price: 40, heat: 0 } },
        ]);

        // Initially hand is empty
        assert!(cards.hand.iter().all(|s| s.is_none()));
        assert_eq!(cards.deck.len(), 4);

        // Draw once - fills all 3 slots
        cards.draw_to_hand();

        assert_eq!(cards.hand.iter().filter(|s| s.is_some()).count(), 3);
        assert_eq!(cards.deck.len(), 1); // 1 card remains in deck

        // Verify correct cards were drawn
        assert_eq!(cards.hand[0].as_ref().unwrap().name, "Card1");
        assert_eq!(cards.hand[1].as_ref().unwrap().name, "Card2");
        assert_eq!(cards.hand[2].as_ref().unwrap().name, "Card3");
    }

    #[test]
    fn test_shuffle_back() {
        let mut cards = Cards::new(vec![
            Card { id: 1, name: "Card1".to_string(), card_type: CardType::Product { price: 10, heat: 0 } },
            Card { id: 2, name: "Card2".to_string(), card_type: CardType::Product { price: 20, heat: 0 } },
            Card { id: 3, name: "Card3".to_string(), card_type: CardType::Product { price: 30, heat: 0 } },
        ]);

        let initial_deck_size = cards.deck.len();

        // Draw cards
        cards.draw_to_hand();
        assert_eq!(cards.deck.len(), 0); // All drawn
        assert_eq!(cards.hand.iter().filter(|s| s.is_some()).count(), 3);

        // Remove one card from hand (simulate playing it)
        cards.hand[0] = None;

        // Shuffle back - only 2 cards should return to deck
        cards.shuffle_back();

        assert_eq!(cards.deck.len(), 2); // 2 unplayed cards returned
        assert!(cards.hand.iter().all(|s| s.is_none())); // Hand is empty
        
        // Verify deck was shuffled (we can't test randomness, but verify size is correct)
        assert_eq!(cards.deck.len(), initial_deck_size - 1); // Lost 1 played card
    }

    #[test]
    fn test_hand_vec_conversion() {
        let mut cards = Cards::empty();
        
        // Empty hand converts to empty vec
        let hand_vec: Vec<Card> = (&cards).into();
        assert_eq!(hand_vec.len(), 0);

        // Add some cards to hand
        cards.hand[0] = Some(Card { id: 1, name: "Card1".to_string(), card_type: CardType::Product { price: 10, heat: 0 } });
        cards.hand[2] = Some(Card { id: 3, name: "Card3".to_string(), card_type: CardType::Product { price: 30, heat: 0 } });

        // Conversion filters out None slots
        let hand_vec: Vec<Card> = (&cards).into();
        assert_eq!(hand_vec.len(), 2);
        assert_eq!(hand_vec[0].name, "Card1");
        assert_eq!(hand_vec[1].name, "Card3");
    }

    #[test]
    fn test_shuffle_deck() {
        // Create deck with 100 cards in order
        let mut deck = Vec::new();
        for i in 0..100 {
            deck.push(Card {
                id: i,
                name: format!("Card{}", i),
                card_type: CardType::Product { price: i, heat: 0 }
            });
        }

        let mut cards = Cards::new(deck.clone());
        let original_order: Vec<u32> = cards.deck.iter().map(|c| c.id).collect();

        cards.shuffle_deck();

        let shuffled_order: Vec<u32> = cards.deck.iter().map(|c| c.id).collect();

        // Deck size should be unchanged
        assert_eq!(cards.deck.len(), 100);
        // Order should have changed (extremely unlikely to be the same for 100 cards)
        assert_ne!(original_order, shuffled_order);
    }

    #[test]
    fn test_collect_unplayed() {
        let mut cards = Cards::new(vec![
            Card { id: 1, name: "Card1".to_string(), card_type: CardType::Product { price: 10, heat: 0 } },
        ]);

        // Put cards in hand
        cards.hand[0] = Some(Card { id: 2, name: "Card2".to_string(), card_type: CardType::Product { price: 20, heat: 0 } });
        cards.hand[1] = Some(Card { id: 3, name: "Card3".to_string(), card_type: CardType::Product { price: 30, heat: 0 } });

        // Add to played (these should NOT be collected)
        cards.played.push(Card { id: 4, name: "Card4".to_string(), card_type: CardType::Product { price: 40, heat: 0 } });

        // Collect unplayed
        cards.collect_unplayed();

        // Deck should have only unplayed cards (1 original + 2 from hand)
        assert_eq!(cards.deck.len(), 3);
        // Hand should be empty
        assert!(cards.hand.iter().all(|s| s.is_none()));
        // Played should still be there (NOT collected)
        assert_eq!(cards.played.len(), 1);
    }

    #[test]
    fn test_collect_all() {
        let mut cards = Cards::new(vec![
            Card { id: 1, name: "Card1".to_string(), card_type: CardType::Product { price: 10, heat: 0 } },
        ]);

        // Put cards in hand
        cards.hand[0] = Some(Card { id: 2, name: "Card2".to_string(), card_type: CardType::Product { price: 20, heat: 0 } });

        // Add to played
        cards.played.push(Card { id: 3, name: "Card3".to_string(), card_type: CardType::Product { price: 30, heat: 0 } });
        cards.played.push(Card { id: 4, name: "Card4".to_string(), card_type: CardType::Product { price: 40, heat: 0 } });

        // Collect all
        cards.collect_all();

        // Deck should have all cards (1 original + 1 from hand + 2 from played)
        assert_eq!(cards.deck.len(), 4);
        // Hand should be empty
        assert!(cards.hand.iter().all(|s| s.is_none()));
        // Played should be empty
        assert_eq!(cards.played.len(), 0);
    }
}
