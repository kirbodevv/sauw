#[derive(Clone)]
pub enum IntOrFloat {
    Int(i32),
    Float(f32),
}

impl IntOrFloat {
    pub fn to_f32(&self) -> f32 {
        match self {
            IntOrFloat::Int(i) => *i as f32 + 0.5,
            IntOrFloat::Float(f) => *f,
        }
    }
}

impl std::str::FromStr for IntOrFloat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('.')
            && let Ok(i) = s.parse::<i32>()
        {
            return Ok(IntOrFloat::Int(i));
        }
        s.parse::<f32>()
            .map(IntOrFloat::Float)
            .map_err(|e| e.to_string())
    }
}
