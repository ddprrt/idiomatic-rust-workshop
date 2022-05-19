use gildedrose::*;

#[test]
pub fn foo() {
    let items = vec![Item::new("foo", 0, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!("foo", rose.items[0].name);
}

#[test]
fn normal_items_decrease_quality() {
    let items = vec![Item::new("+5 Dexterity Vest", 10, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 9);
    assert_eq!(rose.items[0].quality, 19);
}

#[test]
fn quality_degrades_twice_as_fast_after_sellby_date() {
    let items = vec![Item::new("+5 Dexterity Vest", 0, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, -1);
    assert_eq!(rose.items[0].quality, 18);
}

#[test]
fn quality_never_goes_below_zero() {
    let items = vec![Item::new("+5 Dexterity Vest", 0, 1)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, -1);
    assert_eq!(rose.items[0].quality, 0);
}

#[test]
fn aged_brie_increases_quality_as_it_ages() {
    let items = vec![Item::new("Aged Brie", 10, 1)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 9);
    assert_eq!(rose.items[0].quality, 2);
}

#[test]
fn aged_brie_quality_never_exceedes_50() {
    let items = vec![Item::new("Aged Brie", 10, 49)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 8);
    assert_eq!(rose.items[0].quality, 50);
}

#[test]
fn sulfuras_never_changes_sell_in_or_quality() {
    let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 10, 49)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 10);
    assert_eq!(rose.items[0].quality, 49);
}

#[test]
fn backstage_passes_increases_in_quality_by_1_if_sellin_over_10() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        11,
        49,
    )];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 10);
    assert_eq!(rose.items[0].quality, 50);
}

#[test]
fn backstage_passes_increases_in_quality_by_2_if_sellin_under_11_and_over_5() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        10,
        10,
    )];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 9);
    assert_eq!(rose.items[0].quality, 12);
}

#[test]
fn backstage_passes_increases_in_quality_by_3_if_sellin_under_6_and_over_0() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        5,
        10,
    )];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, 4);
    assert_eq!(rose.items[0].quality, 13);
}

#[test]
fn backstage_passes_quality_goes_to_zero_if_sellin_is_0() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        0,
        10,
    )];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].sell_in, -1);
    assert_eq!(rose.items[0].quality, 0);
}
