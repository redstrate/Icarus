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
pub struct LogMessageSheet {
    sheet: Sheet,
}
impl LogMessageSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("LogMessage")?;
        let sheet = resolver.read_excel_sheet(&exh, "LogMessage", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<LogMessageRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<LogMessageRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for LogMessageSheet {
    type Row = LogMessageRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Text: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            LogKind: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            DisplayFlags: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Unknown1: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Duration: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown2: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a LogMessageSheet {
    type Item = (u32, Vec<(u16, LogMessageRow)>);
    type IntoIter = StructuredSheetIterator<'a, LogMessageSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, LogMessageSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogMessageRow {
    ///""
    pub Text: String,
    ///""
    pub LogKind: u16,
    ///"1 << 0 (0x1) = Print in Chat\n /// 1 << 2 (0x4) = WideText (layer 5, top)\n /// 1 << 3 (0x8) = ErrorText (not forced)\n /// 1 << 4 (0x10) = Text (no icons, no sounds)\n /// 1 << 5 (0x20) = WideText (layer 5, bottom, short)\n /// 1 << 6 (0x40) = WideText (layer 5, bottom, long)\n /// 1 << 7 (0x80) = PoisonText (layer 10)\n /// 1 << 8 (0x100) = ErrorText (forced)\n /// 1 << 9 (0x200) = WideText (layer 10, bottom, short)\n /// 1 << 10 (0x0400) = WideText (layer 10, bottom, long)\n /// 1 << 11 (0x0800) = GimmickHint (Info)\n /// 1 << 12 (0x1000) = GimmickHint (Warning)\n /// "
    pub DisplayFlags: u16,
    ///"Enables a SceneFlags check to remove certain DisplayFlags (0xF883)."
    pub Unknown1: u8,
    ///"Used for GimmickHint messages."
    pub Duration: u8,
    ///"Uses the local players ScreenLogManager to print the message in chat, but only if the local player is the source. Unsure why."
    pub Unknown2: bool,
}
