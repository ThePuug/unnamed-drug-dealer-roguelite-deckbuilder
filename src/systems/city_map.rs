// SOW-029: City Map overlay - the management surface for territories.
// Answers "where can I operate, who works where, what's the next zone
// worth" and hosts the two actions: zone unlock (ShopAreaUnlockButton,
// same path as SOW-024) and dealer relocation (RosterMoveButton, same
// path as SOW-025). All derivation lives in ui::map_view; this file only
// orchestrates spawning and clicks.

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::save::SaveData;
use crate::ui::components::*;
use crate::ui::map_view::{self, MoveEligibility, ZoneStatus};
use crate::ui::theme;

/// Map overlay state. Selection arms the move flow: pick a dealer chip,
/// then a destination node's SEND button commits (via RosterMoveButton).
#[derive(Resource, Default)]
pub struct MapUiState {
    pub open: bool,
    pub selected_dealer: Option<usize>,
}

/// Spawn the (hidden) overlay under DeckBuilderRoot - it inherits the
/// 1920x1080 design-space scaling and the on-exit cleanup for free.
pub fn spawn_map_overlay(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                display: Display::None,
                ..default()
            },
            // Opaque canvas - the hub bleeding through read as clutter in
            // the e2e screenshots
            BackgroundColor(Color::srgb(0.02, 0.03, 0.06)),
            // Nodes default to FocusPolicy::Pass, which would let clicks on
            // the canvas fall through to the still-spawned hub buttons
            // beneath (deck cards, roster spends, START RUN). CLOSE is the
            // only way out - the hidden CITY MAP tab no longer toggles.
            bevy::ui::FocusPolicy::Block,
            GlobalZIndex(90),
            MapOverlay,
        ))
        .with_children(|overlay| {
            // Header: title + hint on the left, CLOSE on the right
            overlay
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                    ..default()
                })
                .with_children(|header| {
                    header
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|title| {
                            title.spawn((
                                Text::new("THE CITY"),
                                TextFont::from_font_size(28.0),
                                TextColor(theme::TEXT_HEADER),
                            ));
                            title.spawn((
                                Text::new(""),
                                TextFont::from_font_size(13.0),
                                TextColor(theme::V2_LABEL),
                                MapHintText,
                            ));
                        });

                    header
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(110.0),
                                height: Val::Px(44.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(theme::BUTTON_NEUTRAL_BG),
                            MapCloseButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("CLOSE"),
                                TextFont::from_font_size(16.0),
                                TextColor(Color::WHITE),
                            ));
                        });
                });

            // The three zone nodes (children rebuilt by populate system)
            overlay.spawn((
                Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                MapNodesRow,
            ));
        });
}

/// MAP button opens (or closes) the overlay; CLOSE closes it. Selection
/// resets on close so the move flow never survives a hidden overlay.
pub fn map_toggle_system(
    open_query: Query<&Interaction, (Changed<Interaction>, With<MapButton>)>,
    close_query: Query<&Interaction, (Changed<Interaction>, With<MapCloseButton>)>,
    mut map_state: ResMut<MapUiState>,
    mut overlay_query: Query<&mut Node, With<MapOverlay>>,
) {
    let mut changed = false;
    for interaction in open_query.iter() {
        if *interaction == Interaction::Pressed {
            map_state.open = !map_state.open;
            changed = true;
        }
    }
    for interaction in close_query.iter() {
        if *interaction == Interaction::Pressed {
            map_state.open = false;
            changed = true;
        }
    }
    if !changed {
        return;
    }
    if !map_state.open {
        map_state.selected_dealer = None;
    }
    if let Ok(mut node) = overlay_query.single_mut() {
        node.display = if map_state.open { Display::Flex } else { Display::None };
    }
}

/// Clicking a dealer chip arms (or disarms) the move flow with that dealer
pub fn map_dealer_chip_system(
    chip_query: Query<(&Interaction, &MapDealerChipButton), Changed<Interaction>>,
    save_data: Option<Res<SaveData>>,
    mut map_state: ResMut<MapUiState>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    for (interaction, chip) in chip_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if map_state.selected_dealer == Some(chip.dealer_index) {
            map_state.selected_dealer = None;
        } else if save_data
            .dealers
            .get(chip.dealer_index)
            .is_some_and(|d| d.is_available())
        {
            map_state.selected_dealer = Some(chip.dealer_index);
        }
    }
}

