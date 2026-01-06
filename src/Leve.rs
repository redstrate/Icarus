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
pub struct LeveSheet {
    sheet: Sheet,
}
impl LeveSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Leve")?;
        let sheet = resolver.read_excel_sheet(&exh, "Leve", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<LeveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<LeveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for LeveSheet {
    type Row = LeveRow;
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
impl<'a> IntoIterator for &'a LeveSheet {
    type Item = (u32, Vec<(u16, LeveRow)>);
    type IntoIter = StructuredSheetIterator<'a, LeveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, LeveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LeveRow {
    columns: Vec<Field>,
}
impl LeveRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn ExpFactor<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn ExpReward<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn GilReward<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn LeveRewardItem<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn JournalGenre<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn LevelLevemete<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn LevelStart<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn LeveClient<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn LeveAssignmentType<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Town<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn PlaceNameStart<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn PlaceNameIssued<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn PlaceNameStartZone<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn IconCityState<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn DataId<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn IconIssuer<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn ClassJobLevel<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn FishingSpot<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn BGM<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn TimeLimit<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn AllowanceCost<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn MaxDifficulty<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn LeveVfx<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn LeveVfxFrame<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn CanCancel<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn LockedLeve<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
}
