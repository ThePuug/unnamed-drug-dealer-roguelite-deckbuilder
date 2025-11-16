// SOW-AAA: UI setup functions
// Extracted from main.rs

use bevy::prelude::*;
use super::theme;
use super::components::*;
use super::helpers;
use crate::{DeckPreset, EmojiFont};

pub fn setup_deck_builder(mut commands: Commands, emoji_font: Res<EmojiFont>) {
    // Deck builder root container (initially visible, game starts in DeckBuilding state)
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
    // UI Root container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: theme::UI_ROOT_BG.into(),
            ..default()
        },
        UiRoot,
    ))
    .with_children(|parent| {
        // ====================================================================
        // SOW-011-A Phase 3: TOP ROW - Active Slots + Scenario Card + Heat Bar
        // ====================================================================
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Status (absolute positioned, top-left, floated over main content)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(10),
                    ..default()
                },
                StatusDisplay,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Round 1/3\nCash: $0",
                    TextStyle {
                        font_size: 14.0,
                        color: theme::TEXT_HEADER,
                        ..default()
                    },
                ));
            });

            // Centered content (slots + scenario + heat bar)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    column_gap: Val::Px(15.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Active card slots (horizontal row - Location, Product, Conviction, Insurance)
                // Slots are just empty containers, will be populated by update_active_slots_system
                parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        align_items: AlignItems::FlexEnd, // Bottom justify to match scenario card
                        ..default()
                    },
                    ..default()
                },
                ActiveSlotsContainer,
                ))
                .with_children(|parent| {
                    // Create empty slot containers (will be populated by system)
                    for slot_type in [SlotType::Location, SlotType::Product, SlotType::Conviction, SlotType::Insurance] {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    // No fixed size - let card/placeholder inside determine dimensions
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

                // Scenario card (larger to match heat bar height)
                parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(280.0),
                        height: Val::Px(220.0), // Match heat bar height
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(12.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        align_self: AlignSelf::FlexEnd, // Bottom align
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

                // Vertical heat bar (matches scenario card height)
                parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(40.0), // Slightly wider for visibility
                    height: Val::Px(220.0), // Match scenario card height
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::FlexEnd, // Bottom align
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
                            justify_content: JustifyContent::FlexEnd, // Fill from bottom
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
                                height: Val::Percent(0.0), // Updated by system
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
            });
        });

        // ====================================================================
        // SOW-011-A Phase 3: MIDDLE + BOTTOM COMBINED ROW
        // Narc and Buyer hands span full height (middle + bottom)
        // Center has: Played cards area (middle) + Player hand (bottom)
        // ====================================================================
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_grow: 1.0, // Take remaining space
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(15.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // ================================================================
            // LEFT: Narc section (spans full height - played cards + visible hand)
            // ================================================================
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        row_gap: Val::Px(10.0),
                        justify_content: JustifyContent::FlexStart, // Top justify (not bottom)
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.1, 0.1, 0.5).into(),
                    border_color: theme::NARC_SECTION_COLOR.into(),
                    ..default()
                },
                PlayAreaNarc,
            ))
            .with_children(|parent| {
                // Narc label
                parent.spawn(TextBundle::from_section(
                    "⚠ Narc",
                    TextStyle {
                        font_size: 14.0,
                        color: theme::NARC_SECTION_COLOR,
                        ..default()
                    },
                ));

                // Narc's visible hand section (upcoming cards, no label)
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            padding: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    NarcVisibleHand,
                ));
            });

            // ================================================================
            // CENTER: Played cards area (middle) + Player hand (bottom)
            // ================================================================
            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceBetween, // Totals/pool at top, hand at bottom
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Top section: Totals bar + Played pool
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Totals bar
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(8.0)),
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

                // Middle: Played pool + Discard pile (side by side)
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Played cards pool (grows to fill space)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                column_gap: Val::Px(5.0),
                                row_gap: Val::Px(5.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                min_height: Val::Px(140.0),
                                ..default()
                            },
                            background_color: Color::srgba(0.1, 0.1, 0.15, 0.8).into(),
                            ..default()
                        },
                        PlayAreaDealer,
                    ));

                    // Discard pile (right side, vertical list)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                row_gap: Val::Px(3.0),
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
                        // Discarded cards will be added by system
                    });
                });
                });

                // Bottom: Player hand with betting buttons immediately to the right
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd, // Bottom justify content
                        align_self: AlignSelf::FlexEnd, // Align entire panel to bottom
                        column_gap: Val::Px(15.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: theme::PLAYER_HAND_BG.into(),
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

                    // Betting buttons (immediately to right, vertically stacked)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(10.0),
                                display: Display::Flex, // Always visible
                                ..default()
                            },
                            ..default()
                        },
                        BettingActionsContainer,
                    ))
                    .with_children(|parent| {
                        // Pass button (check)
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

                        // Bail Out button (fold)
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
            });

            // ================================================================
            // RIGHT: Buyer section (spans full height - deck panel + visible hand)
            // ================================================================
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        row_gap: Val::Px(10.0),
                        justify_content: JustifyContent::FlexEnd, // Bottom justify
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.1, 0.5).into(),
                    border_color: theme::BUYER_SECTION_COLOR.into(),
                    ..default()
                },
                BuyerDeckPanel,
            ))
            .with_children(|parent| {
                // Buyer label
                parent.spawn(TextBundle::from_section(
                    "⚠ Buyer",
                    TextStyle {
                        font_size: 14.0,
                        color: theme::BUYER_SECTION_COLOR,
                        ..default()
                    },
                ));

                // Buyer deck section (cards coming up)
                // Will be populated by render_buyer_visible_hand_system
                // For now, just the container

                // Buyer's visible hand section (at bottom of panel, no label)
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    },
                    BuyerVisibleHand,
                ));
            });
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
