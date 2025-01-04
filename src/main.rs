/*
Niveau 1 - Bases de la syntaxe :

Créez une structure BitcoinAddress qui contient une chaîne de caractères
représentant l'adresse. Implémentez les traits Display et Debug pour cette structure.
Écrivez une fonction qui vérifie si une adresse Bitcoin (sous forme de String) est valide
(pour simplifier, vérifiez juste qu'elle commence par "1", "3" ou "bc1" et a une longueur appropriée).
*/

use std::fmt;

const LEGACY_PREFIX: char = '1';
const P2SH_PREFIX: char = '3';
const BECH32_PREFIX: &str = "bc1";

struct BitcoinAddress {
    value: String,
}
impl BitcoinAddress {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    fn is_valid(&self) -> bool {
        if self.value.is_empty() {
            return false;
        }

        self.value
            .chars()
            .next()
            .map_or(false, |first_char| match first_char {
                LEGACY_PREFIX | P2SH_PREFIX => true,
                _ => self.value.starts_with(BECH32_PREFIX),
            })
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

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_addresses() {
        let addresses = vec![
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "3A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "bc1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        ];

        for addr in addresses {
            assert!(BitcoinAddress::new(addr.to_string()).is_valid());
        }
    }

    #[test]
    fn test_invalid_addresses() {
        let addresses = vec!["", "bc2eojfoejfodjoejvodjvovzoe"];

        for addr in addresses {
            assert!(!BitcoinAddress::new(addr.to_string()).is_valid());
        }
    }
}
