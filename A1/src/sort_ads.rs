// sort_ads.rs
use crate::ad::Ad;

pub fn sort_ads(mut ads: Vec<Ad>) -> Vec<Ad> {
    ads.sort_by(|a, b| {
        a.price.cmp(&b.price).then_with(|| a.title.cmp(&b.title))
    });
    ads
}
