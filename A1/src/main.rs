// main.rs
mod ad;
mod sort_ads;
mod input_data;

use crate::sort_ads::sort_ads;
use crate::input_data::get_ads;
fn main() {

    let ads = get_ads();
    let sorted_ads = sort_ads(ads);

    println!("{:?}", sorted_ads)
}
