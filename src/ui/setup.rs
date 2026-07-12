// SOW-AAA: UI setup functions
// Extracted from main.rs
// Updated for Bevy 0.18

use bevy::prelude::*;
use super::theme;
use super::components::*;
use crate::models::fonts::EmojiFont;

pub fn setup_deck_builder(
    mut commands: Commands,
    save_data: Option<Res<crate::save::SaveData>>,
    emoji_font: Res<EmojiFont>,
    deferred: Res<crate::systems::UpgradeChoiceDeferred>,
) {
    // RFC-019: Don't spawn DeckBuilder UI if we're about to redirect to UpgradeChoice
    // SOW-021: unless the player chose DECIDE LATER - then the deck builder MUST
    // spawn or DeckBuilding becomes an empty screen (soft-lock)
    if let Some(ref data) = save_data {
        {
            let character = data.active_character(); // RFC-023: active dealer
            if character.has_pending_upgrades() && !deferred.0 {
                return;
            }
        }
    }

    // SOW-020: Initialize shop state resource
    commands.insert_resource(crate::systems::ShopState::new());

    // Deck builder root container - 1920x1080 design, will be scaled/positioned by scale_ui_to_fit_system
    commands.spawn((
        Node {
            width: Val::Px(1920.0),
            height: Val::Px(1080.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(theme::DECK_BUILDER_BG),
        DeckBuilderRoot,
    ))
    .with_children(|parent| {
        // SOW-020: Tab row (Your Cards / Shop)
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            ShopTabsContainer,
        ))
        .with_children(|tabs| {
            // "Your Cards" tab (default active)
            tabs.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme::CONTINUE_BUTTON_BG), // Active by default
                ShopTab { is_shop: false },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("YOUR CARDS"),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::WHITE),
                ));
            });

            // "Shop" tab
            tabs.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme::BUTTON_NEUTRAL_BG), // Inactive by default
                ShopTab { is_shop: true },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("SHOP"),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::WHITE),
                ));
            });

            // SOW-020: Shop location selector (only visible when shop tab active)
            // SOW-020: Get unlocked locations from save data
            let unlocked_locations = save_data
                .as_ref()
                .map(|d| &d.account.unlocked_locations)
                .cloned()
                .unwrap_or_else(|| std::collections::HashSet::from(["the_corner".to_string()]));

            tabs.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::left(Val::Px(20.0)),
                    display: Display::None, // Hidden initially
                    ..default()
                },
                ShopLocationSelector,
            ))
            .with_children(|locs| {
                // Shop location definitions
                let locations = [
                    ("the_corner", "The Corner"),
                    ("the_block", "The Block"),
                ];

                for (idx, (id, name)) in locations.iter().enumerate() {
                    // Only spawn button if location is unlocked
                    if !unlocked_locations.contains(*id) {
                        continue;
                    }

                    let is_first = idx == 0 || locations[..idx].iter().all(|(loc_id, _)| !unlocked_locations.contains(*loc_id));
                    let bg = if is_first {
                        theme::CONTINUE_BUTTON_BG // Active by default
                    } else {
                        theme::BUTTON_NEUTRAL_BG
                    };

                    locs.spawn((
                        Button,
                        Node {
                            width: Val::Px(140.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(bg),
                        ShopLocationButton { location_id: id.to_string() },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(*name),
                            TextFont::from_font_size(14.0),
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });
        });

        // SOW-023: Operations roster - who's on the payroll, who's in jail,
        // who runs next. Children rebuilt by populate_roster_panel_system.
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                align_items: AlignItems::Stretch,
                ..default()
            },
            RosterPanel,
        ));

        // Card pool container - scrollable, fills available space (YOUR CARDS view)
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0, // Fill available vertical space
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(5.0),
                column_gap: Val::Px(5.0),
                align_content: AlignContent::FlexStart,
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(theme::CARD_POOL_BG),
            Interaction::None, // Required for hover detection
            ScrollPosition::default(), // Bevy 0.18: Required for scrolling
            CardPoolContainer,
        ))
        .with_children(|parent| {
            parent.spawn(Node {
                width: Val::Percent(100.0), // Full width forces cards to wrap to next row
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Click cards to toggle selection (Green = Selected)"),
                    TextFont::from_font_size(20.0),
                    TextColor(Color::WHITE),
                ));
            });
        });

        // SOW-020: Shop cards container (hidden by default, shown when SHOP tab active)
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(5.0),
                column_gap: Val::Px(5.0),
                align_content: AlignContent::FlexStart,
                overflow: Overflow::scroll_y(),
                display: Display::None, // Hidden by default
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.12, 0.1)), // Slightly different bg for shop
            Interaction::None,
            ScrollPosition::default(),
            ShopCardsContainer,
        ));

        // Bottom: Stats, Heat display, and START RUN button
        parent.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::top(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            // Left side: Deck stats and character heat
            parent.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|parent| {
                // Deck stats
                parent.spawn((
                    Text::new("Deck: 20/20 cards"),
                    TextFont::from_font_size(24.0),
                    TextColor(Color::WHITE),
                    DeckStatsDisplay,
                ));

                // SOW-023: the "Heat: N [Tier]" line is gone - per-dealer heat
                // lives on the roster panel now (Reed: it duplicated the panel)

                // Decay info (hidden by default; the only decay surface)
                parent.spawn((
                    Text::new(""),
                    TextFont::from_font_size(18.0),
                    TextColor(Color::srgb(0.5, 0.8, 1.0)),
                    Visibility::Hidden,
                    DecayInfoDisplay,
                ));

                // RFC-016: Account cash display
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Cash: $0"),
                        TextFont::from_font_size(20.0),
                        TextColor(Color::srgb(0.3, 0.9, 0.3)), // Green for cash
                        AccountCashText,
                    ));
                });

                parent.spawn((
                    Text::new("Lifetime: $0"),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::srgb(0.6, 0.6, 0.6)), // Grey for lifetime
                    LifetimeRevenueText,
                ));
            });

            // Right side: Story History button + START RUN button
            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|parent| {
                // Story History button
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(60.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                    StoryHistoryButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("📖"),
                        TextFont {
                            font: emoji_font.0.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                // START RUN button
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(theme::CONTINUE_BUTTON_BG),
                    StartRunButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START RUN"),
                        TextFont::from_font_size(24.0),
                        TextColor(Color::WHITE),
                    ));
                });
            });
        });
    });

    // Story History Overlay (initially hidden)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            display: Display::None,
            ..default()
        },
        GlobalZIndex(100),
        StoryHistoryOverlay,
    ))
    .with_children(|parent| {
        // Semi-transparent backdrop
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ));

        // Story panel
        parent.spawn((
            Node {
                width: Val::Px(700.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            BorderColor::all(Color::srgb(0.4, 0.4, 0.5)),
            GlobalZIndex(101),
        ))
        .with_children(|parent| {
            // Header row with title and close button
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            })
            .with_children(|parent| {
                // Title with emoji
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("📖"),
                        TextFont {
                            font: emoji_font.0.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    parent.spawn((
                        Text::new("Story History"),
                        TextFont::from_font_size(28.0),
                        TextColor(Color::WHITE),
                    ));
                });

                // Close button
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.2, 0.2)),
                    StoryHistoryCloseButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("✕"),
                        TextFont::from_font_size(24.0),
                        TextColor(Color::WHITE),
                    ));
                });
            });

            // Scrollable story content
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                ScrollPosition::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("No stories yet..."),
                    TextFont::from_font_size(14.0),
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    StoryHistoryText,
                ));
            });
        });
    });
}

