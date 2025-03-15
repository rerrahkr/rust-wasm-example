use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Supporter {
    pub name: String,
    pub supporting_club: String,
    pub year: u32,
}

#[wasm_bindgen]
impl Supporter {
    pub fn say(&self) -> String {
        format!(
            "{} has been a supporter of {} since {}!",
            self.name, self.supporting_club, self.year
        )
    }
}

#[wasm_bindgen]
pub fn become_supporter(name: &str, year: u32) -> Supporter {
    Supporter {
        name: String::from(name),
        supporting_club: String::from("Vissel Kobe"),
        year,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supporter_say() {
        let supporter = Supporter {
            name: String::from("Taro"),
            supporting_club: String::from("INAC Kobe"),
            year: 2011,
        };

        assert_eq!(
            supporter.say(),
            "Taro has been a supporter of INAC Kobe since 2011!"
        );
    }

    #[test]
    fn test_become_supporter() {
        let supporter = become_supporter("Hanako", 2006);

        assert_eq!(supporter.name, "Hanako");
        assert_eq!(supporter.supporting_club, "Vissel Kobe");
        assert_eq!(supporter.year, 2006);
    }
}
