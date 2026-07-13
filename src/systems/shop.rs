// SOW-020: Shop system for purchasing cards

use bevy::prelude::*;
use crate::ui::components::*;
use crate::ui::theme;
use crate::models::card::{Card, CardType};
use crate::models::shop_location::{batch_cost, restock_unit};
use crate::save::{SaveData, SaveManager, AccountState, BATCH_SIZE};
use crate::assets::GameAssets;

/// Hover color for buttons
const BUTTON_HOVER_BG: Color = Color::srgb(0.4, 0.9, 0.4);

/// SOW-034: defensive fallback margin if a selected zone's def is somehow
/// missing (validate_shop_locations guarantees a valid per-zone margin, so
/// this is never hit in practice - the real margins live in shop_locations.ron).
const DEFAULT_RESTOCK_MARGIN: f32 = 0.5;

/// SOW-034: a product card's consumable state in the shop - how many charges
/// are on hand, what one charge costs to restock at this zone's margin, and the
/// full batch cost. `None` at the spawn site marks a non-product (a permanent
/// one-time unlock).
struct ProductStock {
    charges: u32,
    unit_price: u32,
    batch_cost: u32,
}

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
            selected_location: "trailer_park".to_string(),
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

    // SOW-031: the zone's supplier relationship shapes the whole tab -
    // header up top, FRONT offers on the cards, stock locked while CutOff
    let area_def = game_assets
        .shop_locations
        .iter()
        .find(|a| a.id == shop_state.selected_location)
        .cloned();
    let front_ctx = save_data.as_ref().map(|data| FrontContext {
        standing: data.standing_with(&shop_state.selected_location),
        has_front: data.front_in(&shop_state.selected_location).is_some(),
        cash: data.account.cash_on_hand,
    });

    // Spawn shop card displays
    commands.entity(container).with_children(|parent| {
        // SOW-031: supplier header first (full-width; ShopCardDisplay so
        // the rebuild clears it with the cards)
        if let (Some(area), Some(data)) = (area_def.as_ref(), save_data.as_ref()) {
            if let Some(header) = crate::ui::front_view::supplier_header(area, data) {
                spawn_supplier_header(parent, &area.id, &header);
            }
        }

        // SOW-034: the zone's per-zone restock margin (authored in RON,
        // validated in (0.0, 1.0) at load) - forgiving in the starter zone,
        // tighter up the ladder. Defensive fallback if the def is missing.
        let margin = area_def
            .as_ref()
            .map(|a| a.restock_margin)
            .unwrap_or(DEFAULT_RESTOCK_MARGIN);

        for card in location_cards {
            let is_unlocked = unlocked_cards.contains(&card.id);
            let price = card.shop_price.unwrap_or(0);
            let cred_gate = card.shop_cred_required.map(|required| CredGate {
                required,
                best: area_best_cred.as_ref().map(|(_, c)| *c).unwrap_or(0),
                unlocked_by: area_best_cred.as_ref().map(|(n, _)| n.clone()),
            });

            // SOW-034: products are consumable stock, priced off their base
            // sale price x the zone margin; every other type stays a one-time
            // unlock (product_stock None).
            let product_stock = if let CardType::Product { price: base, .. } = card.card_type {
                Some(ProductStock {
                    charges: save_data
                        .as_ref()
                        .map(|d| d.account.charges_in(&card.id))
                        .unwrap_or(0),
                    unit_price: restock_unit(base, margin),
                    batch_cost: batch_cost(base, margin),
                })
            } else {
                None
            };

            spawn_shop_card(
                parent,
                card,
                price,
                is_unlocked,
                cred_gate,
                &shop_state.selected_location,
                front_ctx.as_ref(),
                product_stock,
            );
        }
    });
}

/// SOW-031: what the supplier relationship means for this shop tab
struct FrontContext {
    standing: crate::save::SupplierStanding,
    has_front: bool,
    cash: u64,
}

