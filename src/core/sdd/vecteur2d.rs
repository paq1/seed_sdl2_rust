use std::ops;

#[derive(Clone)]
pub struct Vecteur2D<S> {
    pub x: S,
    pub y: S
}

impl<S> Vecteur2D<S> {
    pub fn new(x: S, y: S) -> Self {
        Self {x, y}
    }
}

impl<S> ops::Add<Vecteur2D<S>> for Vecteur2D<S>
    where
        S: ops::Add<Output = S>
{
    type Output = Vecteur2D<S>;

    fn add(self, rhs: Vecteur2D<S>) -> Self::Output {
        Vecteur2D::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl<S> ops::AddAssign<Vecteur2D<S>> for Vecteur2D<S>
    where
        S: ops::AddAssign
{
    fn add_assign(&mut self, rhs: Vecteur2D<S>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<S> ops::SubAssign<Vecteur2D<S>> for Vecteur2D<S>
    where
        S: ops::SubAssign
{
    fn sub_assign(&mut self, rhs: Vecteur2D<S>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}