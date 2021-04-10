//The custom enum AnimalType
enum AnimalType {
    Mammal,
    Fish,
    Reptile,
}

#[allow(dead_code)]
//The Animal Struct which uses the AnimalType enum
pub struct Animal {
    type_of_animal: AnimalType,
    name: String,
}

//Simple function to match the type_of_animal_attribute of the Animal Stuct and
//returns a String saying what type of animal it is
fn match_animal(animal: AnimalType) -> String {
    match animal {
        AnimalType::Fish => "this is a fish".to_string(),
        AnimalType::Mammal => "this is a mammal".to_string(),
        AnimalType::Reptile => "this is a reptile".to_string(),
    }
}

//main function

fn main() {
    let _sheep = Animal {
        type_of_animal: AnimalType::Mammal,
        name: String::from("sheep"),
    };
    let _fish: Animal = Animal {
        type_of_animal: AnimalType::Fish,
        name: String::from("fish"),
    };
    let _lizard: Animal = Animal {
        type_of_animal: AnimalType::Reptile,
        name: String::from("lizard"),
    };
    let animal_type = match_animal(_sheep.type_of_animal);
    println!("{}", animal_type)
}
