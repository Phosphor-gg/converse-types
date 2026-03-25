use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    Any,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Any
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
            Self::Other => write!(f, "Other"),
            Self::Any => write!(f, "Any"),
        }
    }
}

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            "other" => Ok(Self::Other),
            "any" => Ok(Self::Any),
            _ => Err(format!("Invalid gender: {}", s)),
        }
    }
}

impl Gender {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Male => "Male",
            Self::Female => "Female",
            Self::Other => "Other",
            Self::Any => "Any",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hobby {
    pub name: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRange {
    pub min: u8,
    pub max: u8,
}

impl Default for AgeRange {
    fn default() -> Self {
        Self { min: 13, max: 99 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub discord_id: String,
    pub name: String,
    pub birthday: Option<chrono::NaiveDate>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub profile_picture_url: Option<String>,
    pub about_me: Option<String>,
    pub values: Option<String>,
    pub looking_for: Option<String>,
    pub gender: Option<Gender>,
    pub hobbies: Vec<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl UserProfile {
    pub fn age(&self) -> Option<u8> {
        self.birthday.map(|b| {
            let today = chrono::Local::now().date_naive();
            let years = today.years_since(b).unwrap_or(0) as u8;
            years
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileRequest {
    pub discord_id: String,
    pub name: String,
    pub birthday: Option<chrono::NaiveDate>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub profile_picture_url: Option<String>,
    pub about_me: Option<String>,
    pub values: Option<String>,
    pub looking_for: Option<String>,
    pub gender: Option<Gender>,
    pub hobbies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub profile_picture_url: Option<String>,
    pub about_me: Option<String>,
    pub values: Option<String>,
    pub looking_for: Option<String>,
    pub gender: Option<Gender>,
    pub hobbies: Option<Vec<String>>,
}

pub const AVAILABLE_HOBBIES: &[&str] = &[
    "Reading",
    "Writing",
    "Gaming",
    "Cooking",
    "Photography",
    "Traveling",
    "Hiking",
    "Swimming",
    "Cycling",
    "Running",
    "Yoga",
    "Meditation",
    "Drawing",
    "Painting",
    "Music",
    "Guitar",
    "Piano",
    "Singing",
    "Dancing",
    "Theater",
    "Movies",
    "TV Shows",
    "Anime",
    "Comics",
    "Board Games",
    "Chess",
    "Puzzles",
    "Gardening",
    "Crafting",
    "Knitting",
    "Programming",
    "Technology",
    "Science",
    "Astronomy",
    "History",
    "Languages",
    "Volunteering",
    "Fitness",
    "Rock Climbing",
    "Skiing",
    "Surfing",
    "Fishing",
    "Camping",
    "Bird Watching",
    "Collecting",
    "Fashion",
    "Makeup",
    "DIY Projects",
    "Woodworking",
    "Baking",
    "Sports",
    "Basketball",
    "Football",
    "Soccer",
    "Tennis",
    "Baseball",
    "Volleyball",
    "Martial Arts",
    "Boxing",
    "Wrestling",
    "Archery",
    "Bowling",
    "Golf",
    "Skateboarding",
    "Snowboarding",
];

pub fn is_valid_hobby(hobby: &str) -> bool {
    AVAILABLE_HOBBIES
        .iter()
        .any(|&h| h.eq_ignore_ascii_case(hobby))
}
