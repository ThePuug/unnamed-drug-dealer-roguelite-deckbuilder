// SOW-030: Kingpin Ledger overlay - the empire's memory. Three panels
// (empire strip / roster dossiers / fallen-empires board) plus a story
// feed one click deeper. All derivation lives in ui::ledger_view; this
// file only orchestrates spawning and clicks.
//
// The two SOW-029 review lessons are baked in: LedgerUiState is
// init_resource'd in main.rs (systems run on the pending-upgrades frame
// where setup early-returns), and the overlay root carries
// FocusPolicy::Block so canvas clicks never reach the hub beneath.

use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::save::SaveData;
use crate::ui::components::*;
use crate::ui::ledger_view;
use crate::ui::theme;
use crate::ui::view::format_cash;

/// Ledger overlay state. `story_focus` is which record's story feed is
/// open in the third panel.
#[derive(Resource, Default)]
pub struct LedgerUiState {
    pub open: bool,
    pub story_focus: Option<StoryFocus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoryFocus {
    Dealer(usize),
    Epitaph(usize),
}

/// Cap the visible feed - the ledger is a surface, not an archive dump.
/// Older entries collapse into a "… n earlier" tail line.
const STORY_FEED_CAP: usize = 15;

/// Spawn the (hidden) overlay under DeckBuilderRoot - same inheritance
/// as the map overlay (design-space scaling + state-exit cleanup).
pub fn spawn_ledger_overlay(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(12.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(theme::LEDGER_CANVAS_BG),
            // SOW-029 lesson: Nodes default to FocusPolicy::Pass; without
            // Block, canvas clicks land on the live hub buttons beneath
            bevy::ui::FocusPolicy::Block,
            GlobalZIndex(92),
            LedgerOverlay,
        ))
        .with_children(|overlay| {
            // Header: title left, CLOSE right (same geometry as the map's
            // CLOSE so the harness reference click works on both)
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
                                Text::new("THE LEDGER"),
                                TextFont::from_font_size(28.0),
                                TextColor(theme::TEXT_HEADER),
                            ));
                            title.spawn((
                                Text::new("Every deal remembered. Click a dossier or a fallen empire for its stories."),
                                TextFont::from_font_size(13.0),
                                TextColor(theme::V2_LABEL),
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
                            LedgerCloseButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("CLOSE"),
                                TextFont::from_font_size(16.0),
                                TextColor(Color::WHITE),
                            ));
                        });
                });

            // Body (empire strip + three panels) rebuilt by populate system
            overlay.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                LedgerBody,
            ));
        });
}

/// LEDGER tab opens (or closes) the overlay; CLOSE closes it. Story focus
/// resets on close so a stale feed never survives a hidden overlay.
pub fn ledger_toggle_system(
    open_query: Query<&Interaction, (Changed<Interaction>, With<LedgerButton>)>,
    close_query: Query<&Interaction, (Changed<Interaction>, With<LedgerCloseButton>)>,
    mut state: ResMut<LedgerUiState>,
    mut overlay_query: Query<&mut Node, With<LedgerOverlay>>,
) {
    let mut changed = false;
    for interaction in open_query.iter() {
        if *interaction == Interaction::Pressed {
            state.open = !state.open;
            changed = true;
        }
    }
    for interaction in close_query.iter() {
        if *interaction == Interaction::Pressed {
            state.open = false;
            changed = true;
        }
    }
    if !changed {
        return;
    }
    if !state.open {
        state.story_focus = None;
    }
    if let Ok(mut node) = overlay_query.single_mut() {
        node.display = if state.open { Display::Flex } else { Display::None };
    }
}

