use anyhow::{bail, Result};
use bt::{
    cli::{Arguments, Command, InitCommand},
    consts,
    ingredient::{IngredientBook, Pantry},
    recipe::RecipeBook,
};
use clap::Parser;
use std::{collections::BTreeSet, fs, path::Path};

fn main() -> Result<()> {
    let args = Arguments::parse();
    match args.command {
        // default bt command, used for searching recipes using the pantry file and / or required ingredients
        None => {
            let pantry;
            let pantry_opt = if let Some(pantry_path) =
                args.lookup_opts.pantry_path
            {
                pantry = match Pantry::build(&pantry_path) {
                    Ok(pantry) => pantry,
                    Err(_) => bail!("pantry file not found at {pantry_path}"),
                };
                Some(&pantry)
            } else {
                None
            };

            let recipe_book = init_recipe_book(&args.lookup_opts.recipes_path);

            let results = recipe_book.possible_drink_recipes(
                pantry_opt,
                &BTreeSet::from_iter(args.required_ingredients),
                &init_ingredient_book(&args.lookup_opts.ingredients_path),
                args.lookup_opts.include_alt_ingredients,
                args.lookup_opts.count,
            );

            if results.is_empty() {
                println!("# no results found...");
            }
            for recipe in results {
                println!("{recipe}");
            }
        }
        // a subcommand was used
        Some(command) => match command {
            // initialize a file and write to the local directory
            Command::Init { command: cmd } => match cmd {
                Some(InitCommand::Pantry) => write_default_pantry_file()?,
                Some(InitCommand::Ingredients) => {
                    write_default_ingredients_file()?;
                }
                Some(InitCommand::Recipes) => write_default_recipes_file()?,
                None => {
                    write_default_pantry_file()?;
                    write_default_ingredients_file()?;
                    write_default_recipes_file()?;
                }
            },
            // lookup ingredients that are not in the pantry that enable you to make the most recipes
            Command::Missing { lookup_opts } => {
                let pantry = if let Some(pantry_path) = lookup_opts.pantry_path
                {
                    match Pantry::build(&pantry_path) {
                        Ok(pantry) => pantry,
                        Err(_) => {
                            bail!("pantry file not found at {pantry_path}")
                        }
                    }
                } else {
                    bail!("a pantry file is required for this command!")
                };

                let missing_ingredients = if lookup_opts.include_alt_ingredients
                {
                    pantry.most_freq_missing_categories(
                        &init_recipe_book(&lookup_opts.recipes_path),
                        &init_ingredient_book(&lookup_opts.ingredients_path),
                        lookup_opts.count,
                    )
                } else {
                    pantry
                        .most_freq_missing_ingredients(
                            &init_recipe_book(&lookup_opts.recipes_path),
                            &init_ingredient_book(
                                &lookup_opts.ingredients_path,
                            ),
                            lookup_opts.count,
                        )
                        .iter()
                        .map(|i| format!("{i}"))
                        .collect()
                };

                if missing_ingredients.is_empty() {
                    println!("# you're not missing any ingredients, you can make every recipe in your recipe book")
                } else {
                    println!("# top missing ingredients:\n");
                }

                for missing_ingredient in missing_ingredients {
                    println!("{missing_ingredient}");
                }
            }
        },
    };

    Ok(())
}

fn init_ingredient_book(path: &str) -> IngredientBook {
    if let Ok(ingredient_book) = IngredientBook::build(path) {
        println!("# loaded ingredient file from {path}...");
        ingredient_book
    } else {
        let ingredient_book = Default::default();
        println!("# loaded default ingredient file...");
        ingredient_book
    }
}

fn init_recipe_book(path: &str) -> RecipeBook {
    if let Ok(recipe_book) = RecipeBook::build(path) {
        println!("# loaded recipes file from {path}...");
        recipe_book
    } else {
        let recipe_book = Default::default();
        println!("# loaded default recipes file...");
        recipe_book
    }
}

fn write_default_pantry_file() -> Result<()> {
    // if pantry.yaml dne, write default pantry, else bail
    if Path::new(consts::pantry::DEFAULT_PATH).exists() {
        bail!("pantry file {} already exists!", consts::pantry::DEFAULT_PATH);
    }
    fs::write(consts::pantry::DEFAULT_PATH, consts::pantry::DEFAULT_PANTRY)?;
    Ok(())
}

fn write_default_ingredients_file() -> Result<()> {
    // if pantry.yaml dne, write default pantry, else bail
    if Path::new(consts::ingredients::DEFAULT_PATH).exists() {
        bail!(
            "ingredients file {} already exists!",
            consts::ingredients::DEFAULT_PATH
        );
    }
    fs::write(
        consts::ingredients::DEFAULT_PATH,
        consts::ingredients::DEFAULT_INGREDIENTS,
    )?;
    Ok(())
}

fn write_default_recipes_file() -> Result<()> {
    // if pantry.yaml dne, write default pantry, else bail
    if Path::new(consts::recipes::DEFAULT_PATH).exists() {
        bail!("recipes file {} already exists!", consts::recipes::DEFAULT_PATH);
    }
    fs::write(consts::recipes::DEFAULT_PATH, consts::recipes::DEFAULT_RECIPES)?;
    Ok(())
}
