#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color([u8; 4]);

impl Default for Color {
    fn default() -> Self {
        Self([255, 255, 255, 255])
    }
}

impl Color {
    pub const RED: Self = Self([0xff, 0x00, 0x00, 0xff]);
    pub const GLEEN: Self = Self([0x00, 0xff, 0x00, 0xff]);
    pub const BLUE: Self = Self([0x00, 0x00, 0xff, 0xff]);
    pub const YELLOW: Self = Self([0xff, 0xff, 0x00, 0xff]);

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    #[inline(always)]
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    pub fn to_value(&self) -> [f32; 4] {
        [
            self.r() as f32 / 255.0,
            self.g() as f32 / 255.0,
            self.b() as f32 / 255.0,
            self.a() as f32 / 255.0,
        ]
    }
}

/// 从字符串中获取Rgba
/// 1. 以'#'开头, 后面是4个8位的十六进制数字, 如: "#12345678"
/// 2. 4个u8的数组或元组, 如: "[255, 0, 0, 255]" "(255, 0, 0, 255)"
///
impl From<&str> for Color {
    fn from(s: &str) -> Self {
        if s.len() == 9 && s.starts_with('#') {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0xff);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0xff);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0xff);
            let a = u8::from_str_radix(&s[7..9], 16).unwrap_or(0xff);
            return Self([r, g, b, a]);
        }
        let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        if s.starts_with('(') && s.starts_with(')') || s.starts_with('[') && s.starts_with(']') {
            let s = &s[1..s.len() - 1];
            let s = s.split(',').collect::<Vec<_>>();
            if s.len() != 4 {
                return Self::default();
            }
            let r = u8::from_str_radix(s[0], 16).unwrap_or(0xff);
            let g = u8::from_str_radix(s[1], 16).unwrap_or(0xff);
            let b = u8::from_str_radix(s[2], 16).unwrap_or(0xff);
            let a = u8::from_str_radix(s[3], 16).unwrap_or(0xff);
            return Self([r, g, b, a]);
        }
        Self::default()
    }
}

/// 从u32数据中获取Rgba
/// 如: 0xff0000ff
impl From<u32> for Color {
    fn from(n: u32) -> Self {
        Self([
            (n >> 24 & 0xff) as u8,
            (n >> 16 & 0xff) as u8,
            (n >> 8 & 0xff) as u8,
            (n & 0xff) as u8,
        ])
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Color {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Color {}