/// Rebuild the node cards when the save or the map state changes (mirrors
/// populate_roster_panel_system's rebuild pattern)
pub fn populate_map_nodes_system(
    mut commands: Commands,
    save_data: Option<Res<SaveData>>,
    mut map_state: ResMut<MapUiState>,
    row_query: Query<Entity, With<MapNodesRow>>,
    children_query: Query<&Children>,
    mut hint_query: Query<&mut Text, With<MapHintText>>,
    game_assets: Res<GameAssets>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    let Ok(row) = row_query.single() else {
        return; // overlay only exists on the deck-builder screen
    };

    let is_empty = children_query.get(row).map(|c| c.is_empty()).unwrap_or(true);
    if !save_data.is_changed() && !map_state.is_changed() && !is_empty {
        return;
    }

    // A selection that went stale (dealer jailed/moved between frames)
    // disarms the flow; the mutation re-triggers one settling rebuild
    if map_state
        .selected_dealer
        .is_some_and(|i| !save_data.dealers.get(i).is_some_and(|d| d.is_available()))
    {
        map_state.selected_dealer = None;
    }
    let selected = map_state.selected_dealer;

    if let Ok(mut hint) = hint_query.single_mut() {
        let line = map_view::map_hint(&save_data, selected);
        if **hint != line {
            **hint = line;
        }
    }

    if let Ok(children) = children_query.get(row) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    commands.entity(row).with_children(|parent| {
        for area in &game_assets.shop_locations {
            let node = map_view::zone_node_view(
                area,
                &save_data,
                &game_assets.buyers,
                game_assets.products.values(),
            );
            spawn_zone_node(parent, &node, &save_data, selected);
        }
    });
}

/// Text color, dimmed on locked nodes (the aspiration is visible, grayed)
fn ink(color: Color, locked: bool) -> Color {
    if locked { color.with_alpha(0.45) } else { color }
}

fn spawn_zone_node(
    parent: &mut ChildSpawnerCommands,
    node: &map_view::ZoneNodeView,
    save_data: &SaveData,
    selected: Option<usize>,
) {
    let locked = !matches!(node.status, ZoneStatus::Unlocked);

    parent
        .spawn((
            Node {
                width: Val::Px(480.0),
                height: Val::Px(680.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(if locked {
                Color::srgba(0.07, 0.075, 0.11, 0.85)
            } else {
                theme::ROSTER_CARD_BG.with_alpha(0.92)
            }),
            BorderColor::all(if locked {
                theme::ROSTER_CARD_BORDER.with_alpha(0.4)
            } else {
                theme::ROSTER_CARD_BORDER
            }),
        ))
        .with_children(|card| {
            // Header: zone name (+ price when locked) and identity line
            let title = match node.status {
                ZoneStatus::Locked { price, .. } => {
                    format!("{} — ${}", node.name.to_uppercase(), price)
                }
                ZoneStatus::Unlocked => node.name.to_uppercase(),
            };
            card.spawn((
                Text::new(title),
                TextFont::from_font_size(24.0),
                TextColor(ink(Color::WHITE, locked)),
            ));
            card.spawn((
                Text::new(node.identity),
                TextFont::from_font_size(11.0),
                TextColor(ink(theme::V2_LABEL, false)), // label color reads dim already
            ));

            // Narc texture - stays fully visible on locked zones (the risk
            // is part of the pitch)
            card.spawn((
                Text::new(format!("NARC: {}", node.narc_hint)),
                TextFont::from_font_size(13.0),
                TextColor(theme::NARC_BUBBLE_TITLE),
            ));

            // Clientele
            let band = node
                .payout_band
                .as_ref()
                .map(|b| format!("CLIENTELE · {b}"))
                .unwrap_or_else(|| "CLIENTELE".to_string());
            card.spawn((
                Text::new(band),
                TextFont::from_font_size(12.0),
                TextColor(ink(theme::BUYER_BUBBLE_LABEL, locked)),
                Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
            ));
            for line in &node.clientele {
                card.spawn((
                    Text::new(line),
                    TextFont::from_font_size(15.0),
                    TextColor(ink(theme::BUYER_BUBBLE_DEMAND, locked)),
                ));
            }

            // Products
            card.spawn((
                Text::new("PRODUCTS"),
                TextFont::from_font_size(12.0),
                TextColor(ink(theme::BUYER_BUBBLE_LABEL, locked)),
                Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
            ));
            card.spawn((
                Text::new(node.products.join(" · ")),
                TextFont::from_font_size(14.0),
                TextColor(ink(theme::PRODUCT_CARD_COLOR, locked)),
            ));

            // Dealers (unlocked zones only - stations are unlocked areas)
            if !locked {
                card.spawn((
                    Text::new("DEALERS"),
                    TextFont::from_font_size(12.0),
                    TextColor(theme::BUYER_BUBBLE_LABEL),
                    Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
                ));
                if node.dealers.is_empty() {
                    card.spawn((
                        Text::new("nobody stationed here"),
                        TextFont::from_font_size(13.0),
                        TextColor(theme::V2_LABEL),
                    ));
                }
                for chip in &node.dealers {
                    spawn_dealer_chip(card, chip, selected);
                }

                // SOW-030: zone history, derived from the same numbers as
                // the ledger. BELOW the chips so the harness's chip-y
                // reference coordinates stay put.
                if let Some(history) = crate::ui::ledger_view::zone_history_line(save_data, &node.area_id) {
                    card.spawn((
                        Text::new(history),
                        TextFont::from_font_size(12.0),
                        TextColor(theme::SHOP_CREDIT_LINE_TEXT),
                    ));
                }
            }

            // Action area pinned to the card bottom
            card.spawn(Node { flex_grow: 1.0, ..default() });
            match node.status {
                ZoneStatus::Locked { price, affordable } => {
                    card.spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(if affordable {
                            theme::CONTINUE_BUTTON_BG
                        } else {
                            theme::BUTTON_DISABLED_BG
                        }),
                        ShopAreaUnlockButton {
                            location_id: node.area_id.clone(),
                            price,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("UNLOCK · ${price}")),
                            TextFont::from_font_size(18.0),
                            TextColor(Color::WHITE),
                        ));
                    });
                }
                ZoneStatus::Unlocked => {
                    if let Some(dealer_index) = selected {
                        spawn_send_action(card, save_data, dealer_index, node);
                    }
                }
            }
        });
}

