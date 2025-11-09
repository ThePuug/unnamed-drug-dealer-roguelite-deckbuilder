// SOW-001: Minimal Playable Hand
// Phase 1: Card Data Model & State Machine

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            auto_play_system,
            recreate_hand_display_system,
            ui_update_system,
            card_click_system,
        ).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Initialize hand state
    let mut hand_state = HandState::default();
    hand_state.draw_cards();
    commands.spawn(hand_state);

    // Create UI root
    create_ui(&mut commands);
}

// ============================================================================
// UI COMPONENTS - Phase 4
// ============================================================================

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct TotalsDisplay;

#[derive(Component)]
struct StatusDisplay;

#[derive(Component)]
struct PlayAreaNarc;

#[derive(Component)]
struct PlayAreaCustomer;

#[derive(Component)]
struct PlayAreaPlayer;

#[derive(Component)]
struct PlayerHandDisplay;

#[derive(Component)]
struct CardButton {
    card_index: usize,
}

fn create_ui(commands: &mut Commands) {
    // UI Root container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
        UiRoot,
    ))
    .with_children(|parent| {
        // Status display at top
        parent.spawn((
            TextBundle::from_section(
                "Status: Drawing Cards...",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            StatusDisplay,
        ));

        // Totals display
        parent.spawn((
            TextBundle::from_section(
                "Evidence: 0 | Cover: 0 | Heat: 0 | Profit: $0",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
            TotalsDisplay,
        ));

        // Play areas (3 zones)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Narc zone
            create_play_area(parent, "Narc's Cards", Color::srgb(0.8, 0.3, 0.3), PlayAreaNarc);

            // Customer zone
            create_play_area(parent, "Customer's Cards", Color::srgb(0.3, 0.6, 0.8), PlayAreaCustomer);

            // Player zone
            create_play_area(parent, "Your Cards", Color::srgb(0.3, 0.8, 0.3), PlayAreaPlayer);
        });

        // Player hand display
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                ..default()
            },
            PlayerHandDisplay,
        ));
    });
}

fn create_play_area(parent: &mut ChildBuilder, label: &str, color: Color, marker: impl Component) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: color.with_alpha(0.2).into(),
            border_color: color.into(),
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

// ============================================================================
// UI UPDATE SYSTEM
// ============================================================================

fn ui_update_system(
    hand_state_query: Query<&HandState>,
    mut totals_query: Query<&mut Text, (With<TotalsDisplay>, Without<StatusDisplay>)>,
    mut status_query: Query<&mut Text, (With<StatusDisplay>, Without<TotalsDisplay>)>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Update totals display
    if let Ok(mut text) = totals_query.get_single_mut() {
        let totals = hand_state.calculate_totals();
        text.sections[0].value = format!(
            "Evidence: {} | Cover: {} | Heat: {} | Profit: ${}",
            totals.evidence, totals.cover, totals.heat, totals.profit
        );
    }

    // Update status display
    if let Ok(mut text) = status_query.get_single_mut() {
        let status = match hand_state.current_state {
            State::Draw => "Status: Drawing Cards...".to_string(),
            State::NarcPlay => "Status: Narc's Turn".to_string(),
            State::CustomerPlay => "Status: Customer's Turn".to_string(),
            State::PlayerPlay => "Status: YOUR TURN - Click a card to play".to_string(),
            State::Resolve => "Status: Resolving...".to_string(),
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => "Status: SAFE! You got away with it!".to_string(),
                Some(HandOutcome::Busted) => "Status: BUSTED! You got caught!".to_string(),
                None => "Status: Game Over".to_string(),
            },
        };

        text.sections[0].value = status;

        // Color code status
        text.sections[0].style.color = match hand_state.current_state {
            State::PlayerPlay => Color::srgb(0.3, 1.0, 0.3),
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => Color::srgb(0.3, 1.0, 0.3),
                Some(HandOutcome::Busted) => Color::srgb(1.0, 0.3, 0.3),
                None => Color::WHITE,
            },
            _ => Color::WHITE,
        };
    }
}

