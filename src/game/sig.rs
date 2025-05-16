pub enum CPointer {
    #[allow(unused)]
    Absolute,
    Relative,
    #[allow(unused)]
    Normal,
}

// static array of sigs
pub static SIGNATURES: &[Signature] = &[
    Signature::new(
        "48 8B 0D ?? ?? ?? ?? 8B D3 E8 ?? ?? ?? ?? 48 8B F8 48 85 C0 74 76",
        "client.dll",
        "CGameEntitySystem",
        3,
        CPointer::Relative
    ),
];

pub struct Signature {
    pub sig: &'static str,
    pub module: &'static str,
    pub name: &'static str,
    pub offset: usize,
    pub pointer: CPointer,
}

impl Signature {
    pub const fn new(pattern: &'static str,
                     module: &'static str,
                     name: &'static str,
                     offset: usize,
                     pointer: CPointer) -> Self {
        Self {
            sig: pattern,
            module: module,
            name: name,
            offset,
            pointer,
        }
    }
}
