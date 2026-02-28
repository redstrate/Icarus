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
pub struct MountCustomizeSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl MountCustomizeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MountCustomize")?;
        let sheet = resolver.read_excel_sheet(&exh, "MountCustomize", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<MountCustomizeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountCustomizeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MountCustomizeSheet {
    type Row = MountCustomizeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a MountCustomizeSheet {
    type Item = (u32, Vec<(u16, MountCustomizeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MountCustomizeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MountCustomizeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MountCustomizeRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> MountCustomizeRow<'a> {
    pub fn HyurMidlanderMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn HyurMidlanderFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn HyurHighlanderMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn HyurHighlanderFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn ElezenMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn ElezenFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn LalaMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn LalaFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn MiqoMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn MiqoFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn RoeMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn RoeFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn AuRaMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn AuRaFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn HrothgarMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn HrothgarFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn VieraMaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn VieraFemaleScale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn HyurMidlanderMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn HyurMidlanderFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn HyurHighlanderMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn HyurHighlanderFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn ElezenMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn ElezenFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn LalaMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn LalaFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn MiqoMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn MiqoFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn RoeMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn RoeFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn AuRaMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn AuRaFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn HrothgarMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn HrothgarFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn VieraMaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn VieraFemaleCameraHeight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
}
