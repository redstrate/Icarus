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
pub struct GatheringNotebookListSheet {
    sheet: Sheet,
}
impl GatheringNotebookListSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GatheringNotebookList")?;
        let sheet = resolver.read_excel_sheet(&exh, "GatheringNotebookList", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GatheringNotebookListRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GatheringNotebookListRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GatheringNotebookListSheet {
    type Row = GatheringNotebookListRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            GatheringItem: [
                row
                    .columns[1]
                    .into_i32()
                    .copied()
                    .expect("Expected column 1 to be a int32!"),
                row
                    .columns[2]
                    .into_i32()
                    .copied()
                    .expect("Expected column 2 to be a int32!"),
                row
                    .columns[3]
                    .into_i32()
                    .copied()
                    .expect("Expected column 3 to be a int32!"),
                row
                    .columns[4]
                    .into_i32()
                    .copied()
                    .expect("Expected column 4 to be a int32!"),
                row
                    .columns[5]
                    .into_i32()
                    .copied()
                    .expect("Expected column 5 to be a int32!"),
                row
                    .columns[6]
                    .into_i32()
                    .copied()
                    .expect("Expected column 6 to be a int32!"),
                row
                    .columns[7]
                    .into_i32()
                    .copied()
                    .expect("Expected column 7 to be a int32!"),
                row
                    .columns[8]
                    .into_i32()
                    .copied()
                    .expect("Expected column 8 to be a int32!"),
                row
                    .columns[9]
                    .into_i32()
                    .copied()
                    .expect("Expected column 9 to be a int32!"),
                row
                    .columns[10]
                    .into_i32()
                    .copied()
                    .expect("Expected column 10 to be a int32!"),
                row
                    .columns[11]
                    .into_i32()
                    .copied()
                    .expect("Expected column 11 to be a int32!"),
                row
                    .columns[12]
                    .into_i32()
                    .copied()
                    .expect("Expected column 12 to be a int32!"),
                row
                    .columns[13]
                    .into_i32()
                    .copied()
                    .expect("Expected column 13 to be a int32!"),
                row
                    .columns[14]
                    .into_i32()
                    .copied()
                    .expect("Expected column 14 to be a int32!"),
                row
                    .columns[15]
                    .into_i32()
                    .copied()
                    .expect("Expected column 15 to be a int32!"),
                row
                    .columns[16]
                    .into_i32()
                    .copied()
                    .expect("Expected column 16 to be a int32!"),
                row
                    .columns[17]
                    .into_i32()
                    .copied()
                    .expect("Expected column 17 to be a int32!"),
                row
                    .columns[18]
                    .into_i32()
                    .copied()
                    .expect("Expected column 18 to be a int32!"),
                row
                    .columns[19]
                    .into_i32()
                    .copied()
                    .expect("Expected column 19 to be a int32!"),
                row
                    .columns[20]
                    .into_i32()
                    .copied()
                    .expect("Expected column 20 to be a int32!"),
                row
                    .columns[21]
                    .into_i32()
                    .copied()
                    .expect("Expected column 21 to be a int32!"),
                row
                    .columns[22]
                    .into_i32()
                    .copied()
                    .expect("Expected column 22 to be a int32!"),
                row
                    .columns[23]
                    .into_i32()
                    .copied()
                    .expect("Expected column 23 to be a int32!"),
                row
                    .columns[24]
                    .into_i32()
                    .copied()
                    .expect("Expected column 24 to be a int32!"),
                row
                    .columns[25]
                    .into_i32()
                    .copied()
                    .expect("Expected column 25 to be a int32!"),
                row
                    .columns[26]
                    .into_i32()
                    .copied()
                    .expect("Expected column 26 to be a int32!"),
                row
                    .columns[27]
                    .into_i32()
                    .copied()
                    .expect("Expected column 27 to be a int32!"),
                row
                    .columns[28]
                    .into_i32()
                    .copied()
                    .expect("Expected column 28 to be a int32!"),
                row
                    .columns[29]
                    .into_i32()
                    .copied()
                    .expect("Expected column 29 to be a int32!"),
                row
                    .columns[30]
                    .into_i32()
                    .copied()
                    .expect("Expected column 30 to be a int32!"),
                row
                    .columns[31]
                    .into_i32()
                    .copied()
                    .expect("Expected column 31 to be a int32!"),
                row
                    .columns[32]
                    .into_i32()
                    .copied()
                    .expect("Expected column 32 to be a int32!"),
                row
                    .columns[33]
                    .into_i32()
                    .copied()
                    .expect("Expected column 33 to be a int32!"),
                row
                    .columns[34]
                    .into_i32()
                    .copied()
                    .expect("Expected column 34 to be a int32!"),
                row
                    .columns[35]
                    .into_i32()
                    .copied()
                    .expect("Expected column 35 to be a int32!"),
                row
                    .columns[36]
                    .into_i32()
                    .copied()
                    .expect("Expected column 36 to be a int32!"),
                row
                    .columns[37]
                    .into_i32()
                    .copied()
                    .expect("Expected column 37 to be a int32!"),
                row
                    .columns[38]
                    .into_i32()
                    .copied()
                    .expect("Expected column 38 to be a int32!"),
                row
                    .columns[39]
                    .into_i32()
                    .copied()
                    .expect("Expected column 39 to be a int32!"),
                row
                    .columns[40]
                    .into_i32()
                    .copied()
                    .expect("Expected column 40 to be a int32!"),
                row
                    .columns[41]
                    .into_i32()
                    .copied()
                    .expect("Expected column 41 to be a int32!"),
                row
                    .columns[42]
                    .into_i32()
                    .copied()
                    .expect("Expected column 42 to be a int32!"),
                row
                    .columns[43]
                    .into_i32()
                    .copied()
                    .expect("Expected column 43 to be a int32!"),
                row
                    .columns[44]
                    .into_i32()
                    .copied()
                    .expect("Expected column 44 to be a int32!"),
                row
                    .columns[45]
                    .into_i32()
                    .copied()
                    .expect("Expected column 45 to be a int32!"),
                row
                    .columns[46]
                    .into_i32()
                    .copied()
                    .expect("Expected column 46 to be a int32!"),
                row
                    .columns[47]
                    .into_i32()
                    .copied()
                    .expect("Expected column 47 to be a int32!"),
                row
                    .columns[48]
                    .into_i32()
                    .copied()
                    .expect("Expected column 48 to be a int32!"),
                row
                    .columns[49]
                    .into_i32()
                    .copied()
                    .expect("Expected column 49 to be a int32!"),
                row
                    .columns[50]
                    .into_i32()
                    .copied()
                    .expect("Expected column 50 to be a int32!"),
                row
                    .columns[51]
                    .into_i32()
                    .copied()
                    .expect("Expected column 51 to be a int32!"),
                row
                    .columns[52]
                    .into_i32()
                    .copied()
                    .expect("Expected column 52 to be a int32!"),
                row
                    .columns[53]
                    .into_i32()
                    .copied()
                    .expect("Expected column 53 to be a int32!"),
                row
                    .columns[54]
                    .into_i32()
                    .copied()
                    .expect("Expected column 54 to be a int32!"),
                row
                    .columns[55]
                    .into_i32()
                    .copied()
                    .expect("Expected column 55 to be a int32!"),
                row
                    .columns[56]
                    .into_i32()
                    .copied()
                    .expect("Expected column 56 to be a int32!"),
                row
                    .columns[57]
                    .into_i32()
                    .copied()
                    .expect("Expected column 57 to be a int32!"),
                row
                    .columns[58]
                    .into_i32()
                    .copied()
                    .expect("Expected column 58 to be a int32!"),
                row
                    .columns[59]
                    .into_i32()
                    .copied()
                    .expect("Expected column 59 to be a int32!"),
                row
                    .columns[60]
                    .into_i32()
                    .copied()
                    .expect("Expected column 60 to be a int32!"),
                row
                    .columns[61]
                    .into_i32()
                    .copied()
                    .expect("Expected column 61 to be a int32!"),
                row
                    .columns[62]
                    .into_i32()
                    .copied()
                    .expect("Expected column 62 to be a int32!"),
                row
                    .columns[63]
                    .into_i32()
                    .copied()
                    .expect("Expected column 63 to be a int32!"),
                row
                    .columns[64]
                    .into_i32()
                    .copied()
                    .expect("Expected column 64 to be a int32!"),
                row
                    .columns[65]
                    .into_i32()
                    .copied()
                    .expect("Expected column 65 to be a int32!"),
                row
                    .columns[66]
                    .into_i32()
                    .copied()
                    .expect("Expected column 66 to be a int32!"),
                row
                    .columns[67]
                    .into_i32()
                    .copied()
                    .expect("Expected column 67 to be a int32!"),
                row
                    .columns[68]
                    .into_i32()
                    .copied()
                    .expect("Expected column 68 to be a int32!"),
                row
                    .columns[69]
                    .into_i32()
                    .copied()
                    .expect("Expected column 69 to be a int32!"),
                row
                    .columns[70]
                    .into_i32()
                    .copied()
                    .expect("Expected column 70 to be a int32!"),
                row
                    .columns[71]
                    .into_i32()
                    .copied()
                    .expect("Expected column 71 to be a int32!"),
                row
                    .columns[72]
                    .into_i32()
                    .copied()
                    .expect("Expected column 72 to be a int32!"),
                row
                    .columns[73]
                    .into_i32()
                    .copied()
                    .expect("Expected column 73 to be a int32!"),
                row
                    .columns[74]
                    .into_i32()
                    .copied()
                    .expect("Expected column 74 to be a int32!"),
                row
                    .columns[75]
                    .into_i32()
                    .copied()
                    .expect("Expected column 75 to be a int32!"),
                row
                    .columns[76]
                    .into_i32()
                    .copied()
                    .expect("Expected column 76 to be a int32!"),
                row
                    .columns[77]
                    .into_i32()
                    .copied()
                    .expect("Expected column 77 to be a int32!"),
                row
                    .columns[78]
                    .into_i32()
                    .copied()
                    .expect("Expected column 78 to be a int32!"),
                row
                    .columns[79]
                    .into_i32()
                    .copied()
                    .expect("Expected column 79 to be a int32!"),
                row
                    .columns[80]
                    .into_i32()
                    .copied()
                    .expect("Expected column 80 to be a int32!"),
                row
                    .columns[81]
                    .into_i32()
                    .copied()
                    .expect("Expected column 81 to be a int32!"),
                row
                    .columns[82]
                    .into_i32()
                    .copied()
                    .expect("Expected column 82 to be a int32!"),
                row
                    .columns[83]
                    .into_i32()
                    .copied()
                    .expect("Expected column 83 to be a int32!"),
                row
                    .columns[84]
                    .into_i32()
                    .copied()
                    .expect("Expected column 84 to be a int32!"),
                row
                    .columns[85]
                    .into_i32()
                    .copied()
                    .expect("Expected column 85 to be a int32!"),
                row
                    .columns[86]
                    .into_i32()
                    .copied()
                    .expect("Expected column 86 to be a int32!"),
                row
                    .columns[87]
                    .into_i32()
                    .copied()
                    .expect("Expected column 87 to be a int32!"),
                row
                    .columns[88]
                    .into_i32()
                    .copied()
                    .expect("Expected column 88 to be a int32!"),
                row
                    .columns[89]
                    .into_i32()
                    .copied()
                    .expect("Expected column 89 to be a int32!"),
                row
                    .columns[90]
                    .into_i32()
                    .copied()
                    .expect("Expected column 90 to be a int32!"),
                row
                    .columns[91]
                    .into_i32()
                    .copied()
                    .expect("Expected column 91 to be a int32!"),
                row
                    .columns[92]
                    .into_i32()
                    .copied()
                    .expect("Expected column 92 to be a int32!"),
                row
                    .columns[93]
                    .into_i32()
                    .copied()
                    .expect("Expected column 93 to be a int32!"),
                row
                    .columns[94]
                    .into_i32()
                    .copied()
                    .expect("Expected column 94 to be a int32!"),
                row
                    .columns[95]
                    .into_i32()
                    .copied()
                    .expect("Expected column 95 to be a int32!"),
                row
                    .columns[96]
                    .into_i32()
                    .copied()
                    .expect("Expected column 96 to be a int32!"),
                row
                    .columns[97]
                    .into_i32()
                    .copied()
                    .expect("Expected column 97 to be a int32!"),
                row
                    .columns[98]
                    .into_i32()
                    .copied()
                    .expect("Expected column 98 to be a int32!"),
                row
                    .columns[99]
                    .into_i32()
                    .copied()
                    .expect("Expected column 99 to be a int32!"),
                row
                    .columns[100]
                    .into_i32()
                    .copied()
                    .expect("Expected column 100 to be a int32!"),
            ],
            Unknown0: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GatheringNotebookListSheet {
    type Item = (u32, Vec<(u16, GatheringNotebookListRow)>);
    type IntoIter = StructuredSheetIterator<'a, GatheringNotebookListSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GatheringNotebookListSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GatheringNotebookListRow {
    ///""
    pub GatheringItem: [i32; 100],
    ///""
    pub Unknown0: u8,
}