/// SOW-031: the supplier header - name, voice, status, PAY when owed
fn spawn_supplier_header(
    parent: &mut ChildSpawnerCommands,
    area_id: &str,
    header: &crate::ui::front_view::SupplierHeader,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                margin: UiRect::all(Val::Px(5.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.1, 0.06, 0.9)),
            BorderColor::all(theme::LEDGER_BOARD_CURRENT),
            ShopCardDisplay,
        ))
        .with_children(|row| {
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                ..default()
            })
            .with_children(|left| {
                left.spawn((
                    Text::new(header.name_line.as_str()),
                    TextFont::from_font_size(15.0),
                    TextColor(theme::LEDGER_BOARD_CURRENT),
                ));
                left.spawn((
                    Text::new(header.voice_line.as_str()),
                    TextFont::from_font_size(12.0),
                    TextColor(Color::srgb(0.75, 0.72, 0.65)),
                ));
                if let Some(status) = &header.status_line {
                    left.spawn((
                        Text::new(status.as_str()),
                        TextFont::from_font_size(13.0),
                        TextColor(if header.urgent {
                            theme::ROSTER_STATUS_JAILED
                        } else {
                            Color::WHITE
                        }),
                    ));
                }
            });
            if let Some(owed) = header.payable {
                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(theme::CONTINUE_BUTTON_BG),
                    FrontPayButton { area_id: area_id.to_string() },
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(format!("PAY ${owed}")),
                        TextFont::from_font_size(14.0),
                        TextColor(Color::WHITE),
                    ));
                });
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

/// SOW-034: a purchase button. `restock_unit` Some(unit) routes the click
/// through `buy_batch` (a consumable product batch); None is a permanent
/// one-time unlock (Location/Cover/Insurance/Modifier).
fn spawn_purchase_button(
    parent: &mut ChildSpawnerCommands,
    card_id: &str,
    label: String,
    price: u32,
    restock_unit: Option<u32>,
) {
    parent
        .spawn((
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
                card_id: card_id.to_string(),
                price,
                restock_unit,
            },
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont::from_font_size(14.0),
                TextColor(Color::WHITE),
            ));
        });
}

/// SOW-034: FRONT-a-batch button on a product you have access to but can't
/// afford to restock in cash. Full cost + window on the face before commit.
fn spawn_front_button(
    parent: &mut ChildSpawnerCommands,
    card_id: &str,
    area_id: &str,
    batch_cost: u32,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.55, 0.42, 0.12)),
            FrontTakeButton {
                card_id: card_id.to_string(),
                area_id: area_id.to_string(),
                batch_cost,
            },
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(crate::ui::front_view::front_button_label(batch_cost)),
                TextFont::from_font_size(11.0),
                TextColor(Color::WHITE),
            ));
        });
}

/// A cred gate that names the dealer whose rep opened the door (Reed: make
/// the unlocking dealer visible). No-op when the gate is absent or unnamed.
fn spawn_unlocked_by_line(parent: &mut ChildSpawnerCommands, cred_gate: Option<&CredGate>) {
    if let Some(gate) = cred_gate {
        if let Some(name) = &gate.unlocked_by {
            parent.spawn((
                Text::new(format!("unlocked by {name}")),
                TextFont::from_font_size(10.0),
                TextColor(theme::SHOP_CREDIT_LINE_TEXT),
            ));
        }
    }
}

/// The shop's "NEEDS CRED n" lock line
fn spawn_cred_lock_line(parent: &mut ChildSpawnerCommands, gate: &CredGate) {
    parent.spawn((
        Text::new(format!("NEEDS CRED {}\n(best: {})", gate.required, gate.best)),
        TextFont::from_font_size(12.0),
        TextColor(theme::SHOP_CRED_LOCK_TEXT),
        TextLayout::new_with_justify(bevy::text::Justify::Center),
    ));
}

/// The shop's "CUT OFF — settle your debt" stock lock line
fn spawn_cut_off_line(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("CUT OFF\nsettle your debt"),
        TextFont::from_font_size(12.0),
        TextColor(theme::ROSTER_STATUS_JAILED),
        TextLayout::new_with_justify(bevy::text::Justify::Center),
    ));
}

