// SOW-032: Tutorial Arc systems - orchestration only. Beat detection and copy
// live in ui::tutorial_view (pure, unit-tested without ECS); this file wires
// the offer buttons, latches the cursor on save change, paints the goal strip,
// and spawns the offer overlay. Same self-contained shape as the ledger/front
// systems: reads SaveData, writes only the strip/overlay entities + the save.

use bevy::prelude::*;

use crate::save::{SaveData, SaveManager, TutorialStatus};
use crate::ui::components::*;
use crate::ui::theme;
use crate::ui::tutorial_view;

/// SOW-032: spawn the one-time guided-start offer. Full-screen Block overlay
/// (GUIDANCE lesson 1: a live-hub overlay must Block, or canvas clicks fall
/// through to the hub buttons beneath). Spawned by setup_deck_builder ONLY while
/// status == Offered; tutorial_offer_button_system despawns it on the decision.
/// Child of DeckBuilderRoot so it inherits the 1920x1080 scaling and the
/// on-exit cleanup for free.
pub fn spawn_tutorial_offer_overlay(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.03, 0.06, 0.92)),
            bevy::ui::FocusPolicy::Block,
            GlobalZIndex(95),
            TutorialOfferOverlay,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(18.0),
                        padding: UiRect::all(Val::Px(32.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(14.0)),
                        ..default()
                    },
                    BackgroundColor(theme::UI_ROOT_BG),
                    BorderColor::all(theme::LEDGER_TAB_BG),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new(tutorial_view::OFFER_TITLE),
                        TextFont::from_font_size(26.0),
                        TextColor(theme::TEXT_HEADER),
                    ));
                    panel.spawn((
                        Text::new(tutorial_view::OFFER_BODY),
                        TextFont::from_font_size(15.0),
                        TextColor(theme::TEXT_SECONDARY),
                        TextLayout::new_with_justify(bevy::text::Justify::Center),
                        Node {
                            max_width: Val::Px(520.0),
                            ..default()
                        },
                    ));
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(16.0),
                            margin: UiRect::top(Val::Px(6.0)),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Button,
                                Node {
                                    width: Val::Px(240.0),
                                    height: Val::Px(56.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(theme::CONTINUE_BUTTON_BG),
                                TutorialAcceptButton,
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new(tutorial_view::OFFER_ACCEPT),
                                    TextFont::from_font_size(18.0),
                                    TextColor(Color::WHITE),
                                ));
                            });
                            row.spawn((
                                Button,
                                Node {
                                    width: Val::Px(240.0),
                                    height: Val::Px(56.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(theme::BUTTON_NEUTRAL_BG),
                                TutorialDeclineButton,
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new(tutorial_view::OFFER_DECLINE),
                                    TextFont::from_font_size(18.0),
                                    TextColor(Color::WHITE),
                                ));
                            });
                        });
                });
        });
}

