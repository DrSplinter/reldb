pub trait Observable<Obs> {
    fn observe(self) -> Obs;
}

pub trait Equiv<Obs, Rhs = Self>: Observable<Obs>
where
    Obs: Eq,
    Rhs: Observable<Obs>,
    Self: Sized,
{
    fn equiv(self, rhs: Rhs) -> bool {
        self.observe() == rhs.observe()
    }
}

pub fn equiv<Obs, Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> bool
where
    Obs: Eq,
    Lhs: Observable<Obs>,
    Rhs: Observable<Obs>,
{
    lhs.observe() == rhs.observe()
}