/// Clicking a dossier or an epitaph row focuses its story feed; clicking
/// the focused row again closes the feed.
pub fn ledger_story_click_system(
    dossier_query: Query<(&Interaction, &LedgerDossierButton), Changed<Interaction>>,
    epitaph_query: Query<(&Interaction, &LedgerEpitaphButton), Changed<Interaction>>,
    mut state: ResMut<LedgerUiState>,
) {
    for (interaction, dossier) in dossier_query.iter() {
        if *interaction == Interaction::Pressed {
            let focus = StoryFocus::Dealer(dossier.dealer_index);
            state.story_focus = (state.story_focus != Some(focus)).then_some(focus);
        }
    }
    for (interaction, epitaph) in epitaph_query.iter() {
        if *interaction == Interaction::Pressed {
            let focus = StoryFocus::Epitaph(epitaph.epitaph_index);
            state.story_focus = (state.story_focus != Some(focus)).then_some(focus);
        }
    }
}

/// Rebuild the ledger body when the save or the ledger state changes
/// (mirrors populate_map_nodes_system's rebuild pattern)
pub fn populate_ledger_system(
    mut commands: Commands,
    save_data: Option<Res<SaveData>>,
    mut state: ResMut<LedgerUiState>,
    body_query: Query<Entity, With<LedgerBody>>,
    children_query: Query<&Children>,
    game_assets: Res<GameAssets>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    let Ok(body) = body_query.single() else {
        return; // overlay only exists on the deck-builder screen
    };

    let is_empty = children_query.get(body).map(|c| c.is_empty()).unwrap_or(true);
    if !save_data.is_changed() && !state.is_changed() && !is_empty {
        return;
    }

    // A focus that went stale (out-of-range index after an empire reset)
    // closes the feed; the mutation re-triggers one settling rebuild
    let focus_valid = match state.story_focus {
        Some(StoryFocus::Dealer(i)) => i < save_data.dealers.len(),
        Some(StoryFocus::Epitaph(i)) => i < save_data.fallen_empires.len(),
        None => true,
    };
    if !focus_valid {
        state.story_focus = None;
    }
    let focus = state.story_focus;

    if let Ok(children) = children_query.get(body) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    commands.entity(body).with_children(|parent| {
        spawn_empire_strip(parent, &save_data);

        parent
            .spawn(Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            })
            .with_children(|panels| {
                spawn_roster_panel(panels, &save_data, &game_assets, focus);
                spawn_board_panel(panels, &save_data, focus);
                spawn_story_panel(panels, &save_data, focus);
            });
    });
}

/// Panel 1: THE EMPIRE - the numbers the epitaph will freeze
fn spawn_empire_strip(parent: &mut ChildSpawnerCommands, save: &SaveData) {
    let s = ledger_view::empire_summary(save);
    let stats: [(String, &str, Color); 6] = [
        (format_cash(s.lifetime_revenue), "LIFETIME REVENUE", theme::SHOP_CREDIT_LINE_TEXT),
        (format_cash(s.cash_on_hand), "CASH ON HAND", Color::WHITE),
        (s.decks_played.to_string(), "DECKS PLAYED", Color::WHITE),
        (s.dealers_hired.to_string(), "DEALERS HIRED", Color::WHITE),
        (s.zones_unlocked.to_string(), "ZONES UNLOCKED", Color::WHITE),
        (s.convictions.to_string(), "CONVICTIONS", theme::ROSTER_STATUS_JAILED),
    ];

    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                padding: UiRect::all(Val::Px(14.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::ROSTER_CARD_BG.with_alpha(0.6)),
            BorderColor::all(theme::ROSTER_CARD_BORDER),
        ))
        .with_children(|strip| {
            for (value, label, color) in stats {
                strip
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(2.0),
                        ..default()
                    })
                    .with_children(|stat| {
                        stat.spawn((
                            Text::new(value),
                            TextFont::from_font_size(24.0),
                            TextColor(color),
                        ));
                        stat.spawn((
                            Text::new(label),
                            TextFont::from_font_size(11.0),
                            TextColor(theme::V2_LABEL),
                        ));
                    });
            }
        });
}

