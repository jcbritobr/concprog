use std::fmt::Display;

struct Person {
    name: Option<String>,
    surname: Option<String>,
}

impl Person {
    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn get_surname(&self) -> Option<&str> {
        self.surname.as_deref()
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_deref().unwrap_or("");
        let surname = self.surname.as_deref().unwrap_or("");

        write!(f, "name:{}, surname:{}", name, surname)
    }
}

fn print_data(data: Option<&str>) {
    if let Some(value) = data {
        println!("{value}")
    }
}

fn main() {
    let person = Person {
        name: Some("Julio".to_owned()),
        surname: Some("Brito".to_owned()),
    };
    let name = person.get_name();
    let surname = person.get_surname();
    print_data(name);
    print_data(surname);

    let none = Person {
        name: None,
        surname: None,
    };
    let name = none.get_name();
    let surname = none.get_surname();
    print_data(name);
    print_data(surname);

    println!("{person}");

    println!("end of processing")
}
