// SOW-020: Shop system for purchasing cards

use bevy::prelude::*;
use crate::ui::components::*;
use crate::ui::theme;
use crate::models::card::{Card, CardType};
use crate::save::{SaveData, SaveManager, AccountState};
use crate::assets::GameAssets;

/// Hover color for buttons
const BUTTON_HOVER_BG: Color = Color::srgb(0.4, 0.9, 0.4);

/// Resource tracking which shop view is active
#[derive(Resource, Default)]
pub struct ShopState {
    /// Are we viewing the shop (true) or the deck builder cards (false)?
    pub viewing_shop: bool,
    /// Which location is selected in the shop
    pub selected_location: String,
}

impl ShopState {
    pub fn new() -> Self {
        Self {
            viewing_shop: false,
            selected_location: "the_corner".to_string(),
        }
    }
}

/// System to handle tab switching between deck builder and shop
pub fn shop_tab_system(
    mut interaction_query: Query<(&Interaction, &ShopTab, &mut BackgroundColor), Changed<Interaction>>,
    mut shop_state: ResMut<ShopState>,
    mut card_pool_query: Query<&mut Node, (With<CardPoolContainer>, Without<ShopCardsContainer>, Without<ShopLocationSelector>)>,
    mut shop_container_query: Query<&mut Node, (With<ShopCardsContainer>, Without<CardPoolContainer>, Without<ShopLocationSelector>)>,
    mut location_selector_query: Query<&mut Node, (With<ShopLocationSelector>, Without<CardPoolContainer>, Without<ShopCardsContainer>)>,
) {
    for (interaction, tab, mut bg) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            shop_state.viewing_shop = tab.is_shop;

            // Toggle visibility of containers
            if let Ok(mut card_pool_node) = card_pool_query.single_mut() {
                card_pool_node.display = if shop_state.viewing_shop {
                    Display::None
                } else {
                    Display::Flex
                };
            }
            if let Ok(mut shop_node) = shop_container_query.single_mut() {
                shop_node.display = if shop_state.viewing_shop {
                    Display::Flex
                } else {
                    Display::None
                };
            }
            // Toggle location selector visibility
            if let Ok(mut loc_node) = location_selector_query.single_mut() {
                loc_node.display = if shop_state.viewing_shop {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }

        // Visual feedback
        *bg = if *interaction == Interaction::Hovered {
            BackgroundColor(BUTTON_HOVER_BG)
        } else if (tab.is_shop && shop_state.viewing_shop) || (!tab.is_shop && !shop_state.viewing_shop) {
            BackgroundColor(theme::CONTINUE_BUTTON_BG) // Active tab
        } else {
            BackgroundColor(theme::BUTTON_NEUTRAL_BG) // Inactive tab
        };
    }
}

/// System to update tab backgrounds based on active state
pub fn update_shop_tab_visuals(
    shop_state: Res<ShopState>,
    mut tab_query: Query<(&ShopTab, &mut BackgroundColor), Without<Interaction>>,
) {
    if !shop_state.is_changed() {
        return;
    }

    for (tab, mut bg) in tab_query.iter_mut() {
        *bg = if (tab.is_shop && shop_state.viewing_shop) || (!tab.is_shop && !shop_state.viewing_shop) {
            BackgroundColor(theme::CONTINUE_BUTTON_BG) // Active tab
        } else {
            BackgroundColor(theme::BUTTON_NEUTRAL_BG) // Inactive tab
        };
    }
}

/// System to handle shop location selection
pub fn shop_location_button_system(
    mut interaction_query: Query<(&Interaction, &ShopLocationButton), Changed<Interaction>>,
    mut shop_state: ResMut<ShopState>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            shop_state.selected_location = button.location_id.clone();
            info!("Selected shop location: {}", button.location_id);
        }
    }
}

/// System to populate shop cards based on selected location
pub fn populate_shop_cards_system(
    mut commands: Commands,
    shop_state: Res<ShopState>,
    shop_container_query: Query<Entity, With<ShopCardsContainer>>,
    existing_cards_query: Query<Entity, With<ShopCardDisplay>>,
    game_assets: Res<GameAssets>,
    save_data: Option<Res<SaveData>>,
) {
    // Only update if shop state changed
    if !shop_state.is_changed() {
        return;
    }

    // Only populate if viewing shop
    if !shop_state.viewing_shop {
        return;
    }

    // Clear existing shop cards
    for entity in existing_cards_query.iter() {
        commands.entity(entity).despawn();
    }

    let Ok(container) = shop_container_query.single() else {
        return;
    };

    // Get unlocked cards from save data
    let unlocked_cards = save_data
        .as_ref()
        .map(|data| &data.account.unlocked_cards)
        .cloned()
        .unwrap_or_else(|| AccountState::starting_collection());

    // Collect all cards for the selected location
    let mut location_cards: Vec<&Card> = Vec::new();

    // Products
    for card in game_assets.products.values() {
        if card.shop_location.as_deref() == Some(&shop_state.selected_location) {
            location_cards.push(card);
        }
    }
    // Locations (card type)
    for card in game_assets.locations.values() {
        if card.shop_location.as_deref() == Some(&shop_state.selected_location) {
            location_cards.push(card);
        }
    }
    // Cover
    for card in &game_assets.cover {
        if card.shop_location.as_deref() == Some(&shop_state.selected_location) {
            location_cards.push(card);
        }
    }
    // Insurance
    for card in &game_assets.insurance {
        if card.shop_location.as_deref() == Some(&shop_state.selected_location) {
            location_cards.push(card);
        }
    }
    // Modifiers
    for card in &game_assets.modifiers {
        if card.shop_location.as_deref() == Some(&shop_state.selected_location) {
            location_cards.push(card);
        }
    }

    // Sort by type then name
    location_cards.sort_by(|a, b| {
        let type_order = |c: &Card| match c.card_type {
            CardType::Product { .. } => 0,
            CardType::Location { .. } => 1,
            CardType::Cover { .. } => 2,
            CardType::DealModifier { .. } => 3,
            CardType::Insurance { .. } => 4,
            _ => 5,
        };
        type_order(a).cmp(&type_order(b)).then_with(|| a.name.cmp(&b.name))
    });

    // Spawn shop card displays
    commands.entity(container).with_children(|parent| {
        for card in location_cards {
            let is_unlocked = unlocked_cards.contains(&card.id);
            let price = card.shop_price.unwrap_or(0);

            spawn_shop_card(parent, card, price, is_unlocked);
        }
    });
}

