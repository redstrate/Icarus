//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct BaseParamSheet {
    sheet: Sheet,
}
impl BaseParamSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BaseParam")?;
        let sheet = resolver.read_excel_sheet(&exh, "BaseParam", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for BaseParamSheet {
    type Row = BaseParamRow;
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
impl<'a> IntoIterator for &'a BaseParamSheet {
    type Item = (u32, Vec<(u16, BaseParamRow)>);
    type IntoIter = StructuredSheetIterator<'a, BaseParamSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BaseParamSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BaseParamRow {
    columns: Vec<Field>,
}
impl BaseParamRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn OneHandWeaponPercent<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn OffHandPercent<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn HeadPercent<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn ChestPercent<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn HandsPercent<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn WaistPercent<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn LegsPercent<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn FeetPercent<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn EarringPercent<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn NecklacePercent<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn BraceletPercent<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn RingPercent<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn TwoHandWeaponPercent<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn UnderArmorPercent<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn ChestHeadPercent<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn ChestHeadLegsFeetPercent<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn LegsFeetPercent<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn HeadChestHandsLegsFeetPercent<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn ChestLegsGlovesPercent<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn ChestLegsFeetPercent<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn OrderPriority<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn MeldParam<'a>(&'a self) -> [&'a Field; 13] {
        [
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
            &self.columns[32],
            &self.columns[33],
            &self.columns[34],
            &self.columns[35],
            &self.columns[36],
            &self.columns[37],
            &self.columns[38],
        ]
    }
    pub fn PacketIndex<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
}
