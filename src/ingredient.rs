use crate::consts;
use crate::recipe::RecipeBook;
use anyhow::{bail, Result};
use serde::Deserialize;
use std::{
    collections::{BTreeSet, HashMap},
    fmt,
    fs::File,
    hash::Hash,
    io::BufReader,
    rc::Rc,
};

#[derive(Deserialize)]
pub struct IngredientInfo {
    pub names: Vec<String>,
}

impl IngredientInfo {
    pub fn from(ingredient_name: &str) -> IngredientInfo {
        IngredientInfo { names: vec![String::from(ingredient_name)] }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
// an ingredient used to craft a recipe
pub struct Ingredient {
    /// some ingredients belong to a category
    /// categories are used to determine if one ingredient can be substituted for another
    pub category: Option<String>,
    /// ingredients can have multiple names, but must have at least one
    pub names: BTreeSet<String>,
}

impl Ingredient {
    pub fn from(name: &str) -> Ingredient {
        let mut names = BTreeSet::new();
        names.insert(String::from(name));
        Ingredient { names, category: None }
    }

    pub fn shares_name_with(&self, other_name: &str) -> bool {
        self.names.contains(other_name)
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.names.first() {
            write!(f, "{name}")
        } else {
            write!(f, "")
        }
    }
}

#[derive(Deserialize)]
pub struct IngredientFile(pub HashMap<String, Vec<IngredientInfo>>);

pub const INGREDIENT_NO_CATEGORY_KEY: &str = "others";

pub struct IngredientBook {
    /// path is currently unread, but in the future will be used to distinguish between an in-memory book vs. one saved to disk
    _path: Option<String>,
    /// index of ingredients on their names, we use a RC here as an ingredient can have multiple names
    index: HashMap<String, Rc<Ingredient>>,
}

impl IngredientBook {
    pub fn default() -> IngredientBook {
        let ingredient_file: IngredientFile = serde_yaml::from_str(consts::ingredients::DEFAULT_INGREDIENTS)
            .expect("error deserializing default ingredients file, default file should always be valid");

        let index = IngredientBook::build_index(&ingredient_file).expect("error creating index from default ingredients file, default file should always be valid");

        IngredientBook { _path: None, index }
    }

    pub fn build(path: &str) -> Result<IngredientBook> {
        let ingredient_file: IngredientFile =
            serde_yaml::from_reader(BufReader::new(File::open(path)?))?;

        let index = IngredientBook::build_index(&ingredient_file)?;

        Ok(IngredientBook { _path: Some(String::from(path)), index })
    }

    pub fn from(ingredient_file: IngredientFile) -> Result<IngredientBook> {
        let index = IngredientBook::build_index(&ingredient_file)?;
        Ok(IngredientBook { _path: None, index })
    }

    fn build_index(
        ingredient_file: &IngredientFile,
    ) -> Result<HashMap<String, Rc<Ingredient>>> {
        let mut index = HashMap::new();
        for (category, ingredients) in &ingredient_file.0 {
            for ingredient in ingredients {
                // extract the ingredient fields
                let category = if category == INGREDIENT_NO_CATEGORY_KEY {
                    None
                } else {
                    Some(category.clone())
                };
                // store the ingredient on the heap
                let new_ingredient = Rc::new(Ingredient {
                    category,
                    names: BTreeSet::from_iter(ingredient.names.clone()),
                });

                // update the index to link all the ingredient names to the ingredient
                for name in &new_ingredient.names {
                    if index.contains_key(name) {
                        bail!("Duplicate ingredient name {} found, ingredient names must be unique.", name);
                    }
                    index.insert(name.clone(), Rc::clone(&new_ingredient));
                }
            }
        }

        Ok(index)
    }

    pub fn ingredient(&self, name: &str) -> Option<&Rc<Ingredient>> {
        self.index.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    pub fn names_are_equivalent(&self, name1: &str, name2: &str) -> bool {
        if let Some(ingredient1) = self.ingredient(name1) {
            return ingredient1.shares_name_with(name2);
        }
        if let Some(ingredient2) = self.ingredient(name2) {
            return ingredient2.shares_name_with(name1);
        }

        name1 == name2
    }

    pub fn categories_are_equivalent(
        &self,
        category1: &str,
        category2: &str,
    ) -> bool {
        if let Some(ingredient1) = self.ingredient(category1) {
            // if cat1 is an ingredient name, it's implied that it does not have a category, so match it by name
            return ingredient1.shares_name_with(category2);
        }
        if let Some(ingredient2) = self.ingredient(category2) {
            // if cat2 is an ingredient name, it's implied that it does not have a category, so match it by name
            return ingredient2.shares_name_with(category1);
        }

        category1 == category2
    }

    pub fn expand_names<'a>(
        &self,
        names: impl IntoIterator<Item = &'a String>,
    ) -> BTreeSet<String> {
        names.into_iter().map(|n| self.expand_name(n)).flatten().collect()
    }

    pub fn names_to_categories<'a>(
        &self,
        names: impl IntoIterator<Item = &'a String>,
    ) -> BTreeSet<String> {
        names.into_iter().map(|n| self.name_to_category(n)).collect()
    }

    pub fn expand_name(&self, name: &str) -> BTreeSet<String> {
        let mut names = BTreeSet::new();
        if !self.contains(name) {
            // ingredient name is not found in index, so return only known name
            names.insert(String::from(name));
            return names;
        }

        for name in &self.ingredient(name).unwrap().names {
            names.insert(String::from(name));
        }
        names
    }

    pub fn name_to_category(&self, name: &str) -> String {
        if !self.contains(name) {
            // ingredient name is not found in index, so return name as the category
            return String::from(name);
        }

        if let Some(category) = &self.ingredient(name).unwrap().category {
            // valid category found
            return category.clone();
        }
        // ingredient has no category, use the name as the category
        String::from(name)
    }
}

#[derive(Deserialize)]
/// the pantry stores all the ingredients that the user has access to in order to make recipes
pub struct Pantry {
    /// path is currently unread, but in the future will be used to distinguish between an in-memory pantry vs. one saved to disk
    _path: Option<String>,
    /// unique ingredients in the pantry
    ingredient_names: BTreeSet<String>,
}

impl Pantry {
    pub fn new() -> Pantry {
        Pantry { _path: None, ingredient_names: BTreeSet::new() }
    }

