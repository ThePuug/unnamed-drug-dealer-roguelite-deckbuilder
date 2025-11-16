// SOW-013-B Phase 2: Buyer personas from assets
// Loads personas from GameAssets registry

use crate::models::buyer::BuyerPersona;
use crate::assets::GameAssets;

/// SOW-013-B: Get all available Buyer personas from loaded assets (3 personas)
pub fn create_buyer_personas(assets: &GameAssets) -> Vec<BuyerPersona> {
    assets.buyers.clone()
}
