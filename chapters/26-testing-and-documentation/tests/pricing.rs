use harbor_parcels::shipping_cost;

#[test]
fn from_the_outside() {
    assert_eq!(shipping_cost(30), 45);
}
