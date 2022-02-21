use poreader::unit::Unit;

#[derive(Debug)]
pub(crate) struct PoUnit {
    unit: Unit,
}

impl PoUnit {
    pub(super) fn new(unit: Unit) -> Self {
        PoUnit { unit }
    }
}
