#[derive(Debug)]
pub struct Distrib<T> {
    data: Vec<T>,
}

impl<T> Distrib<T> {
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, t: T) {
        self.data.push(t);
    }
}

impl<T> Distrib<T>
where
    for<'a> &'a T: Into<f64>,
{
    pub fn average(&self) -> f64 {
        let sum = self.data.iter().map(|pt| pt.into()).sum::<f64>();
        sum / self.data.len() as f64
    }
}
