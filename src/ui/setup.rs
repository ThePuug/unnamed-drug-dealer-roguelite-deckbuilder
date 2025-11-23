// SOW-AAA: UI setup functions
// Extracted from main.rs

use bevy::prelude::*;
use super::theme;
use super::components::*;
use super::helpers;
use crate::{DeckPreset, EmojiFont};

pub fn setup_deck_builder(mut commands: Commands, emoji_font: Res<EmojiFont>) {
    // Deck builder root container - 1920x1080 design, will be scaled/positioned by scale_ui_to_fit_system
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(1920.0),
                height: Val::Px(1080.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: theme::DECK_BUILDER_BG.into(),
            ..default()
        },
        DeckBuilderRoot,
    ))
    .with_children(|parent| {
        // Title with emoji test
        parent.spawn(helpers::text_bundle_with_emoji(
            "🃏 DECK BUILDER 🎴",
            40.0,
            Color::WHITE,
            &emoji_font,
        ));

        // Main content area (horizontal split: pool | selected)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // SOW-010: Single grid view with toggle selection (green = selected, gray = not)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(5.0),
                        column_gap: Val::Px(5.0),
                        align_content: AlignContent::FlexStart,
                        ..default()
                    },
                    background_color: theme::CARD_POOL_BG.into(),
                    ..default()
                },
                CardPoolContainer,
            ))
            .with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0), // Full width forces cards to wrap to next row
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Click cards to toggle selection (Green = Selected)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });
        });

        // Bottom: Stats and actions
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Deck stats
            parent.spawn((
                TextBundle::from_section(
                    "Deck: 20/20 cards",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                DeckStatsDisplay,
            ));

            // Preset buttons
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Default preset
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: theme::CARD_AVAILABLE_BG.into(),
                        ..default()
                    },
                    PresetButton { preset: DeckPreset::Default },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "DEFAULT (20)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Aggro preset
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: theme::CARD_UNAVAILABLE_BG.into(),
                        ..default()
                    },
                    PresetButton { preset: DeckPreset::Aggro },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "AGGRO (10)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Control preset
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: theme::CARD_UNAVAILABLE_BG.into(),
                        ..default()
                    },
                    PresetButton { preset: DeckPreset::Control },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "CONTROL (16)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });

            // START RUN button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: theme::CONTINUE_BUTTON_BG.into(),
                    ..default()
                },
                StartRunButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "START RUN",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    });
}

