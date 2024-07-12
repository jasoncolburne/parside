use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Matter, Prefixer, Saider, Seqner};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct SealSourceTriples {
    pub value: Vec<SealSourceTriple>,
}

impl Group<SealSourceTriple> for SealSourceTriples {
    const CODE: &'static str = Codex::SealSourceTriples;

    fn new(value: Vec<SealSourceTriple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<SealSourceTriple> {
        &self.value
    }
}

impl SealSourceTriples {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SealSourceTriples)> {
        let (rest, body) = count(
            tuple((Parsers::prefixer_parser(cold_code)?, Parsers::seqner_parser(cold_code)?, Parsers::saider_parser(cold_code)?)),
            counter.count() as usize,
        )(bytes)?;
        let body =
            body.into_iter().map(|(prefixer, seqner, saider)| SealSourceTriple { prefixer, seqner, saider }).collect();

        Ok((rest, SealSourceTriples { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SealSourceTriple {
    pub prefixer: Prefixer,
    pub seqner: Seqner,
    pub saider: Saider,
}

impl SealSourceTriple {
    pub fn new(prefixer: Prefixer, seqner: Seqner, saider: Saider) -> Self {
        Self { prefixer, seqner, saider }
    }
}

impl GroupItem for SealSourceTriple {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out += &self.prefixer.qb64()?;
        out += &self.seqner.qb64()?;
        out += &self.saider.qb64()?;
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()?];
        let mut offset = 0;
        let mut len = self.prefixer.full_size()?;
        out[offset..len].copy_from_slice(&self.prefixer.qb64b()?);
        offset += len;
        len = self.seqner.full_size()?;
        out[offset..len].copy_from_slice(&self.seqner.qb64b()?);
        offset += len;
        len = self.saider.full_size()?;
        out[offset..len].copy_from_slice(&self.saider.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()? / 4 * 3];
        let mut offset = 0;
        let mut len = self.prefixer.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.prefixer.qb2()?);
        offset += len;
        len = self.seqner.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.seqner.qb2()?);
        offset += len;
        len = self.saider.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.saider.qb2()?);
        Ok(out)
    }

    fn full_size(&self) -> ParsideResult<usize> {
        let size = self.prefixer.full_size()? + self.seqner.full_size()? + self.saider.full_size()?;
        Ok(size)
    }
}
