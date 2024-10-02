use core::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum HostileType {
    Enemy,
    Shoot,
}

#[derive(PartialEq, Copy, Clone)]
pub enum FriendilyType {
    Player,
    Shoot,
}

#[derive(PartialEq, Copy, Clone)]
pub enum EntityType {
    Hostile(HostileType),
    Friendily(FriendilyType),
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntityType::Hostile(HostileType::Enemy) => write!(f, "Entity-Enemy"),
            EntityType::Hostile(HostileType::Shoot) => write!(f, "Entity-Enemy-Shoot"),
            EntityType::Friendily(FriendilyType::Player) => write!(f, "Entity-Player"),
            EntityType::Friendily(FriendilyType::Shoot) => write!(f, "Entity-Player-Shoot"),
        }
    }
}

impl EntityType {
    pub fn diff(&self, entity_type: &EntityType) -> bool {
        match (self, entity_type) {
            (EntityType::Hostile(_), EntityType::Friendily(_)) => true,
            (EntityType::Friendily(_), EntityType::Hostile(_)) => true,
            _ => false,
        }
    }
}
