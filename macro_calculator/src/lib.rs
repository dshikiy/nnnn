pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let kcal_str = food.calories.1.replace("kcal", "");
        let kcal: f64 = kcal_str.parse().unwrap_or(0.0);

        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    let round_2 = |val: f64| (val * 100.0).round() / 100.0;

    json::object! {
        "cals": round_2(total_cals),
        "carbs": round_2(total_carbs),
        "proteins": round_2(total_proteins),
        "fats": round_2(total_fats)
    }
}