// New system: only recreate hand display when hand state changes
fn recreate_hand_display_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    hand_display_query: Query<Entity, With<PlayerHandDisplay>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_button_query: Query<Entity, With<CardButton>>,
) {
    // Only run when HandState actually changed
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    let Ok(hand_entity) = hand_display_query.get_single() else {
        return;
    };

    // Clear existing card buttons
    if let Ok(children) = children_query.get(hand_entity) {
        for &child in children.iter() {
            if card_button_query.get(child).is_ok() {
                commands.entity(child).despawn_recursive();
            }
        }
    }

    // Add card buttons for current hand
    if hand_state.current_state == State::PlayerPlay {
        commands.entity(hand_entity).with_children(|parent| {
            for (index, card) in hand_state.player_hand.iter().enumerate() {
                let card_color = match card.card_type {
                    CardType::Product { .. } => Color::srgb(0.9, 0.7, 0.2),
                    CardType::Location { .. } => Color::srgb(0.3, 0.6, 0.9),
                    CardType::Evidence { .. } => Color::srgb(0.8, 0.3, 0.3),
                    CardType::Cover { .. } => Color::srgb(0.3, 0.8, 0.3),
                };

                let card_info = match &card.card_type {
                    CardType::Product { price, heat } =>
                        format!("{}\n${} | Heat: {}", card.name, price, heat),
                    CardType::Location { evidence, cover, heat } =>
                        format!("{}\nE:{} C:{} H:{}", card.name, evidence, cover, heat),
                    CardType::Evidence { evidence, heat } =>
                        format!("{}\nEvidence: {} | Heat: {}", card.name, evidence, heat),
                    CardType::Cover { cover, heat } =>
                        format!("{}\nCover: {} | Heat: {}", card.name, cover, heat),
                };

                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(160.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: card_color.into(),
                        ..default()
                    },
                    CardButton { card_index: index },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        card_info,
                        TextStyle {
                            font_size: 14.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ).with_text_justify(JustifyText::Center));
                });
            }
        });
    }
}

// ============================================================================
// AUTO-PLAY SYSTEM (Narc and Customer play automatically)
// ============================================================================

fn auto_play_system(
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Auto-play for Narc (play first available card)
    if hand_state.current_state == State::NarcPlay && !hand_state.narc_hand.is_empty() {
        let _ = hand_state.play_card(Owner::Narc, 0);
    }

    // Auto-play for Customer (skip if no cards - Customer has no cards in 8-card MVP)
    if hand_state.current_state == State::CustomerPlay {
        if hand_state.customer_hand.is_empty() {
            // Skip customer turn
            hand_state.transition_state();
        } else {
            let _ = hand_state.play_card(Owner::Customer, 0);
        }
    }
}

// ============================================================================
// CARD CLICK SYSTEM
// ============================================================================

fn card_click_system(
    mut interaction_query: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    for (interaction, card_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if hand_state.current_state == State::PlayerPlay {
                // Play the card
                let _ = hand_state.play_card(Owner::Player, card_button.card_index);

                // If we're now in Resolve state, resolve the hand
                if hand_state.current_state == State::Resolve {
                    hand_state.resolve_hand();
                }
            }
        }
    }
}

// ============================================================================
// CARD DATA MODEL
// ============================================================================

/// Who owns this card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    Narc,
    Customer,
    Player,
}

/// Card types with their specific values
#[derive(Debug, Clone)]
enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
}

/// Card instance
#[derive(Component, Debug, Clone)]
struct Card {
    id: u32,
    name: String,
    owner: Owner,
    card_type: CardType,
}

// ============================================================================
// HAND STATE MACHINE
// ============================================================================

/// States the hand can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Draw,
    NarcPlay,
    CustomerPlay,
    PlayerPlay,
    Resolve,
    Bust,
}

/// Outcome of hand resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandOutcome {
    Safe,
    Busted,
}

/// Hand state tracking
#[derive(Component)]
struct HandState {
    pub current_state: State,
    pub cards_played: Vec<Card>,
    narc_deck: Vec<Card>,
    customer_deck: Vec<Card>,
    player_deck: Vec<Card>,
    pub narc_hand: Vec<Card>,
    pub customer_hand: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub outcome: Option<HandOutcome>,
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            current_state: State::Draw,
            cards_played: Vec::new(),
            narc_deck: create_narc_deck(),
            customer_deck: create_customer_deck(),
            player_deck: create_player_deck(),
            narc_hand: Vec::new(),
            customer_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
        }
    }
}

impl HandState {
    /// Reset hand state for replay testing
    fn reset(&mut self) {
        *self = Self::default();
    }

    /// Draw cards from decks to hands (initial draw phase)
    fn draw_cards(&mut self) {
        // For MVP: deal entire deck to hand (simple, no draw count)
        self.narc_hand = self.narc_deck.clone();
        self.customer_hand = self.customer_deck.clone();
        self.player_hand = self.player_deck.clone();

        // Transition to next state after draw
        self.transition_state();
    }

