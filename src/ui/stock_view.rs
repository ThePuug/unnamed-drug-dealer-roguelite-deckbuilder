// SOW-034: Stock view-model - pure presentation for consumable product stock
// (the shop status line and, in Phase 5, the hand-card badge). Same rule as
// the other _view modules: unit-testable without ECS; systems only orchestrate.

/// The shop status line for a product's on-hand stock. Returns
/// `(label, in_stock)`; 0 charges reads OUT OF STOCK (the caller greys it).
pub fn shop_stock_line(charges: u32) -> (String, bool) {
    if charges == 0 {
        ("OUT OF STOCK".to_string(), false)
    } else {
        (format!("IN STOCK: {charges}"), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shop_line_reads_stock_or_out() {
        assert_eq!(shop_stock_line(4), ("IN STOCK: 4".to_string(), true));
        assert_eq!(shop_stock_line(1), ("IN STOCK: 1".to_string(), true));
        assert_eq!(shop_stock_line(0), ("OUT OF STOCK".to_string(), false));
    }
}