/// Spawn a shop card display
fn spawn_shop_card(parent: &mut ChildSpawnerCommands, card: &Card, price: u32, is_unlocked: bool) {
    let card_color = match card.card_type {
        CardType::Product { .. } => theme::PRODUCT_CARD_COLOR,
        CardType::Location { .. } => theme::LOCATION_CARD_COLOR,
        CardType::Cover { .. } => theme::COVER_CARD_COLOR,
        CardType::DealModifier { .. } => theme::DEAL_MODIFIER_CARD_COLOR,
        CardType::Insurance { .. } => theme::INSURANCE_CARD_COLOR,
        _ => Color::WHITE,
    };

    let bg_color = if is_unlocked {
        Color::srgba(0.2, 0.4, 0.2, 0.8) // Green tint for owned
    } else {
        Color::srgba(0.2, 0.2, 0.2, 0.8) // Dark for locked
    };

    parent.spawn((
        Node {
            width: Val::Px(150.0),
            height: Val::Px(200.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            margin: UiRect::all(Val::Px(5.0)),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(card_color),
        ShopCardDisplay {
            card_id: card.id.clone(),
            price,
            is_unlocked,
        },
    ))
    .with_children(|card_parent| {
        // Card name
        card_parent.spawn((
            Text::new(&card.name),
            TextFont::from_font_size(14.0),
            TextColor(card_color),
        ));

        // Card stats (simplified)
        let stats_text = match &card.card_type {
            CardType::Product { price, heat } => format!("${} • {}🔥", price, heat),
            CardType::Location { evidence, cover, heat } => format!("E:{} C:{} H:{}", evidence, cover, heat),
            CardType::Cover { cover, heat } => format!("C:{} H:{}", cover, heat),
            CardType::DealModifier { price_multiplier, .. } => format!("×{:.1}", price_multiplier),
            CardType::Insurance { cover, heat_penalty, .. } => format!("C:{} HP:{}", cover, heat_penalty),
            _ => String::new(),
        };
        card_parent.spawn((
            Text::new(stats_text),
            TextFont::from_font_size(12.0),
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        // Price/status row
        if is_unlocked {
            card_parent.spawn((
                Text::new("✓ OWNED"),
                TextFont::from_font_size(14.0),
                TextColor(Color::srgb(0.3, 0.8, 0.3)),
            ));
        } else if price == 0 {
            // Shouldn't happen - starting cards are unlocked
            card_parent.spawn((
                Text::new("FREE"),
                TextFont::from_font_size(14.0),
                TextColor(Color::srgb(0.8, 0.8, 0.3)),
            ));
        } else {
            // Purchase button
            card_parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme::CONTINUE_BUTTON_BG),
                ShopPurchaseButton {
                    card_id: card.id.clone(),
                    price,
                },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(format!("${}", price)),
                    TextFont::from_font_size(14.0),
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}

/// System to handle card purchases
pub fn shop_purchase_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ShopPurchaseButton), Changed<Interaction>>,
    mut save_data: Option<ResMut<SaveData>>,
    save_manager: Option<Res<SaveManager>>,
    mut shop_state: ResMut<ShopState>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Some(ref mut data) = save_data else {
            warn!("Cannot purchase: no save data");
            continue;
        };

        // Check if player can afford it
        if data.account.cash_on_hand < button.price as u64 {
            info!("Cannot afford card {} (need ${}, have ${})",
                  button.card_id, button.price, data.account.cash_on_hand);
            continue;
        }

        // Deduct cash and unlock card
        data.account.cash_on_hand -= button.price as u64;
        data.account.unlocked_cards.insert(button.card_id.clone());

        info!("Purchased card {} for ${} (remaining: ${})",
              button.card_id, button.price, data.account.cash_on_hand);

        // Save immediately
        if let Some(ref manager) = save_manager {
            if let Err(e) = manager.save(&data) {
                warn!("Failed to save after purchase: {:?}", e);
            }
        }

        // Force shop UI refresh by toggling the location (hacky but works)
        let current = shop_state.selected_location.clone();
        shop_state.selected_location = String::new();
        commands.insert_resource(ShopState {
            viewing_shop: true,
            selected_location: current,
        });
    }
}

/// System to update location button visuals
pub fn update_location_button_visuals(
    shop_state: Res<ShopState>,
    mut button_query: Query<(&ShopLocationButton, &mut BackgroundColor)>,
) {
    if !shop_state.is_changed() {
        return;
    }

    for (button, mut bg) in button_query.iter_mut() {
        *bg = if button.location_id == shop_state.selected_location {
            BackgroundColor(theme::CONTINUE_BUTTON_BG) // Active
        } else {
            BackgroundColor(theme::BUTTON_NEUTRAL_BG) // Inactive
        };
    }
}
