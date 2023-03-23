use lanime_bindfields::BindFields;

#[derive(Debug, BindFields)]
pub struct Transform {
    pub origin: OriginPoint,
    pub position: Position,
}

impl Transform {
    pub const DEFAULT: Self = Self {
        origin: OriginPoint::DEFAULT,
        position: Position::DEFAULT,
    };
}

#[derive(Debug)]
pub enum OriginPoint {
    TopLeft,
    Center,
    CustomRelative(i32, i32),
    CustomPercentage(f32, f32),
}

impl OriginPoint {
    pub const DEFAULT: Self = Self::Center;
}

#[derive(Debug, BindFields)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub const DEFAULT: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };
}
