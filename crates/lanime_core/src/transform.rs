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

    pub fn calculated_transform(&self) -> Position {
        let offset = self.origin.offset(12., 7.);
        Position {
            x: self.position.x - offset.0,
            y: self.position.y - offset.1,
            z: self.position.z,
        }
    }
}

#[derive(Debug)]
pub enum OriginPoint {
    TopLeft,
    Center,
    CustomOffset(fn(f32, f32) -> (f32, f32)),
}

impl OriginPoint {
    pub const DEFAULT: Self = Self::Center;

    #[inline]
    pub fn offset(&self, width: f32, height: f32) -> (f32, f32) {
        match self {
            OriginPoint::TopLeft => (0.0, 0.0),
            OriginPoint::Center => (width / 2., height / 2.),
            OriginPoint::CustomOffset(f) => f(width, height),
        }
    }
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
