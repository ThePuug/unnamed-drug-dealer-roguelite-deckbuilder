// SOW-023: dev save forge - manufactures signed saves for e2e playtests.
// Reed approved signing with the real HMAC key (assets/secrets/save_key.ron,
// already compiled into the crypto module) so scripted scenarios start from
// exact, reproducible states instead of grinding the game into position.
//
// Usage (no Bevy App is built - it writes the file and exits):
//   cargo run -- forge <fresh|funded|roster|hot> [--dir <path>]
// Without --dir the save lands where the game will load it (honors
// DDD_SAVE_DIR, else the platform default).

use super::io;
use super::types::*;
use std::path::PathBuf;

/// Build a named scenario. Pure - the caller decides where it goes.
pub fn scenario(name: &str) -> Option<SaveData> {
    let mut save = SaveData::new();
    match name {
        // A brand-new empire: just the kingpin, no money
        "fresh" => {}
        // Enough cash to exercise HIRE a few times
        "funded" => {
            save.account.cash_on_hand = 5000;
        }
        // A mid-game roster: warm kingpin, an available hire, and a dealer
        // mid-sentence (2 of 3 runs remaining, jailed hot at 75)
        "roster" => {
            save.account.cash_on_hand = 1200;
            save.dealers[0].character.heat = 20;

            // SOW-039: authored zone dealers (faces loaded via the signature +
            // unlockable loops) instead of the retired generic recruit pool.
            let mut ray = DealerState::zone_dealer(DEFAULT_STATION, "Ray", "Bubba");
            ray.character.heat = 45;
            save.dealers.push(ray);

            let mut jailed = DealerState::zone_dealer(DEFAULT_STATION, "Tex", "Gladys");
            jailed.character.heat = 75;
            jailed.status = DealerStatus::Jailed {
                runs_remaining: 2,
                sentence_total: 3,
                heat_at_bust: 75,
            };
            save.dealers.push(jailed);
        }
        // A kingpin one bad hand from game over (tier: Blazing)
        "hot" => {
            save.account.cash_on_hand = 500;
            save.dealers[0].character.heat = 90;
        }
        // SOW-024: enough cash to buy Suburbia ($1,200), which starts locked
        "mogul" => {
            save.account.cash_on_hand = 3000;
            save.dealers[0].character.heat = 20;
        }
        // SOW-025: stationing/cred demo - kingpin repped-up in Trailer Park
        // (4 cred), a hired dealer stationed in the unlocked Suburbia with 2
        // cred (clears Codeine/Xanax there; Red Light's Coke stays out of
        // reach), and $1,500 to afford a Suburbia buy or a move+change
        // (SOW-033: Storage Unit re-homed to Suburbia; Heroin shelved)
        "hustler" => {
            save.account.cash_on_hand = 1500;
            save.account.unlocked_locations.insert("suburbia".to_string());
            save.dealers[0].character.heat = 10;
            for _ in 0..4 {
                save.dealers[0].add_cred(DEFAULT_STATION);
            }

            // SOW-039: Suburbia's authored signature face (Roxanne), stationed
            // there by zone_dealer - replaces the retired generic recruit.
            let mut ray = DealerState::zone_dealer("suburbia", "Ray", "Roxanne");
            ray.character.heat = 30;
            for _ in 0..2 {
                ray.add_cred("suburbia");
            }
            save.dealers.push(ray);
        }
        // SOW-033: Red Light pacing/e2e - kingpin stationed in the Red Light
        // District with entry cred, both expansion zones unlocked, mid-game wallet
        "nightowl" => {
            save.account.cash_on_hand = 2500;
            save.account.unlocked_locations.insert("red_light_district".to_string());
            save.account.unlocked_locations.insert("suburbia".to_string());
            save.dealers[0].station = "red_light_district".to_string();
            save.dealers[0].character.heat = 20;
            for _ in 0..2 {
                save.dealers[0].add_cred("red_light_district");
            }
        }
        // SOW-030: ledger e2e - an empire with history. Two fallen empires
        // bracket the living one ($5,000 > $2,000 living > $900), stories
        // on both active dealers, cred spread across two zones.
        "legacy" => {
            save.account.cash_on_hand = 800;
            save.account.lifetime_revenue = 2000;
            save.account.unlocked_locations.insert("red_light_district".to_string());
            save.dealers[0].character.heat = 35;
            save.dealers[0].character.decks_played = 6;
            for _ in 0..3 {
                save.dealers[0].add_cred(DEFAULT_STATION);
            }
            save.dealers[0].character.story_history.extend([
                "Moved product at the park while the beat cop bought donuts.".to_string(),
                "The frat house wanted it loud; the kingpin kept it quiet.".to_string(),
                "A noise complaint nearly ended the night early.".to_string(),
            ]);

            // SOW-039: Red Light's authored signature face (Marcus), stationed
            // there by zone_dealer - replaces the retired generic recruit.
            let mut ray = DealerState::zone_dealer("red_light_district", "Ray", "Marcus");
            ray.character.heat = 55;
            ray.character.decks_played = 3;
            ray.prior_convictions = 1;
            for _ in 0..2 {
                ray.add_cred("red_light_district");
            }
            ray.character.story_history.push(
                "Ray worked the velvet rope like he owned the club.".to_string(),
            );
            save.dealers.push(ray);

            save.fallen_empires.push(EmpireEpitaph {
                ended_at: 1_700_000_000,
                lifetime_revenue: 900,
                cash_at_fall: 40,
                dealers_hired: 0,
                total_prior_convictions: 1,
                decks_played: 4,
                stories: vec![
                    "The first empire died broke on a corner.".to_string(),
                ],
            });
            save.fallen_empires.push(EmpireEpitaph {
                ended_at: 1_710_000_000,
                lifetime_revenue: 5000,
                cash_at_fall: 1200,
                dealers_hired: 2,
                total_prior_convictions: 3,
                decks_played: 15,
                stories: vec![
                    "The second empire ran three dealers deep.".to_string(),
                    "It ended in a warehouse full of warrants.".to_string(),
                ],
            });
        }
        // SOW-031/034: a live front mid-window - a Shrooms BATCH on Lil
        // Smoke's credit ($125 owed, 3 of 4 runs left), $60 cash (can pay after
        // ~1 deal). Shrooms is unlocked first (fronting is the out-of-stock
        // floor for a product you already have access to).
        "fronted" => {
            save.account.cash_on_hand = 60;
            save.dealers[0].add_cred(DEFAULT_STATION);
            save.account.unlocked_cards.insert("shrooms".to_string());
            save.take_front("shrooms", DEFAULT_STATION, 100)
                .expect("fronted scenario takes the shrooms batch");
            save.fronts[0].runs_remaining = 3;
        }
        // SOW-031/034: the muscle is one run out - CutOff standing, $40 cash
        // (seizure = $8), front expires on the next completed run. On souring
        // it seizes the unsold Shrooms batch (access stays). Also demonstrates
        // the stock lock on the Trailer Park shop tab.
        "strapped" => {
            save.account.cash_on_hand = 40;
            save.dealers[0].add_cred(DEFAULT_STATION);
            save.account.unlocked_cards.insert("shrooms".to_string());
            save.take_front("shrooms", DEFAULT_STATION, 100)
                .expect("strapped scenario takes the shrooms batch");
            save.supplier_standing
                .insert(DEFAULT_STATION.to_string(), SupplierStanding::CutOff);
            save.fronts[0].runs_remaining = 1;
        }
        _ => return None,
    }
    Some(save)
}

