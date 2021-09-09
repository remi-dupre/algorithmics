#[derive(Clone, Debug)]
pub struct Footprint {
    /// Paths that start from current node and ends on it.
    pub ii: Vec<u64>,
    /// Paths that start from current node and ends on any other.
    io: Vec<u64>,
    /// Paths that start from any node and end on any node.
    oo: Vec<u64>,
    /// Paths that do not pass through the root.
    ro: Vec<u64>,
}

impl Footprint {
    fn init(k: usize, weight: u64) -> Self {
        let mut fp = Self {
            ii: vec![0; k + 1],
            io: vec![0; k + 1],
            oo: vec![0; k + 1],
            ro: vec![0; k + 1],
        };

        fp.set_ii_if_greater(0, weight);
        fp
    }

    fn set_ii_if_greater(&mut self, k: usize, val: u64) {
        if self.ii[k] < val {
            self.ii[k] = val;
        }

        self.set_io_if_greater(k, val);
    }

    fn set_io_if_greater(&mut self, k: usize, val: u64) {
        if self.io[k] < val {
            self.io[k] = val;
        }

        self.set_oo_if_greater(k, val);
    }

    fn set_oo_if_greater(&mut self, k: usize, val: u64) {
        if self.oo[k] < val {
            self.oo[k] = val;
        }
    }

    fn set_ro_if_greater(&mut self, k: usize, val: u64) {
        if self.ro[k] < val {
            self.ro[k] = val;
        }
    }

    fn with_ro_as_oo(mut self) -> Self {
        for k in 0..self.ii.len() {
            self.set_oo_if_greater(k, self.ro[k]);
        }

        self
    }

    fn merge(&self, self_weight: u64, child: &Self) -> Self {
        let mut new = self.clone();

        for k in 0..self.ii.len() {
            // ii + io
            for child_k in 0..=k {
                new.set_io_if_greater(k, self.ii[k - child_k] + child.io[child_k]);
            }

            // io + io
            for child_k in 0..k {
                new.set_ii_if_greater(k, self.io[k - child_k - 1] + child.io[child_k]);
            }

            for child_k in 0..=k {
                new.set_oo_if_greater(k, self.io[k - child_k] + child.io[child_k]);
            }

            // oo + io  / io + oo
            for child_k in 0..k {
                new.set_io_if_greater(k, self.oo[k - child_k - 1] + child.io[child_k]);
                new.set_io_if_greater(k, self.io[k - child_k - 1] + child.oo[child_k]);
            }

            // oo + oo
            for child_k in 0..k {
                new.set_oo_if_greater(k, self.oo[k - child_k - 1] + child.oo[child_k]);
            }

            // ro + oo
            new.set_ro_if_greater(k, child.oo[k]);

            for child_k in 0..k {
                new.set_ro_if_greater(k, self.ro[k - child_k - 1] + child.oo[child_k]);
            }

            // ro + io
            for child_k in 0..k {
                new.set_io_if_greater(
                    k,
                    self.ro[k - child_k - 1] + child.io[child_k] + self_weight,
                );
            }
        }

        // Add dummy jumps from `o` to `i`
        for k in 2..self.ii.len() {
            new.set_ii_if_greater(k, self.oo[k - 2]);
        }

        for k in 1..self.ii.len() {
            new.set_ii_if_greater(k, self.io[k - 1]);
            new.set_io_if_greater(k, self.oo[k - 1]);
        }

        new
    }
}

pub struct Mine {
    weights: Vec<u64>,
    children: Vec<Vec<usize>>,
}

impl Mine {
    pub fn new(weights: Vec<u64>, edges: Vec<(usize, usize)>) -> Self {
        let n = weights.len();

        let graph = {
            let mut graph = vec![vec![]; n];

            for (x, y) in edges {
                graph[x - 1].push(y - 1);
                graph[y - 1].push(x - 1);
            }

            graph
        };

        let children = {
            let mut children = vec![vec![]; n];
            let mut run = vec![(0, 0)];

            while let Some((node, parent)) = run.pop() {
                let node_children: Vec<_> = graph[node]
                    .iter()
                    .copied()
                    .filter(|child| *child != parent)
                    .collect();

                run.extend(node_children.iter().map(|child| (*child, node)));
                children[node] = node_children;
            }

            children
        };

        Self { weights, children }
    }

    pub fn optimize(&self, node: usize, k: usize) -> Footprint {
        let fp =
            self.children[node]
                .iter()
                .fold(Footprint::init(k, self.weights[node]), |fp, child| {
                    fp.merge(
                        self.weights[node],
                        &self.optimize(*child, k).with_ro_as_oo(),
                    )
                });

        fp
    }

    fn debug_node(
        &self,
        node: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{}:{}]", node, self.weights[node])?;

        if !self.children[node].is_empty() {
            write!(f, " {{ ")?;

            for &child in &self.children[node] {
                self.debug_node(child, f)?;
                write!(f, ", ")?;
            }

            write!(f, "}}")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Mine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.debug_node(0, f)
    }
}
