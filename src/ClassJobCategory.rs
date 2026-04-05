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
impl<'a> StructuredSheet<'a> for ClassJobCategorySheet {
    type Row = ClassJobCategoryRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ClassJobCategorySheet {
    type Item = (u32, Vec<(u16, ClassJobCategoryRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobCategorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobCategorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassJobCategoryRow<'a> {
    row: &'a Row,
}
impl<'a> ClassJobCategoryRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn ADV(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn GLA(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn PGL(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn MRD(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn LNC(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
    pub fn ARC(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn CNJ(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn THM(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn CRP(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn BSM(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn ARM(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn GSM(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn LTW(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn WVR(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn ALC(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn CUL(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn MIN(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn BTN(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn FSH(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    pub fn PLD(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn MNK(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn WAR(&'a self) -> bool {
        self.row.columns[22].into_bool().copied().unwrap()
    }
    pub fn DRG(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn BRD(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn WHM(&'a self) -> bool {
        self.row.columns[25].into_bool().copied().unwrap()
    }
    pub fn BLM(&'a self) -> bool {
        self.row.columns[26].into_bool().copied().unwrap()
    }
    pub fn ACN(&'a self) -> bool {
        self.row.columns[27].into_bool().copied().unwrap()
    }
    pub fn SMN(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
    pub fn SCH(&'a self) -> bool {
        self.row.columns[29].into_bool().copied().unwrap()
    }
    pub fn ROG(&'a self) -> bool {
        self.row.columns[30].into_bool().copied().unwrap()
    }
    pub fn NIN(&'a self) -> bool {
        self.row.columns[31].into_bool().copied().unwrap()
    }
    pub fn MCH(&'a self) -> bool {
        self.row.columns[32].into_bool().copied().unwrap()
    }
    pub fn DRK(&'a self) -> bool {
        self.row.columns[33].into_bool().copied().unwrap()
    }
    pub fn AST(&'a self) -> bool {
        self.row.columns[34].into_bool().copied().unwrap()
    }
    pub fn SAM(&'a self) -> bool {
        self.row.columns[35].into_bool().copied().unwrap()
    }
    pub fn RDM(&'a self) -> bool {
        self.row.columns[36].into_bool().copied().unwrap()
    }
    pub fn BLU(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn GNB(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn DNC(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn RPR(&'a self) -> bool {
        self.row.columns[40].into_bool().copied().unwrap()
    }
    pub fn SGE(&'a self) -> bool {
        self.row.columns[41].into_bool().copied().unwrap()
    }
    pub fn VPR(&'a self) -> bool {
        self.row.columns[42].into_bool().copied().unwrap()
    }
    pub fn PCT(&'a self) -> bool {
        self.row.columns[43].into_bool().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> bool {
        self.row.columns[44].into_bool().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[45].into_bool().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[46].into_bool().copied().unwrap()
    }
}
