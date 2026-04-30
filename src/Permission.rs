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
pub struct PermissionSheet {
    sheet: Sheet,
}
impl PermissionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Permission")?;
        let sheet = resolver.read_excel_sheet(&exh, "Permission", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PermissionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PermissionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PermissionSheet {
    type Row = PermissionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
            Unknown1: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            Unknown2: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Unknown3: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown4: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown5: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            Unknown6: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            Unknown7: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            Unknown8: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            Unknown9: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            Unknown10: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            Unknown11: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            Unknown12: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Unknown13: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            Unknown14: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown15: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            Unknown16: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Unknown17: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            Unknown18: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            Unknown19: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            Unknown20: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            Unknown21: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            Unknown22: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            Unknown23: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            Unknown24: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            Unknown25: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            Unknown26: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            Unknown27: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Unknown28: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Unknown29: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            Unknown30: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
            Unknown31: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            Unknown32: row
                .columns[32]
                .into_bool()
                .copied()
                .expect("Expected column 32 to be a bool!"),
            Unknown33: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
            Unknown34: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            Unknown35: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
            Unknown36: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            Unknown37: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            Unknown38: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown39: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            Unknown40: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
            Unknown41: row
                .columns[41]
                .into_bool()
                .copied()
                .expect("Expected column 41 to be a bool!"),
            Unknown42: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            Unknown43: row
                .columns[43]
                .into_bool()
                .copied()
                .expect("Expected column 43 to be a bool!"),
            Unknown44: row
                .columns[44]
                .into_bool()
                .copied()
                .expect("Expected column 44 to be a bool!"),
            Unknown45: row
                .columns[45]
                .into_bool()
                .copied()
                .expect("Expected column 45 to be a bool!"),
            Unknown46: row
                .columns[46]
                .into_bool()
                .copied()
                .expect("Expected column 46 to be a bool!"),
            Unknown47: row
                .columns[47]
                .into_bool()
                .copied()
                .expect("Expected column 47 to be a bool!"),
            Unknown48: row
                .columns[48]
                .into_bool()
                .copied()
                .expect("Expected column 48 to be a bool!"),
            Unknown49: row
                .columns[49]
                .into_bool()
                .copied()
                .expect("Expected column 49 to be a bool!"),
            Unknown50: row
                .columns[50]
                .into_bool()
                .copied()
                .expect("Expected column 50 to be a bool!"),
            Unknown51: row
                .columns[51]
                .into_bool()
                .copied()
                .expect("Expected column 51 to be a bool!"),
            Unknown52: row
                .columns[52]
                .into_bool()
                .copied()
                .expect("Expected column 52 to be a bool!"),
            Unknown53: row
                .columns[53]
                .into_bool()
                .copied()
                .expect("Expected column 53 to be a bool!"),
            Unknown54: row
                .columns[54]
                .into_bool()
                .copied()
                .expect("Expected column 54 to be a bool!"),
            Unknown55: row
                .columns[55]
                .into_bool()
                .copied()
                .expect("Expected column 55 to be a bool!"),
            Unknown56: row
                .columns[56]
                .into_bool()
                .copied()
                .expect("Expected column 56 to be a bool!"),
            Unknown57: row
                .columns[57]
                .into_bool()
                .copied()
                .expect("Expected column 57 to be a bool!"),
            Unknown58: row
                .columns[58]
                .into_bool()
                .copied()
                .expect("Expected column 58 to be a bool!"),
            Unknown59: row
                .columns[59]
                .into_bool()
                .copied()
                .expect("Expected column 59 to be a bool!"),
            Unknown60: row
                .columns[60]
                .into_bool()
                .copied()
                .expect("Expected column 60 to be a bool!"),
            Unknown61: row
                .columns[61]
                .into_bool()
                .copied()
                .expect("Expected column 61 to be a bool!"),
            Unknown62: row
                .columns[62]
                .into_bool()
                .copied()
                .expect("Expected column 62 to be a bool!"),
            Unknown63: row
                .columns[63]
                .into_bool()
                .copied()
                .expect("Expected column 63 to be a bool!"),
            Unknown64: row
                .columns[64]
                .into_bool()
                .copied()
                .expect("Expected column 64 to be a bool!"),
            Unknown65: row
                .columns[65]
                .into_bool()
                .copied()
                .expect("Expected column 65 to be a bool!"),
            Unknown66: row
                .columns[66]
                .into_bool()
                .copied()
                .expect("Expected column 66 to be a bool!"),
            Unknown67: row
                .columns[67]
                .into_bool()
                .copied()
                .expect("Expected column 67 to be a bool!"),
            Unknown68: row
                .columns[68]
                .into_bool()
                .copied()
                .expect("Expected column 68 to be a bool!"),
            Unknown69: row
                .columns[69]
                .into_bool()
                .copied()
                .expect("Expected column 69 to be a bool!"),
            Unknown70: row
                .columns[70]
                .into_bool()
                .copied()
                .expect("Expected column 70 to be a bool!"),
            Unknown71: row
                .columns[71]
                .into_bool()
                .copied()
                .expect("Expected column 71 to be a bool!"),
            Unknown72: row
                .columns[72]
                .into_bool()
                .copied()
                .expect("Expected column 72 to be a bool!"),
            Unknown73: row
                .columns[73]
                .into_bool()
                .copied()
                .expect("Expected column 73 to be a bool!"),
            Unknown74: row
                .columns[74]
                .into_bool()
                .copied()
                .expect("Expected column 74 to be a bool!"),
            Unknown75: row
                .columns[75]
                .into_bool()
                .copied()
                .expect("Expected column 75 to be a bool!"),
            Unknown76: row
                .columns[76]
                .into_bool()
                .copied()
                .expect("Expected column 76 to be a bool!"),
            Unknown77: row
                .columns[77]
                .into_bool()
                .copied()
                .expect("Expected column 77 to be a bool!"),
            Unknown78: row
                .columns[78]
                .into_bool()
                .copied()
                .expect("Expected column 78 to be a bool!"),
            Unknown79: row
                .columns[79]
                .into_bool()
                .copied()
                .expect("Expected column 79 to be a bool!"),
            Unknown80: row
                .columns[80]
                .into_bool()
                .copied()
                .expect("Expected column 80 to be a bool!"),
            Unknown81: row
                .columns[81]
                .into_bool()
                .copied()
                .expect("Expected column 81 to be a bool!"),
            Unknown82: row
                .columns[82]
                .into_bool()
                .copied()
                .expect("Expected column 82 to be a bool!"),
            Unknown83: row
                .columns[83]
                .into_bool()
                .copied()
                .expect("Expected column 83 to be a bool!"),
            Unknown84: row
                .columns[84]
                .into_bool()
                .copied()
                .expect("Expected column 84 to be a bool!"),
            Unknown85: row
                .columns[85]
                .into_bool()
                .copied()
                .expect("Expected column 85 to be a bool!"),
            Unknown86: row
                .columns[86]
                .into_bool()
                .copied()
                .expect("Expected column 86 to be a bool!"),
            Unknown87: row
                .columns[87]
                .into_bool()
                .copied()
                .expect("Expected column 87 to be a bool!"),
            Unknown88: row
                .columns[88]
                .into_bool()
                .copied()
                .expect("Expected column 88 to be a bool!"),
            Unknown89: row
                .columns[89]
                .into_bool()
                .copied()
                .expect("Expected column 89 to be a bool!"),
            Unknown90: row
                .columns[90]
                .into_bool()
                .copied()
                .expect("Expected column 90 to be a bool!"),
            Unknown91: row
                .columns[91]
                .into_bool()
                .copied()
                .expect("Expected column 91 to be a bool!"),
            Unknown92: row
                .columns[92]
                .into_bool()
                .copied()
                .expect("Expected column 92 to be a bool!"),
            Unknown93: row
                .columns[93]
                .into_bool()
                .copied()
                .expect("Expected column 93 to be a bool!"),
            Unknown94: row
                .columns[94]
                .into_bool()
                .copied()
                .expect("Expected column 94 to be a bool!"),
            Unknown95: row
                .columns[95]
                .into_bool()
                .copied()
                .expect("Expected column 95 to be a bool!"),
            Unknown96: row
                .columns[96]
                .into_bool()
                .copied()
                .expect("Expected column 96 to be a bool!"),
            Unknown97: row
                .columns[97]
                .into_bool()
                .copied()
                .expect("Expected column 97 to be a bool!"),
            Unknown98: row
                .columns[98]
                .into_bool()
                .copied()
                .expect("Expected column 98 to be a bool!"),
            Unknown99: row
                .columns[99]
                .into_bool()
                .copied()
                .expect("Expected column 99 to be a bool!"),
            Unknown100: row
                .columns[100]
                .into_bool()
                .copied()
                .expect("Expected column 100 to be a bool!"),
            Unknown101: row
                .columns[101]
                .into_bool()
                .copied()
                .expect("Expected column 101 to be a bool!"),
            Unknown102: row
                .columns[102]
                .into_bool()
                .copied()
                .expect("Expected column 102 to be a bool!"),
            Unknown104: row
                .columns[103]
                .into_bool()
                .copied()
                .expect("Expected column 103 to be a bool!"),
            Unknown105: row
                .columns[104]
                .into_bool()
                .copied()
                .expect("Expected column 104 to be a bool!"),
            Unknown106: row
                .columns[105]
                .into_bool()
                .copied()
                .expect("Expected column 105 to be a bool!"),
            Unknown107: row
                .columns[106]
                .into_bool()
                .copied()
                .expect("Expected column 106 to be a bool!"),
            Unknown108: row
                .columns[107]
                .into_bool()
                .copied()
                .expect("Expected column 107 to be a bool!"),
            Unknown109: row
                .columns[108]
                .into_bool()
                .copied()
                .expect("Expected column 108 to be a bool!"),
            Unknown110: row
                .columns[109]
                .into_bool()
                .copied()
                .expect("Expected column 109 to be a bool!"),
            Unknown111: row
                .columns[110]
                .into_bool()
                .copied()
                .expect("Expected column 110 to be a bool!"),
            Unknown103: row
                .columns[111]
                .into_bool()
                .copied()
                .expect("Expected column 111 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a PermissionSheet {
    type Item = (u32, Vec<(u16, PermissionRow)>);
    type IntoIter = StructuredSheetIterator<'a, PermissionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PermissionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionRow {
    ///""
    pub Unknown0: bool,
    ///""
    pub Unknown1: bool,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub Unknown19: bool,
    ///""
    pub Unknown20: bool,
    ///""
    pub Unknown21: bool,
    ///""
    pub Unknown22: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub Unknown24: bool,
    ///""
    pub Unknown25: bool,
    ///""
    pub Unknown26: bool,
    ///""
    pub Unknown27: bool,
    ///""
    pub Unknown28: bool,
    ///""
    pub Unknown29: bool,
    ///""
    pub Unknown30: bool,
    ///""
    pub Unknown31: bool,
    ///""
    pub Unknown32: bool,
    ///""
    pub Unknown33: bool,
    ///""
    pub Unknown34: bool,
    ///""
    pub Unknown35: bool,
    ///""
    pub Unknown36: bool,
    ///""
    pub Unknown37: bool,
    ///""
    pub Unknown38: bool,
    ///""
    pub Unknown39: bool,
    ///""
    pub Unknown40: bool,
    ///""
    pub Unknown41: bool,
    ///""
    pub Unknown42: bool,
    ///""
    pub Unknown43: bool,
    ///""
    pub Unknown44: bool,
    ///""
    pub Unknown45: bool,
    ///""
    pub Unknown46: bool,
    ///""
    pub Unknown47: bool,
    ///""
    pub Unknown48: bool,
    ///""
    pub Unknown49: bool,
    ///""
    pub Unknown50: bool,
    ///""
    pub Unknown51: bool,
    ///""
    pub Unknown52: bool,
    ///""
    pub Unknown53: bool,
    ///""
    pub Unknown54: bool,
    ///""
    pub Unknown55: bool,
    ///""
    pub Unknown56: bool,
    ///""
    pub Unknown57: bool,
    ///""
    pub Unknown58: bool,
    ///""
    pub Unknown59: bool,
    ///""
    pub Unknown60: bool,
    ///""
    pub Unknown61: bool,
    ///""
    pub Unknown62: bool,
    ///""
    pub Unknown63: bool,
    ///""
    pub Unknown64: bool,
    ///""
    pub Unknown65: bool,
    ///""
    pub Unknown66: bool,
    ///""
    pub Unknown67: bool,
    ///""
    pub Unknown68: bool,
    ///""
    pub Unknown69: bool,
    ///""
    pub Unknown70: bool,
    ///""
    pub Unknown71: bool,
    ///""
    pub Unknown72: bool,
    ///""
    pub Unknown73: bool,
    ///""
    pub Unknown74: bool,
    ///""
    pub Unknown75: bool,
    ///""
    pub Unknown76: bool,
    ///""
    pub Unknown77: bool,
    ///""
    pub Unknown78: bool,
    ///""
    pub Unknown79: bool,
    ///""
    pub Unknown80: bool,
    ///""
    pub Unknown81: bool,
    ///""
    pub Unknown82: bool,
    ///""
    pub Unknown83: bool,
    ///""
    pub Unknown84: bool,
    ///""
    pub Unknown85: bool,
    ///""
    pub Unknown86: bool,
    ///""
    pub Unknown87: bool,
    ///""
    pub Unknown88: bool,
    ///""
    pub Unknown89: bool,
    ///""
    pub Unknown90: bool,
    ///""
    pub Unknown91: bool,
    ///""
    pub Unknown92: bool,
    ///""
    pub Unknown93: bool,
    ///""
    pub Unknown94: bool,
    ///""
    pub Unknown95: bool,
    ///""
    pub Unknown96: bool,
    ///""
    pub Unknown97: bool,
    ///""
    pub Unknown98: bool,
    ///""
    pub Unknown99: bool,
    ///""
    pub Unknown100: bool,
    ///""
    pub Unknown101: bool,
    ///""
    pub Unknown102: bool,
    ///""
    pub Unknown104: bool,
    ///""
    pub Unknown105: bool,
    ///""
    pub Unknown106: bool,
    ///""
    pub Unknown107: bool,
    ///""
    pub Unknown108: bool,
    ///""
    pub Unknown109: bool,
    ///""
    pub Unknown110: bool,
    ///""
    pub Unknown111: bool,
    ///""
    pub Unknown103: bool,
}
