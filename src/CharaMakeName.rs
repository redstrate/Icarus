//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct CharaMakeNameSheet {
    sheet: Sheet,
}
impl CharaMakeNameSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CharaMakeName")?;
        let sheet = resolver.read_excel_sheet(&exh, "CharaMakeName", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CharaMakeNameRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CharaMakeNameRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for CharaMakeNameSheet {
    type Row = CharaMakeNameRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a CharaMakeNameSheet {
    type Item = (u32, Vec<(u16, CharaMakeNameRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, CharaMakeNameSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CharaMakeNameSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CharaMakeNameRow<'a> {
    row: &'a Row,
}
impl<'a> CharaMakeNameRow<'a> {
    pub fn HyurMidlanderMale(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn HyurMidlanderFemale(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn HyurMidlanderLastName(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn HyurHighlanderMale(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn HyurHighlanderFemale(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    pub fn HyurHighlanderLastName(&'a self) -> &'a str {
        self.row.columns[5].into_string().unwrap()
    }
    pub fn ElezenMale(&'a self) -> &'a str {
        self.row.columns[6].into_string().unwrap()
    }
    pub fn ElezenFemale(&'a self) -> &'a str {
        self.row.columns[7].into_string().unwrap()
    }
    pub fn ElezenWildwoodLastName(&'a self) -> &'a str {
        self.row.columns[8].into_string().unwrap()
    }
    pub fn ElezenDuskwightLastName(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn MiqoteSunMale(&'a self) -> &'a str {
        self.row.columns[10].into_string().unwrap()
    }
    pub fn MiqoteSunFemale(&'a self) -> &'a str {
        self.row.columns[11].into_string().unwrap()
    }
    pub fn MiqoteSunMaleLastName(&'a self) -> &'a str {
        self.row.columns[12].into_string().unwrap()
    }
    pub fn MiqoteSunFemaleLastName(&'a self) -> &'a str {
        self.row.columns[13].into_string().unwrap()
    }
    pub fn MiqoteMoonMale(&'a self) -> &'a str {
        self.row.columns[14].into_string().unwrap()
    }
    pub fn MiqoteMoonFemale(&'a self) -> &'a str {
        self.row.columns[15].into_string().unwrap()
    }
    pub fn MiqoteMoonLastname(&'a self) -> &'a str {
        self.row.columns[16].into_string().unwrap()
    }
    pub fn LalafellPlainsfolkFirstNameStart(&'a self) -> &'a str {
        self.row.columns[17].into_string().unwrap()
    }
    pub fn LalafellPlainsfolkLastNameStart(&'a self) -> &'a str {
        self.row.columns[18].into_string().unwrap()
    }
    pub fn LalafellPlainsfolkEndOfNames(&'a self) -> &'a str {
        self.row.columns[19].into_string().unwrap()
    }
    pub fn LalafellDunesfolkMale(&'a self) -> &'a str {
        self.row.columns[20].into_string().unwrap()
    }
    pub fn LalafellDunesfolkMaleLastName(&'a self) -> &'a str {
        self.row.columns[21].into_string().unwrap()
    }
    pub fn LalafellDunesfolkFemale(&'a self) -> &'a str {
        self.row.columns[22].into_string().unwrap()
    }
    pub fn LalafellDunesfolkFemaleLastName(&'a self) -> &'a str {
        self.row.columns[23].into_string().unwrap()
    }
    pub fn RoegadynSeaWolfMale(&'a self) -> &'a str {
        self.row.columns[24].into_string().unwrap()
    }
    pub fn RoegadynSeaWolfMaleLastName(&'a self) -> &'a str {
        self.row.columns[25].into_string().unwrap()
    }
    pub fn RoegadynSeaWolfFemale(&'a self) -> &'a str {
        self.row.columns[26].into_string().unwrap()
    }
    pub fn RoegadynSeaWolfFemaleLastName(&'a self) -> &'a str {
        self.row.columns[27].into_string().unwrap()
    }
    pub fn RoegadynHellsguardFirstName(&'a self) -> &'a str {
        self.row.columns[28].into_string().unwrap()
    }
    pub fn RoegadynHellsguardMaleLastName(&'a self) -> &'a str {
        self.row.columns[29].into_string().unwrap()
    }
    pub fn RoegadynHellsguardFemaleLastName(&'a self) -> &'a str {
        self.row.columns[30].into_string().unwrap()
    }
    pub fn AuRaRaenMale(&'a self) -> &'a str {
        self.row.columns[31].into_string().unwrap()
    }
    pub fn AuRaRaenFemale(&'a self) -> &'a str {
        self.row.columns[32].into_string().unwrap()
    }
    pub fn AuRaRaenLastName(&'a self) -> &'a str {
        self.row.columns[33].into_string().unwrap()
    }
    pub fn AuRaXaelaMale(&'a self) -> &'a str {
        self.row.columns[34].into_string().unwrap()
    }
    pub fn AuRaXaelaFemale(&'a self) -> &'a str {
        self.row.columns[35].into_string().unwrap()
    }
    pub fn AuRaXaelaLastName(&'a self) -> &'a str {
        self.row.columns[36].into_string().unwrap()
    }
    pub fn HrothgarHellionsFirstName(&'a self) -> &'a str {
        self.row.columns[37].into_string().unwrap()
    }
    pub fn HrothgarHellionsLastName(&'a self) -> &'a str {
        self.row.columns[38].into_string().unwrap()
    }
    pub fn HrothgarLostFirstName(&'a self) -> &'a str {
        self.row.columns[39].into_string().unwrap()
    }
    pub fn HrothgarLostLastName(&'a self) -> &'a str {
        self.row.columns[40].into_string().unwrap()
    }
    pub fn Unknown0(&'a self) -> &'a str {
        self.row.columns[41].into_string().unwrap()
    }
    pub fn Unknown1(&'a self) -> &'a str {
        self.row.columns[42].into_string().unwrap()
    }
    pub fn Unknown2(&'a self) -> &'a str {
        self.row.columns[43].into_string().unwrap()
    }
    pub fn VieraFirstName(&'a self) -> &'a str {
        self.row.columns[44].into_string().unwrap()
    }
    pub fn VieraRavaLastName(&'a self) -> &'a str {
        self.row.columns[45].into_string().unwrap()
    }
    pub fn VieraVeenaLastName(&'a self) -> &'a str {
        self.row.columns[46].into_string().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> &'a str {
        self.row.columns[47].into_string().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> &'a str {
        self.row.columns[48].into_string().unwrap()
    }
    pub fn Unknown_70_3(&'a self) -> &'a str {
        self.row.columns[49].into_string().unwrap()
    }
}
