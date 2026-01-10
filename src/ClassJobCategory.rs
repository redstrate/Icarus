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
impl StructuredSheet for ClassJobCategorySheet {
    type Row = ClassJobCategoryRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct ClassJobCategoryRow {
    columns: Vec<Field>,
}
impl ClassJobCategoryRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn ADV<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn GLA<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn PGL<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn MRD<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn LNC<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn ARC<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn CNJ<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn THM<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn CRP<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn BSM<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn ARM<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn GSM<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn LTW<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn WVR<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn ALC<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn CUL<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn MIN<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn BTN<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn FSH<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn PLD<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn MNK<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn WAR<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn DRG<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn BRD<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn WHM<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn BLM<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn ACN<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn SMN<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn SCH<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn ROG<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
    pub fn NIN<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn MCH<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn DRK<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn AST<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn SAM<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn RDM<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn BLU<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn GNB<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn DNC<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn RPR<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn SGE<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn VPR<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn PCT<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
}
