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

/// The hand-card stock badge for a product with `charges` on hand. Returns
/// `(label, in_stock)`; 0 reads OUT OF STOCK (the caller greys the card too).
pub fn hand_badge(charges: u32) -> (String, bool) {
    if charges == 0 {
        ("OUT OF STOCK".to_string(), false)
    } else {
        (format!("{charges} LEFT"), true)
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

    #[test]
    fn hand_badge_reads_left_or_out() {
        assert_eq!(hand_badge(3), ("3 LEFT".to_string(), true));
        assert_eq!(hand_badge(1), ("1 LEFT".to_string(), true));
        assert_eq!(hand_badge(0), ("OUT OF STOCK".to_string(), false));
    }
}