/// Spawn a shop card display
fn spawn_shop_card(
    parent: &mut ChildSpawnerCommands,
    card: &Card,
    price: u32,
    is_unlocked: bool,
    cred_gate: Option<CredGate>,
    area_id: &str,
    front_ctx: Option<&FrontContext>,
    product_stock: Option<ProductStock>,
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
        ShopCardDisplay,
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
        let cut_off = front_ctx
            .is_some_and(|ctx| ctx.standing == crate::save::SupplierStanding::CutOff);
        match &product_stock {
            // SOW-034: consumable product - stock badge, then BUY BATCH (first
            // acquisition) / RESTOCK (already have access). Gating order mirrors
            // the unlock path: cred lock, then the supplier's stock lock.
            Some(ps) => {
                let batch = ps.batch_cost;
                let (stock_label, in_stock) =
                    crate::ui::stock_view::shop_stock_line(ps.charges);
                card_parent.spawn((
                    Text::new(stock_label),
                    TextFont::from_font_size(13.0),
                    TextColor(if in_stock {
                        Color::srgb(0.3, 0.8, 0.3)
                    } else {
                        theme::ROSTER_STATUS_JAILED
                    }),
                ));

                if cred_gate.as_ref().is_some_and(|gate| !gate.met()) {
                    spawn_cred_lock_line(card_parent, cred_gate.as_ref().unwrap());
                } else if cut_off {
                    // SOW-031: a blown due date locks this supplier's whole
                    // stock until the debt is settled - no buy, no front
                    spawn_cut_off_line(card_parent);
                } else {
                    spawn_unlocked_by_line(card_parent, cred_gate.as_ref());
                    let label = if is_unlocked {
                        format!("RESTOCK ${batch}")
                    } else {
                        format!("BUY BATCH ${batch}")
                    };
                    spawn_purchase_button(
                        card_parent,
                        &card.id,
                        label,
                        batch,
                        Some(ps.unit_price),
                    );
                    // SOW-034: FRONT a batch when you have access but can't
                    // afford it in cash, while the supplier still deals on
                    // trust (Good + no front already running with them).
                    if is_unlocked
                        && front_ctx.is_some_and(|ctx| {
                            ctx.standing == crate::save::SupplierStanding::Good
                                && !ctx.has_front
                                && ctx.cash < batch as u64
                        })
                    {
                        spawn_front_button(card_parent, &card.id, area_id, batch);
                    }
                }
            }
            // Non-products stay permanent one-time unlocks (unchanged)
            None => {
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
                    spawn_cred_lock_line(card_parent, cred_gate.as_ref().unwrap());
                } else if cut_off {
                    spawn_cut_off_line(card_parent);
                } else {
                    spawn_unlocked_by_line(card_parent, cred_gate.as_ref());
                    spawn_purchase_button(card_parent, &card.id, format!("${price}"), price, None);
                }
            }
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
    mut deck_builder: Option<ResMut<crate::DeckBuilder>>,
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
            // SOW-031: a CutOff supplier's stock is locked until settled
            // (same spawn-vs-click defense as the cred guard)
            if let Some(area) = card.shop_location.as_deref() {
                if data.standing_with(area) == crate::save::SupplierStanding::CutOff {
                    info!("Cannot buy {}: supplier in {} cut you off - settle the front", button.card_id, area);
                    continue;
                }
            }
        }

        // SOW-034: a consumable product routes through buy_batch (grants
        // access on the first buy, adds a batch of charges, spends unit x
        // BATCH_SIZE); every other card is a permanent one-time unlock.
        match button.restock_unit {
            Some(unit) => {
                if !data.account.buy_batch(&button.card_id, unit, BATCH_SIZE) {
                    // Affordability was pre-checked, but buy_batch is the source
                    // of truth (no mutation on a short wallet)
                    info!("Cannot afford batch of {}", button.card_id);
                    continue;
                }
                info!(
                    "Bought a batch of {} for ${} ({} charges, remaining: ${})",
                    button.card_id,
                    button.price,
                    data.account.charges_in(&button.card_id),
                    data.account.cash_on_hand
                );
            }
            None => {
                data.account.cash_on_hand -= button.price as u64;
                data.account.unlocked_cards.insert(button.card_id.clone());
                info!(
                    "Purchased card {} for ${} (remaining: ${})",
                    button.card_id, button.price, data.account.cash_on_hand
                );
            }
        }

        // SOW-031 review fix: the pool reflects the buy NOW - the
        // DeckBuilder is otherwise only rebuilt at go-home
        if let Some(ref mut db) = deck_builder {
            db.resync_available(&game_assets, &data.account.unlocked_cards);
        }

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

