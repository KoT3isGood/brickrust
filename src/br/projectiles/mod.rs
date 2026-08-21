#![allow(non_snake_case)]

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EAmmoType
{
    Default,
    Incendiary,
    HighExplosive,
    TargetSeeking,
    Guided,
    Flare,
    Max
}

impl EAmmoType
{
    pub fn IsExplosive(&self) -> bool
    {
        use EAmmoType::*;
        match self {
            HighExplosive => true,
            TargetSeeking => true,
            Guided => true,
            Incendiary => true,
            _ => false
        }
    }
}
