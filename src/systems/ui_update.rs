// SOW-AAA: UI update systems
// SOW-022: Rebound to the Game Play v2 screen (character clusters, hand fan,
// deck/discard stacks). Pure derivation logic lives in ui::view.
// Updated for Bevy 0.18

use bevy::prelude::*;
use crate::{CardType, HandState, HandPhase, DeckBuilder, Owner};
use crate::game_state::GameState;
use crate::ui::components::*;
use crate::ui::setup::spotlight_gradient;
use crate::ui::theme;
use crate::ui::view;
use crate::ui;
use crate::data::validate_deck;

/// SOW-022: Rebuild the fanned player hand (bottom arc)
pub fn recreate_hand_display_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    hand_display_query: Query<Entity, With<PlayerHandDisplay>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let Ok(hand_entity) = hand_display_query.single() else {
        return;
    };

    // Clear ALL existing children (wrappers + card buttons)
    if let Ok(children) = children_query.get(hand_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    // SOW-008/009: cards visible in every phase after the deal
    let show_cards = matches!(
        hand_state.current_state,
        HandPhase::PlayerPhase | HandPhase::DealerReveal | HandPhase::Resolve | HandPhase::Bust
    );
    if !show_cards {
        return;
    }

    let hand = &hand_state.cards(Owner::Player).hand;
    let fan = view::fan_layout(hand.len());
    let (card_width, _) = ui::CardSize::Hand.dimensions();

    commands.entity(hand_entity).with_children(|parent| {
        for (slot_index, (slot, fan_slot)) in hand.iter().zip(fan.iter()).enumerate() {
            // Positioned wrapper: the fan transform lives here so hover can
            // adjust it without touching the card button itself
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(1920.0 / 2.0 + fan_slot.offset_x - card_width / 2.0),
                        bottom: Val::Px(fan_slot.lift),
                        ..default()
                    },
                    UiTransform::from_rotation(Rot2::degrees(fan_slot.angle_deg)),
                    ZIndex(fan_slot.z),
                    HandCardWrapper {
                        angle_deg: fan_slot.angle_deg,
                        base_z: fan_slot.z,
                    },
                ))
                .with_children(|parent| {
                    if let Some(card) = slot {
                        // RFC-017: upgrade badge info from play counts
                        let tier = hand_state.get_card_tier(&card.name);
                        let play_count = hand_state.card_play_counts.get(&card.name).copied().unwrap_or(0);
                        let upgrade_info = ui::UpgradeInfo {
                            tier_name: tier.name().to_string(),
                            plays: play_count,
                            plays_to_next: tier.plays_to_next(),
                            multiplier: tier.multiplier(),
                            star_color: tier.star_color(),
                            is_foil: tier.is_foil(),
                        };

                        ui::spawn_card_button_with_upgrade(
                            parent,
                            &card.name,
                            &card.card_type,
                            ui::CardSize::Hand,
                            ui::CardDisplayState::Active,
                            CardButton { card_index: slot_index },
                            game_assets.card_template.clone(),
                            &emoji_font,
                            Some(upgrade_info),
                        );
                    } else {
                        ui::spawn_placeholder(
                            parent,
                            "Drawing...",
                            ui::CardSize::Hand,
                            theme::CARD_BORDER_NORMAL,
                            game_assets.card_back.clone(),
                        );
                    }
                });
        }
    });
}

/// SOW-022: Hovered hand card lifts, scales, and rises above its neighbors
pub fn hand_hover_system(
    interaction_query: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<CardButton>)>,
    mut wrapper_query: Query<(Entity, &HandCardWrapper, &mut UiTransform, &mut ZIndex)>,
    mut commands: Commands,
) {
    for (interaction, child_of) in interaction_query.iter() {
        let Ok((entity, wrapper, mut transform, mut z_index)) = wrapper_query.get_mut(child_of.parent()) else {
            continue;
        };
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *transform = UiTransform {
                    translation: Val2::px(0.0, -20.0),
                    scale: Vec2::splat(1.05),
                    rotation: Rot2::degrees(wrapper.angle_deg),
                };
                *z_index = ZIndex(30);
                commands.entity(entity).insert(BoxShadow::new(
                    theme::HAND_HOVER_GLOW,
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Px(22.0),
                ));
            }
            Interaction::None => {
                *transform = UiTransform::from_rotation(Rot2::degrees(wrapper.angle_deg));
                *z_index = ZIndex(wrapper.base_z);
                commands.entity(entity).remove::<BoxShadow>();
            }
        }
    }
}

