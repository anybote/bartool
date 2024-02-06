use crate::consts;
use crate::ingredient::{Ingredient, IngredientBook, Pantry};
use anyhow::Result;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::{cmp, collections::BTreeSet, fmt, fs::File, io::BufReader};

#[derive(Deserialize)]
/// the drink recipe info is how the drink recipe is represented on disk
pub struct DrinkRecipeInfo {
    /// unique name for the recipe
    pub name: String,
    /// names of the ingredients required to make the recipe
    pub ingredients: BTreeSet<String>,
    /// instructions for the reader
    pub instructions: String,
}

#[derive(
    Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone, Ord, PartialOrd,
)]
/// a drink recipe contains everything needed to instruct the user how to craft a recipe
pub struct DrinkRecipe {
    /// unique name for the recipe
    pub name: String,
    /// names of the ingredients required to make the recipe
    pub ingredient_names: BTreeSet<String>,
    /// instructions for the reader
    pub instructions: String,
}

impl DrinkRecipe {
    pub fn from(drink_recipe_info: DrinkRecipeInfo) -> DrinkRecipe {
        DrinkRecipe {
            name: drink_recipe_info.name,
            ingredient_names: drink_recipe_info.ingredients,
            instructions: drink_recipe_info.instructions,
        }
    }

    pub fn from_all(
        drink_recipe_infos: impl IntoIterator<Item = DrinkRecipeInfo>,
    ) -> Vec<DrinkRecipe> {
        drink_recipe_infos.into_iter().map(DrinkRecipe::from).collect()
    }

    pub fn contains_required_ingredients(
        &self,
        required_names: &BTreeSet<String>,
        ingredient_book: &IngredientBook,
    ) -> bool {
        'required_ingredients: for required_name in required_names {
            for name in &self.ingredient_names {
                if ingredient_book.names_are_equivalent(required_name, name) {
                    continue 'required_ingredients;
                }
            }
            return false;
        }

        true
    }

    pub fn contains_required_categories(
        &self,
        required_categories: &BTreeSet<String>,
        ingredient_book: &IngredientBook,
    ) -> bool {
        'required_categories: for required_category in required_categories {
            for name in &self.ingredient_names {
                let my_category = &ingredient_book.name_to_category(name);
                if ingredient_book
                    .categories_are_equivalent(my_category, required_category)
                {
                    continue 'required_categories;
                }
            }
            return false;
        }

        true
    }

    pub fn can_make(
        &self,
        pantry: &Pantry,
        ingredient_book: &IngredientBook,
        use_alt_ingredients: bool,
    ) -> bool {
        if use_alt_ingredients {
            // ingredients in recipe are subset of pantry ingredients, but replace both with valid categories
            let transformed_pantry_ingredients =
                ingredient_book.names_to_categories(pantry.ingredient_names());
            let transformed_recipe_ingredients =
                ingredient_book.names_to_categories(&self.ingredient_names);

            return transformed_recipe_ingredients
                .is_subset(&transformed_pantry_ingredients);
        }

        // ingredients in recipe are subset of pantry, need to expand pantry to consider alternate names
        let expanded_pantry_ingredients =
            ingredient_book.expand_names(pantry.ingredient_names());

        self.ingredient_names.is_subset(&expanded_pantry_ingredients)
    }

    pub fn missing_ingredients(
        &self,
        pantry: &Pantry,
        ingredient_book: &IngredientBook,
    ) -> BTreeSet<Rc<Ingredient>> {
        let mut missing_names = BTreeSet::new();
        for name in &self.ingredient_names {
            if !pantry.contains_ingredient(name, ingredient_book) {
                missing_names.insert(name.clone());
            }
        }

        missing_names
            .iter()
            .map(|name| {
                if let Some(ingredient) = ingredient_book.ingredient(name) {
                    Rc::clone(ingredient)
                } else {
                    Rc::new(Ingredient::from(name))
                }
            })
            .collect()
    }

    pub fn missing_categories(
        &self,
        pantry: &Pantry,
        ingredient_book: &IngredientBook,
    ) -> BTreeSet<String> {
        let mut missing_categories = BTreeSet::new();
        for name in &self.ingredient_names {
            let category = ingredient_book.name_to_category(name);
            if !pantry.contains_category(&category, ingredient_book) {
                missing_categories.insert(category);
            }
        }

        missing_categories
    }
}

