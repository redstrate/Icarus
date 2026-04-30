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
pub struct CharaMakeStructElement {
    pub Menu: u32,
    pub SubMenuMask: u32,
    pub Customize: u32,
    pub SubMenuParam: [u32; 100],
    pub Unknown0: [u32; 6],
    pub InitVal: u8,
    pub SubMenuType: u8,
    pub SubMenuNum: u8,
    pub LookAt: u8,
    pub SubMenuGraphic: [u8; 10],
}
#[derive(Clone, Debug, PartialEq)]
pub struct FacialFeatureOptionElement {
    pub Option1: i32,
    pub Option2: i32,
    pub Option3: i32,
    pub Option4: i32,
    pub Option5: i32,
    pub Option6: i32,
    pub Option7: i32,
}
#[derive(Debug, Clone)]
pub struct HairMakeTypeSheet {
    sheet: Sheet,
}
impl HairMakeTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HairMakeType")?;
        let sheet = resolver.read_excel_sheet(&exh, "HairMakeType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HairMakeTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HairMakeTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for HairMakeTypeSheet {
    type Row = HairMakeTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            CharaMakeStruct: [
                CharaMakeStructElement {
                    Menu: row
                        .columns[3]
                        .into_u32()
                        .copied()
                        .expect("Expected column 3 to be a uint32!"),
                    SubMenuMask: row
                        .columns[48]
                        .into_u32()
                        .copied()
                        .expect("Expected column 48 to be a uint32!"),
                    Customize: row
                        .columns[57]
                        .into_u32()
                        .copied()
                        .expect("Expected column 57 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[66]
                            .into_u32()
                            .copied()
                            .expect("Expected column 66 to be a uint32!"),
                        row
                            .columns[75]
                            .into_u32()
                            .copied()
                            .expect("Expected column 75 to be a uint32!"),
                        row
                            .columns[84]
                            .into_u32()
                            .copied()
                            .expect("Expected column 84 to be a uint32!"),
                        row
                            .columns[93]
                            .into_u32()
                            .copied()
                            .expect("Expected column 93 to be a uint32!"),
                        row
                            .columns[102]
                            .into_u32()
                            .copied()
                            .expect("Expected column 102 to be a uint32!"),
                        row
                            .columns[111]
                            .into_u32()
                            .copied()
                            .expect("Expected column 111 to be a uint32!"),
                        row
                            .columns[120]
                            .into_u32()
                            .copied()
                            .expect("Expected column 120 to be a uint32!"),
                        row
                            .columns[129]
                            .into_u32()
                            .copied()
                            .expect("Expected column 129 to be a uint32!"),
                        row
                            .columns[138]
                            .into_u32()
                            .copied()
                            .expect("Expected column 138 to be a uint32!"),
                        row
                            .columns[147]
                            .into_u32()
                            .copied()
                            .expect("Expected column 147 to be a uint32!"),
                        row
                            .columns[156]
                            .into_u32()
                            .copied()
                            .expect("Expected column 156 to be a uint32!"),
                        row
                            .columns[165]
                            .into_u32()
                            .copied()
                            .expect("Expected column 165 to be a uint32!"),
                        row
                            .columns[174]
                            .into_u32()
                            .copied()
                            .expect("Expected column 174 to be a uint32!"),
                        row
                            .columns[183]
                            .into_u32()
                            .copied()
                            .expect("Expected column 183 to be a uint32!"),
                        row
                            .columns[192]
                            .into_u32()
                            .copied()
                            .expect("Expected column 192 to be a uint32!"),
                        row
                            .columns[201]
                            .into_u32()
                            .copied()
                            .expect("Expected column 201 to be a uint32!"),
                        row
                            .columns[210]
                            .into_u32()
                            .copied()
                            .expect("Expected column 210 to be a uint32!"),
                        row
                            .columns[219]
                            .into_u32()
                            .copied()
                            .expect("Expected column 219 to be a uint32!"),
                        row
                            .columns[228]
                            .into_u32()
                            .copied()
                            .expect("Expected column 228 to be a uint32!"),
                        row
                            .columns[237]
                            .into_u32()
                            .copied()
                            .expect("Expected column 237 to be a uint32!"),
                        row
                            .columns[246]
                            .into_u32()
                            .copied()
                            .expect("Expected column 246 to be a uint32!"),
                        row
                            .columns[255]
                            .into_u32()
                            .copied()
                            .expect("Expected column 255 to be a uint32!"),
                        row
                            .columns[264]
                            .into_u32()
                            .copied()
                            .expect("Expected column 264 to be a uint32!"),
                        row
                            .columns[273]
                            .into_u32()
                            .copied()
                            .expect("Expected column 273 to be a uint32!"),
                        row
                            .columns[282]
                            .into_u32()
                            .copied()
                            .expect("Expected column 282 to be a uint32!"),
                        row
                            .columns[291]
                            .into_u32()
                            .copied()
                            .expect("Expected column 291 to be a uint32!"),
                        row
                            .columns[300]
                            .into_u32()
                            .copied()
                            .expect("Expected column 300 to be a uint32!"),
                        row
                            .columns[309]
                            .into_u32()
                            .copied()
                            .expect("Expected column 309 to be a uint32!"),
                        row
                            .columns[318]
                            .into_u32()
                            .copied()
                            .expect("Expected column 318 to be a uint32!"),
                        row
                            .columns[327]
                            .into_u32()
                            .copied()
                            .expect("Expected column 327 to be a uint32!"),
                        row
                            .columns[336]
                            .into_u32()
                            .copied()
                            .expect("Expected column 336 to be a uint32!"),
                        row
                            .columns[345]
                            .into_u32()
                            .copied()
                            .expect("Expected column 345 to be a uint32!"),
                        row
                            .columns[354]
                            .into_u32()
                            .copied()
                            .expect("Expected column 354 to be a uint32!"),
                        row
                            .columns[363]
                            .into_u32()
                            .copied()
                            .expect("Expected column 363 to be a uint32!"),
                        row
                            .columns[372]
                            .into_u32()
                            .copied()
                            .expect("Expected column 372 to be a uint32!"),
                        row
                            .columns[381]
                            .into_u32()
                            .copied()
                            .expect("Expected column 381 to be a uint32!"),
                        row
                            .columns[390]
                            .into_u32()
                            .copied()
                            .expect("Expected column 390 to be a uint32!"),
                        row
                            .columns[399]
                            .into_u32()
                            .copied()
                            .expect("Expected column 399 to be a uint32!"),
                        row
                            .columns[408]
                            .into_u32()
                            .copied()
                            .expect("Expected column 408 to be a uint32!"),
                        row
                            .columns[417]
                            .into_u32()
                            .copied()
                            .expect("Expected column 417 to be a uint32!"),
                        row
                            .columns[426]
                            .into_u32()
                            .copied()
                            .expect("Expected column 426 to be a uint32!"),
                        row
                            .columns[435]
                            .into_u32()
                            .copied()
                            .expect("Expected column 435 to be a uint32!"),
                        row
                            .columns[444]
                            .into_u32()
                            .copied()
                            .expect("Expected column 444 to be a uint32!"),
                        row
                            .columns[453]
                            .into_u32()
                            .copied()
                            .expect("Expected column 453 to be a uint32!"),
                        row
                            .columns[462]
                            .into_u32()
                            .copied()
                            .expect("Expected column 462 to be a uint32!"),
                        row
                            .columns[471]
                            .into_u32()
                            .copied()
                            .expect("Expected column 471 to be a uint32!"),
                        row
                            .columns[480]
                            .into_u32()
                            .copied()
                            .expect("Expected column 480 to be a uint32!"),
                        row
                            .columns[489]
                            .into_u32()
                            .copied()
                            .expect("Expected column 489 to be a uint32!"),
                        row
                            .columns[498]
                            .into_u32()
                            .copied()
                            .expect("Expected column 498 to be a uint32!"),
                        row
                            .columns[507]
                            .into_u32()
                            .copied()
                            .expect("Expected column 507 to be a uint32!"),
                        row
                            .columns[516]
                            .into_u32()
                            .copied()
                            .expect("Expected column 516 to be a uint32!"),
                        row
                            .columns[525]
                            .into_u32()
                            .copied()
                            .expect("Expected column 525 to be a uint32!"),
                        row
                            .columns[534]
                            .into_u32()
                            .copied()
                            .expect("Expected column 534 to be a uint32!"),
                        row
                            .columns[543]
                            .into_u32()
                            .copied()
                            .expect("Expected column 543 to be a uint32!"),
                        row
                            .columns[552]
                            .into_u32()
                            .copied()
                            .expect("Expected column 552 to be a uint32!"),
                        row
                            .columns[561]
                            .into_u32()
                            .copied()
                            .expect("Expected column 561 to be a uint32!"),
                        row
                            .columns[570]
                            .into_u32()
                            .copied()
                            .expect("Expected column 570 to be a uint32!"),
                        row
                            .columns[579]
                            .into_u32()
                            .copied()
                            .expect("Expected column 579 to be a uint32!"),
                        row
                            .columns[588]
                            .into_u32()
                            .copied()
                            .expect("Expected column 588 to be a uint32!"),
                        row
                            .columns[597]
                            .into_u32()
                            .copied()
                            .expect("Expected column 597 to be a uint32!"),
                        row
                            .columns[606]
                            .into_u32()
                            .copied()
                            .expect("Expected column 606 to be a uint32!"),
                        row
                            .columns[615]
                            .into_u32()
                            .copied()
                            .expect("Expected column 615 to be a uint32!"),
                        row
                            .columns[624]
                            .into_u32()
                            .copied()
                            .expect("Expected column 624 to be a uint32!"),
                        row
                            .columns[633]
                            .into_u32()
                            .copied()
                            .expect("Expected column 633 to be a uint32!"),
                        row
                            .columns[642]
                            .into_u32()
                            .copied()
                            .expect("Expected column 642 to be a uint32!"),
                        row
                            .columns[651]
                            .into_u32()
                            .copied()
                            .expect("Expected column 651 to be a uint32!"),
                        row
                            .columns[660]
                            .into_u32()
                            .copied()
                            .expect("Expected column 660 to be a uint32!"),
                        row
                            .columns[669]
                            .into_u32()
                            .copied()
                            .expect("Expected column 669 to be a uint32!"),
                        row
                            .columns[678]
                            .into_u32()
                            .copied()
                            .expect("Expected column 678 to be a uint32!"),
                        row
                            .columns[687]
                            .into_u32()
                            .copied()
                            .expect("Expected column 687 to be a uint32!"),
                        row
                            .columns[696]
                            .into_u32()
                            .copied()
                            .expect("Expected column 696 to be a uint32!"),
                        row
                            .columns[705]
                            .into_u32()
                            .copied()
                            .expect("Expected column 705 to be a uint32!"),
                        row
                            .columns[714]
                            .into_u32()
                            .copied()
                            .expect("Expected column 714 to be a uint32!"),
                        row
                            .columns[723]
                            .into_u32()
                            .copied()
                            .expect("Expected column 723 to be a uint32!"),
                        row
                            .columns[732]
                            .into_u32()
                            .copied()
                            .expect("Expected column 732 to be a uint32!"),
                        row
                            .columns[741]
                            .into_u32()
                            .copied()
                            .expect("Expected column 741 to be a uint32!"),
                        row
                            .columns[750]
                            .into_u32()
                            .copied()
                            .expect("Expected column 750 to be a uint32!"),
                        row
                            .columns[759]
                            .into_u32()
                            .copied()
                            .expect("Expected column 759 to be a uint32!"),
                        row
                            .columns[768]
                            .into_u32()
                            .copied()
                            .expect("Expected column 768 to be a uint32!"),
                        row
                            .columns[777]
                            .into_u32()
                            .copied()
                            .expect("Expected column 777 to be a uint32!"),
                        row
                            .columns[786]
                            .into_u32()
                            .copied()
                            .expect("Expected column 786 to be a uint32!"),
                        row
                            .columns[795]
                            .into_u32()
                            .copied()
                            .expect("Expected column 795 to be a uint32!"),
                        row
                            .columns[804]
                            .into_u32()
                            .copied()
                            .expect("Expected column 804 to be a uint32!"),
                        row
                            .columns[813]
                            .into_u32()
                            .copied()
                            .expect("Expected column 813 to be a uint32!"),
                        row
                            .columns[822]
                            .into_u32()
                            .copied()
                            .expect("Expected column 822 to be a uint32!"),
                        row
                            .columns[831]
                            .into_u32()
                            .copied()
                            .expect("Expected column 831 to be a uint32!"),
                        row
                            .columns[840]
                            .into_u32()
                            .copied()
                            .expect("Expected column 840 to be a uint32!"),
                        row
                            .columns[849]
                            .into_u32()
                            .copied()
                            .expect("Expected column 849 to be a uint32!"),
                        row
                            .columns[858]
                            .into_u32()
                            .copied()
                            .expect("Expected column 858 to be a uint32!"),
                        row
                            .columns[867]
                            .into_u32()
                            .copied()
                            .expect("Expected column 867 to be a uint32!"),
                        row
                            .columns[876]
                            .into_u32()
                            .copied()
                            .expect("Expected column 876 to be a uint32!"),
                        row
                            .columns[885]
                            .into_u32()
                            .copied()
                            .expect("Expected column 885 to be a uint32!"),
                        row
                            .columns[894]
                            .into_u32()
                            .copied()
                            .expect("Expected column 894 to be a uint32!"),
                        row
                            .columns[903]
                            .into_u32()
                            .copied()
                            .expect("Expected column 903 to be a uint32!"),
                        row
                            .columns[912]
                            .into_u32()
                            .copied()
                            .expect("Expected column 912 to be a uint32!"),
                        row
                            .columns[921]
                            .into_u32()
                            .copied()
                            .expect("Expected column 921 to be a uint32!"),
                        row
                            .columns[930]
                            .into_u32()
                            .copied()
                            .expect("Expected column 930 to be a uint32!"),
                        row
                            .columns[939]
                            .into_u32()
                            .copied()
                            .expect("Expected column 939 to be a uint32!"),
                        row
                            .columns[948]
                            .into_u32()
                            .copied()
                            .expect("Expected column 948 to be a uint32!"),
                        row
                            .columns[957]
                            .into_u32()
                            .copied()
                            .expect("Expected column 957 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[966]
                            .into_u32()
                            .copied()
                            .expect("Expected column 966 to be a uint32!"),
                        row
                            .columns[975]
                            .into_u32()
                            .copied()
                            .expect("Expected column 975 to be a uint32!"),
                        row
                            .columns[984]
                            .into_u32()
                            .copied()
                            .expect("Expected column 984 to be a uint32!"),
                        row
                            .columns[993]
                            .into_u32()
                            .copied()
                            .expect("Expected column 993 to be a uint32!"),
                        row
                            .columns[1002]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1002 to be a uint32!"),
                        row
                            .columns[1011]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1011 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[12]
                        .into_u8()
                        .copied()
                        .expect("Expected column 12 to be a uint8!"),
                    SubMenuType: row
                        .columns[21]
                        .into_u8()
                        .copied()
                        .expect("Expected column 21 to be a uint8!"),
                    SubMenuNum: row
                        .columns[30]
                        .into_u8()
                        .copied()
                        .expect("Expected column 30 to be a uint8!"),
                    LookAt: row
                        .columns[39]
                        .into_u8()
                        .copied()
                        .expect("Expected column 39 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1020]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1020 to be a uint8!"),
                        row
                            .columns[1029]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1029 to be a uint8!"),
                        row
                            .columns[1038]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1038 to be a uint8!"),
                        row
                            .columns[1047]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1047 to be a uint8!"),
                        row
                            .columns[1056]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1056 to be a uint8!"),
                        row
                            .columns[1065]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1065 to be a uint8!"),
                        row
                            .columns[1074]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1074 to be a uint8!"),
                        row
                            .columns[1083]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1083 to be a uint8!"),
                        row
                            .columns[1092]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1092 to be a uint8!"),
                        row
                            .columns[1101]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1101 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    SubMenuMask: row
                        .columns[49]
                        .into_u32()
                        .copied()
                        .expect("Expected column 49 to be a uint32!"),
                    Customize: row
                        .columns[58]
                        .into_u32()
                        .copied()
                        .expect("Expected column 58 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[67]
                            .into_u32()
                            .copied()
                            .expect("Expected column 67 to be a uint32!"),
                        row
                            .columns[76]
                            .into_u32()
                            .copied()
                            .expect("Expected column 76 to be a uint32!"),
                        row
                            .columns[85]
                            .into_u32()
                            .copied()
                            .expect("Expected column 85 to be a uint32!"),
                        row
                            .columns[94]
                            .into_u32()
                            .copied()
                            .expect("Expected column 94 to be a uint32!"),
                        row
                            .columns[103]
                            .into_u32()
                            .copied()
                            .expect("Expected column 103 to be a uint32!"),
                        row
                            .columns[112]
                            .into_u32()
                            .copied()
                            .expect("Expected column 112 to be a uint32!"),
                        row
                            .columns[121]
                            .into_u32()
                            .copied()
                            .expect("Expected column 121 to be a uint32!"),
                        row
                            .columns[130]
                            .into_u32()
                            .copied()
                            .expect("Expected column 130 to be a uint32!"),
                        row
                            .columns[139]
                            .into_u32()
                            .copied()
                            .expect("Expected column 139 to be a uint32!"),
                        row
                            .columns[148]
                            .into_u32()
                            .copied()
                            .expect("Expected column 148 to be a uint32!"),
                        row
                            .columns[157]
                            .into_u32()
                            .copied()
                            .expect("Expected column 157 to be a uint32!"),
                        row
                            .columns[166]
                            .into_u32()
                            .copied()
                            .expect("Expected column 166 to be a uint32!"),
                        row
                            .columns[175]
                            .into_u32()
                            .copied()
                            .expect("Expected column 175 to be a uint32!"),
                        row
                            .columns[184]
                            .into_u32()
                            .copied()
                            .expect("Expected column 184 to be a uint32!"),
                        row
                            .columns[193]
                            .into_u32()
                            .copied()
                            .expect("Expected column 193 to be a uint32!"),
                        row
                            .columns[202]
                            .into_u32()
                            .copied()
                            .expect("Expected column 202 to be a uint32!"),
                        row
                            .columns[211]
                            .into_u32()
                            .copied()
                            .expect("Expected column 211 to be a uint32!"),
                        row
                            .columns[220]
                            .into_u32()
                            .copied()
                            .expect("Expected column 220 to be a uint32!"),
                        row
                            .columns[229]
                            .into_u32()
                            .copied()
                            .expect("Expected column 229 to be a uint32!"),
                        row
                            .columns[238]
                            .into_u32()
                            .copied()
                            .expect("Expected column 238 to be a uint32!"),
                        row
                            .columns[247]
                            .into_u32()
                            .copied()
                            .expect("Expected column 247 to be a uint32!"),
                        row
                            .columns[256]
                            .into_u32()
                            .copied()
                            .expect("Expected column 256 to be a uint32!"),
                        row
                            .columns[265]
                            .into_u32()
                            .copied()
                            .expect("Expected column 265 to be a uint32!"),
                        row
                            .columns[274]
                            .into_u32()
                            .copied()
                            .expect("Expected column 274 to be a uint32!"),
                        row
                            .columns[283]
                            .into_u32()
                            .copied()
                            .expect("Expected column 283 to be a uint32!"),
                        row
                            .columns[292]
                            .into_u32()
                            .copied()
                            .expect("Expected column 292 to be a uint32!"),
                        row
                            .columns[301]
                            .into_u32()
                            .copied()
                            .expect("Expected column 301 to be a uint32!"),
                        row
                            .columns[310]
                            .into_u32()
                            .copied()
                            .expect("Expected column 310 to be a uint32!"),
                        row
                            .columns[319]
                            .into_u32()
                            .copied()
                            .expect("Expected column 319 to be a uint32!"),
                        row
                            .columns[328]
                            .into_u32()
                            .copied()
                            .expect("Expected column 328 to be a uint32!"),
                        row
                            .columns[337]
                            .into_u32()
                            .copied()
                            .expect("Expected column 337 to be a uint32!"),
                        row
                            .columns[346]
                            .into_u32()
                            .copied()
                            .expect("Expected column 346 to be a uint32!"),
                        row
                            .columns[355]
                            .into_u32()
                            .copied()
                            .expect("Expected column 355 to be a uint32!"),
                        row
                            .columns[364]
                            .into_u32()
                            .copied()
                            .expect("Expected column 364 to be a uint32!"),
                        row
                            .columns[373]
                            .into_u32()
                            .copied()
                            .expect("Expected column 373 to be a uint32!"),
                        row
                            .columns[382]
                            .into_u32()
                            .copied()
                            .expect("Expected column 382 to be a uint32!"),
                        row
                            .columns[391]
                            .into_u32()
                            .copied()
                            .expect("Expected column 391 to be a uint32!"),
                        row
                            .columns[400]
                            .into_u32()
                            .copied()
                            .expect("Expected column 400 to be a uint32!"),
                        row
                            .columns[409]
                            .into_u32()
                            .copied()
                            .expect("Expected column 409 to be a uint32!"),
                        row
                            .columns[418]
                            .into_u32()
                            .copied()
                            .expect("Expected column 418 to be a uint32!"),
                        row
                            .columns[427]
                            .into_u32()
                            .copied()
                            .expect("Expected column 427 to be a uint32!"),
                        row
                            .columns[436]
                            .into_u32()
                            .copied()
                            .expect("Expected column 436 to be a uint32!"),
                        row
                            .columns[445]
                            .into_u32()
                            .copied()
                            .expect("Expected column 445 to be a uint32!"),
                        row
                            .columns[454]
                            .into_u32()
                            .copied()
                            .expect("Expected column 454 to be a uint32!"),
                        row
                            .columns[463]
                            .into_u32()
                            .copied()
                            .expect("Expected column 463 to be a uint32!"),
                        row
                            .columns[472]
                            .into_u32()
                            .copied()
                            .expect("Expected column 472 to be a uint32!"),
                        row
                            .columns[481]
                            .into_u32()
                            .copied()
                            .expect("Expected column 481 to be a uint32!"),
                        row
                            .columns[490]
                            .into_u32()
                            .copied()
                            .expect("Expected column 490 to be a uint32!"),
                        row
                            .columns[499]
                            .into_u32()
                            .copied()
                            .expect("Expected column 499 to be a uint32!"),
                        row
                            .columns[508]
                            .into_u32()
                            .copied()
                            .expect("Expected column 508 to be a uint32!"),
                        row
                            .columns[517]
                            .into_u32()
                            .copied()
                            .expect("Expected column 517 to be a uint32!"),
                        row
                            .columns[526]
                            .into_u32()
                            .copied()
                            .expect("Expected column 526 to be a uint32!"),
                        row
                            .columns[535]
                            .into_u32()
                            .copied()
                            .expect("Expected column 535 to be a uint32!"),
                        row
                            .columns[544]
                            .into_u32()
                            .copied()
                            .expect("Expected column 544 to be a uint32!"),
                        row
                            .columns[553]
                            .into_u32()
                            .copied()
                            .expect("Expected column 553 to be a uint32!"),
                        row
                            .columns[562]
                            .into_u32()
                            .copied()
                            .expect("Expected column 562 to be a uint32!"),
                        row
                            .columns[571]
                            .into_u32()
                            .copied()
                            .expect("Expected column 571 to be a uint32!"),
                        row
                            .columns[580]
                            .into_u32()
                            .copied()
                            .expect("Expected column 580 to be a uint32!"),
                        row
                            .columns[589]
                            .into_u32()
                            .copied()
                            .expect("Expected column 589 to be a uint32!"),
                        row
                            .columns[598]
                            .into_u32()
                            .copied()
                            .expect("Expected column 598 to be a uint32!"),
                        row
                            .columns[607]
                            .into_u32()
                            .copied()
                            .expect("Expected column 607 to be a uint32!"),
                        row
                            .columns[616]
                            .into_u32()
                            .copied()
                            .expect("Expected column 616 to be a uint32!"),
                        row
                            .columns[625]
                            .into_u32()
                            .copied()
                            .expect("Expected column 625 to be a uint32!"),
                        row
                            .columns[634]
                            .into_u32()
                            .copied()
                            .expect("Expected column 634 to be a uint32!"),
                        row
                            .columns[643]
                            .into_u32()
                            .copied()
                            .expect("Expected column 643 to be a uint32!"),
                        row
                            .columns[652]
                            .into_u32()
                            .copied()
                            .expect("Expected column 652 to be a uint32!"),
                        row
                            .columns[661]
                            .into_u32()
                            .copied()
                            .expect("Expected column 661 to be a uint32!"),
                        row
                            .columns[670]
                            .into_u32()
                            .copied()
                            .expect("Expected column 670 to be a uint32!"),
                        row
                            .columns[679]
                            .into_u32()
                            .copied()
                            .expect("Expected column 679 to be a uint32!"),
                        row
                            .columns[688]
                            .into_u32()
                            .copied()
                            .expect("Expected column 688 to be a uint32!"),
                        row
                            .columns[697]
                            .into_u32()
                            .copied()
                            .expect("Expected column 697 to be a uint32!"),
                        row
                            .columns[706]
                            .into_u32()
                            .copied()
                            .expect("Expected column 706 to be a uint32!"),
                        row
                            .columns[715]
                            .into_u32()
                            .copied()
                            .expect("Expected column 715 to be a uint32!"),
                        row
                            .columns[724]
                            .into_u32()
                            .copied()
                            .expect("Expected column 724 to be a uint32!"),
                        row
                            .columns[733]
                            .into_u32()
                            .copied()
                            .expect("Expected column 733 to be a uint32!"),
                        row
                            .columns[742]
                            .into_u32()
                            .copied()
                            .expect("Expected column 742 to be a uint32!"),
                        row
                            .columns[751]
                            .into_u32()
                            .copied()
                            .expect("Expected column 751 to be a uint32!"),
                        row
                            .columns[760]
                            .into_u32()
                            .copied()
                            .expect("Expected column 760 to be a uint32!"),
                        row
                            .columns[769]
                            .into_u32()
                            .copied()
                            .expect("Expected column 769 to be a uint32!"),
                        row
                            .columns[778]
                            .into_u32()
                            .copied()
                            .expect("Expected column 778 to be a uint32!"),
                        row
                            .columns[787]
                            .into_u32()
                            .copied()
                            .expect("Expected column 787 to be a uint32!"),
                        row
                            .columns[796]
                            .into_u32()
                            .copied()
                            .expect("Expected column 796 to be a uint32!"),
                        row
                            .columns[805]
                            .into_u32()
                            .copied()
                            .expect("Expected column 805 to be a uint32!"),
                        row
                            .columns[814]
                            .into_u32()
                            .copied()
                            .expect("Expected column 814 to be a uint32!"),
                        row
                            .columns[823]
                            .into_u32()
                            .copied()
                            .expect("Expected column 823 to be a uint32!"),
                        row
                            .columns[832]
                            .into_u32()
                            .copied()
                            .expect("Expected column 832 to be a uint32!"),
                        row
                            .columns[841]
                            .into_u32()
                            .copied()
                            .expect("Expected column 841 to be a uint32!"),
                        row
                            .columns[850]
                            .into_u32()
                            .copied()
                            .expect("Expected column 850 to be a uint32!"),
                        row
                            .columns[859]
                            .into_u32()
                            .copied()
                            .expect("Expected column 859 to be a uint32!"),
                        row
                            .columns[868]
                            .into_u32()
                            .copied()
                            .expect("Expected column 868 to be a uint32!"),
                        row
                            .columns[877]
                            .into_u32()
                            .copied()
                            .expect("Expected column 877 to be a uint32!"),
                        row
                            .columns[886]
                            .into_u32()
                            .copied()
                            .expect("Expected column 886 to be a uint32!"),
                        row
                            .columns[895]
                            .into_u32()
                            .copied()
                            .expect("Expected column 895 to be a uint32!"),
                        row
                            .columns[904]
                            .into_u32()
                            .copied()
                            .expect("Expected column 904 to be a uint32!"),
                        row
                            .columns[913]
                            .into_u32()
                            .copied()
                            .expect("Expected column 913 to be a uint32!"),
                        row
                            .columns[922]
                            .into_u32()
                            .copied()
                            .expect("Expected column 922 to be a uint32!"),
                        row
                            .columns[931]
                            .into_u32()
                            .copied()
                            .expect("Expected column 931 to be a uint32!"),
                        row
                            .columns[940]
                            .into_u32()
                            .copied()
                            .expect("Expected column 940 to be a uint32!"),
                        row
                            .columns[949]
                            .into_u32()
                            .copied()
                            .expect("Expected column 949 to be a uint32!"),
                        row
                            .columns[958]
                            .into_u32()
                            .copied()
                            .expect("Expected column 958 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[967]
                            .into_u32()
                            .copied()
                            .expect("Expected column 967 to be a uint32!"),
                        row
                            .columns[976]
                            .into_u32()
                            .copied()
                            .expect("Expected column 976 to be a uint32!"),
                        row
                            .columns[985]
                            .into_u32()
                            .copied()
                            .expect("Expected column 985 to be a uint32!"),
                        row
                            .columns[994]
                            .into_u32()
                            .copied()
                            .expect("Expected column 994 to be a uint32!"),
                        row
                            .columns[1003]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1003 to be a uint32!"),
                        row
                            .columns[1012]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1012 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[13]
                        .into_u8()
                        .copied()
                        .expect("Expected column 13 to be a uint8!"),
                    SubMenuType: row
                        .columns[22]
                        .into_u8()
                        .copied()
                        .expect("Expected column 22 to be a uint8!"),
                    SubMenuNum: row
                        .columns[31]
                        .into_u8()
                        .copied()
                        .expect("Expected column 31 to be a uint8!"),
                    LookAt: row
                        .columns[40]
                        .into_u8()
                        .copied()
                        .expect("Expected column 40 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1021]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1021 to be a uint8!"),
                        row
                            .columns[1030]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1030 to be a uint8!"),
                        row
                            .columns[1039]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1039 to be a uint8!"),
                        row
                            .columns[1048]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1048 to be a uint8!"),
                        row
                            .columns[1057]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1057 to be a uint8!"),
                        row
                            .columns[1066]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1066 to be a uint8!"),
                        row
                            .columns[1075]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1075 to be a uint8!"),
                        row
                            .columns[1084]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1084 to be a uint8!"),
                        row
                            .columns[1093]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1093 to be a uint8!"),
                        row
                            .columns[1102]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1102 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    SubMenuMask: row
                        .columns[50]
                        .into_u32()
                        .copied()
                        .expect("Expected column 50 to be a uint32!"),
                    Customize: row
                        .columns[59]
                        .into_u32()
                        .copied()
                        .expect("Expected column 59 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[68]
                            .into_u32()
                            .copied()
                            .expect("Expected column 68 to be a uint32!"),
                        row
                            .columns[77]
                            .into_u32()
                            .copied()
                            .expect("Expected column 77 to be a uint32!"),
                        row
                            .columns[86]
                            .into_u32()
                            .copied()
                            .expect("Expected column 86 to be a uint32!"),
                        row
                            .columns[95]
                            .into_u32()
                            .copied()
                            .expect("Expected column 95 to be a uint32!"),
                        row
                            .columns[104]
                            .into_u32()
                            .copied()
                            .expect("Expected column 104 to be a uint32!"),
                        row
                            .columns[113]
                            .into_u32()
                            .copied()
                            .expect("Expected column 113 to be a uint32!"),
                        row
                            .columns[122]
                            .into_u32()
                            .copied()
                            .expect("Expected column 122 to be a uint32!"),
                        row
                            .columns[131]
                            .into_u32()
                            .copied()
                            .expect("Expected column 131 to be a uint32!"),
                        row
                            .columns[140]
                            .into_u32()
                            .copied()
                            .expect("Expected column 140 to be a uint32!"),
                        row
                            .columns[149]
                            .into_u32()
                            .copied()
                            .expect("Expected column 149 to be a uint32!"),
                        row
                            .columns[158]
                            .into_u32()
                            .copied()
                            .expect("Expected column 158 to be a uint32!"),
                        row
                            .columns[167]
                            .into_u32()
                            .copied()
                            .expect("Expected column 167 to be a uint32!"),
                        row
                            .columns[176]
                            .into_u32()
                            .copied()
                            .expect("Expected column 176 to be a uint32!"),
                        row
                            .columns[185]
                            .into_u32()
                            .copied()
                            .expect("Expected column 185 to be a uint32!"),
                        row
                            .columns[194]
                            .into_u32()
                            .copied()
                            .expect("Expected column 194 to be a uint32!"),
                        row
                            .columns[203]
                            .into_u32()
                            .copied()
                            .expect("Expected column 203 to be a uint32!"),
                        row
                            .columns[212]
                            .into_u32()
                            .copied()
                            .expect("Expected column 212 to be a uint32!"),
                        row
                            .columns[221]
                            .into_u32()
                            .copied()
                            .expect("Expected column 221 to be a uint32!"),
                        row
                            .columns[230]
                            .into_u32()
                            .copied()
                            .expect("Expected column 230 to be a uint32!"),
                        row
                            .columns[239]
                            .into_u32()
                            .copied()
                            .expect("Expected column 239 to be a uint32!"),
                        row
                            .columns[248]
                            .into_u32()
                            .copied()
                            .expect("Expected column 248 to be a uint32!"),
                        row
                            .columns[257]
                            .into_u32()
                            .copied()
                            .expect("Expected column 257 to be a uint32!"),
                        row
                            .columns[266]
                            .into_u32()
                            .copied()
                            .expect("Expected column 266 to be a uint32!"),
                        row
                            .columns[275]
                            .into_u32()
                            .copied()
                            .expect("Expected column 275 to be a uint32!"),
                        row
                            .columns[284]
                            .into_u32()
                            .copied()
                            .expect("Expected column 284 to be a uint32!"),
                        row
                            .columns[293]
                            .into_u32()
                            .copied()
                            .expect("Expected column 293 to be a uint32!"),
                        row
                            .columns[302]
                            .into_u32()
                            .copied()
                            .expect("Expected column 302 to be a uint32!"),
                        row
                            .columns[311]
                            .into_u32()
                            .copied()
                            .expect("Expected column 311 to be a uint32!"),
                        row
                            .columns[320]
                            .into_u32()
                            .copied()
                            .expect("Expected column 320 to be a uint32!"),
                        row
                            .columns[329]
                            .into_u32()
                            .copied()
                            .expect("Expected column 329 to be a uint32!"),
                        row
                            .columns[338]
                            .into_u32()
                            .copied()
                            .expect("Expected column 338 to be a uint32!"),
                        row
                            .columns[347]
                            .into_u32()
                            .copied()
                            .expect("Expected column 347 to be a uint32!"),
                        row
                            .columns[356]
                            .into_u32()
                            .copied()
                            .expect("Expected column 356 to be a uint32!"),
                        row
                            .columns[365]
                            .into_u32()
                            .copied()
                            .expect("Expected column 365 to be a uint32!"),
                        row
                            .columns[374]
                            .into_u32()
                            .copied()
                            .expect("Expected column 374 to be a uint32!"),
                        row
                            .columns[383]
                            .into_u32()
                            .copied()
                            .expect("Expected column 383 to be a uint32!"),
                        row
                            .columns[392]
                            .into_u32()
                            .copied()
                            .expect("Expected column 392 to be a uint32!"),
                        row
                            .columns[401]
                            .into_u32()
                            .copied()
                            .expect("Expected column 401 to be a uint32!"),
                        row
                            .columns[410]
                            .into_u32()
                            .copied()
                            .expect("Expected column 410 to be a uint32!"),
                        row
                            .columns[419]
                            .into_u32()
                            .copied()
                            .expect("Expected column 419 to be a uint32!"),
                        row
                            .columns[428]
                            .into_u32()
                            .copied()
                            .expect("Expected column 428 to be a uint32!"),
                        row
                            .columns[437]
                            .into_u32()
                            .copied()
                            .expect("Expected column 437 to be a uint32!"),
                        row
                            .columns[446]
                            .into_u32()
                            .copied()
                            .expect("Expected column 446 to be a uint32!"),
                        row
                            .columns[455]
                            .into_u32()
                            .copied()
                            .expect("Expected column 455 to be a uint32!"),
                        row
                            .columns[464]
                            .into_u32()
                            .copied()
                            .expect("Expected column 464 to be a uint32!"),
                        row
                            .columns[473]
                            .into_u32()
                            .copied()
                            .expect("Expected column 473 to be a uint32!"),
                        row
                            .columns[482]
                            .into_u32()
                            .copied()
                            .expect("Expected column 482 to be a uint32!"),
                        row
                            .columns[491]
                            .into_u32()
                            .copied()
                            .expect("Expected column 491 to be a uint32!"),
                        row
                            .columns[500]
                            .into_u32()
                            .copied()
                            .expect("Expected column 500 to be a uint32!"),
                        row
                            .columns[509]
                            .into_u32()
                            .copied()
                            .expect("Expected column 509 to be a uint32!"),
                        row
                            .columns[518]
                            .into_u32()
                            .copied()
                            .expect("Expected column 518 to be a uint32!"),
                        row
                            .columns[527]
                            .into_u32()
                            .copied()
                            .expect("Expected column 527 to be a uint32!"),
                        row
                            .columns[536]
                            .into_u32()
                            .copied()
                            .expect("Expected column 536 to be a uint32!"),
                        row
                            .columns[545]
                            .into_u32()
                            .copied()
                            .expect("Expected column 545 to be a uint32!"),
                        row
                            .columns[554]
                            .into_u32()
                            .copied()
                            .expect("Expected column 554 to be a uint32!"),
                        row
                            .columns[563]
                            .into_u32()
                            .copied()
                            .expect("Expected column 563 to be a uint32!"),
                        row
                            .columns[572]
                            .into_u32()
                            .copied()
                            .expect("Expected column 572 to be a uint32!"),
                        row
                            .columns[581]
                            .into_u32()
                            .copied()
                            .expect("Expected column 581 to be a uint32!"),
                        row
                            .columns[590]
                            .into_u32()
                            .copied()
                            .expect("Expected column 590 to be a uint32!"),
                        row
                            .columns[599]
                            .into_u32()
                            .copied()
                            .expect("Expected column 599 to be a uint32!"),
                        row
                            .columns[608]
                            .into_u32()
                            .copied()
                            .expect("Expected column 608 to be a uint32!"),
                        row
                            .columns[617]
                            .into_u32()
                            .copied()
                            .expect("Expected column 617 to be a uint32!"),
                        row
                            .columns[626]
                            .into_u32()
                            .copied()
                            .expect("Expected column 626 to be a uint32!"),
                        row
                            .columns[635]
                            .into_u32()
                            .copied()
                            .expect("Expected column 635 to be a uint32!"),
                        row
                            .columns[644]
                            .into_u32()
                            .copied()
                            .expect("Expected column 644 to be a uint32!"),
                        row
                            .columns[653]
                            .into_u32()
                            .copied()
                            .expect("Expected column 653 to be a uint32!"),
                        row
                            .columns[662]
                            .into_u32()
                            .copied()
                            .expect("Expected column 662 to be a uint32!"),
                        row
                            .columns[671]
                            .into_u32()
                            .copied()
                            .expect("Expected column 671 to be a uint32!"),
                        row
                            .columns[680]
                            .into_u32()
                            .copied()
                            .expect("Expected column 680 to be a uint32!"),
                        row
                            .columns[689]
                            .into_u32()
                            .copied()
                            .expect("Expected column 689 to be a uint32!"),
                        row
                            .columns[698]
                            .into_u32()
                            .copied()
                            .expect("Expected column 698 to be a uint32!"),
                        row
                            .columns[707]
                            .into_u32()
                            .copied()
                            .expect("Expected column 707 to be a uint32!"),
                        row
                            .columns[716]
                            .into_u32()
                            .copied()
                            .expect("Expected column 716 to be a uint32!"),
                        row
                            .columns[725]
                            .into_u32()
                            .copied()
                            .expect("Expected column 725 to be a uint32!"),
                        row
                            .columns[734]
                            .into_u32()
                            .copied()
                            .expect("Expected column 734 to be a uint32!"),
                        row
                            .columns[743]
                            .into_u32()
                            .copied()
                            .expect("Expected column 743 to be a uint32!"),
                        row
                            .columns[752]
                            .into_u32()
                            .copied()
                            .expect("Expected column 752 to be a uint32!"),
                        row
                            .columns[761]
                            .into_u32()
                            .copied()
                            .expect("Expected column 761 to be a uint32!"),
                        row
                            .columns[770]
                            .into_u32()
                            .copied()
                            .expect("Expected column 770 to be a uint32!"),
                        row
                            .columns[779]
                            .into_u32()
                            .copied()
                            .expect("Expected column 779 to be a uint32!"),
                        row
                            .columns[788]
                            .into_u32()
                            .copied()
                            .expect("Expected column 788 to be a uint32!"),
                        row
                            .columns[797]
                            .into_u32()
                            .copied()
                            .expect("Expected column 797 to be a uint32!"),
                        row
                            .columns[806]
                            .into_u32()
                            .copied()
                            .expect("Expected column 806 to be a uint32!"),
                        row
                            .columns[815]
                            .into_u32()
                            .copied()
                            .expect("Expected column 815 to be a uint32!"),
                        row
                            .columns[824]
                            .into_u32()
                            .copied()
                            .expect("Expected column 824 to be a uint32!"),
                        row
                            .columns[833]
                            .into_u32()
                            .copied()
                            .expect("Expected column 833 to be a uint32!"),
                        row
                            .columns[842]
                            .into_u32()
                            .copied()
                            .expect("Expected column 842 to be a uint32!"),
                        row
                            .columns[851]
                            .into_u32()
                            .copied()
                            .expect("Expected column 851 to be a uint32!"),
                        row
                            .columns[860]
                            .into_u32()
                            .copied()
                            .expect("Expected column 860 to be a uint32!"),
                        row
                            .columns[869]
                            .into_u32()
                            .copied()
                            .expect("Expected column 869 to be a uint32!"),
                        row
                            .columns[878]
                            .into_u32()
                            .copied()
                            .expect("Expected column 878 to be a uint32!"),
                        row
                            .columns[887]
                            .into_u32()
                            .copied()
                            .expect("Expected column 887 to be a uint32!"),
                        row
                            .columns[896]
                            .into_u32()
                            .copied()
                            .expect("Expected column 896 to be a uint32!"),
                        row
                            .columns[905]
                            .into_u32()
                            .copied()
                            .expect("Expected column 905 to be a uint32!"),
                        row
                            .columns[914]
                            .into_u32()
                            .copied()
                            .expect("Expected column 914 to be a uint32!"),
                        row
                            .columns[923]
                            .into_u32()
                            .copied()
                            .expect("Expected column 923 to be a uint32!"),
                        row
                            .columns[932]
                            .into_u32()
                            .copied()
                            .expect("Expected column 932 to be a uint32!"),
                        row
                            .columns[941]
                            .into_u32()
                            .copied()
                            .expect("Expected column 941 to be a uint32!"),
                        row
                            .columns[950]
                            .into_u32()
                            .copied()
                            .expect("Expected column 950 to be a uint32!"),
                        row
                            .columns[959]
                            .into_u32()
                            .copied()
                            .expect("Expected column 959 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[968]
                            .into_u32()
                            .copied()
                            .expect("Expected column 968 to be a uint32!"),
                        row
                            .columns[977]
                            .into_u32()
                            .copied()
                            .expect("Expected column 977 to be a uint32!"),
                        row
                            .columns[986]
                            .into_u32()
                            .copied()
                            .expect("Expected column 986 to be a uint32!"),
                        row
                            .columns[995]
                            .into_u32()
                            .copied()
                            .expect("Expected column 995 to be a uint32!"),
                        row
                            .columns[1004]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1004 to be a uint32!"),
                        row
                            .columns[1013]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1013 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[14]
                        .into_u8()
                        .copied()
                        .expect("Expected column 14 to be a uint8!"),
                    SubMenuType: row
                        .columns[23]
                        .into_u8()
                        .copied()
                        .expect("Expected column 23 to be a uint8!"),
                    SubMenuNum: row
                        .columns[32]
                        .into_u8()
                        .copied()
                        .expect("Expected column 32 to be a uint8!"),
                    LookAt: row
                        .columns[41]
                        .into_u8()
                        .copied()
                        .expect("Expected column 41 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1022]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1022 to be a uint8!"),
                        row
                            .columns[1031]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1031 to be a uint8!"),
                        row
                            .columns[1040]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1040 to be a uint8!"),
                        row
                            .columns[1049]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1049 to be a uint8!"),
                        row
                            .columns[1058]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1058 to be a uint8!"),
                        row
                            .columns[1067]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1067 to be a uint8!"),
                        row
                            .columns[1076]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1076 to be a uint8!"),
                        row
                            .columns[1085]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1085 to be a uint8!"),
                        row
                            .columns[1094]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1094 to be a uint8!"),
                        row
                            .columns[1103]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1103 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    SubMenuMask: row
                        .columns[51]
                        .into_u32()
                        .copied()
                        .expect("Expected column 51 to be a uint32!"),
                    Customize: row
                        .columns[60]
                        .into_u32()
                        .copied()
                        .expect("Expected column 60 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[69]
                            .into_u32()
                            .copied()
                            .expect("Expected column 69 to be a uint32!"),
                        row
                            .columns[78]
                            .into_u32()
                            .copied()
                            .expect("Expected column 78 to be a uint32!"),
                        row
                            .columns[87]
                            .into_u32()
                            .copied()
                            .expect("Expected column 87 to be a uint32!"),
                        row
                            .columns[96]
                            .into_u32()
                            .copied()
                            .expect("Expected column 96 to be a uint32!"),
                        row
                            .columns[105]
                            .into_u32()
                            .copied()
                            .expect("Expected column 105 to be a uint32!"),
                        row
                            .columns[114]
                            .into_u32()
                            .copied()
                            .expect("Expected column 114 to be a uint32!"),
                        row
                            .columns[123]
                            .into_u32()
                            .copied()
                            .expect("Expected column 123 to be a uint32!"),
                        row
                            .columns[132]
                            .into_u32()
                            .copied()
                            .expect("Expected column 132 to be a uint32!"),
                        row
                            .columns[141]
                            .into_u32()
                            .copied()
                            .expect("Expected column 141 to be a uint32!"),
                        row
                            .columns[150]
                            .into_u32()
                            .copied()
                            .expect("Expected column 150 to be a uint32!"),
                        row
                            .columns[159]
                            .into_u32()
                            .copied()
                            .expect("Expected column 159 to be a uint32!"),
                        row
                            .columns[168]
                            .into_u32()
                            .copied()
                            .expect("Expected column 168 to be a uint32!"),
                        row
                            .columns[177]
                            .into_u32()
                            .copied()
                            .expect("Expected column 177 to be a uint32!"),
                        row
                            .columns[186]
                            .into_u32()
                            .copied()
                            .expect("Expected column 186 to be a uint32!"),
                        row
                            .columns[195]
                            .into_u32()
                            .copied()
                            .expect("Expected column 195 to be a uint32!"),
                        row
                            .columns[204]
                            .into_u32()
                            .copied()
                            .expect("Expected column 204 to be a uint32!"),
                        row
                            .columns[213]
                            .into_u32()
                            .copied()
                            .expect("Expected column 213 to be a uint32!"),
                        row
                            .columns[222]
                            .into_u32()
                            .copied()
                            .expect("Expected column 222 to be a uint32!"),
                        row
                            .columns[231]
                            .into_u32()
                            .copied()
                            .expect("Expected column 231 to be a uint32!"),
                        row
                            .columns[240]
                            .into_u32()
                            .copied()
                            .expect("Expected column 240 to be a uint32!"),
                        row
                            .columns[249]
                            .into_u32()
                            .copied()
                            .expect("Expected column 249 to be a uint32!"),
                        row
                            .columns[258]
                            .into_u32()
                            .copied()
                            .expect("Expected column 258 to be a uint32!"),
                        row
                            .columns[267]
                            .into_u32()
                            .copied()
                            .expect("Expected column 267 to be a uint32!"),
                        row
                            .columns[276]
                            .into_u32()
                            .copied()
                            .expect("Expected column 276 to be a uint32!"),
                        row
                            .columns[285]
                            .into_u32()
                            .copied()
                            .expect("Expected column 285 to be a uint32!"),
                        row
                            .columns[294]
                            .into_u32()
                            .copied()
                            .expect("Expected column 294 to be a uint32!"),
                        row
                            .columns[303]
                            .into_u32()
                            .copied()
                            .expect("Expected column 303 to be a uint32!"),
                        row
                            .columns[312]
                            .into_u32()
                            .copied()
                            .expect("Expected column 312 to be a uint32!"),
                        row
                            .columns[321]
                            .into_u32()
                            .copied()
                            .expect("Expected column 321 to be a uint32!"),
                        row
                            .columns[330]
                            .into_u32()
                            .copied()
                            .expect("Expected column 330 to be a uint32!"),
                        row
                            .columns[339]
                            .into_u32()
                            .copied()
                            .expect("Expected column 339 to be a uint32!"),
                        row
                            .columns[348]
                            .into_u32()
                            .copied()
                            .expect("Expected column 348 to be a uint32!"),
                        row
                            .columns[357]
                            .into_u32()
                            .copied()
                            .expect("Expected column 357 to be a uint32!"),
                        row
                            .columns[366]
                            .into_u32()
                            .copied()
                            .expect("Expected column 366 to be a uint32!"),
                        row
                            .columns[375]
                            .into_u32()
                            .copied()
                            .expect("Expected column 375 to be a uint32!"),
                        row
                            .columns[384]
                            .into_u32()
                            .copied()
                            .expect("Expected column 384 to be a uint32!"),
                        row
                            .columns[393]
                            .into_u32()
                            .copied()
                            .expect("Expected column 393 to be a uint32!"),
                        row
                            .columns[402]
                            .into_u32()
                            .copied()
                            .expect("Expected column 402 to be a uint32!"),
                        row
                            .columns[411]
                            .into_u32()
                            .copied()
                            .expect("Expected column 411 to be a uint32!"),
                        row
                            .columns[420]
                            .into_u32()
                            .copied()
                            .expect("Expected column 420 to be a uint32!"),
                        row
                            .columns[429]
                            .into_u32()
                            .copied()
                            .expect("Expected column 429 to be a uint32!"),
                        row
                            .columns[438]
                            .into_u32()
                            .copied()
                            .expect("Expected column 438 to be a uint32!"),
                        row
                            .columns[447]
                            .into_u32()
                            .copied()
                            .expect("Expected column 447 to be a uint32!"),
                        row
                            .columns[456]
                            .into_u32()
                            .copied()
                            .expect("Expected column 456 to be a uint32!"),
                        row
                            .columns[465]
                            .into_u32()
                            .copied()
                            .expect("Expected column 465 to be a uint32!"),
                        row
                            .columns[474]
                            .into_u32()
                            .copied()
                            .expect("Expected column 474 to be a uint32!"),
                        row
                            .columns[483]
                            .into_u32()
                            .copied()
                            .expect("Expected column 483 to be a uint32!"),
                        row
                            .columns[492]
                            .into_u32()
                            .copied()
                            .expect("Expected column 492 to be a uint32!"),
                        row
                            .columns[501]
                            .into_u32()
                            .copied()
                            .expect("Expected column 501 to be a uint32!"),
                        row
                            .columns[510]
                            .into_u32()
                            .copied()
                            .expect("Expected column 510 to be a uint32!"),
                        row
                            .columns[519]
                            .into_u32()
                            .copied()
                            .expect("Expected column 519 to be a uint32!"),
                        row
                            .columns[528]
                            .into_u32()
                            .copied()
                            .expect("Expected column 528 to be a uint32!"),
                        row
                            .columns[537]
                            .into_u32()
                            .copied()
                            .expect("Expected column 537 to be a uint32!"),
                        row
                            .columns[546]
                            .into_u32()
                            .copied()
                            .expect("Expected column 546 to be a uint32!"),
                        row
                            .columns[555]
                            .into_u32()
                            .copied()
                            .expect("Expected column 555 to be a uint32!"),
                        row
                            .columns[564]
                            .into_u32()
                            .copied()
                            .expect("Expected column 564 to be a uint32!"),
                        row
                            .columns[573]
                            .into_u32()
                            .copied()
                            .expect("Expected column 573 to be a uint32!"),
                        row
                            .columns[582]
                            .into_u32()
                            .copied()
                            .expect("Expected column 582 to be a uint32!"),
                        row
                            .columns[591]
                            .into_u32()
                            .copied()
                            .expect("Expected column 591 to be a uint32!"),
                        row
                            .columns[600]
                            .into_u32()
                            .copied()
                            .expect("Expected column 600 to be a uint32!"),
                        row
                            .columns[609]
                            .into_u32()
                            .copied()
                            .expect("Expected column 609 to be a uint32!"),
                        row
                            .columns[618]
                            .into_u32()
                            .copied()
                            .expect("Expected column 618 to be a uint32!"),
                        row
                            .columns[627]
                            .into_u32()
                            .copied()
                            .expect("Expected column 627 to be a uint32!"),
                        row
                            .columns[636]
                            .into_u32()
                            .copied()
                            .expect("Expected column 636 to be a uint32!"),
                        row
                            .columns[645]
                            .into_u32()
                            .copied()
                            .expect("Expected column 645 to be a uint32!"),
                        row
                            .columns[654]
                            .into_u32()
                            .copied()
                            .expect("Expected column 654 to be a uint32!"),
                        row
                            .columns[663]
                            .into_u32()
                            .copied()
                            .expect("Expected column 663 to be a uint32!"),
                        row
                            .columns[672]
                            .into_u32()
                            .copied()
                            .expect("Expected column 672 to be a uint32!"),
                        row
                            .columns[681]
                            .into_u32()
                            .copied()
                            .expect("Expected column 681 to be a uint32!"),
                        row
                            .columns[690]
                            .into_u32()
                            .copied()
                            .expect("Expected column 690 to be a uint32!"),
                        row
                            .columns[699]
                            .into_u32()
                            .copied()
                            .expect("Expected column 699 to be a uint32!"),
                        row
                            .columns[708]
                            .into_u32()
                            .copied()
                            .expect("Expected column 708 to be a uint32!"),
                        row
                            .columns[717]
                            .into_u32()
                            .copied()
                            .expect("Expected column 717 to be a uint32!"),
                        row
                            .columns[726]
                            .into_u32()
                            .copied()
                            .expect("Expected column 726 to be a uint32!"),
                        row
                            .columns[735]
                            .into_u32()
                            .copied()
                            .expect("Expected column 735 to be a uint32!"),
                        row
                            .columns[744]
                            .into_u32()
                            .copied()
                            .expect("Expected column 744 to be a uint32!"),
                        row
                            .columns[753]
                            .into_u32()
                            .copied()
                            .expect("Expected column 753 to be a uint32!"),
                        row
                            .columns[762]
                            .into_u32()
                            .copied()
                            .expect("Expected column 762 to be a uint32!"),
                        row
                            .columns[771]
                            .into_u32()
                            .copied()
                            .expect("Expected column 771 to be a uint32!"),
                        row
                            .columns[780]
                            .into_u32()
                            .copied()
                            .expect("Expected column 780 to be a uint32!"),
                        row
                            .columns[789]
                            .into_u32()
                            .copied()
                            .expect("Expected column 789 to be a uint32!"),
                        row
                            .columns[798]
                            .into_u32()
                            .copied()
                            .expect("Expected column 798 to be a uint32!"),
                        row
                            .columns[807]
                            .into_u32()
                            .copied()
                            .expect("Expected column 807 to be a uint32!"),
                        row
                            .columns[816]
                            .into_u32()
                            .copied()
                            .expect("Expected column 816 to be a uint32!"),
                        row
                            .columns[825]
                            .into_u32()
                            .copied()
                            .expect("Expected column 825 to be a uint32!"),
                        row
                            .columns[834]
                            .into_u32()
                            .copied()
                            .expect("Expected column 834 to be a uint32!"),
                        row
                            .columns[843]
                            .into_u32()
                            .copied()
                            .expect("Expected column 843 to be a uint32!"),
                        row
                            .columns[852]
                            .into_u32()
                            .copied()
                            .expect("Expected column 852 to be a uint32!"),
                        row
                            .columns[861]
                            .into_u32()
                            .copied()
                            .expect("Expected column 861 to be a uint32!"),
                        row
                            .columns[870]
                            .into_u32()
                            .copied()
                            .expect("Expected column 870 to be a uint32!"),
                        row
                            .columns[879]
                            .into_u32()
                            .copied()
                            .expect("Expected column 879 to be a uint32!"),
                        row
                            .columns[888]
                            .into_u32()
                            .copied()
                            .expect("Expected column 888 to be a uint32!"),
                        row
                            .columns[897]
                            .into_u32()
                            .copied()
                            .expect("Expected column 897 to be a uint32!"),
                        row
                            .columns[906]
                            .into_u32()
                            .copied()
                            .expect("Expected column 906 to be a uint32!"),
                        row
                            .columns[915]
                            .into_u32()
                            .copied()
                            .expect("Expected column 915 to be a uint32!"),
                        row
                            .columns[924]
                            .into_u32()
                            .copied()
                            .expect("Expected column 924 to be a uint32!"),
                        row
                            .columns[933]
                            .into_u32()
                            .copied()
                            .expect("Expected column 933 to be a uint32!"),
                        row
                            .columns[942]
                            .into_u32()
                            .copied()
                            .expect("Expected column 942 to be a uint32!"),
                        row
                            .columns[951]
                            .into_u32()
                            .copied()
                            .expect("Expected column 951 to be a uint32!"),
                        row
                            .columns[960]
                            .into_u32()
                            .copied()
                            .expect("Expected column 960 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[969]
                            .into_u32()
                            .copied()
                            .expect("Expected column 969 to be a uint32!"),
                        row
                            .columns[978]
                            .into_u32()
                            .copied()
                            .expect("Expected column 978 to be a uint32!"),
                        row
                            .columns[987]
                            .into_u32()
                            .copied()
                            .expect("Expected column 987 to be a uint32!"),
                        row
                            .columns[996]
                            .into_u32()
                            .copied()
                            .expect("Expected column 996 to be a uint32!"),
                        row
                            .columns[1005]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1005 to be a uint32!"),
                        row
                            .columns[1014]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1014 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[15]
                        .into_u8()
                        .copied()
                        .expect("Expected column 15 to be a uint8!"),
                    SubMenuType: row
                        .columns[24]
                        .into_u8()
                        .copied()
                        .expect("Expected column 24 to be a uint8!"),
                    SubMenuNum: row
                        .columns[33]
                        .into_u8()
                        .copied()
                        .expect("Expected column 33 to be a uint8!"),
                    LookAt: row
                        .columns[42]
                        .into_u8()
                        .copied()
                        .expect("Expected column 42 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1023]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1023 to be a uint8!"),
                        row
                            .columns[1032]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1032 to be a uint8!"),
                        row
                            .columns[1041]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1041 to be a uint8!"),
                        row
                            .columns[1050]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1050 to be a uint8!"),
                        row
                            .columns[1059]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1059 to be a uint8!"),
                        row
                            .columns[1068]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1068 to be a uint8!"),
                        row
                            .columns[1077]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1077 to be a uint8!"),
                        row
                            .columns[1086]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1086 to be a uint8!"),
                        row
                            .columns[1095]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1095 to be a uint8!"),
                        row
                            .columns[1104]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1104 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    SubMenuMask: row
                        .columns[52]
                        .into_u32()
                        .copied()
                        .expect("Expected column 52 to be a uint32!"),
                    Customize: row
                        .columns[61]
                        .into_u32()
                        .copied()
                        .expect("Expected column 61 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[70]
                            .into_u32()
                            .copied()
                            .expect("Expected column 70 to be a uint32!"),
                        row
                            .columns[79]
                            .into_u32()
                            .copied()
                            .expect("Expected column 79 to be a uint32!"),
                        row
                            .columns[88]
                            .into_u32()
                            .copied()
                            .expect("Expected column 88 to be a uint32!"),
                        row
                            .columns[97]
                            .into_u32()
                            .copied()
                            .expect("Expected column 97 to be a uint32!"),
                        row
                            .columns[106]
                            .into_u32()
                            .copied()
                            .expect("Expected column 106 to be a uint32!"),
                        row
                            .columns[115]
                            .into_u32()
                            .copied()
                            .expect("Expected column 115 to be a uint32!"),
                        row
                            .columns[124]
                            .into_u32()
                            .copied()
                            .expect("Expected column 124 to be a uint32!"),
                        row
                            .columns[133]
                            .into_u32()
                            .copied()
                            .expect("Expected column 133 to be a uint32!"),
                        row
                            .columns[142]
                            .into_u32()
                            .copied()
                            .expect("Expected column 142 to be a uint32!"),
                        row
                            .columns[151]
                            .into_u32()
                            .copied()
                            .expect("Expected column 151 to be a uint32!"),
                        row
                            .columns[160]
                            .into_u32()
                            .copied()
                            .expect("Expected column 160 to be a uint32!"),
                        row
                            .columns[169]
                            .into_u32()
                            .copied()
                            .expect("Expected column 169 to be a uint32!"),
                        row
                            .columns[178]
                            .into_u32()
                            .copied()
                            .expect("Expected column 178 to be a uint32!"),
                        row
                            .columns[187]
                            .into_u32()
                            .copied()
                            .expect("Expected column 187 to be a uint32!"),
                        row
                            .columns[196]
                            .into_u32()
                            .copied()
                            .expect("Expected column 196 to be a uint32!"),
                        row
                            .columns[205]
                            .into_u32()
                            .copied()
                            .expect("Expected column 205 to be a uint32!"),
                        row
                            .columns[214]
                            .into_u32()
                            .copied()
                            .expect("Expected column 214 to be a uint32!"),
                        row
                            .columns[223]
                            .into_u32()
                            .copied()
                            .expect("Expected column 223 to be a uint32!"),
                        row
                            .columns[232]
                            .into_u32()
                            .copied()
                            .expect("Expected column 232 to be a uint32!"),
                        row
                            .columns[241]
                            .into_u32()
                            .copied()
                            .expect("Expected column 241 to be a uint32!"),
                        row
                            .columns[250]
                            .into_u32()
                            .copied()
                            .expect("Expected column 250 to be a uint32!"),
                        row
                            .columns[259]
                            .into_u32()
                            .copied()
                            .expect("Expected column 259 to be a uint32!"),
                        row
                            .columns[268]
                            .into_u32()
                            .copied()
                            .expect("Expected column 268 to be a uint32!"),
                        row
                            .columns[277]
                            .into_u32()
                            .copied()
                            .expect("Expected column 277 to be a uint32!"),
                        row
                            .columns[286]
                            .into_u32()
                            .copied()
                            .expect("Expected column 286 to be a uint32!"),
                        row
                            .columns[295]
                            .into_u32()
                            .copied()
                            .expect("Expected column 295 to be a uint32!"),
                        row
                            .columns[304]
                            .into_u32()
                            .copied()
                            .expect("Expected column 304 to be a uint32!"),
                        row
                            .columns[313]
                            .into_u32()
                            .copied()
                            .expect("Expected column 313 to be a uint32!"),
                        row
                            .columns[322]
                            .into_u32()
                            .copied()
                            .expect("Expected column 322 to be a uint32!"),
                        row
                            .columns[331]
                            .into_u32()
                            .copied()
                            .expect("Expected column 331 to be a uint32!"),
                        row
                            .columns[340]
                            .into_u32()
                            .copied()
                            .expect("Expected column 340 to be a uint32!"),
                        row
                            .columns[349]
                            .into_u32()
                            .copied()
                            .expect("Expected column 349 to be a uint32!"),
                        row
                            .columns[358]
                            .into_u32()
                            .copied()
                            .expect("Expected column 358 to be a uint32!"),
                        row
                            .columns[367]
                            .into_u32()
                            .copied()
                            .expect("Expected column 367 to be a uint32!"),
                        row
                            .columns[376]
                            .into_u32()
                            .copied()
                            .expect("Expected column 376 to be a uint32!"),
                        row
                            .columns[385]
                            .into_u32()
                            .copied()
                            .expect("Expected column 385 to be a uint32!"),
                        row
                            .columns[394]
                            .into_u32()
                            .copied()
                            .expect("Expected column 394 to be a uint32!"),
                        row
                            .columns[403]
                            .into_u32()
                            .copied()
                            .expect("Expected column 403 to be a uint32!"),
                        row
                            .columns[412]
                            .into_u32()
                            .copied()
                            .expect("Expected column 412 to be a uint32!"),
                        row
                            .columns[421]
                            .into_u32()
                            .copied()
                            .expect("Expected column 421 to be a uint32!"),
                        row
                            .columns[430]
                            .into_u32()
                            .copied()
                            .expect("Expected column 430 to be a uint32!"),
                        row
                            .columns[439]
                            .into_u32()
                            .copied()
                            .expect("Expected column 439 to be a uint32!"),
                        row
                            .columns[448]
                            .into_u32()
                            .copied()
                            .expect("Expected column 448 to be a uint32!"),
                        row
                            .columns[457]
                            .into_u32()
                            .copied()
                            .expect("Expected column 457 to be a uint32!"),
                        row
                            .columns[466]
                            .into_u32()
                            .copied()
                            .expect("Expected column 466 to be a uint32!"),
                        row
                            .columns[475]
                            .into_u32()
                            .copied()
                            .expect("Expected column 475 to be a uint32!"),
                        row
                            .columns[484]
                            .into_u32()
                            .copied()
                            .expect("Expected column 484 to be a uint32!"),
                        row
                            .columns[493]
                            .into_u32()
                            .copied()
                            .expect("Expected column 493 to be a uint32!"),
                        row
                            .columns[502]
                            .into_u32()
                            .copied()
                            .expect("Expected column 502 to be a uint32!"),
                        row
                            .columns[511]
                            .into_u32()
                            .copied()
                            .expect("Expected column 511 to be a uint32!"),
                        row
                            .columns[520]
                            .into_u32()
                            .copied()
                            .expect("Expected column 520 to be a uint32!"),
                        row
                            .columns[529]
                            .into_u32()
                            .copied()
                            .expect("Expected column 529 to be a uint32!"),
                        row
                            .columns[538]
                            .into_u32()
                            .copied()
                            .expect("Expected column 538 to be a uint32!"),
                        row
                            .columns[547]
                            .into_u32()
                            .copied()
                            .expect("Expected column 547 to be a uint32!"),
                        row
                            .columns[556]
                            .into_u32()
                            .copied()
                            .expect("Expected column 556 to be a uint32!"),
                        row
                            .columns[565]
                            .into_u32()
                            .copied()
                            .expect("Expected column 565 to be a uint32!"),
                        row
                            .columns[574]
                            .into_u32()
                            .copied()
                            .expect("Expected column 574 to be a uint32!"),
                        row
                            .columns[583]
                            .into_u32()
                            .copied()
                            .expect("Expected column 583 to be a uint32!"),
                        row
                            .columns[592]
                            .into_u32()
                            .copied()
                            .expect("Expected column 592 to be a uint32!"),
                        row
                            .columns[601]
                            .into_u32()
                            .copied()
                            .expect("Expected column 601 to be a uint32!"),
                        row
                            .columns[610]
                            .into_u32()
                            .copied()
                            .expect("Expected column 610 to be a uint32!"),
                        row
                            .columns[619]
                            .into_u32()
                            .copied()
                            .expect("Expected column 619 to be a uint32!"),
                        row
                            .columns[628]
                            .into_u32()
                            .copied()
                            .expect("Expected column 628 to be a uint32!"),
                        row
                            .columns[637]
                            .into_u32()
                            .copied()
                            .expect("Expected column 637 to be a uint32!"),
                        row
                            .columns[646]
                            .into_u32()
                            .copied()
                            .expect("Expected column 646 to be a uint32!"),
                        row
                            .columns[655]
                            .into_u32()
                            .copied()
                            .expect("Expected column 655 to be a uint32!"),
                        row
                            .columns[664]
                            .into_u32()
                            .copied()
                            .expect("Expected column 664 to be a uint32!"),
                        row
                            .columns[673]
                            .into_u32()
                            .copied()
                            .expect("Expected column 673 to be a uint32!"),
                        row
                            .columns[682]
                            .into_u32()
                            .copied()
                            .expect("Expected column 682 to be a uint32!"),
                        row
                            .columns[691]
                            .into_u32()
                            .copied()
                            .expect("Expected column 691 to be a uint32!"),
                        row
                            .columns[700]
                            .into_u32()
                            .copied()
                            .expect("Expected column 700 to be a uint32!"),
                        row
                            .columns[709]
                            .into_u32()
                            .copied()
                            .expect("Expected column 709 to be a uint32!"),
                        row
                            .columns[718]
                            .into_u32()
                            .copied()
                            .expect("Expected column 718 to be a uint32!"),
                        row
                            .columns[727]
                            .into_u32()
                            .copied()
                            .expect("Expected column 727 to be a uint32!"),
                        row
                            .columns[736]
                            .into_u32()
                            .copied()
                            .expect("Expected column 736 to be a uint32!"),
                        row
                            .columns[745]
                            .into_u32()
                            .copied()
                            .expect("Expected column 745 to be a uint32!"),
                        row
                            .columns[754]
                            .into_u32()
                            .copied()
                            .expect("Expected column 754 to be a uint32!"),
                        row
                            .columns[763]
                            .into_u32()
                            .copied()
                            .expect("Expected column 763 to be a uint32!"),
                        row
                            .columns[772]
                            .into_u32()
                            .copied()
                            .expect("Expected column 772 to be a uint32!"),
                        row
                            .columns[781]
                            .into_u32()
                            .copied()
                            .expect("Expected column 781 to be a uint32!"),
                        row
                            .columns[790]
                            .into_u32()
                            .copied()
                            .expect("Expected column 790 to be a uint32!"),
                        row
                            .columns[799]
                            .into_u32()
                            .copied()
                            .expect("Expected column 799 to be a uint32!"),
                        row
                            .columns[808]
                            .into_u32()
                            .copied()
                            .expect("Expected column 808 to be a uint32!"),
                        row
                            .columns[817]
                            .into_u32()
                            .copied()
                            .expect("Expected column 817 to be a uint32!"),
                        row
                            .columns[826]
                            .into_u32()
                            .copied()
                            .expect("Expected column 826 to be a uint32!"),
                        row
                            .columns[835]
                            .into_u32()
                            .copied()
                            .expect("Expected column 835 to be a uint32!"),
                        row
                            .columns[844]
                            .into_u32()
                            .copied()
                            .expect("Expected column 844 to be a uint32!"),
                        row
                            .columns[853]
                            .into_u32()
                            .copied()
                            .expect("Expected column 853 to be a uint32!"),
                        row
                            .columns[862]
                            .into_u32()
                            .copied()
                            .expect("Expected column 862 to be a uint32!"),
                        row
                            .columns[871]
                            .into_u32()
                            .copied()
                            .expect("Expected column 871 to be a uint32!"),
                        row
                            .columns[880]
                            .into_u32()
                            .copied()
                            .expect("Expected column 880 to be a uint32!"),
                        row
                            .columns[889]
                            .into_u32()
                            .copied()
                            .expect("Expected column 889 to be a uint32!"),
                        row
                            .columns[898]
                            .into_u32()
                            .copied()
                            .expect("Expected column 898 to be a uint32!"),
                        row
                            .columns[907]
                            .into_u32()
                            .copied()
                            .expect("Expected column 907 to be a uint32!"),
                        row
                            .columns[916]
                            .into_u32()
                            .copied()
                            .expect("Expected column 916 to be a uint32!"),
                        row
                            .columns[925]
                            .into_u32()
                            .copied()
                            .expect("Expected column 925 to be a uint32!"),
                        row
                            .columns[934]
                            .into_u32()
                            .copied()
                            .expect("Expected column 934 to be a uint32!"),
                        row
                            .columns[943]
                            .into_u32()
                            .copied()
                            .expect("Expected column 943 to be a uint32!"),
                        row
                            .columns[952]
                            .into_u32()
                            .copied()
                            .expect("Expected column 952 to be a uint32!"),
                        row
                            .columns[961]
                            .into_u32()
                            .copied()
                            .expect("Expected column 961 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[970]
                            .into_u32()
                            .copied()
                            .expect("Expected column 970 to be a uint32!"),
                        row
                            .columns[979]
                            .into_u32()
                            .copied()
                            .expect("Expected column 979 to be a uint32!"),
                        row
                            .columns[988]
                            .into_u32()
                            .copied()
                            .expect("Expected column 988 to be a uint32!"),
                        row
                            .columns[997]
                            .into_u32()
                            .copied()
                            .expect("Expected column 997 to be a uint32!"),
                        row
                            .columns[1006]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1006 to be a uint32!"),
                        row
                            .columns[1015]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1015 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[16]
                        .into_u8()
                        .copied()
                        .expect("Expected column 16 to be a uint8!"),
                    SubMenuType: row
                        .columns[25]
                        .into_u8()
                        .copied()
                        .expect("Expected column 25 to be a uint8!"),
                    SubMenuNum: row
                        .columns[34]
                        .into_u8()
                        .copied()
                        .expect("Expected column 34 to be a uint8!"),
                    LookAt: row
                        .columns[43]
                        .into_u8()
                        .copied()
                        .expect("Expected column 43 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1024]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1024 to be a uint8!"),
                        row
                            .columns[1033]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1033 to be a uint8!"),
                        row
                            .columns[1042]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1042 to be a uint8!"),
                        row
                            .columns[1051]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1051 to be a uint8!"),
                        row
                            .columns[1060]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1060 to be a uint8!"),
                        row
                            .columns[1069]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1069 to be a uint8!"),
                        row
                            .columns[1078]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1078 to be a uint8!"),
                        row
                            .columns[1087]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1087 to be a uint8!"),
                        row
                            .columns[1096]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1096 to be a uint8!"),
                        row
                            .columns[1105]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1105 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    SubMenuMask: row
                        .columns[53]
                        .into_u32()
                        .copied()
                        .expect("Expected column 53 to be a uint32!"),
                    Customize: row
                        .columns[62]
                        .into_u32()
                        .copied()
                        .expect("Expected column 62 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[71]
                            .into_u32()
                            .copied()
                            .expect("Expected column 71 to be a uint32!"),
                        row
                            .columns[80]
                            .into_u32()
                            .copied()
                            .expect("Expected column 80 to be a uint32!"),
                        row
                            .columns[89]
                            .into_u32()
                            .copied()
                            .expect("Expected column 89 to be a uint32!"),
                        row
                            .columns[98]
                            .into_u32()
                            .copied()
                            .expect("Expected column 98 to be a uint32!"),
                        row
                            .columns[107]
                            .into_u32()
                            .copied()
                            .expect("Expected column 107 to be a uint32!"),
                        row
                            .columns[116]
                            .into_u32()
                            .copied()
                            .expect("Expected column 116 to be a uint32!"),
                        row
                            .columns[125]
                            .into_u32()
                            .copied()
                            .expect("Expected column 125 to be a uint32!"),
                        row
                            .columns[134]
                            .into_u32()
                            .copied()
                            .expect("Expected column 134 to be a uint32!"),
                        row
                            .columns[143]
                            .into_u32()
                            .copied()
                            .expect("Expected column 143 to be a uint32!"),
                        row
                            .columns[152]
                            .into_u32()
                            .copied()
                            .expect("Expected column 152 to be a uint32!"),
                        row
                            .columns[161]
                            .into_u32()
                            .copied()
                            .expect("Expected column 161 to be a uint32!"),
                        row
                            .columns[170]
                            .into_u32()
                            .copied()
                            .expect("Expected column 170 to be a uint32!"),
                        row
                            .columns[179]
                            .into_u32()
                            .copied()
                            .expect("Expected column 179 to be a uint32!"),
                        row
                            .columns[188]
                            .into_u32()
                            .copied()
                            .expect("Expected column 188 to be a uint32!"),
                        row
                            .columns[197]
                            .into_u32()
                            .copied()
                            .expect("Expected column 197 to be a uint32!"),
                        row
                            .columns[206]
                            .into_u32()
                            .copied()
                            .expect("Expected column 206 to be a uint32!"),
                        row
                            .columns[215]
                            .into_u32()
                            .copied()
                            .expect("Expected column 215 to be a uint32!"),
                        row
                            .columns[224]
                            .into_u32()
                            .copied()
                            .expect("Expected column 224 to be a uint32!"),
                        row
                            .columns[233]
                            .into_u32()
                            .copied()
                            .expect("Expected column 233 to be a uint32!"),
                        row
                            .columns[242]
                            .into_u32()
                            .copied()
                            .expect("Expected column 242 to be a uint32!"),
                        row
                            .columns[251]
                            .into_u32()
                            .copied()
                            .expect("Expected column 251 to be a uint32!"),
                        row
                            .columns[260]
                            .into_u32()
                            .copied()
                            .expect("Expected column 260 to be a uint32!"),
                        row
                            .columns[269]
                            .into_u32()
                            .copied()
                            .expect("Expected column 269 to be a uint32!"),
                        row
                            .columns[278]
                            .into_u32()
                            .copied()
                            .expect("Expected column 278 to be a uint32!"),
                        row
                            .columns[287]
                            .into_u32()
                            .copied()
                            .expect("Expected column 287 to be a uint32!"),
                        row
                            .columns[296]
                            .into_u32()
                            .copied()
                            .expect("Expected column 296 to be a uint32!"),
                        row
                            .columns[305]
                            .into_u32()
                            .copied()
                            .expect("Expected column 305 to be a uint32!"),
                        row
                            .columns[314]
                            .into_u32()
                            .copied()
                            .expect("Expected column 314 to be a uint32!"),
                        row
                            .columns[323]
                            .into_u32()
                            .copied()
                            .expect("Expected column 323 to be a uint32!"),
                        row
                            .columns[332]
                            .into_u32()
                            .copied()
                            .expect("Expected column 332 to be a uint32!"),
                        row
                            .columns[341]
                            .into_u32()
                            .copied()
                            .expect("Expected column 341 to be a uint32!"),
                        row
                            .columns[350]
                            .into_u32()
                            .copied()
                            .expect("Expected column 350 to be a uint32!"),
                        row
                            .columns[359]
                            .into_u32()
                            .copied()
                            .expect("Expected column 359 to be a uint32!"),
                        row
                            .columns[368]
                            .into_u32()
                            .copied()
                            .expect("Expected column 368 to be a uint32!"),
                        row
                            .columns[377]
                            .into_u32()
                            .copied()
                            .expect("Expected column 377 to be a uint32!"),
                        row
                            .columns[386]
                            .into_u32()
                            .copied()
                            .expect("Expected column 386 to be a uint32!"),
                        row
                            .columns[395]
                            .into_u32()
                            .copied()
                            .expect("Expected column 395 to be a uint32!"),
                        row
                            .columns[404]
                            .into_u32()
                            .copied()
                            .expect("Expected column 404 to be a uint32!"),
                        row
                            .columns[413]
                            .into_u32()
                            .copied()
                            .expect("Expected column 413 to be a uint32!"),
                        row
                            .columns[422]
                            .into_u32()
                            .copied()
                            .expect("Expected column 422 to be a uint32!"),
                        row
                            .columns[431]
                            .into_u32()
                            .copied()
                            .expect("Expected column 431 to be a uint32!"),
                        row
                            .columns[440]
                            .into_u32()
                            .copied()
                            .expect("Expected column 440 to be a uint32!"),
                        row
                            .columns[449]
                            .into_u32()
                            .copied()
                            .expect("Expected column 449 to be a uint32!"),
                        row
                            .columns[458]
                            .into_u32()
                            .copied()
                            .expect("Expected column 458 to be a uint32!"),
                        row
                            .columns[467]
                            .into_u32()
                            .copied()
                            .expect("Expected column 467 to be a uint32!"),
                        row
                            .columns[476]
                            .into_u32()
                            .copied()
                            .expect("Expected column 476 to be a uint32!"),
                        row
                            .columns[485]
                            .into_u32()
                            .copied()
                            .expect("Expected column 485 to be a uint32!"),
                        row
                            .columns[494]
                            .into_u32()
                            .copied()
                            .expect("Expected column 494 to be a uint32!"),
                        row
                            .columns[503]
                            .into_u32()
                            .copied()
                            .expect("Expected column 503 to be a uint32!"),
                        row
                            .columns[512]
                            .into_u32()
                            .copied()
                            .expect("Expected column 512 to be a uint32!"),
                        row
                            .columns[521]
                            .into_u32()
                            .copied()
                            .expect("Expected column 521 to be a uint32!"),
                        row
                            .columns[530]
                            .into_u32()
                            .copied()
                            .expect("Expected column 530 to be a uint32!"),
                        row
                            .columns[539]
                            .into_u32()
                            .copied()
                            .expect("Expected column 539 to be a uint32!"),
                        row
                            .columns[548]
                            .into_u32()
                            .copied()
                            .expect("Expected column 548 to be a uint32!"),
                        row
                            .columns[557]
                            .into_u32()
                            .copied()
                            .expect("Expected column 557 to be a uint32!"),
                        row
                            .columns[566]
                            .into_u32()
                            .copied()
                            .expect("Expected column 566 to be a uint32!"),
                        row
                            .columns[575]
                            .into_u32()
                            .copied()
                            .expect("Expected column 575 to be a uint32!"),
                        row
                            .columns[584]
                            .into_u32()
                            .copied()
                            .expect("Expected column 584 to be a uint32!"),
                        row
                            .columns[593]
                            .into_u32()
                            .copied()
                            .expect("Expected column 593 to be a uint32!"),
                        row
                            .columns[602]
                            .into_u32()
                            .copied()
                            .expect("Expected column 602 to be a uint32!"),
                        row
                            .columns[611]
                            .into_u32()
                            .copied()
                            .expect("Expected column 611 to be a uint32!"),
                        row
                            .columns[620]
                            .into_u32()
                            .copied()
                            .expect("Expected column 620 to be a uint32!"),
                        row
                            .columns[629]
                            .into_u32()
                            .copied()
                            .expect("Expected column 629 to be a uint32!"),
                        row
                            .columns[638]
                            .into_u32()
                            .copied()
                            .expect("Expected column 638 to be a uint32!"),
                        row
                            .columns[647]
                            .into_u32()
                            .copied()
                            .expect("Expected column 647 to be a uint32!"),
                        row
                            .columns[656]
                            .into_u32()
                            .copied()
                            .expect("Expected column 656 to be a uint32!"),
                        row
                            .columns[665]
                            .into_u32()
                            .copied()
                            .expect("Expected column 665 to be a uint32!"),
                        row
                            .columns[674]
                            .into_u32()
                            .copied()
                            .expect("Expected column 674 to be a uint32!"),
                        row
                            .columns[683]
                            .into_u32()
                            .copied()
                            .expect("Expected column 683 to be a uint32!"),
                        row
                            .columns[692]
                            .into_u32()
                            .copied()
                            .expect("Expected column 692 to be a uint32!"),
                        row
                            .columns[701]
                            .into_u32()
                            .copied()
                            .expect("Expected column 701 to be a uint32!"),
                        row
                            .columns[710]
                            .into_u32()
                            .copied()
                            .expect("Expected column 710 to be a uint32!"),
                        row
                            .columns[719]
                            .into_u32()
                            .copied()
                            .expect("Expected column 719 to be a uint32!"),
                        row
                            .columns[728]
                            .into_u32()
                            .copied()
                            .expect("Expected column 728 to be a uint32!"),
                        row
                            .columns[737]
                            .into_u32()
                            .copied()
                            .expect("Expected column 737 to be a uint32!"),
                        row
                            .columns[746]
                            .into_u32()
                            .copied()
                            .expect("Expected column 746 to be a uint32!"),
                        row
                            .columns[755]
                            .into_u32()
                            .copied()
                            .expect("Expected column 755 to be a uint32!"),
                        row
                            .columns[764]
                            .into_u32()
                            .copied()
                            .expect("Expected column 764 to be a uint32!"),
                        row
                            .columns[773]
                            .into_u32()
                            .copied()
                            .expect("Expected column 773 to be a uint32!"),
                        row
                            .columns[782]
                            .into_u32()
                            .copied()
                            .expect("Expected column 782 to be a uint32!"),
                        row
                            .columns[791]
                            .into_u32()
                            .copied()
                            .expect("Expected column 791 to be a uint32!"),
                        row
                            .columns[800]
                            .into_u32()
                            .copied()
                            .expect("Expected column 800 to be a uint32!"),
                        row
                            .columns[809]
                            .into_u32()
                            .copied()
                            .expect("Expected column 809 to be a uint32!"),
                        row
                            .columns[818]
                            .into_u32()
                            .copied()
                            .expect("Expected column 818 to be a uint32!"),
                        row
                            .columns[827]
                            .into_u32()
                            .copied()
                            .expect("Expected column 827 to be a uint32!"),
                        row
                            .columns[836]
                            .into_u32()
                            .copied()
                            .expect("Expected column 836 to be a uint32!"),
                        row
                            .columns[845]
                            .into_u32()
                            .copied()
                            .expect("Expected column 845 to be a uint32!"),
                        row
                            .columns[854]
                            .into_u32()
                            .copied()
                            .expect("Expected column 854 to be a uint32!"),
                        row
                            .columns[863]
                            .into_u32()
                            .copied()
                            .expect("Expected column 863 to be a uint32!"),
                        row
                            .columns[872]
                            .into_u32()
                            .copied()
                            .expect("Expected column 872 to be a uint32!"),
                        row
                            .columns[881]
                            .into_u32()
                            .copied()
                            .expect("Expected column 881 to be a uint32!"),
                        row
                            .columns[890]
                            .into_u32()
                            .copied()
                            .expect("Expected column 890 to be a uint32!"),
                        row
                            .columns[899]
                            .into_u32()
                            .copied()
                            .expect("Expected column 899 to be a uint32!"),
                        row
                            .columns[908]
                            .into_u32()
                            .copied()
                            .expect("Expected column 908 to be a uint32!"),
                        row
                            .columns[917]
                            .into_u32()
                            .copied()
                            .expect("Expected column 917 to be a uint32!"),
                        row
                            .columns[926]
                            .into_u32()
                            .copied()
                            .expect("Expected column 926 to be a uint32!"),
                        row
                            .columns[935]
                            .into_u32()
                            .copied()
                            .expect("Expected column 935 to be a uint32!"),
                        row
                            .columns[944]
                            .into_u32()
                            .copied()
                            .expect("Expected column 944 to be a uint32!"),
                        row
                            .columns[953]
                            .into_u32()
                            .copied()
                            .expect("Expected column 953 to be a uint32!"),
                        row
                            .columns[962]
                            .into_u32()
                            .copied()
                            .expect("Expected column 962 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[971]
                            .into_u32()
                            .copied()
                            .expect("Expected column 971 to be a uint32!"),
                        row
                            .columns[980]
                            .into_u32()
                            .copied()
                            .expect("Expected column 980 to be a uint32!"),
                        row
                            .columns[989]
                            .into_u32()
                            .copied()
                            .expect("Expected column 989 to be a uint32!"),
                        row
                            .columns[998]
                            .into_u32()
                            .copied()
                            .expect("Expected column 998 to be a uint32!"),
                        row
                            .columns[1007]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1007 to be a uint32!"),
                        row
                            .columns[1016]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1016 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[17]
                        .into_u8()
                        .copied()
                        .expect("Expected column 17 to be a uint8!"),
                    SubMenuType: row
                        .columns[26]
                        .into_u8()
                        .copied()
                        .expect("Expected column 26 to be a uint8!"),
                    SubMenuNum: row
                        .columns[35]
                        .into_u8()
                        .copied()
                        .expect("Expected column 35 to be a uint8!"),
                    LookAt: row
                        .columns[44]
                        .into_u8()
                        .copied()
                        .expect("Expected column 44 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1025]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1025 to be a uint8!"),
                        row
                            .columns[1034]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1034 to be a uint8!"),
                        row
                            .columns[1043]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1043 to be a uint8!"),
                        row
                            .columns[1052]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1052 to be a uint8!"),
                        row
                            .columns[1061]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1061 to be a uint8!"),
                        row
                            .columns[1070]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1070 to be a uint8!"),
                        row
                            .columns[1079]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1079 to be a uint8!"),
                        row
                            .columns[1088]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1088 to be a uint8!"),
                        row
                            .columns[1097]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1097 to be a uint8!"),
                        row
                            .columns[1106]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1106 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    SubMenuMask: row
                        .columns[54]
                        .into_u32()
                        .copied()
                        .expect("Expected column 54 to be a uint32!"),
                    Customize: row
                        .columns[63]
                        .into_u32()
                        .copied()
                        .expect("Expected column 63 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[72]
                            .into_u32()
                            .copied()
                            .expect("Expected column 72 to be a uint32!"),
                        row
                            .columns[81]
                            .into_u32()
                            .copied()
                            .expect("Expected column 81 to be a uint32!"),
                        row
                            .columns[90]
                            .into_u32()
                            .copied()
                            .expect("Expected column 90 to be a uint32!"),
                        row
                            .columns[99]
                            .into_u32()
                            .copied()
                            .expect("Expected column 99 to be a uint32!"),
                        row
                            .columns[108]
                            .into_u32()
                            .copied()
                            .expect("Expected column 108 to be a uint32!"),
                        row
                            .columns[117]
                            .into_u32()
                            .copied()
                            .expect("Expected column 117 to be a uint32!"),
                        row
                            .columns[126]
                            .into_u32()
                            .copied()
                            .expect("Expected column 126 to be a uint32!"),
                        row
                            .columns[135]
                            .into_u32()
                            .copied()
                            .expect("Expected column 135 to be a uint32!"),
                        row
                            .columns[144]
                            .into_u32()
                            .copied()
                            .expect("Expected column 144 to be a uint32!"),
                        row
                            .columns[153]
                            .into_u32()
                            .copied()
                            .expect("Expected column 153 to be a uint32!"),
                        row
                            .columns[162]
                            .into_u32()
                            .copied()
                            .expect("Expected column 162 to be a uint32!"),
                        row
                            .columns[171]
                            .into_u32()
                            .copied()
                            .expect("Expected column 171 to be a uint32!"),
                        row
                            .columns[180]
                            .into_u32()
                            .copied()
                            .expect("Expected column 180 to be a uint32!"),
                        row
                            .columns[189]
                            .into_u32()
                            .copied()
                            .expect("Expected column 189 to be a uint32!"),
                        row
                            .columns[198]
                            .into_u32()
                            .copied()
                            .expect("Expected column 198 to be a uint32!"),
                        row
                            .columns[207]
                            .into_u32()
                            .copied()
                            .expect("Expected column 207 to be a uint32!"),
                        row
                            .columns[216]
                            .into_u32()
                            .copied()
                            .expect("Expected column 216 to be a uint32!"),
                        row
                            .columns[225]
                            .into_u32()
                            .copied()
                            .expect("Expected column 225 to be a uint32!"),
                        row
                            .columns[234]
                            .into_u32()
                            .copied()
                            .expect("Expected column 234 to be a uint32!"),
                        row
                            .columns[243]
                            .into_u32()
                            .copied()
                            .expect("Expected column 243 to be a uint32!"),
                        row
                            .columns[252]
                            .into_u32()
                            .copied()
                            .expect("Expected column 252 to be a uint32!"),
                        row
                            .columns[261]
                            .into_u32()
                            .copied()
                            .expect("Expected column 261 to be a uint32!"),
                        row
                            .columns[270]
                            .into_u32()
                            .copied()
                            .expect("Expected column 270 to be a uint32!"),
                        row
                            .columns[279]
                            .into_u32()
                            .copied()
                            .expect("Expected column 279 to be a uint32!"),
                        row
                            .columns[288]
                            .into_u32()
                            .copied()
                            .expect("Expected column 288 to be a uint32!"),
                        row
                            .columns[297]
                            .into_u32()
                            .copied()
                            .expect("Expected column 297 to be a uint32!"),
                        row
                            .columns[306]
                            .into_u32()
                            .copied()
                            .expect("Expected column 306 to be a uint32!"),
                        row
                            .columns[315]
                            .into_u32()
                            .copied()
                            .expect("Expected column 315 to be a uint32!"),
                        row
                            .columns[324]
                            .into_u32()
                            .copied()
                            .expect("Expected column 324 to be a uint32!"),
                        row
                            .columns[333]
                            .into_u32()
                            .copied()
                            .expect("Expected column 333 to be a uint32!"),
                        row
                            .columns[342]
                            .into_u32()
                            .copied()
                            .expect("Expected column 342 to be a uint32!"),
                        row
                            .columns[351]
                            .into_u32()
                            .copied()
                            .expect("Expected column 351 to be a uint32!"),
                        row
                            .columns[360]
                            .into_u32()
                            .copied()
                            .expect("Expected column 360 to be a uint32!"),
                        row
                            .columns[369]
                            .into_u32()
                            .copied()
                            .expect("Expected column 369 to be a uint32!"),
                        row
                            .columns[378]
                            .into_u32()
                            .copied()
                            .expect("Expected column 378 to be a uint32!"),
                        row
                            .columns[387]
                            .into_u32()
                            .copied()
                            .expect("Expected column 387 to be a uint32!"),
                        row
                            .columns[396]
                            .into_u32()
                            .copied()
                            .expect("Expected column 396 to be a uint32!"),
                        row
                            .columns[405]
                            .into_u32()
                            .copied()
                            .expect("Expected column 405 to be a uint32!"),
                        row
                            .columns[414]
                            .into_u32()
                            .copied()
                            .expect("Expected column 414 to be a uint32!"),
                        row
                            .columns[423]
                            .into_u32()
                            .copied()
                            .expect("Expected column 423 to be a uint32!"),
                        row
                            .columns[432]
                            .into_u32()
                            .copied()
                            .expect("Expected column 432 to be a uint32!"),
                        row
                            .columns[441]
                            .into_u32()
                            .copied()
                            .expect("Expected column 441 to be a uint32!"),
                        row
                            .columns[450]
                            .into_u32()
                            .copied()
                            .expect("Expected column 450 to be a uint32!"),
                        row
                            .columns[459]
                            .into_u32()
                            .copied()
                            .expect("Expected column 459 to be a uint32!"),
                        row
                            .columns[468]
                            .into_u32()
                            .copied()
                            .expect("Expected column 468 to be a uint32!"),
                        row
                            .columns[477]
                            .into_u32()
                            .copied()
                            .expect("Expected column 477 to be a uint32!"),
                        row
                            .columns[486]
                            .into_u32()
                            .copied()
                            .expect("Expected column 486 to be a uint32!"),
                        row
                            .columns[495]
                            .into_u32()
                            .copied()
                            .expect("Expected column 495 to be a uint32!"),
                        row
                            .columns[504]
                            .into_u32()
                            .copied()
                            .expect("Expected column 504 to be a uint32!"),
                        row
                            .columns[513]
                            .into_u32()
                            .copied()
                            .expect("Expected column 513 to be a uint32!"),
                        row
                            .columns[522]
                            .into_u32()
                            .copied()
                            .expect("Expected column 522 to be a uint32!"),
                        row
                            .columns[531]
                            .into_u32()
                            .copied()
                            .expect("Expected column 531 to be a uint32!"),
                        row
                            .columns[540]
                            .into_u32()
                            .copied()
                            .expect("Expected column 540 to be a uint32!"),
                        row
                            .columns[549]
                            .into_u32()
                            .copied()
                            .expect("Expected column 549 to be a uint32!"),
                        row
                            .columns[558]
                            .into_u32()
                            .copied()
                            .expect("Expected column 558 to be a uint32!"),
                        row
                            .columns[567]
                            .into_u32()
                            .copied()
                            .expect("Expected column 567 to be a uint32!"),
                        row
                            .columns[576]
                            .into_u32()
                            .copied()
                            .expect("Expected column 576 to be a uint32!"),
                        row
                            .columns[585]
                            .into_u32()
                            .copied()
                            .expect("Expected column 585 to be a uint32!"),
                        row
                            .columns[594]
                            .into_u32()
                            .copied()
                            .expect("Expected column 594 to be a uint32!"),
                        row
                            .columns[603]
                            .into_u32()
                            .copied()
                            .expect("Expected column 603 to be a uint32!"),
                        row
                            .columns[612]
                            .into_u32()
                            .copied()
                            .expect("Expected column 612 to be a uint32!"),
                        row
                            .columns[621]
                            .into_u32()
                            .copied()
                            .expect("Expected column 621 to be a uint32!"),
                        row
                            .columns[630]
                            .into_u32()
                            .copied()
                            .expect("Expected column 630 to be a uint32!"),
                        row
                            .columns[639]
                            .into_u32()
                            .copied()
                            .expect("Expected column 639 to be a uint32!"),
                        row
                            .columns[648]
                            .into_u32()
                            .copied()
                            .expect("Expected column 648 to be a uint32!"),
                        row
                            .columns[657]
                            .into_u32()
                            .copied()
                            .expect("Expected column 657 to be a uint32!"),
                        row
                            .columns[666]
                            .into_u32()
                            .copied()
                            .expect("Expected column 666 to be a uint32!"),
                        row
                            .columns[675]
                            .into_u32()
                            .copied()
                            .expect("Expected column 675 to be a uint32!"),
                        row
                            .columns[684]
                            .into_u32()
                            .copied()
                            .expect("Expected column 684 to be a uint32!"),
                        row
                            .columns[693]
                            .into_u32()
                            .copied()
                            .expect("Expected column 693 to be a uint32!"),
                        row
                            .columns[702]
                            .into_u32()
                            .copied()
                            .expect("Expected column 702 to be a uint32!"),
                        row
                            .columns[711]
                            .into_u32()
                            .copied()
                            .expect("Expected column 711 to be a uint32!"),
                        row
                            .columns[720]
                            .into_u32()
                            .copied()
                            .expect("Expected column 720 to be a uint32!"),
                        row
                            .columns[729]
                            .into_u32()
                            .copied()
                            .expect("Expected column 729 to be a uint32!"),
                        row
                            .columns[738]
                            .into_u32()
                            .copied()
                            .expect("Expected column 738 to be a uint32!"),
                        row
                            .columns[747]
                            .into_u32()
                            .copied()
                            .expect("Expected column 747 to be a uint32!"),
                        row
                            .columns[756]
                            .into_u32()
                            .copied()
                            .expect("Expected column 756 to be a uint32!"),
                        row
                            .columns[765]
                            .into_u32()
                            .copied()
                            .expect("Expected column 765 to be a uint32!"),
                        row
                            .columns[774]
                            .into_u32()
                            .copied()
                            .expect("Expected column 774 to be a uint32!"),
                        row
                            .columns[783]
                            .into_u32()
                            .copied()
                            .expect("Expected column 783 to be a uint32!"),
                        row
                            .columns[792]
                            .into_u32()
                            .copied()
                            .expect("Expected column 792 to be a uint32!"),
                        row
                            .columns[801]
                            .into_u32()
                            .copied()
                            .expect("Expected column 801 to be a uint32!"),
                        row
                            .columns[810]
                            .into_u32()
                            .copied()
                            .expect("Expected column 810 to be a uint32!"),
                        row
                            .columns[819]
                            .into_u32()
                            .copied()
                            .expect("Expected column 819 to be a uint32!"),
                        row
                            .columns[828]
                            .into_u32()
                            .copied()
                            .expect("Expected column 828 to be a uint32!"),
                        row
                            .columns[837]
                            .into_u32()
                            .copied()
                            .expect("Expected column 837 to be a uint32!"),
                        row
                            .columns[846]
                            .into_u32()
                            .copied()
                            .expect("Expected column 846 to be a uint32!"),
                        row
                            .columns[855]
                            .into_u32()
                            .copied()
                            .expect("Expected column 855 to be a uint32!"),
                        row
                            .columns[864]
                            .into_u32()
                            .copied()
                            .expect("Expected column 864 to be a uint32!"),
                        row
                            .columns[873]
                            .into_u32()
                            .copied()
                            .expect("Expected column 873 to be a uint32!"),
                        row
                            .columns[882]
                            .into_u32()
                            .copied()
                            .expect("Expected column 882 to be a uint32!"),
                        row
                            .columns[891]
                            .into_u32()
                            .copied()
                            .expect("Expected column 891 to be a uint32!"),
                        row
                            .columns[900]
                            .into_u32()
                            .copied()
                            .expect("Expected column 900 to be a uint32!"),
                        row
                            .columns[909]
                            .into_u32()
                            .copied()
                            .expect("Expected column 909 to be a uint32!"),
                        row
                            .columns[918]
                            .into_u32()
                            .copied()
                            .expect("Expected column 918 to be a uint32!"),
                        row
                            .columns[927]
                            .into_u32()
                            .copied()
                            .expect("Expected column 927 to be a uint32!"),
                        row
                            .columns[936]
                            .into_u32()
                            .copied()
                            .expect("Expected column 936 to be a uint32!"),
                        row
                            .columns[945]
                            .into_u32()
                            .copied()
                            .expect("Expected column 945 to be a uint32!"),
                        row
                            .columns[954]
                            .into_u32()
                            .copied()
                            .expect("Expected column 954 to be a uint32!"),
                        row
                            .columns[963]
                            .into_u32()
                            .copied()
                            .expect("Expected column 963 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[972]
                            .into_u32()
                            .copied()
                            .expect("Expected column 972 to be a uint32!"),
                        row
                            .columns[981]
                            .into_u32()
                            .copied()
                            .expect("Expected column 981 to be a uint32!"),
                        row
                            .columns[990]
                            .into_u32()
                            .copied()
                            .expect("Expected column 990 to be a uint32!"),
                        row
                            .columns[999]
                            .into_u32()
                            .copied()
                            .expect("Expected column 999 to be a uint32!"),
                        row
                            .columns[1008]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1008 to be a uint32!"),
                        row
                            .columns[1017]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1017 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[18]
                        .into_u8()
                        .copied()
                        .expect("Expected column 18 to be a uint8!"),
                    SubMenuType: row
                        .columns[27]
                        .into_u8()
                        .copied()
                        .expect("Expected column 27 to be a uint8!"),
                    SubMenuNum: row
                        .columns[36]
                        .into_u8()
                        .copied()
                        .expect("Expected column 36 to be a uint8!"),
                    LookAt: row
                        .columns[45]
                        .into_u8()
                        .copied()
                        .expect("Expected column 45 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1026]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1026 to be a uint8!"),
                        row
                            .columns[1035]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1035 to be a uint8!"),
                        row
                            .columns[1044]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1044 to be a uint8!"),
                        row
                            .columns[1053]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1053 to be a uint8!"),
                        row
                            .columns[1062]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1062 to be a uint8!"),
                        row
                            .columns[1071]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1071 to be a uint8!"),
                        row
                            .columns[1080]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1080 to be a uint8!"),
                        row
                            .columns[1089]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1089 to be a uint8!"),
                        row
                            .columns[1098]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1098 to be a uint8!"),
                        row
                            .columns[1107]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1107 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    SubMenuMask: row
                        .columns[55]
                        .into_u32()
                        .copied()
                        .expect("Expected column 55 to be a uint32!"),
                    Customize: row
                        .columns[64]
                        .into_u32()
                        .copied()
                        .expect("Expected column 64 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[73]
                            .into_u32()
                            .copied()
                            .expect("Expected column 73 to be a uint32!"),
                        row
                            .columns[82]
                            .into_u32()
                            .copied()
                            .expect("Expected column 82 to be a uint32!"),
                        row
                            .columns[91]
                            .into_u32()
                            .copied()
                            .expect("Expected column 91 to be a uint32!"),
                        row
                            .columns[100]
                            .into_u32()
                            .copied()
                            .expect("Expected column 100 to be a uint32!"),
                        row
                            .columns[109]
                            .into_u32()
                            .copied()
                            .expect("Expected column 109 to be a uint32!"),
                        row
                            .columns[118]
                            .into_u32()
                            .copied()
                            .expect("Expected column 118 to be a uint32!"),
                        row
                            .columns[127]
                            .into_u32()
                            .copied()
                            .expect("Expected column 127 to be a uint32!"),
                        row
                            .columns[136]
                            .into_u32()
                            .copied()
                            .expect("Expected column 136 to be a uint32!"),
                        row
                            .columns[145]
                            .into_u32()
                            .copied()
                            .expect("Expected column 145 to be a uint32!"),
                        row
                            .columns[154]
                            .into_u32()
                            .copied()
                            .expect("Expected column 154 to be a uint32!"),
                        row
                            .columns[163]
                            .into_u32()
                            .copied()
                            .expect("Expected column 163 to be a uint32!"),
                        row
                            .columns[172]
                            .into_u32()
                            .copied()
                            .expect("Expected column 172 to be a uint32!"),
                        row
                            .columns[181]
                            .into_u32()
                            .copied()
                            .expect("Expected column 181 to be a uint32!"),
                        row
                            .columns[190]
                            .into_u32()
                            .copied()
                            .expect("Expected column 190 to be a uint32!"),
                        row
                            .columns[199]
                            .into_u32()
                            .copied()
                            .expect("Expected column 199 to be a uint32!"),
                        row
                            .columns[208]
                            .into_u32()
                            .copied()
                            .expect("Expected column 208 to be a uint32!"),
                        row
                            .columns[217]
                            .into_u32()
                            .copied()
                            .expect("Expected column 217 to be a uint32!"),
                        row
                            .columns[226]
                            .into_u32()
                            .copied()
                            .expect("Expected column 226 to be a uint32!"),
                        row
                            .columns[235]
                            .into_u32()
                            .copied()
                            .expect("Expected column 235 to be a uint32!"),
                        row
                            .columns[244]
                            .into_u32()
                            .copied()
                            .expect("Expected column 244 to be a uint32!"),
                        row
                            .columns[253]
                            .into_u32()
                            .copied()
                            .expect("Expected column 253 to be a uint32!"),
                        row
                            .columns[262]
                            .into_u32()
                            .copied()
                            .expect("Expected column 262 to be a uint32!"),
                        row
                            .columns[271]
                            .into_u32()
                            .copied()
                            .expect("Expected column 271 to be a uint32!"),
                        row
                            .columns[280]
                            .into_u32()
                            .copied()
                            .expect("Expected column 280 to be a uint32!"),
                        row
                            .columns[289]
                            .into_u32()
                            .copied()
                            .expect("Expected column 289 to be a uint32!"),
                        row
                            .columns[298]
                            .into_u32()
                            .copied()
                            .expect("Expected column 298 to be a uint32!"),
                        row
                            .columns[307]
                            .into_u32()
                            .copied()
                            .expect("Expected column 307 to be a uint32!"),
                        row
                            .columns[316]
                            .into_u32()
                            .copied()
                            .expect("Expected column 316 to be a uint32!"),
                        row
                            .columns[325]
                            .into_u32()
                            .copied()
                            .expect("Expected column 325 to be a uint32!"),
                        row
                            .columns[334]
                            .into_u32()
                            .copied()
                            .expect("Expected column 334 to be a uint32!"),
                        row
                            .columns[343]
                            .into_u32()
                            .copied()
                            .expect("Expected column 343 to be a uint32!"),
                        row
                            .columns[352]
                            .into_u32()
                            .copied()
                            .expect("Expected column 352 to be a uint32!"),
                        row
                            .columns[361]
                            .into_u32()
                            .copied()
                            .expect("Expected column 361 to be a uint32!"),
                        row
                            .columns[370]
                            .into_u32()
                            .copied()
                            .expect("Expected column 370 to be a uint32!"),
                        row
                            .columns[379]
                            .into_u32()
                            .copied()
                            .expect("Expected column 379 to be a uint32!"),
                        row
                            .columns[388]
                            .into_u32()
                            .copied()
                            .expect("Expected column 388 to be a uint32!"),
                        row
                            .columns[397]
                            .into_u32()
                            .copied()
                            .expect("Expected column 397 to be a uint32!"),
                        row
                            .columns[406]
                            .into_u32()
                            .copied()
                            .expect("Expected column 406 to be a uint32!"),
                        row
                            .columns[415]
                            .into_u32()
                            .copied()
                            .expect("Expected column 415 to be a uint32!"),
                        row
                            .columns[424]
                            .into_u32()
                            .copied()
                            .expect("Expected column 424 to be a uint32!"),
                        row
                            .columns[433]
                            .into_u32()
                            .copied()
                            .expect("Expected column 433 to be a uint32!"),
                        row
                            .columns[442]
                            .into_u32()
                            .copied()
                            .expect("Expected column 442 to be a uint32!"),
                        row
                            .columns[451]
                            .into_u32()
                            .copied()
                            .expect("Expected column 451 to be a uint32!"),
                        row
                            .columns[460]
                            .into_u32()
                            .copied()
                            .expect("Expected column 460 to be a uint32!"),
                        row
                            .columns[469]
                            .into_u32()
                            .copied()
                            .expect("Expected column 469 to be a uint32!"),
                        row
                            .columns[478]
                            .into_u32()
                            .copied()
                            .expect("Expected column 478 to be a uint32!"),
                        row
                            .columns[487]
                            .into_u32()
                            .copied()
                            .expect("Expected column 487 to be a uint32!"),
                        row
                            .columns[496]
                            .into_u32()
                            .copied()
                            .expect("Expected column 496 to be a uint32!"),
                        row
                            .columns[505]
                            .into_u32()
                            .copied()
                            .expect("Expected column 505 to be a uint32!"),
                        row
                            .columns[514]
                            .into_u32()
                            .copied()
                            .expect("Expected column 514 to be a uint32!"),
                        row
                            .columns[523]
                            .into_u32()
                            .copied()
                            .expect("Expected column 523 to be a uint32!"),
                        row
                            .columns[532]
                            .into_u32()
                            .copied()
                            .expect("Expected column 532 to be a uint32!"),
                        row
                            .columns[541]
                            .into_u32()
                            .copied()
                            .expect("Expected column 541 to be a uint32!"),
                        row
                            .columns[550]
                            .into_u32()
                            .copied()
                            .expect("Expected column 550 to be a uint32!"),
                        row
                            .columns[559]
                            .into_u32()
                            .copied()
                            .expect("Expected column 559 to be a uint32!"),
                        row
                            .columns[568]
                            .into_u32()
                            .copied()
                            .expect("Expected column 568 to be a uint32!"),
                        row
                            .columns[577]
                            .into_u32()
                            .copied()
                            .expect("Expected column 577 to be a uint32!"),
                        row
                            .columns[586]
                            .into_u32()
                            .copied()
                            .expect("Expected column 586 to be a uint32!"),
                        row
                            .columns[595]
                            .into_u32()
                            .copied()
                            .expect("Expected column 595 to be a uint32!"),
                        row
                            .columns[604]
                            .into_u32()
                            .copied()
                            .expect("Expected column 604 to be a uint32!"),
                        row
                            .columns[613]
                            .into_u32()
                            .copied()
                            .expect("Expected column 613 to be a uint32!"),
                        row
                            .columns[622]
                            .into_u32()
                            .copied()
                            .expect("Expected column 622 to be a uint32!"),
                        row
                            .columns[631]
                            .into_u32()
                            .copied()
                            .expect("Expected column 631 to be a uint32!"),
                        row
                            .columns[640]
                            .into_u32()
                            .copied()
                            .expect("Expected column 640 to be a uint32!"),
                        row
                            .columns[649]
                            .into_u32()
                            .copied()
                            .expect("Expected column 649 to be a uint32!"),
                        row
                            .columns[658]
                            .into_u32()
                            .copied()
                            .expect("Expected column 658 to be a uint32!"),
                        row
                            .columns[667]
                            .into_u32()
                            .copied()
                            .expect("Expected column 667 to be a uint32!"),
                        row
                            .columns[676]
                            .into_u32()
                            .copied()
                            .expect("Expected column 676 to be a uint32!"),
                        row
                            .columns[685]
                            .into_u32()
                            .copied()
                            .expect("Expected column 685 to be a uint32!"),
                        row
                            .columns[694]
                            .into_u32()
                            .copied()
                            .expect("Expected column 694 to be a uint32!"),
                        row
                            .columns[703]
                            .into_u32()
                            .copied()
                            .expect("Expected column 703 to be a uint32!"),
                        row
                            .columns[712]
                            .into_u32()
                            .copied()
                            .expect("Expected column 712 to be a uint32!"),
                        row
                            .columns[721]
                            .into_u32()
                            .copied()
                            .expect("Expected column 721 to be a uint32!"),
                        row
                            .columns[730]
                            .into_u32()
                            .copied()
                            .expect("Expected column 730 to be a uint32!"),
                        row
                            .columns[739]
                            .into_u32()
                            .copied()
                            .expect("Expected column 739 to be a uint32!"),
                        row
                            .columns[748]
                            .into_u32()
                            .copied()
                            .expect("Expected column 748 to be a uint32!"),
                        row
                            .columns[757]
                            .into_u32()
                            .copied()
                            .expect("Expected column 757 to be a uint32!"),
                        row
                            .columns[766]
                            .into_u32()
                            .copied()
                            .expect("Expected column 766 to be a uint32!"),
                        row
                            .columns[775]
                            .into_u32()
                            .copied()
                            .expect("Expected column 775 to be a uint32!"),
                        row
                            .columns[784]
                            .into_u32()
                            .copied()
                            .expect("Expected column 784 to be a uint32!"),
                        row
                            .columns[793]
                            .into_u32()
                            .copied()
                            .expect("Expected column 793 to be a uint32!"),
                        row
                            .columns[802]
                            .into_u32()
                            .copied()
                            .expect("Expected column 802 to be a uint32!"),
                        row
                            .columns[811]
                            .into_u32()
                            .copied()
                            .expect("Expected column 811 to be a uint32!"),
                        row
                            .columns[820]
                            .into_u32()
                            .copied()
                            .expect("Expected column 820 to be a uint32!"),
                        row
                            .columns[829]
                            .into_u32()
                            .copied()
                            .expect("Expected column 829 to be a uint32!"),
                        row
                            .columns[838]
                            .into_u32()
                            .copied()
                            .expect("Expected column 838 to be a uint32!"),
                        row
                            .columns[847]
                            .into_u32()
                            .copied()
                            .expect("Expected column 847 to be a uint32!"),
                        row
                            .columns[856]
                            .into_u32()
                            .copied()
                            .expect("Expected column 856 to be a uint32!"),
                        row
                            .columns[865]
                            .into_u32()
                            .copied()
                            .expect("Expected column 865 to be a uint32!"),
                        row
                            .columns[874]
                            .into_u32()
                            .copied()
                            .expect("Expected column 874 to be a uint32!"),
                        row
                            .columns[883]
                            .into_u32()
                            .copied()
                            .expect("Expected column 883 to be a uint32!"),
                        row
                            .columns[892]
                            .into_u32()
                            .copied()
                            .expect("Expected column 892 to be a uint32!"),
                        row
                            .columns[901]
                            .into_u32()
                            .copied()
                            .expect("Expected column 901 to be a uint32!"),
                        row
                            .columns[910]
                            .into_u32()
                            .copied()
                            .expect("Expected column 910 to be a uint32!"),
                        row
                            .columns[919]
                            .into_u32()
                            .copied()
                            .expect("Expected column 919 to be a uint32!"),
                        row
                            .columns[928]
                            .into_u32()
                            .copied()
                            .expect("Expected column 928 to be a uint32!"),
                        row
                            .columns[937]
                            .into_u32()
                            .copied()
                            .expect("Expected column 937 to be a uint32!"),
                        row
                            .columns[946]
                            .into_u32()
                            .copied()
                            .expect("Expected column 946 to be a uint32!"),
                        row
                            .columns[955]
                            .into_u32()
                            .copied()
                            .expect("Expected column 955 to be a uint32!"),
                        row
                            .columns[964]
                            .into_u32()
                            .copied()
                            .expect("Expected column 964 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[973]
                            .into_u32()
                            .copied()
                            .expect("Expected column 973 to be a uint32!"),
                        row
                            .columns[982]
                            .into_u32()
                            .copied()
                            .expect("Expected column 982 to be a uint32!"),
                        row
                            .columns[991]
                            .into_u32()
                            .copied()
                            .expect("Expected column 991 to be a uint32!"),
                        row
                            .columns[1000]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1000 to be a uint32!"),
                        row
                            .columns[1009]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1009 to be a uint32!"),
                        row
                            .columns[1018]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1018 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[19]
                        .into_u8()
                        .copied()
                        .expect("Expected column 19 to be a uint8!"),
                    SubMenuType: row
                        .columns[28]
                        .into_u8()
                        .copied()
                        .expect("Expected column 28 to be a uint8!"),
                    SubMenuNum: row
                        .columns[37]
                        .into_u8()
                        .copied()
                        .expect("Expected column 37 to be a uint8!"),
                    LookAt: row
                        .columns[46]
                        .into_u8()
                        .copied()
                        .expect("Expected column 46 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1027]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1027 to be a uint8!"),
                        row
                            .columns[1036]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1036 to be a uint8!"),
                        row
                            .columns[1045]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1045 to be a uint8!"),
                        row
                            .columns[1054]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1054 to be a uint8!"),
                        row
                            .columns[1063]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1063 to be a uint8!"),
                        row
                            .columns[1072]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1072 to be a uint8!"),
                        row
                            .columns[1081]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1081 to be a uint8!"),
                        row
                            .columns[1090]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1090 to be a uint8!"),
                        row
                            .columns[1099]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1099 to be a uint8!"),
                        row
                            .columns[1108]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1108 to be a uint8!"),
                    ],
                },
                CharaMakeStructElement {
                    Menu: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    SubMenuMask: row
                        .columns[56]
                        .into_u32()
                        .copied()
                        .expect("Expected column 56 to be a uint32!"),
                    Customize: row
                        .columns[65]
                        .into_u32()
                        .copied()
                        .expect("Expected column 65 to be a uint32!"),
                    SubMenuParam: [
                        row
                            .columns[74]
                            .into_u32()
                            .copied()
                            .expect("Expected column 74 to be a uint32!"),
                        row
                            .columns[83]
                            .into_u32()
                            .copied()
                            .expect("Expected column 83 to be a uint32!"),
                        row
                            .columns[92]
                            .into_u32()
                            .copied()
                            .expect("Expected column 92 to be a uint32!"),
                        row
                            .columns[101]
                            .into_u32()
                            .copied()
                            .expect("Expected column 101 to be a uint32!"),
                        row
                            .columns[110]
                            .into_u32()
                            .copied()
                            .expect("Expected column 110 to be a uint32!"),
                        row
                            .columns[119]
                            .into_u32()
                            .copied()
                            .expect("Expected column 119 to be a uint32!"),
                        row
                            .columns[128]
                            .into_u32()
                            .copied()
                            .expect("Expected column 128 to be a uint32!"),
                        row
                            .columns[137]
                            .into_u32()
                            .copied()
                            .expect("Expected column 137 to be a uint32!"),
                        row
                            .columns[146]
                            .into_u32()
                            .copied()
                            .expect("Expected column 146 to be a uint32!"),
                        row
                            .columns[155]
                            .into_u32()
                            .copied()
                            .expect("Expected column 155 to be a uint32!"),
                        row
                            .columns[164]
                            .into_u32()
                            .copied()
                            .expect("Expected column 164 to be a uint32!"),
                        row
                            .columns[173]
                            .into_u32()
                            .copied()
                            .expect("Expected column 173 to be a uint32!"),
                        row
                            .columns[182]
                            .into_u32()
                            .copied()
                            .expect("Expected column 182 to be a uint32!"),
                        row
                            .columns[191]
                            .into_u32()
                            .copied()
                            .expect("Expected column 191 to be a uint32!"),
                        row
                            .columns[200]
                            .into_u32()
                            .copied()
                            .expect("Expected column 200 to be a uint32!"),
                        row
                            .columns[209]
                            .into_u32()
                            .copied()
                            .expect("Expected column 209 to be a uint32!"),
                        row
                            .columns[218]
                            .into_u32()
                            .copied()
                            .expect("Expected column 218 to be a uint32!"),
                        row
                            .columns[227]
                            .into_u32()
                            .copied()
                            .expect("Expected column 227 to be a uint32!"),
                        row
                            .columns[236]
                            .into_u32()
                            .copied()
                            .expect("Expected column 236 to be a uint32!"),
                        row
                            .columns[245]
                            .into_u32()
                            .copied()
                            .expect("Expected column 245 to be a uint32!"),
                        row
                            .columns[254]
                            .into_u32()
                            .copied()
                            .expect("Expected column 254 to be a uint32!"),
                        row
                            .columns[263]
                            .into_u32()
                            .copied()
                            .expect("Expected column 263 to be a uint32!"),
                        row
                            .columns[272]
                            .into_u32()
                            .copied()
                            .expect("Expected column 272 to be a uint32!"),
                        row
                            .columns[281]
                            .into_u32()
                            .copied()
                            .expect("Expected column 281 to be a uint32!"),
                        row
                            .columns[290]
                            .into_u32()
                            .copied()
                            .expect("Expected column 290 to be a uint32!"),
                        row
                            .columns[299]
                            .into_u32()
                            .copied()
                            .expect("Expected column 299 to be a uint32!"),
                        row
                            .columns[308]
                            .into_u32()
                            .copied()
                            .expect("Expected column 308 to be a uint32!"),
                        row
                            .columns[317]
                            .into_u32()
                            .copied()
                            .expect("Expected column 317 to be a uint32!"),
                        row
                            .columns[326]
                            .into_u32()
                            .copied()
                            .expect("Expected column 326 to be a uint32!"),
                        row
                            .columns[335]
                            .into_u32()
                            .copied()
                            .expect("Expected column 335 to be a uint32!"),
                        row
                            .columns[344]
                            .into_u32()
                            .copied()
                            .expect("Expected column 344 to be a uint32!"),
                        row
                            .columns[353]
                            .into_u32()
                            .copied()
                            .expect("Expected column 353 to be a uint32!"),
                        row
                            .columns[362]
                            .into_u32()
                            .copied()
                            .expect("Expected column 362 to be a uint32!"),
                        row
                            .columns[371]
                            .into_u32()
                            .copied()
                            .expect("Expected column 371 to be a uint32!"),
                        row
                            .columns[380]
                            .into_u32()
                            .copied()
                            .expect("Expected column 380 to be a uint32!"),
                        row
                            .columns[389]
                            .into_u32()
                            .copied()
                            .expect("Expected column 389 to be a uint32!"),
                        row
                            .columns[398]
                            .into_u32()
                            .copied()
                            .expect("Expected column 398 to be a uint32!"),
                        row
                            .columns[407]
                            .into_u32()
                            .copied()
                            .expect("Expected column 407 to be a uint32!"),
                        row
                            .columns[416]
                            .into_u32()
                            .copied()
                            .expect("Expected column 416 to be a uint32!"),
                        row
                            .columns[425]
                            .into_u32()
                            .copied()
                            .expect("Expected column 425 to be a uint32!"),
                        row
                            .columns[434]
                            .into_u32()
                            .copied()
                            .expect("Expected column 434 to be a uint32!"),
                        row
                            .columns[443]
                            .into_u32()
                            .copied()
                            .expect("Expected column 443 to be a uint32!"),
                        row
                            .columns[452]
                            .into_u32()
                            .copied()
                            .expect("Expected column 452 to be a uint32!"),
                        row
                            .columns[461]
                            .into_u32()
                            .copied()
                            .expect("Expected column 461 to be a uint32!"),
                        row
                            .columns[470]
                            .into_u32()
                            .copied()
                            .expect("Expected column 470 to be a uint32!"),
                        row
                            .columns[479]
                            .into_u32()
                            .copied()
                            .expect("Expected column 479 to be a uint32!"),
                        row
                            .columns[488]
                            .into_u32()
                            .copied()
                            .expect("Expected column 488 to be a uint32!"),
                        row
                            .columns[497]
                            .into_u32()
                            .copied()
                            .expect("Expected column 497 to be a uint32!"),
                        row
                            .columns[506]
                            .into_u32()
                            .copied()
                            .expect("Expected column 506 to be a uint32!"),
                        row
                            .columns[515]
                            .into_u32()
                            .copied()
                            .expect("Expected column 515 to be a uint32!"),
                        row
                            .columns[524]
                            .into_u32()
                            .copied()
                            .expect("Expected column 524 to be a uint32!"),
                        row
                            .columns[533]
                            .into_u32()
                            .copied()
                            .expect("Expected column 533 to be a uint32!"),
                        row
                            .columns[542]
                            .into_u32()
                            .copied()
                            .expect("Expected column 542 to be a uint32!"),
                        row
                            .columns[551]
                            .into_u32()
                            .copied()
                            .expect("Expected column 551 to be a uint32!"),
                        row
                            .columns[560]
                            .into_u32()
                            .copied()
                            .expect("Expected column 560 to be a uint32!"),
                        row
                            .columns[569]
                            .into_u32()
                            .copied()
                            .expect("Expected column 569 to be a uint32!"),
                        row
                            .columns[578]
                            .into_u32()
                            .copied()
                            .expect("Expected column 578 to be a uint32!"),
                        row
                            .columns[587]
                            .into_u32()
                            .copied()
                            .expect("Expected column 587 to be a uint32!"),
                        row
                            .columns[596]
                            .into_u32()
                            .copied()
                            .expect("Expected column 596 to be a uint32!"),
                        row
                            .columns[605]
                            .into_u32()
                            .copied()
                            .expect("Expected column 605 to be a uint32!"),
                        row
                            .columns[614]
                            .into_u32()
                            .copied()
                            .expect("Expected column 614 to be a uint32!"),
                        row
                            .columns[623]
                            .into_u32()
                            .copied()
                            .expect("Expected column 623 to be a uint32!"),
                        row
                            .columns[632]
                            .into_u32()
                            .copied()
                            .expect("Expected column 632 to be a uint32!"),
                        row
                            .columns[641]
                            .into_u32()
                            .copied()
                            .expect("Expected column 641 to be a uint32!"),
                        row
                            .columns[650]
                            .into_u32()
                            .copied()
                            .expect("Expected column 650 to be a uint32!"),
                        row
                            .columns[659]
                            .into_u32()
                            .copied()
                            .expect("Expected column 659 to be a uint32!"),
                        row
                            .columns[668]
                            .into_u32()
                            .copied()
                            .expect("Expected column 668 to be a uint32!"),
                        row
                            .columns[677]
                            .into_u32()
                            .copied()
                            .expect("Expected column 677 to be a uint32!"),
                        row
                            .columns[686]
                            .into_u32()
                            .copied()
                            .expect("Expected column 686 to be a uint32!"),
                        row
                            .columns[695]
                            .into_u32()
                            .copied()
                            .expect("Expected column 695 to be a uint32!"),
                        row
                            .columns[704]
                            .into_u32()
                            .copied()
                            .expect("Expected column 704 to be a uint32!"),
                        row
                            .columns[713]
                            .into_u32()
                            .copied()
                            .expect("Expected column 713 to be a uint32!"),
                        row
                            .columns[722]
                            .into_u32()
                            .copied()
                            .expect("Expected column 722 to be a uint32!"),
                        row
                            .columns[731]
                            .into_u32()
                            .copied()
                            .expect("Expected column 731 to be a uint32!"),
                        row
                            .columns[740]
                            .into_u32()
                            .copied()
                            .expect("Expected column 740 to be a uint32!"),
                        row
                            .columns[749]
                            .into_u32()
                            .copied()
                            .expect("Expected column 749 to be a uint32!"),
                        row
                            .columns[758]
                            .into_u32()
                            .copied()
                            .expect("Expected column 758 to be a uint32!"),
                        row
                            .columns[767]
                            .into_u32()
                            .copied()
                            .expect("Expected column 767 to be a uint32!"),
                        row
                            .columns[776]
                            .into_u32()
                            .copied()
                            .expect("Expected column 776 to be a uint32!"),
                        row
                            .columns[785]
                            .into_u32()
                            .copied()
                            .expect("Expected column 785 to be a uint32!"),
                        row
                            .columns[794]
                            .into_u32()
                            .copied()
                            .expect("Expected column 794 to be a uint32!"),
                        row
                            .columns[803]
                            .into_u32()
                            .copied()
                            .expect("Expected column 803 to be a uint32!"),
                        row
                            .columns[812]
                            .into_u32()
                            .copied()
                            .expect("Expected column 812 to be a uint32!"),
                        row
                            .columns[821]
                            .into_u32()
                            .copied()
                            .expect("Expected column 821 to be a uint32!"),
                        row
                            .columns[830]
                            .into_u32()
                            .copied()
                            .expect("Expected column 830 to be a uint32!"),
                        row
                            .columns[839]
                            .into_u32()
                            .copied()
                            .expect("Expected column 839 to be a uint32!"),
                        row
                            .columns[848]
                            .into_u32()
                            .copied()
                            .expect("Expected column 848 to be a uint32!"),
                        row
                            .columns[857]
                            .into_u32()
                            .copied()
                            .expect("Expected column 857 to be a uint32!"),
                        row
                            .columns[866]
                            .into_u32()
                            .copied()
                            .expect("Expected column 866 to be a uint32!"),
                        row
                            .columns[875]
                            .into_u32()
                            .copied()
                            .expect("Expected column 875 to be a uint32!"),
                        row
                            .columns[884]
                            .into_u32()
                            .copied()
                            .expect("Expected column 884 to be a uint32!"),
                        row
                            .columns[893]
                            .into_u32()
                            .copied()
                            .expect("Expected column 893 to be a uint32!"),
                        row
                            .columns[902]
                            .into_u32()
                            .copied()
                            .expect("Expected column 902 to be a uint32!"),
                        row
                            .columns[911]
                            .into_u32()
                            .copied()
                            .expect("Expected column 911 to be a uint32!"),
                        row
                            .columns[920]
                            .into_u32()
                            .copied()
                            .expect("Expected column 920 to be a uint32!"),
                        row
                            .columns[929]
                            .into_u32()
                            .copied()
                            .expect("Expected column 929 to be a uint32!"),
                        row
                            .columns[938]
                            .into_u32()
                            .copied()
                            .expect("Expected column 938 to be a uint32!"),
                        row
                            .columns[947]
                            .into_u32()
                            .copied()
                            .expect("Expected column 947 to be a uint32!"),
                        row
                            .columns[956]
                            .into_u32()
                            .copied()
                            .expect("Expected column 956 to be a uint32!"),
                        row
                            .columns[965]
                            .into_u32()
                            .copied()
                            .expect("Expected column 965 to be a uint32!"),
                    ],
                    Unknown0: [
                        row
                            .columns[974]
                            .into_u32()
                            .copied()
                            .expect("Expected column 974 to be a uint32!"),
                        row
                            .columns[983]
                            .into_u32()
                            .copied()
                            .expect("Expected column 983 to be a uint32!"),
                        row
                            .columns[992]
                            .into_u32()
                            .copied()
                            .expect("Expected column 992 to be a uint32!"),
                        row
                            .columns[1001]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1001 to be a uint32!"),
                        row
                            .columns[1010]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1010 to be a uint32!"),
                        row
                            .columns[1019]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1019 to be a uint32!"),
                    ],
                    InitVal: row
                        .columns[20]
                        .into_u8()
                        .copied()
                        .expect("Expected column 20 to be a uint8!"),
                    SubMenuType: row
                        .columns[29]
                        .into_u8()
                        .copied()
                        .expect("Expected column 29 to be a uint8!"),
                    SubMenuNum: row
                        .columns[38]
                        .into_u8()
                        .copied()
                        .expect("Expected column 38 to be a uint8!"),
                    LookAt: row
                        .columns[47]
                        .into_u8()
                        .copied()
                        .expect("Expected column 47 to be a uint8!"),
                    SubMenuGraphic: [
                        row
                            .columns[1028]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1028 to be a uint8!"),
                        row
                            .columns[1037]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1037 to be a uint8!"),
                        row
                            .columns[1046]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1046 to be a uint8!"),
                        row
                            .columns[1055]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1055 to be a uint8!"),
                        row
                            .columns[1064]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1064 to be a uint8!"),
                        row
                            .columns[1073]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1073 to be a uint8!"),
                        row
                            .columns[1082]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1082 to be a uint8!"),
                        row
                            .columns[1091]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1091 to be a uint8!"),
                        row
                            .columns[1100]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1100 to be a uint8!"),
                        row
                            .columns[1109]
                            .into_u8()
                            .copied()
                            .expect("Expected column 1109 to be a uint8!"),
                    ],
                },
            ],
            FacialFeatureOption: [
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1110]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1110 to be a int32!"),
                    Option2: row
                        .columns[1118]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1118 to be a int32!"),
                    Option3: row
                        .columns[1126]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1126 to be a int32!"),
                    Option4: row
                        .columns[1134]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1134 to be a int32!"),
                    Option5: row
                        .columns[1142]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1142 to be a int32!"),
                    Option6: row
                        .columns[1150]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1150 to be a int32!"),
                    Option7: row
                        .columns[1158]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1158 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1111]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1111 to be a int32!"),
                    Option2: row
                        .columns[1119]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1119 to be a int32!"),
                    Option3: row
                        .columns[1127]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1127 to be a int32!"),
                    Option4: row
                        .columns[1135]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1135 to be a int32!"),
                    Option5: row
                        .columns[1143]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1143 to be a int32!"),
                    Option6: row
                        .columns[1151]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1151 to be a int32!"),
                    Option7: row
                        .columns[1159]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1159 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1112]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1112 to be a int32!"),
                    Option2: row
                        .columns[1120]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1120 to be a int32!"),
                    Option3: row
                        .columns[1128]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1128 to be a int32!"),
                    Option4: row
                        .columns[1136]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1136 to be a int32!"),
                    Option5: row
                        .columns[1144]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1144 to be a int32!"),
                    Option6: row
                        .columns[1152]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1152 to be a int32!"),
                    Option7: row
                        .columns[1160]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1160 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1113]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1113 to be a int32!"),
                    Option2: row
                        .columns[1121]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1121 to be a int32!"),
                    Option3: row
                        .columns[1129]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1129 to be a int32!"),
                    Option4: row
                        .columns[1137]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1137 to be a int32!"),
                    Option5: row
                        .columns[1145]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1145 to be a int32!"),
                    Option6: row
                        .columns[1153]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1153 to be a int32!"),
                    Option7: row
                        .columns[1161]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1161 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1114]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1114 to be a int32!"),
                    Option2: row
                        .columns[1122]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1122 to be a int32!"),
                    Option3: row
                        .columns[1130]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1130 to be a int32!"),
                    Option4: row
                        .columns[1138]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1138 to be a int32!"),
                    Option5: row
                        .columns[1146]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1146 to be a int32!"),
                    Option6: row
                        .columns[1154]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1154 to be a int32!"),
                    Option7: row
                        .columns[1162]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1162 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1115]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1115 to be a int32!"),
                    Option2: row
                        .columns[1123]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1123 to be a int32!"),
                    Option3: row
                        .columns[1131]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1131 to be a int32!"),
                    Option4: row
                        .columns[1139]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1139 to be a int32!"),
                    Option5: row
                        .columns[1147]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1147 to be a int32!"),
                    Option6: row
                        .columns[1155]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1155 to be a int32!"),
                    Option7: row
                        .columns[1163]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1163 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1116]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1116 to be a int32!"),
                    Option2: row
                        .columns[1124]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1124 to be a int32!"),
                    Option3: row
                        .columns[1132]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1132 to be a int32!"),
                    Option4: row
                        .columns[1140]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1140 to be a int32!"),
                    Option5: row
                        .columns[1148]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1148 to be a int32!"),
                    Option6: row
                        .columns[1156]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1156 to be a int32!"),
                    Option7: row
                        .columns[1164]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1164 to be a int32!"),
                },
                FacialFeatureOptionElement {
                    Option1: row
                        .columns[1117]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1117 to be a int32!"),
                    Option2: row
                        .columns[1125]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1125 to be a int32!"),
                    Option3: row
                        .columns[1133]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1133 to be a int32!"),
                    Option4: row
                        .columns[1141]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1141 to be a int32!"),
                    Option5: row
                        .columns[1149]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1149 to be a int32!"),
                    Option6: row
                        .columns[1157]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1157 to be a int32!"),
                    Option7: row
                        .columns[1165]
                        .into_i32()
                        .copied()
                        .expect("Expected column 1165 to be a int32!"),
                },
            ],
            Race: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            Tribe: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            Gender: row
                .columns[2]
                .into_i8()
                .copied()
                .expect("Expected column 2 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a HairMakeTypeSheet {
    type Item = (u32, Vec<(u16, HairMakeTypeRow)>);
    type IntoIter = StructuredSheetIterator<'a, HairMakeTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HairMakeTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HairMakeTypeRow {
    ///""
    pub CharaMakeStruct: [CharaMakeStructElement; 9],
    ///""
    pub FacialFeatureOption: [FacialFeatureOptionElement; 8],
    ///""
    pub Race: i32,
    ///""
    pub Tribe: i32,
    ///""
    pub Gender: i8,
}