/// SOW-022: Deck/discard stacks + card-back chip icons
pub fn update_deck_discard_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    deck_text_query: Query<Entity, With<DeckCounter>>,
    discard_text_query: Query<Entity, With<DiscardCountText>>,
    mut images: Query<&mut ImageNode>,
    deck_image_query: Query<Entity, With<DeckStackImage>>,
    narc_icon_query: Query<Entity, With<NarcCountChipIcon>>,
    buyer_icon_query: Query<Entity, With<BuyerCountChipIcon>>,
    top_slot_query: Query<Entity, With<DiscardTopCardSlot>>,
    children_query: Query<&Children>,
    mut commands: Commands,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Deck count
    if let Ok(entity) = deck_text_query.single() {
        if let Ok(mut text) = texts.get_mut(entity) {
            let label = format!("DECK · {}", hand_state.cards(Owner::Player).deck.len());
            if **text != label {
                **text = label;
            }
        }
    }

    // Card-back images (idempotent handle assignment)
    for entity in deck_image_query
        .iter()
        .chain(narc_icon_query.iter())
        .chain(buyer_icon_query.iter())
    {
        if let Ok(mut image) = images.get_mut(entity) {
            if image.image != game_assets.card_back {
                image.image = game_assets.card_back.clone();
            }
        }
    }

    // Discard stack: count + face-up top card
    let (discard_count, top_card) = view::discard_view(&hand_state.cards_played);

    if let Ok(entity) = discard_text_query.single() {
        if let Ok(mut text) = texts.get_mut(entity) {
            let label = format!("DISCARD · {discard_count}");
            if **text != label {
                **text = label;
            }
        }
    }

    if let Ok(slot_entity) = top_slot_query.single() {
        if let Ok(children) = children_query.get(slot_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        if let Some(card) = top_card {
            // RFC-017/018: badge info so the face-up card matches the stats
            // the totals actually applied (upgraded cover, tier-scaled evidence)
            let upgrade_info = ui::upgrade_info_for(hand_state, &card);
            commands.entity(slot_entity).with_children(|parent| {
                ui::spawn_card_display_with_upgrade(
                    parent,
                    &card.name,
                    &card.card_type,
                    ui::CardSize::Compact,
                    ui::CardDisplayState::Active,
                    PlayedCardDisplay,
                    game_assets.card_template.clone(),
                    &emoji_font,
                    upgrade_info,
                );
            });
        }
    }
}

/// SOW-022: spawn (emoji, value) stat rows for an intent/reaction bubble
fn spawn_bubble_stat_rows(
    parent: &mut ChildSpawnerCommands,
    rows: &[view::IntentRow],
    emoji_font: &crate::EmojiFont,
) {
    for (emoji, value) in rows {
        let color = match *emoji {
            "🔥" => theme::NARC_STAT_HEAT,
            "🛡" => theme::BALANCE_COVER_TEXT,
            "💰" => theme::BUYER_BUBBLE_PAYOUT,
            _ => theme::NARC_STAT_EVIDENCE, // 🔍 / ⚠
        };
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(*emoji),
                TextFont {
                    font: emoji_font.0.clone(),
                    font_size: 15.0,
                    ..default()
                },
                TextColor(color),
            ));
            parent.spawn((
                Text::new(value),
                TextFont::from_font_size(15.0),
                TextColor(color),
            ));
        });
    }
}

