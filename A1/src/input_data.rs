// input_data.rs
use crate::ad::Ad;

pub fn get_ads() -> Vec<Ad> {
    vec![
        Ad { title: "Vélo de course".to_string(), price: 100, tags: vec!["Bleu".to_string(), "Rouge".to_string(), "Course".to_string()] },
        Ad { title: "Robot mélangeur".to_string(), price: 10, tags: vec!["Cuisine".to_string(), "Robot".to_string()] },
        Ad { title: "Vélo de course".to_string(), price: 100, tags: vec!["Bleu".to_string(), "Rouge".to_string(), "Course".to_string()] },
        Ad { title: "Robot mélangeur".to_string(), price: 10, tags: vec!["Cuisine".to_string(), "Robot".to_string()] },
        Ad { title: "Vélo de ville".to_string(), price: 50, tags: vec!["Vert".to_string(), "Rouge".to_string(), "Ville".to_string()] },
        Ad { title: "Chaussures (41)".to_string(), price: 5, tags: vec!["Bleu".to_string(), "Rouge".to_string()] },
        Ad { title: "Tapis".to_string(), price: 150, tags: vec!["Blanc".to_string(), "Décoration".to_string()] },
        Ad { title: "Armoire".to_string(), price: 400, tags: vec!["Décoration".to_string(), "Meubles".to_string()] },
        Ad { title: "Scooter électrique".to_string(), price: 1000, tags: vec!["Blanc".to_string(), "Scooter".to_string()] },
        Ad { title: "Pots de peinture (don)".to_string(), price: 0, tags: vec![] },
        Ad { title: "Boîtes à thé".to_string(), price: 5, tags: vec!["Rangements".to_string()] },
    ]
}
