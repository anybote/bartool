use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(flatten)]
    pub lookup_opts: LookupOpts,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// These ingredients must be present in resulting recipe
    pub required_ingredients: Vec<String>,
}

//
// Common arguments that are used throughout the various commands and subcommands
//
#[derive(Args)]
pub struct LookupOpts {
    /// path to the pantry file
    #[arg(short = 'p', long = "pantry")]
    pub pantry_path: Option<String>,

    /// whether or not alternate ingredients should be included in the search
    #[arg(short = 'a', long = "alts", default_value_t = false)]
    pub include_alt_ingredients: bool,

    /// path to the ingredients file
    #[arg(
        short = 'i',
        long = "ingredients",
        default_value = "ingredients.yaml"
    )]
    pub ingredients_path: String,

    /// path to the recipes file
    #[arg(short = 'r', long = "recipes", default_value = "recipes.yaml")]
    pub recipes_path: String,

    /// maximum number of items returned
    #[arg(short = 'c', long = "count")]
    pub count: Option<usize>,
}

//
// The following are subcommand definitions
//
#[derive(Subcommand)]
pub enum Command {
    /// write a starter file to disk if you don't have one or want to modify the built-in. bt init -h for information about each file.
    Init {
        #[command(subcommand)]
        command: Option<InitCommand>,
    },
    /// return a list of ingredients not found in your pantry, ordered by how frequently they appear in recipes. bt missing -h for more information.
    Missing {
        #[command(flatten)]
        lookup_opts: LookupOpts,
    },
}

#[derive(Subcommand)]
pub enum InitCommand {
    /// the pantry file contains all the ingredients you have at home and can be used for convenient filtering
    Pantry,
    /// the recipes file contains the full corpus of recipes including their name, required ingredients, and instructions
    Recipes,
    /// the ingredients file contains metadata about ingredients including synonymous ingredient names and possible alternates
    Ingredients,
}
