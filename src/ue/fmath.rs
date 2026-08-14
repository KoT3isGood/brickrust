
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector2D
{
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FQuat
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FRotator
{
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FColor
{
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}
