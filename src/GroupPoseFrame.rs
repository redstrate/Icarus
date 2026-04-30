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
pub struct GroupPoseFrameSheet {
    sheet: Sheet,
}
impl GroupPoseFrameSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GroupPoseFrame")?;
        let sheet = resolver.read_excel_sheet(&exh, "GroupPoseFrame", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GroupPoseFrameRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GroupPoseFrameRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GroupPoseFrameSheet {
    type Row = GroupPoseFrameRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Text: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            GridText: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Unknown0: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            UnlockCriteria: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Unknown1: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            Image: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            Unknown2: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            Festival: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            SortKey: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            UnlockType: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GroupPoseFrameSheet {
    type Item = (u32, Vec<(u16, GroupPoseFrameRow)>);
    type IntoIter = StructuredSheetIterator<'a, GroupPoseFrameSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GroupPoseFrameSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GroupPoseFrameRow {
    ///""
    pub Text: String,
    ///""
    pub GridText: String,
    ///""
    pub Unknown0: u32,
    ///""
    pub UnlockCriteria: u32,
    ///""
    pub Unknown1: i32,
    ///""
    pub Image: i32,
    ///""
    pub Unknown2: i32,
    ///""
    pub Festival: i32,
    ///""
    pub SortKey: u8,
    ///"1 = Quest\n /// 2 = UnlockLink\n /// 3 = InstanceContent unlocked\n /// 4 = InstanceContent completed\n /// 5 = Emote unlocked\n /// 6 = Companion unlocked\n /// 7 = Mount unlocked\n /// 8 = Ornament unlocked\n /// 9 = FramersKit unlocked\n /// 10 = Always available\n /// "
    pub UnlockType: u8,
}