impl fmt::Display for DrinkRecipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\
\"{}\":
  ingredients:
{}
  recipe: {}",
            self.name,
            format_ingredients(&self.ingredient_names),
            format_instructions(&self.instructions)
        )
    }
}

fn format_ingredients<'a>(
    names: impl IntoIterator<Item = &'a String>,
) -> String {
    let mut ingredients = String::new();
    for name in names {
        ingredients.push_str("    - ");
        ingredients.push_str(name);
        ingredients.push('\n');
    }

    ingredients
}

fn format_instructions(instructions: &str) -> String {
    let mut formatted_instructions = String::from(instructions);
    if instructions.trim().contains('\n') {
        formatted_instructions = String::new();
        formatted_instructions.push_str(" |\n");
        for line in instructions.lines() {
            formatted_instructions.push_str("    ");
            formatted_instructions.push_str(line);
            formatted_instructions.push('\n');
        }
    }

    formatted_instructions
}

#[derive(Deserialize)]
/// the recipe file is how a collection of recipes is represented on disk
pub struct RecipeFile {
    /// a collection of drink recipes for making cocktails
    pub cocktails: Vec<DrinkRecipeInfo>,
}

pub struct RecipeBook {
    /// path is currently unread, but in the future will be used to distinguish between an in-memory book vs. one saved to disk
    _path: Option<String>,
    /// a collection of drink recipes for making cocktails
    cocktails: Vec<DrinkRecipe>,
}

impl RecipeBook {
    pub fn build(path: &str) -> Result<RecipeBook> {
        let recipe_file: RecipeFile =
            serde_yaml::from_reader(BufReader::new(File::open(path)?))?;

        Ok(RecipeBook {
            _path: Some(String::from(path)),
            cocktails: DrinkRecipe::from_all(recipe_file.cocktails),
        })
    }

    pub fn from(recipe_file: RecipeFile) -> Result<RecipeBook> {
        Ok(RecipeBook {
            _path: None,
            cocktails: DrinkRecipe::from_all(recipe_file.cocktails),
        })
    }

    pub fn cocktail_recipes(&self) -> &Vec<DrinkRecipe> {
        &self.cocktails
    }

    pub fn possible_drink_recipes(
        &self,
        pantry: Option<&Pantry>,
        required_ingredients: &BTreeSet<String>,
        ingredient_book: &IngredientBook,
        use_alt_ingredients: bool,
        max_recipes: Option<usize>,
    ) -> BTreeSet<&DrinkRecipe> {
        let mut recipes = Vec::new();
        for drink_recipe in &self.cocktails {
            // check for required ingredients
            if (use_alt_ingredients
                && drink_recipe.contains_required_categories(
                    &ingredient_book.names_to_categories(required_ingredients),
                    ingredient_book,
                ))
                || (!use_alt_ingredients
                    && drink_recipe.contains_required_ingredients(
                        required_ingredients,
                        ingredient_book,
                    ))
            {
                if let Some(pantry) = pantry {
                    // if a pantry is passed, also check to see if the recipe can be made with the given pantry
                    if !drink_recipe.can_make(
                        pantry,
                        ingredient_book,
                        use_alt_ingredients,
                    ) {
                        continue;
                    }
                }
                recipes.push(drink_recipe);
            }
        }

        if let Some(max_recipes) = max_recipes {
            if max_recipes == 1 {
                let recipe_index =
                    rand::thread_rng().gen_range(0..recipes.len());
                let mut recipe = BTreeSet::new();
                recipe.insert(recipes[recipe_index]);
                return recipe;
            }

            recipes.shuffle(&mut rand::thread_rng());
            return BTreeSet::from_iter(
                recipes[..cmp::min(max_recipes, recipes.len())].to_vec(),
            );
        }

        BTreeSet::from_iter(recipes)
    }
}

