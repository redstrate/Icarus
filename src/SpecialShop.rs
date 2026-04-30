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
pub struct ItemElement {
    pub ReceiveCount: [u32; 2],
    pub CurrencyCost: [u32; 3],
    pub Item: [i32; 2],
    pub Category: [i32; 2],
    pub ItemCost: [i32; 3],
    pub Quest: i32,
    pub Unknown0: [i32; 4],
    pub AchievementUnlock: i32,
    pub Unknown2: i32,
    pub CollectabilityCost: [u16; 3],
    pub PatchNumber: u16,
    pub CostType: [u8; 3],
    pub Unknown1: [u8; 5],
    pub Order: u8,
    pub ReceiveHq: [bool; 2],
}
#[derive(Debug, Clone)]
pub struct SpecialShopSheet {
    sheet: Sheet,
}
impl SpecialShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SpecialShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "SpecialShop", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SpecialShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SpecialShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SpecialShopSheet {
    type Row = SpecialShopRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Item: [
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[61]
                            .into_u32()
                            .copied()
                            .expect("Expected column 61 to be a uint32!"),
                        row
                            .columns[301]
                            .into_u32()
                            .copied()
                            .expect("Expected column 301 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[541]
                            .into_u32()
                            .copied()
                            .expect("Expected column 541 to be a uint32!"),
                        row
                            .columns[781]
                            .into_u32()
                            .copied()
                            .expect("Expected column 781 to be a uint32!"),
                        row
                            .columns[1021]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1021 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[1]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1 to be a int32!"),
                        row
                            .columns[241]
                            .into_i32()
                            .copied()
                            .expect("Expected column 241 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[121]
                            .into_i32()
                            .copied()
                            .expect("Expected column 121 to be a int32!"),
                        row
                            .columns[361]
                            .into_i32()
                            .copied()
                            .expect("Expected column 361 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[481]
                            .into_i32()
                            .copied()
                            .expect("Expected column 481 to be a int32!"),
                        row
                            .columns[721]
                            .into_i32()
                            .copied()
                            .expect("Expected column 721 to be a int32!"),
                        row
                            .columns[961]
                            .into_i32()
                            .copied()
                            .expect("Expected column 961 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1201]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1201 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1261]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1261 to be a int32!"),
                        row
                            .columns[1501]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1501 to be a int32!"),
                        row
                            .columns[1561]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1561 to be a int32!"),
                        row
                            .columns[1681]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1681 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1741]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1741 to be a int32!"),
                    Unknown2: row
                        .columns[1861]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1861 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[661]
                            .into_u16()
                            .copied()
                            .expect("Expected column 661 to be a uint16!"),
                        row
                            .columns[901]
                            .into_u16()
                            .copied()
                            .expect("Expected column 901 to be a uint16!"),
                        row
                            .columns[1141]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1141 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1981]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1981 to be a uint16!"),
                    CostType: [
                        row
                            .columns[601]
                            .into_u8()
                            .copied()
                            .expect("Expected column 601 to be a uint8!"),
                        row
                            .columns[841]
                            .into_u8()
                            .copied()
                            .expect("Expected column 841 to be a uint8!"),
                        row
                            .columns[1081]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1081 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1321]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1321 to be a uint8!"),
                        row
                            .columns[1381]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1381 to be a uint8!"),
                        row
                            .columns[1441]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1441 to be a uint8!"),
                        row
                            .columns[1621]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1621 to be a uint8!"),
                        row
                            .columns[1801]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1801 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1921]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1921 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[181]
                            .into_bool()
                            .copied()
                            .expect("Expected column 181 to be a bool!"),
                        row
                            .columns[421]
                            .into_bool()
                            .copied()
                            .expect("Expected column 421 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[62]
                            .into_u32()
                            .copied()
                            .expect("Expected column 62 to be a uint32!"),
                        row
                            .columns[302]
                            .into_u32()
                            .copied()
                            .expect("Expected column 302 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[542]
                            .into_u32()
                            .copied()
                            .expect("Expected column 542 to be a uint32!"),
                        row
                            .columns[782]
                            .into_u32()
                            .copied()
                            .expect("Expected column 782 to be a uint32!"),
                        row
                            .columns[1022]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1022 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[2]
                            .into_i32()
                            .copied()
                            .expect("Expected column 2 to be a int32!"),
                        row
                            .columns[242]
                            .into_i32()
                            .copied()
                            .expect("Expected column 242 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[122]
                            .into_i32()
                            .copied()
                            .expect("Expected column 122 to be a int32!"),
                        row
                            .columns[362]
                            .into_i32()
                            .copied()
                            .expect("Expected column 362 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[482]
                            .into_i32()
                            .copied()
                            .expect("Expected column 482 to be a int32!"),
                        row
                            .columns[722]
                            .into_i32()
                            .copied()
                            .expect("Expected column 722 to be a int32!"),
                        row
                            .columns[962]
                            .into_i32()
                            .copied()
                            .expect("Expected column 962 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1202]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1202 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1262]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1262 to be a int32!"),
                        row
                            .columns[1502]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1502 to be a int32!"),
                        row
                            .columns[1562]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1562 to be a int32!"),
                        row
                            .columns[1682]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1682 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1742]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1742 to be a int32!"),
                    Unknown2: row
                        .columns[1862]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1862 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[662]
                            .into_u16()
                            .copied()
                            .expect("Expected column 662 to be a uint16!"),
                        row
                            .columns[902]
                            .into_u16()
                            .copied()
                            .expect("Expected column 902 to be a uint16!"),
                        row
                            .columns[1142]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1142 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1982]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1982 to be a uint16!"),
                    CostType: [
                        row
                            .columns[602]
                            .into_u8()
                            .copied()
                            .expect("Expected column 602 to be a uint8!"),
                        row
                            .columns[842]
                            .into_u8()
                            .copied()
                            .expect("Expected column 842 to be a uint8!"),
                        row
                            .columns[1082]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1082 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1322]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1322 to be a uint8!"),
                        row
                            .columns[1382]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1382 to be a uint8!"),
                        row
                            .columns[1442]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1442 to be a uint8!"),
                        row
                            .columns[1622]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1622 to be a uint8!"),
                        row
                            .columns[1802]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1802 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1922]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1922 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[182]
                            .into_bool()
                            .copied()
                            .expect("Expected column 182 to be a bool!"),
                        row
                            .columns[422]
                            .into_bool()
                            .copied()
                            .expect("Expected column 422 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[63]
                            .into_u32()
                            .copied()
                            .expect("Expected column 63 to be a uint32!"),
                        row
                            .columns[303]
                            .into_u32()
                            .copied()
                            .expect("Expected column 303 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[543]
                            .into_u32()
                            .copied()
                            .expect("Expected column 543 to be a uint32!"),
                        row
                            .columns[783]
                            .into_u32()
                            .copied()
                            .expect("Expected column 783 to be a uint32!"),
                        row
                            .columns[1023]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1023 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[3]
                            .into_i32()
                            .copied()
                            .expect("Expected column 3 to be a int32!"),
                        row
                            .columns[243]
                            .into_i32()
                            .copied()
                            .expect("Expected column 243 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[123]
                            .into_i32()
                            .copied()
                            .expect("Expected column 123 to be a int32!"),
                        row
                            .columns[363]
                            .into_i32()
                            .copied()
                            .expect("Expected column 363 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[483]
                            .into_i32()
                            .copied()
                            .expect("Expected column 483 to be a int32!"),
                        row
                            .columns[723]
                            .into_i32()
                            .copied()
                            .expect("Expected column 723 to be a int32!"),
                        row
                            .columns[963]
                            .into_i32()
                            .copied()
                            .expect("Expected column 963 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1203]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1203 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1263]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1263 to be a int32!"),
                        row
                            .columns[1503]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1503 to be a int32!"),
                        row
                            .columns[1563]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1563 to be a int32!"),
                        row
                            .columns[1683]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1683 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1743]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1743 to be a int32!"),
                    Unknown2: row
                        .columns[1863]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1863 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[663]
                            .into_u16()
                            .copied()
                            .expect("Expected column 663 to be a uint16!"),
                        row
                            .columns[903]
                            .into_u16()
                            .copied()
                            .expect("Expected column 903 to be a uint16!"),
                        row
                            .columns[1143]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1143 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1983]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1983 to be a uint16!"),
                    CostType: [
                        row
                            .columns[603]
                            .into_u8()
                            .copied()
                            .expect("Expected column 603 to be a uint8!"),
                        row
                            .columns[843]
                            .into_u8()
                            .copied()
                            .expect("Expected column 843 to be a uint8!"),
                        row
                            .columns[1083]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1083 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1323]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1323 to be a uint8!"),
                        row
                            .columns[1383]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1383 to be a uint8!"),
                        row
                            .columns[1443]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1443 to be a uint8!"),
                        row
                            .columns[1623]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1623 to be a uint8!"),
                        row
                            .columns[1803]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1803 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1923]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1923 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[183]
                            .into_bool()
                            .copied()
                            .expect("Expected column 183 to be a bool!"),
                        row
                            .columns[423]
                            .into_bool()
                            .copied()
                            .expect("Expected column 423 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[64]
                            .into_u32()
                            .copied()
                            .expect("Expected column 64 to be a uint32!"),
                        row
                            .columns[304]
                            .into_u32()
                            .copied()
                            .expect("Expected column 304 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[544]
                            .into_u32()
                            .copied()
                            .expect("Expected column 544 to be a uint32!"),
                        row
                            .columns[784]
                            .into_u32()
                            .copied()
                            .expect("Expected column 784 to be a uint32!"),
                        row
                            .columns[1024]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1024 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[4]
                            .into_i32()
                            .copied()
                            .expect("Expected column 4 to be a int32!"),
                        row
                            .columns[244]
                            .into_i32()
                            .copied()
                            .expect("Expected column 244 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[124]
                            .into_i32()
                            .copied()
                            .expect("Expected column 124 to be a int32!"),
                        row
                            .columns[364]
                            .into_i32()
                            .copied()
                            .expect("Expected column 364 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[484]
                            .into_i32()
                            .copied()
                            .expect("Expected column 484 to be a int32!"),
                        row
                            .columns[724]
                            .into_i32()
                            .copied()
                            .expect("Expected column 724 to be a int32!"),
                        row
                            .columns[964]
                            .into_i32()
                            .copied()
                            .expect("Expected column 964 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1204]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1204 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1264]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1264 to be a int32!"),
                        row
                            .columns[1504]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1504 to be a int32!"),
                        row
                            .columns[1564]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1564 to be a int32!"),
                        row
                            .columns[1684]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1684 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1744]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1744 to be a int32!"),
                    Unknown2: row
                        .columns[1864]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1864 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[664]
                            .into_u16()
                            .copied()
                            .expect("Expected column 664 to be a uint16!"),
                        row
                            .columns[904]
                            .into_u16()
                            .copied()
                            .expect("Expected column 904 to be a uint16!"),
                        row
                            .columns[1144]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1144 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1984]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1984 to be a uint16!"),
                    CostType: [
                        row
                            .columns[604]
                            .into_u8()
                            .copied()
                            .expect("Expected column 604 to be a uint8!"),
                        row
                            .columns[844]
                            .into_u8()
                            .copied()
                            .expect("Expected column 844 to be a uint8!"),
                        row
                            .columns[1084]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1084 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1324]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1324 to be a uint8!"),
                        row
                            .columns[1384]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1384 to be a uint8!"),
                        row
                            .columns[1444]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1444 to be a uint8!"),
                        row
                            .columns[1624]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1624 to be a uint8!"),
                        row
                            .columns[1804]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1804 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1924]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1924 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[184]
                            .into_bool()
                            .copied()
                            .expect("Expected column 184 to be a bool!"),
                        row
                            .columns[424]
                            .into_bool()
                            .copied()
                            .expect("Expected column 424 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[65]
                            .into_u32()
                            .copied()
                            .expect("Expected column 65 to be a uint32!"),
                        row
                            .columns[305]
                            .into_u32()
                            .copied()
                            .expect("Expected column 305 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[545]
                            .into_u32()
                            .copied()
                            .expect("Expected column 545 to be a uint32!"),
                        row
                            .columns[785]
                            .into_u32()
                            .copied()
                            .expect("Expected column 785 to be a uint32!"),
                        row
                            .columns[1025]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1025 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[5]
                            .into_i32()
                            .copied()
                            .expect("Expected column 5 to be a int32!"),
                        row
                            .columns[245]
                            .into_i32()
                            .copied()
                            .expect("Expected column 245 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[125]
                            .into_i32()
                            .copied()
                            .expect("Expected column 125 to be a int32!"),
                        row
                            .columns[365]
                            .into_i32()
                            .copied()
                            .expect("Expected column 365 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[485]
                            .into_i32()
                            .copied()
                            .expect("Expected column 485 to be a int32!"),
                        row
                            .columns[725]
                            .into_i32()
                            .copied()
                            .expect("Expected column 725 to be a int32!"),
                        row
                            .columns[965]
                            .into_i32()
                            .copied()
                            .expect("Expected column 965 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1205]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1205 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1265]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1265 to be a int32!"),
                        row
                            .columns[1505]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1505 to be a int32!"),
                        row
                            .columns[1565]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1565 to be a int32!"),
                        row
                            .columns[1685]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1685 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1745]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1745 to be a int32!"),
                    Unknown2: row
                        .columns[1865]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1865 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[665]
                            .into_u16()
                            .copied()
                            .expect("Expected column 665 to be a uint16!"),
                        row
                            .columns[905]
                            .into_u16()
                            .copied()
                            .expect("Expected column 905 to be a uint16!"),
                        row
                            .columns[1145]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1145 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1985]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1985 to be a uint16!"),
                    CostType: [
                        row
                            .columns[605]
                            .into_u8()
                            .copied()
                            .expect("Expected column 605 to be a uint8!"),
                        row
                            .columns[845]
                            .into_u8()
                            .copied()
                            .expect("Expected column 845 to be a uint8!"),
                        row
                            .columns[1085]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1085 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1325]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1325 to be a uint8!"),
                        row
                            .columns[1385]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1385 to be a uint8!"),
                        row
                            .columns[1445]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1445 to be a uint8!"),
                        row
                            .columns[1625]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1625 to be a uint8!"),
                        row
                            .columns[1805]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1805 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1925]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1925 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[185]
                            .into_bool()
                            .copied()
                            .expect("Expected column 185 to be a bool!"),
                        row
                            .columns[425]
                            .into_bool()
                            .copied()
                            .expect("Expected column 425 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[66]
                            .into_u32()
                            .copied()
                            .expect("Expected column 66 to be a uint32!"),
                        row
                            .columns[306]
                            .into_u32()
                            .copied()
                            .expect("Expected column 306 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[546]
                            .into_u32()
                            .copied()
                            .expect("Expected column 546 to be a uint32!"),
                        row
                            .columns[786]
                            .into_u32()
                            .copied()
                            .expect("Expected column 786 to be a uint32!"),
                        row
                            .columns[1026]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1026 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[6]
                            .into_i32()
                            .copied()
                            .expect("Expected column 6 to be a int32!"),
                        row
                            .columns[246]
                            .into_i32()
                            .copied()
                            .expect("Expected column 246 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[126]
                            .into_i32()
                            .copied()
                            .expect("Expected column 126 to be a int32!"),
                        row
                            .columns[366]
                            .into_i32()
                            .copied()
                            .expect("Expected column 366 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[486]
                            .into_i32()
                            .copied()
                            .expect("Expected column 486 to be a int32!"),
                        row
                            .columns[726]
                            .into_i32()
                            .copied()
                            .expect("Expected column 726 to be a int32!"),
                        row
                            .columns[966]
                            .into_i32()
                            .copied()
                            .expect("Expected column 966 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1206]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1206 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1266]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1266 to be a int32!"),
                        row
                            .columns[1506]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1506 to be a int32!"),
                        row
                            .columns[1566]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1566 to be a int32!"),
                        row
                            .columns[1686]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1686 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1746]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1746 to be a int32!"),
                    Unknown2: row
                        .columns[1866]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1866 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[666]
                            .into_u16()
                            .copied()
                            .expect("Expected column 666 to be a uint16!"),
                        row
                            .columns[906]
                            .into_u16()
                            .copied()
                            .expect("Expected column 906 to be a uint16!"),
                        row
                            .columns[1146]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1146 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1986]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1986 to be a uint16!"),
                    CostType: [
                        row
                            .columns[606]
                            .into_u8()
                            .copied()
                            .expect("Expected column 606 to be a uint8!"),
                        row
                            .columns[846]
                            .into_u8()
                            .copied()
                            .expect("Expected column 846 to be a uint8!"),
                        row
                            .columns[1086]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1086 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1326]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1326 to be a uint8!"),
                        row
                            .columns[1386]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1386 to be a uint8!"),
                        row
                            .columns[1446]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1446 to be a uint8!"),
                        row
                            .columns[1626]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1626 to be a uint8!"),
                        row
                            .columns[1806]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1806 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1926]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1926 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[186]
                            .into_bool()
                            .copied()
                            .expect("Expected column 186 to be a bool!"),
                        row
                            .columns[426]
                            .into_bool()
                            .copied()
                            .expect("Expected column 426 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[67]
                            .into_u32()
                            .copied()
                            .expect("Expected column 67 to be a uint32!"),
                        row
                            .columns[307]
                            .into_u32()
                            .copied()
                            .expect("Expected column 307 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[547]
                            .into_u32()
                            .copied()
                            .expect("Expected column 547 to be a uint32!"),
                        row
                            .columns[787]
                            .into_u32()
                            .copied()
                            .expect("Expected column 787 to be a uint32!"),
                        row
                            .columns[1027]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1027 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[7]
                            .into_i32()
                            .copied()
                            .expect("Expected column 7 to be a int32!"),
                        row
                            .columns[247]
                            .into_i32()
                            .copied()
                            .expect("Expected column 247 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[127]
                            .into_i32()
                            .copied()
                            .expect("Expected column 127 to be a int32!"),
                        row
                            .columns[367]
                            .into_i32()
                            .copied()
                            .expect("Expected column 367 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[487]
                            .into_i32()
                            .copied()
                            .expect("Expected column 487 to be a int32!"),
                        row
                            .columns[727]
                            .into_i32()
                            .copied()
                            .expect("Expected column 727 to be a int32!"),
                        row
                            .columns[967]
                            .into_i32()
                            .copied()
                            .expect("Expected column 967 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1207]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1207 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1267]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1267 to be a int32!"),
                        row
                            .columns[1507]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1507 to be a int32!"),
                        row
                            .columns[1567]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1567 to be a int32!"),
                        row
                            .columns[1687]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1687 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1747]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1747 to be a int32!"),
                    Unknown2: row
                        .columns[1867]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1867 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[667]
                            .into_u16()
                            .copied()
                            .expect("Expected column 667 to be a uint16!"),
                        row
                            .columns[907]
                            .into_u16()
                            .copied()
                            .expect("Expected column 907 to be a uint16!"),
                        row
                            .columns[1147]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1147 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1987]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1987 to be a uint16!"),
                    CostType: [
                        row
                            .columns[607]
                            .into_u8()
                            .copied()
                            .expect("Expected column 607 to be a uint8!"),
                        row
                            .columns[847]
                            .into_u8()
                            .copied()
                            .expect("Expected column 847 to be a uint8!"),
                        row
                            .columns[1087]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1087 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1327]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1327 to be a uint8!"),
                        row
                            .columns[1387]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1387 to be a uint8!"),
                        row
                            .columns[1447]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1447 to be a uint8!"),
                        row
                            .columns[1627]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1627 to be a uint8!"),
                        row
                            .columns[1807]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1807 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1927]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1927 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[187]
                            .into_bool()
                            .copied()
                            .expect("Expected column 187 to be a bool!"),
                        row
                            .columns[427]
                            .into_bool()
                            .copied()
                            .expect("Expected column 427 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[68]
                            .into_u32()
                            .copied()
                            .expect("Expected column 68 to be a uint32!"),
                        row
                            .columns[308]
                            .into_u32()
                            .copied()
                            .expect("Expected column 308 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[548]
                            .into_u32()
                            .copied()
                            .expect("Expected column 548 to be a uint32!"),
                        row
                            .columns[788]
                            .into_u32()
                            .copied()
                            .expect("Expected column 788 to be a uint32!"),
                        row
                            .columns[1028]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1028 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[8]
                            .into_i32()
                            .copied()
                            .expect("Expected column 8 to be a int32!"),
                        row
                            .columns[248]
                            .into_i32()
                            .copied()
                            .expect("Expected column 248 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[128]
                            .into_i32()
                            .copied()
                            .expect("Expected column 128 to be a int32!"),
                        row
                            .columns[368]
                            .into_i32()
                            .copied()
                            .expect("Expected column 368 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[488]
                            .into_i32()
                            .copied()
                            .expect("Expected column 488 to be a int32!"),
                        row
                            .columns[728]
                            .into_i32()
                            .copied()
                            .expect("Expected column 728 to be a int32!"),
                        row
                            .columns[968]
                            .into_i32()
                            .copied()
                            .expect("Expected column 968 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1208]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1208 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1268]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1268 to be a int32!"),
                        row
                            .columns[1508]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1508 to be a int32!"),
                        row
                            .columns[1568]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1568 to be a int32!"),
                        row
                            .columns[1688]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1688 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1748]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1748 to be a int32!"),
                    Unknown2: row
                        .columns[1868]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1868 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[668]
                            .into_u16()
                            .copied()
                            .expect("Expected column 668 to be a uint16!"),
                        row
                            .columns[908]
                            .into_u16()
                            .copied()
                            .expect("Expected column 908 to be a uint16!"),
                        row
                            .columns[1148]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1148 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1988]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1988 to be a uint16!"),
                    CostType: [
                        row
                            .columns[608]
                            .into_u8()
                            .copied()
                            .expect("Expected column 608 to be a uint8!"),
                        row
                            .columns[848]
                            .into_u8()
                            .copied()
                            .expect("Expected column 848 to be a uint8!"),
                        row
                            .columns[1088]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1088 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1328]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1328 to be a uint8!"),
                        row
                            .columns[1388]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1388 to be a uint8!"),
                        row
                            .columns[1448]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1448 to be a uint8!"),
                        row
                            .columns[1628]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1628 to be a uint8!"),
                        row
                            .columns[1808]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1808 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1928]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1928 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[188]
                            .into_bool()
                            .copied()
                            .expect("Expected column 188 to be a bool!"),
                        row
                            .columns[428]
                            .into_bool()
                            .copied()
                            .expect("Expected column 428 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[69]
                            .into_u32()
                            .copied()
                            .expect("Expected column 69 to be a uint32!"),
                        row
                            .columns[309]
                            .into_u32()
                            .copied()
                            .expect("Expected column 309 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[549]
                            .into_u32()
                            .copied()
                            .expect("Expected column 549 to be a uint32!"),
                        row
                            .columns[789]
                            .into_u32()
                            .copied()
                            .expect("Expected column 789 to be a uint32!"),
                        row
                            .columns[1029]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1029 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[9]
                            .into_i32()
                            .copied()
                            .expect("Expected column 9 to be a int32!"),
                        row
                            .columns[249]
                            .into_i32()
                            .copied()
                            .expect("Expected column 249 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[129]
                            .into_i32()
                            .copied()
                            .expect("Expected column 129 to be a int32!"),
                        row
                            .columns[369]
                            .into_i32()
                            .copied()
                            .expect("Expected column 369 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[489]
                            .into_i32()
                            .copied()
                            .expect("Expected column 489 to be a int32!"),
                        row
                            .columns[729]
                            .into_i32()
                            .copied()
                            .expect("Expected column 729 to be a int32!"),
                        row
                            .columns[969]
                            .into_i32()
                            .copied()
                            .expect("Expected column 969 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1209]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1209 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1269]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1269 to be a int32!"),
                        row
                            .columns[1509]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1509 to be a int32!"),
                        row
                            .columns[1569]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1569 to be a int32!"),
                        row
                            .columns[1689]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1689 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1749]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1749 to be a int32!"),
                    Unknown2: row
                        .columns[1869]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1869 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[669]
                            .into_u16()
                            .copied()
                            .expect("Expected column 669 to be a uint16!"),
                        row
                            .columns[909]
                            .into_u16()
                            .copied()
                            .expect("Expected column 909 to be a uint16!"),
                        row
                            .columns[1149]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1149 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1989]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1989 to be a uint16!"),
                    CostType: [
                        row
                            .columns[609]
                            .into_u8()
                            .copied()
                            .expect("Expected column 609 to be a uint8!"),
                        row
                            .columns[849]
                            .into_u8()
                            .copied()
                            .expect("Expected column 849 to be a uint8!"),
                        row
                            .columns[1089]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1089 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1329]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1329 to be a uint8!"),
                        row
                            .columns[1389]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1389 to be a uint8!"),
                        row
                            .columns[1449]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1449 to be a uint8!"),
                        row
                            .columns[1629]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1629 to be a uint8!"),
                        row
                            .columns[1809]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1809 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1929]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1929 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[189]
                            .into_bool()
                            .copied()
                            .expect("Expected column 189 to be a bool!"),
                        row
                            .columns[429]
                            .into_bool()
                            .copied()
                            .expect("Expected column 429 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[70]
                            .into_u32()
                            .copied()
                            .expect("Expected column 70 to be a uint32!"),
                        row
                            .columns[310]
                            .into_u32()
                            .copied()
                            .expect("Expected column 310 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[550]
                            .into_u32()
                            .copied()
                            .expect("Expected column 550 to be a uint32!"),
                        row
                            .columns[790]
                            .into_u32()
                            .copied()
                            .expect("Expected column 790 to be a uint32!"),
                        row
                            .columns[1030]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1030 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[10]
                            .into_i32()
                            .copied()
                            .expect("Expected column 10 to be a int32!"),
                        row
                            .columns[250]
                            .into_i32()
                            .copied()
                            .expect("Expected column 250 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[130]
                            .into_i32()
                            .copied()
                            .expect("Expected column 130 to be a int32!"),
                        row
                            .columns[370]
                            .into_i32()
                            .copied()
                            .expect("Expected column 370 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[490]
                            .into_i32()
                            .copied()
                            .expect("Expected column 490 to be a int32!"),
                        row
                            .columns[730]
                            .into_i32()
                            .copied()
                            .expect("Expected column 730 to be a int32!"),
                        row
                            .columns[970]
                            .into_i32()
                            .copied()
                            .expect("Expected column 970 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1210]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1210 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1270]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1270 to be a int32!"),
                        row
                            .columns[1510]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1510 to be a int32!"),
                        row
                            .columns[1570]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1570 to be a int32!"),
                        row
                            .columns[1690]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1690 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1750]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1750 to be a int32!"),
                    Unknown2: row
                        .columns[1870]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1870 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[670]
                            .into_u16()
                            .copied()
                            .expect("Expected column 670 to be a uint16!"),
                        row
                            .columns[910]
                            .into_u16()
                            .copied()
                            .expect("Expected column 910 to be a uint16!"),
                        row
                            .columns[1150]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1150 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1990]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1990 to be a uint16!"),
                    CostType: [
                        row
                            .columns[610]
                            .into_u8()
                            .copied()
                            .expect("Expected column 610 to be a uint8!"),
                        row
                            .columns[850]
                            .into_u8()
                            .copied()
                            .expect("Expected column 850 to be a uint8!"),
                        row
                            .columns[1090]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1090 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1330]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1330 to be a uint8!"),
                        row
                            .columns[1390]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1390 to be a uint8!"),
                        row
                            .columns[1450]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1450 to be a uint8!"),
                        row
                            .columns[1630]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1630 to be a uint8!"),
                        row
                            .columns[1810]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1810 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1930]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1930 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[190]
                            .into_bool()
                            .copied()
                            .expect("Expected column 190 to be a bool!"),
                        row
                            .columns[430]
                            .into_bool()
                            .copied()
                            .expect("Expected column 430 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[71]
                            .into_u32()
                            .copied()
                            .expect("Expected column 71 to be a uint32!"),
                        row
                            .columns[311]
                            .into_u32()
                            .copied()
                            .expect("Expected column 311 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[551]
                            .into_u32()
                            .copied()
                            .expect("Expected column 551 to be a uint32!"),
                        row
                            .columns[791]
                            .into_u32()
                            .copied()
                            .expect("Expected column 791 to be a uint32!"),
                        row
                            .columns[1031]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1031 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[11]
                            .into_i32()
                            .copied()
                            .expect("Expected column 11 to be a int32!"),
                        row
                            .columns[251]
                            .into_i32()
                            .copied()
                            .expect("Expected column 251 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[131]
                            .into_i32()
                            .copied()
                            .expect("Expected column 131 to be a int32!"),
                        row
                            .columns[371]
                            .into_i32()
                            .copied()
                            .expect("Expected column 371 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[491]
                            .into_i32()
                            .copied()
                            .expect("Expected column 491 to be a int32!"),
                        row
                            .columns[731]
                            .into_i32()
                            .copied()
                            .expect("Expected column 731 to be a int32!"),
                        row
                            .columns[971]
                            .into_i32()
                            .copied()
                            .expect("Expected column 971 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1211]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1211 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1271]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1271 to be a int32!"),
                        row
                            .columns[1511]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1511 to be a int32!"),
                        row
                            .columns[1571]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1571 to be a int32!"),
                        row
                            .columns[1691]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1691 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1751]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1751 to be a int32!"),
                    Unknown2: row
                        .columns[1871]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1871 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[671]
                            .into_u16()
                            .copied()
                            .expect("Expected column 671 to be a uint16!"),
                        row
                            .columns[911]
                            .into_u16()
                            .copied()
                            .expect("Expected column 911 to be a uint16!"),
                        row
                            .columns[1151]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1151 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1991]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1991 to be a uint16!"),
                    CostType: [
                        row
                            .columns[611]
                            .into_u8()
                            .copied()
                            .expect("Expected column 611 to be a uint8!"),
                        row
                            .columns[851]
                            .into_u8()
                            .copied()
                            .expect("Expected column 851 to be a uint8!"),
                        row
                            .columns[1091]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1091 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1331]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1331 to be a uint8!"),
                        row
                            .columns[1391]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1391 to be a uint8!"),
                        row
                            .columns[1451]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1451 to be a uint8!"),
                        row
                            .columns[1631]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1631 to be a uint8!"),
                        row
                            .columns[1811]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1811 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1931]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1931 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[191]
                            .into_bool()
                            .copied()
                            .expect("Expected column 191 to be a bool!"),
                        row
                            .columns[431]
                            .into_bool()
                            .copied()
                            .expect("Expected column 431 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[72]
                            .into_u32()
                            .copied()
                            .expect("Expected column 72 to be a uint32!"),
                        row
                            .columns[312]
                            .into_u32()
                            .copied()
                            .expect("Expected column 312 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[552]
                            .into_u32()
                            .copied()
                            .expect("Expected column 552 to be a uint32!"),
                        row
                            .columns[792]
                            .into_u32()
                            .copied()
                            .expect("Expected column 792 to be a uint32!"),
                        row
                            .columns[1032]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1032 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[12]
                            .into_i32()
                            .copied()
                            .expect("Expected column 12 to be a int32!"),
                        row
                            .columns[252]
                            .into_i32()
                            .copied()
                            .expect("Expected column 252 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[132]
                            .into_i32()
                            .copied()
                            .expect("Expected column 132 to be a int32!"),
                        row
                            .columns[372]
                            .into_i32()
                            .copied()
                            .expect("Expected column 372 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[492]
                            .into_i32()
                            .copied()
                            .expect("Expected column 492 to be a int32!"),
                        row
                            .columns[732]
                            .into_i32()
                            .copied()
                            .expect("Expected column 732 to be a int32!"),
                        row
                            .columns[972]
                            .into_i32()
                            .copied()
                            .expect("Expected column 972 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1212]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1212 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1272]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1272 to be a int32!"),
                        row
                            .columns[1512]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1512 to be a int32!"),
                        row
                            .columns[1572]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1572 to be a int32!"),
                        row
                            .columns[1692]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1692 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1752]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1752 to be a int32!"),
                    Unknown2: row
                        .columns[1872]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1872 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[672]
                            .into_u16()
                            .copied()
                            .expect("Expected column 672 to be a uint16!"),
                        row
                            .columns[912]
                            .into_u16()
                            .copied()
                            .expect("Expected column 912 to be a uint16!"),
                        row
                            .columns[1152]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1152 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1992]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1992 to be a uint16!"),
                    CostType: [
                        row
                            .columns[612]
                            .into_u8()
                            .copied()
                            .expect("Expected column 612 to be a uint8!"),
                        row
                            .columns[852]
                            .into_u8()
                            .copied()
                            .expect("Expected column 852 to be a uint8!"),
                        row
                            .columns[1092]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1092 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1332]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1332 to be a uint8!"),
                        row
                            .columns[1392]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1392 to be a uint8!"),
                        row
                            .columns[1452]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1452 to be a uint8!"),
                        row
                            .columns[1632]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1632 to be a uint8!"),
                        row
                            .columns[1812]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1812 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1932]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1932 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[192]
                            .into_bool()
                            .copied()
                            .expect("Expected column 192 to be a bool!"),
                        row
                            .columns[432]
                            .into_bool()
                            .copied()
                            .expect("Expected column 432 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[73]
                            .into_u32()
                            .copied()
                            .expect("Expected column 73 to be a uint32!"),
                        row
                            .columns[313]
                            .into_u32()
                            .copied()
                            .expect("Expected column 313 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[553]
                            .into_u32()
                            .copied()
                            .expect("Expected column 553 to be a uint32!"),
                        row
                            .columns[793]
                            .into_u32()
                            .copied()
                            .expect("Expected column 793 to be a uint32!"),
                        row
                            .columns[1033]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1033 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[13]
                            .into_i32()
                            .copied()
                            .expect("Expected column 13 to be a int32!"),
                        row
                            .columns[253]
                            .into_i32()
                            .copied()
                            .expect("Expected column 253 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[133]
                            .into_i32()
                            .copied()
                            .expect("Expected column 133 to be a int32!"),
                        row
                            .columns[373]
                            .into_i32()
                            .copied()
                            .expect("Expected column 373 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[493]
                            .into_i32()
                            .copied()
                            .expect("Expected column 493 to be a int32!"),
                        row
                            .columns[733]
                            .into_i32()
                            .copied()
                            .expect("Expected column 733 to be a int32!"),
                        row
                            .columns[973]
                            .into_i32()
                            .copied()
                            .expect("Expected column 973 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1213]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1213 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1273]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1273 to be a int32!"),
                        row
                            .columns[1513]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1513 to be a int32!"),
                        row
                            .columns[1573]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1573 to be a int32!"),
                        row
                            .columns[1693]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1693 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1753]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1753 to be a int32!"),
                    Unknown2: row
                        .columns[1873]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1873 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[673]
                            .into_u16()
                            .copied()
                            .expect("Expected column 673 to be a uint16!"),
                        row
                            .columns[913]
                            .into_u16()
                            .copied()
                            .expect("Expected column 913 to be a uint16!"),
                        row
                            .columns[1153]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1153 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1993]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1993 to be a uint16!"),
                    CostType: [
                        row
                            .columns[613]
                            .into_u8()
                            .copied()
                            .expect("Expected column 613 to be a uint8!"),
                        row
                            .columns[853]
                            .into_u8()
                            .copied()
                            .expect("Expected column 853 to be a uint8!"),
                        row
                            .columns[1093]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1093 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1333]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1333 to be a uint8!"),
                        row
                            .columns[1393]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1393 to be a uint8!"),
                        row
                            .columns[1453]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1453 to be a uint8!"),
                        row
                            .columns[1633]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1633 to be a uint8!"),
                        row
                            .columns[1813]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1813 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1933]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1933 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[193]
                            .into_bool()
                            .copied()
                            .expect("Expected column 193 to be a bool!"),
                        row
                            .columns[433]
                            .into_bool()
                            .copied()
                            .expect("Expected column 433 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[74]
                            .into_u32()
                            .copied()
                            .expect("Expected column 74 to be a uint32!"),
                        row
                            .columns[314]
                            .into_u32()
                            .copied()
                            .expect("Expected column 314 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[554]
                            .into_u32()
                            .copied()
                            .expect("Expected column 554 to be a uint32!"),
                        row
                            .columns[794]
                            .into_u32()
                            .copied()
                            .expect("Expected column 794 to be a uint32!"),
                        row
                            .columns[1034]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1034 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[14]
                            .into_i32()
                            .copied()
                            .expect("Expected column 14 to be a int32!"),
                        row
                            .columns[254]
                            .into_i32()
                            .copied()
                            .expect("Expected column 254 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[134]
                            .into_i32()
                            .copied()
                            .expect("Expected column 134 to be a int32!"),
                        row
                            .columns[374]
                            .into_i32()
                            .copied()
                            .expect("Expected column 374 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[494]
                            .into_i32()
                            .copied()
                            .expect("Expected column 494 to be a int32!"),
                        row
                            .columns[734]
                            .into_i32()
                            .copied()
                            .expect("Expected column 734 to be a int32!"),
                        row
                            .columns[974]
                            .into_i32()
                            .copied()
                            .expect("Expected column 974 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1214]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1214 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1274]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1274 to be a int32!"),
                        row
                            .columns[1514]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1514 to be a int32!"),
                        row
                            .columns[1574]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1574 to be a int32!"),
                        row
                            .columns[1694]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1694 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1754]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1754 to be a int32!"),
                    Unknown2: row
                        .columns[1874]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1874 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[674]
                            .into_u16()
                            .copied()
                            .expect("Expected column 674 to be a uint16!"),
                        row
                            .columns[914]
                            .into_u16()
                            .copied()
                            .expect("Expected column 914 to be a uint16!"),
                        row
                            .columns[1154]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1154 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1994]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1994 to be a uint16!"),
                    CostType: [
                        row
                            .columns[614]
                            .into_u8()
                            .copied()
                            .expect("Expected column 614 to be a uint8!"),
                        row
                            .columns[854]
                            .into_u8()
                            .copied()
                            .expect("Expected column 854 to be a uint8!"),
                        row
                            .columns[1094]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1094 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1334]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1334 to be a uint8!"),
                        row
                            .columns[1394]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1394 to be a uint8!"),
                        row
                            .columns[1454]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1454 to be a uint8!"),
                        row
                            .columns[1634]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1634 to be a uint8!"),
                        row
                            .columns[1814]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1814 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1934]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1934 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[194]
                            .into_bool()
                            .copied()
                            .expect("Expected column 194 to be a bool!"),
                        row
                            .columns[434]
                            .into_bool()
                            .copied()
                            .expect("Expected column 434 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[75]
                            .into_u32()
                            .copied()
                            .expect("Expected column 75 to be a uint32!"),
                        row
                            .columns[315]
                            .into_u32()
                            .copied()
                            .expect("Expected column 315 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[555]
                            .into_u32()
                            .copied()
                            .expect("Expected column 555 to be a uint32!"),
                        row
                            .columns[795]
                            .into_u32()
                            .copied()
                            .expect("Expected column 795 to be a uint32!"),
                        row
                            .columns[1035]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1035 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[15]
                            .into_i32()
                            .copied()
                            .expect("Expected column 15 to be a int32!"),
                        row
                            .columns[255]
                            .into_i32()
                            .copied()
                            .expect("Expected column 255 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[135]
                            .into_i32()
                            .copied()
                            .expect("Expected column 135 to be a int32!"),
                        row
                            .columns[375]
                            .into_i32()
                            .copied()
                            .expect("Expected column 375 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[495]
                            .into_i32()
                            .copied()
                            .expect("Expected column 495 to be a int32!"),
                        row
                            .columns[735]
                            .into_i32()
                            .copied()
                            .expect("Expected column 735 to be a int32!"),
                        row
                            .columns[975]
                            .into_i32()
                            .copied()
                            .expect("Expected column 975 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1215]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1215 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1275]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1275 to be a int32!"),
                        row
                            .columns[1515]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1515 to be a int32!"),
                        row
                            .columns[1575]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1575 to be a int32!"),
                        row
                            .columns[1695]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1695 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1755]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1755 to be a int32!"),
                    Unknown2: row
                        .columns[1875]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1875 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[675]
                            .into_u16()
                            .copied()
                            .expect("Expected column 675 to be a uint16!"),
                        row
                            .columns[915]
                            .into_u16()
                            .copied()
                            .expect("Expected column 915 to be a uint16!"),
                        row
                            .columns[1155]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1155 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1995]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1995 to be a uint16!"),
                    CostType: [
                        row
                            .columns[615]
                            .into_u8()
                            .copied()
                            .expect("Expected column 615 to be a uint8!"),
                        row
                            .columns[855]
                            .into_u8()
                            .copied()
                            .expect("Expected column 855 to be a uint8!"),
                        row
                            .columns[1095]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1095 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1335]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1335 to be a uint8!"),
                        row
                            .columns[1395]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1395 to be a uint8!"),
                        row
                            .columns[1455]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1455 to be a uint8!"),
                        row
                            .columns[1635]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1635 to be a uint8!"),
                        row
                            .columns[1815]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1815 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1935]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1935 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[195]
                            .into_bool()
                            .copied()
                            .expect("Expected column 195 to be a bool!"),
                        row
                            .columns[435]
                            .into_bool()
                            .copied()
                            .expect("Expected column 435 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[76]
                            .into_u32()
                            .copied()
                            .expect("Expected column 76 to be a uint32!"),
                        row
                            .columns[316]
                            .into_u32()
                            .copied()
                            .expect("Expected column 316 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[556]
                            .into_u32()
                            .copied()
                            .expect("Expected column 556 to be a uint32!"),
                        row
                            .columns[796]
                            .into_u32()
                            .copied()
                            .expect("Expected column 796 to be a uint32!"),
                        row
                            .columns[1036]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1036 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[16]
                            .into_i32()
                            .copied()
                            .expect("Expected column 16 to be a int32!"),
                        row
                            .columns[256]
                            .into_i32()
                            .copied()
                            .expect("Expected column 256 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[136]
                            .into_i32()
                            .copied()
                            .expect("Expected column 136 to be a int32!"),
                        row
                            .columns[376]
                            .into_i32()
                            .copied()
                            .expect("Expected column 376 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[496]
                            .into_i32()
                            .copied()
                            .expect("Expected column 496 to be a int32!"),
                        row
                            .columns[736]
                            .into_i32()
                            .copied()
                            .expect("Expected column 736 to be a int32!"),
                        row
                            .columns[976]
                            .into_i32()
                            .copied()
                            .expect("Expected column 976 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1216]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1216 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1276]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1276 to be a int32!"),
                        row
                            .columns[1516]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1516 to be a int32!"),
                        row
                            .columns[1576]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1576 to be a int32!"),
                        row
                            .columns[1696]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1696 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1756]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1756 to be a int32!"),
                    Unknown2: row
                        .columns[1876]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1876 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[676]
                            .into_u16()
                            .copied()
                            .expect("Expected column 676 to be a uint16!"),
                        row
                            .columns[916]
                            .into_u16()
                            .copied()
                            .expect("Expected column 916 to be a uint16!"),
                        row
                            .columns[1156]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1156 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1996]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1996 to be a uint16!"),
                    CostType: [
                        row
                            .columns[616]
                            .into_u8()
                            .copied()
                            .expect("Expected column 616 to be a uint8!"),
                        row
                            .columns[856]
                            .into_u8()
                            .copied()
                            .expect("Expected column 856 to be a uint8!"),
                        row
                            .columns[1096]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1096 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1336]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1336 to be a uint8!"),
                        row
                            .columns[1396]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1396 to be a uint8!"),
                        row
                            .columns[1456]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1456 to be a uint8!"),
                        row
                            .columns[1636]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1636 to be a uint8!"),
                        row
                            .columns[1816]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1816 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1936]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1936 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[196]
                            .into_bool()
                            .copied()
                            .expect("Expected column 196 to be a bool!"),
                        row
                            .columns[436]
                            .into_bool()
                            .copied()
                            .expect("Expected column 436 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[77]
                            .into_u32()
                            .copied()
                            .expect("Expected column 77 to be a uint32!"),
                        row
                            .columns[317]
                            .into_u32()
                            .copied()
                            .expect("Expected column 317 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[557]
                            .into_u32()
                            .copied()
                            .expect("Expected column 557 to be a uint32!"),
                        row
                            .columns[797]
                            .into_u32()
                            .copied()
                            .expect("Expected column 797 to be a uint32!"),
                        row
                            .columns[1037]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1037 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[17]
                            .into_i32()
                            .copied()
                            .expect("Expected column 17 to be a int32!"),
                        row
                            .columns[257]
                            .into_i32()
                            .copied()
                            .expect("Expected column 257 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[137]
                            .into_i32()
                            .copied()
                            .expect("Expected column 137 to be a int32!"),
                        row
                            .columns[377]
                            .into_i32()
                            .copied()
                            .expect("Expected column 377 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[497]
                            .into_i32()
                            .copied()
                            .expect("Expected column 497 to be a int32!"),
                        row
                            .columns[737]
                            .into_i32()
                            .copied()
                            .expect("Expected column 737 to be a int32!"),
                        row
                            .columns[977]
                            .into_i32()
                            .copied()
                            .expect("Expected column 977 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1217]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1217 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1277]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1277 to be a int32!"),
                        row
                            .columns[1517]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1517 to be a int32!"),
                        row
                            .columns[1577]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1577 to be a int32!"),
                        row
                            .columns[1697]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1697 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1757]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1757 to be a int32!"),
                    Unknown2: row
                        .columns[1877]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1877 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[677]
                            .into_u16()
                            .copied()
                            .expect("Expected column 677 to be a uint16!"),
                        row
                            .columns[917]
                            .into_u16()
                            .copied()
                            .expect("Expected column 917 to be a uint16!"),
                        row
                            .columns[1157]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1157 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1997]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1997 to be a uint16!"),
                    CostType: [
                        row
                            .columns[617]
                            .into_u8()
                            .copied()
                            .expect("Expected column 617 to be a uint8!"),
                        row
                            .columns[857]
                            .into_u8()
                            .copied()
                            .expect("Expected column 857 to be a uint8!"),
                        row
                            .columns[1097]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1097 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1337]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1337 to be a uint8!"),
                        row
                            .columns[1397]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1397 to be a uint8!"),
                        row
                            .columns[1457]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1457 to be a uint8!"),
                        row
                            .columns[1637]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1637 to be a uint8!"),
                        row
                            .columns[1817]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1817 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1937]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1937 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[197]
                            .into_bool()
                            .copied()
                            .expect("Expected column 197 to be a bool!"),
                        row
                            .columns[437]
                            .into_bool()
                            .copied()
                            .expect("Expected column 437 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[78]
                            .into_u32()
                            .copied()
                            .expect("Expected column 78 to be a uint32!"),
                        row
                            .columns[318]
                            .into_u32()
                            .copied()
                            .expect("Expected column 318 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[558]
                            .into_u32()
                            .copied()
                            .expect("Expected column 558 to be a uint32!"),
                        row
                            .columns[798]
                            .into_u32()
                            .copied()
                            .expect("Expected column 798 to be a uint32!"),
                        row
                            .columns[1038]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1038 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[18]
                            .into_i32()
                            .copied()
                            .expect("Expected column 18 to be a int32!"),
                        row
                            .columns[258]
                            .into_i32()
                            .copied()
                            .expect("Expected column 258 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[138]
                            .into_i32()
                            .copied()
                            .expect("Expected column 138 to be a int32!"),
                        row
                            .columns[378]
                            .into_i32()
                            .copied()
                            .expect("Expected column 378 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[498]
                            .into_i32()
                            .copied()
                            .expect("Expected column 498 to be a int32!"),
                        row
                            .columns[738]
                            .into_i32()
                            .copied()
                            .expect("Expected column 738 to be a int32!"),
                        row
                            .columns[978]
                            .into_i32()
                            .copied()
                            .expect("Expected column 978 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1218]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1218 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1278]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1278 to be a int32!"),
                        row
                            .columns[1518]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1518 to be a int32!"),
                        row
                            .columns[1578]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1578 to be a int32!"),
                        row
                            .columns[1698]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1698 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1758]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1758 to be a int32!"),
                    Unknown2: row
                        .columns[1878]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1878 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[678]
                            .into_u16()
                            .copied()
                            .expect("Expected column 678 to be a uint16!"),
                        row
                            .columns[918]
                            .into_u16()
                            .copied()
                            .expect("Expected column 918 to be a uint16!"),
                        row
                            .columns[1158]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1158 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1998]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1998 to be a uint16!"),
                    CostType: [
                        row
                            .columns[618]
                            .into_u8()
                            .copied()
                            .expect("Expected column 618 to be a uint8!"),
                        row
                            .columns[858]
                            .into_u8()
                            .copied()
                            .expect("Expected column 858 to be a uint8!"),
                        row
                            .columns[1098]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1098 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1338]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1338 to be a uint8!"),
                        row
                            .columns[1398]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1398 to be a uint8!"),
                        row
                            .columns[1458]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1458 to be a uint8!"),
                        row
                            .columns[1638]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1638 to be a uint8!"),
                        row
                            .columns[1818]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1818 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1938]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1938 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[198]
                            .into_bool()
                            .copied()
                            .expect("Expected column 198 to be a bool!"),
                        row
                            .columns[438]
                            .into_bool()
                            .copied()
                            .expect("Expected column 438 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[79]
                            .into_u32()
                            .copied()
                            .expect("Expected column 79 to be a uint32!"),
                        row
                            .columns[319]
                            .into_u32()
                            .copied()
                            .expect("Expected column 319 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[559]
                            .into_u32()
                            .copied()
                            .expect("Expected column 559 to be a uint32!"),
                        row
                            .columns[799]
                            .into_u32()
                            .copied()
                            .expect("Expected column 799 to be a uint32!"),
                        row
                            .columns[1039]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1039 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[19]
                            .into_i32()
                            .copied()
                            .expect("Expected column 19 to be a int32!"),
                        row
                            .columns[259]
                            .into_i32()
                            .copied()
                            .expect("Expected column 259 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[139]
                            .into_i32()
                            .copied()
                            .expect("Expected column 139 to be a int32!"),
                        row
                            .columns[379]
                            .into_i32()
                            .copied()
                            .expect("Expected column 379 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[499]
                            .into_i32()
                            .copied()
                            .expect("Expected column 499 to be a int32!"),
                        row
                            .columns[739]
                            .into_i32()
                            .copied()
                            .expect("Expected column 739 to be a int32!"),
                        row
                            .columns[979]
                            .into_i32()
                            .copied()
                            .expect("Expected column 979 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1219]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1219 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1279]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1279 to be a int32!"),
                        row
                            .columns[1519]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1519 to be a int32!"),
                        row
                            .columns[1579]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1579 to be a int32!"),
                        row
                            .columns[1699]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1699 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1759]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1759 to be a int32!"),
                    Unknown2: row
                        .columns[1879]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1879 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[679]
                            .into_u16()
                            .copied()
                            .expect("Expected column 679 to be a uint16!"),
                        row
                            .columns[919]
                            .into_u16()
                            .copied()
                            .expect("Expected column 919 to be a uint16!"),
                        row
                            .columns[1159]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1159 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[1999]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1999 to be a uint16!"),
                    CostType: [
                        row
                            .columns[619]
                            .into_u8()
                            .copied()
                            .expect("Expected column 619 to be a uint8!"),
                        row
                            .columns[859]
                            .into_u8()
                            .copied()
                            .expect("Expected column 859 to be a uint8!"),
                        row
                            .columns[1099]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1099 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1339]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1339 to be a uint8!"),
                        row
                            .columns[1399]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1399 to be a uint8!"),
                        row
                            .columns[1459]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1459 to be a uint8!"),
                        row
                            .columns[1639]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1639 to be a uint8!"),
                        row
                            .columns[1819]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1819 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1939]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1939 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[199]
                            .into_bool()
                            .copied()
                            .expect("Expected column 199 to be a bool!"),
                        row
                            .columns[439]
                            .into_bool()
                            .copied()
                            .expect("Expected column 439 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[80]
                            .into_u32()
                            .copied()
                            .expect("Expected column 80 to be a uint32!"),
                        row
                            .columns[320]
                            .into_u32()
                            .copied()
                            .expect("Expected column 320 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[560]
                            .into_u32()
                            .copied()
                            .expect("Expected column 560 to be a uint32!"),
                        row
                            .columns[800]
                            .into_u32()
                            .copied()
                            .expect("Expected column 800 to be a uint32!"),
                        row
                            .columns[1040]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1040 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[20]
                            .into_i32()
                            .copied()
                            .expect("Expected column 20 to be a int32!"),
                        row
                            .columns[260]
                            .into_i32()
                            .copied()
                            .expect("Expected column 260 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[140]
                            .into_i32()
                            .copied()
                            .expect("Expected column 140 to be a int32!"),
                        row
                            .columns[380]
                            .into_i32()
                            .copied()
                            .expect("Expected column 380 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[500]
                            .into_i32()
                            .copied()
                            .expect("Expected column 500 to be a int32!"),
                        row
                            .columns[740]
                            .into_i32()
                            .copied()
                            .expect("Expected column 740 to be a int32!"),
                        row
                            .columns[980]
                            .into_i32()
                            .copied()
                            .expect("Expected column 980 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1220]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1220 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1280]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1280 to be a int32!"),
                        row
                            .columns[1520]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1520 to be a int32!"),
                        row
                            .columns[1580]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1580 to be a int32!"),
                        row
                            .columns[1700]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1700 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1760]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1760 to be a int32!"),
                    Unknown2: row
                        .columns[1880]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1880 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[680]
                            .into_u16()
                            .copied()
                            .expect("Expected column 680 to be a uint16!"),
                        row
                            .columns[920]
                            .into_u16()
                            .copied()
                            .expect("Expected column 920 to be a uint16!"),
                        row
                            .columns[1160]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1160 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2000]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2000 to be a uint16!"),
                    CostType: [
                        row
                            .columns[620]
                            .into_u8()
                            .copied()
                            .expect("Expected column 620 to be a uint8!"),
                        row
                            .columns[860]
                            .into_u8()
                            .copied()
                            .expect("Expected column 860 to be a uint8!"),
                        row
                            .columns[1100]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1100 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1340]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1340 to be a uint8!"),
                        row
                            .columns[1400]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1400 to be a uint8!"),
                        row
                            .columns[1460]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1460 to be a uint8!"),
                        row
                            .columns[1640]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1640 to be a uint8!"),
                        row
                            .columns[1820]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1820 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1940]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1940 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[200]
                            .into_bool()
                            .copied()
                            .expect("Expected column 200 to be a bool!"),
                        row
                            .columns[440]
                            .into_bool()
                            .copied()
                            .expect("Expected column 440 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[81]
                            .into_u32()
                            .copied()
                            .expect("Expected column 81 to be a uint32!"),
                        row
                            .columns[321]
                            .into_u32()
                            .copied()
                            .expect("Expected column 321 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[561]
                            .into_u32()
                            .copied()
                            .expect("Expected column 561 to be a uint32!"),
                        row
                            .columns[801]
                            .into_u32()
                            .copied()
                            .expect("Expected column 801 to be a uint32!"),
                        row
                            .columns[1041]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1041 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[21]
                            .into_i32()
                            .copied()
                            .expect("Expected column 21 to be a int32!"),
                        row
                            .columns[261]
                            .into_i32()
                            .copied()
                            .expect("Expected column 261 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[141]
                            .into_i32()
                            .copied()
                            .expect("Expected column 141 to be a int32!"),
                        row
                            .columns[381]
                            .into_i32()
                            .copied()
                            .expect("Expected column 381 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[501]
                            .into_i32()
                            .copied()
                            .expect("Expected column 501 to be a int32!"),
                        row
                            .columns[741]
                            .into_i32()
                            .copied()
                            .expect("Expected column 741 to be a int32!"),
                        row
                            .columns[981]
                            .into_i32()
                            .copied()
                            .expect("Expected column 981 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1221]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1221 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1281]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1281 to be a int32!"),
                        row
                            .columns[1521]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1521 to be a int32!"),
                        row
                            .columns[1581]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1581 to be a int32!"),
                        row
                            .columns[1701]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1701 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1761]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1761 to be a int32!"),
                    Unknown2: row
                        .columns[1881]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1881 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[681]
                            .into_u16()
                            .copied()
                            .expect("Expected column 681 to be a uint16!"),
                        row
                            .columns[921]
                            .into_u16()
                            .copied()
                            .expect("Expected column 921 to be a uint16!"),
                        row
                            .columns[1161]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1161 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2001]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2001 to be a uint16!"),
                    CostType: [
                        row
                            .columns[621]
                            .into_u8()
                            .copied()
                            .expect("Expected column 621 to be a uint8!"),
                        row
                            .columns[861]
                            .into_u8()
                            .copied()
                            .expect("Expected column 861 to be a uint8!"),
                        row
                            .columns[1101]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1101 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1341]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1341 to be a uint8!"),
                        row
                            .columns[1401]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1401 to be a uint8!"),
                        row
                            .columns[1461]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1461 to be a uint8!"),
                        row
                            .columns[1641]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1641 to be a uint8!"),
                        row
                            .columns[1821]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1821 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1941]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1941 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[201]
                            .into_bool()
                            .copied()
                            .expect("Expected column 201 to be a bool!"),
                        row
                            .columns[441]
                            .into_bool()
                            .copied()
                            .expect("Expected column 441 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[82]
                            .into_u32()
                            .copied()
                            .expect("Expected column 82 to be a uint32!"),
                        row
                            .columns[322]
                            .into_u32()
                            .copied()
                            .expect("Expected column 322 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[562]
                            .into_u32()
                            .copied()
                            .expect("Expected column 562 to be a uint32!"),
                        row
                            .columns[802]
                            .into_u32()
                            .copied()
                            .expect("Expected column 802 to be a uint32!"),
                        row
                            .columns[1042]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1042 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[22]
                            .into_i32()
                            .copied()
                            .expect("Expected column 22 to be a int32!"),
                        row
                            .columns[262]
                            .into_i32()
                            .copied()
                            .expect("Expected column 262 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[142]
                            .into_i32()
                            .copied()
                            .expect("Expected column 142 to be a int32!"),
                        row
                            .columns[382]
                            .into_i32()
                            .copied()
                            .expect("Expected column 382 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[502]
                            .into_i32()
                            .copied()
                            .expect("Expected column 502 to be a int32!"),
                        row
                            .columns[742]
                            .into_i32()
                            .copied()
                            .expect("Expected column 742 to be a int32!"),
                        row
                            .columns[982]
                            .into_i32()
                            .copied()
                            .expect("Expected column 982 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1222]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1222 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1282]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1282 to be a int32!"),
                        row
                            .columns[1522]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1522 to be a int32!"),
                        row
                            .columns[1582]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1582 to be a int32!"),
                        row
                            .columns[1702]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1702 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1762]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1762 to be a int32!"),
                    Unknown2: row
                        .columns[1882]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1882 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[682]
                            .into_u16()
                            .copied()
                            .expect("Expected column 682 to be a uint16!"),
                        row
                            .columns[922]
                            .into_u16()
                            .copied()
                            .expect("Expected column 922 to be a uint16!"),
                        row
                            .columns[1162]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1162 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2002]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2002 to be a uint16!"),
                    CostType: [
                        row
                            .columns[622]
                            .into_u8()
                            .copied()
                            .expect("Expected column 622 to be a uint8!"),
                        row
                            .columns[862]
                            .into_u8()
                            .copied()
                            .expect("Expected column 862 to be a uint8!"),
                        row
                            .columns[1102]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1102 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1342]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1342 to be a uint8!"),
                        row
                            .columns[1402]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1402 to be a uint8!"),
                        row
                            .columns[1462]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1462 to be a uint8!"),
                        row
                            .columns[1642]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1642 to be a uint8!"),
                        row
                            .columns[1822]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1822 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1942]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1942 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[202]
                            .into_bool()
                            .copied()
                            .expect("Expected column 202 to be a bool!"),
                        row
                            .columns[442]
                            .into_bool()
                            .copied()
                            .expect("Expected column 442 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[83]
                            .into_u32()
                            .copied()
                            .expect("Expected column 83 to be a uint32!"),
                        row
                            .columns[323]
                            .into_u32()
                            .copied()
                            .expect("Expected column 323 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[563]
                            .into_u32()
                            .copied()
                            .expect("Expected column 563 to be a uint32!"),
                        row
                            .columns[803]
                            .into_u32()
                            .copied()
                            .expect("Expected column 803 to be a uint32!"),
                        row
                            .columns[1043]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1043 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[23]
                            .into_i32()
                            .copied()
                            .expect("Expected column 23 to be a int32!"),
                        row
                            .columns[263]
                            .into_i32()
                            .copied()
                            .expect("Expected column 263 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[143]
                            .into_i32()
                            .copied()
                            .expect("Expected column 143 to be a int32!"),
                        row
                            .columns[383]
                            .into_i32()
                            .copied()
                            .expect("Expected column 383 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[503]
                            .into_i32()
                            .copied()
                            .expect("Expected column 503 to be a int32!"),
                        row
                            .columns[743]
                            .into_i32()
                            .copied()
                            .expect("Expected column 743 to be a int32!"),
                        row
                            .columns[983]
                            .into_i32()
                            .copied()
                            .expect("Expected column 983 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1223]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1223 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1283]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1283 to be a int32!"),
                        row
                            .columns[1523]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1523 to be a int32!"),
                        row
                            .columns[1583]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1583 to be a int32!"),
                        row
                            .columns[1703]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1703 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1763]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1763 to be a int32!"),
                    Unknown2: row
                        .columns[1883]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1883 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[683]
                            .into_u16()
                            .copied()
                            .expect("Expected column 683 to be a uint16!"),
                        row
                            .columns[923]
                            .into_u16()
                            .copied()
                            .expect("Expected column 923 to be a uint16!"),
                        row
                            .columns[1163]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1163 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2003]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2003 to be a uint16!"),
                    CostType: [
                        row
                            .columns[623]
                            .into_u8()
                            .copied()
                            .expect("Expected column 623 to be a uint8!"),
                        row
                            .columns[863]
                            .into_u8()
                            .copied()
                            .expect("Expected column 863 to be a uint8!"),
                        row
                            .columns[1103]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1103 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1343]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1343 to be a uint8!"),
                        row
                            .columns[1403]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1403 to be a uint8!"),
                        row
                            .columns[1463]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1463 to be a uint8!"),
                        row
                            .columns[1643]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1643 to be a uint8!"),
                        row
                            .columns[1823]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1823 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1943]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1943 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[203]
                            .into_bool()
                            .copied()
                            .expect("Expected column 203 to be a bool!"),
                        row
                            .columns[443]
                            .into_bool()
                            .copied()
                            .expect("Expected column 443 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[84]
                            .into_u32()
                            .copied()
                            .expect("Expected column 84 to be a uint32!"),
                        row
                            .columns[324]
                            .into_u32()
                            .copied()
                            .expect("Expected column 324 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[564]
                            .into_u32()
                            .copied()
                            .expect("Expected column 564 to be a uint32!"),
                        row
                            .columns[804]
                            .into_u32()
                            .copied()
                            .expect("Expected column 804 to be a uint32!"),
                        row
                            .columns[1044]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1044 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[24]
                            .into_i32()
                            .copied()
                            .expect("Expected column 24 to be a int32!"),
                        row
                            .columns[264]
                            .into_i32()
                            .copied()
                            .expect("Expected column 264 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[144]
                            .into_i32()
                            .copied()
                            .expect("Expected column 144 to be a int32!"),
                        row
                            .columns[384]
                            .into_i32()
                            .copied()
                            .expect("Expected column 384 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[504]
                            .into_i32()
                            .copied()
                            .expect("Expected column 504 to be a int32!"),
                        row
                            .columns[744]
                            .into_i32()
                            .copied()
                            .expect("Expected column 744 to be a int32!"),
                        row
                            .columns[984]
                            .into_i32()
                            .copied()
                            .expect("Expected column 984 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1224]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1224 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1284]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1284 to be a int32!"),
                        row
                            .columns[1524]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1524 to be a int32!"),
                        row
                            .columns[1584]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1584 to be a int32!"),
                        row
                            .columns[1704]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1704 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1764]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1764 to be a int32!"),
                    Unknown2: row
                        .columns[1884]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1884 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[684]
                            .into_u16()
                            .copied()
                            .expect("Expected column 684 to be a uint16!"),
                        row
                            .columns[924]
                            .into_u16()
                            .copied()
                            .expect("Expected column 924 to be a uint16!"),
                        row
                            .columns[1164]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1164 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2004]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2004 to be a uint16!"),
                    CostType: [
                        row
                            .columns[624]
                            .into_u8()
                            .copied()
                            .expect("Expected column 624 to be a uint8!"),
                        row
                            .columns[864]
                            .into_u8()
                            .copied()
                            .expect("Expected column 864 to be a uint8!"),
                        row
                            .columns[1104]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1104 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1344]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1344 to be a uint8!"),
                        row
                            .columns[1404]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1404 to be a uint8!"),
                        row
                            .columns[1464]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1464 to be a uint8!"),
                        row
                            .columns[1644]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1644 to be a uint8!"),
                        row
                            .columns[1824]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1824 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1944]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1944 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[204]
                            .into_bool()
                            .copied()
                            .expect("Expected column 204 to be a bool!"),
                        row
                            .columns[444]
                            .into_bool()
                            .copied()
                            .expect("Expected column 444 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[85]
                            .into_u32()
                            .copied()
                            .expect("Expected column 85 to be a uint32!"),
                        row
                            .columns[325]
                            .into_u32()
                            .copied()
                            .expect("Expected column 325 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[565]
                            .into_u32()
                            .copied()
                            .expect("Expected column 565 to be a uint32!"),
                        row
                            .columns[805]
                            .into_u32()
                            .copied()
                            .expect("Expected column 805 to be a uint32!"),
                        row
                            .columns[1045]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1045 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[25]
                            .into_i32()
                            .copied()
                            .expect("Expected column 25 to be a int32!"),
                        row
                            .columns[265]
                            .into_i32()
                            .copied()
                            .expect("Expected column 265 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[145]
                            .into_i32()
                            .copied()
                            .expect("Expected column 145 to be a int32!"),
                        row
                            .columns[385]
                            .into_i32()
                            .copied()
                            .expect("Expected column 385 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[505]
                            .into_i32()
                            .copied()
                            .expect("Expected column 505 to be a int32!"),
                        row
                            .columns[745]
                            .into_i32()
                            .copied()
                            .expect("Expected column 745 to be a int32!"),
                        row
                            .columns[985]
                            .into_i32()
                            .copied()
                            .expect("Expected column 985 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1225]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1225 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1285]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1285 to be a int32!"),
                        row
                            .columns[1525]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1525 to be a int32!"),
                        row
                            .columns[1585]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1585 to be a int32!"),
                        row
                            .columns[1705]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1705 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1765]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1765 to be a int32!"),
                    Unknown2: row
                        .columns[1885]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1885 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[685]
                            .into_u16()
                            .copied()
                            .expect("Expected column 685 to be a uint16!"),
                        row
                            .columns[925]
                            .into_u16()
                            .copied()
                            .expect("Expected column 925 to be a uint16!"),
                        row
                            .columns[1165]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1165 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2005]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2005 to be a uint16!"),
                    CostType: [
                        row
                            .columns[625]
                            .into_u8()
                            .copied()
                            .expect("Expected column 625 to be a uint8!"),
                        row
                            .columns[865]
                            .into_u8()
                            .copied()
                            .expect("Expected column 865 to be a uint8!"),
                        row
                            .columns[1105]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1105 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1345]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1345 to be a uint8!"),
                        row
                            .columns[1405]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1405 to be a uint8!"),
                        row
                            .columns[1465]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1465 to be a uint8!"),
                        row
                            .columns[1645]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1645 to be a uint8!"),
                        row
                            .columns[1825]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1825 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1945]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1945 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[205]
                            .into_bool()
                            .copied()
                            .expect("Expected column 205 to be a bool!"),
                        row
                            .columns[445]
                            .into_bool()
                            .copied()
                            .expect("Expected column 445 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[86]
                            .into_u32()
                            .copied()
                            .expect("Expected column 86 to be a uint32!"),
                        row
                            .columns[326]
                            .into_u32()
                            .copied()
                            .expect("Expected column 326 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[566]
                            .into_u32()
                            .copied()
                            .expect("Expected column 566 to be a uint32!"),
                        row
                            .columns[806]
                            .into_u32()
                            .copied()
                            .expect("Expected column 806 to be a uint32!"),
                        row
                            .columns[1046]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1046 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[26]
                            .into_i32()
                            .copied()
                            .expect("Expected column 26 to be a int32!"),
                        row
                            .columns[266]
                            .into_i32()
                            .copied()
                            .expect("Expected column 266 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[146]
                            .into_i32()
                            .copied()
                            .expect("Expected column 146 to be a int32!"),
                        row
                            .columns[386]
                            .into_i32()
                            .copied()
                            .expect("Expected column 386 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[506]
                            .into_i32()
                            .copied()
                            .expect("Expected column 506 to be a int32!"),
                        row
                            .columns[746]
                            .into_i32()
                            .copied()
                            .expect("Expected column 746 to be a int32!"),
                        row
                            .columns[986]
                            .into_i32()
                            .copied()
                            .expect("Expected column 986 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1226]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1226 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1286]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1286 to be a int32!"),
                        row
                            .columns[1526]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1526 to be a int32!"),
                        row
                            .columns[1586]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1586 to be a int32!"),
                        row
                            .columns[1706]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1706 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1766]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1766 to be a int32!"),
                    Unknown2: row
                        .columns[1886]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1886 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[686]
                            .into_u16()
                            .copied()
                            .expect("Expected column 686 to be a uint16!"),
                        row
                            .columns[926]
                            .into_u16()
                            .copied()
                            .expect("Expected column 926 to be a uint16!"),
                        row
                            .columns[1166]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1166 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2006]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2006 to be a uint16!"),
                    CostType: [
                        row
                            .columns[626]
                            .into_u8()
                            .copied()
                            .expect("Expected column 626 to be a uint8!"),
                        row
                            .columns[866]
                            .into_u8()
                            .copied()
                            .expect("Expected column 866 to be a uint8!"),
                        row
                            .columns[1106]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1106 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1346]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1346 to be a uint8!"),
                        row
                            .columns[1406]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1406 to be a uint8!"),
                        row
                            .columns[1466]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1466 to be a uint8!"),
                        row
                            .columns[1646]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1646 to be a uint8!"),
                        row
                            .columns[1826]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1826 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1946]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1946 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[206]
                            .into_bool()
                            .copied()
                            .expect("Expected column 206 to be a bool!"),
                        row
                            .columns[446]
                            .into_bool()
                            .copied()
                            .expect("Expected column 446 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[87]
                            .into_u32()
                            .copied()
                            .expect("Expected column 87 to be a uint32!"),
                        row
                            .columns[327]
                            .into_u32()
                            .copied()
                            .expect("Expected column 327 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[567]
                            .into_u32()
                            .copied()
                            .expect("Expected column 567 to be a uint32!"),
                        row
                            .columns[807]
                            .into_u32()
                            .copied()
                            .expect("Expected column 807 to be a uint32!"),
                        row
                            .columns[1047]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1047 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[27]
                            .into_i32()
                            .copied()
                            .expect("Expected column 27 to be a int32!"),
                        row
                            .columns[267]
                            .into_i32()
                            .copied()
                            .expect("Expected column 267 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[147]
                            .into_i32()
                            .copied()
                            .expect("Expected column 147 to be a int32!"),
                        row
                            .columns[387]
                            .into_i32()
                            .copied()
                            .expect("Expected column 387 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[507]
                            .into_i32()
                            .copied()
                            .expect("Expected column 507 to be a int32!"),
                        row
                            .columns[747]
                            .into_i32()
                            .copied()
                            .expect("Expected column 747 to be a int32!"),
                        row
                            .columns[987]
                            .into_i32()
                            .copied()
                            .expect("Expected column 987 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1227]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1227 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1287]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1287 to be a int32!"),
                        row
                            .columns[1527]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1527 to be a int32!"),
                        row
                            .columns[1587]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1587 to be a int32!"),
                        row
                            .columns[1707]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1707 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1767]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1767 to be a int32!"),
                    Unknown2: row
                        .columns[1887]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1887 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[687]
                            .into_u16()
                            .copied()
                            .expect("Expected column 687 to be a uint16!"),
                        row
                            .columns[927]
                            .into_u16()
                            .copied()
                            .expect("Expected column 927 to be a uint16!"),
                        row
                            .columns[1167]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1167 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2007]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2007 to be a uint16!"),
                    CostType: [
                        row
                            .columns[627]
                            .into_u8()
                            .copied()
                            .expect("Expected column 627 to be a uint8!"),
                        row
                            .columns[867]
                            .into_u8()
                            .copied()
                            .expect("Expected column 867 to be a uint8!"),
                        row
                            .columns[1107]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1107 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1347]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1347 to be a uint8!"),
                        row
                            .columns[1407]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1407 to be a uint8!"),
                        row
                            .columns[1467]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1467 to be a uint8!"),
                        row
                            .columns[1647]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1647 to be a uint8!"),
                        row
                            .columns[1827]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1827 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1947]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1947 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[207]
                            .into_bool()
                            .copied()
                            .expect("Expected column 207 to be a bool!"),
                        row
                            .columns[447]
                            .into_bool()
                            .copied()
                            .expect("Expected column 447 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[88]
                            .into_u32()
                            .copied()
                            .expect("Expected column 88 to be a uint32!"),
                        row
                            .columns[328]
                            .into_u32()
                            .copied()
                            .expect("Expected column 328 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[568]
                            .into_u32()
                            .copied()
                            .expect("Expected column 568 to be a uint32!"),
                        row
                            .columns[808]
                            .into_u32()
                            .copied()
                            .expect("Expected column 808 to be a uint32!"),
                        row
                            .columns[1048]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1048 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[28]
                            .into_i32()
                            .copied()
                            .expect("Expected column 28 to be a int32!"),
                        row
                            .columns[268]
                            .into_i32()
                            .copied()
                            .expect("Expected column 268 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[148]
                            .into_i32()
                            .copied()
                            .expect("Expected column 148 to be a int32!"),
                        row
                            .columns[388]
                            .into_i32()
                            .copied()
                            .expect("Expected column 388 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[508]
                            .into_i32()
                            .copied()
                            .expect("Expected column 508 to be a int32!"),
                        row
                            .columns[748]
                            .into_i32()
                            .copied()
                            .expect("Expected column 748 to be a int32!"),
                        row
                            .columns[988]
                            .into_i32()
                            .copied()
                            .expect("Expected column 988 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1228]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1228 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1288]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1288 to be a int32!"),
                        row
                            .columns[1528]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1528 to be a int32!"),
                        row
                            .columns[1588]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1588 to be a int32!"),
                        row
                            .columns[1708]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1708 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1768]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1768 to be a int32!"),
                    Unknown2: row
                        .columns[1888]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1888 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[688]
                            .into_u16()
                            .copied()
                            .expect("Expected column 688 to be a uint16!"),
                        row
                            .columns[928]
                            .into_u16()
                            .copied()
                            .expect("Expected column 928 to be a uint16!"),
                        row
                            .columns[1168]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1168 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2008]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2008 to be a uint16!"),
                    CostType: [
                        row
                            .columns[628]
                            .into_u8()
                            .copied()
                            .expect("Expected column 628 to be a uint8!"),
                        row
                            .columns[868]
                            .into_u8()
                            .copied()
                            .expect("Expected column 868 to be a uint8!"),
                        row
                            .columns[1108]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1108 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1348]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1348 to be a uint8!"),
                        row
                            .columns[1408]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1408 to be a uint8!"),
                        row
                            .columns[1468]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1468 to be a uint8!"),
                        row
                            .columns[1648]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1648 to be a uint8!"),
                        row
                            .columns[1828]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1828 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1948]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1948 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[208]
                            .into_bool()
                            .copied()
                            .expect("Expected column 208 to be a bool!"),
                        row
                            .columns[448]
                            .into_bool()
                            .copied()
                            .expect("Expected column 448 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[89]
                            .into_u32()
                            .copied()
                            .expect("Expected column 89 to be a uint32!"),
                        row
                            .columns[329]
                            .into_u32()
                            .copied()
                            .expect("Expected column 329 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[569]
                            .into_u32()
                            .copied()
                            .expect("Expected column 569 to be a uint32!"),
                        row
                            .columns[809]
                            .into_u32()
                            .copied()
                            .expect("Expected column 809 to be a uint32!"),
                        row
                            .columns[1049]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1049 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[29]
                            .into_i32()
                            .copied()
                            .expect("Expected column 29 to be a int32!"),
                        row
                            .columns[269]
                            .into_i32()
                            .copied()
                            .expect("Expected column 269 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[149]
                            .into_i32()
                            .copied()
                            .expect("Expected column 149 to be a int32!"),
                        row
                            .columns[389]
                            .into_i32()
                            .copied()
                            .expect("Expected column 389 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[509]
                            .into_i32()
                            .copied()
                            .expect("Expected column 509 to be a int32!"),
                        row
                            .columns[749]
                            .into_i32()
                            .copied()
                            .expect("Expected column 749 to be a int32!"),
                        row
                            .columns[989]
                            .into_i32()
                            .copied()
                            .expect("Expected column 989 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1229]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1229 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1289]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1289 to be a int32!"),
                        row
                            .columns[1529]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1529 to be a int32!"),
                        row
                            .columns[1589]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1589 to be a int32!"),
                        row
                            .columns[1709]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1709 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1769]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1769 to be a int32!"),
                    Unknown2: row
                        .columns[1889]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1889 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[689]
                            .into_u16()
                            .copied()
                            .expect("Expected column 689 to be a uint16!"),
                        row
                            .columns[929]
                            .into_u16()
                            .copied()
                            .expect("Expected column 929 to be a uint16!"),
                        row
                            .columns[1169]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1169 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2009]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2009 to be a uint16!"),
                    CostType: [
                        row
                            .columns[629]
                            .into_u8()
                            .copied()
                            .expect("Expected column 629 to be a uint8!"),
                        row
                            .columns[869]
                            .into_u8()
                            .copied()
                            .expect("Expected column 869 to be a uint8!"),
                        row
                            .columns[1109]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1109 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1349]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1349 to be a uint8!"),
                        row
                            .columns[1409]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1409 to be a uint8!"),
                        row
                            .columns[1469]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1469 to be a uint8!"),
                        row
                            .columns[1649]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1649 to be a uint8!"),
                        row
                            .columns[1829]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1829 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1949]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1949 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[209]
                            .into_bool()
                            .copied()
                            .expect("Expected column 209 to be a bool!"),
                        row
                            .columns[449]
                            .into_bool()
                            .copied()
                            .expect("Expected column 449 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[90]
                            .into_u32()
                            .copied()
                            .expect("Expected column 90 to be a uint32!"),
                        row
                            .columns[330]
                            .into_u32()
                            .copied()
                            .expect("Expected column 330 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[570]
                            .into_u32()
                            .copied()
                            .expect("Expected column 570 to be a uint32!"),
                        row
                            .columns[810]
                            .into_u32()
                            .copied()
                            .expect("Expected column 810 to be a uint32!"),
                        row
                            .columns[1050]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1050 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[30]
                            .into_i32()
                            .copied()
                            .expect("Expected column 30 to be a int32!"),
                        row
                            .columns[270]
                            .into_i32()
                            .copied()
                            .expect("Expected column 270 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[150]
                            .into_i32()
                            .copied()
                            .expect("Expected column 150 to be a int32!"),
                        row
                            .columns[390]
                            .into_i32()
                            .copied()
                            .expect("Expected column 390 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[510]
                            .into_i32()
                            .copied()
                            .expect("Expected column 510 to be a int32!"),
                        row
                            .columns[750]
                            .into_i32()
                            .copied()
                            .expect("Expected column 750 to be a int32!"),
                        row
                            .columns[990]
                            .into_i32()
                            .copied()
                            .expect("Expected column 990 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1230]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1230 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1290]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1290 to be a int32!"),
                        row
                            .columns[1530]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1530 to be a int32!"),
                        row
                            .columns[1590]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1590 to be a int32!"),
                        row
                            .columns[1710]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1710 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1770]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1770 to be a int32!"),
                    Unknown2: row
                        .columns[1890]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1890 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[690]
                            .into_u16()
                            .copied()
                            .expect("Expected column 690 to be a uint16!"),
                        row
                            .columns[930]
                            .into_u16()
                            .copied()
                            .expect("Expected column 930 to be a uint16!"),
                        row
                            .columns[1170]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1170 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2010]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2010 to be a uint16!"),
                    CostType: [
                        row
                            .columns[630]
                            .into_u8()
                            .copied()
                            .expect("Expected column 630 to be a uint8!"),
                        row
                            .columns[870]
                            .into_u8()
                            .copied()
                            .expect("Expected column 870 to be a uint8!"),
                        row
                            .columns[1110]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1110 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1350]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1350 to be a uint8!"),
                        row
                            .columns[1410]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1410 to be a uint8!"),
                        row
                            .columns[1470]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1470 to be a uint8!"),
                        row
                            .columns[1650]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1650 to be a uint8!"),
                        row
                            .columns[1830]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1830 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1950]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1950 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[210]
                            .into_bool()
                            .copied()
                            .expect("Expected column 210 to be a bool!"),
                        row
                            .columns[450]
                            .into_bool()
                            .copied()
                            .expect("Expected column 450 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[91]
                            .into_u32()
                            .copied()
                            .expect("Expected column 91 to be a uint32!"),
                        row
                            .columns[331]
                            .into_u32()
                            .copied()
                            .expect("Expected column 331 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[571]
                            .into_u32()
                            .copied()
                            .expect("Expected column 571 to be a uint32!"),
                        row
                            .columns[811]
                            .into_u32()
                            .copied()
                            .expect("Expected column 811 to be a uint32!"),
                        row
                            .columns[1051]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1051 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[31]
                            .into_i32()
                            .copied()
                            .expect("Expected column 31 to be a int32!"),
                        row
                            .columns[271]
                            .into_i32()
                            .copied()
                            .expect("Expected column 271 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[151]
                            .into_i32()
                            .copied()
                            .expect("Expected column 151 to be a int32!"),
                        row
                            .columns[391]
                            .into_i32()
                            .copied()
                            .expect("Expected column 391 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[511]
                            .into_i32()
                            .copied()
                            .expect("Expected column 511 to be a int32!"),
                        row
                            .columns[751]
                            .into_i32()
                            .copied()
                            .expect("Expected column 751 to be a int32!"),
                        row
                            .columns[991]
                            .into_i32()
                            .copied()
                            .expect("Expected column 991 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1231]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1231 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1291]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1291 to be a int32!"),
                        row
                            .columns[1531]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1531 to be a int32!"),
                        row
                            .columns[1591]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1591 to be a int32!"),
                        row
                            .columns[1711]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1711 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1771]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1771 to be a int32!"),
                    Unknown2: row
                        .columns[1891]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1891 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[691]
                            .into_u16()
                            .copied()
                            .expect("Expected column 691 to be a uint16!"),
                        row
                            .columns[931]
                            .into_u16()
                            .copied()
                            .expect("Expected column 931 to be a uint16!"),
                        row
                            .columns[1171]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1171 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2011]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2011 to be a uint16!"),
                    CostType: [
                        row
                            .columns[631]
                            .into_u8()
                            .copied()
                            .expect("Expected column 631 to be a uint8!"),
                        row
                            .columns[871]
                            .into_u8()
                            .copied()
                            .expect("Expected column 871 to be a uint8!"),
                        row
                            .columns[1111]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1111 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1351]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1351 to be a uint8!"),
                        row
                            .columns[1411]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1411 to be a uint8!"),
                        row
                            .columns[1471]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1471 to be a uint8!"),
                        row
                            .columns[1651]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1651 to be a uint8!"),
                        row
                            .columns[1831]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1831 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1951]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1951 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[211]
                            .into_bool()
                            .copied()
                            .expect("Expected column 211 to be a bool!"),
                        row
                            .columns[451]
                            .into_bool()
                            .copied()
                            .expect("Expected column 451 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[92]
                            .into_u32()
                            .copied()
                            .expect("Expected column 92 to be a uint32!"),
                        row
                            .columns[332]
                            .into_u32()
                            .copied()
                            .expect("Expected column 332 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[572]
                            .into_u32()
                            .copied()
                            .expect("Expected column 572 to be a uint32!"),
                        row
                            .columns[812]
                            .into_u32()
                            .copied()
                            .expect("Expected column 812 to be a uint32!"),
                        row
                            .columns[1052]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1052 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[32]
                            .into_i32()
                            .copied()
                            .expect("Expected column 32 to be a int32!"),
                        row
                            .columns[272]
                            .into_i32()
                            .copied()
                            .expect("Expected column 272 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[152]
                            .into_i32()
                            .copied()
                            .expect("Expected column 152 to be a int32!"),
                        row
                            .columns[392]
                            .into_i32()
                            .copied()
                            .expect("Expected column 392 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[512]
                            .into_i32()
                            .copied()
                            .expect("Expected column 512 to be a int32!"),
                        row
                            .columns[752]
                            .into_i32()
                            .copied()
                            .expect("Expected column 752 to be a int32!"),
                        row
                            .columns[992]
                            .into_i32()
                            .copied()
                            .expect("Expected column 992 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1232]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1232 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1292]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1292 to be a int32!"),
                        row
                            .columns[1532]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1532 to be a int32!"),
                        row
                            .columns[1592]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1592 to be a int32!"),
                        row
                            .columns[1712]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1712 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1772]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1772 to be a int32!"),
                    Unknown2: row
                        .columns[1892]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1892 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[692]
                            .into_u16()
                            .copied()
                            .expect("Expected column 692 to be a uint16!"),
                        row
                            .columns[932]
                            .into_u16()
                            .copied()
                            .expect("Expected column 932 to be a uint16!"),
                        row
                            .columns[1172]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1172 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2012]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2012 to be a uint16!"),
                    CostType: [
                        row
                            .columns[632]
                            .into_u8()
                            .copied()
                            .expect("Expected column 632 to be a uint8!"),
                        row
                            .columns[872]
                            .into_u8()
                            .copied()
                            .expect("Expected column 872 to be a uint8!"),
                        row
                            .columns[1112]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1112 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1352]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1352 to be a uint8!"),
                        row
                            .columns[1412]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1412 to be a uint8!"),
                        row
                            .columns[1472]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1472 to be a uint8!"),
                        row
                            .columns[1652]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1652 to be a uint8!"),
                        row
                            .columns[1832]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1832 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1952]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1952 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[212]
                            .into_bool()
                            .copied()
                            .expect("Expected column 212 to be a bool!"),
                        row
                            .columns[452]
                            .into_bool()
                            .copied()
                            .expect("Expected column 452 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[93]
                            .into_u32()
                            .copied()
                            .expect("Expected column 93 to be a uint32!"),
                        row
                            .columns[333]
                            .into_u32()
                            .copied()
                            .expect("Expected column 333 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[573]
                            .into_u32()
                            .copied()
                            .expect("Expected column 573 to be a uint32!"),
                        row
                            .columns[813]
                            .into_u32()
                            .copied()
                            .expect("Expected column 813 to be a uint32!"),
                        row
                            .columns[1053]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1053 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[33]
                            .into_i32()
                            .copied()
                            .expect("Expected column 33 to be a int32!"),
                        row
                            .columns[273]
                            .into_i32()
                            .copied()
                            .expect("Expected column 273 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[153]
                            .into_i32()
                            .copied()
                            .expect("Expected column 153 to be a int32!"),
                        row
                            .columns[393]
                            .into_i32()
                            .copied()
                            .expect("Expected column 393 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[513]
                            .into_i32()
                            .copied()
                            .expect("Expected column 513 to be a int32!"),
                        row
                            .columns[753]
                            .into_i32()
                            .copied()
                            .expect("Expected column 753 to be a int32!"),
                        row
                            .columns[993]
                            .into_i32()
                            .copied()
                            .expect("Expected column 993 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1233]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1233 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1293]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1293 to be a int32!"),
                        row
                            .columns[1533]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1533 to be a int32!"),
                        row
                            .columns[1593]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1593 to be a int32!"),
                        row
                            .columns[1713]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1713 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1773]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1773 to be a int32!"),
                    Unknown2: row
                        .columns[1893]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1893 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[693]
                            .into_u16()
                            .copied()
                            .expect("Expected column 693 to be a uint16!"),
                        row
                            .columns[933]
                            .into_u16()
                            .copied()
                            .expect("Expected column 933 to be a uint16!"),
                        row
                            .columns[1173]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1173 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2013]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2013 to be a uint16!"),
                    CostType: [
                        row
                            .columns[633]
                            .into_u8()
                            .copied()
                            .expect("Expected column 633 to be a uint8!"),
                        row
                            .columns[873]
                            .into_u8()
                            .copied()
                            .expect("Expected column 873 to be a uint8!"),
                        row
                            .columns[1113]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1113 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1353]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1353 to be a uint8!"),
                        row
                            .columns[1413]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1413 to be a uint8!"),
                        row
                            .columns[1473]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1473 to be a uint8!"),
                        row
                            .columns[1653]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1653 to be a uint8!"),
                        row
                            .columns[1833]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1833 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1953]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1953 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[213]
                            .into_bool()
                            .copied()
                            .expect("Expected column 213 to be a bool!"),
                        row
                            .columns[453]
                            .into_bool()
                            .copied()
                            .expect("Expected column 453 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[94]
                            .into_u32()
                            .copied()
                            .expect("Expected column 94 to be a uint32!"),
                        row
                            .columns[334]
                            .into_u32()
                            .copied()
                            .expect("Expected column 334 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[574]
                            .into_u32()
                            .copied()
                            .expect("Expected column 574 to be a uint32!"),
                        row
                            .columns[814]
                            .into_u32()
                            .copied()
                            .expect("Expected column 814 to be a uint32!"),
                        row
                            .columns[1054]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1054 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[34]
                            .into_i32()
                            .copied()
                            .expect("Expected column 34 to be a int32!"),
                        row
                            .columns[274]
                            .into_i32()
                            .copied()
                            .expect("Expected column 274 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[154]
                            .into_i32()
                            .copied()
                            .expect("Expected column 154 to be a int32!"),
                        row
                            .columns[394]
                            .into_i32()
                            .copied()
                            .expect("Expected column 394 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[514]
                            .into_i32()
                            .copied()
                            .expect("Expected column 514 to be a int32!"),
                        row
                            .columns[754]
                            .into_i32()
                            .copied()
                            .expect("Expected column 754 to be a int32!"),
                        row
                            .columns[994]
                            .into_i32()
                            .copied()
                            .expect("Expected column 994 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1234]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1234 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1294]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1294 to be a int32!"),
                        row
                            .columns[1534]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1534 to be a int32!"),
                        row
                            .columns[1594]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1594 to be a int32!"),
                        row
                            .columns[1714]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1714 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1774]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1774 to be a int32!"),
                    Unknown2: row
                        .columns[1894]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1894 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[694]
                            .into_u16()
                            .copied()
                            .expect("Expected column 694 to be a uint16!"),
                        row
                            .columns[934]
                            .into_u16()
                            .copied()
                            .expect("Expected column 934 to be a uint16!"),
                        row
                            .columns[1174]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1174 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2014]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2014 to be a uint16!"),
                    CostType: [
                        row
                            .columns[634]
                            .into_u8()
                            .copied()
                            .expect("Expected column 634 to be a uint8!"),
                        row
                            .columns[874]
                            .into_u8()
                            .copied()
                            .expect("Expected column 874 to be a uint8!"),
                        row
                            .columns[1114]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1114 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1354]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1354 to be a uint8!"),
                        row
                            .columns[1414]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1414 to be a uint8!"),
                        row
                            .columns[1474]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1474 to be a uint8!"),
                        row
                            .columns[1654]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1654 to be a uint8!"),
                        row
                            .columns[1834]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1834 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1954]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1954 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[214]
                            .into_bool()
                            .copied()
                            .expect("Expected column 214 to be a bool!"),
                        row
                            .columns[454]
                            .into_bool()
                            .copied()
                            .expect("Expected column 454 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[95]
                            .into_u32()
                            .copied()
                            .expect("Expected column 95 to be a uint32!"),
                        row
                            .columns[335]
                            .into_u32()
                            .copied()
                            .expect("Expected column 335 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[575]
                            .into_u32()
                            .copied()
                            .expect("Expected column 575 to be a uint32!"),
                        row
                            .columns[815]
                            .into_u32()
                            .copied()
                            .expect("Expected column 815 to be a uint32!"),
                        row
                            .columns[1055]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1055 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[35]
                            .into_i32()
                            .copied()
                            .expect("Expected column 35 to be a int32!"),
                        row
                            .columns[275]
                            .into_i32()
                            .copied()
                            .expect("Expected column 275 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[155]
                            .into_i32()
                            .copied()
                            .expect("Expected column 155 to be a int32!"),
                        row
                            .columns[395]
                            .into_i32()
                            .copied()
                            .expect("Expected column 395 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[515]
                            .into_i32()
                            .copied()
                            .expect("Expected column 515 to be a int32!"),
                        row
                            .columns[755]
                            .into_i32()
                            .copied()
                            .expect("Expected column 755 to be a int32!"),
                        row
                            .columns[995]
                            .into_i32()
                            .copied()
                            .expect("Expected column 995 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1235]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1235 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1295]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1295 to be a int32!"),
                        row
                            .columns[1535]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1535 to be a int32!"),
                        row
                            .columns[1595]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1595 to be a int32!"),
                        row
                            .columns[1715]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1715 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1775]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1775 to be a int32!"),
                    Unknown2: row
                        .columns[1895]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1895 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[695]
                            .into_u16()
                            .copied()
                            .expect("Expected column 695 to be a uint16!"),
                        row
                            .columns[935]
                            .into_u16()
                            .copied()
                            .expect("Expected column 935 to be a uint16!"),
                        row
                            .columns[1175]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1175 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2015]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2015 to be a uint16!"),
                    CostType: [
                        row
                            .columns[635]
                            .into_u8()
                            .copied()
                            .expect("Expected column 635 to be a uint8!"),
                        row
                            .columns[875]
                            .into_u8()
                            .copied()
                            .expect("Expected column 875 to be a uint8!"),
                        row
                            .columns[1115]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1115 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1355]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1355 to be a uint8!"),
                        row
                            .columns[1415]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1415 to be a uint8!"),
                        row
                            .columns[1475]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1475 to be a uint8!"),
                        row
                            .columns[1655]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1655 to be a uint8!"),
                        row
                            .columns[1835]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1835 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1955]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1955 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[215]
                            .into_bool()
                            .copied()
                            .expect("Expected column 215 to be a bool!"),
                        row
                            .columns[455]
                            .into_bool()
                            .copied()
                            .expect("Expected column 455 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[96]
                            .into_u32()
                            .copied()
                            .expect("Expected column 96 to be a uint32!"),
                        row
                            .columns[336]
                            .into_u32()
                            .copied()
                            .expect("Expected column 336 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[576]
                            .into_u32()
                            .copied()
                            .expect("Expected column 576 to be a uint32!"),
                        row
                            .columns[816]
                            .into_u32()
                            .copied()
                            .expect("Expected column 816 to be a uint32!"),
                        row
                            .columns[1056]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1056 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[36]
                            .into_i32()
                            .copied()
                            .expect("Expected column 36 to be a int32!"),
                        row
                            .columns[276]
                            .into_i32()
                            .copied()
                            .expect("Expected column 276 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[156]
                            .into_i32()
                            .copied()
                            .expect("Expected column 156 to be a int32!"),
                        row
                            .columns[396]
                            .into_i32()
                            .copied()
                            .expect("Expected column 396 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[516]
                            .into_i32()
                            .copied()
                            .expect("Expected column 516 to be a int32!"),
                        row
                            .columns[756]
                            .into_i32()
                            .copied()
                            .expect("Expected column 756 to be a int32!"),
                        row
                            .columns[996]
                            .into_i32()
                            .copied()
                            .expect("Expected column 996 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1236]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1236 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1296]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1296 to be a int32!"),
                        row
                            .columns[1536]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1536 to be a int32!"),
                        row
                            .columns[1596]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1596 to be a int32!"),
                        row
                            .columns[1716]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1716 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1776]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1776 to be a int32!"),
                    Unknown2: row
                        .columns[1896]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1896 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[696]
                            .into_u16()
                            .copied()
                            .expect("Expected column 696 to be a uint16!"),
                        row
                            .columns[936]
                            .into_u16()
                            .copied()
                            .expect("Expected column 936 to be a uint16!"),
                        row
                            .columns[1176]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1176 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2016]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2016 to be a uint16!"),
                    CostType: [
                        row
                            .columns[636]
                            .into_u8()
                            .copied()
                            .expect("Expected column 636 to be a uint8!"),
                        row
                            .columns[876]
                            .into_u8()
                            .copied()
                            .expect("Expected column 876 to be a uint8!"),
                        row
                            .columns[1116]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1116 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1356]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1356 to be a uint8!"),
                        row
                            .columns[1416]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1416 to be a uint8!"),
                        row
                            .columns[1476]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1476 to be a uint8!"),
                        row
                            .columns[1656]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1656 to be a uint8!"),
                        row
                            .columns[1836]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1836 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1956]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1956 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[216]
                            .into_bool()
                            .copied()
                            .expect("Expected column 216 to be a bool!"),
                        row
                            .columns[456]
                            .into_bool()
                            .copied()
                            .expect("Expected column 456 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[97]
                            .into_u32()
                            .copied()
                            .expect("Expected column 97 to be a uint32!"),
                        row
                            .columns[337]
                            .into_u32()
                            .copied()
                            .expect("Expected column 337 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[577]
                            .into_u32()
                            .copied()
                            .expect("Expected column 577 to be a uint32!"),
                        row
                            .columns[817]
                            .into_u32()
                            .copied()
                            .expect("Expected column 817 to be a uint32!"),
                        row
                            .columns[1057]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1057 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[37]
                            .into_i32()
                            .copied()
                            .expect("Expected column 37 to be a int32!"),
                        row
                            .columns[277]
                            .into_i32()
                            .copied()
                            .expect("Expected column 277 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[157]
                            .into_i32()
                            .copied()
                            .expect("Expected column 157 to be a int32!"),
                        row
                            .columns[397]
                            .into_i32()
                            .copied()
                            .expect("Expected column 397 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[517]
                            .into_i32()
                            .copied()
                            .expect("Expected column 517 to be a int32!"),
                        row
                            .columns[757]
                            .into_i32()
                            .copied()
                            .expect("Expected column 757 to be a int32!"),
                        row
                            .columns[997]
                            .into_i32()
                            .copied()
                            .expect("Expected column 997 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1237]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1237 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1297]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1297 to be a int32!"),
                        row
                            .columns[1537]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1537 to be a int32!"),
                        row
                            .columns[1597]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1597 to be a int32!"),
                        row
                            .columns[1717]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1717 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1777]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1777 to be a int32!"),
                    Unknown2: row
                        .columns[1897]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1897 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[697]
                            .into_u16()
                            .copied()
                            .expect("Expected column 697 to be a uint16!"),
                        row
                            .columns[937]
                            .into_u16()
                            .copied()
                            .expect("Expected column 937 to be a uint16!"),
                        row
                            .columns[1177]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1177 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2017]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2017 to be a uint16!"),
                    CostType: [
                        row
                            .columns[637]
                            .into_u8()
                            .copied()
                            .expect("Expected column 637 to be a uint8!"),
                        row
                            .columns[877]
                            .into_u8()
                            .copied()
                            .expect("Expected column 877 to be a uint8!"),
                        row
                            .columns[1117]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1117 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1357]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1357 to be a uint8!"),
                        row
                            .columns[1417]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1417 to be a uint8!"),
                        row
                            .columns[1477]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1477 to be a uint8!"),
                        row
                            .columns[1657]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1657 to be a uint8!"),
                        row
                            .columns[1837]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1837 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1957]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1957 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[217]
                            .into_bool()
                            .copied()
                            .expect("Expected column 217 to be a bool!"),
                        row
                            .columns[457]
                            .into_bool()
                            .copied()
                            .expect("Expected column 457 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[98]
                            .into_u32()
                            .copied()
                            .expect("Expected column 98 to be a uint32!"),
                        row
                            .columns[338]
                            .into_u32()
                            .copied()
                            .expect("Expected column 338 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[578]
                            .into_u32()
                            .copied()
                            .expect("Expected column 578 to be a uint32!"),
                        row
                            .columns[818]
                            .into_u32()
                            .copied()
                            .expect("Expected column 818 to be a uint32!"),
                        row
                            .columns[1058]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1058 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[38]
                            .into_i32()
                            .copied()
                            .expect("Expected column 38 to be a int32!"),
                        row
                            .columns[278]
                            .into_i32()
                            .copied()
                            .expect("Expected column 278 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[158]
                            .into_i32()
                            .copied()
                            .expect("Expected column 158 to be a int32!"),
                        row
                            .columns[398]
                            .into_i32()
                            .copied()
                            .expect("Expected column 398 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[518]
                            .into_i32()
                            .copied()
                            .expect("Expected column 518 to be a int32!"),
                        row
                            .columns[758]
                            .into_i32()
                            .copied()
                            .expect("Expected column 758 to be a int32!"),
                        row
                            .columns[998]
                            .into_i32()
                            .copied()
                            .expect("Expected column 998 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1238]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1238 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1298]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1298 to be a int32!"),
                        row
                            .columns[1538]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1538 to be a int32!"),
                        row
                            .columns[1598]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1598 to be a int32!"),
                        row
                            .columns[1718]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1718 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1778]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1778 to be a int32!"),
                    Unknown2: row
                        .columns[1898]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1898 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[698]
                            .into_u16()
                            .copied()
                            .expect("Expected column 698 to be a uint16!"),
                        row
                            .columns[938]
                            .into_u16()
                            .copied()
                            .expect("Expected column 938 to be a uint16!"),
                        row
                            .columns[1178]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1178 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2018]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2018 to be a uint16!"),
                    CostType: [
                        row
                            .columns[638]
                            .into_u8()
                            .copied()
                            .expect("Expected column 638 to be a uint8!"),
                        row
                            .columns[878]
                            .into_u8()
                            .copied()
                            .expect("Expected column 878 to be a uint8!"),
                        row
                            .columns[1118]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1118 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1358]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1358 to be a uint8!"),
                        row
                            .columns[1418]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1418 to be a uint8!"),
                        row
                            .columns[1478]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1478 to be a uint8!"),
                        row
                            .columns[1658]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1658 to be a uint8!"),
                        row
                            .columns[1838]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1838 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1958]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1958 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[218]
                            .into_bool()
                            .copied()
                            .expect("Expected column 218 to be a bool!"),
                        row
                            .columns[458]
                            .into_bool()
                            .copied()
                            .expect("Expected column 458 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[99]
                            .into_u32()
                            .copied()
                            .expect("Expected column 99 to be a uint32!"),
                        row
                            .columns[339]
                            .into_u32()
                            .copied()
                            .expect("Expected column 339 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[579]
                            .into_u32()
                            .copied()
                            .expect("Expected column 579 to be a uint32!"),
                        row
                            .columns[819]
                            .into_u32()
                            .copied()
                            .expect("Expected column 819 to be a uint32!"),
                        row
                            .columns[1059]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1059 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[39]
                            .into_i32()
                            .copied()
                            .expect("Expected column 39 to be a int32!"),
                        row
                            .columns[279]
                            .into_i32()
                            .copied()
                            .expect("Expected column 279 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[159]
                            .into_i32()
                            .copied()
                            .expect("Expected column 159 to be a int32!"),
                        row
                            .columns[399]
                            .into_i32()
                            .copied()
                            .expect("Expected column 399 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[519]
                            .into_i32()
                            .copied()
                            .expect("Expected column 519 to be a int32!"),
                        row
                            .columns[759]
                            .into_i32()
                            .copied()
                            .expect("Expected column 759 to be a int32!"),
                        row
                            .columns[999]
                            .into_i32()
                            .copied()
                            .expect("Expected column 999 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1239]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1239 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1299]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1299 to be a int32!"),
                        row
                            .columns[1539]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1539 to be a int32!"),
                        row
                            .columns[1599]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1599 to be a int32!"),
                        row
                            .columns[1719]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1719 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1779]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1779 to be a int32!"),
                    Unknown2: row
                        .columns[1899]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1899 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[699]
                            .into_u16()
                            .copied()
                            .expect("Expected column 699 to be a uint16!"),
                        row
                            .columns[939]
                            .into_u16()
                            .copied()
                            .expect("Expected column 939 to be a uint16!"),
                        row
                            .columns[1179]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1179 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2019]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2019 to be a uint16!"),
                    CostType: [
                        row
                            .columns[639]
                            .into_u8()
                            .copied()
                            .expect("Expected column 639 to be a uint8!"),
                        row
                            .columns[879]
                            .into_u8()
                            .copied()
                            .expect("Expected column 879 to be a uint8!"),
                        row
                            .columns[1119]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1119 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1359]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1359 to be a uint8!"),
                        row
                            .columns[1419]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1419 to be a uint8!"),
                        row
                            .columns[1479]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1479 to be a uint8!"),
                        row
                            .columns[1659]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1659 to be a uint8!"),
                        row
                            .columns[1839]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1839 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1959]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1959 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[219]
                            .into_bool()
                            .copied()
                            .expect("Expected column 219 to be a bool!"),
                        row
                            .columns[459]
                            .into_bool()
                            .copied()
                            .expect("Expected column 459 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[100]
                            .into_u32()
                            .copied()
                            .expect("Expected column 100 to be a uint32!"),
                        row
                            .columns[340]
                            .into_u32()
                            .copied()
                            .expect("Expected column 340 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[580]
                            .into_u32()
                            .copied()
                            .expect("Expected column 580 to be a uint32!"),
                        row
                            .columns[820]
                            .into_u32()
                            .copied()
                            .expect("Expected column 820 to be a uint32!"),
                        row
                            .columns[1060]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1060 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[40]
                            .into_i32()
                            .copied()
                            .expect("Expected column 40 to be a int32!"),
                        row
                            .columns[280]
                            .into_i32()
                            .copied()
                            .expect("Expected column 280 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[160]
                            .into_i32()
                            .copied()
                            .expect("Expected column 160 to be a int32!"),
                        row
                            .columns[400]
                            .into_i32()
                            .copied()
                            .expect("Expected column 400 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[520]
                            .into_i32()
                            .copied()
                            .expect("Expected column 520 to be a int32!"),
                        row
                            .columns[760]
                            .into_i32()
                            .copied()
                            .expect("Expected column 760 to be a int32!"),
                        row
                            .columns[1000]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1000 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1240]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1240 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1300]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1300 to be a int32!"),
                        row
                            .columns[1540]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1540 to be a int32!"),
                        row
                            .columns[1600]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1600 to be a int32!"),
                        row
                            .columns[1720]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1720 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1780]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1780 to be a int32!"),
                    Unknown2: row
                        .columns[1900]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1900 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[700]
                            .into_u16()
                            .copied()
                            .expect("Expected column 700 to be a uint16!"),
                        row
                            .columns[940]
                            .into_u16()
                            .copied()
                            .expect("Expected column 940 to be a uint16!"),
                        row
                            .columns[1180]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1180 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2020]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2020 to be a uint16!"),
                    CostType: [
                        row
                            .columns[640]
                            .into_u8()
                            .copied()
                            .expect("Expected column 640 to be a uint8!"),
                        row
                            .columns[880]
                            .into_u8()
                            .copied()
                            .expect("Expected column 880 to be a uint8!"),
                        row
                            .columns[1120]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1120 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1360]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1360 to be a uint8!"),
                        row
                            .columns[1420]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1420 to be a uint8!"),
                        row
                            .columns[1480]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1480 to be a uint8!"),
                        row
                            .columns[1660]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1660 to be a uint8!"),
                        row
                            .columns[1840]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1840 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1960]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1960 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[220]
                            .into_bool()
                            .copied()
                            .expect("Expected column 220 to be a bool!"),
                        row
                            .columns[460]
                            .into_bool()
                            .copied()
                            .expect("Expected column 460 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[101]
                            .into_u32()
                            .copied()
                            .expect("Expected column 101 to be a uint32!"),
                        row
                            .columns[341]
                            .into_u32()
                            .copied()
                            .expect("Expected column 341 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[581]
                            .into_u32()
                            .copied()
                            .expect("Expected column 581 to be a uint32!"),
                        row
                            .columns[821]
                            .into_u32()
                            .copied()
                            .expect("Expected column 821 to be a uint32!"),
                        row
                            .columns[1061]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1061 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[41]
                            .into_i32()
                            .copied()
                            .expect("Expected column 41 to be a int32!"),
                        row
                            .columns[281]
                            .into_i32()
                            .copied()
                            .expect("Expected column 281 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[161]
                            .into_i32()
                            .copied()
                            .expect("Expected column 161 to be a int32!"),
                        row
                            .columns[401]
                            .into_i32()
                            .copied()
                            .expect("Expected column 401 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[521]
                            .into_i32()
                            .copied()
                            .expect("Expected column 521 to be a int32!"),
                        row
                            .columns[761]
                            .into_i32()
                            .copied()
                            .expect("Expected column 761 to be a int32!"),
                        row
                            .columns[1001]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1001 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1241]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1241 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1301]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1301 to be a int32!"),
                        row
                            .columns[1541]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1541 to be a int32!"),
                        row
                            .columns[1601]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1601 to be a int32!"),
                        row
                            .columns[1721]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1721 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1781]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1781 to be a int32!"),
                    Unknown2: row
                        .columns[1901]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1901 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[701]
                            .into_u16()
                            .copied()
                            .expect("Expected column 701 to be a uint16!"),
                        row
                            .columns[941]
                            .into_u16()
                            .copied()
                            .expect("Expected column 941 to be a uint16!"),
                        row
                            .columns[1181]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1181 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2021]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2021 to be a uint16!"),
                    CostType: [
                        row
                            .columns[641]
                            .into_u8()
                            .copied()
                            .expect("Expected column 641 to be a uint8!"),
                        row
                            .columns[881]
                            .into_u8()
                            .copied()
                            .expect("Expected column 881 to be a uint8!"),
                        row
                            .columns[1121]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1121 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1361]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1361 to be a uint8!"),
                        row
                            .columns[1421]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1421 to be a uint8!"),
                        row
                            .columns[1481]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1481 to be a uint8!"),
                        row
                            .columns[1661]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1661 to be a uint8!"),
                        row
                            .columns[1841]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1841 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1961]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1961 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[221]
                            .into_bool()
                            .copied()
                            .expect("Expected column 221 to be a bool!"),
                        row
                            .columns[461]
                            .into_bool()
                            .copied()
                            .expect("Expected column 461 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[102]
                            .into_u32()
                            .copied()
                            .expect("Expected column 102 to be a uint32!"),
                        row
                            .columns[342]
                            .into_u32()
                            .copied()
                            .expect("Expected column 342 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[582]
                            .into_u32()
                            .copied()
                            .expect("Expected column 582 to be a uint32!"),
                        row
                            .columns[822]
                            .into_u32()
                            .copied()
                            .expect("Expected column 822 to be a uint32!"),
                        row
                            .columns[1062]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1062 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[42]
                            .into_i32()
                            .copied()
                            .expect("Expected column 42 to be a int32!"),
                        row
                            .columns[282]
                            .into_i32()
                            .copied()
                            .expect("Expected column 282 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[162]
                            .into_i32()
                            .copied()
                            .expect("Expected column 162 to be a int32!"),
                        row
                            .columns[402]
                            .into_i32()
                            .copied()
                            .expect("Expected column 402 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[522]
                            .into_i32()
                            .copied()
                            .expect("Expected column 522 to be a int32!"),
                        row
                            .columns[762]
                            .into_i32()
                            .copied()
                            .expect("Expected column 762 to be a int32!"),
                        row
                            .columns[1002]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1002 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1242]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1242 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1302]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1302 to be a int32!"),
                        row
                            .columns[1542]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1542 to be a int32!"),
                        row
                            .columns[1602]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1602 to be a int32!"),
                        row
                            .columns[1722]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1722 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1782]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1782 to be a int32!"),
                    Unknown2: row
                        .columns[1902]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1902 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[702]
                            .into_u16()
                            .copied()
                            .expect("Expected column 702 to be a uint16!"),
                        row
                            .columns[942]
                            .into_u16()
                            .copied()
                            .expect("Expected column 942 to be a uint16!"),
                        row
                            .columns[1182]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1182 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2022]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2022 to be a uint16!"),
                    CostType: [
                        row
                            .columns[642]
                            .into_u8()
                            .copied()
                            .expect("Expected column 642 to be a uint8!"),
                        row
                            .columns[882]
                            .into_u8()
                            .copied()
                            .expect("Expected column 882 to be a uint8!"),
                        row
                            .columns[1122]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1122 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1362]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1362 to be a uint8!"),
                        row
                            .columns[1422]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1422 to be a uint8!"),
                        row
                            .columns[1482]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1482 to be a uint8!"),
                        row
                            .columns[1662]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1662 to be a uint8!"),
                        row
                            .columns[1842]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1842 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1962]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1962 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[222]
                            .into_bool()
                            .copied()
                            .expect("Expected column 222 to be a bool!"),
                        row
                            .columns[462]
                            .into_bool()
                            .copied()
                            .expect("Expected column 462 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[103]
                            .into_u32()
                            .copied()
                            .expect("Expected column 103 to be a uint32!"),
                        row
                            .columns[343]
                            .into_u32()
                            .copied()
                            .expect("Expected column 343 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[583]
                            .into_u32()
                            .copied()
                            .expect("Expected column 583 to be a uint32!"),
                        row
                            .columns[823]
                            .into_u32()
                            .copied()
                            .expect("Expected column 823 to be a uint32!"),
                        row
                            .columns[1063]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1063 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[43]
                            .into_i32()
                            .copied()
                            .expect("Expected column 43 to be a int32!"),
                        row
                            .columns[283]
                            .into_i32()
                            .copied()
                            .expect("Expected column 283 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[163]
                            .into_i32()
                            .copied()
                            .expect("Expected column 163 to be a int32!"),
                        row
                            .columns[403]
                            .into_i32()
                            .copied()
                            .expect("Expected column 403 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[523]
                            .into_i32()
                            .copied()
                            .expect("Expected column 523 to be a int32!"),
                        row
                            .columns[763]
                            .into_i32()
                            .copied()
                            .expect("Expected column 763 to be a int32!"),
                        row
                            .columns[1003]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1003 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1243]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1243 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1303]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1303 to be a int32!"),
                        row
                            .columns[1543]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1543 to be a int32!"),
                        row
                            .columns[1603]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1603 to be a int32!"),
                        row
                            .columns[1723]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1723 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1783]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1783 to be a int32!"),
                    Unknown2: row
                        .columns[1903]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1903 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[703]
                            .into_u16()
                            .copied()
                            .expect("Expected column 703 to be a uint16!"),
                        row
                            .columns[943]
                            .into_u16()
                            .copied()
                            .expect("Expected column 943 to be a uint16!"),
                        row
                            .columns[1183]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1183 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2023]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2023 to be a uint16!"),
                    CostType: [
                        row
                            .columns[643]
                            .into_u8()
                            .copied()
                            .expect("Expected column 643 to be a uint8!"),
                        row
                            .columns[883]
                            .into_u8()
                            .copied()
                            .expect("Expected column 883 to be a uint8!"),
                        row
                            .columns[1123]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1123 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1363]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1363 to be a uint8!"),
                        row
                            .columns[1423]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1423 to be a uint8!"),
                        row
                            .columns[1483]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1483 to be a uint8!"),
                        row
                            .columns[1663]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1663 to be a uint8!"),
                        row
                            .columns[1843]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1843 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1963]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1963 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[223]
                            .into_bool()
                            .copied()
                            .expect("Expected column 223 to be a bool!"),
                        row
                            .columns[463]
                            .into_bool()
                            .copied()
                            .expect("Expected column 463 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[104]
                            .into_u32()
                            .copied()
                            .expect("Expected column 104 to be a uint32!"),
                        row
                            .columns[344]
                            .into_u32()
                            .copied()
                            .expect("Expected column 344 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[584]
                            .into_u32()
                            .copied()
                            .expect("Expected column 584 to be a uint32!"),
                        row
                            .columns[824]
                            .into_u32()
                            .copied()
                            .expect("Expected column 824 to be a uint32!"),
                        row
                            .columns[1064]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1064 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[44]
                            .into_i32()
                            .copied()
                            .expect("Expected column 44 to be a int32!"),
                        row
                            .columns[284]
                            .into_i32()
                            .copied()
                            .expect("Expected column 284 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[164]
                            .into_i32()
                            .copied()
                            .expect("Expected column 164 to be a int32!"),
                        row
                            .columns[404]
                            .into_i32()
                            .copied()
                            .expect("Expected column 404 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[524]
                            .into_i32()
                            .copied()
                            .expect("Expected column 524 to be a int32!"),
                        row
                            .columns[764]
                            .into_i32()
                            .copied()
                            .expect("Expected column 764 to be a int32!"),
                        row
                            .columns[1004]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1004 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1244]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1244 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1304]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1304 to be a int32!"),
                        row
                            .columns[1544]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1544 to be a int32!"),
                        row
                            .columns[1604]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1604 to be a int32!"),
                        row
                            .columns[1724]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1724 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1784]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1784 to be a int32!"),
                    Unknown2: row
                        .columns[1904]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1904 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[704]
                            .into_u16()
                            .copied()
                            .expect("Expected column 704 to be a uint16!"),
                        row
                            .columns[944]
                            .into_u16()
                            .copied()
                            .expect("Expected column 944 to be a uint16!"),
                        row
                            .columns[1184]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1184 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2024]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2024 to be a uint16!"),
                    CostType: [
                        row
                            .columns[644]
                            .into_u8()
                            .copied()
                            .expect("Expected column 644 to be a uint8!"),
                        row
                            .columns[884]
                            .into_u8()
                            .copied()
                            .expect("Expected column 884 to be a uint8!"),
                        row
                            .columns[1124]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1124 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1364]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1364 to be a uint8!"),
                        row
                            .columns[1424]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1424 to be a uint8!"),
                        row
                            .columns[1484]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1484 to be a uint8!"),
                        row
                            .columns[1664]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1664 to be a uint8!"),
                        row
                            .columns[1844]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1844 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1964]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1964 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[224]
                            .into_bool()
                            .copied()
                            .expect("Expected column 224 to be a bool!"),
                        row
                            .columns[464]
                            .into_bool()
                            .copied()
                            .expect("Expected column 464 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[105]
                            .into_u32()
                            .copied()
                            .expect("Expected column 105 to be a uint32!"),
                        row
                            .columns[345]
                            .into_u32()
                            .copied()
                            .expect("Expected column 345 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[585]
                            .into_u32()
                            .copied()
                            .expect("Expected column 585 to be a uint32!"),
                        row
                            .columns[825]
                            .into_u32()
                            .copied()
                            .expect("Expected column 825 to be a uint32!"),
                        row
                            .columns[1065]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1065 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[45]
                            .into_i32()
                            .copied()
                            .expect("Expected column 45 to be a int32!"),
                        row
                            .columns[285]
                            .into_i32()
                            .copied()
                            .expect("Expected column 285 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[165]
                            .into_i32()
                            .copied()
                            .expect("Expected column 165 to be a int32!"),
                        row
                            .columns[405]
                            .into_i32()
                            .copied()
                            .expect("Expected column 405 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[525]
                            .into_i32()
                            .copied()
                            .expect("Expected column 525 to be a int32!"),
                        row
                            .columns[765]
                            .into_i32()
                            .copied()
                            .expect("Expected column 765 to be a int32!"),
                        row
                            .columns[1005]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1005 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1245]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1245 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1305]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1305 to be a int32!"),
                        row
                            .columns[1545]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1545 to be a int32!"),
                        row
                            .columns[1605]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1605 to be a int32!"),
                        row
                            .columns[1725]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1725 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1785]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1785 to be a int32!"),
                    Unknown2: row
                        .columns[1905]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1905 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[705]
                            .into_u16()
                            .copied()
                            .expect("Expected column 705 to be a uint16!"),
                        row
                            .columns[945]
                            .into_u16()
                            .copied()
                            .expect("Expected column 945 to be a uint16!"),
                        row
                            .columns[1185]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1185 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2025]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2025 to be a uint16!"),
                    CostType: [
                        row
                            .columns[645]
                            .into_u8()
                            .copied()
                            .expect("Expected column 645 to be a uint8!"),
                        row
                            .columns[885]
                            .into_u8()
                            .copied()
                            .expect("Expected column 885 to be a uint8!"),
                        row
                            .columns[1125]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1125 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1365]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1365 to be a uint8!"),
                        row
                            .columns[1425]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1425 to be a uint8!"),
                        row
                            .columns[1485]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1485 to be a uint8!"),
                        row
                            .columns[1665]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1665 to be a uint8!"),
                        row
                            .columns[1845]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1845 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1965]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1965 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[225]
                            .into_bool()
                            .copied()
                            .expect("Expected column 225 to be a bool!"),
                        row
                            .columns[465]
                            .into_bool()
                            .copied()
                            .expect("Expected column 465 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[106]
                            .into_u32()
                            .copied()
                            .expect("Expected column 106 to be a uint32!"),
                        row
                            .columns[346]
                            .into_u32()
                            .copied()
                            .expect("Expected column 346 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[586]
                            .into_u32()
                            .copied()
                            .expect("Expected column 586 to be a uint32!"),
                        row
                            .columns[826]
                            .into_u32()
                            .copied()
                            .expect("Expected column 826 to be a uint32!"),
                        row
                            .columns[1066]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1066 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[46]
                            .into_i32()
                            .copied()
                            .expect("Expected column 46 to be a int32!"),
                        row
                            .columns[286]
                            .into_i32()
                            .copied()
                            .expect("Expected column 286 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[166]
                            .into_i32()
                            .copied()
                            .expect("Expected column 166 to be a int32!"),
                        row
                            .columns[406]
                            .into_i32()
                            .copied()
                            .expect("Expected column 406 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[526]
                            .into_i32()
                            .copied()
                            .expect("Expected column 526 to be a int32!"),
                        row
                            .columns[766]
                            .into_i32()
                            .copied()
                            .expect("Expected column 766 to be a int32!"),
                        row
                            .columns[1006]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1006 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1246]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1246 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1306]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1306 to be a int32!"),
                        row
                            .columns[1546]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1546 to be a int32!"),
                        row
                            .columns[1606]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1606 to be a int32!"),
                        row
                            .columns[1726]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1726 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1786]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1786 to be a int32!"),
                    Unknown2: row
                        .columns[1906]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1906 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[706]
                            .into_u16()
                            .copied()
                            .expect("Expected column 706 to be a uint16!"),
                        row
                            .columns[946]
                            .into_u16()
                            .copied()
                            .expect("Expected column 946 to be a uint16!"),
                        row
                            .columns[1186]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1186 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2026]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2026 to be a uint16!"),
                    CostType: [
                        row
                            .columns[646]
                            .into_u8()
                            .copied()
                            .expect("Expected column 646 to be a uint8!"),
                        row
                            .columns[886]
                            .into_u8()
                            .copied()
                            .expect("Expected column 886 to be a uint8!"),
                        row
                            .columns[1126]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1126 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1366]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1366 to be a uint8!"),
                        row
                            .columns[1426]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1426 to be a uint8!"),
                        row
                            .columns[1486]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1486 to be a uint8!"),
                        row
                            .columns[1666]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1666 to be a uint8!"),
                        row
                            .columns[1846]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1846 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1966]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1966 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[226]
                            .into_bool()
                            .copied()
                            .expect("Expected column 226 to be a bool!"),
                        row
                            .columns[466]
                            .into_bool()
                            .copied()
                            .expect("Expected column 466 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[107]
                            .into_u32()
                            .copied()
                            .expect("Expected column 107 to be a uint32!"),
                        row
                            .columns[347]
                            .into_u32()
                            .copied()
                            .expect("Expected column 347 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[587]
                            .into_u32()
                            .copied()
                            .expect("Expected column 587 to be a uint32!"),
                        row
                            .columns[827]
                            .into_u32()
                            .copied()
                            .expect("Expected column 827 to be a uint32!"),
                        row
                            .columns[1067]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1067 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[47]
                            .into_i32()
                            .copied()
                            .expect("Expected column 47 to be a int32!"),
                        row
                            .columns[287]
                            .into_i32()
                            .copied()
                            .expect("Expected column 287 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[167]
                            .into_i32()
                            .copied()
                            .expect("Expected column 167 to be a int32!"),
                        row
                            .columns[407]
                            .into_i32()
                            .copied()
                            .expect("Expected column 407 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[527]
                            .into_i32()
                            .copied()
                            .expect("Expected column 527 to be a int32!"),
                        row
                            .columns[767]
                            .into_i32()
                            .copied()
                            .expect("Expected column 767 to be a int32!"),
                        row
                            .columns[1007]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1007 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1247]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1247 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1307]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1307 to be a int32!"),
                        row
                            .columns[1547]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1547 to be a int32!"),
                        row
                            .columns[1607]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1607 to be a int32!"),
                        row
                            .columns[1727]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1727 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1787]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1787 to be a int32!"),
                    Unknown2: row
                        .columns[1907]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1907 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[707]
                            .into_u16()
                            .copied()
                            .expect("Expected column 707 to be a uint16!"),
                        row
                            .columns[947]
                            .into_u16()
                            .copied()
                            .expect("Expected column 947 to be a uint16!"),
                        row
                            .columns[1187]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1187 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2027]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2027 to be a uint16!"),
                    CostType: [
                        row
                            .columns[647]
                            .into_u8()
                            .copied()
                            .expect("Expected column 647 to be a uint8!"),
                        row
                            .columns[887]
                            .into_u8()
                            .copied()
                            .expect("Expected column 887 to be a uint8!"),
                        row
                            .columns[1127]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1127 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1367]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1367 to be a uint8!"),
                        row
                            .columns[1427]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1427 to be a uint8!"),
                        row
                            .columns[1487]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1487 to be a uint8!"),
                        row
                            .columns[1667]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1667 to be a uint8!"),
                        row
                            .columns[1847]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1847 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1967]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1967 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[227]
                            .into_bool()
                            .copied()
                            .expect("Expected column 227 to be a bool!"),
                        row
                            .columns[467]
                            .into_bool()
                            .copied()
                            .expect("Expected column 467 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[108]
                            .into_u32()
                            .copied()
                            .expect("Expected column 108 to be a uint32!"),
                        row
                            .columns[348]
                            .into_u32()
                            .copied()
                            .expect("Expected column 348 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[588]
                            .into_u32()
                            .copied()
                            .expect("Expected column 588 to be a uint32!"),
                        row
                            .columns[828]
                            .into_u32()
                            .copied()
                            .expect("Expected column 828 to be a uint32!"),
                        row
                            .columns[1068]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1068 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[48]
                            .into_i32()
                            .copied()
                            .expect("Expected column 48 to be a int32!"),
                        row
                            .columns[288]
                            .into_i32()
                            .copied()
                            .expect("Expected column 288 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[168]
                            .into_i32()
                            .copied()
                            .expect("Expected column 168 to be a int32!"),
                        row
                            .columns[408]
                            .into_i32()
                            .copied()
                            .expect("Expected column 408 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[528]
                            .into_i32()
                            .copied()
                            .expect("Expected column 528 to be a int32!"),
                        row
                            .columns[768]
                            .into_i32()
                            .copied()
                            .expect("Expected column 768 to be a int32!"),
                        row
                            .columns[1008]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1008 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1248]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1248 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1308]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1308 to be a int32!"),
                        row
                            .columns[1548]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1548 to be a int32!"),
                        row
                            .columns[1608]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1608 to be a int32!"),
                        row
                            .columns[1728]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1728 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1788]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1788 to be a int32!"),
                    Unknown2: row
                        .columns[1908]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1908 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[708]
                            .into_u16()
                            .copied()
                            .expect("Expected column 708 to be a uint16!"),
                        row
                            .columns[948]
                            .into_u16()
                            .copied()
                            .expect("Expected column 948 to be a uint16!"),
                        row
                            .columns[1188]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1188 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2028]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2028 to be a uint16!"),
                    CostType: [
                        row
                            .columns[648]
                            .into_u8()
                            .copied()
                            .expect("Expected column 648 to be a uint8!"),
                        row
                            .columns[888]
                            .into_u8()
                            .copied()
                            .expect("Expected column 888 to be a uint8!"),
                        row
                            .columns[1128]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1128 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1368]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1368 to be a uint8!"),
                        row
                            .columns[1428]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1428 to be a uint8!"),
                        row
                            .columns[1488]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1488 to be a uint8!"),
                        row
                            .columns[1668]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1668 to be a uint8!"),
                        row
                            .columns[1848]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1848 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1968]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1968 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[228]
                            .into_bool()
                            .copied()
                            .expect("Expected column 228 to be a bool!"),
                        row
                            .columns[468]
                            .into_bool()
                            .copied()
                            .expect("Expected column 468 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[109]
                            .into_u32()
                            .copied()
                            .expect("Expected column 109 to be a uint32!"),
                        row
                            .columns[349]
                            .into_u32()
                            .copied()
                            .expect("Expected column 349 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[589]
                            .into_u32()
                            .copied()
                            .expect("Expected column 589 to be a uint32!"),
                        row
                            .columns[829]
                            .into_u32()
                            .copied()
                            .expect("Expected column 829 to be a uint32!"),
                        row
                            .columns[1069]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1069 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[49]
                            .into_i32()
                            .copied()
                            .expect("Expected column 49 to be a int32!"),
                        row
                            .columns[289]
                            .into_i32()
                            .copied()
                            .expect("Expected column 289 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[169]
                            .into_i32()
                            .copied()
                            .expect("Expected column 169 to be a int32!"),
                        row
                            .columns[409]
                            .into_i32()
                            .copied()
                            .expect("Expected column 409 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[529]
                            .into_i32()
                            .copied()
                            .expect("Expected column 529 to be a int32!"),
                        row
                            .columns[769]
                            .into_i32()
                            .copied()
                            .expect("Expected column 769 to be a int32!"),
                        row
                            .columns[1009]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1009 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1249]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1249 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1309]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1309 to be a int32!"),
                        row
                            .columns[1549]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1549 to be a int32!"),
                        row
                            .columns[1609]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1609 to be a int32!"),
                        row
                            .columns[1729]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1729 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1789]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1789 to be a int32!"),
                    Unknown2: row
                        .columns[1909]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1909 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[709]
                            .into_u16()
                            .copied()
                            .expect("Expected column 709 to be a uint16!"),
                        row
                            .columns[949]
                            .into_u16()
                            .copied()
                            .expect("Expected column 949 to be a uint16!"),
                        row
                            .columns[1189]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1189 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2029]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2029 to be a uint16!"),
                    CostType: [
                        row
                            .columns[649]
                            .into_u8()
                            .copied()
                            .expect("Expected column 649 to be a uint8!"),
                        row
                            .columns[889]
                            .into_u8()
                            .copied()
                            .expect("Expected column 889 to be a uint8!"),
                        row
                            .columns[1129]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1129 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1369]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1369 to be a uint8!"),
                        row
                            .columns[1429]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1429 to be a uint8!"),
                        row
                            .columns[1489]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1489 to be a uint8!"),
                        row
                            .columns[1669]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1669 to be a uint8!"),
                        row
                            .columns[1849]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1849 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1969]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1969 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[229]
                            .into_bool()
                            .copied()
                            .expect("Expected column 229 to be a bool!"),
                        row
                            .columns[469]
                            .into_bool()
                            .copied()
                            .expect("Expected column 469 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[110]
                            .into_u32()
                            .copied()
                            .expect("Expected column 110 to be a uint32!"),
                        row
                            .columns[350]
                            .into_u32()
                            .copied()
                            .expect("Expected column 350 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[590]
                            .into_u32()
                            .copied()
                            .expect("Expected column 590 to be a uint32!"),
                        row
                            .columns[830]
                            .into_u32()
                            .copied()
                            .expect("Expected column 830 to be a uint32!"),
                        row
                            .columns[1070]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1070 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[50]
                            .into_i32()
                            .copied()
                            .expect("Expected column 50 to be a int32!"),
                        row
                            .columns[290]
                            .into_i32()
                            .copied()
                            .expect("Expected column 290 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[170]
                            .into_i32()
                            .copied()
                            .expect("Expected column 170 to be a int32!"),
                        row
                            .columns[410]
                            .into_i32()
                            .copied()
                            .expect("Expected column 410 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[530]
                            .into_i32()
                            .copied()
                            .expect("Expected column 530 to be a int32!"),
                        row
                            .columns[770]
                            .into_i32()
                            .copied()
                            .expect("Expected column 770 to be a int32!"),
                        row
                            .columns[1010]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1010 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1250]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1250 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1310]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1310 to be a int32!"),
                        row
                            .columns[1550]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1550 to be a int32!"),
                        row
                            .columns[1610]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1610 to be a int32!"),
                        row
                            .columns[1730]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1730 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1790]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1790 to be a int32!"),
                    Unknown2: row
                        .columns[1910]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1910 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[710]
                            .into_u16()
                            .copied()
                            .expect("Expected column 710 to be a uint16!"),
                        row
                            .columns[950]
                            .into_u16()
                            .copied()
                            .expect("Expected column 950 to be a uint16!"),
                        row
                            .columns[1190]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1190 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2030]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2030 to be a uint16!"),
                    CostType: [
                        row
                            .columns[650]
                            .into_u8()
                            .copied()
                            .expect("Expected column 650 to be a uint8!"),
                        row
                            .columns[890]
                            .into_u8()
                            .copied()
                            .expect("Expected column 890 to be a uint8!"),
                        row
                            .columns[1130]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1130 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1370]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1370 to be a uint8!"),
                        row
                            .columns[1430]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1430 to be a uint8!"),
                        row
                            .columns[1490]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1490 to be a uint8!"),
                        row
                            .columns[1670]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1670 to be a uint8!"),
                        row
                            .columns[1850]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1850 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1970]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1970 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[230]
                            .into_bool()
                            .copied()
                            .expect("Expected column 230 to be a bool!"),
                        row
                            .columns[470]
                            .into_bool()
                            .copied()
                            .expect("Expected column 470 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[111]
                            .into_u32()
                            .copied()
                            .expect("Expected column 111 to be a uint32!"),
                        row
                            .columns[351]
                            .into_u32()
                            .copied()
                            .expect("Expected column 351 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[591]
                            .into_u32()
                            .copied()
                            .expect("Expected column 591 to be a uint32!"),
                        row
                            .columns[831]
                            .into_u32()
                            .copied()
                            .expect("Expected column 831 to be a uint32!"),
                        row
                            .columns[1071]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1071 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[51]
                            .into_i32()
                            .copied()
                            .expect("Expected column 51 to be a int32!"),
                        row
                            .columns[291]
                            .into_i32()
                            .copied()
                            .expect("Expected column 291 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[171]
                            .into_i32()
                            .copied()
                            .expect("Expected column 171 to be a int32!"),
                        row
                            .columns[411]
                            .into_i32()
                            .copied()
                            .expect("Expected column 411 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[531]
                            .into_i32()
                            .copied()
                            .expect("Expected column 531 to be a int32!"),
                        row
                            .columns[771]
                            .into_i32()
                            .copied()
                            .expect("Expected column 771 to be a int32!"),
                        row
                            .columns[1011]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1011 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1251]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1251 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1311]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1311 to be a int32!"),
                        row
                            .columns[1551]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1551 to be a int32!"),
                        row
                            .columns[1611]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1611 to be a int32!"),
                        row
                            .columns[1731]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1731 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1791]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1791 to be a int32!"),
                    Unknown2: row
                        .columns[1911]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1911 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[711]
                            .into_u16()
                            .copied()
                            .expect("Expected column 711 to be a uint16!"),
                        row
                            .columns[951]
                            .into_u16()
                            .copied()
                            .expect("Expected column 951 to be a uint16!"),
                        row
                            .columns[1191]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1191 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2031]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2031 to be a uint16!"),
                    CostType: [
                        row
                            .columns[651]
                            .into_u8()
                            .copied()
                            .expect("Expected column 651 to be a uint8!"),
                        row
                            .columns[891]
                            .into_u8()
                            .copied()
                            .expect("Expected column 891 to be a uint8!"),
                        row
                            .columns[1131]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1131 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1371]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1371 to be a uint8!"),
                        row
                            .columns[1431]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1431 to be a uint8!"),
                        row
                            .columns[1491]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1491 to be a uint8!"),
                        row
                            .columns[1671]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1671 to be a uint8!"),
                        row
                            .columns[1851]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1851 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1971]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1971 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[231]
                            .into_bool()
                            .copied()
                            .expect("Expected column 231 to be a bool!"),
                        row
                            .columns[471]
                            .into_bool()
                            .copied()
                            .expect("Expected column 471 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[112]
                            .into_u32()
                            .copied()
                            .expect("Expected column 112 to be a uint32!"),
                        row
                            .columns[352]
                            .into_u32()
                            .copied()
                            .expect("Expected column 352 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[592]
                            .into_u32()
                            .copied()
                            .expect("Expected column 592 to be a uint32!"),
                        row
                            .columns[832]
                            .into_u32()
                            .copied()
                            .expect("Expected column 832 to be a uint32!"),
                        row
                            .columns[1072]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1072 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[52]
                            .into_i32()
                            .copied()
                            .expect("Expected column 52 to be a int32!"),
                        row
                            .columns[292]
                            .into_i32()
                            .copied()
                            .expect("Expected column 292 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[172]
                            .into_i32()
                            .copied()
                            .expect("Expected column 172 to be a int32!"),
                        row
                            .columns[412]
                            .into_i32()
                            .copied()
                            .expect("Expected column 412 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[532]
                            .into_i32()
                            .copied()
                            .expect("Expected column 532 to be a int32!"),
                        row
                            .columns[772]
                            .into_i32()
                            .copied()
                            .expect("Expected column 772 to be a int32!"),
                        row
                            .columns[1012]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1012 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1252]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1252 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1312]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1312 to be a int32!"),
                        row
                            .columns[1552]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1552 to be a int32!"),
                        row
                            .columns[1612]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1612 to be a int32!"),
                        row
                            .columns[1732]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1732 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1792]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1792 to be a int32!"),
                    Unknown2: row
                        .columns[1912]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1912 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[712]
                            .into_u16()
                            .copied()
                            .expect("Expected column 712 to be a uint16!"),
                        row
                            .columns[952]
                            .into_u16()
                            .copied()
                            .expect("Expected column 952 to be a uint16!"),
                        row
                            .columns[1192]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1192 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2032]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2032 to be a uint16!"),
                    CostType: [
                        row
                            .columns[652]
                            .into_u8()
                            .copied()
                            .expect("Expected column 652 to be a uint8!"),
                        row
                            .columns[892]
                            .into_u8()
                            .copied()
                            .expect("Expected column 892 to be a uint8!"),
                        row
                            .columns[1132]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1132 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1372]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1372 to be a uint8!"),
                        row
                            .columns[1432]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1432 to be a uint8!"),
                        row
                            .columns[1492]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1492 to be a uint8!"),
                        row
                            .columns[1672]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1672 to be a uint8!"),
                        row
                            .columns[1852]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1852 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1972]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1972 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[232]
                            .into_bool()
                            .copied()
                            .expect("Expected column 232 to be a bool!"),
                        row
                            .columns[472]
                            .into_bool()
                            .copied()
                            .expect("Expected column 472 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[113]
                            .into_u32()
                            .copied()
                            .expect("Expected column 113 to be a uint32!"),
                        row
                            .columns[353]
                            .into_u32()
                            .copied()
                            .expect("Expected column 353 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[593]
                            .into_u32()
                            .copied()
                            .expect("Expected column 593 to be a uint32!"),
                        row
                            .columns[833]
                            .into_u32()
                            .copied()
                            .expect("Expected column 833 to be a uint32!"),
                        row
                            .columns[1073]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1073 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[53]
                            .into_i32()
                            .copied()
                            .expect("Expected column 53 to be a int32!"),
                        row
                            .columns[293]
                            .into_i32()
                            .copied()
                            .expect("Expected column 293 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[173]
                            .into_i32()
                            .copied()
                            .expect("Expected column 173 to be a int32!"),
                        row
                            .columns[413]
                            .into_i32()
                            .copied()
                            .expect("Expected column 413 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[533]
                            .into_i32()
                            .copied()
                            .expect("Expected column 533 to be a int32!"),
                        row
                            .columns[773]
                            .into_i32()
                            .copied()
                            .expect("Expected column 773 to be a int32!"),
                        row
                            .columns[1013]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1013 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1253]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1253 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1313]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1313 to be a int32!"),
                        row
                            .columns[1553]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1553 to be a int32!"),
                        row
                            .columns[1613]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1613 to be a int32!"),
                        row
                            .columns[1733]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1733 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1793]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1793 to be a int32!"),
                    Unknown2: row
                        .columns[1913]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1913 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[713]
                            .into_u16()
                            .copied()
                            .expect("Expected column 713 to be a uint16!"),
                        row
                            .columns[953]
                            .into_u16()
                            .copied()
                            .expect("Expected column 953 to be a uint16!"),
                        row
                            .columns[1193]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1193 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2033]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2033 to be a uint16!"),
                    CostType: [
                        row
                            .columns[653]
                            .into_u8()
                            .copied()
                            .expect("Expected column 653 to be a uint8!"),
                        row
                            .columns[893]
                            .into_u8()
                            .copied()
                            .expect("Expected column 893 to be a uint8!"),
                        row
                            .columns[1133]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1133 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1373]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1373 to be a uint8!"),
                        row
                            .columns[1433]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1433 to be a uint8!"),
                        row
                            .columns[1493]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1493 to be a uint8!"),
                        row
                            .columns[1673]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1673 to be a uint8!"),
                        row
                            .columns[1853]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1853 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1973]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1973 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[233]
                            .into_bool()
                            .copied()
                            .expect("Expected column 233 to be a bool!"),
                        row
                            .columns[473]
                            .into_bool()
                            .copied()
                            .expect("Expected column 473 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[114]
                            .into_u32()
                            .copied()
                            .expect("Expected column 114 to be a uint32!"),
                        row
                            .columns[354]
                            .into_u32()
                            .copied()
                            .expect("Expected column 354 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[594]
                            .into_u32()
                            .copied()
                            .expect("Expected column 594 to be a uint32!"),
                        row
                            .columns[834]
                            .into_u32()
                            .copied()
                            .expect("Expected column 834 to be a uint32!"),
                        row
                            .columns[1074]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1074 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[54]
                            .into_i32()
                            .copied()
                            .expect("Expected column 54 to be a int32!"),
                        row
                            .columns[294]
                            .into_i32()
                            .copied()
                            .expect("Expected column 294 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[174]
                            .into_i32()
                            .copied()
                            .expect("Expected column 174 to be a int32!"),
                        row
                            .columns[414]
                            .into_i32()
                            .copied()
                            .expect("Expected column 414 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[534]
                            .into_i32()
                            .copied()
                            .expect("Expected column 534 to be a int32!"),
                        row
                            .columns[774]
                            .into_i32()
                            .copied()
                            .expect("Expected column 774 to be a int32!"),
                        row
                            .columns[1014]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1014 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1254]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1254 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1314]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1314 to be a int32!"),
                        row
                            .columns[1554]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1554 to be a int32!"),
                        row
                            .columns[1614]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1614 to be a int32!"),
                        row
                            .columns[1734]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1734 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1794]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1794 to be a int32!"),
                    Unknown2: row
                        .columns[1914]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1914 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[714]
                            .into_u16()
                            .copied()
                            .expect("Expected column 714 to be a uint16!"),
                        row
                            .columns[954]
                            .into_u16()
                            .copied()
                            .expect("Expected column 954 to be a uint16!"),
                        row
                            .columns[1194]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1194 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2034]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2034 to be a uint16!"),
                    CostType: [
                        row
                            .columns[654]
                            .into_u8()
                            .copied()
                            .expect("Expected column 654 to be a uint8!"),
                        row
                            .columns[894]
                            .into_u8()
                            .copied()
                            .expect("Expected column 894 to be a uint8!"),
                        row
                            .columns[1134]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1134 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1374]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1374 to be a uint8!"),
                        row
                            .columns[1434]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1434 to be a uint8!"),
                        row
                            .columns[1494]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1494 to be a uint8!"),
                        row
                            .columns[1674]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1674 to be a uint8!"),
                        row
                            .columns[1854]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1854 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1974]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1974 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[234]
                            .into_bool()
                            .copied()
                            .expect("Expected column 234 to be a bool!"),
                        row
                            .columns[474]
                            .into_bool()
                            .copied()
                            .expect("Expected column 474 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[115]
                            .into_u32()
                            .copied()
                            .expect("Expected column 115 to be a uint32!"),
                        row
                            .columns[355]
                            .into_u32()
                            .copied()
                            .expect("Expected column 355 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[595]
                            .into_u32()
                            .copied()
                            .expect("Expected column 595 to be a uint32!"),
                        row
                            .columns[835]
                            .into_u32()
                            .copied()
                            .expect("Expected column 835 to be a uint32!"),
                        row
                            .columns[1075]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1075 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[55]
                            .into_i32()
                            .copied()
                            .expect("Expected column 55 to be a int32!"),
                        row
                            .columns[295]
                            .into_i32()
                            .copied()
                            .expect("Expected column 295 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[175]
                            .into_i32()
                            .copied()
                            .expect("Expected column 175 to be a int32!"),
                        row
                            .columns[415]
                            .into_i32()
                            .copied()
                            .expect("Expected column 415 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[535]
                            .into_i32()
                            .copied()
                            .expect("Expected column 535 to be a int32!"),
                        row
                            .columns[775]
                            .into_i32()
                            .copied()
                            .expect("Expected column 775 to be a int32!"),
                        row
                            .columns[1015]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1015 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1255]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1255 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1315]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1315 to be a int32!"),
                        row
                            .columns[1555]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1555 to be a int32!"),
                        row
                            .columns[1615]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1615 to be a int32!"),
                        row
                            .columns[1735]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1735 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1795]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1795 to be a int32!"),
                    Unknown2: row
                        .columns[1915]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1915 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[715]
                            .into_u16()
                            .copied()
                            .expect("Expected column 715 to be a uint16!"),
                        row
                            .columns[955]
                            .into_u16()
                            .copied()
                            .expect("Expected column 955 to be a uint16!"),
                        row
                            .columns[1195]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1195 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2035]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2035 to be a uint16!"),
                    CostType: [
                        row
                            .columns[655]
                            .into_u8()
                            .copied()
                            .expect("Expected column 655 to be a uint8!"),
                        row
                            .columns[895]
                            .into_u8()
                            .copied()
                            .expect("Expected column 895 to be a uint8!"),
                        row
                            .columns[1135]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1135 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1375]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1375 to be a uint8!"),
                        row
                            .columns[1435]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1435 to be a uint8!"),
                        row
                            .columns[1495]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1495 to be a uint8!"),
                        row
                            .columns[1675]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1675 to be a uint8!"),
                        row
                            .columns[1855]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1855 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1975]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1975 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[235]
                            .into_bool()
                            .copied()
                            .expect("Expected column 235 to be a bool!"),
                        row
                            .columns[475]
                            .into_bool()
                            .copied()
                            .expect("Expected column 475 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[116]
                            .into_u32()
                            .copied()
                            .expect("Expected column 116 to be a uint32!"),
                        row
                            .columns[356]
                            .into_u32()
                            .copied()
                            .expect("Expected column 356 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[596]
                            .into_u32()
                            .copied()
                            .expect("Expected column 596 to be a uint32!"),
                        row
                            .columns[836]
                            .into_u32()
                            .copied()
                            .expect("Expected column 836 to be a uint32!"),
                        row
                            .columns[1076]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1076 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[56]
                            .into_i32()
                            .copied()
                            .expect("Expected column 56 to be a int32!"),
                        row
                            .columns[296]
                            .into_i32()
                            .copied()
                            .expect("Expected column 296 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[176]
                            .into_i32()
                            .copied()
                            .expect("Expected column 176 to be a int32!"),
                        row
                            .columns[416]
                            .into_i32()
                            .copied()
                            .expect("Expected column 416 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[536]
                            .into_i32()
                            .copied()
                            .expect("Expected column 536 to be a int32!"),
                        row
                            .columns[776]
                            .into_i32()
                            .copied()
                            .expect("Expected column 776 to be a int32!"),
                        row
                            .columns[1016]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1016 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1256]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1256 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1316]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1316 to be a int32!"),
                        row
                            .columns[1556]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1556 to be a int32!"),
                        row
                            .columns[1616]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1616 to be a int32!"),
                        row
                            .columns[1736]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1736 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1796]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1796 to be a int32!"),
                    Unknown2: row
                        .columns[1916]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1916 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[716]
                            .into_u16()
                            .copied()
                            .expect("Expected column 716 to be a uint16!"),
                        row
                            .columns[956]
                            .into_u16()
                            .copied()
                            .expect("Expected column 956 to be a uint16!"),
                        row
                            .columns[1196]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1196 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2036]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2036 to be a uint16!"),
                    CostType: [
                        row
                            .columns[656]
                            .into_u8()
                            .copied()
                            .expect("Expected column 656 to be a uint8!"),
                        row
                            .columns[896]
                            .into_u8()
                            .copied()
                            .expect("Expected column 896 to be a uint8!"),
                        row
                            .columns[1136]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1136 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1376]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1376 to be a uint8!"),
                        row
                            .columns[1436]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1436 to be a uint8!"),
                        row
                            .columns[1496]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1496 to be a uint8!"),
                        row
                            .columns[1676]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1676 to be a uint8!"),
                        row
                            .columns[1856]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1856 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1976]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1976 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[236]
                            .into_bool()
                            .copied()
                            .expect("Expected column 236 to be a bool!"),
                        row
                            .columns[476]
                            .into_bool()
                            .copied()
                            .expect("Expected column 476 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[117]
                            .into_u32()
                            .copied()
                            .expect("Expected column 117 to be a uint32!"),
                        row
                            .columns[357]
                            .into_u32()
                            .copied()
                            .expect("Expected column 357 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[597]
                            .into_u32()
                            .copied()
                            .expect("Expected column 597 to be a uint32!"),
                        row
                            .columns[837]
                            .into_u32()
                            .copied()
                            .expect("Expected column 837 to be a uint32!"),
                        row
                            .columns[1077]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1077 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[57]
                            .into_i32()
                            .copied()
                            .expect("Expected column 57 to be a int32!"),
                        row
                            .columns[297]
                            .into_i32()
                            .copied()
                            .expect("Expected column 297 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[177]
                            .into_i32()
                            .copied()
                            .expect("Expected column 177 to be a int32!"),
                        row
                            .columns[417]
                            .into_i32()
                            .copied()
                            .expect("Expected column 417 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[537]
                            .into_i32()
                            .copied()
                            .expect("Expected column 537 to be a int32!"),
                        row
                            .columns[777]
                            .into_i32()
                            .copied()
                            .expect("Expected column 777 to be a int32!"),
                        row
                            .columns[1017]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1017 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1257]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1257 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1317]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1317 to be a int32!"),
                        row
                            .columns[1557]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1557 to be a int32!"),
                        row
                            .columns[1617]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1617 to be a int32!"),
                        row
                            .columns[1737]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1737 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1797]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1797 to be a int32!"),
                    Unknown2: row
                        .columns[1917]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1917 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[717]
                            .into_u16()
                            .copied()
                            .expect("Expected column 717 to be a uint16!"),
                        row
                            .columns[957]
                            .into_u16()
                            .copied()
                            .expect("Expected column 957 to be a uint16!"),
                        row
                            .columns[1197]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1197 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2037]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2037 to be a uint16!"),
                    CostType: [
                        row
                            .columns[657]
                            .into_u8()
                            .copied()
                            .expect("Expected column 657 to be a uint8!"),
                        row
                            .columns[897]
                            .into_u8()
                            .copied()
                            .expect("Expected column 897 to be a uint8!"),
                        row
                            .columns[1137]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1137 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1377]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1377 to be a uint8!"),
                        row
                            .columns[1437]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1437 to be a uint8!"),
                        row
                            .columns[1497]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1497 to be a uint8!"),
                        row
                            .columns[1677]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1677 to be a uint8!"),
                        row
                            .columns[1857]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1857 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1977]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1977 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[237]
                            .into_bool()
                            .copied()
                            .expect("Expected column 237 to be a bool!"),
                        row
                            .columns[477]
                            .into_bool()
                            .copied()
                            .expect("Expected column 477 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[118]
                            .into_u32()
                            .copied()
                            .expect("Expected column 118 to be a uint32!"),
                        row
                            .columns[358]
                            .into_u32()
                            .copied()
                            .expect("Expected column 358 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[598]
                            .into_u32()
                            .copied()
                            .expect("Expected column 598 to be a uint32!"),
                        row
                            .columns[838]
                            .into_u32()
                            .copied()
                            .expect("Expected column 838 to be a uint32!"),
                        row
                            .columns[1078]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1078 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[58]
                            .into_i32()
                            .copied()
                            .expect("Expected column 58 to be a int32!"),
                        row
                            .columns[298]
                            .into_i32()
                            .copied()
                            .expect("Expected column 298 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[178]
                            .into_i32()
                            .copied()
                            .expect("Expected column 178 to be a int32!"),
                        row
                            .columns[418]
                            .into_i32()
                            .copied()
                            .expect("Expected column 418 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[538]
                            .into_i32()
                            .copied()
                            .expect("Expected column 538 to be a int32!"),
                        row
                            .columns[778]
                            .into_i32()
                            .copied()
                            .expect("Expected column 778 to be a int32!"),
                        row
                            .columns[1018]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1018 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1258]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1258 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1318]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1318 to be a int32!"),
                        row
                            .columns[1558]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1558 to be a int32!"),
                        row
                            .columns[1618]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1618 to be a int32!"),
                        row
                            .columns[1738]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1738 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1798]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1798 to be a int32!"),
                    Unknown2: row
                        .columns[1918]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1918 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[718]
                            .into_u16()
                            .copied()
                            .expect("Expected column 718 to be a uint16!"),
                        row
                            .columns[958]
                            .into_u16()
                            .copied()
                            .expect("Expected column 958 to be a uint16!"),
                        row
                            .columns[1198]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1198 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2038]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2038 to be a uint16!"),
                    CostType: [
                        row
                            .columns[658]
                            .into_u8()
                            .copied()
                            .expect("Expected column 658 to be a uint8!"),
                        row
                            .columns[898]
                            .into_u8()
                            .copied()
                            .expect("Expected column 898 to be a uint8!"),
                        row
                            .columns[1138]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1138 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1378]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1378 to be a uint8!"),
                        row
                            .columns[1438]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1438 to be a uint8!"),
                        row
                            .columns[1498]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1498 to be a uint8!"),
                        row
                            .columns[1678]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1678 to be a uint8!"),
                        row
                            .columns[1858]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1858 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1978]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1978 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[238]
                            .into_bool()
                            .copied()
                            .expect("Expected column 238 to be a bool!"),
                        row
                            .columns[478]
                            .into_bool()
                            .copied()
                            .expect("Expected column 478 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[119]
                            .into_u32()
                            .copied()
                            .expect("Expected column 119 to be a uint32!"),
                        row
                            .columns[359]
                            .into_u32()
                            .copied()
                            .expect("Expected column 359 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[599]
                            .into_u32()
                            .copied()
                            .expect("Expected column 599 to be a uint32!"),
                        row
                            .columns[839]
                            .into_u32()
                            .copied()
                            .expect("Expected column 839 to be a uint32!"),
                        row
                            .columns[1079]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1079 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[59]
                            .into_i32()
                            .copied()
                            .expect("Expected column 59 to be a int32!"),
                        row
                            .columns[299]
                            .into_i32()
                            .copied()
                            .expect("Expected column 299 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[179]
                            .into_i32()
                            .copied()
                            .expect("Expected column 179 to be a int32!"),
                        row
                            .columns[419]
                            .into_i32()
                            .copied()
                            .expect("Expected column 419 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[539]
                            .into_i32()
                            .copied()
                            .expect("Expected column 539 to be a int32!"),
                        row
                            .columns[779]
                            .into_i32()
                            .copied()
                            .expect("Expected column 779 to be a int32!"),
                        row
                            .columns[1019]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1019 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1259]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1259 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1319]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1319 to be a int32!"),
                        row
                            .columns[1559]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1559 to be a int32!"),
                        row
                            .columns[1619]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1619 to be a int32!"),
                        row
                            .columns[1739]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1739 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1799]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1799 to be a int32!"),
                    Unknown2: row
                        .columns[1919]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1919 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[719]
                            .into_u16()
                            .copied()
                            .expect("Expected column 719 to be a uint16!"),
                        row
                            .columns[959]
                            .into_u16()
                            .copied()
                            .expect("Expected column 959 to be a uint16!"),
                        row
                            .columns[1199]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1199 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2039]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2039 to be a uint16!"),
                    CostType: [
                        row
                            .columns[659]
                            .into_u8()
                            .copied()
                            .expect("Expected column 659 to be a uint8!"),
                        row
                            .columns[899]
                            .into_u8()
                            .copied()
                            .expect("Expected column 899 to be a uint8!"),
                        row
                            .columns[1139]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1139 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1379]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1379 to be a uint8!"),
                        row
                            .columns[1439]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1439 to be a uint8!"),
                        row
                            .columns[1499]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1499 to be a uint8!"),
                        row
                            .columns[1679]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1679 to be a uint8!"),
                        row
                            .columns[1859]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1859 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1979]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1979 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[239]
                            .into_bool()
                            .copied()
                            .expect("Expected column 239 to be a bool!"),
                        row
                            .columns[479]
                            .into_bool()
                            .copied()
                            .expect("Expected column 479 to be a bool!"),
                    ],
                },
                ItemElement {
                    ReceiveCount: [
                        row
                            .columns[120]
                            .into_u32()
                            .copied()
                            .expect("Expected column 120 to be a uint32!"),
                        row
                            .columns[360]
                            .into_u32()
                            .copied()
                            .expect("Expected column 360 to be a uint32!"),
                    ],
                    CurrencyCost: [
                        row
                            .columns[600]
                            .into_u32()
                            .copied()
                            .expect("Expected column 600 to be a uint32!"),
                        row
                            .columns[840]
                            .into_u32()
                            .copied()
                            .expect("Expected column 840 to be a uint32!"),
                        row
                            .columns[1080]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1080 to be a uint32!"),
                    ],
                    Item: [
                        row
                            .columns[60]
                            .into_i32()
                            .copied()
                            .expect("Expected column 60 to be a int32!"),
                        row
                            .columns[300]
                            .into_i32()
                            .copied()
                            .expect("Expected column 300 to be a int32!"),
                    ],
                    Category: [
                        row
                            .columns[180]
                            .into_i32()
                            .copied()
                            .expect("Expected column 180 to be a int32!"),
                        row
                            .columns[420]
                            .into_i32()
                            .copied()
                            .expect("Expected column 420 to be a int32!"),
                    ],
                    ItemCost: [
                        row
                            .columns[540]
                            .into_i32()
                            .copied()
                            .expect("Expected column 540 to be a int32!"),
                        row
                            .columns[780]
                            .into_i32()
                            .copied()
                            .expect("Expected column 780 to be a int32!"),
                        row
                            .columns[1020]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1020 to be a int32!"),
                    ],
                    Quest: row
                        .columns[1260]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1260 to be a int32!"),
                    Unknown0: [
                        row
                            .columns[1320]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1320 to be a int32!"),
                        row
                            .columns[1560]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1560 to be a int32!"),
                        row
                            .columns[1620]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1620 to be a int32!"),
                        row
                            .columns[1740]
                            .into_i32()
                            .copied()
                            .expect("Expected column 1740 to be a int32!"),
                    ],
                    AchievementUnlock: row
                        .columns[1800]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1800 to be a int32!"),
                    Unknown2: row
                        .columns[1920]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1920 to be a int32!"),
                    CollectabilityCost: [
                        row
                            .columns[720]
                            .into_u16()
                            .copied()
                            .expect("Expected column 720 to be a uint16!"),
                        row
                            .columns[960]
                            .into_u16()
                            .copied()
                            .expect("Expected column 960 to be a uint16!"),
                        row
                            .columns[1200]
                            .into_u16()
                            .copied()
                            .expect("Expected column 1200 to be a uint16!"),
                    ],
                    PatchNumber: row
                        .columns[2040]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2040 to be a uint16!"),
                    CostType: [
                        row
                            .columns[660]
                            .into_u8()
                            .copied()
                            .expect("Expected column 660 to be a uint8!"),
                        row
                            .columns[900]
                            .into_u8()
                            .copied()
                            .expect("Expected column 900 to be a uint8!"),
                        row
                            .columns[1140]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1140 to be a uint8!"),
                    ],
                    Unknown1: [
                        row
                            .columns[1380]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1380 to be a uint8!"),
                        row
                            .columns[1440]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1440 to be a uint8!"),
                        row
                            .columns[1500]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1500 to be a uint8!"),
                        row
                            .columns[1680]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1680 to be a uint8!"),
                        row
                            .columns[1860]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1860 to be a uint8!"),
                    ],
                    Order: row
                        .columns[1980]
                        .into_u8()
                        .copied()
                        .expect("Expected column 1980 to be a uint8!"),
                    ReceiveHq: [
                        row
                            .columns[240]
                            .into_bool()
                            .copied()
                            .expect("Expected column 240 to be a bool!"),
                        row
                            .columns[480]
                            .into_bool()
                            .copied()
                            .expect("Expected column 480 to be a bool!"),
                    ],
                },
            ],
            Quest: row
                .columns[2042]
                .into_u32()
                .copied()
                .expect("Expected column 2042 to be a uint32!"),
            CustomTalk: row
                .columns[2045]
                .into_u32()
                .copied()
                .expect("Expected column 2045 to be a uint32!"),
            RequiredContentFinderCondition: row
                .columns[2049]
                .into_u32()
                .copied()
                .expect("Expected column 2049 to be a uint32!"),
            CompleteText: row
                .columns[2043]
                .into_i32()
                .copied()
                .expect("Expected column 2043 to be a int32!"),
            NotCompleteText: row
                .columns[2044]
                .into_i32()
                .copied()
                .expect("Expected column 2044 to be a int32!"),
            RequiredFestival: row
                .columns[2047]
                .into_u16()
                .copied()
                .expect("Expected column 2047 to be a uint16!"),
            RequiredFestivalPhase: row
                .columns[2048]
                .into_u16()
                .copied()
                .expect("Expected column 2048 to be a uint16!"),
            UseCurrencyType: row
                .columns[2041]
                .into_u8()
                .copied()
                .expect("Expected column 2041 to be a uint8!"),
            Unknown3: row
                .columns[2046]
                .into_bool()
                .copied()
                .expect("Expected column 2046 to be a bool!"),
            RequiredContentFinderConditionComplete: row
                .columns[2050]
                .into_bool()
                .copied()
                .expect("Expected column 2050 to be a bool!"),
            Unknown4: row
                .columns[2051]
                .into_bool()
                .copied()
                .expect("Expected column 2051 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a SpecialShopSheet {
    type Item = (u32, Vec<(u16, SpecialShopRow)>);
    type IntoIter = StructuredSheetIterator<'a, SpecialShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SpecialShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpecialShopRow {
    ///""
    pub Name: String,
    ///""
    pub Item: [ItemElement; 60],
    ///""
    pub Quest: u32,
    ///""
    pub CustomTalk: u32,
    ///""
    pub RequiredContentFinderCondition: u32,
    ///""
    pub CompleteText: i32,
    ///""
    pub NotCompleteText: i32,
    ///""
    pub RequiredFestival: u16,
    ///""
    pub RequiredFestivalPhase: u16,
    ///""
    pub UseCurrencyType: u8,
    ///""
    pub Unknown3: bool,
    ///"If this is true, then the CFC needs to be completed; If this is false, then the CFC just needs to be unlocked"
    pub RequiredContentFinderConditionComplete: bool,
    ///""
    pub Unknown4: bool,
}
