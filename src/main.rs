/*
Niveau 1 - Bases de la syntaxe :

Créez une structure BitcoinAddress qui contient une chaîne de caractères
représentant l'adresse. Implémentez les traits Display et Debug pour cette structure.
Écrivez une fonction qui vérifie si une adresse Bitcoin (sous forme de String) est valide
(pour simplifier, vérifiez juste qu'elle commence par "1", "3" ou "bc1" et a une longueur appropriée).
*/

use std::fmt;

struct BitcoinAddress {
    value: String,
}
impl BitcoinAddress {
    fn is_valid(&self) -> bool {
        if self.value.chars().count() <= 0 {
            return false;
        }

        match self.value.chars().nth(0).unwrap() {
            '1' => return true,
            '3' => return true,
            _ => {
                return &self.value[..3] == "bc1";
            }
        }
    }
}
impl fmt::Display for BitcoinAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitcoin address: {}", self.value)
    }
}
impl fmt::Debug for BitcoinAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Address {} {}",
            self.value,
            if self.is_valid() {
                "is valid"
            } else {
                "is not valid"
            }
        )
    }
}

fn main() {
    // Valid cases
    let valid_address1 = BitcoinAddress {
        value: String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
    };
    let valid_address2 = BitcoinAddress {
        value: String::from("3A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
    };
    let valid_address3 = BitcoinAddress {
        value: String::from("bc1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
    };
    std::assert_eq!(valid_address1.is_valid(), true);
    std::assert_eq!(valid_address2.is_valid(), true);
    std::assert_eq!(valid_address3.is_valid(), true);
    println!("{:?}", valid_address1);
    println!("{:?}", valid_address2);
    println!("{:?}", valid_address3);
    // Invalid case
    let invalid_address1 = BitcoinAddress {
        value: String::from(""),
    };
    let invalid_address2 = BitcoinAddress {
        value: String::from("bc2eojfoejfodjoejvodjvovzoe"),
    };
    std::assert_eq!(invalid_address1.is_valid(), false);
    std::assert_eq!(invalid_address2.is_valid(), false);
    println!("{:?}", invalid_address1);
    println!("{:?}", invalid_address2);
}