fn panel_frame<'a>(
    panels: &'a mut ChildSpawnerCommands,
    heading: &str,
    width: Val,
) -> bevy::ecs::system::EntityCommands<'a> {
    let mut panel = panels.spawn((
        Node {
            width,
            flex_grow: if width == Val::Auto { 1.0 } else { 0.0 },
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            padding: UiRect::all(Val::Px(14.0)),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            overflow: Overflow::clip_y(),
            ..default()
        },
        BackgroundColor(theme::ROSTER_CARD_BG.with_alpha(0.35)),
        BorderColor::all(theme::ROSTER_CARD_BORDER.with_alpha(0.6)),
    ));
    let heading = heading.to_string();
    panel.with_children(|p| {
        p.spawn((
            Text::new(heading),
            TextFont::from_font_size(12.0),
            TextColor(theme::BUYER_BUBBLE_LABEL),
        ));
    });
    panel
}

/// Panel 2: THE ROSTER - dossiers, kingpin first (dealers[0] invariant)
fn spawn_roster_panel(
    panels: &mut ChildSpawnerCommands,
    save: &SaveData,
    game_assets: &GameAssets,
    focus: Option<StoryFocus>,
) {
    let rows = ledger_view::dossier_rows(save, &game_assets.shop_locations);
    panel_frame(panels, "THE ROSTER", Val::Px(560.0)).with_children(|panel| {
        for row in &rows {
            let focused = focus == Some(StoryFocus::Dealer(row.dealer_index));
            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(2.0),
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(7.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(if row.status_note.is_some() {
                        theme::ROSTER_CARD_BG_JAILED
                    } else {
                        theme::ROSTER_CARD_BG
                    }),
                    BorderColor::all(if focused {
                        theme::ROSTER_CARD_BORDER_ACTIVE
                    } else {
                        theme::ROSTER_CARD_BORDER
                    }),
                    LedgerDossierButton { dealer_index: row.dealer_index },
                ))
                .with_children(|dossier| {
                    dossier
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        })
                        .with_children(|top| {
                            top.spawn((
                                Text::new(if row.is_kingpin {
                                    format!("{} · BOSS", row.name)
                                } else {
                                    row.name.clone()
                                }),
                                TextFont::from_font_size(15.0),
                                TextColor(if row.is_kingpin {
                                    theme::ROSTER_KINGPIN_BADGE
                                } else {
                                    Color::WHITE
                                }),
                            ));
                            let (r, g, b) = row.tier_color;
                            top.spawn((
                                Text::new(format!("HEAT {} [{}]", row.heat, row.tier_name)),
                                TextFont::from_font_size(12.0),
                                TextColor(Color::srgb(r, g, b)),
                            ));
                        });
                    dossier.spawn((
                        Text::new(format!(
                            "{} · {} DEALS · {} DECKS · {} PRIORS · {} STORIES",
                            row.station.to_uppercase(),
                            row.deals_closed,
                            row.decks_played,
                            row.priors,
                            row.story_count
                        )),
                        TextFont::from_font_size(11.0),
                        TextColor(theme::ROSTER_STATION_TEXT),
                    ));
                    if !row.cred_line.is_empty() {
                        dossier.spawn((
                            Text::new(format!("CRED — {}", row.cred_line)),
                            TextFont::from_font_size(11.0),
                            TextColor(theme::SHOP_CREDIT_LINE_TEXT),
                        ));
                    }
                    if let Some(note) = &row.status_note {
                        dossier.spawn((
                            Text::new(note.as_str()),
                            TextFont::from_font_size(11.0),
                            TextColor(theme::ROSTER_STATUS_JAILED),
                        ));
                    }
                });
        }
    });
}

