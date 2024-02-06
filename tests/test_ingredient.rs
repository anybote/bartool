use bt::ingredient::IngredientBook;

#[test]
fn test_load_default_ingredients_file() {
    let ingredient_book: IngredientBook = Default::default();
    assert!(ingredient_book.contains("rye"));
    assert!(ingredient_book.contains("rye whiskey"));
    assert!(ingredient_book.contains("lemon"));
}
