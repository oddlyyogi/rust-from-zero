//! Prices and labels for Harbor Parcels.

/// Works out the cost of sending a parcel.
///
/// # Examples
///
/// ```
/// let cost = harbor_parcels::shipping_cost(20);
/// assert_eq!(cost, 25);
/// ```
pub fn shipping_cost(weight: u32) -> u32 {
    if weight <= 10 {
        5
    } else {
        5 + (weight - 10) * 2
    }
}

pub fn label(weight: u32) -> String {
    assert!(weight > 0, "weight must not be zero");
    format!("{weight} kg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_parcel_costs_five() {
        assert_eq!(shipping_cost(8), 5);
    }

    #[test]
    fn heavy_parcel() {
        assert_eq!(shipping_cost(20), 25);
    }

    #[test]
    fn label_mentions_kilos() {
        assert!(label(12).contains("kg"));
    }

    #[test]
    #[should_panic(expected = "must not be zero")]
    fn zero_weight_panics() {
        label(0);
    }

    #[test]
    fn parsed_weight() -> Result<(), std::num::ParseIntError> {
        let w: u32 = "12".parse()?;
        assert_eq!(shipping_cost(w), 9);
        Ok(())
    }

    #[test]
    fn boundary() {
        assert_eq!(shipping_cost(10), 5);
        assert_eq!(shipping_cost(11), 7);
    }

    #[test]
    #[ignore = "too slow for every run"]
    fn very_heavy_parcel() {
        assert_eq!(shipping_cost(1000), 1985);
    }
}