/// Panel 3: FALLEN EMPIRES - the arcade board with the living empire
/// slotted unranked at its would-be position
fn spawn_board_panel(
    panels: &mut ChildSpawnerCommands,
    save: &SaveData,
    focus: Option<StoryFocus>,
) {
    let rows = ledger_view::board_rows(save);
    panel_frame(panels, "FALLEN EMPIRES", Val::Px(560.0)).with_children(|panel| {
        if save.fallen_empires.is_empty() {
            panel.spawn((
                Text::new("no empires have fallen — yet"),
                TextFont::from_font_size(13.0),
                TextColor(theme::V2_LABEL),
            ));
        }
        for row in &rows {
            let title = match row.rank {
                Some(rank) => format!("{}. {}", rank, format_cash(row.lifetime_revenue)),
                None => format!("— {} · IN PROGRESS", format_cash(row.lifetime_revenue)),
            };
            let detail = format!(
                "{} DECKS · {} HIRES · {} CONVICTIONS",
                row.decks_played, row.dealers_hired, row.convictions
            );
            let mut entity = panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(7.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(theme::ROSTER_CARD_BG),
                BorderColor::all(if row.is_current {
                    theme::LEDGER_BOARD_CURRENT
                } else if row.epitaph_index.is_some_and(|i| focus == Some(StoryFocus::Epitaph(i))) {
                    theme::ROSTER_CARD_BORDER_ACTIVE
                } else {
                    theme::ROSTER_CARD_BORDER
                }),
            ));
            // Only the dead have archives to open
            if let Some(epitaph_index) = row.epitaph_index {
                entity.insert((Button, LedgerEpitaphButton { epitaph_index }));
            }
            entity.with_children(|board_row| {
                board_row.spawn((
                    Text::new(title),
                    TextFont::from_font_size(15.0),
                    TextColor(if row.is_current {
                        theme::LEDGER_BOARD_CURRENT
                    } else {
                        Color::WHITE
                    }),
                ));
                board_row.spawn((
                    Text::new(detail),
                    TextFont::from_font_size(11.0),
                    TextColor(theme::ROSTER_STATION_TEXT),
                ));
            });
        }
    });
}

/// The story feed for whatever record is focused
fn spawn_story_panel(
    panels: &mut ChildSpawnerCommands,
    save: &SaveData,
    focus: Option<StoryFocus>,
) {
    let (heading, stories) = match focus {
        Some(StoryFocus::Dealer(i)) => {
            let name = save
                .dealers
                .get(i)
                .map(|d| d.name.to_uppercase())
                .unwrap_or_default();
            (format!("STORIES — {name}"), ledger_view::dealer_stories(save, i))
        }
        Some(StoryFocus::Epitaph(i)) => {
            let revenue = save
                .fallen_empires
                .get(i)
                .map(|e| format_cash(e.lifetime_revenue))
                .unwrap_or_default();
            (
                format!("STORIES — FALLEN EMPIRE ({revenue})"),
                ledger_view::epitaph_stories(save, i),
            )
        }
        None => ("STORIES".to_string(), Vec::new()),
    };

    panel_frame(panels, &heading, Val::Auto).with_children(|panel| {
        if focus.is_none() {
            panel.spawn((
                Text::new("click a dossier or a fallen empire to read its record"),
                TextFont::from_font_size(13.0),
                TextColor(theme::V2_LABEL),
            ));
            return;
        }
        if stories.is_empty() {
            panel.spawn((
                Text::new("nothing on the record yet"),
                TextFont::from_font_size(13.0),
                TextColor(theme::V2_LABEL),
            ));
            return;
        }
        for story in stories.iter().take(STORY_FEED_CAP) {
            panel.spawn((
                Text::new(story.as_str()),
                TextFont::from_font_size(13.0),
                TextColor(theme::LEDGER_STORY_TEXT),
            ));
        }
        let hidden = stories.len().saturating_sub(STORY_FEED_CAP);
        if hidden > 0 {
            panel.spawn((
                Text::new(format!("… {hidden} earlier stories")),
                TextFont::from_font_size(12.0),
                TextColor(theme::V2_LABEL),
            ));
        }
    });
}
