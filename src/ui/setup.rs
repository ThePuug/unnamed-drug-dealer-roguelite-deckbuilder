// SOW-AAA: UI setup functions
// Extracted from main.rs
// Updated for Bevy 0.17

use bevy::prelude::*;
use super::theme;
use super::components::*;

pub fn setup_deck_builder(mut commands: Commands) {
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
        // Card pool container - scrollable, fills available space
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
            ScrollPosition::default(), // Bevy 0.17: Required for scrolling
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

                // Character heat display
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Heat: 0"),
                        TextFont::from_font_size(20.0),
                        TextColor(Color::WHITE),
                        CharacterHeatText,
                    ));
                    parent.spawn((
                        Text::new("[Cold]"),
                        TextFont::from_font_size(20.0),
                        TextColor(Color::srgb(0.3, 0.7, 0.3)),
                        CharacterTierText,
                    ));
                });

                // Decay info (hidden by default)
                parent.spawn((
                    Text::new(""),
                    TextFont::from_font_size(18.0),
                    TextColor(Color::srgb(0.5, 0.8, 1.0)),
                    Visibility::Hidden,
                    DecayInfoDisplay,
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
}

pub fn create_ui(commands: &mut Commands) {
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
        BackgroundColor(theme::UI_ROOT_BG),
        BackgroundImage,
    ))
    .with_children(|parent| {
        // Image node inside container
        parent.spawn((
            Node::default(),
            ImageNode::default(),
            BackgroundImageNode,
        ));
    });

    // UI Root container - 1920x1080 design, will be scaled/positioned by scale_ui_to_fit_system
    commands.spawn((
        Node {
            width: Val::Px(1920.0),
            height: Val::Px(1080.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        UiRoot,
    ))
    .with_children(|parent| {
        // ====================================================================
        // LEFT COLUMN: Narc Hand (full height)
        // ====================================================================
        parent.spawn((
            Node {
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            PlayAreaNarc,
        ))
        .with_children(|parent| {
            // Narc's visible hand section
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                NarcVisibleHand,
            ));
        });

        // ====================================================================
        // CENTER COLUMN: Game Area (4 rows)
        // ====================================================================
        parent.spawn(Node {
            flex_grow: 1.0,
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|parent| {
            // ================================================================
            // ROW 1: Deal Row - Slots/Scenario/Heat (center aligned)
            // ================================================================
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(12.0),
                padding: UiRect::vertical(Val::Px(5.0)),
                ..default()
            })
            .with_children(|parent| {
                    // Active card slots (horizontal row)
                    parent.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            align_items: AlignItems::FlexEnd,
                            ..default()
                        },
                        ActiveSlotsContainer,
                    ))
                    .with_children(|parent| {
                        // Create empty slot containers - let cards define size
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

                    // Scenario card - landscape orientation (870:601 aspect ratio, 275px height)
                    parent.spawn((
                        Node {
                            width: Val::Px(398.09),
                            height: Val::Px(275.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                        BackgroundColor(theme::SCENARIO_CARD_BG),
                        BorderColor::all(theme::SCENARIO_CARD_BORDER),
                        BuyerScenarioCard,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Buyer Scenario Info"),
                            TextFont::from_font_size(16.0),
                            TextColor(theme::SCENARIO_CARD_TEXT),
                            BuyerScenarioCardText,
                        ));
                    });

                    // Vertical heat bar - 275px to match card height
                    parent.spawn(Node {
                        width: Val::Px(42.0),
                        height: Val::Px(275.0),
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::FlexEnd,
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    })
                    .with_children(|parent| {
                        // Heat bar container
                        parent.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(theme::HEAT_BAR_BG),
                            BorderColor::all(theme::CARD_BORDER_NORMAL),
                            HeatBar,
                        ))
                        .with_children(|parent| {
                            // Heat bar fill
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(0.0),
                                    ..default()
                                },
                                BackgroundColor(theme::HEAT_BAR_GREEN),
                                HeatBarFill,
                            ));
                        });

                        // Heat bar text below
                        parent.spawn((
                            Text::new("0/100"),
                            TextFont::from_font_size(11.0),
                            TextColor(theme::TEXT_SECONDARY),
                            TextLayout::new_with_justify(bevy::text::Justify::Center),
                            HeatBarText,
                        ));
                    });

                    // Discard pile - 275px to match card height
                    parent.spawn((
                        Node {
                            width: Val::Px(190.0),
                            height: Val::Px(275.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            row_gap: Val::Px(3.0),
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                        BorderColor::all(theme::CARD_BORDER_NORMAL),
                        DiscardPile,
                    ))
                    .with_children(|parent| {
                        // Discard pile header
                        parent.spawn((
                            Text::new("Discard Pile"),
                            TextFont::from_font_size(12.0),
                            TextColor(theme::TEXT_SECONDARY),
                        ));
                    });
            });

            // ================================================================
            // ROW 2: Counters Row - Evidence, Cover, Multiplier totals
            // ================================================================
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                padding: UiRect::vertical(Val::Px(5.0)),
                ..default()
            })
            .with_children(|parent| {
                // Evidence total
                parent.spawn((
                    Text::new("● Evidence: 0"),
                    TextFont::from_font_size(16.0),
                    TextColor(theme::EVIDENCE_CARD_COLOR),
                    EvidencePool,
                ));

                // Cover total
                parent.spawn((
                    Text::new("● Cover: 0"),
                    TextFont::from_font_size(16.0),
                    TextColor(theme::COVER_CARD_COLOR),
                    CoverPool,
                ));

                // Multiplier total
                parent.spawn((
                    Text::new("● Multiplier: ×1.0"),
                    TextFont::from_font_size(16.0),
                    TextColor(theme::DEAL_MODIFIER_CARD_COLOR),
                    DealModPool,
                ));
            });

            // ================================================================
            // ROW 3: Play Area Row - Played cards only (no wrapper, direct container)
            // ================================================================
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0, // Take available space
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::FlexStart, // Align cards to top
                    ..default()
                },
                PlayAreaDealer,
            ));

            // ================================================================
            // ROW 4: Player Hand Row - 3 columns (Narc Image | Player Hand | Buyer Image)
            // ================================================================
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::vertical(Val::Px(5.0)),
                ..default()
            })
            .with_children(|parent| {
                // Left: Narc portrait - 50% larger than 70% (255 * 1.5 = 383)
                parent.spawn((
                    Node {
                        width: Val::Px(383.0),
                        height: Val::Px(383.0),
                        ..default()
                    },
                    ImageNode::default(),
                    NarcPortrait,
                ));

                // Center: Player hand + betting buttons
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    column_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Player hand cards
                    parent.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(10.0),
                            ..default()
                        },
                        PlayerHandDisplay,
                    ));

                    // Betting buttons (vertically stacked)
                    parent.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            row_gap: Val::Px(10.0),
                            display: Display::Flex,
                            ..default()
                        },
                        BettingActionsContainer,
                    ))
                    .with_children(|parent| {
                        // Pass button - fixed pixels
                        parent.spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme::BUTTON_ENABLED_BG),
                            CheckButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Pass"),
                                TextFont::from_font_size(16.0),
                                TextColor(Color::WHITE),
                            ));
                        });

                        // Bail Out button - fixed pixels
                        parent.spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme::BUTTON_NEUTRAL_BG),
                            FoldButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Bail Out"),
                                TextFont::from_font_size(16.0),
                                TextColor(Color::WHITE),
                            ));
                        });
                    });
                });

                // Right: Buyer portrait - 50% larger than 70% (255 * 1.5 = 383)
                parent.spawn((
                    Node {
                        width: Val::Px(383.0),
                        height: Val::Px(383.0),
                        ..default()
                    },
                    ImageNode::default(),
                    BuyerPortrait,
                ));
            });
        });

        // ====================================================================
        // RIGHT COLUMN: Buyer Hand (full height)
        // ====================================================================
        parent.spawn((
            Node {
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            BuyerDeckPanel,
        ))
        .with_children(|parent| {
            // Buyer's visible hand section
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                BuyerVisibleHand,
            ));
        });

        // ====================================================================
        // SOW-011-B: HAND RESOLUTION OVERLAY (initially hidden)
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