// ============================================================================
// SOW-022: Game Play v2 screen
// ============================================================================
// Layout transcribed from the "Game Play v2" design mockup (1920x1080).
// Static composition only - all live values are bound by update systems.

/// Spawn a standalone emoji glyph Text (NotoEmoji handle - see font rules in
/// GUIDANCE: emoji never share a Text with words)
fn spawn_emoji(parent: &mut ChildSpawnerCommands, glyph: &str, size: f32, color: Color, emoji_font: &EmojiFont) {
    parent.spawn((
        Text::new(glyph),
        TextFont {
            font: emoji_font.0.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
    ));
}

/// Radial spotlight gradient behind an actor portrait
pub fn spotlight_gradient(color: Color) -> BackgroundGradient {
    BackgroundGradient(vec![Gradient::Radial(RadialGradient {
        position: UiPosition::CENTER,
        shape: RadialGradientShape::FarthestSide,
        stops: vec![
            ColorStop::new(color, Val::Percent(0.0)),
            ColorStop::new(color.with_alpha(0.0), Val::Percent(70.0)),
        ],
        color_space: default(),
    })])
}

/// PASS button face (enabled/disabled variants swapped by input system)
pub fn pass_button_gradient(enabled: bool) -> BackgroundGradient {
    let (top, bottom) = if enabled {
        (theme::PASS_BUTTON_TOP, theme::PASS_BUTTON_BOTTOM)
    } else {
        (theme::PASS_BUTTON_DISABLED_TOP, theme::PASS_BUTTON_DISABLED_BOTTOM)
    };
    BackgroundGradient(vec![Gradient::Linear(LinearGradient::new(
        LinearGradient::TO_BOTTOM,
        vec![
            ColorStop::new(top, Val::Percent(0.0)),
            ColorStop::new(bottom, Val::Percent(100.0)),
        ],
    ))])
}

pub fn create_ui(commands: &mut Commands, emoji_font: &EmojiFont) {
    // Background image layer (behind everything)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow::clip(),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(-1),
        BackgroundColor(theme::GAMEPLAY_CANVAS_BG),
        BackgroundImage,
    ))
    .with_children(|parent| {
        parent.spawn((
            Node::default(),
            ImageNode::default(),
            BackgroundImageNode,
        ));
    });

    // UI Root container - 1920x1080 design, scaled/positioned by scale_ui_to_fit_system
    commands.spawn((
        Node {
            width: Val::Px(1920.0),
            height: Val::Px(1080.0),
            ..default()
        },
        UiRoot,
    ))
    .with_children(|parent| {
        // ====================================================================
        // Vignette over the location background
        // ====================================================================
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundGradient(vec![Gradient::Radial(RadialGradient {
                position: UiPosition::anchor(Vec2::new(0.0, -0.18)), // 50% 32%
                shape: RadialGradientShape::Ellipse(Val::Percent(120.0), Val::Percent(90.0)),
                stops: vec![
                    ColorStop::new(theme::VIGNETTE_INNER, Val::Percent(0.0)),
                    ColorStop::new(theme::VIGNETTE_MID, Val::Percent(62.0)),
                    ColorStop::new(theme::VIGNETTE_OUTER, Val::Percent(100.0)),
                ],
                color_space: default(),
            })]),
            ScreenVignette,
        ));

        // ====================================================================
        // YOUR STANDING panel (left-mid: cash + heat)
        // ====================================================================
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(36.0),
                top: Val::Px(452.0),
                width: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(14.0)),
                row_gap: Val::Px(13.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            BackgroundColor(theme::STANDING_PANEL_BG),
            BorderColor::all(theme::STANDING_PANEL_BORDER),
            BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.45), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(24.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("YOUR STANDING"),
                TextFont::from_font_size(11.0),
                TextColor(theme::V2_LABEL),
            ));

            // Cash row
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_emoji(parent, "💵", 13.0, theme::STANDING_CASH_LABEL, emoji_font);
                    parent.spawn((
                        Text::new("CASH"),
                        TextFont::from_font_size(13.0),
                        TextColor(theme::STANDING_CASH_LABEL),
                    ));
                });
                parent.spawn((
                    Text::new("$0"),
                    TextFont::from_font_size(24.0),
                    TextColor(theme::STANDING_CASH_VALUE),
                    StandingCashText,
                ));
            });

            // Divider
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    ..default()
                },
                BackgroundColor(theme::STANDING_DIVIDER),
            ));

            // Heat block
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|parent| {
                // Header: "HEAT" label | tier chip + value
                parent.spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(6.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        spawn_emoji(parent, "🔥", 13.0, theme::STANDING_HEAT_LABEL, emoji_font);
                        parent.spawn((
                            Text::new("HEAT"),
                            TextFont::from_font_size(13.0),
                            TextColor(theme::STANDING_HEAT_LABEL),
                        ));
                    });
                    parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(9.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Heat tier chip (text/border colored by tier at runtime)
                        parent.spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(2.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::MAX,
                                ..default()
                            },
                            BorderColor::all(theme::STANDING_HEAT_VALUE),
                            StandingHeatTierChip,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("COLD"),
                                TextFont::from_font_size(10.0),
                                TextColor(theme::STANDING_HEAT_VALUE),
                                StandingHeatTierText,
                            ));
                        });
                        parent.spawn((
                            Text::new("0"),
                            TextFont::from_font_size(22.0),
                            TextColor(theme::STANDING_HEAT_VALUE),
                            StandingHeatValueText,
                        ));
                        parent.spawn((
                            Text::new("/ 100"),
                            TextFont::from_font_size(12.0),
                            TextColor(theme::STANDING_HEAT_VALUE_DIM),
                        ));
                    });
                });

                // Heat track (fixed 0..100 scale)
                parent.spawn((
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Percent(100.0),
                        height: Val::Px(14.0),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(7.0)),
                        ..default()
                    },
                    BackgroundColor(theme::STANDING_HEAT_TRACK_BG),
                    BorderColor::all(theme::STANDING_HEAT_TRACK_BORDER),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            width: Val::Percent(0.0),
                            border_radius: BorderRadius::all(Val::Px(7.0)),
                            ..default()
                        },
                        BackgroundGradient(vec![Gradient::Linear(LinearGradient::new(
                            LinearGradient::TO_RIGHT,
                            vec![
                                ColorStop::new(theme::STANDING_HEAT_FILL_LOW, Val::Percent(0.0)),
                                ColorStop::new(theme::STANDING_HEAT_FILL_HIGH, Val::Percent(100.0)),
                            ],
                        ))]),
                        StandingHeatBarFill,
                    ));
                    // Conviction-threshold tick marks (children rebuilt at runtime)
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        StandingHeatTicks,
                    ));
                });

                // Tick labels row (children rebuilt at runtime)
                parent.spawn((
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Percent(100.0),
                        height: Val::Px(13.0),
                        margin: UiRect::top(Val::Px(3.0)),
                        ..default()
                    },
                    StandingHeatTickLabels,
                ));
            });
        });

        // ====================================================================
        // Turn indicator (top center): round header + actor pill
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(34.0),
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("ROUND 1 / 3  ·  DEAL IN PROGRESS"),
                TextFont::from_font_size(12.0),
                TextColor(theme::V2_LABEL),
                TurnIndicatorText,
            ));
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    padding: UiRect::axes(Val::Px(22.0), Val::Px(8.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BackgroundColor(theme::PILL_NEUTRAL_BG),
                BorderColor::all(theme::PILL_NEUTRAL_BORDER),
                BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.35), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(26.0)),
                TurnPill,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        width: Val::Px(9.0),
                        height: Val::Px(9.0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(theme::PILL_NEUTRAL_DOT),
                    TurnPillDot,
                ));
                parent.spawn((
                    Text::new("DEALING..."),
                    TextFont::from_font_size(16.0),
                    TextColor(theme::PILL_NEUTRAL_TEXT),
                    TurnPillText,
                ));
            });
        });

        // ====================================================================
        // Narc character cluster (top-left)
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(150.0),
            top: Val::Px(120.0),
            width: Val::Px(360.0),
            height: Val::Px(300.0),
            ..default()
        })
        .with_children(|parent| {
            // Turn spotlight behind portrait
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(70.0),
                    width: Val::Px(230.0),
                    height: Val::Px(230.0),
                    ..default()
                },
                spotlight_gradient(theme::NARC_SPOTLIGHT),
                NarcSpotlight,
            ));

            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(6.0),
                    top: Val::Px(50.0),
                    width: Val::Px(240.0),
                    height: Val::Px(240.0),
                    ..default()
                },
                ImageNode::default(),
                NarcPortrait,
            ));

            // Name plate (count chip removed - it added little over the intent bubble)
            parent.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(14.0),
                top: Val::Px(274.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("NARC"),
                    TextFont::from_font_size(14.0),
                    TextColor(theme::NARC_NAME),
                ));
            });

            // Intent bubble (hidden until the narc telegraphs)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(150.0),
                    top: Val::Px(2.0),
                    width: Val::Px(250.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(theme::NARC_BUBBLE_BG),
                BorderColor::all(theme::NARC_BUBBLE_BORDER),
                BoxShadow::new(Color::srgba(0.784, 0.196, 0.196, 0.3), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(22.0)),
                NarcIntentBubble,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("INTENT"),
                    TextFont::from_font_size(11.0),
                    TextColor(theme::NARC_BUBBLE_TITLE),
                    NarcIntentTitleText,
                ));
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(14.0),
                        ..default()
                    },
                    NarcIntentStatsRow,
                ));
                // Speech tail
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(44.0),
                        bottom: Val::Px(-8.0),
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(theme::NARC_BUBBLE_BG),
                    UiTransform::from_rotation(Rot2::degrees(45.0)),
                ));
            });
        });

        // ====================================================================
        // Buyer character cluster (top-right)
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(150.0),
            top: Val::Px(120.0),
            width: Val::Px(360.0),
            height: Val::Px(300.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    top: Val::Px(70.0),
                    width: Val::Px(230.0),
                    height: Val::Px(230.0),
                    ..default()
                },
                spotlight_gradient(theme::BUYER_SPOTLIGHT),
                BuyerSpotlight,
            ));

            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(6.0),
                    top: Val::Px(50.0),
                    width: Val::Px(240.0),
                    height: Val::Px(240.0),
                    ..default()
                },
                ImageNode::default(),
                BuyerPortrait,
            ));

            // Name plate
            parent.spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(14.0),
                top: Val::Px(274.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("BUYER"),
                    TextFont::from_font_size(14.0),
                    TextColor(theme::BUYER_NAME),
                    BuyerNameText,
                ));
            });

            // Speech bubble: "PLAYED · <card>" - the buyer's ACTIONS, symmetric
            // with the narc's intent bubble (hidden until the buyer reacts)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(150.0),
                    top: Val::Px(2.0),
                    width: Val::Px(250.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    row_gap: Val::Px(4.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(theme::BUYER_BUBBLE_BG),
                BorderColor::all(theme::BUYER_BUBBLE_BORDER),
                BoxShadow::new(Color::srgba(0.824, 0.745, 0.196, 0.28), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(22.0)),
                BuyerPlayedBubble,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("PLAYED"),
                    TextFont::from_font_size(11.0),
                    TextColor(theme::BUYER_BUBBLE_TITLE),
                    BuyerPlayedTitleText,
                ));
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(14.0),
                        ..default()
                    },
                    BuyerPlayedStatsRow,
                ));
                // Speech tail
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(44.0),
                        bottom: Val::Px(-8.0),
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(theme::BUYER_BUBBLE_BG),
                    UiTransform::from_rotation(Rot2::degrees(45.0)),
                ));
            });

            // Scenario placard: standing WANTS info under the name plate
            // (hover for detail; confidence face replaces the heat-cap chip)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(14.0),
                    top: Val::Px(306.0),
                    width: Val::Px(258.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Val::Px(15.0), Val::Px(11.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(theme::BUYER_BUBBLE_BG),
                BorderColor::all(theme::BUYER_BUBBLE_BORDER),
                BoxShadow::new(Color::srgba(0.824, 0.745, 0.196, 0.28), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(22.0)),
                Interaction::None,
                BuyerBubble,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("WANTS"),
                    TextFont::from_font_size(11.0),
                    TextColor(theme::BUYER_BUBBLE_TITLE),
                    TextLayout::new_with_justify(bevy::text::Justify::Right),
                    Node {
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::bottom(Val::Px(7.0)),
                        ..default()
                    },
                    BuyerScenarioNameText,
                ));
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    column_gap: Val::Px(5.0),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("DEMAND"),
                        TextFont::from_font_size(13.0),
                        TextColor(theme::BUYER_BUBBLE_LABEL),
                    ));
                    parent.spawn((
                        Text::new("—"),
                        TextFont::from_font_size(13.0),
                        TextColor(theme::BUYER_BUBBLE_DEMAND),
                        BuyerDemandText,
                    ));
                });
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    column_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PAYOUT"),
                        TextFont::from_font_size(13.0),
                        TextColor(theme::BUYER_BUBBLE_LABEL),
                    ));
                    parent.spawn((
                        Text::new("×1.0"),
                        TextFont::from_font_size(13.0),
                        TextColor(theme::BUYER_BUBBLE_PAYOUT),
                        BuyerPayoutText,
                    ));
                });
                // Confidence row: how close the buyer is to bailing
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("🙂"),
                        TextFont {
                            font: emoji_font.0.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme::SAFE_CHIP_TEXT),
                        BuyerConfidenceEmoji,
                    ));
                    parent.spawn((
                        Text::new("CONFIDENT"),
                        TextFont::from_font_size(12.0),
                        TextColor(theme::SAFE_CHIP_TEXT),
                        BuyerConfidenceText,
                    ));
                });
                parent.spawn((
                    Text::new("▾ HOVER FOR DETAIL"),
                    TextFont::from_font_size(9.0),
                    TextColor(theme::BUYER_BUBBLE_HINT),
                    TextLayout::new_with_justify(bevy::text::Justify::Right),
                    Node {
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::top(Val::Px(8.0)),
                        padding: UiRect::top(Val::Px(7.0)),
                        border: UiRect::top(Val::Px(1.0)),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    BorderColor::all(theme::BUYER_BUBBLE_DIVIDER),
                ));
                // Hover detail (scenario description, preferred locations)
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::top(Val::Px(8.0)),
                        display: Display::None,
                        ..default()
                    },
                    BuyerDetailPanel,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        TextFont::from_font_size(12.0),
                        TextColor(theme::TEXT_SECONDARY),
                        TextLayout::new_with_justify(bevy::text::Justify::Right),
                        BuyerDetailText,
                    ));
                });
                // (no speech tail - the placard is standing info, not speech)
            });
        });

        // ====================================================================
        // THE DEAL ON THE TABLE (center: active slots + ghost insurance)
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(398.0),
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(4.0)),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BackgroundColor(theme::V2_SCRIM_BG),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("THE DEAL ON THE TABLE"),
                    TextFont::from_font_size(11.0),
                    TextColor(theme::V2_LABEL),
                ));
            });
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                ActiveSlotsContainer,
            ))
            .with_children(|parent| {
                for slot_type in [SlotType::Location, SlotType::Product, SlotType::Conviction, SlotType::Insurance] {
                    parent.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ActiveSlot { slot_type },
                    ));
                }
            });
        });

        // ====================================================================
        // EVIDENCE vs COVER balance bar
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(684.0),
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(664.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(theme::V2_SCRIM_BG),
            ))
            .with_children(|parent| {
                // Header: evidence | chips | cover
                parent.spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(6.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        spawn_emoji(parent, "🔍", 14.0, theme::BALANCE_EVIDENCE_TEXT, emoji_font);
                        parent.spawn((
                            Text::new("EVIDENCE 0"),
                            TextFont::from_font_size(14.0),
                            TextColor(theme::BALANCE_EVIDENCE_TEXT),
                            BalanceEvidenceText,
                        ));
                    });
                    parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(2.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::MAX,
                                ..default()
                            },
                            BackgroundColor(theme::SAFE_CHIP_BG),
                            BorderColor::all(theme::SAFE_CHIP_BORDER),
                            BalanceStatusChip,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("SAFE"),
                                TextFont::from_font_size(12.0),
                                TextColor(theme::SAFE_CHIP_TEXT),
                                BalanceStatusChipText,
                            ));
                        });
                        parent.spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(2.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::MAX,
                                ..default()
                            },
                            BackgroundColor(theme::PAYOUT_CHIP_BG),
                            BorderColor::all(theme::PAYOUT_CHIP_BORDER),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("PAYOUT ×1.0"),
                                TextFont::from_font_size(12.0),
                                TextColor(theme::PAYOUT_CHIP_TEXT),
                                BalancePayoutChipText,
                            ));
                        });
                    });
                    parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(6.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("COVER 0"),
                            TextFont::from_font_size(14.0),
                            TextColor(theme::BALANCE_COVER_TEXT),
                            BalanceCoverText,
                        ));
                        spawn_emoji(parent, "🛡", 14.0, theme::BALANCE_COVER_TEXT, emoji_font);
                    });
                });

                // Track
                parent.spawn((
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Percent(100.0),
                        height: Val::Px(12.0),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(theme::BALANCE_TRACK_BG),
                    BorderColor::all(theme::BALANCE_TRACK_BORDER),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            width: Val::Percent(50.0),
                            ..default()
                        },
                        BackgroundGradient(vec![Gradient::Linear(LinearGradient::new(
                            LinearGradient::TO_RIGHT,
                            vec![
                                ColorStop::new(theme::BALANCE_EVIDENCE_FILL_LOW, Val::Percent(0.0)),
                                ColorStop::new(theme::BALANCE_EVIDENCE_FILL_HIGH, Val::Percent(100.0)),
                            ],
                        ))]),
                        BalanceEvidenceFill,
                    ));
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            width: Val::Percent(50.0),
                            ..default()
                        },
                        BackgroundGradient(vec![Gradient::Linear(LinearGradient::new(
                            LinearGradient::TO_RIGHT,
                            vec![
                                ColorStop::new(theme::BALANCE_COVER_FILL_LOW, Val::Percent(0.0)),
                                ColorStop::new(theme::BALANCE_COVER_FILL_HIGH, Val::Percent(100.0)),
                            ],
                        ))]),
                        BalanceCoverFill,
                    ));
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            width: Val::Px(2.0),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        BoxShadow::new(Color::srgba(1.0, 1.0, 1.0, 0.8), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(8.0)),
                        BalanceDivider,
                    ));
                });
            });
        });

        // ====================================================================
        // Deck stack (bottom-left)
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(70.0),
            bottom: Val::Px(70.0),
            width: Val::Px(150.0),
            height: Val::Px(210.0),
            ..default()
        })
        .with_children(|parent| {
            for (offset, bg, border) in [
                (8.0, theme::STACK_PLATE_DEEP, theme::STACK_PLATE_BORDER_DEEP),
                (4.0, theme::STACK_PLATE_MID, theme::STACK_PLATE_BORDER_MID),
            ] {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(offset),
                        top: Val::Px(offset),
                        width: Val::Px(134.0),
                        height: Val::Px(194.0),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(bg),
                    BorderColor::all(border),
                ));
            }
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(134.0),
                    height: Val::Px(194.0),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BorderColor::all(theme::STACK_TOP_BORDER),
                BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.5), Val::Px(0.0), Val::Px(6.0), Val::Px(0.0), Val::Px(18.0)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ImageNode::default(),
                    DeckStackImage,
                ));
            });
            parent.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(-26.0),
                width: Val::Px(134.0),
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("DECK · 0"),
                    TextFont::from_font_size(12.0),
                    TextColor(theme::STACK_LABEL),
                    DeckCounter,
                ));
            });
        });

        // ====================================================================
        // Discard stack (bottom-right) - face-up resolved past
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(70.0),
            bottom: Val::Px(70.0),
            width: Val::Px(150.0),
            height: Val::Px(210.0),
            ..default()
        })
        .with_children(|parent| {
            for (offset, bg, border) in [
                (10.0, theme::STACK_PLATE_DEEP, theme::STACK_PLATE_BORDER_DEEP),
                (5.0, theme::STACK_PLATE_MID, theme::STACK_PLATE_BORDER_MID),
            ] {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(offset),
                        top: Val::Px(offset),
                        width: Val::Px(134.0),
                        height: Val::Px(194.0),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(bg),
                    BorderColor::all(border),
                ));
            }
            // Face-up top card (spawned by update_deck_discard_system)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                DiscardTopCardSlot,
            ));
            parent.spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                bottom: Val::Px(-26.0),
                width: Val::Px(134.0),
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("DISCARD · 0"),
                    TextFont::from_font_size(12.0),
                    TextColor(theme::STACK_LABEL),
                    DiscardCountText,
                ));
            });
        });

        // ====================================================================
        // Player hand fan (bottom center, children rebuilt at runtime)
        // ====================================================================
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                height: Val::Px(360.0),
                ..default()
            },
            PlayerHandDisplay,
        ));

        // ====================================================================
        // Action buttons (bottom-right of hand)
        // ====================================================================
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(280.0),
            bottom: Val::Px(60.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(52.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                pass_button_gradient(true),
                BoxShadow::new(theme::PASS_BUTTON_GLOW, Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(20.0)),
                CheckButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("PASS"),
                    TextFont::from_font_size(17.0),
                    TextColor(theme::PASS_BUTTON_TEXT),
                ));
            });

            parent.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(52.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(theme::BAIL_BUTTON_BG),
                BorderColor::all(theme::BAIL_BUTTON_BORDER),
                FoldButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("BAIL OUT"),
                    TextFont::from_font_size(15.0),
                    TextColor(theme::BAIL_BUTTON_TEXT),
                ));
            });
        });

        // ====================================================================
        // SOW-011-B: HAND RESOLUTION OVERLAY (initially hidden, unchanged)
        // ====================================================================
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None, // Hidden until hand resolves
                ..default()
            },
            GlobalZIndex(100), // On top of everything
            ResolutionOverlay,
        ))
        .with_children(|parent| {
            // Semi-transparent backdrop
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Dark semi-transparent
                ResolutionBackdrop,
            ));

            // Results panel (centered)
            parent.spawn((
                Node {
                    width: Val::Px(600.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(30.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    row_gap: Val::Px(20.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme::UI_ROOT_BG),
                BorderColor::all(theme::SCENARIO_CARD_BORDER),
                GlobalZIndex(101),
                ResolutionPanel,
            ))
            .with_children(|parent| {
                // Title
                parent.spawn((
                    Text::new("HAND COMPLETE"),
                    TextFont::from_font_size(32.0),
                    TextColor(theme::TEXT_HEADER),
                    ResolutionTitle,
                ));

                // Story text (SOW-012: Narrative generation)
                parent.spawn((
                    Text::new(""),
                    TextFont::from_font_size(16.0),
                    TextColor(theme::TEXT_SECONDARY),
                    TextLayout::new_with_justify(bevy::text::Justify::Center),
                    Node {
                        margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(20.0)),
                        max_width: Val::Px(540.0),
                        ..default()
                    },
                    ResolutionStory,
                ));

                // Results text (will be updated by system)
                parent.spawn((
                    Text::new("Results..."),
                    TextFont::from_font_size(18.0),
                    TextColor(theme::TEXT_PRIMARY),
                    TextLayout::new_with_justify(bevy::text::Justify::Center),
                    ResolutionResults,
                ));

                // Action buttons (NEW DEAL / GO HOME)
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // New Deal button
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(theme::CONTINUE_BUTTON_BG),
                        RestartButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("NEW DEAL"),
                            TextFont::from_font_size(24.0),
                            TextColor(Color::WHITE),
                        ));
                    });

                    // Go Home button
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(theme::RESTART_BUTTON_BG),
                        GoHomeButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("GO HOME"),
                            TextFont::from_font_size(24.0),
                            TextColor(Color::WHITE),
                        ));
                    });
                });
            });
        });
    });
}