pub fn create_ui(commands: &mut Commands) {
    // Background image layer (behind everything)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                overflow: Overflow::clip(),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            z_index: ZIndex::Global(-1),
            background_color: theme::UI_ROOT_BG.into(),
            ..default()
        },
        BackgroundImage,
    ))
    .with_children(|parent| {
        // Image node inside container
        parent.spawn((
            NodeBundle {
                style: Style {
                    ..default()
                },
                ..default()
            },
            UiImage::default(),
            BackgroundImageNode,
        ));
    });

    // UI Root container - 1920x1080 design, will be scaled/positioned by scale_ui_to_fit_system
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(1920.0),
                height: Val::Px(1080.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        UiRoot,
    ))
    .with_children(|parent| {
        // ====================================================================
        // LEFT COLUMN: Narc Hand (full height)
        // ====================================================================
        parent.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            },
            PlayAreaNarc,
        ))
        .with_children(|parent| {
            // Narc's visible hand section
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                NarcVisibleHand,
            ));
        });

        // ====================================================================
        // CENTER COLUMN: Game Area (4 rows)
        // ====================================================================
        parent.spawn(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // ================================================================
            // ROW 1: Deal Row - Slots/Scenario/Heat (center aligned)
            // ================================================================
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    column_gap: Val::Px(12.0),
                    padding: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                    // Active card slots (horizontal row)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(10.0),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            ..default()
                        },
                        ActiveSlotsContainer,
                    ))
                    .with_children(|parent| {
                        // Create empty slot containers - let cards define size
                        for slot_type in [SlotType::Location, SlotType::Product, SlotType::Conviction, SlotType::Insurance] {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ActiveSlot { slot_type },
                            ));
                        }
                    });

                    // Scenario card - landscape orientation (870:601 aspect ratio, 275px height)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(398.09),
                                height: Val::Px(275.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            background_color: theme::SCENARIO_CARD_BG.into(),
                            border_color: theme::SCENARIO_CARD_BORDER.into(),
                            ..default()
                        },
                        BuyerScenarioCard,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Buyer Scenario Info",
                                TextStyle {
                                    font_size: 16.0,
                                    color: theme::SCENARIO_CARD_TEXT,
                                    ..default()
                                },
                            ),
                            BuyerScenarioCardText,
                        ));
                    });

                    // Vertical heat bar - 275px to match card height
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(42.0),
                            height: Val::Px(275.0),
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::FlexEnd,
                            justify_content: JustifyContent::FlexEnd,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Heat bar container
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::FlexEnd,
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                background_color: theme::HEAT_BAR_BG.into(),
                                border_color: theme::CARD_BORDER_NORMAL.into(),
                                ..default()
                            },
                            HeatBar,
                        ))
                        .with_children(|parent| {
                            // Heat bar fill
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(0.0),
                                        ..default()
                                    },
                                    background_color: theme::HEAT_BAR_GREEN.into(),
                                    ..default()
                                },
                                HeatBarFill,
                            ));
                        });

                        // Heat bar text below
                        parent.spawn((
                            TextBundle::from_section(
                                "0/100",
                                TextStyle {
                                    font_size: 11.0,
                                    color: theme::TEXT_SECONDARY,
                                    ..default()
                                },
                            ).with_text_justify(JustifyText::Center),
                            HeatBarText,
                        ));
                    });

                    // Discard pile - 275px to match card height
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(190.0),
                                height: Val::Px(275.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(6.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                row_gap: Val::Px(3.0),
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                            border_color: theme::CARD_BORDER_NORMAL.into(),
                            ..default()
                        },
                        DiscardPile,
                    ))
                    .with_children(|parent| {
                        // Discard pile header
                        parent.spawn(TextBundle::from_section(
                            "Discard Pile",
                            TextStyle {
                                font_size: 12.0,
                                color: theme::TEXT_SECONDARY,
                                ..default()
                            },
                        ));
                    });
            });

            // ================================================================
            // ROW 2: Counters Row - Evidence, Cover, Multiplier totals
            // ================================================================
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Evidence total
                parent.spawn((
                    TextBundle::from_section(
                        "● Evidence: 0",
                        TextStyle {
                            font_size: 16.0,
                            color: theme::EVIDENCE_CARD_COLOR,
                            ..default()
                        },
                    ),
                    EvidencePool,
                ));

                // Cover total
                parent.spawn((
                    TextBundle::from_section(
                        "● Cover: 0",
                        TextStyle {
                            font_size: 16.0,
                            color: theme::COVER_CARD_COLOR,
                            ..default()
                        },
                    ),
                    CoverPool,
                ));

                // Multiplier total
                parent.spawn((
                    TextBundle::from_section(
                        "● Multiplier: ×1.0",
                        TextStyle {
                            font_size: 16.0,
                            color: theme::DEAL_MODIFIER_CARD_COLOR,
                            ..default()
                        },
                    ),
                    DealModPool,
                ));
            });

            // ================================================================
            // ROW 3: Play Area Row - Played cards only (no wrapper, direct container)
            // ================================================================
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0, // Take available space
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::FlexStart, // Align cards to top
                        ..default()
                    },
                    ..default()
                },
                PlayAreaDealer,
            ));

            // ================================================================
            // ROW 4: Player Hand Row - 3 columns (Narc Image | Player Hand | Buyer Image)
            // ================================================================
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexEnd,
                    padding: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Left: Narc portrait - 50% larger than 70% (255 * 1.5 = 383)
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(383.0),
                            height: Val::Px(383.0),
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::default(),
                    NarcPortrait,
                ));

                // Center: Player hand + betting buttons
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        column_gap: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Player hand cards
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        },
                        PlayerHandDisplay,
                    ));

                    // Betting buttons (vertically stacked)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(10.0),
                                display: Display::Flex,
                                ..default()
                            },
                            ..default()
                        },
                        BettingActionsContainer,
                    ))
                    .with_children(|parent| {
                        // Pass button - fixed pixels
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: theme::BUTTON_ENABLED_BG.into(),
                                ..default()
                            },
                            CheckButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Pass",
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });

                        // Bail Out button - fixed pixels
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: theme::BUTTON_NEUTRAL_BG.into(),
                                ..default()
                            },
                            FoldButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Bail Out",
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });
                    });
                });

                // Right: Buyer portrait - 50% larger than 70% (255 * 1.5 = 383)
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(383.0),
                            height: Val::Px(383.0),
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::default(),
                    BuyerPortrait,
                ));
            });
        });

        // ====================================================================
        // RIGHT COLUMN: Buyer Hand (full height)
        // ====================================================================
        parent.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            },
            BuyerDeckPanel,
        ))
        .with_children(|parent| {
            // Buyer's visible hand section
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                BuyerVisibleHand,
            ));
        });

        // ====================================================================
        // SOW-011-B: HAND RESOLUTION OVERLAY (initially hidden)
        // ====================================================================
        parent.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::None, // Hidden until hand resolves
                    ..default()
                },
                z_index: ZIndex::Global(100), // On top of everything
                ..default()
            },
            ResolutionOverlay,
        ))
        .with_children(|parent| {
            // Semi-transparent backdrop
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(), // Dark semi-transparent
                    ..default()
                },
                ResolutionBackdrop,
            ));

            // Results panel (centered)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        row_gap: Val::Px(20.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: theme::UI_ROOT_BG.into(),
                    border_color: theme::SCENARIO_CARD_BORDER.into(),
                    z_index: ZIndex::Global(101),
                    ..default()
                },
                ResolutionPanel,
            ))
            .with_children(|parent| {
                // Title
                parent.spawn((
                    TextBundle::from_section(
                        "HAND COMPLETE",
                        TextStyle {
                            font_size: 32.0,
                            color: theme::TEXT_HEADER,
                            ..default()
                        },
                    ),
                    ResolutionTitle,
                ));

                // Story text (SOW-012: Narrative generation)
                parent.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 16.0,
                            color: theme::TEXT_SECONDARY,
                            ..default()
                        },
                    ).with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(20.0)),
                        max_width: Val::Px(540.0),
                        ..default()
                    }),
                    ResolutionStory,
                ));

                // Results text (will be updated by system)
                parent.spawn((
                    TextBundle::from_section(
                        "Results...",
                        TextStyle {
                            font_size: 18.0,
                            color: theme::TEXT_PRIMARY,
                            ..default()
                        },
                    ).with_text_justify(JustifyText::Center),
                    ResolutionResults,
                ));

                // Action buttons (NEW DEAL / GO HOME)
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // New Deal button
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(180.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: theme::CONTINUE_BUTTON_BG.into(),
                            ..default()
                        },
                        RestartButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "NEW DEAL",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });

                    // Go Home button
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(180.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: theme::RESTART_BUTTON_BG.into(),
                            ..default()
                        },
                        GoHomeButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "GO HOME",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
                });
            });
        });
    });
}
