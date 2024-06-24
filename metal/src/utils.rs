use crate::fact::MetalTypedFactExt;
use tract_core::internal::*;

pub fn div_ceil(m: usize, b: usize) -> metal::NSUInteger {
    ((m + b - 1) / b) as metal::NSUInteger
}

pub fn metal_output_facts(
    facts: &[&TypedFact],
    resolve_facts: impl Fn(&[&TypedFact]) -> TractResult<TVec<TypedFact>>,
) -> TractResult<TVec<TypedFact>> {
    if facts.iter().all(|it| it.datum_type == DatumType::Opaque) {
        let metal_facts = facts
            .iter()
            .map(|it| it.to_metal_fact().map(|it| it.as_ref()))
            .collect::<TractResult<Vec<_>>>()?;
        let output_facts = (resolve_facts)(metal_facts.as_slice())?;
        Ok(output_facts
            .into_iter()
            .map(|it| it.into_opaque_metal_fact())
            .collect::<TractResult<_>>()?)
    } else if facts.iter().all(|it| it.datum_type != DatumType::Opaque) {
        (resolve_facts)(facts)
    } else {
        bail!(
            "Inconsistent facts datum type: {:?}",
            facts.iter().map(|it| it.datum_type).collect::<Vec<_>>()
        );
    }
}

pub fn metal_facts<T>(
    facts: &[&TypedFact],
    resolve_facts: impl Fn(&[&TypedFact]) -> TractResult<T>,
) -> TractResult<T> {
    if facts.iter().all(|it| it.datum_type == DatumType::Opaque) {
        let metal_facts = facts
            .iter()
            .map(|it| it.to_metal_fact().map(|it| it.as_ref()))
            .collect::<TractResult<Vec<_>>>()?;
        (resolve_facts)(metal_facts.as_slice())
    } else if facts.iter().all(|it| it.datum_type != DatumType::Opaque) {
        (resolve_facts)(facts)
    } else {
        bail!(
            "Inconsistent facts datum type: {:?}",
            facts.iter().map(|it| it.datum_type).collect::<Vec<_>>()
        );
    }
}