/// SOW-022: Narc count chip + intent bubble (telegraph / last played)
pub fn update_narc_intent_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    narc_count_query: Query<Entity, With<NarcCardCountText>>,
    title_query: Query<Entity, With<NarcIntentTitleText>>,
    mut bubble_query: Query<&mut Node, With<NarcIntentBubble>>,
    stats_row_query: Query<Entity, With<NarcIntentStatsRow>>,
    children_query: Query<&Children>,
    mut commands: Commands,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Face-down count chip
    if let Ok(entity) = narc_count_query.single() {
        if let Ok(mut text) = texts.get_mut(entity) {
            let count = hand_state.cards(Owner::Narc).hand.iter().flatten().count();
            let label = count.to_string();
            if **text != label {
                **text = label;
            }
        }
    }

    let Ok(mut bubble_node) = bubble_query.single_mut() else {
        return;
    };

    let Some(intent) = view::narc_intent(hand_state) else {
        bubble_node.display = Display::None;
        return;
    };

    bubble_node.display = Display::Flex;

    if let Ok(entity) = title_query.single() {
        if let Ok(mut text) = texts.get_mut(entity) {
            let label = format!("{} · {}", intent.verb, intent.card_name);
            if **text != label {
                **text = label;
            }
        }
    }

    if let Ok(row_entity) = stats_row_query.single() {
        if let Ok(children) = children_query.get(row_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        commands.entity(row_entity).with_children(|parent| {
            spawn_bubble_stat_rows(parent, &intent.rows, &emoji_font);
        });
    }
}

/// SOW-022 follow-up: on-screen callout for buyer reactions - the buyer's
/// plays were previously announced only on stdout, making their heat swings
/// look like bugs. Mirrors the narc intent bubble.
pub fn update_buyer_played_bubble_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    title_query: Query<Entity, With<BuyerPlayedTitleText>>,
    mut bubble_query: Query<&mut Node, With<BuyerPlayedBubble>>,
    stats_row_query: Query<Entity, With<BuyerPlayedStatsRow>>,
    children_query: Query<&Children>,
    mut commands: Commands,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let Ok(mut bubble_node) = bubble_query.single_mut() else {
        return;
    };

    let Some(played) = view::buyer_played(hand_state) else {
        bubble_node.display = Display::None;
        return;
    };

    bubble_node.display = Display::Flex;

    if let Ok(entity) = title_query.single() {
        if let Ok(mut text) = texts.get_mut(entity) {
            let label = format!("{} · {}", played.verb, played.card_name);
            if **text != label {
                **text = label;
            }
        }
    }

    if let Ok(row_entity) = stats_row_query.single() {
        if let Ok(children) = children_query.get(row_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        commands.entity(row_entity).with_children(|parent| {
            spawn_bubble_stat_rows(parent, &played.rows, &emoji_font);
        });
    }
}

/// SOW-022: Buyer cluster - name, wants bubble, heat-cap chip, count chip
pub fn update_buyer_panel_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    buyer_count_query: Query<Entity, With<BuyerCardCountText>>,
    name_query: Query<Entity, With<BuyerNameText>>,
    scenario_query: Query<Entity, With<BuyerScenarioNameText>>,
    demand_query: Query<Entity, With<BuyerDemandText>>,
    payout_query: Query<Entity, With<BuyerPayoutText>>,
    cap_text_query: Query<Entity, With<BuyerHeatCapText>>,
    detail_query: Query<Entity, With<BuyerDetailText>>,
    mut cap_chip_query: Query<&mut Node, With<BuyerHeatCapChip>>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let mut set_text = |entity_result: Result<Entity, _>, value: String| {
        if let Ok(entity) = entity_result {
            if let Ok(mut text) = texts.get_mut(entity) {
                if **text != value {
                    **text = value;
                }
            }
        }
    };

    // Card count chip
    let count = hand_state.cards(Owner::Buyer).hand.iter().flatten().count();
    set_text(buyer_count_query.single(), count.to_string());

    let Some(persona) = &hand_state.buyer_persona else {
        return;
    };

    set_text(name_query.single(), persona.display_name.to_uppercase());
    set_text(payout_query.single(), format!("×{:.1}", persona.base_multiplier));

    let scenario = persona
        .active_scenario_index
        .and_then(|idx| persona.scenarios.get(idx));

    if let Some(scenario) = scenario {
        set_text(
            scenario_query.single(),
            format!("WANTS · {}", scenario.display_name.to_uppercase()),
        );
        set_text(demand_query.single(), scenario.products.join(" / "));

        let mut detail = scenario.description.clone();
        if !scenario.locations.is_empty() {
            detail.push_str(&format!("\n\nPREFERS: {}", scenario.locations.join(", ")));
        }
        if let Some(evidence_cap) = persona.evidence_threshold {
            detail.push_str(&format!("\nBAILS AT EVIDENCE {evidence_cap}"));
        }
        set_text(detail_query.single(), detail);

        // Heat-cap chip: shown only when the scenario has a cap
        if let Ok(mut chip_node) = cap_chip_query.single_mut() {
            match scenario.heat_threshold {
                Some(cap) => {
                    chip_node.display = Display::Flex;
                    set_text(cap_text_query.single(), cap.to_string());
                }
                None => {
                    chip_node.display = Display::None;
                }
            }
        }
    }
}