/// The offer overlay's ACCEPT / DECLINE and the strip's SKIP THE LESSONS.
/// ACCEPT takes the guided start; DECLINE (offer) and DISMISS (mid-arc) both
/// retire the arc to Declined - always free, never a benefit. Saves on change
/// like roster_button_system; despawns the offer overlay once a decision lands.
pub fn tutorial_offer_button_system(
    accept_query: Query<&Interaction, (Changed<Interaction>, With<TutorialAcceptButton>)>,
    decline_query: Query<&Interaction, (Changed<Interaction>, With<TutorialDeclineButton>)>,
    dismiss_query: Query<&Interaction, (Changed<Interaction>, With<GoalStripDismissButton>)>,
    save_data: Option<ResMut<SaveData>>,
    save_manager: Option<Res<SaveManager>>,
    overlay_query: Query<Entity, With<TutorialOfferOverlay>>,
    mut commands: Commands,
) {
    let (Some(mut save_data), Some(save_manager)) = (save_data, save_manager) else {
        return;
    };

    let mut dirty = false;
    let mut decided = false;

    for interaction in accept_query.iter() {
        if *interaction == Interaction::Pressed
            && save_data.tutorial.status == TutorialStatus::Offered
        {
            save_data.tutorial.status = TutorialStatus::Accepted;
            dirty = true;
            decided = true;
        }
    }
    for interaction in decline_query.iter() {
        if *interaction == Interaction::Pressed
            && save_data.tutorial.status == TutorialStatus::Offered
        {
            save_data.tutorial.status = TutorialStatus::Declined;
            dirty = true;
            decided = true;
        }
    }
    // DISMISS retires a LIVE (Accepted) arc mid-stream. Never touches an
    // already-decided run, so a stray hidden-strip click is inert.
    for interaction in dismiss_query.iter() {
        if *interaction == Interaction::Pressed
            && save_data.tutorial.status == TutorialStatus::Accepted
        {
            save_data.tutorial.status = TutorialStatus::Declined;
            dirty = true;
        }
    }

    if decided {
        for entity in overlay_query.iter() {
            commands.entity(entity).despawn();
        }
    }
    if dirty {
        if let Err(e) = save_manager.save(&save_data) {
            bevy::log::warn!("Failed to save tutorial choice: {:?}", e);
        }
    }
}

/// Latch the cursor forward whenever the save changes (mirrors
/// populate_map_nodes_system's change-gate). advance is pure; clone-advance-
/// compare keeps the &SaveData read from overlapping the &mut tutorial write,
/// and we persist only when the tutorial actually moved. Writing back marks
/// SaveData changed, but next frame's advance produces no delta, so it settles.
pub fn tutorial_progress_system(
    save_data: Option<ResMut<SaveData>>,
    save_manager: Option<Res<SaveManager>>,
) {
    let (Some(mut save_data), Some(save_manager)) = (save_data, save_manager) else {
        return;
    };
    if !save_data.is_changed() {
        return;
    }
    let mut next = save_data.tutorial.clone();
    next.advance(&save_data);
    if next != save_data.tutorial {
        save_data.tutorial = next;
        if let Err(e) = save_manager.save(&save_data) {
            bevy::log::warn!("Failed to save tutorial progress: {:?}", e);
        }
    }
}

/// Paint the goal strip: text = current beat line + hint (or the graduation
/// send-off), visibility follows the derived view (hidden while Offered or
/// Declined, the closing line while Graduated). Queries only - on the
/// pending-upgrade early-return frame the strip entity is absent and this
/// no-ops (never a bare ResMut on a missing resource - GUIDANCE lesson 2).
pub fn populate_goal_strip_system(
    save_data: Option<Res<SaveData>>,
    mut strip_query: Query<&mut Node, (With<GoalStrip>, Without<GoalStripDismissButton>)>,
    mut dismiss_query: Query<&mut Node, (With<GoalStripDismissButton>, Without<GoalStrip>)>,
    mut text_query: Query<&mut Text, With<GoalStripText>>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    let view = tutorial_view::derive_view(&save_data, &save_data.tutorial);

    // Hide the whole strip when there is nothing to say (Offered / Declined);
    // show it for a live beat or the graduation send-off.
    if let Ok(mut node) = strip_query.single_mut() {
        let display = if view.line.is_empty() {
            Display::None
        } else {
            Display::Flex
        };
        if node.display != display {
            node.display = display;
        }
    }

    // A retired arc (graduated) shows its closing line but offers nothing to
    // skip - the dismiss button goes away.
    if let Ok(mut node) = dismiss_query.single_mut() {
        let display = if view.retired {
            Display::None
        } else {
            Display::Flex
        };
        if node.display != display {
            node.display = display;
        }
    }

    if let Ok(mut text) = text_query.single_mut() {
        let content = if view.hint.is_empty() {
            view.line.clone()
        } else {
            format!("{}\n{}", view.line, view.hint)
        };
        if **text != content {
            **text = content;
        }
    }
}
