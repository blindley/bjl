#[derive(Debug, Clone)]
pub struct Grid {
    m_v : Vec<i32>,
    m_width : i32,
    m_height : i32
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut g = Grid {
            m_width: width,
            m_height: height,
            m_v: Vec::new()
        };

        g.resize(width, height);
        g
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.m_width = width;
        self.m_height = height;

        // TODO : use this instead when Vec::resize becomes stable
        // self.m_v.resize(width * height, 0);

        self.m_v = vec![0; (width * height) as usize];
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        self.m_v[(y * self.m_width + x) as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, val: i32) {
        self.m_v[(y * self.m_width + x) as usize] = val;
    }

    pub fn width(&self) -> i32 { self.m_width }
    pub fn height(&self) -> i32 { self.m_height }
}
