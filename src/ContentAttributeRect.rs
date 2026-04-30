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
pub struct ContentAttributeRectSheet {
    sheet: Sheet,
}
impl ContentAttributeRectSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentAttributeRect")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentAttributeRect", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentAttributeRectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ContentAttributeRectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ContentAttributeRectSheet {
    type Row = ContentAttributeRectRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[32]
                .into_u32()
                .copied()
                .expect("Expected column 32 to be a uint32!"),
            Unknown1: row
                .columns[64]
                .into_u32()
                .copied()
                .expect("Expected column 64 to be a uint32!"),
            Unknown2: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Unknown3: row
                .columns[96]
                .into_u8()
                .copied()
                .expect("Expected column 96 to be a uint8!"),
            Unknown4: row
                .columns[33]
                .into_u32()
                .copied()
                .expect("Expected column 33 to be a uint32!"),
            Unknown5: row
                .columns[65]
                .into_u32()
                .copied()
                .expect("Expected column 65 to be a uint32!"),
            Unknown6: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown7: row
                .columns[97]
                .into_u8()
                .copied()
                .expect("Expected column 97 to be a uint8!"),
            Unknown8: row
                .columns[34]
                .into_u32()
                .copied()
                .expect("Expected column 34 to be a uint32!"),
            Unknown9: row
                .columns[66]
                .into_u32()
                .copied()
                .expect("Expected column 66 to be a uint32!"),
            Unknown10: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown11: row
                .columns[98]
                .into_u8()
                .copied()
                .expect("Expected column 98 to be a uint8!"),
            Unknown12: row
                .columns[35]
                .into_u32()
                .copied()
                .expect("Expected column 35 to be a uint32!"),
            Unknown13: row
                .columns[67]
                .into_u32()
                .copied()
                .expect("Expected column 67 to be a uint32!"),
            Unknown14: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown15: row
                .columns[99]
                .into_u8()
                .copied()
                .expect("Expected column 99 to be a uint8!"),
            Unknown16: row
                .columns[36]
                .into_u32()
                .copied()
                .expect("Expected column 36 to be a uint32!"),
            Unknown17: row
                .columns[68]
                .into_u32()
                .copied()
                .expect("Expected column 68 to be a uint32!"),
            Unknown18: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown19: row
                .columns[100]
                .into_u8()
                .copied()
                .expect("Expected column 100 to be a uint8!"),
            Unknown20: row
                .columns[37]
                .into_u32()
                .copied()
                .expect("Expected column 37 to be a uint32!"),
            Unknown21: row
                .columns[69]
                .into_u32()
                .copied()
                .expect("Expected column 69 to be a uint32!"),
            Unknown22: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown23: row
                .columns[101]
                .into_u8()
                .copied()
                .expect("Expected column 101 to be a uint8!"),
            Unknown24: row
                .columns[38]
                .into_u32()
                .copied()
                .expect("Expected column 38 to be a uint32!"),
            Unknown25: row
                .columns[70]
                .into_u32()
                .copied()
                .expect("Expected column 70 to be a uint32!"),
            Unknown26: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Unknown27: row
                .columns[102]
                .into_u8()
                .copied()
                .expect("Expected column 102 to be a uint8!"),
            Unknown28: row
                .columns[39]
                .into_u32()
                .copied()
                .expect("Expected column 39 to be a uint32!"),
            Unknown29: row
                .columns[71]
                .into_u32()
                .copied()
                .expect("Expected column 71 to be a uint32!"),
            Unknown30: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown31: row
                .columns[103]
                .into_u8()
                .copied()
                .expect("Expected column 103 to be a uint8!"),
            Unknown32: row
                .columns[40]
                .into_u32()
                .copied()
                .expect("Expected column 40 to be a uint32!"),
            Unknown33: row
                .columns[72]
                .into_u32()
                .copied()
                .expect("Expected column 72 to be a uint32!"),
            Unknown34: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown35: row
                .columns[104]
                .into_u8()
                .copied()
                .expect("Expected column 104 to be a uint8!"),
            Unknown36: row
                .columns[41]
                .into_u32()
                .copied()
                .expect("Expected column 41 to be a uint32!"),
            Unknown37: row
                .columns[73]
                .into_u32()
                .copied()
                .expect("Expected column 73 to be a uint32!"),
            Unknown38: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown39: row
                .columns[105]
                .into_u8()
                .copied()
                .expect("Expected column 105 to be a uint8!"),
            Unknown40: row
                .columns[42]
                .into_u32()
                .copied()
                .expect("Expected column 42 to be a uint32!"),
            Unknown41: row
                .columns[74]
                .into_u32()
                .copied()
                .expect("Expected column 74 to be a uint32!"),
            Unknown42: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown43: row
                .columns[106]
                .into_u8()
                .copied()
                .expect("Expected column 106 to be a uint8!"),
            Unknown44: row
                .columns[43]
                .into_u32()
                .copied()
                .expect("Expected column 43 to be a uint32!"),
            Unknown45: row
                .columns[75]
                .into_u32()
                .copied()
                .expect("Expected column 75 to be a uint32!"),
            Unknown46: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            Unknown47: row
                .columns[107]
                .into_u8()
                .copied()
                .expect("Expected column 107 to be a uint8!"),
            Unknown48: row
                .columns[44]
                .into_u32()
                .copied()
                .expect("Expected column 44 to be a uint32!"),
            Unknown49: row
                .columns[76]
                .into_u32()
                .copied()
                .expect("Expected column 76 to be a uint32!"),
            Unknown50: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Unknown51: row
                .columns[108]
                .into_u8()
                .copied()
                .expect("Expected column 108 to be a uint8!"),
            Unknown52: row
                .columns[45]
                .into_u32()
                .copied()
                .expect("Expected column 45 to be a uint32!"),
            Unknown53: row
                .columns[77]
                .into_u32()
                .copied()
                .expect("Expected column 77 to be a uint32!"),
            Unknown54: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Unknown55: row
                .columns[109]
                .into_u8()
                .copied()
                .expect("Expected column 109 to be a uint8!"),
            Unknown56: row
                .columns[46]
                .into_u32()
                .copied()
                .expect("Expected column 46 to be a uint32!"),
            Unknown57: row
                .columns[78]
                .into_u32()
                .copied()
                .expect("Expected column 78 to be a uint32!"),
            Unknown58: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            Unknown59: row
                .columns[110]
                .into_u8()
                .copied()
                .expect("Expected column 110 to be a uint8!"),
            Unknown60: row
                .columns[47]
                .into_u32()
                .copied()
                .expect("Expected column 47 to be a uint32!"),
            Unknown61: row
                .columns[79]
                .into_u32()
                .copied()
                .expect("Expected column 79 to be a uint32!"),
            Unknown62: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            Unknown63: row
                .columns[111]
                .into_u8()
                .copied()
                .expect("Expected column 111 to be a uint8!"),
            Unknown64: row
                .columns[48]
                .into_u32()
                .copied()
                .expect("Expected column 48 to be a uint32!"),
            Unknown65: row
                .columns[80]
                .into_u32()
                .copied()
                .expect("Expected column 80 to be a uint32!"),
            Unknown66: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            Unknown67: row
                .columns[112]
                .into_u8()
                .copied()
                .expect("Expected column 112 to be a uint8!"),
            Unknown68: row
                .columns[49]
                .into_u32()
                .copied()
                .expect("Expected column 49 to be a uint32!"),
            Unknown69: row
                .columns[81]
                .into_u32()
                .copied()
                .expect("Expected column 81 to be a uint32!"),
            Unknown70: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            Unknown71: row
                .columns[113]
                .into_u8()
                .copied()
                .expect("Expected column 113 to be a uint8!"),
            Unknown72: row
                .columns[50]
                .into_u32()
                .copied()
                .expect("Expected column 50 to be a uint32!"),
            Unknown73: row
                .columns[82]
                .into_u32()
                .copied()
                .expect("Expected column 82 to be a uint32!"),
            Unknown74: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            Unknown75: row
                .columns[114]
                .into_u8()
                .copied()
                .expect("Expected column 114 to be a uint8!"),
            Unknown76: row
                .columns[51]
                .into_u32()
                .copied()
                .expect("Expected column 51 to be a uint32!"),
            Unknown77: row
                .columns[83]
                .into_u32()
                .copied()
                .expect("Expected column 83 to be a uint32!"),
            Unknown78: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            Unknown79: row
                .columns[115]
                .into_u8()
                .copied()
                .expect("Expected column 115 to be a uint8!"),
            Unknown80: row
                .columns[52]
                .into_u32()
                .copied()
                .expect("Expected column 52 to be a uint32!"),
            Unknown81: row
                .columns[84]
                .into_u32()
                .copied()
                .expect("Expected column 84 to be a uint32!"),
            Unknown82: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            Unknown83: row
                .columns[116]
                .into_u8()
                .copied()
                .expect("Expected column 116 to be a uint8!"),
            Unknown84: row
                .columns[53]
                .into_u32()
                .copied()
                .expect("Expected column 53 to be a uint32!"),
            Unknown85: row
                .columns[85]
                .into_u32()
                .copied()
                .expect("Expected column 85 to be a uint32!"),
            Unknown86: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            Unknown87: row
                .columns[117]
                .into_u8()
                .copied()
                .expect("Expected column 117 to be a uint8!"),
            Unknown88: row
                .columns[54]
                .into_u32()
                .copied()
                .expect("Expected column 54 to be a uint32!"),
            Unknown89: row
                .columns[86]
                .into_u32()
                .copied()
                .expect("Expected column 86 to be a uint32!"),
            Unknown90: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            Unknown91: row
                .columns[118]
                .into_u8()
                .copied()
                .expect("Expected column 118 to be a uint8!"),
            Unknown92: row
                .columns[55]
                .into_u32()
                .copied()
                .expect("Expected column 55 to be a uint32!"),
            Unknown93: row
                .columns[87]
                .into_u32()
                .copied()
                .expect("Expected column 87 to be a uint32!"),
            Unknown94: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            Unknown95: row
                .columns[119]
                .into_u8()
                .copied()
                .expect("Expected column 119 to be a uint8!"),
            Unknown96: row
                .columns[56]
                .into_u32()
                .copied()
                .expect("Expected column 56 to be a uint32!"),
            Unknown97: row
                .columns[88]
                .into_u32()
                .copied()
                .expect("Expected column 88 to be a uint32!"),
            Unknown98: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            Unknown99: row
                .columns[120]
                .into_u8()
                .copied()
                .expect("Expected column 120 to be a uint8!"),
            Unknown100: row
                .columns[57]
                .into_u32()
                .copied()
                .expect("Expected column 57 to be a uint32!"),
            Unknown101: row
                .columns[89]
                .into_u32()
                .copied()
                .expect("Expected column 89 to be a uint32!"),
            Unknown102: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            Unknown103: row
                .columns[121]
                .into_u8()
                .copied()
                .expect("Expected column 121 to be a uint8!"),
            Unknown104: row
                .columns[58]
                .into_u32()
                .copied()
                .expect("Expected column 58 to be a uint32!"),
            Unknown105: row
                .columns[90]
                .into_u32()
                .copied()
                .expect("Expected column 90 to be a uint32!"),
            Unknown106: row
                .columns[26]
                .into_u8()
                .copied()
                .expect("Expected column 26 to be a uint8!"),
            Unknown107: row
                .columns[122]
                .into_u8()
                .copied()
                .expect("Expected column 122 to be a uint8!"),
            Unknown108: row
                .columns[59]
                .into_u32()
                .copied()
                .expect("Expected column 59 to be a uint32!"),
            Unknown109: row
                .columns[91]
                .into_u32()
                .copied()
                .expect("Expected column 91 to be a uint32!"),
            Unknown110: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            Unknown111: row
                .columns[123]
                .into_u8()
                .copied()
                .expect("Expected column 123 to be a uint8!"),
            Unknown112: row
                .columns[60]
                .into_u32()
                .copied()
                .expect("Expected column 60 to be a uint32!"),
            Unknown113: row
                .columns[92]
                .into_u32()
                .copied()
                .expect("Expected column 92 to be a uint32!"),
            Unknown114: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            Unknown115: row
                .columns[124]
                .into_u8()
                .copied()
                .expect("Expected column 124 to be a uint8!"),
            Unknown116: row
                .columns[61]
                .into_u32()
                .copied()
                .expect("Expected column 61 to be a uint32!"),
            Unknown117: row
                .columns[93]
                .into_u32()
                .copied()
                .expect("Expected column 93 to be a uint32!"),
            Unknown118: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            Unknown119: row
                .columns[125]
                .into_u8()
                .copied()
                .expect("Expected column 125 to be a uint8!"),
            Unknown120: row
                .columns[62]
                .into_u32()
                .copied()
                .expect("Expected column 62 to be a uint32!"),
            Unknown121: row
                .columns[94]
                .into_u32()
                .copied()
                .expect("Expected column 94 to be a uint32!"),
            Unknown122: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            Unknown123: row
                .columns[126]
                .into_u8()
                .copied()
                .expect("Expected column 126 to be a uint8!"),
            Unknown124: row
                .columns[63]
                .into_u32()
                .copied()
                .expect("Expected column 63 to be a uint32!"),
            Unknown125: row
                .columns[95]
                .into_u32()
                .copied()
                .expect("Expected column 95 to be a uint32!"),
            Unknown126: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            Unknown127: row
                .columns[127]
                .into_u8()
                .copied()
                .expect("Expected column 127 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a ContentAttributeRectSheet {
    type Item = (u32, Vec<(u16, ContentAttributeRectRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentAttributeRectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentAttributeRectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentAttributeRectRow {
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown1: u32,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: u32,
    ///""
    pub Unknown5: u32,
    ///""
    pub Unknown6: u8,
    ///""
    pub Unknown7: u8,
    ///""
    pub Unknown8: u32,
    ///""
    pub Unknown9: u32,
    ///""
    pub Unknown10: u8,
    ///""
    pub Unknown11: u8,
    ///""
    pub Unknown12: u32,
    ///""
    pub Unknown13: u32,
    ///""
    pub Unknown14: u8,
    ///""
    pub Unknown15: u8,
    ///""
    pub Unknown16: u32,
    ///""
    pub Unknown17: u32,
    ///""
    pub Unknown18: u8,
    ///""
    pub Unknown19: u8,
    ///""
    pub Unknown20: u32,
    ///""
    pub Unknown21: u32,
    ///""
    pub Unknown22: u8,
    ///""
    pub Unknown23: u8,
    ///""
    pub Unknown24: u32,
    ///""
    pub Unknown25: u32,
    ///""
    pub Unknown26: u8,
    ///""
    pub Unknown27: u8,
    ///""
    pub Unknown28: u32,
    ///""
    pub Unknown29: u32,
    ///""
    pub Unknown30: u8,
    ///""
    pub Unknown31: u8,
    ///""
    pub Unknown32: u32,
    ///""
    pub Unknown33: u32,
    ///""
    pub Unknown34: u8,
    ///""
    pub Unknown35: u8,
    ///""
    pub Unknown36: u32,
    ///""
    pub Unknown37: u32,
    ///""
    pub Unknown38: u8,
    ///""
    pub Unknown39: u8,
    ///""
    pub Unknown40: u32,
    ///""
    pub Unknown41: u32,
    ///""
    pub Unknown42: u8,
    ///""
    pub Unknown43: u8,
    ///""
    pub Unknown44: u32,
    ///""
    pub Unknown45: u32,
    ///""
    pub Unknown46: u8,
    ///""
    pub Unknown47: u8,
    ///""
    pub Unknown48: u32,
    ///""
    pub Unknown49: u32,
    ///""
    pub Unknown50: u8,
    ///""
    pub Unknown51: u8,
    ///""
    pub Unknown52: u32,
    ///""
    pub Unknown53: u32,
    ///""
    pub Unknown54: u8,
    ///""
    pub Unknown55: u8,
    ///""
    pub Unknown56: u32,
    ///""
    pub Unknown57: u32,
    ///""
    pub Unknown58: u8,
    ///""
    pub Unknown59: u8,
    ///""
    pub Unknown60: u32,
    ///""
    pub Unknown61: u32,
    ///""
    pub Unknown62: u8,
    ///""
    pub Unknown63: u8,
    ///""
    pub Unknown64: u32,
    ///""
    pub Unknown65: u32,
    ///""
    pub Unknown66: u8,
    ///""
    pub Unknown67: u8,
    ///""
    pub Unknown68: u32,
    ///""
    pub Unknown69: u32,
    ///""
    pub Unknown70: u8,
    ///""
    pub Unknown71: u8,
    ///""
    pub Unknown72: u32,
    ///""
    pub Unknown73: u32,
    ///""
    pub Unknown74: u8,
    ///""
    pub Unknown75: u8,
    ///""
    pub Unknown76: u32,
    ///""
    pub Unknown77: u32,
    ///""
    pub Unknown78: u8,
    ///""
    pub Unknown79: u8,
    ///""
    pub Unknown80: u32,
    ///""
    pub Unknown81: u32,
    ///""
    pub Unknown82: u8,
    ///""
    pub Unknown83: u8,
    ///""
    pub Unknown84: u32,
    ///""
    pub Unknown85: u32,
    ///""
    pub Unknown86: u8,
    ///""
    pub Unknown87: u8,
    ///""
    pub Unknown88: u32,
    ///""
    pub Unknown89: u32,
    ///""
    pub Unknown90: u8,
    ///""
    pub Unknown91: u8,
    ///""
    pub Unknown92: u32,
    ///""
    pub Unknown93: u32,
    ///""
    pub Unknown94: u8,
    ///""
    pub Unknown95: u8,
    ///""
    pub Unknown96: u32,
    ///""
    pub Unknown97: u32,
    ///""
    pub Unknown98: u8,
    ///""
    pub Unknown99: u8,
    ///""
    pub Unknown100: u32,
    ///""
    pub Unknown101: u32,
    ///""
    pub Unknown102: u8,
    ///""
    pub Unknown103: u8,
    ///""
    pub Unknown104: u32,
    ///""
    pub Unknown105: u32,
    ///""
    pub Unknown106: u8,
    ///""
    pub Unknown107: u8,
    ///""
    pub Unknown108: u32,
    ///""
    pub Unknown109: u32,
    ///""
    pub Unknown110: u8,
    ///""
    pub Unknown111: u8,
    ///""
    pub Unknown112: u32,
    ///""
    pub Unknown113: u32,
    ///""
    pub Unknown114: u8,
    ///""
    pub Unknown115: u8,
    ///""
    pub Unknown116: u32,
    ///""
    pub Unknown117: u32,
    ///""
    pub Unknown118: u8,
    ///""
    pub Unknown119: u8,
    ///""
    pub Unknown120: u32,
    ///""
    pub Unknown121: u32,
    ///""
    pub Unknown122: u8,
    ///""
    pub Unknown123: u8,
    ///""
    pub Unknown124: u32,
    ///""
    pub Unknown125: u32,
    ///""
    pub Unknown126: u8,
    ///""
    pub Unknown127: u8,
}
