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

/// SOW-022: Deck/discard stacks
pub fn update_deck_discard_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    deck_text_query: Query<Entity, With<DeckCounter>>,
    discard_text_query: Query<Entity, With<DiscardCountText>>,
    mut images: Query<&mut ImageNode>,
    deck_image_query: Query<Entity, With<DeckStackImage>>,
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

    // Deck stack card-back image (idempotent handle assignment)
    if let Ok(entity) = deck_image_query.single() {
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

/// SOW-022: Narc intent bubble (telegraph / last played)
pub fn update_narc_intent_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
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

/// SOW-022: Buyer cluster - name, scenario placard (wants / payout /
/// confidence face / hover detail)
pub fn update_buyer_panel_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    name_query: Query<Entity, With<BuyerNameText>>,
    scenario_query: Query<Entity, With<BuyerScenarioNameText>>,
    demand_query: Query<Entity, With<BuyerDemandText>>,
    payout_query: Query<Entity, With<BuyerPayoutText>>,
    detail_query: Query<Entity, With<BuyerDetailText>>,
    confidence_emoji_query: Query<Entity, With<BuyerConfidenceEmoji>>,
    confidence_text_query: Query<Entity, With<BuyerConfidenceText>>,
    mut text_colors: Query<&mut TextColor>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Confidence face: how close the buyer is to bailing (worst of the
    // heat / evidence bail axes)
    let confidence = view::buyer_confidence(hand_state);
    if let Some(confidence) = confidence {
        let color = match confidence {
            view::BuyerConfidence::Confident => theme::SAFE_CHIP_TEXT,
            view::BuyerConfidence::Nervous => theme::STANDING_HEAT_VALUE,
            view::BuyerConfidence::Scared => theme::RISK_CHIP_TEXT,
        };
        for (entity_result, value) in [
            (confidence_emoji_query.single(), confidence.emoji()),
            (confidence_text_query.single(), confidence.label()),
        ] {
            if let Ok(entity) = entity_result {
                if let Ok(mut text) = texts.get_mut(entity) {
                    if **text != value {
                        **text = value.to_string();
                    }
                }
                if let Ok(mut text_color) = text_colors.get_mut(entity) {
                    if text_color.0 != color {
                        text_color.0 = color;
                    }
                }
            }
        }
    }

    let mut set_text = |entity_result: Result<Entity, _>, value: String| {
        if let Ok(entity) = entity_result {
            if let Ok(mut text) = texts.get_mut(entity) {
                if **text != value {
                    **text = value;
                }
            }
        }
    };

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

        // Exact bail thresholds are scenario detail - they live in the hover
        // panel now that the confidence face summarizes them at a glance
        let mut detail = scenario.description.clone();
        if !scenario.locations.is_empty() {
            detail.push_str(&format!("\n\nPREFERS: {}", scenario.locations.join(", ")));
        }
        if let Some(heat_cap) = scenario.heat_threshold {
            detail.push_str(&format!("\nBAILS AT HEAT {heat_cap}"));
        }
        if let Some(evidence_cap) = persona.evidence_threshold {
            detail.push_str(&format!("\nBAILS AT EVIDENCE {evidence_cap}"));
        }
        set_text(detail_query.single(), detail);
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

/// SOW-023: Operations roster - one card per dealer (portrait, name,
/// tier, scars, status), plus a HIRE card. Rebuilt when the save changes or
/// the panel is freshly (re)spawned. Card width is FIXED at 250px so the e2e
/// harness can target rows deterministically.
pub fn populate_roster_panel_system(
    mut commands: Commands,
    save_data: Option<Res<crate::save::SaveData>>,
    panel_query: Query<Entity, With<RosterPanel>>,
    children_query: Query<&Children>,
    game_assets: Res<crate::assets::GameAssets>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    let Ok(panel) = panel_query.single() else {
        return; // panel only exists on the deck-builder screen
    };

    let is_empty = children_query.get(panel).map(|c| c.is_empty()).unwrap_or(true);
    if !save_data.is_changed() && !is_empty {
        return;
    }

    if let Ok(children) = children_query.get(panel) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    let cash = save_data.account.cash_on_hand;
    let hire_cost = save_data.next_hire_cost();

    commands.entity(panel).with_children(|parent| {
        for (index, dealer) in save_data.dealers.iter().enumerate() {
            let is_active = index == save_data.active_dealer;
            let jailed = dealer.jail_remaining();

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(if jailed.is_some() {
                        theme::ROSTER_CARD_BG_JAILED
                    } else {
                        theme::ROSTER_CARD_BG
                    }),
                    BorderColor::all(if is_active {
                        theme::ROSTER_CARD_BORDER_ACTIVE
                    } else {
                        theme::ROSTER_CARD_BORDER
                    }),
                    RosterDealerButton { dealer_index: index },
                ))
                .with_children(|parent| {
                    // Portrait
                    parent.spawn((
                        Node {
                            width: Val::Px(72.0),
                            height: Val::Px(72.0),
                            flex_shrink: 0.0,
                            ..default()
                        },
                        ImageNode {
                            image: game_assets
                                .actor_portraits
                                .get(&dealer.portrait)
                                .cloned()
                                .unwrap_or_default(),
                            ..default()
                        },
                    ));

                    // Identity + status column. min_width 0 lets the column
                    // shrink below its text's intrinsic width (SOW-027: the
                    // action stack must stay INSIDE the fixed 250px card -
                    // overflowing slides it under the next card, which then
                    // steals the buttons' clicks)
                    parent.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(2.0),
                        flex_grow: 1.0,
                        flex_shrink: 1.0,
                        min_width: Val::Px(0.0),
                        overflow: Overflow::clip_x(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Name row (+ BOSS badge, + scars)
                        parent.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(6.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(&dealer.name),
                                TextFont::from_font_size(15.0),
                                TextColor(Color::WHITE),
                            ));
                            if dealer.is_kingpin {
                                parent.spawn((
                                    Text::new("BOSS"),
                                    TextFont::from_font_size(11.0),
                                    TextColor(theme::ROSTER_KINGPIN_BADGE),
                                ));
                            }
                            if dealer.prior_convictions > 0 {
                                // ⚖ renders in DejaVuSans (established convention)
                                parent.spawn((
                                    Text::new(format!("⚖ {}", dealer.prior_convictions)),
                                    TextFont::from_font_size(11.0),
                                    TextColor(theme::ROSTER_SCAR_TEXT),
                                ));
                            }
                        });

                        // Heat tier
                        let tier = dealer.character.heat_tier();
                        let (r, g, b) = tier.color();
                        parent.spawn((
                            Text::new(format!("Heat {} [{}]", dealer.character.heat, tier.name())),
                            TextFont::from_font_size(12.0),
                            TextColor(Color::srgb(r, g, b)),
                        ));

                        // SOW-025: station + reputation there ("THE BLOCK · CRED 5")
                        let station_name = game_assets
                            .shop_locations
                            .iter()
                            .find(|a| a.id == dealer.station)
                            .map(|a| a.name.to_uppercase())
                            .unwrap_or_else(|| dealer.station.to_uppercase());
                        parent.spawn((
                            Text::new(format!(
                                "{} · CRED {}",
                                station_name,
                                dealer.cred_in(&dealer.station)
                            )),
                            TextFont::from_font_size(11.0),
                            TextColor(theme::ROSTER_STATION_TEXT),
                        ));

                        // Status
                        if let Some(runs) = jailed {
                            parent.spawn((
                                Text::new(format!(
                                    "JAILED · {} RUN{}",
                                    runs,
                                    if runs == 1 { "" } else { "S" }
                                )),
                                TextFont::from_font_size(12.0),
                                TextColor(theme::ROSTER_STATUS_JAILED),
                            ));
                        } else if let Some(runs) = dealer.relocating_remaining() {
                            // SOW-025: getting established in the new station
                            parent.spawn((
                                Text::new(format!(
                                    "MOVING · {} RUN{}",
                                    runs,
                                    if runs == 1 { "" } else { "S" }
                                )),
                                TextFont::from_font_size(12.0),
                                TextColor(theme::ROSTER_STATUS_MOVING),
                            ));
                        } else if let Some(runs) = dealer.laying_low_remaining() {
                            // SOW-027: gone dark - resurfaces cooler
                            parent.spawn((
                                Text::new(format!(
                                    "LAYING LOW · {} RUN{}",
                                    runs,
                                    if runs == 1 { "" } else { "S" }
                                )),
                                TextFont::from_font_size(12.0),
                                TextColor(theme::ROSTER_STATUS_LAYING_LOW),
                            ));
                        } else {
                            parent.spawn((
                                Text::new("READY"),
                                TextFont::from_font_size(12.0),
                                TextColor(theme::ROSTER_STATUS_READY),
                            ));
                        }
                    });

                    // Bail button on jailed dealers (nested Button blocks the
                    // row's select interaction, so bailing doesn't re-select)
                    if let Some(runs) = jailed {
                        let cost = crate::save::bail_cost(runs);
                        let affordable = cash >= cost;
                        parent
                            .spawn((
                                Button,
                                Node {
                                    padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    flex_shrink: 0.0,
                                    ..default()
                                },
                                BackgroundColor(if affordable {
                                    theme::ROSTER_BAIL_BG
                                } else {
                                    theme::BUTTON_DISABLED_BG
                                }),
                                RosterBailButton { dealer_index: index },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(format!("BAIL\n${cost}")),
                                    TextFont::from_font_size(12.0),
                                    TextColor(Color::WHITE),
                                    TextLayout::new_with_justify(bevy::text::Justify::Center),
                                ));
                            });
                    }

                    // SOW-025/SOW-027: action stack for available dealers -
                    // MOVE (relocate), LAY LOW and LAWYER (heat coolers)
                    // stacked vertically so the fixed 250px row never
                    // overflows. Buttons are nested so clicks don't re-select
                    // the row; ineligible actions render disabled with the
                    // reason in the label.
                    if dealer.is_available() {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(4.0),
                                flex_shrink: 0.0,
                                ..default()
                            })
                            .with_children(|parent| {
                                // SOW-025: MOVE - first unlocked area that
                                // isn't their station; the full area picker
                                // arrives with the map screen SOW.
                                let move_target = game_assets
                                    .shop_locations
                                    .iter()
                                    .find(|a| {
                                        a.id != dealer.station
                                            && save_data
                                                .account
                                                .unlocked_locations
                                                .contains(&a.id)
                                    });
                                if let Some(target) = move_target {
                                    let fee = save_data.move_fee();
                                    let affordable = cash >= fee;
                                    parent
                                        .spawn((
                                            Button,
                                            Node {
                                                padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            BackgroundColor(if affordable {
                                                theme::ROSTER_MOVE_BG
                                            } else {
                                                theme::BUTTON_DISABLED_BG
                                            }),
                                            RosterMoveButton {
                                                dealer_index: index,
                                                to_area: target.id.clone(),
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new(format!(
                                                    "MOVE TO\n{}\n${fee}",
                                                    target.name.to_uppercase()
                                                )),
                                                TextFont::from_font_size(10.0),
                                                TextColor(Color::WHITE),
                                                TextLayout::new_with_justify(
                                                    bevy::text::Justify::Center,
                                                ),
                                            ));
                                        });
                                }

                                // SOW-027: the coolers. Both need heat to
                                // shed; the label carries the disable reason.
                                let has_heat = dealer.character.heat > 0;

                                let lay_low_cost = save_data.lay_low_cost();
                                let lay_low_ok = has_heat && cash >= lay_low_cost;
                                parent
                                    .spawn((
                                        Button,
                                        Node {
                                            padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(if lay_low_ok {
                                            theme::ROSTER_LAY_LOW_BG
                                        } else {
                                            theme::BUTTON_DISABLED_BG
                                        }),
                                        RosterLayLowButton { dealer_index: index },
                                    ))
                                    .with_children(|parent| {
                                        let label = if !has_heat {
                                            "LAY LOW\nNO HEAT".to_string()
                                        } else {
                                            format!(
                                                "LAY LOW\n{} RUNS · -{}\n${lay_low_cost}",
                                                crate::save::LAY_LOW_RUNS,
                                                crate::save::LAY_LOW_COOLING,
                                            )
                                        };
                                        parent.spawn((
                                            Text::new(label),
                                            TextFont::from_font_size(10.0),
                                            TextColor(Color::WHITE),
                                            TextLayout::new_with_justify(
                                                bevy::text::Justify::Center,
                                            ),
                                        ));
                                    });

                                let lawyer_cost = save_data.lawyer_cost();
                                let lawyer_ok = has_heat && cash >= lawyer_cost;
                                parent
                                    .spawn((
                                        Button,
                                        Node {
                                            padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        BackgroundColor(if lawyer_ok {
                                            theme::ROSTER_LAWYER_BG
                                        } else {
                                            theme::BUTTON_DISABLED_BG
                                        }),
                                        RosterLawyerButton { dealer_index: index },
                                    ))
                                    .with_children(|parent| {
                                        let label = if !has_heat {
                                            "LAWYER\nNO HEAT".to_string()
                                        } else {
                                            format!(
                                                "LAWYER\n-{} NOW\n${lawyer_cost}",
                                                crate::save::LAWYER_COOLING,
                                            )
                                        };
                                        parent.spawn((
                                            Text::new(label),
                                            TextFont::from_font_size(10.0),
                                            TextColor(Color::WHITE),
                                            TextLayout::new_with_justify(
                                                bevy::text::Justify::Center,
                                            ),
                                        ));
                                    });
                            });
                    }
                });
        }

        // HIRE card
        let affordable = cash >= hire_cost;
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(140.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(if affordable {
                    theme::ROSTER_HIRE_BG
                } else {
                    theme::BUTTON_DISABLED_BG
                }),
                BorderColor::all(theme::ROSTER_CARD_BORDER),
                RosterHireButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(format!("HIRE\n${hire_cost}")),
                    TextFont::from_font_size(14.0),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(bevy::text::Justify::Center),
                ));
            });
    });
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

                // RFC-017/023: Get upgrade info from the ACTIVE dealer's record
                let upgrade_info = save_data.as_ref().map(|save| {
                    let character = save.active_character();
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

/// SOW-031: the hub's front clock next to START RUN - the most urgent
/// front named to its supplier, red ink when a due date is one run out.
/// Empty while the books are clean. Derivation lives in ui::front_view.
pub fn update_front_pressure_system(
    save_data: Option<Res<crate::save::SaveData>>,
    game_assets: Res<crate::assets::GameAssets>,
    mut query: Query<(&mut Text, &mut TextColor), With<FrontPressureText>>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    if !save_data.is_changed() {
        return;
    }
    for (mut text, mut color) in query.iter_mut() {
        match crate::ui::front_view::pressure_line(&save_data, &game_assets.shop_locations) {
            Some(line) => {
                **text = line;
                *color = TextColor(if crate::ui::front_view::pressure_urgent(&save_data) {
                    theme::ROSTER_STATUS_JAILED
                } else {
                    theme::LEDGER_BOARD_CURRENT
                });
            }
            None => **text = String::new(),
        }
    }
}
