# SOW-037: back_of_the_club Shop Location (guard + verify)

## Status

**Review** - 2026-07-14

## References

- **Builds on:** SOW-033 (Zone Retheme / Red Light District nightlife zone,
  which authored `back_of_the_club`), SOW-020 (Location Card Shops), SOW-036
  (Signature Dealers + Pimp reaction-deck boot fix).
- **Branch:** `worktree-wf_d1600241-c3a-1`
- **Implementation Time:** ~1 hour

---

## Feature Summary

`back_of_the_club` is already a purchasable Red Light shop location. It ships in
`assets/cards/locations.ron` with the full shop hook:

```
shop_location:      Some("red_light_district")
shop_price:         Some(800)
shop_cred_required: Some(1)
card_type:          Location(evidence: 12, cover: 22, heat: 0)
```

Nothing about the buy path is broken. This SOW is a **regression guard**: it adds
one unit test that pins the shop hook so a future edit cannot silently strip it,
and it records why the hook is safe to keep. No production/content change.

**Why now:** the existing orphan test only checks price-IF-located and passes
vacuously if the `shop_location` is removed (see Discussion). The shop hook was
therefore unguarded, and a recent boot fix touched an adjacent reference to this
same id (the Pimp reaction deck), making an accidental future strip plausible.

---

## Implementation Plan

### Phase 1: Regression guard

**Goal:** Pin `back_of_the_club`'s shop hook with a dedicated, non-vacuous test.

**Deliverables:**
- `test_shipped_back_of_the_club_is_buyable` in the `#[cfg(test)]` module of
  `src/assets/loader.rs`, placed next to
  `test_shipped_ladder_leaves_no_card_orphaned`. It reuses the existing
  `load_all_shipped_player_cards()` helper and asserts the card:
  - exists in the shipped player-card set,
  - `shop_location == Some("red_light_district")`,
  - `shop_price == Some(800)` AND `> 0`,
  - `shop_cred_required == Some(1)`,
  - `matches!(card_type, CardType::Location { .. })`.

**Architectural Constraints:**
- Test-only change; no production code, no content (`.ron`) edits.
- Loads real shipped assets via the existing helper (no fixtures), so the guard
  tracks what actually ships.

**Success Criteria:**
- The new test passes against shipped assets.
- Deleting or altering any of the four shop fields on `back_of_the_club` fails
  the test loudly (this is the non-vacuous property the orphan test lacks).
- Full `cargo test` stays green; `cargo build` and `cargo test` emit zero
  warnings.

---

## Acceptance Criteria

**Functional:**
- `back_of_the_club` remains a purchasable Red Light location: stocked at
  `red_light_district`, priced 800, cred-gated at 1, a Location card.

**Code Quality:**
- Zero warnings on `cargo build` AND `cargo test`.
- The guard is non-vacuous (fails if the shop hook is stripped or the price
  drifts).

---

## Discussion

### Stale-premise finding: the boot fix dropped the Pimp reaction deck, not the shop pool

The premise "back_of_the_club needs to be made buyable" is stale. It already is.
Where the confusion comes from: the SOW-036 P0 boot fix dropped `back_of_the_club`
from **one** place - the **Pimp buyer's `reaction_deck_ids`** in `assets/buyers.ron`
(SOW-036 acceptance review: "the alley dropped to 7 cards"). It was **never**
dropped from the shop pool in `assets/cards/locations.ron`.

Confirmed in this worktree:
- `assets/cards/locations.ron` (the `back_of_the_club` entry) still carries
  `shop_location: Some("red_light_district")`, `shop_price: Some(800)`,
  `shop_cred_required: Some(1)`.
- `assets/buyers.ron`: the id `back_of_the_club` appears in **no**
  `reaction_deck_ids`. The Pimp's deck is now the 7 cards `vip_room`, `in_a_limo`,
  `comped_bottles`, `crowd_cover`, `making_a_scene`, `invite_only`,
  `noise_complaint`. "Back of the Club" still appears in the Pimp's
  `demand.locations` / `scenarios[].locations` - but those are **display names**
  (matched by card NAME per the SOW-021 validation rule), not card ids, so they
  are unrelated to the shop hook.

### Why the orphan test did not already guard this

`test_shipped_ladder_leaves_no_card_orphaned` iterates all shipped cards and, for
each card **with** a `shop_location`, asserts it also has a `shop_price`. That is
a price-IF-located invariant. If someone removes `back_of_the_club`'s
`shop_location`, the card simply drops out of the `is_some()` branch and the test
still passes - vacuously. The test's explicit id list (`shrooms`, `codeine`,
`at_the_park`) does not include `back_of_the_club`, so nothing pinned it. The new
guard asserts the concrete field values directly, so a strip or a price drift
fails loudly.

### Scope

Guard-only. No production or content change - the buy path is already correct;
this SOW just makes "already correct" enforced.

---

## Acceptance Review

### Live buy -> play e2e (to be run by the human on a real desktop window)

The unit guard proves the data ships correctly; the following live check proves
the end-to-end buy-then-play loop against a real window. Driver:
`tools/e2e/game-drive.ps1` (needs a real desktop window - not run in CI/subagent).

1. **Forge a save** with the Red Light District unlocked
   (`account.unlocked_locations` contains `red_light_district`), a dealer
   stationed there at **street cred >= 1**, and **cash >= 800**.
2. **Open the Red Light shop** and buy **Back of the Club** ($800).
   - Expect: cash decreases by exactly 800; the card flips to **OWNED** in the
     shop; the purchase persists across a save/reload.
3. **Start a Red Light deal** and **play Back of the Club** as the location.
   - Expect: the deal accepts it as a Location card contributing
     **E12 / C22 / H0**; a "behind the club" narrative fragment (from the card's
     `location_clauses`) can surface; **no panic**, deal resolves normally.

**Pass condition:** buy debits 800 and flips OWNED; the owned card is playable in
a Red Light deal with the E12/C22/H0 contribution and a behind-the-club fragment,
with no panic.

---

## Conclusion

`back_of_the_club` was already a correctly-hooked, purchasable Red Light shop
location; the only gap was that no test pinned it (the orphan test guards it only
vacuously). This SOW adds `test_shipped_back_of_the_club_is_buyable` to lock the
shop hook against accidental future strips - motivated by the SOW-036 boot fix
having touched the adjacent Pimp reaction-deck reference to the same id - and
records the stale-premise finding. Test-only; full suite green, zero warnings.
Live buy -> play e2e left for the human on a real window per the checklist above.