    pub fn build(path: &str) -> Result<Pantry> {
        let pantry: HashMap<String, Vec<String>> =
            serde_yaml::from_reader(BufReader::new(File::open(path)?))?;
        Pantry::build_internal(Some(String::from(path)), pantry)
    }

    pub fn from(pantry: HashMap<String, Vec<String>>) -> Result<Pantry> {
        Pantry::build_internal(None, pantry)
    }

    fn build_internal(
        path: Option<String>,
        pantry: HashMap<String, Vec<String>>,
    ) -> Result<Pantry> {
        let mut ingredient_names = BTreeSet::new();
        for key in pantry.keys() {
            for item in &pantry[key] {
                ingredient_names.insert(item.clone());
            }
        }

        Ok(Pantry { _path: path, ingredient_names })
    }

    pub fn ingredient_names(&self) -> &BTreeSet<String> {
        &self.ingredient_names
    }

    pub fn contains_ingredient(
        &self,
        name: &str,
        ingredient_book: &IngredientBook,
    ) -> bool {
        ingredient_book.expand_names(self.ingredient_names()).contains(name)
    }

    pub fn contains_category(
        &self,
        category: &str,
        ingredient_book: &IngredientBook,
    ) -> bool {
        ingredient_book
            .names_to_categories(self.ingredient_names())
            .contains(category)
    }

    pub fn most_freq_missing_ingredients(
        &self,
        recipe_book: &RecipeBook,
        ingredient_book: &IngredientBook,
        max_ingredients: Option<usize>,
    ) -> Vec<Rc<Ingredient>> {
        let mut missing_counts: HashMap<_, u32> = HashMap::new();

        for recipe in recipe_book.cocktail_recipes() {
            let missing_ingredients =
                recipe.missing_ingredients(self, ingredient_book);
            for missing_ingredient in missing_ingredients {
                // found a recipe we cannot make, so update the missing ingredient counts
                *missing_counts.entry(missing_ingredient).or_default() += 1;
            }
        }

        let mut ordered_missing: Vec<(&Rc<Ingredient>, &u32)> =
            missing_counts.iter().collect();
        ordered_missing.sort_by(|a, b| b.1.cmp(a.1));

        ordered_missing
            .iter()
            .take(max_ingredients.unwrap_or(ordered_missing.len()))
            .map(|i| Rc::clone(i.0))
            .collect()
    }

