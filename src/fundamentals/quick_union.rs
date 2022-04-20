pub struct QuickUnion {
    id: Vec<usize>,
    count: usize,
}

impl QuickUnion {
    pub fn new(n: usize) -> QuickUnion {
        QuickUnion {
            id: Self::initialize_vec(n),
            count: n,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        self.validate(p);
        self.validate(q);

        let p_root = self.find(p);
        let q_root = self.find(q);

        if p_root == q_root {
            return;
        }
        self.id[p_root] = q_root;
        self.count -= 1;
    }

    pub fn find(&self, mut p: usize) -> usize {
        self.validate(p);
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    fn validate(&self, p: usize) {
        if p >= self.count() {
            panic!(
                "Index out of range! Must be in between 0 and {}",
                self.count - 1
            );
        }
    }

    pub fn print(&self) {
        for item in &self.id {
            println!("{}", item);
        }
    }

    fn initialize_vec(n: usize) -> Vec<usize> {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        vec
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_initializes() {
        let qf = QuickUnion::new(10);
    }
}
