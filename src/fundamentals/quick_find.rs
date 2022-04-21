pub struct QuickFindUF {
    id: Vec<usize>,
    count: usize,
}

impl QuickFindUF {
    pub fn new(n: usize) -> QuickFindUF {
        QuickFindUF {
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
        let pid = self.find(p);
        let qid = self.find(q);

        if pid == qid {
            return;
        }

        for i in 0..self.count {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
        self.count -= 1;
    }

    pub fn find(&self, p: usize) -> usize {
        self.validate(p);
        self.id[p]
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
        let qf = QuickFindUF::new(10);
    }
}
