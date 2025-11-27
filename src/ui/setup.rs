// SOW-AAA: UI setup functions
// Extracted from main.rs
// Updated for Bevy 0.17

use bevy::prelude::*;
use super::theme;
use super::components::*;
use crate::models::fonts::EmojiFont;

pub fn setup_deck_builder(
    mut commands: Commands,
    save_data: Option<Res<crate::save::SaveData>>,
    emoji_font: Res<EmojiFont>,
) {
    // RFC-019: Don't spawn DeckBuilder UI if we're about to redirect to UpgradeChoice
    if let Some(ref data) = save_data {
        if let Some(ref character) = data.character {
            if character.has_pending_upgrades() {
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
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(10.0),
                            display: Display::Flex,
                            ..default()
                        },
                        BettingActionsContainer,
                    ))
                    .with_children(|parent| {
                        // Deck counter above buttons
                        parent.spawn((
                            Text::new("Deck: 20"),
                            TextFont::from_font_size(16.0),
                            TextColor(theme::TEXT_SECONDARY),
                            DeckCounter,
                        ));

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
