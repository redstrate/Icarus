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
    index_mapping: Vec<usize>,
}
impl CharaMakeNameSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CharaMakeName")?;
        let sheet = resolver.read_excel_sheet(&exh, "CharaMakeName", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> CharaMakeNameRow<'a> {
    pub fn HyurMidlanderMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn HyurMidlanderFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn HyurMidlanderLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn HyurHighlanderMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn HyurHighlanderFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn HyurHighlanderLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn ElezenMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn ElezenFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn ElezenWildwoodLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn ElezenDuskwightLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn MiqoteSunMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn MiqoteSunFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn MiqoteSunMaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn MiqoteSunFemaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn MiqoteMoonMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn MiqoteMoonFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn MiqoteMoonLastname(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn LalafellPlainsfolkFirstNameStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn LalafellPlainsfolkLastNameStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn LalafellPlainsfolkEndOfNames(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn LalafellDunesfolkMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn LalafellDunesfolkMaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn LalafellDunesfolkFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn LalafellDunesfolkFemaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn RoegadynSeaWolfMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn RoegadynSeaWolfMaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn RoegadynSeaWolfFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn RoegadynSeaWolfFemaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn RoegadynHellsguardFirstName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn RoegadynHellsguardMaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn RoegadynHellsguardFemaleLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn AuRaRaenMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn AuRaRaenFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn AuRaRaenLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn AuRaXaelaMale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn AuRaXaelaFemale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn AuRaXaelaLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn HrothgarHellionsFirstName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn HrothgarHellionsLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn HrothgarLostFirstName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn HrothgarLostLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn VieraFirstName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn VieraRavaLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn VieraVeenaLastName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn Unknown_70_3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
}
