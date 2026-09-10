//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectiveIconElement {
    pub LayoutId: u32,
    pub Icon: u16,
}
#[derive(Debug, Clone)]
pub struct FateSheet {
    sheet: Sheet,
}
impl FateSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Fate")?;
        let sheet = resolver.read_excel_sheet(&exh, "Fate", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<FateRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for FateSheet {
    type Row = FateRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Description: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            Objective: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            StatusText: [
                row
                    .columns[3]
                    .into_string()
                    .cloned()
                    .expect("Expected column 3 to be a string!"),
                row
                    .columns[4]
                    .into_string()
                    .cloned()
                    .expect("Expected column 4 to be a string!"),
                row
                    .columns[5]
                    .into_string()
                    .cloned()
                    .expect("Expected column 5 to be a string!"),
            ],
            ArrayEventHandler: row
                .columns[38]
                .into_u32()
                .copied()
                .expect("Expected column 38 to be a uint32!"),
            Unknown1: row
                .columns[39]
                .into_u32()
                .copied()
                .expect("Expected column 39 to be a uint32!"),
            Unknown14: row
                .columns[40]
                .into_u32()
                .copied()
                .expect("Expected column 40 to be a uint32!"),
            ReqEventItem: row
                .columns[41]
                .into_u32()
                .copied()
                .expect("Expected column 41 to be a uint32!"),
            TurnInEventItem: row
                .columns[42]
                .into_u32()
                .copied()
                .expect("Expected column 42 to be a uint32!"),
            Unknown2: [
                row
                    .columns[43]
                    .into_u32()
                    .copied()
                    .expect("Expected column 43 to be a uint32!"),
                row
                    .columns[44]
                    .into_u32()
                    .copied()
                    .expect("Expected column 44 to be a uint32!"),
                row
                    .columns[45]
                    .into_u32()
                    .copied()
                    .expect("Expected column 45 to be a uint32!"),
            ],
            Unknown10: row
                .columns[46]
                .into_u32()
                .copied()
                .expect("Expected column 46 to be a uint32!"),
            Unknown11: row
                .columns[47]
                .into_u32()
                .copied()
                .expect("Expected column 47 to be a uint32!"),
            Unknown12: row
                .columns[48]
                .into_u32()
                .copied()
                .expect("Expected column 48 to be a uint32!"),
            ObjectiveIcon: [
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[49]
                        .into_u32()
                        .copied()
                        .expect("Expected column 49 to be a uint32!"),
                    Icon: row
                        .columns[81]
                        .into_u16()
                        .copied()
                        .expect("Expected column 81 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[50]
                        .into_u32()
                        .copied()
                        .expect("Expected column 50 to be a uint32!"),
                    Icon: row
                        .columns[82]
                        .into_u16()
                        .copied()
                        .expect("Expected column 82 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[51]
                        .into_u32()
                        .copied()
                        .expect("Expected column 51 to be a uint32!"),
                    Icon: row
                        .columns[83]
                        .into_u16()
                        .copied()
                        .expect("Expected column 83 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[52]
                        .into_u32()
                        .copied()
                        .expect("Expected column 52 to be a uint32!"),
                    Icon: row
                        .columns[84]
                        .into_u16()
                        .copied()
                        .expect("Expected column 84 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[53]
                        .into_u32()
                        .copied()
                        .expect("Expected column 53 to be a uint32!"),
                    Icon: row
                        .columns[85]
                        .into_u16()
                        .copied()
                        .expect("Expected column 85 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[54]
                        .into_u32()
                        .copied()
                        .expect("Expected column 54 to be a uint32!"),
                    Icon: row
                        .columns[86]
                        .into_u16()
                        .copied()
                        .expect("Expected column 86 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[55]
                        .into_u32()
                        .copied()
                        .expect("Expected column 55 to be a uint32!"),
                    Icon: row
                        .columns[87]
                        .into_u16()
                        .copied()
                        .expect("Expected column 87 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[56]
                        .into_u32()
                        .copied()
                        .expect("Expected column 56 to be a uint32!"),
                    Icon: row
                        .columns[88]
                        .into_u16()
                        .copied()
                        .expect("Expected column 88 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[57]
                        .into_u32()
                        .copied()
                        .expect("Expected column 57 to be a uint32!"),
                    Icon: row
                        .columns[89]
                        .into_u16()
                        .copied()
                        .expect("Expected column 89 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[58]
                        .into_u32()
                        .copied()
                        .expect("Expected column 58 to be a uint32!"),
                    Icon: row
                        .columns[90]
                        .into_u16()
                        .copied()
                        .expect("Expected column 90 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[59]
                        .into_u32()
                        .copied()
                        .expect("Expected column 59 to be a uint32!"),
                    Icon: row
                        .columns[91]
                        .into_u16()
                        .copied()
                        .expect("Expected column 91 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[60]
                        .into_u32()
                        .copied()
                        .expect("Expected column 60 to be a uint32!"),
                    Icon: row
                        .columns[92]
                        .into_u16()
                        .copied()
                        .expect("Expected column 92 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[61]
                        .into_u32()
                        .copied()
                        .expect("Expected column 61 to be a uint32!"),
                    Icon: row
                        .columns[93]
                        .into_u16()
                        .copied()
                        .expect("Expected column 93 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[62]
                        .into_u32()
                        .copied()
                        .expect("Expected column 62 to be a uint32!"),
                    Icon: row
                        .columns[94]
                        .into_u16()
                        .copied()
                        .expect("Expected column 94 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[63]
                        .into_u32()
                        .copied()
                        .expect("Expected column 63 to be a uint32!"),
                    Icon: row
                        .columns[95]
                        .into_u16()
                        .copied()
                        .expect("Expected column 95 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[64]
                        .into_u32()
                        .copied()
                        .expect("Expected column 64 to be a uint32!"),
                    Icon: row
                        .columns[96]
                        .into_u16()
                        .copied()
                        .expect("Expected column 96 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[65]
                        .into_u32()
                        .copied()
                        .expect("Expected column 65 to be a uint32!"),
                    Icon: row
                        .columns[97]
                        .into_u16()
                        .copied()
                        .expect("Expected column 97 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[66]
                        .into_u32()
                        .copied()
                        .expect("Expected column 66 to be a uint32!"),
                    Icon: row
                        .columns[98]
                        .into_u16()
                        .copied()
                        .expect("Expected column 98 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[67]
                        .into_u32()
                        .copied()
                        .expect("Expected column 67 to be a uint32!"),
                    Icon: row
                        .columns[99]
                        .into_u16()
                        .copied()
                        .expect("Expected column 99 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[68]
                        .into_u32()
                        .copied()
                        .expect("Expected column 68 to be a uint32!"),
                    Icon: row
                        .columns[100]
                        .into_u16()
                        .copied()
                        .expect("Expected column 100 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[69]
                        .into_u32()
                        .copied()
                        .expect("Expected column 69 to be a uint32!"),
                    Icon: row
                        .columns[101]
                        .into_u16()
                        .copied()
                        .expect("Expected column 101 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[70]
                        .into_u32()
                        .copied()
                        .expect("Expected column 70 to be a uint32!"),
                    Icon: row
                        .columns[102]
                        .into_u16()
                        .copied()
                        .expect("Expected column 102 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[71]
                        .into_u32()
                        .copied()
                        .expect("Expected column 71 to be a uint32!"),
                    Icon: row
                        .columns[103]
                        .into_u16()
                        .copied()
                        .expect("Expected column 103 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[72]
                        .into_u32()
                        .copied()
                        .expect("Expected column 72 to be a uint32!"),
                    Icon: row
                        .columns[104]
                        .into_u16()
                        .copied()
                        .expect("Expected column 104 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[73]
                        .into_u32()
                        .copied()
                        .expect("Expected column 73 to be a uint32!"),
                    Icon: row
                        .columns[105]
                        .into_u16()
                        .copied()
                        .expect("Expected column 105 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[74]
                        .into_u32()
                        .copied()
                        .expect("Expected column 74 to be a uint32!"),
                    Icon: row
                        .columns[106]
                        .into_u16()
                        .copied()
                        .expect("Expected column 106 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[75]
                        .into_u32()
                        .copied()
                        .expect("Expected column 75 to be a uint32!"),
                    Icon: row
                        .columns[107]
                        .into_u16()
                        .copied()
                        .expect("Expected column 107 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[76]
                        .into_u32()
                        .copied()
                        .expect("Expected column 76 to be a uint32!"),
                    Icon: row
                        .columns[108]
                        .into_u16()
                        .copied()
                        .expect("Expected column 108 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[77]
                        .into_u32()
                        .copied()
                        .expect("Expected column 77 to be a uint32!"),
                    Icon: row
                        .columns[109]
                        .into_u16()
                        .copied()
                        .expect("Expected column 109 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[78]
                        .into_u32()
                        .copied()
                        .expect("Expected column 78 to be a uint32!"),
                    Icon: row
                        .columns[110]
                        .into_u16()
                        .copied()
                        .expect("Expected column 110 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[79]
                        .into_u32()
                        .copied()
                        .expect("Expected column 79 to be a uint32!"),
                    Icon: row
                        .columns[111]
                        .into_u16()
                        .copied()
                        .expect("Expected column 111 to be a uint16!"),
                },
                ObjectiveIconElement {
                    LayoutId: row
                        .columns[80]
                        .into_u32()
                        .copied()
                        .expect("Expected column 80 to be a uint32!"),
                    Icon: row
                        .columns[112]
                        .into_u16()
                        .copied()
                        .expect("Expected column 112 to be a uint16!"),
                },
            ],
            Location: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            EventItem: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            Icon: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            MapIcon: row
                .columns[17]
                .into_u32()
                .copied()
                .expect("Expected column 17 to be a uint32!"),
            InactiveMapIcon: row
                .columns[18]
                .into_u32()
                .copied()
                .expect("Expected column 18 to be a uint32!"),
            LGBGuardNPCLocation: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            RequiredQuest: row
                .columns[25]
                .into_u32()
                .copied()
                .expect("Expected column 25 to be a uint32!"),
            FATEChain: row
                .columns[34]
                .into_u32()
                .copied()
                .expect("Expected column 34 to be a uint32!"),
            RequiredQuest2: row
                .columns[36]
                .into_u32()
                .copied()
                .expect("Expected column 36 to be a uint32!"),
            FateRuleEx: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Music: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            ScreenImageAccept: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            ScreenImageComplete: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            ScreenImageFailed: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            GivenStatus: row
                .columns[28]
                .into_u16()
                .copied()
                .expect("Expected column 28 to be a uint16!"),
            Unknown4: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            OccupiedPermission: row
                .columns[37]
                .into_u16()
                .copied()
                .expect("Expected column 37 to be a uint16!"),
            FateMode: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Rule: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            ClassJobLevel: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            ClassJobLevelMax: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            StatusValue: [
                row
                    .columns[13]
                    .into_u8()
                    .copied()
                    .expect("Expected column 13 to be a uint8!"),
                row
                    .columns[14]
                    .into_u8()
                    .copied()
                    .expect("Expected column 14 to be a uint8!"),
                row
                    .columns[15]
                    .into_u8()
                    .copied()
                    .expect("Expected column 15 to be a uint8!"),
            ],
            Unknown6: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            RequiredClassJobCategory: row
                .columns[35]
                .into_u8()
                .copied()
                .expect("Expected column 35 to be a uint8!"),
            SpecialFate: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            Unknown8: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Unknown15: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            AdventEvent: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            MoonFaireEvent: row
                .columns[32]
                .into_bool()
                .copied()
                .expect("Expected column 32 to be a bool!"),
            Unknown9: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a FateSheet {
    type Item = (u32, Vec<(u16, FateRow)>);
    type IntoIter = StructuredSheetIterator<'a, FateSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FateSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FateRow {
    ///""
    pub Name: String,
    ///""
    pub Description: String,
    ///""
    pub Objective: String,
    ///""
    pub StatusText: [String; 3],
    ///""
    pub ArrayEventHandler: u32,
    ///""
    pub Unknown1: u32,
    ///""
    pub Unknown14: u32,
    ///""
    pub ReqEventItem: u32,
    ///""
    pub TurnInEventItem: u32,
    ///""
    pub Unknown2: [u32; 3],
    ///""
    pub Unknown10: u32,
    ///""
    pub Unknown11: u32,
    ///""
    pub Unknown12: u32,
    ///""
    pub ObjectiveIcon: [ObjectiveIconElement; 32],
    ///"Instance ID of the EventRange where this spawns."
    pub Location: u32,
    ///""
    pub EventItem: u32,
    ///""
    pub Icon: u32,
    ///""
    pub MapIcon: u32,
    ///""
    pub InactiveMapIcon: u32,
    ///""
    pub LGBGuardNPCLocation: u32,
    ///""
    pub RequiredQuest: u32,
    ///""
    pub FATEChain: u32,
    ///"Used when Unknown6 is 3"
    pub RequiredQuest2: u32,
    ///""
    pub FateRuleEx: u16,
    ///""
    pub Music: u16,
    ///""
    pub ScreenImageAccept: u16,
    ///""
    pub ScreenImageComplete: u16,
    ///""
    pub ScreenImageFailed: u16,
    ///""
    pub GivenStatus: u16,
    ///""
    pub Unknown4: u16,
    ///""
    pub OccupiedPermission: u16,
    ///""
    pub FateMode: u8,
    ///""
    pub Rule: u8,
    ///""
    pub ClassJobLevel: u8,
    ///""
    pub ClassJobLevelMax: u8,
    ///""
    pub StatusValue: [u8; 3],
    ///""
    pub Unknown6: u8,
    ///""
    pub RequiredClassJobCategory: u8,
    ///""
    pub SpecialFate: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub AdventEvent: bool,
    ///""
    pub MoonFaireEvent: bool,
    ///""
    pub Unknown9: bool,
}
