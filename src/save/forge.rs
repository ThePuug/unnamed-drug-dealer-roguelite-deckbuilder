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

            let mut ray = DealerState::recruit(&save.dealers);
            ray.name = "Ray".to_string();
            ray.character.heat = 45;
            save.dealers.push(ray);

            let mut jailed = DealerState::recruit(&save.dealers);
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
        // SOW-024: enough cash to buy The Block ($2,000), which starts locked
        "mogul" => {
            save.account.cash_on_hand = 3000;
            save.dealers[0].character.heat = 20;
        }
        _ => return None,
    }
    Some(save)
}

/// CLI entry: parse `<scenario> [--dir <path>]`, write the signed save
pub fn run_cli(args: &[String]) {
    let Some(name) = args.first() else {
        eprintln!("usage: forge <fresh|funded|roster|hot|mogul> [--dir <path>]");
        std::process::exit(2);
    };

    let Some(save) = scenario(name) else {
        eprintln!("unknown scenario '{name}' (fresh|funded|roster|hot|mogul)");
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
        for name in ["fresh", "funded", "roster", "hot", "mogul"] {
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
    fn unknown_scenario_is_none() {
        assert!(scenario("nope").is_none());
    }
}