/// SOW-022: Hovering the wants bubble expands scenario detail
pub fn buyer_bubble_hover_system(
    bubble_query: Query<&Interaction, (Changed<Interaction>, With<BuyerBubble>)>,
    mut detail_query: Query<&mut Node, With<BuyerDetailPanel>>,
) {
    for interaction in bubble_query.iter() {
        if let Ok(mut node) = detail_query.single_mut() {
            node.display = if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

/// SOW-022: Portrait spotlight follows the active actor
pub fn update_spotlights_system(
    hand_state_query: Query<&HandState>,
    mut narc_query: Query<&mut BackgroundGradient, (With<NarcSpotlight>, Without<BuyerSpotlight>)>,
    mut buyer_query: Query<&mut BackgroundGradient, (With<BuyerSpotlight>, Without<NarcSpotlight>)>,
    mut last_actor: Local<Option<view::PillActor>>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let (_, actor) = view::turn_pill(hand_state);
    if *last_actor == Some(actor) {
        return;
    }
    *last_actor = Some(actor);

    if let Ok(mut gradient) = narc_query.single_mut() {
        let color = if actor == view::PillActor::Narc {
            theme::NARC_SPOTLIGHT
        } else {
            theme::NARC_SPOTLIGHT.with_alpha(0.10)
        };
        *gradient = spotlight_gradient(color);
    }
    if let Ok(mut gradient) = buyer_query.single_mut() {
        let color = if actor == view::PillActor::Buyer {
            theme::BUYER_SPOTLIGHT.with_alpha(0.30)
        } else {
            theme::BUYER_SPOTLIGHT
        };
        *gradient = spotlight_gradient(color);
    }
}

pub fn update_deck_builder_ui_system(
    deck_builder: Option<Res<DeckBuilder>>,
    mut stats_query: Query<(&mut Text, &mut TextColor), With<DeckStatsDisplay>>,
) {
    let Some(deck_builder) = deck_builder else {
        return;
    };
    if !deck_builder.is_changed() {
        return;
    }

    for (mut text, mut text_color) in stats_query.iter_mut() {
        let count = deck_builder.selected_cards.len();
        let validation = validate_deck(&deck_builder.selected_cards);

        let is_valid = validation.is_ok();
        **text = match validation {
            Ok(_) => format!("Deck: {count}/20 cards ✓ VALID"),
            Err(msg) => format!("Deck: {count}/20 cards ✗ {msg}"),
        };

        text_color.0 = if is_valid {
            theme::SELECTED_DECK_BG_VALID
        } else {
            theme::SELECTED_DECK_BG_INVALID
        };
    }
}

pub fn populate_deck_builder_cards_system(
    mut commands: Commands,
    deck_builder: Option<Res<DeckBuilder>>,
    pool_container_query: Query<Entity, With<CardPoolContainer>>,
    card_button_query: Query<Entity, With<DeckBuilderCardButton>>,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
    save_data: Option<Res<crate::save::SaveData>>, // RFC-017: For card upgrade tiers
) {
    let Some(deck_builder) = deck_builder else {
        return;
    };

    // Only repopulate if DeckBuilder changed OR if there are no card buttons yet
    // (handles returning from UpgradeChoice where DeckBuilder exists but UI was just created)
    let has_card_buttons = !card_button_query.is_empty();
    if !deck_builder.is_changed() && has_card_buttons {
        return;
    }

    // Clear existing card buttons
    for entity in card_button_query.iter() {
        commands.entity(entity).despawn();
    }

    // SOW-010: Populate single grid with all cards (styled like played cards)
    if let Ok(pool_entity) = pool_container_query.single() {
        commands.entity(pool_entity).with_children(|parent| {
            // Sort cards by type, then alphabetically by name
            let mut sorted_cards = deck_builder.available_cards.clone();
            sorted_cards.sort_by(|a, b| {
                let type_order_a = match a.card_type {
                    CardType::Product { .. } => 0,
                    CardType::Location { .. } => 1,
                    CardType::Cover { .. } => 2,
                    CardType::DealModifier { .. } => 3,
                    CardType::Insurance { .. } => 4,
                    CardType::Evidence { .. } => 5,
                    CardType::Conviction { .. } => 6,
                };
                let type_order_b = match b.card_type {
                    CardType::Product { .. } => 0,
                    CardType::Location { .. } => 1,
                    CardType::Cover { .. } => 2,
                    CardType::DealModifier { .. } => 3,
                    CardType::Insurance { .. } => 4,
                    CardType::Evidence { .. } => 5,
                    CardType::Conviction { .. } => 6,
                };

                // First sort by type, then alphabetically by name
                type_order_a.cmp(&type_order_b).then_with(|| a.name.cmp(&b.name))
            });

            for card in &sorted_cards {
                let is_selected = deck_builder.selected_cards.iter().any(|c| c.id == card.id);

                let display_state = if is_selected {
                    ui::CardDisplayState::Selected
                } else {
                    ui::CardDisplayState::Inactive
                };

                // RFC-017: Get upgrade info from character state if available
                let upgrade_info = save_data.as_ref().and_then(|save| {
                    save.character.as_ref().map(|character| {
                        let play_count = character.get_play_count(&card.name);
                        let tier = character.get_card_tier(&card.name);
                        ui::UpgradeInfo {
                            tier_name: tier.name().to_string(),
                            plays: play_count,
                            plays_to_next: tier.plays_to_next(),
                            multiplier: tier.multiplier(),
                            star_color: tier.star_color(),
                            is_foil: tier.is_foil(),
                        }
                    })
                });

                // Use template-based rendering for deck builder cards
                ui::spawn_card_button_with_upgrade(
                    parent,
                    &card.name,
                    &card.card_type,
                    ui::CardSize::Small,
                    display_state,
                    DeckBuilderCardButton {
                        card_id: card.id.clone(),
                    },
                    game_assets.card_template.clone(),
                    &emoji_font,
                    upgrade_info,
                );
            }
        });
    }
}

pub fn toggle_game_state_ui_system(
    current_state: Res<bevy::state::state::State<GameState>>,
    mut deck_builder_query: Query<&mut Node, (With<DeckBuilderRoot>, Without<UiRoot>)>,
    mut gameplay_ui_query: Query<&mut Node, (With<UiRoot>, Without<DeckBuilderRoot>)>,
) {
    // Show deck builder in DeckBuilding state, hide in InRun
    if let Ok(mut node) = deck_builder_query.single_mut() {
        node.display = if current_state.get() == &GameState::DeckBuilding {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Show gameplay UI in InRun state, hide in DeckBuilding
    if let Ok(mut node) = gameplay_ui_query.single_mut() {
        node.display = if current_state.get() == &GameState::InRun {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_actor_portraits_system(
    hand_state_query: Query<&HandState>,
    mut buyer_portrait_query: Query<&mut ImageNode, (With<BuyerPortrait>, Without<NarcPortrait>)>,
    mut narc_portrait_query: Query<&mut ImageNode, (With<NarcPortrait>, Without<BuyerPortrait>)>,
    game_assets: Res<crate::assets::GameAssets>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Update buyer portrait based on current buyer persona
    if let Ok(mut buyer_image) = buyer_portrait_query.single_mut() {
        if let Some(persona) = &hand_state.buyer_persona {
            if let Some(portrait_handle) = game_assets.actor_portraits.get(&persona.display_name) {
                if buyer_image.image != *portrait_handle {
                    buyer_image.image = portrait_handle.clone();
                }
            }
        }
    }

    // Update narc portrait (always "Narc")
    if let Ok(mut narc_image) = narc_portrait_query.single_mut() {
        if let Some(portrait_handle) = game_assets.actor_portraits.get("Narc") {
            if narc_image.image != *portrait_handle {
                narc_image.image = portrait_handle.clone();
            }
        }
    }
}
