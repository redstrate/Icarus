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
    index_mapping: Vec<usize>,
}
impl ClassJobCategorySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJobCategory")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJobCategory", language)?;
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> ClassJobCategoryRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn ADV(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn GLA(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn PGL(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn MRD(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn LNC(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn ARC(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn CNJ(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn THM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn CRP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn BSM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn ARM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn GSM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn LTW(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn WVR(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn ALC(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn CUL(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn MIN(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn BTN(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn FSH(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn PLD(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn MNK(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn WAR(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn DRG(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn BRD(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn WHM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn BLM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn ACN(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn SMN(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn SCH(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn ROG(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn NIN(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn MCH(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn DRK(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn AST(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn SAM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn RDM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn BLU(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn GNB(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn DNC(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn RPR(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn SGE(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn VPR(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn PCT(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
}
