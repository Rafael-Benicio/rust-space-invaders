use core::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum HostileType {
    Enemy,
    Shoot,
}

#[derive(PartialEq, Copy, Clone)]
pub enum EntityType {
    Hostile(HostileType),
    Friendily,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntityType::Hostile(HostileType::Enemy) => write!(f, "Entity-Enemy"),
            EntityType::Hostile(HostileType::Shoot) => write!(f, "Entity-Shoot"),
            EntityType::Friendily => write!(f, "Entity-Allay"),
        }
    }
}

impl EntityType {
    pub fn diff(&self, entity: &EntityType) -> bool {
        if ((*self == EntityType::Hostile(HostileType::Enemy)
            || *self == EntityType::Hostile(HostileType::Shoot))
            && *entity == EntityType::Friendily)
            || ((*entity == EntityType::Hostile(HostileType::Enemy)
                || *entity == EntityType::Hostile(HostileType::Shoot))
                && *self == EntityType::Friendily)
        {
            return true;
        }
        false
    }
}