impl Default for RecipeBook {
    fn default() -> RecipeBook {
        let recipe_file: RecipeFile = serde_yaml::from_str(consts::recipes::DEFAULT_RECIPES)
            .expect("error deserializing default recipes file, default file should always be valid");

        RecipeBook {
            _path: None,
            cocktails: DrinkRecipe::from_all(recipe_file.cocktails),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashMap};

    use crate::ingredient::{
        IngredientBook, IngredientFile, IngredientInfo,
        INGREDIENT_NO_CATEGORY_KEY,
    };

    use super::*;

    #[test]
    fn test_contains_required_ingredient() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("i2"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };

        // ingredient names
        assert!(recipe.contains_required_ingredients(
            &BTreeSet::from_iter(vec![String::from("i1")]),
            &ingredient_book
        ));
        assert!(recipe.contains_required_ingredients(
            &BTreeSet::from_iter(vec![String::from("i1"), String::from("i2")]),
            &ingredient_book
        ));
        assert!(!recipe.contains_required_ingredients(
            &BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("not found")
            ]),
            &ingredient_book
        ));

        // other names
        assert!(recipe.contains_required_ingredients(
            &BTreeSet::from_iter(vec![String::from("i1a")]),
            &ingredient_book
        ));
        assert!(!recipe.contains_required_ingredients(
            &BTreeSet::from_iter(vec![
                String::from("i1a"),
                String::from("not in pantry")
            ]),
            &ingredient_book
        ));

        // use alt ingredients
        assert!(recipe.contains_required_categories(
            &BTreeSet::from_iter(vec![String::from("cat1")]),
            &ingredient_book
        ));
    }

    fn generate_test_ingredient_file() -> IngredientFile {
        let names = vec![String::from("i1"), String::from("i1a")];
        let ingredient1 = IngredientInfo { names };

        let names = vec![String::from("not in pantry")];
        let ingredient2 = IngredientInfo { names };

        let names = vec![String::from("i2")];
        let ingredient3 = IngredientInfo { names };

        let names = vec![String::from("not in pantry #3")];
        let ingredient4 = IngredientInfo { names };

        let mut ingredient_categories = HashMap::new();
        ingredient_categories
            .insert(String::from("cat1"), vec![ingredient1, ingredient2]);
        ingredient_categories.insert(String::from("cat2"), vec![ingredient3]);
        ingredient_categories.insert(
            String::from(INGREDIENT_NO_CATEGORY_KEY),
            vec![ingredient4],
        );

        IngredientFile(ingredient_categories)
    }

    #[test]
    fn test_can_make_recipe() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();
        let pantry = generate_test_pantry();

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("i2"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert!(recipe.can_make(&pantry, &ingredient_book, false));
        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![String::from("i1a")]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert!(recipe.can_make(&pantry, &ingredient_book, false));

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("not in pantry"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert!(!recipe.can_make(&pantry, &ingredient_book, false));

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("not in pantry #3"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert!(!recipe.can_make(&pantry, &ingredient_book, false));
    }

    #[test]
    fn test_can_make_recipe_with_alts() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();
        let pantry = generate_test_pantry();

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("not in pantry"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert!(recipe.can_make(&pantry, &ingredient_book, true));

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("not in pantry"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert!(!recipe.can_make(&pantry, &ingredient_book, false));
    }

    fn generate_test_pantry() -> Pantry {
        let mut pantry = HashMap::new();
        let ingredient_names1 =
            vec![String::from("i1"), String::from("i2"), String::from("i3")];
        let ingredient_names2 = vec![String::from("i4"), String::from("i5")];
        pantry.insert(String::from("heading 1"), ingredient_names1);
        pantry.insert(String::from("heading 2"), ingredient_names2);

        Pantry::from(pantry).unwrap()
    }

    #[test]
    fn test_missing_ingredients() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();
        let pantry = generate_test_pantry();

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("i2"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert_eq!(
            recipe.missing_ingredients(&pantry, &ingredient_book),
            BTreeSet::new()
        );

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("i1"),
                String::from("missing"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert_eq!(
            recipe.missing_ingredients(&pantry, &ingredient_book),
            BTreeSet::from_iter(vec![Rc::new(Ingredient::from("missing"))])
        );
    }

    #[test]
    fn test_missing_ingredients_with_alts() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();
        let pantry = generate_test_pantry();

        let recipe = DrinkRecipe {
            name: String::from("test"),
            ingredient_names: BTreeSet::from_iter(vec![
                String::from("not in pantry"),
                String::from("missing"),
            ]),
            instructions: String::from("drink recipe used for testing"),
        };
        assert_eq!(
            recipe.missing_categories(&pantry, &ingredient_book),
            BTreeSet::from_iter(vec![String::from("missing")])
        );
    }

    #[test]
    fn test_possible_drink_recipes() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();
        let pantry = generate_test_pantry();
        let recipe_book =
            RecipeBook::from(generate_test_recipe_file()).unwrap();

        // get all recipes
        let recipes =
            DrinkRecipe::from_all(generate_test_recipe_file().cocktails);
        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                None,
                &BTreeSet::new(),
                &ingredient_book,
                false,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::from_iter(recipes.clone()));

        // filter by pantry
        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                Some(&pantry),
                &BTreeSet::new(),
                &ingredient_book,
                false,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::from_iter(recipes[..2].to_vec()));

        // filter by pantry and required ingredients
        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                Some(&pantry),
                &BTreeSet::from_iter(vec![String::from("i2")]),
                &ingredient_book,
                false,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::from_iter(recipes[..1].to_vec()));

        // filter by pantry and required ingredients as an alternate name
        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                Some(&pantry),
                &BTreeSet::from_iter(vec![String::from("i1a")]),
                &ingredient_book,
                false,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::from_iter(recipes[..2].to_vec()));
    }

    #[test]
    fn test_possible_drink_recipe_with_alts() {
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();
        let pantry = generate_test_pantry();
        let recipe_book =
            RecipeBook::from(generate_test_recipe_file()).unwrap();

        // filter by required and include alts
        let recipes =
            DrinkRecipe::from_all(generate_test_recipe_file().cocktails);
        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                None,
                &BTreeSet::from_iter(vec![String::from("not in pantry")]),
                &ingredient_book,
                false,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::from_iter(recipes[3..4].to_vec()));

        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                None,
                &BTreeSet::from_iter(vec![String::from("not in pantry")]),
                &ingredient_book,
                true,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        let mut expected = BTreeSet::new();
        expected.insert(recipes[0].clone());
        expected.insert(recipes[1].clone());
        expected.insert(recipes[3].clone());
        assert_eq!(result, expected);

        // filter by pantry and required, include alts
        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                Some(&pantry),
                &BTreeSet::from_iter(vec![String::from("not in pantry")]),
                &ingredient_book,
                false,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::new());

        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                Some(&pantry),
                &BTreeSet::from_iter(vec![String::from("not in pantry")]),
                &ingredient_book,
                true,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        let mut expected = BTreeSet::new();
        expected.insert(recipes[0].clone());
        expected.insert(recipes[1].clone());
        expected.insert(recipes[3].clone());
        assert_eq!(result, expected);

        let result: BTreeSet<DrinkRecipe> = recipe_book
            .possible_drink_recipes(
                Some(&pantry),
                &BTreeSet::from_iter(vec![String::from("not in pantry #3")]),
                &ingredient_book,
                true,
                Some(10),
            )
            .iter()
            .map(|r| (*r).clone())
            .collect();
        assert_eq!(result, BTreeSet::new());
    }

    fn generate_test_recipe_file() -> RecipeFile {
        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("i1"));
        ingredients.insert(String::from("i2"));
        let drink1 = DrinkRecipeInfo {
            name: String::from("test drink 1"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("i1"));
        let drink2 = DrinkRecipeInfo {
            name: String::from("test drink 2"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("i3"));
        ingredients.insert(String::from("i4"));
        ingredients.insert(String::from("missing"));
        let drink3 = DrinkRecipeInfo {
            name: String::from("test drink 3"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("not in pantry"));
        let drink4 = DrinkRecipeInfo {
            name: String::from("test drink 4"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        RecipeFile { cocktails: vec![drink1, drink2, drink3, drink4] }
    }
}
