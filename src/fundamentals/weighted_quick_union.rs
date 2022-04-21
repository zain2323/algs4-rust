pub struct WeightedQuickUnion {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnion {
    pub fn new(n: usize) -> WeightedQuickUnion {
        let (id_vec, size_vec) = Self::initialize_vec(n);
        WeightedQuickUnion {
            id: id_vec,
            size: size_vec,
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
        let p_root = self.find(p);
        let q_root = self.find(q);

        if p_root == q_root {
            return;
        }
        if self.size[p_root] < self.size[q_root] {
            self.id[p_root] = q_root;
            self.size[q_root] += self.size[p_root];
        } else {
            self.id[q_root] = p_root;
            self.size[p_root] += self.size[q_root];
        }
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

    fn initialize_vec(n: usize) -> (Vec<usize>, Vec<usize>) {
        let mut id = vec![0; n];
        let mut size = vec![0; n];
        for i in 0..n {
            id[i] = i;
            size[i] = 1;
        }
        (id, size)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_initializes() {
        let qf = WeightedQuickUnion::new(10);
    }
}
