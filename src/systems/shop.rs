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

    // SOW-025: street cred gates deeper stock - the roster's best rep in
    // THIS area opens the door, and the shop names who ("unlocked by Ray")
    let area_best_cred = save_data.as_ref().and_then(|data| {
        data.best_cred(&shop_state.selected_location)
            .map(|(idx, cred)| (data.dealers[idx].name.clone(), cred))
    });

    // Spawn shop card displays
    commands.entity(container).with_children(|parent| {
        for card in location_cards {
            let is_unlocked = unlocked_cards.contains(&card.id);
            let price = card.shop_price.unwrap_or(0);
            let cred_gate = card.shop_cred_required.map(|required| CredGate {
                required,
                best: area_best_cred.as_ref().map(|(_, c)| *c).unwrap_or(0),
                unlocked_by: area_best_cred.as_ref().map(|(n, _)| n.clone()),
            });

            spawn_shop_card(parent, card, price, is_unlocked, cred_gate);
        }
    });
}

/// SOW-025: find a shop card by id across all shop-stocked collections
/// (mirrors populate_shop_cards_system's gather)
fn find_shop_card<'a>(assets: &'a GameAssets, card_id: &str) -> Option<&'a Card> {
    assets
        .products
        .values()
        .chain(assets.locations.values())
        .chain(assets.cover.iter())
        .chain(assets.insurance.iter())
        .chain(assets.modifiers.iter())
        .find(|c| c.id == card_id)
}

/// SOW-025: how the roster's reputation measures up to an item's requirement
struct CredGate {
    required: u32,
    best: u32,
    unlocked_by: Option<String>,
}

impl CredGate {
    fn met(&self) -> bool {
        self.best >= self.required
    }
}

/// Spawn a shop card display
fn spawn_shop_card(
    parent: &mut ChildSpawnerCommands,
    card: &Card,
    price: u32,
    is_unlocked: bool,
    cred_gate: Option<CredGate>,
) {
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
        } else if cred_gate.as_ref().is_some_and(|gate| !gate.met()) {
            // SOW-025: cred-locked - "to unlock it, you gotta deal here"
            let gate = cred_gate.as_ref().unwrap();
            card_parent.spawn((
                Text::new(format!("NEEDS CRED {}\n(best: {})", gate.required, gate.best)),
                TextFont::from_font_size(12.0),
                TextColor(theme::SHOP_CRED_LOCK_TEXT),
                TextLayout::new_with_justify(bevy::text::Justify::Center),
            ));
        } else {
            // SOW-025: a met cred requirement names the dealer whose rep
            // opened the door (Reed: make the unlocking dealer visible)
            if let Some(gate) = cred_gate.as_ref() {
                if let Some(name) = &gate.unlocked_by {
                    card_parent.spawn((
                        Text::new(format!("unlocked by {name}")),
                        TextFont::from_font_size(10.0),
                        TextColor(theme::SHOP_CREDIT_LINE_TEXT),
                    ));
                }
            }

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
    game_assets: Res<GameAssets>,
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

        // SOW-025: server-side cred guard (the button doesn't spawn when
        // cred-locked, but state can shift between spawn and click)
        if let Some(card) = find_shop_card(&game_assets, &button.card_id) {
            if let (Some(required), Some(area)) = (card.shop_cred_required, card.shop_location.as_deref()) {
                let best = data.best_cred(area).map(|(_, c)| c).unwrap_or(0);
                if best < required {
                    info!(
                        "Cannot buy {}: needs {} cred in {} (roster best: {})",
                        button.card_id, required, area, best
                    );
                    continue;
                }
            }
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

/// SOW-024: Buying a locked area - deduct global cash, unlock, rebuild the
/// selector row, select the new turf, and announce it
pub fn area_unlock_button_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ShopAreaUnlockButton), Changed<Interaction>>,
    mut save_data: Option<ResMut<SaveData>>,
    save_manager: Option<Res<SaveManager>>,
    mut shop_state: ResMut<ShopState>,
    selector_query: Query<Entity, With<ShopLocationSelector>>,
    children_query: Query<&Children>,
    mut feedback_query: Query<&mut Text, With<ShopFeedbackText>>,
    game_assets: Res<GameAssets>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Some(ref mut data) = save_data else {
            continue;
        };

        match data.account.purchase_location(&button.location_id, button.price as u64) {
            Ok(()) => {
                let area_name = game_assets
                    .shop_locations
                    .iter()
                    .find(|a| a.id == button.location_id)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| button.location_id.clone());
                info!("Unlocked area {} for ${}", area_name, button.price);

                if let Some(ref manager) = save_manager {
                    if let Err(e) = manager.save(data) {
                        warn!("Failed to save after area purchase: {:?}", e);
                    }
                }

                // Rebuild the selector row to reflect the new unlock
                if let Ok(selector) = selector_query.single() {
                    if let Ok(children) = children_query.get(selector) {
                        for child in children.iter() {
                            commands.entity(child).despawn();
                        }
                    }
                    let unlocked = data.account.unlocked_locations.clone();
                    let areas = game_assets.shop_locations.clone();
                    commands.entity(selector).with_children(move |parent| {
                        crate::ui::setup::spawn_area_selector_buttons(parent, &areas, &unlocked);
                    });
                }

                // Select the new turf and announce it
                shop_state.selected_location = button.location_id.clone();
                if let Ok(mut feedback) = feedback_query.single_mut() {
                    **feedback = format!("New turf: {area_name}");
                }
            }
            Err(reason) => {
                info!("Cannot unlock {}: {}", button.location_id, reason);
            }
        }
    }
}

/// SOW-024: Locked-area purchase buttons read as buyable only when affordable
pub fn update_area_unlock_button_visuals(
    save_data: Option<Res<SaveData>>,
    mut button_query: Query<(&ShopAreaUnlockButton, &mut BackgroundColor)>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    if !save_data.is_changed() {
        return;
    }

    for (button, mut bg) in button_query.iter_mut() {
        *bg = if save_data.account.cash_on_hand >= button.price as u64 {
            BackgroundColor(theme::CONTINUE_BUTTON_BG)
        } else {
            BackgroundColor(theme::BUTTON_DISABLED_BG)
        };
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
