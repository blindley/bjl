pub struct Grid {
    m_v : Vec<i32>,
    m_width : usize,
    m_height : usize
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut g = Grid {
            m_width: width,
            m_height: height,
            m_v: Vec::new()
        };

        g.resize(width, height);
        g
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.m_width = width;
        self.m_height = height;
        self.m_v.resize(width * height, 0);
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.m_v[y * self.m_width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: i32) {
        self.m_v[y * self.m_width + x] = val;
    }

    pub fn width(&self) -> usize { self.m_width }
    pub fn height(&self) -> usize { self.m_height }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            m_width : self.m_width,
            m_height : self.m_height,
            m_v : self.m_v.clone()
        }
    }
}