mod craft_world;

use craft_world::america;
use craft_world::china;
use craft_world::russia;

fn main() {
    china::tell_china_crafts();
    println!("----------------");
    america::tell_america_crafts();
    println!("----------------");
    russia::tell_russia_crafts();
}