fn spawn_dealer_chip(
    card: &mut ChildSpawnerCommands,
    chip: &map_view::DealerChip,
    selected: Option<usize>,
) {
    let is_selected = selected == Some(chip.dealer_index);
    card.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(2.0),
            padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(if chip.selectable {
            theme::ROSTER_CARD_BG
        } else {
            theme::ROSTER_CARD_BG_JAILED
        }),
        BorderColor::all(if is_selected {
            theme::ROSTER_CARD_BORDER_ACTIVE
        } else {
            theme::ROSTER_CARD_BORDER
        }),
        MapDealerChipButton { dealer_index: chip.dealer_index },
    ))
    .with_children(|chip_node| {
        chip_node
            .spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            })
            .with_children(|row| {
                // ★ renders in DejaVuSans (used by upgrade tiers)
                let name = if chip.is_best_cred {
                    format!("★ {}", chip.name)
                } else {
                    chip.name.clone()
                };
                row.spawn((
                    Text::new(name),
                    TextFont::from_font_size(14.0),
                    TextColor(if chip.is_best_cred {
                        theme::SHOP_CREDIT_LINE_TEXT
                    } else {
                        Color::WHITE
                    }),
                ));
                let (r, g, b) = chip.tier_color;
                row.spawn((
                    Text::new(format!("HEAT {} [{}] · CRED {}", chip.heat, chip.tier_name, chip.cred)),
                    TextFont::from_font_size(12.0),
                    TextColor(Color::srgb(r, g, b)),
                ));
            });
        if let Some(note) = &chip.status_note {
            chip_node.spawn((
                Text::new(note.as_str()),
                TextFont::from_font_size(11.0),
                TextColor(theme::ROSTER_STATUS_JAILED),
            ));
        }
    });
}

/// The destination action for the armed dealer: SEND button (full cost on
/// its face - the click IS the confirm) or a "stationed here" tag
fn spawn_send_action(
    card: &mut ChildSpawnerCommands,
    save_data: &SaveData,
    dealer_index: usize,
    node: &map_view::ZoneNodeView,
) {
    let dealer_name = save_data
        .dealers
        .get(dealer_index)
        .map(|d| d.name.to_uppercase())
        .unwrap_or_default();

    match map_view::move_eligibility(save_data, dealer_index, &node.area_id) {
        MoveEligibility::StationedHere => {
            card.spawn((
                Text::new(format!("{dealer_name} IS STATIONED HERE")),
                TextFont::from_font_size(13.0),
                TextColor(theme::ROSTER_STATION_TEXT),
                Node { align_self: AlignSelf::Center, ..default() },
            ));
        }
        MoveEligibility::DealerUnavailable => {} // stale selection settles next rebuild
        eligibility @ (MoveEligibility::Eligible { fee } | MoveEligibility::CantAfford { fee }) => {
            let affordable = matches!(eligibility, MoveEligibility::Eligible { .. });
            card.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(56.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(if affordable {
                    theme::ROSTER_MOVE_BG
                } else {
                    theme::BUTTON_DISABLED_BG
                }),
                // Same commit path as the roster's MOVE button (SOW-025)
                RosterMoveButton {
                    dealer_index,
                    to_area: node.area_id.clone(),
                },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(format!("SEND {dealer_name} HERE · ${fee} · 1 RUN OUT")),
                    TextFont::from_font_size(15.0),
                    TextColor(Color::WHITE),
                ));
            });
        }
    }
}