/// CLI entry: parse `<scenario> [--dir <path>]`, write the signed save
pub fn run_cli(args: &[String]) {
    let Some(name) = args.first() else {
        eprintln!("usage: forge <fresh|funded|roster|hot|mogul|hustler|nightowl|legacy|fronted|strapped> [--dir <path>]");
        std::process::exit(2);
    };

    let Some(save) = scenario(name) else {
        eprintln!("unknown scenario '{name}' (fresh|funded|roster|hot|mogul|hustler|nightowl|legacy|fronted|strapped)");
        std::process::exit(2);
    };
    save.validate().expect("forged scenario must pass save validation");

    let dir = args
        .iter()
        .position(|a| a == "--dir")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(io::get_save_directory);
    let _ = std::fs::create_dir_all(&dir);

    let save_path = dir.join("save.dat");
    let backup_path = dir.join("save.dat.bak");
    io::save_atomic(&save_path, &backup_path, &save).expect("failed to write forged save");
    println!("forged '{}' -> {}", name, save_path.display());
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn every_scenario_validates_and_roundtrips() {
        let dir = tempdir().unwrap();
        for name in [
            "fresh", "funded", "roster", "hot", "mogul", "hustler", "nightowl", "legacy",
            "fronted", "strapped",
        ] {
            let save = scenario(name).expect(name);
            save.validate().unwrap_or_else(|e| panic!("{name} invalid: {e:?}"));

            let path = dir.path().join(format!("{name}.dat"));
            let backup = dir.path().join(format!("{name}.bak"));
            io::save_atomic(&path, &backup, &save).unwrap();
            let loaded = io::load_save(&path).unwrap_or_else(|e| panic!("{name} load: {e:?}"));
            assert_eq!(loaded.dealers.len(), save.dealers.len());
        }
    }

    #[test]
    fn roster_scenario_shape() {
        let save = scenario("roster").unwrap();
        assert_eq!(save.dealers.len(), 3);
        assert!(save.dealers[0].is_kingpin);
        assert_eq!(save.dealers[1].name, "Ray");
        assert!(save.dealers[1].is_available());
        assert_eq!(save.dealers[2].jail_remaining(), Some(2));
        assert_eq!(save.account.cash_on_hand, 1200);
    }

    #[test]
    fn hustler_scenario_shape() {
        let save = scenario("hustler").unwrap();
        assert_eq!(save.dealers.len(), 2);
        assert_eq!(save.dealers[0].cred_in(DEFAULT_STATION), 4); // clears Storage Unit's 3
        assert_eq!(save.dealers[1].station, "suburbia");
        assert_eq!(save.dealers[1].cred_in("suburbia"), 2); // Heroin's 5 stays locked
        assert!(save.account.unlocked_locations.contains("suburbia"));
    }

    #[test]
    fn nightowl_scenario_shape() {
        let save = scenario("nightowl").unwrap();
        assert_eq!(save.dealers[0].station, "red_light_district");
        assert_eq!(save.dealers[0].cred_in("red_light_district"), 2);
        assert!(save.account.unlocked_locations.contains("red_light_district"));
        assert!(save.account.unlocked_locations.contains("suburbia"));
    }

    #[test]
    fn legacy_scenario_shape() {
        let save = scenario("legacy").unwrap();
        assert_eq!(save.fallen_empires.len(), 2);
        // The living empire ($2,000 lifetime) slots between the fallen
        // ($5,000 and $900) - the exact bracket the ledger e2e verifies
        assert!(save.fallen_empires.iter().any(|e| e.lifetime_revenue > 2000));
        assert!(save.fallen_empires.iter().any(|e| e.lifetime_revenue < 2000));
        assert!(!save.dealers[0].character.story_history.is_empty());
        assert_eq!(save.dealers[1].name, "Ray");
        assert!(!save.dealers[1].character.story_history.is_empty());
    }

    #[test]
    fn fronted_scenario_shape() {
        let save = scenario("fronted").unwrap();
        let front = save.front_in(DEFAULT_STATION).expect("front live");
        assert_eq!(front.card_id, "shrooms");
        assert_eq!(front.owed, 125);
        assert_eq!(front.runs_remaining, 3);
        assert_eq!(front.charges, BATCH_SIZE);
        // SOW-034: the batch is in stock and playable, access already granted
        assert_eq!(save.account.charges_in("shrooms"), BATCH_SIZE);
        assert!(save.account.unlocked_cards.contains("shrooms"));
        assert_eq!(save.standing_with(DEFAULT_STATION), SupplierStanding::Good);
        assert_eq!(save.account.cash_on_hand, 60);
    }

    #[test]
    fn strapped_scenario_shape() {
        let save = scenario("strapped").unwrap();
        assert_eq!(save.standing_with(DEFAULT_STATION), SupplierStanding::CutOff);
        assert_eq!(save.front_in(DEFAULT_STATION).unwrap().runs_remaining, 1);
        assert_eq!(save.account.charges_in("shrooms"), BATCH_SIZE); // seized on souring
        assert_eq!(save.account.cash_on_hand, 40); // muscle seizure will be $8
    }

    #[test]
    fn unknown_scenario_is_none() {
        assert!(scenario("nope").is_none());
    }
}
