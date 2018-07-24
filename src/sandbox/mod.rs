//pub mod lifted4;
//pub mod almost_working_no_enums;

#[allow(dead_code)]
pub mod lazy8;

pub trait HKT: 'static {
    type Kind: HKT;
}

#[derive(Debug)]
pub struct Empty {}
impl HKT for Empty {
    type Kind = Empty;
}

#[derive(Debug)]
pub struct IdKind {}
impl HKT for IdKind {
    type Kind = IdKind;
}

#[derive(Debug)]
pub struct OptionKind;
impl HKT for OptionKind {
    type Kind = OptionKind;
}
