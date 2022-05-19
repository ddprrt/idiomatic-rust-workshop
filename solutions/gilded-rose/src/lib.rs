use std::{
    convert::Infallible,
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Default)]
pub struct Item {
    kind: ItemType,
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        let name = name.into();
        Item {
            kind: name.clone().as_str().parse().unwrap(),
            name,
            sell_in,
            quality,
        }
    }

    pub fn update_quality(&mut self) {
        let (sell_in, quality) = self.kind.update_quality(self.sell_in, self.quality);
        self.sell_in = sell_in;
        self.quality = quality;
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

enum ItemType {
    Normal,
    AgedBrie,
    Backstage,
    Sulfuras,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Normal
    }
}

impl ItemType {
    pub fn update_quality(&self, sell_in: i32, quality: i32) -> (i32, i32) {
        match self {
            ItemType::AgedBrie => (
                sell_in - 1,
                if quality < 50 { quality + 1 } else { quality },
            ),
            ItemType::Backstage => (
                sell_in - 1,
                match sell_in {
                    1..=5 => quality + 3,
                    6..=10 => quality + 2,
                    0 => 0,
                    _ => quality + 1,
                },
            ),
            ItemType::Sulfuras => (sell_in, quality),
            ItemType::Normal => (
                sell_in - 1,
                if sell_in < 1 && quality > 1 {
                    quality - 2
                } else if quality > 0 {
                    quality - 1
                } else {
                    0
                },
            ),
        }
    }
}

impl From<String> for ItemType {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "Aged Brie" => ItemType::AgedBrie,
            "Sulfuras, Hand of Ragnaros" => ItemType::Sulfuras,
            "Backstage passes to a TAFKAL80ETC concert" => ItemType::Backstage,
            _ => ItemType::Normal,
        }
    }
}

impl FromStr for ItemType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(String::from(s).into())
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            item.update_quality()
        }
    }
}
