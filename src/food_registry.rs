use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FoodType {
    Snack,
    Drink,
    #[allow(dead_code)]
    Meal,
}

#[derive(Debug, Clone)]
pub struct FoodDetails {
    pub name: String,
    pub food_type: FoodType,
    #[allow(dead_code)]
    pub calories: u32,
    #[allow(dead_code)]
    pub price: f64,
}

#[derive(Debug)]
pub enum RegistryError {
    #[allow(dead_code)]
    FoodNotFound(String),
    #[allow(dead_code)]
    InvalidInput(String),
    #[allow(dead_code)]
    DuplicateEntry(String),
}

pub struct FoodRegistry {
    foods: HashMap<String, FoodDetails>,
}

impl FoodRegistry {
    pub fn new() -> Self {
        FoodRegistry {
            foods: HashMap::new(),
        }
    }

    pub fn add_food(&mut self, food: FoodDetails) -> Result<(), RegistryError> {
        if self.foods.contains_key(&food.name) {
            return Err(RegistryError::DuplicateEntry(food.name.clone()));
        }
        self.foods.insert(food.name.clone(), food);
        Ok(())
    }

    pub fn get_food(&self, name: &str) -> Result<&FoodDetails, RegistryError> {
        self.foods
            .get(name)
            .ok_or_else(|| RegistryError::FoodNotFound(name.to_string()))
    }

    #[allow(dead_code)]
    pub fn list_by_type(&self, food_type: FoodType) -> Vec<&FoodDetails> {
        self.foods
            .values()
            .filter(|food| matches!((&food.food_type, &food_type), (FoodType::Snack, FoodType::Snack) | (FoodType::Drink, FoodType::Drink) | (FoodType::Meal, FoodType::Meal)))
            .collect()
    }

    #[allow(dead_code)]
    pub fn remove_food(&mut self, name: &str) -> Result<FoodDetails, RegistryError> {
        self.foods
            .remove(name)
            .ok_or_else(|| RegistryError::FoodNotFound(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_food() {
        let mut registry = FoodRegistry::new();
        let snack = FoodDetails {
            name: "Swallow".to_string(),
            food_type: FoodType::Snack,
            calories: 150,
            price: 2.50,
        };
        assert!(registry.add_food(snack).is_ok());
    }

    #[test]
    fn test_duplicate_entry() {
        let mut registry = FoodRegistry::new();
        let snack = FoodDetails {
            name: "Beans".to_string(),
            food_type: FoodType::Snack,
            calories: 200,
            price: 3.00,
        };
        registry.add_food(snack.clone()).unwrap();
        assert!(registry.add_food(snack).is_err());
    }

    #[test]
    fn test_get_food() {
        let mut registry = FoodRegistry::new();
        let drink = FoodDetails {
            name: "Water".to_string(),
            food_type: FoodType::Drink,
            calories: 0,
            price: 1.00,
        };
        registry.add_food(drink).unwrap();
        assert!(registry.get_food("Water").is_ok());
        assert!(registry.get_food("Juice").is_err());
    }
}
