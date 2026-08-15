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
pub struct ClassJobCategorySheet {
    sheet: Sheet,
}
impl ClassJobCategorySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJobCategory")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJobCategory", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ClassJobCategoryRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobCategoryRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ClassJobCategorySheet {
    type Row = ClassJobCategoryRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            ADV: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            GLA: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            PGL: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            MRD: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            LNC: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            ARC: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            CNJ: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            THM: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            CRP: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            BSM: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            ARM: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            GSM: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            LTW: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            WVR: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            ALC: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            CUL: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            MIN: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            BTN: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            FSH: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            PLD: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            MNK: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            WAR: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            DRG: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            BRD: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            WHM: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            BLM: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            ACN: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            SMN: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            SCH: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            ROG: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
            NIN: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            MCH: row
                .columns[32]
                .into_bool()
                .copied()
                .expect("Expected column 32 to be a bool!"),
            DRK: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
            AST: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            SAM: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
            RDM: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            BLU: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            GNB: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            DNC: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            RPR: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
            SGE: row
                .columns[41]
                .into_bool()
                .copied()
                .expect("Expected column 41 to be a bool!"),
            VPR: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            PCT: row
                .columns[43]
                .into_bool()
                .copied()
                .expect("Expected column 43 to be a bool!"),
            Unknown0: row
                .columns[44]
                .into_bool()
                .copied()
                .expect("Expected column 44 to be a bool!"),
            Unknown1: row
                .columns[45]
                .into_bool()
                .copied()
                .expect("Expected column 45 to be a bool!"),
            Unknown2: row
                .columns[46]
                .into_bool()
                .copied()
                .expect("Expected column 46 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ClassJobCategorySheet {
    type Item = (u32, Vec<(u16, ClassJobCategoryRow)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobCategorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobCategorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClassJobCategoryRow {
    ///""
    pub Name: String,
    ///""
    pub ADV: bool,
    ///""
    pub GLA: bool,
    ///""
    pub PGL: bool,
    ///""
    pub MRD: bool,
    ///""
    pub LNC: bool,
    ///""
    pub ARC: bool,
    ///""
    pub CNJ: bool,
    ///""
    pub THM: bool,
    ///""
    pub CRP: bool,
    ///""
    pub BSM: bool,
    ///""
    pub ARM: bool,
    ///""
    pub GSM: bool,
    ///""
    pub LTW: bool,
    ///""
    pub WVR: bool,
    ///""
    pub ALC: bool,
    ///""
    pub CUL: bool,
    ///""
    pub MIN: bool,
    ///""
    pub BTN: bool,
    ///""
    pub FSH: bool,
    ///""
    pub PLD: bool,
    ///""
    pub MNK: bool,
    ///""
    pub WAR: bool,
    ///""
    pub DRG: bool,
    ///""
    pub BRD: bool,
    ///""
    pub WHM: bool,
    ///""
    pub BLM: bool,
    ///""
    pub ACN: bool,
    ///""
    pub SMN: bool,
    ///""
    pub SCH: bool,
    ///""
    pub ROG: bool,
    ///""
    pub NIN: bool,
    ///""
    pub MCH: bool,
    ///""
    pub DRK: bool,
    ///""
    pub AST: bool,
    ///""
    pub SAM: bool,
    ///""
    pub RDM: bool,
    ///""
    pub BLU: bool,
    ///""
    pub GNB: bool,
    ///""
    pub DNC: bool,
    ///""
    pub RPR: bool,
    ///""
    pub SGE: bool,
    ///""
    pub VPR: bool,
    ///""
    pub PCT: bool,
    ///""
    pub Unknown0: bool,
    ///""
    pub Unknown1: bool,
    ///""
    pub Unknown2: bool,
}