/// SOW-031/034: take a BATCH on the supplier's credit. The model owns every
/// guard (standing, one-per-supplier, access precondition); this system just
/// routes the click and refreshes the tab.
pub fn front_take_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &FrontTakeButton), Changed<Interaction>>,
    mut save_data: Option<ResMut<SaveData>>,
    save_manager: Option<Res<SaveManager>>,
    shop_state: Res<ShopState>,
    game_assets: Res<GameAssets>,
    mut deck_builder: Option<ResMut<crate::DeckBuilder>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(ref mut data) = save_data else {
            continue;
        };
        match data.take_front(&button.card_id, &button.area_id, button.batch_cost) {
            Ok(()) => {
                let front = data.front_in(&button.area_id).expect("just taken");
                info!(
                    "Fronted a batch of {} in {}: ${} due in {} runs",
                    button.card_id, button.area_id, front.owed, front.runs_remaining
                );
                // SOW-031 review fix: playable NOW must be literal - the
                // window starts ticking on the very next run, so waiting
                // for the go-home rebuild burns a tick before the card
                // can earn
                if let Some(ref mut db) = deck_builder {
                    db.resync_available(&game_assets, &data.account.unlocked_cards);
                }
                if let Some(ref manager) = save_manager {
                    if let Err(e) = manager.save(data) {
                        warn!("Failed to save after front: {:?}", e);
                    }
                }
                refresh_shop_tab(&mut commands, &shop_state);
            }
            Err(reason) => info!("No front on {}: {}", button.card_id, reason),
        }
    }
}

/// SOW-031: settle a front in cash - the card becomes owned forever
pub fn front_pay_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &FrontPayButton), Changed<Interaction>>,
    mut save_data: Option<ResMut<SaveData>>,
    save_manager: Option<Res<SaveManager>>,
    shop_state: Res<ShopState>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(ref mut data) = save_data else {
            continue;
        };
        if data.pay_front(&button.area_id) {
            info!(
                "Front settled in {} (cash: ${})",
                button.area_id, data.account.cash_on_hand
            );
            if let Some(ref manager) = save_manager {
                if let Err(e) = manager.save(data) {
                    warn!("Failed to save after payoff: {:?}", e);
                }
            }
            refresh_shop_tab(&mut commands, &shop_state);
        } else {
            info!("Cannot settle front in {} - short on cash", button.area_id);
        }
    }
}

/// SOW-031: the shop tab rebuild trick shared by purchase/front/pay
/// (reinsert ShopState so populate_shop_cards_system sees a change)
fn refresh_shop_tab(commands: &mut Commands, shop_state: &ShopState) {
    commands.insert_resource(ShopState {
        viewing_shop: shop_state.viewing_shop,
        selected_location: shop_state.selected_location.clone(),
    });
}

/// SOW-031 review fix: affordability + FRONT offers are computed at
/// populate time, but cash can move without the shop being touched (the
/// roster strip's HIRE/BAIL/coolers are clickable while the tab is open).
/// Any save change while the shop is viewing re-populates the tab. Cheap:
/// fires only on actual SaveData writes, and populate never writes back.
pub fn shop_save_refresh_system(
    mut commands: Commands,
    save_data: Option<Res<SaveData>>,
    shop_state: Option<Res<ShopState>>,
) {
    let Some(save) = save_data else { return };
    if !save.is_changed() || save.is_added() {
        return;
    }
    let Some(shop_state) = shop_state else { return };
    if !shop_state.viewing_shop {
        return;
    }
    refresh_shop_tab(&mut commands, &shop_state);
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
