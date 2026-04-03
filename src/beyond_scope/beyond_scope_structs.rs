// src/beyond_scope/beyond_scope_structs.rs


/// Represents all topics organized by category that the AI should not answer about
#[derive(Debug)]
pub struct BeyondScope {
    pub sports: Vec<BeyondScopeTopic>,
    pub food: Vec<BeyondScopeTopic>,
    pub movies: Vec<BeyondScopeTopic>,
    pub history: Vec<BeyondScopeTopic>,
    pub geography: Vec<BeyondScopeTopic>,
    pub politics: Vec<BeyondScopeTopic>,
    pub science: Vec<BeyondScopeTopic>,
    pub health: Vec<BeyondScopeTopic>,
    pub art: Vec<BeyondScopeTopic>,
    pub music: Vec<BeyondScopeTopic>,
    pub fashion: Vec<BeyondScopeTopic>,
    pub travel: Vec<BeyondScopeTopic>,
    pub pets: Vec<BeyondScopeTopic>,
    pub cars: Vec<BeyondScopeTopic>,
}


/// Represents a topic that the AI should not answer about
#[derive(Debug)]
pub struct BeyondScopeTopic {
    /// The topic value (e.g., "soccer", "olympics")
    pub value: String,
    /// The prefix to use when forming the question (e.g., "is", "is the")
    pub prefix: String,
}



impl Default for BeyondScope {
    fn default() -> Self {
        Self {
            sports: vec![
                BeyondScopeTopic { value: "soccer".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "basketball".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "football".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "baseball".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "tennis".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "golf".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "cricket".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "rugby".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "hockey".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "olympics".to_string(), prefix: "is the".to_string() },
                BeyondScopeTopic { value: "world cup".to_string(), prefix: "is the".to_string() },
                BeyondScopeTopic { value: "super bowl".to_string(), prefix: "is the".to_string() },
            ],
            food: vec![
                BeyondScopeTopic { value: "cooking".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "baking".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "recipes".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "pasta".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "pizza".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "dessert".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "vegetarian".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "vegan".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "wine".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "coffee".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "restaurant".to_string(), prefix: "is a".to_string() },
                BeyondScopeTopic { value: "cuisine".to_string(), prefix: "is".to_string() },
            ],
            movies: vec![
                BeyondScopeTopic { value: "movies".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "tv shows".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "netflix".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "actors".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "actresses".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "directors".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "oscars".to_string(), prefix: "are the".to_string() },
                BeyondScopeTopic { value: "hollywood".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "cinema".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "documentaries".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "animation".to_string(), prefix: "is".to_string() },
            ],
            history: vec![
                BeyondScopeTopic { value: "ancient egypt".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "roman empire".to_string(), prefix: "is the".to_string() },
                BeyondScopeTopic { value: "world war 1".to_string(), prefix: "was".to_string() },
                BeyondScopeTopic { value: "world war 2".to_string(), prefix: "was".to_string() },
                BeyondScopeTopic { value: "civil war".to_string(), prefix: "was the".to_string() },
                BeyondScopeTopic { value: "renaissance".to_string(), prefix: "was the".to_string() },
                BeyondScopeTopic { value: "industrial revolution".to_string(), prefix: "was the".to_string() },
                BeyondScopeTopic { value: "cold war".to_string(), prefix: "was the".to_string() },
                BeyondScopeTopic { value: "middle ages".to_string(), prefix: "were the".to_string() },
            ],
            geography: vec![
                BeyondScopeTopic { value: "countries".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "capitals".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "mountains".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "rivers".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "oceans".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "deserts".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "forests".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "volcanoes".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "islands".to_string(), prefix: "are".to_string() },
            ],
            politics: vec![
                BeyondScopeTopic { value: "president".to_string(), prefix: "is the".to_string() },
                BeyondScopeTopic { value: "elections".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "government".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "democracy".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "senate".to_string(), prefix: "is the".to_string() },
                BeyondScopeTopic { value: "congress".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "supreme court".to_string(), prefix: "is the".to_string() },
                BeyondScopeTopic { value: "taxes".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "immigration".to_string(), prefix: "is".to_string() },
            ],
            science: vec![
                BeyondScopeTopic { value: "biology".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "chemistry".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "physics".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "astronomy".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "evolution".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "dna".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "cells".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "planets".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "black holes".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "climate change".to_string(), prefix: "is".to_string() },
            ],
            health: vec![
                BeyondScopeTopic { value: "medicine".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "diseases".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "exercise".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "nutrition".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "mental health".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "vaccines".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "surgery".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "pharmacy".to_string(), prefix: "is".to_string() },
            ],
            art: vec![
                BeyondScopeTopic { value: "art".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "painting".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "sculpture".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "books".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "poetry".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "novels".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "authors".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "literature".to_string(), prefix: "is".to_string() },
            ],
            music: vec![
                BeyondScopeTopic { value: "rock music".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "jazz".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "classical music".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "hip hop".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "pop music".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "concerts".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "instruments".to_string(), prefix: "are".to_string() },
            ],
            fashion: vec![
                BeyondScopeTopic { value: "fashion".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "clothing".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "shoes".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "designers".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "models".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "runway".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "style".to_string(), prefix: "is".to_string() },
            ],
            travel: vec![
                BeyondScopeTopic { value: "travel".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "hotels".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "flights".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "tourism".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "destinations".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "beaches".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "mountains".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "cities".to_string(), prefix: "are".to_string() },
            ],
            pets: vec![
                BeyondScopeTopic { value: "dogs".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "cats".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "birds".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "fish".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "reptiles".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "pet care".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "training".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "veterinary".to_string(), prefix: "is".to_string() },
            ],
            cars: vec![
                BeyondScopeTopic { value: "cars".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "trucks".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "electric vehicles".to_string(), prefix: "are".to_string() },
                BeyondScopeTopic { value: "racing".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "maintenance".to_string(), prefix: "is".to_string() },
                BeyondScopeTopic { value: "dealerships".to_string(), prefix: "are".to_string() },
            ],
        }
    }
}
