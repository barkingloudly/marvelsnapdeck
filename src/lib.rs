//! The game Marvel Snap allows sharing decks through the use of encoded strings.
//! This simple crate supports both encoding and decoding of that data to support
//! building other tools on top of the deck information.

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

use base64::DecodeError;
use base64::{engine::general_purpose, Engine as _};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;

/// List of errors returned from a Result
#[derive(Debug, Error)]
pub enum DeckListError {
    /// This error should not occur and likley points to an underlying
    /// issue with string encoding. Make sure passed in data is valid.
    #[error("Failed to encode bytes")]
    EncodingError,

    /// Likely a bad code, this is a common error and should fail gracefully
    #[error("Failed to decode data as base64")]
    DecodingError(#[from] DecodeError),

    /// Likely a bad code, this is a common error and should fail gracefully
    #[error("Invalid data")]
    InvalidDeckInput,
}

/// The game Marvel Snap allows sharing decks through the use of encoded strings.
/// This simple crate supports both encoding and decoding of that data to support
/// building other tools on top of the deck information.
///
/// It does not include actual card data to keep this library simple as cards are
/// added to the pool frequently enough that this would get stale.
///
/// # Example (To Share)
///
/// ```rust
/// use marvelsnapdeck::DeckList;
///
/// let mut list = DeckList::new();
/// list.set_name("Thanos".to_string());
/// list.set_cards(&["AntMan", "Agent13", "Quinjet", "Angela",
/// "Okoye", "Armor", "Falcon", "Mystique", "Lockjaw",
/// "KaZar", "DevilDinosaur", "Thanos"]);
/// let code = list.into_code().unwrap();
/// ```
///
/// # Example (From Code)
///
/// ```no_run
/// use marvelsnapdeck::DeckList;
///
/// let clipboard = "...";
/// let mut list = DeckList::from_code(clipboard);
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckList {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cards")]
    cards: Vec<Card>,
}

/// An individual card
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(rename = "CardDefId")]
    name: String,
}

impl DeckList {
    /// Create an empty DeckList to prepare
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            cards: Default::default(),
        }
    }

    /// Set the deck name visible to the player in game
    ///
    /// # Example
    ///
    /// ```rust
    /// use marvelsnapdeck::DeckList;
    ///
    /// let mut list = DeckList::new();
    /// list.set_name("Thanos".into());
    ///
    /// assert_eq!(list.name(), "Thanos");
    /// ```
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Gets the deck name visible to the player in game
    ///
    /// # Example
    ///
    /// ```rust
    /// use marvelsnapdeck::DeckList;
    ///
    /// let mut list = DeckList::new();
    /// list.set_name("Thanos".into());
    ///
    /// assert_eq!(list.name(), "Thanos");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the list of cards.
    ///  
    /// # Example
    ///
    /// ```rust
    /// use marvelsnapdeck::DeckList;
    ///
    /// let mut list = DeckList::new();
    /// list.set_cards(&["AntMan"]);
    ///
    /// let output = list.cards();
    ///
    /// assert_eq!(output[0], "AntMan");
    /// ```
    pub fn set_cards<T: AsRef<str> + std::fmt::Display>(&mut self, cards: &[T]) {
        let list = cards
            .iter()
            .map(|name| Card {
                name: name.to_string(),
            })
            .collect();
        self.cards = list;
    }

    /// Get list of cards as a vector of strings
    ///
    /// # Example
    ///
    /// ```rust
    /// use marvelsnapdeck::DeckList;
    ///
    /// let mut list = DeckList::new();
    /// list.set_cards(&["AntMan"]);
    ///
    /// let output = list.cards();
    ///
    /// assert_eq!(output[0], "AntMan");
    /// ```
    pub fn cards(&self) -> Vec<String> {
        self.cards.iter().map(|card| card.name.clone()).collect()
    }

    /// Convert a string copied from Marvel Snap into a DeckList.
    ///
    /// # Panics
    ///
    /// Panics if the code cannot be resolved into a valid DeckList struct.
    pub fn from_code<T: AsRef<[u8]>>(code: T) -> Result<Self, DeckListError> {
        let value = general_purpose::STANDARD_NO_PAD
            .decode(code)
            .map_err(|err| DeckListError::DecodingError(err))?;

        let json: DeckList = serde_json::from_slice(value.as_slice())
            .map_err(|_| DeckListError::InvalidDeckInput)?;

        Ok(json)
    }

    /// Converts DeckList into a string for pasting into Marvel Snap
    ///
    /// For a complete deck, make sure to set both the deck name and include 12 valid cards.
    /// For simplicity, this library does not validate if the cards exist in the game.
    ///
    /// # Example
    /// ```rust
    /// use marvelsnapdeck::DeckList;
    ///
    /// let mut list = DeckList::new();
    /// list.set_name("Thanos".to_string());
    /// list.set_cards(&["AntMan", "Agent13", "Quinjet", "Angela",
    /// "Okoye", "Armor", "Falcon", "Mystique", "Lockjaw",
    /// "KaZar", "DevilDinosaur", "Thanos"]);
    /// let code = list.into_code().unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying card list fails to encode as a string
    pub fn into_code(&self) -> Result<String, DeckListError> {
        let data = serde_json::to_string(self).map_err(|_| DeckListError::EncodingError)?;

        let code = general_purpose::STANDARD_NO_PAD.encode(data);

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use crate::DeckList;

    const VALID_CODE: &'static str = "eyJOYW1lIjoiVGhhbm9zIiwiQ2FyZHMiOlt7IkNhcmREZWZJZCI6IkFudE1hbiJ9LHsiQ2FyZERlZklkIjoiQWdlbnQxMyJ9LHsiQ2FyZERlZklkIjoiUXVpbmpldCJ9LHsiQ2FyZERlZklkIjoiQW5nZWxhIn0seyJDYXJkRGVmSWQiOiJPa295ZSJ9LHsiQ2FyZERlZklkIjoiQXJtb3IifSx7IkNhcmREZWZJZCI6IkZhbGNvbiJ9LHsiQ2FyZERlZklkIjoiTXlzdGlxdWUifSx7IkNhcmREZWZJZCI6IkxvY2tqYXcifSx7IkNhcmREZWZJZCI6IkthWmFyIn0seyJDYXJkRGVmSWQiOiJEZXZpbERpbm9zYXVyIn0seyJDYXJkRGVmSWQiOiJUaGFub3MifV19";

    #[test]
    fn decode_is_valid() {
        let list = DeckList::from_code(&VALID_CODE.to_string()).unwrap();
        assert_eq!(list.name(), "Thanos");
        assert_eq!(list.cards.len(), 12);
    }

    #[test]
    fn decode_cards() {
        let list = DeckList::from_code(&VALID_CODE.to_string()).unwrap();
        let cards = list.cards();

        assert_eq!(cards.len(), 12);
        assert_eq!(cards[0], "AntMan");
    }

    #[test]
    fn encode_cards() {
        let mut list = DeckList::new();
        list.set_name("Thanos".to_string());
        list.set_cards(&[
            "AntMan",
            "Agent13",
            "Quinjet",
            "Angela",
            "Okoye",
            "Armor",
            "Falcon",
            "Mystique",
            "Lockjaw",
            "KaZar",
            "DevilDinosaur",
            "Thanos",
        ]);
        let code = list.into_code().unwrap();
        assert_eq!(code, VALID_CODE.to_string());
    }
}