    /// Transition to next state
    pub fn transition_state(&mut self) {
        self.current_state = match self.current_state {
            State::Draw => State::NarcPlay,
            State::NarcPlay => State::CustomerPlay,
            State::CustomerPlay => State::PlayerPlay,
            State::PlayerPlay => State::Resolve,
            State::Resolve => State::Bust, // Will be refined in Phase 3 (Safe vs Busted)
            State::Bust => State::Bust, // Terminal state
        };
    }

    /// Play a card from hand to the play area
    fn play_card(&mut self, owner: Owner, card_index: usize) -> Result<(), String> {
        // Verify it's the correct player's turn
        match (self.current_state, owner) {
            (State::NarcPlay, Owner::Narc) => {}
            (State::CustomerPlay, Owner::Customer) => {}
            (State::PlayerPlay, Owner::Player) => {}
            _ => return Err(format!("Wrong turn: state {:?}, owner {:?}", self.current_state, owner)),
        }

        // Get the card from the appropriate hand
        let hand = match owner {
            Owner::Narc => &mut self.narc_hand,
            Owner::Customer => &mut self.customer_hand,
            Owner::Player => &mut self.player_hand,
        };

        if card_index >= hand.len() {
            return Err(format!("Card index {} out of bounds", card_index));
        }

        let card = hand.remove(card_index);
        self.cards_played.push(card);

        // Transition to next state after playing
        self.transition_state();

        Ok(())
    }
}

// ============================================================================
// BUST CHECK & RESOLUTION - Phase 3
// ============================================================================

impl HandState {
    /// Resolve the hand by checking if player gets busted
    ///
    /// Bust check runs at Resolve state (after all cards played)
    /// - Evidence > Cover → Busted (run ends)
    /// - Evidence ≤ Cover → Safe (continue possible, but single round so ends)
    /// - Tie goes to player (Evidence = Cover is Safe)
    ///
    /// Extensible: RFC-003 will add insurance check before bust finalization
    fn resolve_hand(&mut self) -> HandOutcome {
        let totals = self.calculate_totals();

        let outcome = if totals.evidence > totals.cover {
            HandOutcome::Busted
        } else {
            // Evidence ≤ Cover → Safe (tie goes to player)
            HandOutcome::Safe
        };

        self.outcome = Some(outcome);
        self.current_state = State::Bust; // Transition to terminal state
        outcome
    }
}

// ============================================================================
// CARD INTERACTION ENGINE - Phase 2
// ============================================================================

/// Totals calculated from all played cards
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Totals {
    evidence: u32,
    cover: u32,
    heat: i32,
    profit: u32,
}

impl HandState {
    /// Get active Product card (last Product played, if any)
    /// Override rule: Only last Product matters
    fn active_product(&self) -> Option<&Card> {
        self.cards_played
            .iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Product { .. }))
    }

    /// Get active Location card (last Location played, required)
    /// Override rule: Only last Location matters
    fn active_location(&self) -> Option<&Card> {
        self.cards_played
            .iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Location { .. }))
    }

    /// Calculate current totals from all played cards
    ///
    /// Override rules:
    /// - Last Product played becomes active (previous discarded)
    /// - Last Location played becomes active (Evidence/Cover base changes)
    ///
    /// Additive rules:
    /// - Evidence = Location base + sum(all Evidence cards)
    /// - Cover = Location base + sum(all Cover cards)
    /// - Heat = sum(all heat modifiers from all cards)
    /// - Profit = Active Product price (or 0 if no Product)
    fn calculate_totals(&self) -> Totals {
        let mut totals = Totals::default();

        // Get base Evidence/Cover from active Location
        if let Some(location) = self.active_location() {
            if let CardType::Location { evidence, cover, heat } = location.card_type {
                totals.evidence = evidence;
                totals.cover = cover;
                totals.heat += heat;
            }
        }

        // Add Evidence from all Evidence cards
        for card in &self.cards_played {
            if let CardType::Evidence { evidence, heat } = card.card_type {
                totals.evidence += evidence;
                totals.heat += heat;
            }
        }

        // Add Cover from all Cover cards
        for card in &self.cards_played {
            if let CardType::Cover { cover, heat } = card.card_type {
                totals.cover += cover;
                totals.heat += heat;
            }
        }

        // Get profit from active Product
        if let Some(product) = self.active_product() {
            if let CardType::Product { price, heat } = product.card_type {
                totals.profit = price;
                totals.heat += heat;
            }
        }

        totals
    }
}

// ============================================================================
// 8-CARD COLLECTION (MVP)
// ============================================================================

