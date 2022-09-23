pub trait Monoid {
    type S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn id() -> Self::S;
}

pub struct SegmentTree<M>
where
    M: Monoid,
{
    size: usize,
    data: Vec<M::S>,
}

impl<M> SegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    /// Creates a segment tree with $\\{a_{i}\\}_{i=1}^{N}$ inside.
    /// n: lenght of $\\{a_{i}\\}_{i=1}^{N}$ (i.e. N)
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        SegmentTree::<M> { size, data: vec![M::id(); 2 * size - 1] }
    }

    /// Returns lenght of $\\{a_{i}\\}_{i=1}^{N}$ (i.e. Return N)
    pub fn len(&self) -> usize {
        self.size
    }

    /// Sets x to $a_{idx}$.
    pub fn set(&mut self, mut idx: usize, x: M::S) {
        idx += self.size - 1;
        self.data[idx] = x;
    }

    /// Builds segment tree.
    pub fn build(&mut self) {
        for idx in (0..self.size - 1).rev() {
            self.data[idx] = M::op(&self.data[2 * idx + 1], &self.data[2 * idx + 2]);
        }
    }

    /// Updates $a_{idx}$ to x.
    pub fn update(&mut self, mut idx: usize, x: M::S) {
        idx += self.size - 1;
        self.data[idx] = x;
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.data[idx] = M::op(&self.data[2 * idx + 1], &self.data[2 * idx + 2]);
        }
    }

    /// Returns $a_{idx}$.
    pub fn get(&self, mut idx: usize) -> M::S {
        idx += self.size - 1;
        self.data[idx].clone()
    }

    /// Returns the result (fold op $\left[a_{l}, ... ,a_{r}\right)).$
    /// (i.e. Return $a_{l} (op) a_{l + 1} (op) \cdots (op) a_{r-1})$
    /// Notice that this is a half-opened section.
    pub fn fold(&self, mut l: usize, mut r: usize) -> M::S {
        l += self.size - 1;
        r += self.size - 1;

        let mut xl = M::id();
        let mut xr = M::id();

        while l < r {
            if l % 2 == 0 {
                xl = M::op(&xl, &self.data[l]);
            }
            if r % 2 == 0 {
                xr = M::op(&self.data[r - 1], &xr);
            }
            l = l / 2;
            r = (r - 1) / 2;
        }

        M::op(&xl, &xr)
    }
}