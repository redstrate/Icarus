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
pub struct BaseParamSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl BaseParamSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BaseParam")?;
        let sheet = resolver.read_excel_sheet(&exh, "BaseParam", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<BaseParamRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BaseParamRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BaseParamSheet {
    type Row = BaseParamRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a BaseParamSheet {
    type Item = (u32, Vec<(u16, BaseParamRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BaseParamSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BaseParamSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BaseParamRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> BaseParamRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Description(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn OneHandWeaponPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn OffHandPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn HeadPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn ChestPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn HandsPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn WaistPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn LegsPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn FeetPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn EarringPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn NecklacePercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn BraceletPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn RingPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn TwoHandWeaponPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn UnderArmorPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn ChestHeadPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn ChestHeadLegsFeetPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn LegsFeetPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn HeadChestHandsLegsFeetPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn ChestLegsGlovesPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn ChestLegsFeetPercent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn OrderPriority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn MeldParam(&'a self) -> [&'a Field; 13] {
        [
            &self.row.columns[self.index_mapping[26]],
            &self.row.columns[self.index_mapping[27]],
            &self.row.columns[self.index_mapping[28]],
            &self.row.columns[self.index_mapping[29]],
            &self.row.columns[self.index_mapping[30]],
            &self.row.columns[self.index_mapping[31]],
            &self.row.columns[self.index_mapping[32]],
            &self.row.columns[self.index_mapping[33]],
            &self.row.columns[self.index_mapping[34]],
            &self.row.columns[self.index_mapping[35]],
            &self.row.columns[self.index_mapping[36]],
            &self.row.columns[self.index_mapping[37]],
            &self.row.columns[self.index_mapping[38]],
        ]
    }
    pub fn PacketIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
}