fn create_narc_deck() -> Vec<Card> {
    vec![
        // 2 Evidence cards for Narc
        Card {
            id: 1,
            name: "Patrol".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        },
        Card {
            id: 2,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        },
    ]
}

fn create_customer_deck() -> Vec<Card> {
    // Customer doesn't have cards in 8-card MVP (just Narc opposition + Player choices)
    vec![]
}

fn create_player_deck() -> Vec<Card> {
    vec![
        // 3 Products
        Card {
            id: 10,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        },
        Card {
            id: 11,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        },
        Card {
            id: 12,
            name: "Heroin".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 150, heat: 45 },
        },
        // 2 Locations
        Card {
            id: 13,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        },
        Card {
            id: 14,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        },
        // 1 Cover card
        Card {
            id: 15,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        },
    ]
}

// ============================================================================
// TESTS - Phase 1
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_instantiation() {
        // Verify all 8 cards can be instantiated with correct values
        let narc_deck = create_narc_deck();
        assert_eq!(narc_deck.len(), 2);
        assert_eq!(narc_deck[0].name, "Patrol");
        assert_eq!(narc_deck[1].name, "Surveillance");

        let player_deck = create_player_deck();
        assert_eq!(player_deck.len(), 6);

        // Verify Product cards
        if let CardType::Product { price, heat } = player_deck[0].card_type {
            assert_eq!(price, 30);
            assert_eq!(heat, 5);
        } else {
            panic!("Expected Product card");
        }

        // Verify Location cards
        if let CardType::Location { evidence, cover, heat } = player_deck[3].card_type {
            assert_eq!(evidence, 10);
            assert_eq!(cover, 30);
            assert_eq!(heat, -5);
        } else {
            panic!("Expected Location card");
        }
    }

    #[test]
    fn test_state_transitions() {
        let mut hand_state = HandState::default();

        // Initial state should be Draw
        assert_eq!(hand_state.current_state, State::Draw);

        // Transition through all states
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::NarcPlay);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::CustomerPlay);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::PlayerPlay);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Resolve);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Bust);

        // Bust is terminal state
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Bust);
    }

    #[test]
    fn test_reset() {
        let mut hand_state = HandState::default();

        // Modify state
        hand_state.transition_state();
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::CustomerPlay);

        // Reset should return to initial state
        hand_state.reset();
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.cards_played.len(), 0);
    }

    #[test]
    fn test_draw_cards() {
        let mut hand_state = HandState::default();
        assert_eq!(hand_state.narc_hand.len(), 0);
        assert_eq!(hand_state.player_hand.len(), 0);

        hand_state.draw_cards();

        // Cards should be dealt to hands
        assert_eq!(hand_state.narc_hand.len(), 2);
        assert_eq!(hand_state.player_hand.len(), 6);

        // State should advance to NarcPlay
        assert_eq!(hand_state.current_state, State::NarcPlay);
    }

    #[test]
    fn test_play_card_wrong_turn() {
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // State is NarcPlay, player shouldn't be able to play
        let result = hand_state.play_card(Owner::Player, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_play_card_success() {
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // State is NarcPlay after draw
        assert_eq!(hand_state.current_state, State::NarcPlay);
        assert_eq!(hand_state.narc_hand.len(), 2);

        // Narc plays first card
        let result = hand_state.play_card(Owner::Narc, 0);
        assert!(result.is_ok());

        // Card should be moved from hand to played
        assert_eq!(hand_state.narc_hand.len(), 1);
        assert_eq!(hand_state.cards_played.len(), 1);

        // State should advance to CustomerPlay
        assert_eq!(hand_state.current_state, State::CustomerPlay);
    }

    // ========================================================================
    // TESTS - Phase 2 (Card Interaction Engine)
    // ========================================================================

    #[test]
    fn test_override_product() {
        let mut hand_state = HandState::default();

        // Play Weed, then Meth
        let weed = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        let meth = Card {
            id: 2,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        };

        hand_state.cards_played.push(weed);
        hand_state.cards_played.push(meth.clone());

        // Active product should be Meth (last played)
        let active = hand_state.active_product();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Meth");

        // Totals should reflect Meth price (100), not Weed price (30)
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.profit, 100);
    }

    #[test]
    fn test_override_location() {
        let mut hand_state = HandState::default();

        // Play School Zone, then Safe House
        let school_zone = Card {
            id: 1,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        let safe_house = Card {
            id: 2,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };

        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(safe_house.clone());

        // Active location should be Safe House (last played)
        let active = hand_state.active_location();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Safe House");

        // Totals should reflect Safe House base (Evidence 10, Cover 30)
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.evidence, 10);
        assert_eq!(totals.cover, 30);
    }

    #[test]
    fn test_additive_evidence() {
        let mut hand_state = HandState::default();

        // Play Location (base 10 Evidence)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Play Evidence cards
        let patrol = Card {
            id: 2,
            name: "Patrol".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        };
        let surveillance = Card {
            id: 3,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(patrol);
        hand_state.cards_played.push(surveillance);

        // Evidence should stack: 10 (location) + 5 (patrol) + 20 (surveillance) = 35
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.evidence, 35);
    }

    #[test]
    fn test_additive_cover() {
        let mut hand_state = HandState::default();

        // Play Location (base 30 Cover)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Play Cover card
        let alibi = Card {
            id: 2,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(alibi);

        // Cover should stack: 30 (location) + 30 (alibi) = 60
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.cover, 60);
    }

    #[test]
    fn test_heat_accumulation() {
        let mut hand_state = HandState::default();

        // Play cards with various heat modifiers
        let meth = Card {
            id: 1,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        };
        let school_zone = Card {
            id: 2,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        let surveillance = Card {
            id: 3,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };

        hand_state.cards_played.push(meth);
        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(surveillance);

        // Heat should accumulate: 30 + 20 + 5 = 55
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.heat, 55);
    }

    #[test]
    fn test_no_product_played() {
        let mut hand_state = HandState::default();

        // Play Location only
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Profit should be 0 (no Product played)
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.profit, 0);
    }

    #[test]
    fn test_complete_hand_scenario() {
        let mut hand_state = HandState::default();

        // Scenario: Player plays complete round
        // 1. Location: Safe House (Evidence 10, Cover 30, Heat -5)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // 2. Product: Meth (Price 100, Heat 30)
        let product = Card {
            id: 2,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        };
        hand_state.cards_played.push(product);

        // 3. Cover: Alibi (Cover 30, Heat -5)
        let cover = Card {
            id: 3,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(cover);

        // 4. Evidence: Surveillance (Evidence 20, Heat 5)
        let evidence = Card {
            id: 4,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(evidence);

        let totals = hand_state.calculate_totals();

        // Verify totals:
        // Evidence: 10 (location) + 20 (surveillance) = 30
        // Cover: 30 (location) + 30 (alibi) = 60
        // Heat: -5 (location) + 30 (meth) - 5 (alibi) + 5 (surveillance) = 25
        // Profit: 100 (meth)
        assert_eq!(totals.evidence, 30);
        assert_eq!(totals.cover, 60);
        assert_eq!(totals.heat, 25);
        assert_eq!(totals.profit, 100);
    }

    // ========================================================================
    // TESTS - Phase 3 (Bust Check & Resolution)
    // ========================================================================

    #[test]
    fn test_bust_evidence_greater_than_cover() {
        let mut hand_state = HandState::default();

        // Location with high Evidence, low Cover
        let location = Card {
            id: 1,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        hand_state.cards_played.push(location);

        // Add more Evidence
        let evidence = Card {
            id: 2,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(evidence);

        // Totals: Evidence 60, Cover 5 → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted));
        assert_eq!(hand_state.current_state, State::Bust);
    }

    #[test]
    fn test_safe_evidence_less_than_cover() {
        let mut hand_state = HandState::default();

        // Location with low Evidence, high Cover
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Add Cover
        let cover = Card {
            id: 2,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(cover);

        // Totals: Evidence 10, Cover 60 → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
        assert_eq!(hand_state.current_state, State::Bust);
    }

    #[test]
    fn test_tie_goes_to_player() {
        let mut hand_state = HandState::default();

        // Location with equal Evidence and Cover
        let location = Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Totals: Evidence 30, Cover 30 → Safe (tie goes to player)
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
    }

    #[test]
    fn test_edge_case_one_more_evidence() {
        let mut hand_state = HandState::default();

        // Location
        let location = Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Add 1 Evidence (31 > 30 → Busted)
        let evidence = Card {
            id: 2,
            name: "Evidence".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 1, heat: 0 },
        };
        hand_state.cards_played.push(evidence);

        // Totals: Evidence 31, Cover 30 → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
    }

    #[test]
    fn test_edge_case_one_more_cover() {
        let mut hand_state = HandState::default();

        // Location
        let location = Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Add 1 Cover (30 ≤ 31 → Safe)
        let cover = Card {
            id: 2,
            name: "Cover".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 1, heat: 0 },
        };
        hand_state.cards_played.push(cover);

        // Totals: Evidence 30, Cover 31 → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
    }
}