    pub fn most_freq_missing_categories(
        &self,
        recipe_book: &RecipeBook,
        ingredient_book: &IngredientBook,
        max_categories: Option<usize>,
    ) -> Vec<String> {
        let mut missing_counts: HashMap<_, u32> = HashMap::new();

        for recipe in recipe_book.cocktail_recipes() {
            for missing_category in
                recipe.missing_categories(self, ingredient_book)
            {
                // found a recipe we cannot make, so update the missing ingredient counts
                *missing_counts.entry(missing_category).or_default() += 1;
            }
        }

        let mut ordered_missing: Vec<(&String, &u32)> =
            missing_counts.iter().collect();
        ordered_missing.sort_by(|a, b| b.1.cmp(a.1));

        ordered_missing
            .iter()
            .take(max_categories.unwrap_or(ordered_missing.len()))
            .map(|i| i.0.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::recipe::{DrinkRecipeInfo, RecipeFile};

    use super::*;

    #[test]
    fn test_building_ingredient_index() {
        let ingredients = vec![
            String::from("ingredient1"),
            String::from("ingredient2"),
            String::from("ingredient3"),
            String::from("ingredient4"),
        ];

        let ingredient_file =
            ingredient_file_from_ingredient_names(ingredients);
        let ingredient_book = IngredientBook::from(ingredient_file).unwrap();
        assert!(ingredient_book.contains("ingredient1"));

        let ingredients = vec![
            IngredientInfo {
                names: vec![
                    String::from("ingredient1"),
                    String::from("duplicate"),
                ],
            },
            IngredientInfo {
                names: vec![
                    String::from("ingredient2"),
                    String::from("duplicate"),
                ],
            },
        ];

        let ingredient_file = ingredient_file_from_ingredients(ingredients);
        let ingredient_book = IngredientBook::from(ingredient_file);
        assert!(matches!(ingredient_book, Err(_)));
    }

    fn ingredient_file_from_ingredient_names(
        ingredient_names: Vec<String>,
    ) -> IngredientFile {
        let mut ingredient_categories = HashMap::new();
        let mut ingredients = Vec::new();
        for ingredient_name in &ingredient_names {
            ingredients.push(IngredientInfo::from(ingredient_name));
        }

        ingredient_categories.insert(String::from("others"), ingredients);
        IngredientFile(ingredient_categories)
    }

    fn ingredient_file_from_ingredients(
        ingredients: Vec<IngredientInfo>,
    ) -> IngredientFile {
        let mut ingredient_categories = HashMap::new();
        ingredient_categories.insert(String::from("others"), ingredients);
        IngredientFile(ingredient_categories)
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

    /// Pantry File contains: i1, i2, i3, i4, i5
    /// Ingredient File contains: (i1, i1a): (i1b), (not in pantry): (i1), (not in pantry #2), (also not in pantry)
    ///                           (not in pantry #3): (not in pantry)
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
    fn test_most_freq_missing_ingredients() {
        let pantry: Pantry = generate_test_pantry();
        let recipe_book =
            RecipeBook::from(generate_test_recipe_file()).unwrap();
        let ingredient_book =
            IngredientBook::from(generate_test_ingredient_file()).unwrap();

        let missing_ingredients = pantry.most_freq_missing_ingredients(
            &recipe_book,
            &ingredient_book,
            Some(1),
        );
        assert_eq!(
            missing_ingredients,
            vec![Rc::new(Ingredient::from("missing"))]
        );

        let missing_ingredients = pantry.most_freq_missing_ingredients(
            &recipe_book,
            &ingredient_book,
            Some(20000),
        );
        assert_eq!(
            missing_ingredients,
            vec![
                Rc::new(Ingredient::from("missing")),
                Rc::new(Ingredient::from("also missing")),
                Rc::new(Ingredient::from("missing only once"))
            ]
        );
    }

    fn generate_test_recipe_file() -> RecipeFile {
        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("i1"));
        ingredients.insert(String::from("i2"));
        ingredients.insert(String::from("missing"));
        let drink1 = DrinkRecipeInfo {
            name: String::from("test drink 1"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("i1"));
        ingredients.insert(String::from("missing"));
        ingredients.insert(String::from("also missing"));
        let drink2 = DrinkRecipeInfo {
            name: String::from("test drink 1"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        let mut ingredients = BTreeSet::new();
        ingredients.insert(String::from("i3"));
        ingredients.insert(String::from("i4"));
        ingredients.insert(String::from("missing"));
        ingredients.insert(String::from("also missing"));
        ingredients.insert(String::from("missing only once"));
        let drink3 = DrinkRecipeInfo {
            name: String::from("test drink 1"),
            ingredients,
            instructions: String::from("this is for testing"),
        };

        RecipeFile { cocktails: vec![drink1, drink2, drink3] }
    }
}
