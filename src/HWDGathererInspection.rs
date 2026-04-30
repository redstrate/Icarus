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
pub struct HWDGathererInspectionDataElement {
    pub RequiredItem: u32,
    pub FishParameter: u32,
    pub ItemReceived: u32,
    pub Reward: [u16; 2],
    pub AmountRequired: u8,
    pub Phase: u8,
}
#[derive(Debug, Clone)]
pub struct HWDGathererInspectionSheet {
    sheet: Sheet,
}
impl HWDGathererInspectionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HWDGathererInspection")?;
        let sheet = resolver.read_excel_sheet(&exh, "HWDGathererInspection", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HWDGathererInspectionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<HWDGathererInspectionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for HWDGathererInspectionSheet {
    type Row = HWDGathererInspectionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            HWDGathererInspectionData: [
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[0]
                        .into_u32()
                        .copied()
                        .expect("Expected column 0 to be a uint32!"),
                    FishParameter: row
                        .columns[79]
                        .into_u32()
                        .copied()
                        .expect("Expected column 79 to be a uint32!"),
                    ItemReceived: row
                        .columns[237]
                        .into_u32()
                        .copied()
                        .expect("Expected column 237 to be a uint32!"),
                    Reward: [
                        row
                            .columns[316]
                            .into_u16()
                            .copied()
                            .expect("Expected column 316 to be a uint16!"),
                        row
                            .columns[395]
                            .into_u16()
                            .copied()
                            .expect("Expected column 395 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[158]
                        .into_u8()
                        .copied()
                        .expect("Expected column 158 to be a uint8!"),
                    Phase: row
                        .columns[474]
                        .into_u8()
                        .copied()
                        .expect("Expected column 474 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[1]
                        .into_u32()
                        .copied()
                        .expect("Expected column 1 to be a uint32!"),
                    FishParameter: row
                        .columns[80]
                        .into_u32()
                        .copied()
                        .expect("Expected column 80 to be a uint32!"),
                    ItemReceived: row
                        .columns[238]
                        .into_u32()
                        .copied()
                        .expect("Expected column 238 to be a uint32!"),
                    Reward: [
                        row
                            .columns[317]
                            .into_u16()
                            .copied()
                            .expect("Expected column 317 to be a uint16!"),
                        row
                            .columns[396]
                            .into_u16()
                            .copied()
                            .expect("Expected column 396 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[159]
                        .into_u8()
                        .copied()
                        .expect("Expected column 159 to be a uint8!"),
                    Phase: row
                        .columns[475]
                        .into_u8()
                        .copied()
                        .expect("Expected column 475 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[2]
                        .into_u32()
                        .copied()
                        .expect("Expected column 2 to be a uint32!"),
                    FishParameter: row
                        .columns[81]
                        .into_u32()
                        .copied()
                        .expect("Expected column 81 to be a uint32!"),
                    ItemReceived: row
                        .columns[239]
                        .into_u32()
                        .copied()
                        .expect("Expected column 239 to be a uint32!"),
                    Reward: [
                        row
                            .columns[318]
                            .into_u16()
                            .copied()
                            .expect("Expected column 318 to be a uint16!"),
                        row
                            .columns[397]
                            .into_u16()
                            .copied()
                            .expect("Expected column 397 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[160]
                        .into_u8()
                        .copied()
                        .expect("Expected column 160 to be a uint8!"),
                    Phase: row
                        .columns[476]
                        .into_u8()
                        .copied()
                        .expect("Expected column 476 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[3]
                        .into_u32()
                        .copied()
                        .expect("Expected column 3 to be a uint32!"),
                    FishParameter: row
                        .columns[82]
                        .into_u32()
                        .copied()
                        .expect("Expected column 82 to be a uint32!"),
                    ItemReceived: row
                        .columns[240]
                        .into_u32()
                        .copied()
                        .expect("Expected column 240 to be a uint32!"),
                    Reward: [
                        row
                            .columns[319]
                            .into_u16()
                            .copied()
                            .expect("Expected column 319 to be a uint16!"),
                        row
                            .columns[398]
                            .into_u16()
                            .copied()
                            .expect("Expected column 398 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[161]
                        .into_u8()
                        .copied()
                        .expect("Expected column 161 to be a uint8!"),
                    Phase: row
                        .columns[477]
                        .into_u8()
                        .copied()
                        .expect("Expected column 477 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    FishParameter: row
                        .columns[83]
                        .into_u32()
                        .copied()
                        .expect("Expected column 83 to be a uint32!"),
                    ItemReceived: row
                        .columns[241]
                        .into_u32()
                        .copied()
                        .expect("Expected column 241 to be a uint32!"),
                    Reward: [
                        row
                            .columns[320]
                            .into_u16()
                            .copied()
                            .expect("Expected column 320 to be a uint16!"),
                        row
                            .columns[399]
                            .into_u16()
                            .copied()
                            .expect("Expected column 399 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[162]
                        .into_u8()
                        .copied()
                        .expect("Expected column 162 to be a uint8!"),
                    Phase: row
                        .columns[478]
                        .into_u8()
                        .copied()
                        .expect("Expected column 478 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    FishParameter: row
                        .columns[84]
                        .into_u32()
                        .copied()
                        .expect("Expected column 84 to be a uint32!"),
                    ItemReceived: row
                        .columns[242]
                        .into_u32()
                        .copied()
                        .expect("Expected column 242 to be a uint32!"),
                    Reward: [
                        row
                            .columns[321]
                            .into_u16()
                            .copied()
                            .expect("Expected column 321 to be a uint16!"),
                        row
                            .columns[400]
                            .into_u16()
                            .copied()
                            .expect("Expected column 400 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[163]
                        .into_u8()
                        .copied()
                        .expect("Expected column 163 to be a uint8!"),
                    Phase: row
                        .columns[479]
                        .into_u8()
                        .copied()
                        .expect("Expected column 479 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    FishParameter: row
                        .columns[85]
                        .into_u32()
                        .copied()
                        .expect("Expected column 85 to be a uint32!"),
                    ItemReceived: row
                        .columns[243]
                        .into_u32()
                        .copied()
                        .expect("Expected column 243 to be a uint32!"),
                    Reward: [
                        row
                            .columns[322]
                            .into_u16()
                            .copied()
                            .expect("Expected column 322 to be a uint16!"),
                        row
                            .columns[401]
                            .into_u16()
                            .copied()
                            .expect("Expected column 401 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[164]
                        .into_u8()
                        .copied()
                        .expect("Expected column 164 to be a uint8!"),
                    Phase: row
                        .columns[480]
                        .into_u8()
                        .copied()
                        .expect("Expected column 480 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    FishParameter: row
                        .columns[86]
                        .into_u32()
                        .copied()
                        .expect("Expected column 86 to be a uint32!"),
                    ItemReceived: row
                        .columns[244]
                        .into_u32()
                        .copied()
                        .expect("Expected column 244 to be a uint32!"),
                    Reward: [
                        row
                            .columns[323]
                            .into_u16()
                            .copied()
                            .expect("Expected column 323 to be a uint16!"),
                        row
                            .columns[402]
                            .into_u16()
                            .copied()
                            .expect("Expected column 402 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[165]
                        .into_u8()
                        .copied()
                        .expect("Expected column 165 to be a uint8!"),
                    Phase: row
                        .columns[481]
                        .into_u8()
                        .copied()
                        .expect("Expected column 481 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    FishParameter: row
                        .columns[87]
                        .into_u32()
                        .copied()
                        .expect("Expected column 87 to be a uint32!"),
                    ItemReceived: row
                        .columns[245]
                        .into_u32()
                        .copied()
                        .expect("Expected column 245 to be a uint32!"),
                    Reward: [
                        row
                            .columns[324]
                            .into_u16()
                            .copied()
                            .expect("Expected column 324 to be a uint16!"),
                        row
                            .columns[403]
                            .into_u16()
                            .copied()
                            .expect("Expected column 403 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[166]
                        .into_u8()
                        .copied()
                        .expect("Expected column 166 to be a uint8!"),
                    Phase: row
                        .columns[482]
                        .into_u8()
                        .copied()
                        .expect("Expected column 482 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    FishParameter: row
                        .columns[88]
                        .into_u32()
                        .copied()
                        .expect("Expected column 88 to be a uint32!"),
                    ItemReceived: row
                        .columns[246]
                        .into_u32()
                        .copied()
                        .expect("Expected column 246 to be a uint32!"),
                    Reward: [
                        row
                            .columns[325]
                            .into_u16()
                            .copied()
                            .expect("Expected column 325 to be a uint16!"),
                        row
                            .columns[404]
                            .into_u16()
                            .copied()
                            .expect("Expected column 404 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[167]
                        .into_u8()
                        .copied()
                        .expect("Expected column 167 to be a uint8!"),
                    Phase: row
                        .columns[483]
                        .into_u8()
                        .copied()
                        .expect("Expected column 483 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    FishParameter: row
                        .columns[89]
                        .into_u32()
                        .copied()
                        .expect("Expected column 89 to be a uint32!"),
                    ItemReceived: row
                        .columns[247]
                        .into_u32()
                        .copied()
                        .expect("Expected column 247 to be a uint32!"),
                    Reward: [
                        row
                            .columns[326]
                            .into_u16()
                            .copied()
                            .expect("Expected column 326 to be a uint16!"),
                        row
                            .columns[405]
                            .into_u16()
                            .copied()
                            .expect("Expected column 405 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[168]
                        .into_u8()
                        .copied()
                        .expect("Expected column 168 to be a uint8!"),
                    Phase: row
                        .columns[484]
                        .into_u8()
                        .copied()
                        .expect("Expected column 484 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    FishParameter: row
                        .columns[90]
                        .into_u32()
                        .copied()
                        .expect("Expected column 90 to be a uint32!"),
                    ItemReceived: row
                        .columns[248]
                        .into_u32()
                        .copied()
                        .expect("Expected column 248 to be a uint32!"),
                    Reward: [
                        row
                            .columns[327]
                            .into_u16()
                            .copied()
                            .expect("Expected column 327 to be a uint16!"),
                        row
                            .columns[406]
                            .into_u16()
                            .copied()
                            .expect("Expected column 406 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[169]
                        .into_u8()
                        .copied()
                        .expect("Expected column 169 to be a uint8!"),
                    Phase: row
                        .columns[485]
                        .into_u8()
                        .copied()
                        .expect("Expected column 485 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[12]
                        .into_u32()
                        .copied()
                        .expect("Expected column 12 to be a uint32!"),
                    FishParameter: row
                        .columns[91]
                        .into_u32()
                        .copied()
                        .expect("Expected column 91 to be a uint32!"),
                    ItemReceived: row
                        .columns[249]
                        .into_u32()
                        .copied()
                        .expect("Expected column 249 to be a uint32!"),
                    Reward: [
                        row
                            .columns[328]
                            .into_u16()
                            .copied()
                            .expect("Expected column 328 to be a uint16!"),
                        row
                            .columns[407]
                            .into_u16()
                            .copied()
                            .expect("Expected column 407 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[170]
                        .into_u8()
                        .copied()
                        .expect("Expected column 170 to be a uint8!"),
                    Phase: row
                        .columns[486]
                        .into_u8()
                        .copied()
                        .expect("Expected column 486 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[13]
                        .into_u32()
                        .copied()
                        .expect("Expected column 13 to be a uint32!"),
                    FishParameter: row
                        .columns[92]
                        .into_u32()
                        .copied()
                        .expect("Expected column 92 to be a uint32!"),
                    ItemReceived: row
                        .columns[250]
                        .into_u32()
                        .copied()
                        .expect("Expected column 250 to be a uint32!"),
                    Reward: [
                        row
                            .columns[329]
                            .into_u16()
                            .copied()
                            .expect("Expected column 329 to be a uint16!"),
                        row
                            .columns[408]
                            .into_u16()
                            .copied()
                            .expect("Expected column 408 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[171]
                        .into_u8()
                        .copied()
                        .expect("Expected column 171 to be a uint8!"),
                    Phase: row
                        .columns[487]
                        .into_u8()
                        .copied()
                        .expect("Expected column 487 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[14]
                        .into_u32()
                        .copied()
                        .expect("Expected column 14 to be a uint32!"),
                    FishParameter: row
                        .columns[93]
                        .into_u32()
                        .copied()
                        .expect("Expected column 93 to be a uint32!"),
                    ItemReceived: row
                        .columns[251]
                        .into_u32()
                        .copied()
                        .expect("Expected column 251 to be a uint32!"),
                    Reward: [
                        row
                            .columns[330]
                            .into_u16()
                            .copied()
                            .expect("Expected column 330 to be a uint16!"),
                        row
                            .columns[409]
                            .into_u16()
                            .copied()
                            .expect("Expected column 409 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[172]
                        .into_u8()
                        .copied()
                        .expect("Expected column 172 to be a uint8!"),
                    Phase: row
                        .columns[488]
                        .into_u8()
                        .copied()
                        .expect("Expected column 488 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[15]
                        .into_u32()
                        .copied()
                        .expect("Expected column 15 to be a uint32!"),
                    FishParameter: row
                        .columns[94]
                        .into_u32()
                        .copied()
                        .expect("Expected column 94 to be a uint32!"),
                    ItemReceived: row
                        .columns[252]
                        .into_u32()
                        .copied()
                        .expect("Expected column 252 to be a uint32!"),
                    Reward: [
                        row
                            .columns[331]
                            .into_u16()
                            .copied()
                            .expect("Expected column 331 to be a uint16!"),
                        row
                            .columns[410]
                            .into_u16()
                            .copied()
                            .expect("Expected column 410 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[173]
                        .into_u8()
                        .copied()
                        .expect("Expected column 173 to be a uint8!"),
                    Phase: row
                        .columns[489]
                        .into_u8()
                        .copied()
                        .expect("Expected column 489 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                    FishParameter: row
                        .columns[95]
                        .into_u32()
                        .copied()
                        .expect("Expected column 95 to be a uint32!"),
                    ItemReceived: row
                        .columns[253]
                        .into_u32()
                        .copied()
                        .expect("Expected column 253 to be a uint32!"),
                    Reward: [
                        row
                            .columns[332]
                            .into_u16()
                            .copied()
                            .expect("Expected column 332 to be a uint16!"),
                        row
                            .columns[411]
                            .into_u16()
                            .copied()
                            .expect("Expected column 411 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[174]
                        .into_u8()
                        .copied()
                        .expect("Expected column 174 to be a uint8!"),
                    Phase: row
                        .columns[490]
                        .into_u8()
                        .copied()
                        .expect("Expected column 490 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    FishParameter: row
                        .columns[96]
                        .into_u32()
                        .copied()
                        .expect("Expected column 96 to be a uint32!"),
                    ItemReceived: row
                        .columns[254]
                        .into_u32()
                        .copied()
                        .expect("Expected column 254 to be a uint32!"),
                    Reward: [
                        row
                            .columns[333]
                            .into_u16()
                            .copied()
                            .expect("Expected column 333 to be a uint16!"),
                        row
                            .columns[412]
                            .into_u16()
                            .copied()
                            .expect("Expected column 412 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[175]
                        .into_u8()
                        .copied()
                        .expect("Expected column 175 to be a uint8!"),
                    Phase: row
                        .columns[491]
                        .into_u8()
                        .copied()
                        .expect("Expected column 491 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    FishParameter: row
                        .columns[97]
                        .into_u32()
                        .copied()
                        .expect("Expected column 97 to be a uint32!"),
                    ItemReceived: row
                        .columns[255]
                        .into_u32()
                        .copied()
                        .expect("Expected column 255 to be a uint32!"),
                    Reward: [
                        row
                            .columns[334]
                            .into_u16()
                            .copied()
                            .expect("Expected column 334 to be a uint16!"),
                        row
                            .columns[413]
                            .into_u16()
                            .copied()
                            .expect("Expected column 413 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[176]
                        .into_u8()
                        .copied()
                        .expect("Expected column 176 to be a uint8!"),
                    Phase: row
                        .columns[492]
                        .into_u8()
                        .copied()
                        .expect("Expected column 492 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    FishParameter: row
                        .columns[98]
                        .into_u32()
                        .copied()
                        .expect("Expected column 98 to be a uint32!"),
                    ItemReceived: row
                        .columns[256]
                        .into_u32()
                        .copied()
                        .expect("Expected column 256 to be a uint32!"),
                    Reward: [
                        row
                            .columns[335]
                            .into_u16()
                            .copied()
                            .expect("Expected column 335 to be a uint16!"),
                        row
                            .columns[414]
                            .into_u16()
                            .copied()
                            .expect("Expected column 414 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[177]
                        .into_u8()
                        .copied()
                        .expect("Expected column 177 to be a uint8!"),
                    Phase: row
                        .columns[493]
                        .into_u8()
                        .copied()
                        .expect("Expected column 493 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                    FishParameter: row
                        .columns[99]
                        .into_u32()
                        .copied()
                        .expect("Expected column 99 to be a uint32!"),
                    ItemReceived: row
                        .columns[257]
                        .into_u32()
                        .copied()
                        .expect("Expected column 257 to be a uint32!"),
                    Reward: [
                        row
                            .columns[336]
                            .into_u16()
                            .copied()
                            .expect("Expected column 336 to be a uint16!"),
                        row
                            .columns[415]
                            .into_u16()
                            .copied()
                            .expect("Expected column 415 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[178]
                        .into_u8()
                        .copied()
                        .expect("Expected column 178 to be a uint8!"),
                    Phase: row
                        .columns[494]
                        .into_u8()
                        .copied()
                        .expect("Expected column 494 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[21]
                        .into_u32()
                        .copied()
                        .expect("Expected column 21 to be a uint32!"),
                    FishParameter: row
                        .columns[100]
                        .into_u32()
                        .copied()
                        .expect("Expected column 100 to be a uint32!"),
                    ItemReceived: row
                        .columns[258]
                        .into_u32()
                        .copied()
                        .expect("Expected column 258 to be a uint32!"),
                    Reward: [
                        row
                            .columns[337]
                            .into_u16()
                            .copied()
                            .expect("Expected column 337 to be a uint16!"),
                        row
                            .columns[416]
                            .into_u16()
                            .copied()
                            .expect("Expected column 416 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[179]
                        .into_u8()
                        .copied()
                        .expect("Expected column 179 to be a uint8!"),
                    Phase: row
                        .columns[495]
                        .into_u8()
                        .copied()
                        .expect("Expected column 495 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[22]
                        .into_u32()
                        .copied()
                        .expect("Expected column 22 to be a uint32!"),
                    FishParameter: row
                        .columns[101]
                        .into_u32()
                        .copied()
                        .expect("Expected column 101 to be a uint32!"),
                    ItemReceived: row
                        .columns[259]
                        .into_u32()
                        .copied()
                        .expect("Expected column 259 to be a uint32!"),
                    Reward: [
                        row
                            .columns[338]
                            .into_u16()
                            .copied()
                            .expect("Expected column 338 to be a uint16!"),
                        row
                            .columns[417]
                            .into_u16()
                            .copied()
                            .expect("Expected column 417 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[180]
                        .into_u8()
                        .copied()
                        .expect("Expected column 180 to be a uint8!"),
                    Phase: row
                        .columns[496]
                        .into_u8()
                        .copied()
                        .expect("Expected column 496 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[23]
                        .into_u32()
                        .copied()
                        .expect("Expected column 23 to be a uint32!"),
                    FishParameter: row
                        .columns[102]
                        .into_u32()
                        .copied()
                        .expect("Expected column 102 to be a uint32!"),
                    ItemReceived: row
                        .columns[260]
                        .into_u32()
                        .copied()
                        .expect("Expected column 260 to be a uint32!"),
                    Reward: [
                        row
                            .columns[339]
                            .into_u16()
                            .copied()
                            .expect("Expected column 339 to be a uint16!"),
                        row
                            .columns[418]
                            .into_u16()
                            .copied()
                            .expect("Expected column 418 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[181]
                        .into_u8()
                        .copied()
                        .expect("Expected column 181 to be a uint8!"),
                    Phase: row
                        .columns[497]
                        .into_u8()
                        .copied()
                        .expect("Expected column 497 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[24]
                        .into_u32()
                        .copied()
                        .expect("Expected column 24 to be a uint32!"),
                    FishParameter: row
                        .columns[103]
                        .into_u32()
                        .copied()
                        .expect("Expected column 103 to be a uint32!"),
                    ItemReceived: row
                        .columns[261]
                        .into_u32()
                        .copied()
                        .expect("Expected column 261 to be a uint32!"),
                    Reward: [
                        row
                            .columns[340]
                            .into_u16()
                            .copied()
                            .expect("Expected column 340 to be a uint16!"),
                        row
                            .columns[419]
                            .into_u16()
                            .copied()
                            .expect("Expected column 419 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[182]
                        .into_u8()
                        .copied()
                        .expect("Expected column 182 to be a uint8!"),
                    Phase: row
                        .columns[498]
                        .into_u8()
                        .copied()
                        .expect("Expected column 498 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[25]
                        .into_u32()
                        .copied()
                        .expect("Expected column 25 to be a uint32!"),
                    FishParameter: row
                        .columns[104]
                        .into_u32()
                        .copied()
                        .expect("Expected column 104 to be a uint32!"),
                    ItemReceived: row
                        .columns[262]
                        .into_u32()
                        .copied()
                        .expect("Expected column 262 to be a uint32!"),
                    Reward: [
                        row
                            .columns[341]
                            .into_u16()
                            .copied()
                            .expect("Expected column 341 to be a uint16!"),
                        row
                            .columns[420]
                            .into_u16()
                            .copied()
                            .expect("Expected column 420 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[183]
                        .into_u8()
                        .copied()
                        .expect("Expected column 183 to be a uint8!"),
                    Phase: row
                        .columns[499]
                        .into_u8()
                        .copied()
                        .expect("Expected column 499 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[26]
                        .into_u32()
                        .copied()
                        .expect("Expected column 26 to be a uint32!"),
                    FishParameter: row
                        .columns[105]
                        .into_u32()
                        .copied()
                        .expect("Expected column 105 to be a uint32!"),
                    ItemReceived: row
                        .columns[263]
                        .into_u32()
                        .copied()
                        .expect("Expected column 263 to be a uint32!"),
                    Reward: [
                        row
                            .columns[342]
                            .into_u16()
                            .copied()
                            .expect("Expected column 342 to be a uint16!"),
                        row
                            .columns[421]
                            .into_u16()
                            .copied()
                            .expect("Expected column 421 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[184]
                        .into_u8()
                        .copied()
                        .expect("Expected column 184 to be a uint8!"),
                    Phase: row
                        .columns[500]
                        .into_u8()
                        .copied()
                        .expect("Expected column 500 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[27]
                        .into_u32()
                        .copied()
                        .expect("Expected column 27 to be a uint32!"),
                    FishParameter: row
                        .columns[106]
                        .into_u32()
                        .copied()
                        .expect("Expected column 106 to be a uint32!"),
                    ItemReceived: row
                        .columns[264]
                        .into_u32()
                        .copied()
                        .expect("Expected column 264 to be a uint32!"),
                    Reward: [
                        row
                            .columns[343]
                            .into_u16()
                            .copied()
                            .expect("Expected column 343 to be a uint16!"),
                        row
                            .columns[422]
                            .into_u16()
                            .copied()
                            .expect("Expected column 422 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[185]
                        .into_u8()
                        .copied()
                        .expect("Expected column 185 to be a uint8!"),
                    Phase: row
                        .columns[501]
                        .into_u8()
                        .copied()
                        .expect("Expected column 501 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[28]
                        .into_u32()
                        .copied()
                        .expect("Expected column 28 to be a uint32!"),
                    FishParameter: row
                        .columns[107]
                        .into_u32()
                        .copied()
                        .expect("Expected column 107 to be a uint32!"),
                    ItemReceived: row
                        .columns[265]
                        .into_u32()
                        .copied()
                        .expect("Expected column 265 to be a uint32!"),
                    Reward: [
                        row
                            .columns[344]
                            .into_u16()
                            .copied()
                            .expect("Expected column 344 to be a uint16!"),
                        row
                            .columns[423]
                            .into_u16()
                            .copied()
                            .expect("Expected column 423 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[186]
                        .into_u8()
                        .copied()
                        .expect("Expected column 186 to be a uint8!"),
                    Phase: row
                        .columns[502]
                        .into_u8()
                        .copied()
                        .expect("Expected column 502 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[29]
                        .into_u32()
                        .copied()
                        .expect("Expected column 29 to be a uint32!"),
                    FishParameter: row
                        .columns[108]
                        .into_u32()
                        .copied()
                        .expect("Expected column 108 to be a uint32!"),
                    ItemReceived: row
                        .columns[266]
                        .into_u32()
                        .copied()
                        .expect("Expected column 266 to be a uint32!"),
                    Reward: [
                        row
                            .columns[345]
                            .into_u16()
                            .copied()
                            .expect("Expected column 345 to be a uint16!"),
                        row
                            .columns[424]
                            .into_u16()
                            .copied()
                            .expect("Expected column 424 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[187]
                        .into_u8()
                        .copied()
                        .expect("Expected column 187 to be a uint8!"),
                    Phase: row
                        .columns[503]
                        .into_u8()
                        .copied()
                        .expect("Expected column 503 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[30]
                        .into_u32()
                        .copied()
                        .expect("Expected column 30 to be a uint32!"),
                    FishParameter: row
                        .columns[109]
                        .into_u32()
                        .copied()
                        .expect("Expected column 109 to be a uint32!"),
                    ItemReceived: row
                        .columns[267]
                        .into_u32()
                        .copied()
                        .expect("Expected column 267 to be a uint32!"),
                    Reward: [
                        row
                            .columns[346]
                            .into_u16()
                            .copied()
                            .expect("Expected column 346 to be a uint16!"),
                        row
                            .columns[425]
                            .into_u16()
                            .copied()
                            .expect("Expected column 425 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[188]
                        .into_u8()
                        .copied()
                        .expect("Expected column 188 to be a uint8!"),
                    Phase: row
                        .columns[504]
                        .into_u8()
                        .copied()
                        .expect("Expected column 504 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[31]
                        .into_u32()
                        .copied()
                        .expect("Expected column 31 to be a uint32!"),
                    FishParameter: row
                        .columns[110]
                        .into_u32()
                        .copied()
                        .expect("Expected column 110 to be a uint32!"),
                    ItemReceived: row
                        .columns[268]
                        .into_u32()
                        .copied()
                        .expect("Expected column 268 to be a uint32!"),
                    Reward: [
                        row
                            .columns[347]
                            .into_u16()
                            .copied()
                            .expect("Expected column 347 to be a uint16!"),
                        row
                            .columns[426]
                            .into_u16()
                            .copied()
                            .expect("Expected column 426 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[189]
                        .into_u8()
                        .copied()
                        .expect("Expected column 189 to be a uint8!"),
                    Phase: row
                        .columns[505]
                        .into_u8()
                        .copied()
                        .expect("Expected column 505 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[32]
                        .into_u32()
                        .copied()
                        .expect("Expected column 32 to be a uint32!"),
                    FishParameter: row
                        .columns[111]
                        .into_u32()
                        .copied()
                        .expect("Expected column 111 to be a uint32!"),
                    ItemReceived: row
                        .columns[269]
                        .into_u32()
                        .copied()
                        .expect("Expected column 269 to be a uint32!"),
                    Reward: [
                        row
                            .columns[348]
                            .into_u16()
                            .copied()
                            .expect("Expected column 348 to be a uint16!"),
                        row
                            .columns[427]
                            .into_u16()
                            .copied()
                            .expect("Expected column 427 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[190]
                        .into_u8()
                        .copied()
                        .expect("Expected column 190 to be a uint8!"),
                    Phase: row
                        .columns[506]
                        .into_u8()
                        .copied()
                        .expect("Expected column 506 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[33]
                        .into_u32()
                        .copied()
                        .expect("Expected column 33 to be a uint32!"),
                    FishParameter: row
                        .columns[112]
                        .into_u32()
                        .copied()
                        .expect("Expected column 112 to be a uint32!"),
                    ItemReceived: row
                        .columns[270]
                        .into_u32()
                        .copied()
                        .expect("Expected column 270 to be a uint32!"),
                    Reward: [
                        row
                            .columns[349]
                            .into_u16()
                            .copied()
                            .expect("Expected column 349 to be a uint16!"),
                        row
                            .columns[428]
                            .into_u16()
                            .copied()
                            .expect("Expected column 428 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[191]
                        .into_u8()
                        .copied()
                        .expect("Expected column 191 to be a uint8!"),
                    Phase: row
                        .columns[507]
                        .into_u8()
                        .copied()
                        .expect("Expected column 507 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[34]
                        .into_u32()
                        .copied()
                        .expect("Expected column 34 to be a uint32!"),
                    FishParameter: row
                        .columns[113]
                        .into_u32()
                        .copied()
                        .expect("Expected column 113 to be a uint32!"),
                    ItemReceived: row
                        .columns[271]
                        .into_u32()
                        .copied()
                        .expect("Expected column 271 to be a uint32!"),
                    Reward: [
                        row
                            .columns[350]
                            .into_u16()
                            .copied()
                            .expect("Expected column 350 to be a uint16!"),
                        row
                            .columns[429]
                            .into_u16()
                            .copied()
                            .expect("Expected column 429 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[192]
                        .into_u8()
                        .copied()
                        .expect("Expected column 192 to be a uint8!"),
                    Phase: row
                        .columns[508]
                        .into_u8()
                        .copied()
                        .expect("Expected column 508 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[35]
                        .into_u32()
                        .copied()
                        .expect("Expected column 35 to be a uint32!"),
                    FishParameter: row
                        .columns[114]
                        .into_u32()
                        .copied()
                        .expect("Expected column 114 to be a uint32!"),
                    ItemReceived: row
                        .columns[272]
                        .into_u32()
                        .copied()
                        .expect("Expected column 272 to be a uint32!"),
                    Reward: [
                        row
                            .columns[351]
                            .into_u16()
                            .copied()
                            .expect("Expected column 351 to be a uint16!"),
                        row
                            .columns[430]
                            .into_u16()
                            .copied()
                            .expect("Expected column 430 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[193]
                        .into_u8()
                        .copied()
                        .expect("Expected column 193 to be a uint8!"),
                    Phase: row
                        .columns[509]
                        .into_u8()
                        .copied()
                        .expect("Expected column 509 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[36]
                        .into_u32()
                        .copied()
                        .expect("Expected column 36 to be a uint32!"),
                    FishParameter: row
                        .columns[115]
                        .into_u32()
                        .copied()
                        .expect("Expected column 115 to be a uint32!"),
                    ItemReceived: row
                        .columns[273]
                        .into_u32()
                        .copied()
                        .expect("Expected column 273 to be a uint32!"),
                    Reward: [
                        row
                            .columns[352]
                            .into_u16()
                            .copied()
                            .expect("Expected column 352 to be a uint16!"),
                        row
                            .columns[431]
                            .into_u16()
                            .copied()
                            .expect("Expected column 431 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[194]
                        .into_u8()
                        .copied()
                        .expect("Expected column 194 to be a uint8!"),
                    Phase: row
                        .columns[510]
                        .into_u8()
                        .copied()
                        .expect("Expected column 510 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[37]
                        .into_u32()
                        .copied()
                        .expect("Expected column 37 to be a uint32!"),
                    FishParameter: row
                        .columns[116]
                        .into_u32()
                        .copied()
                        .expect("Expected column 116 to be a uint32!"),
                    ItemReceived: row
                        .columns[274]
                        .into_u32()
                        .copied()
                        .expect("Expected column 274 to be a uint32!"),
                    Reward: [
                        row
                            .columns[353]
                            .into_u16()
                            .copied()
                            .expect("Expected column 353 to be a uint16!"),
                        row
                            .columns[432]
                            .into_u16()
                            .copied()
                            .expect("Expected column 432 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[195]
                        .into_u8()
                        .copied()
                        .expect("Expected column 195 to be a uint8!"),
                    Phase: row
                        .columns[511]
                        .into_u8()
                        .copied()
                        .expect("Expected column 511 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[38]
                        .into_u32()
                        .copied()
                        .expect("Expected column 38 to be a uint32!"),
                    FishParameter: row
                        .columns[117]
                        .into_u32()
                        .copied()
                        .expect("Expected column 117 to be a uint32!"),
                    ItemReceived: row
                        .columns[275]
                        .into_u32()
                        .copied()
                        .expect("Expected column 275 to be a uint32!"),
                    Reward: [
                        row
                            .columns[354]
                            .into_u16()
                            .copied()
                            .expect("Expected column 354 to be a uint16!"),
                        row
                            .columns[433]
                            .into_u16()
                            .copied()
                            .expect("Expected column 433 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[196]
                        .into_u8()
                        .copied()
                        .expect("Expected column 196 to be a uint8!"),
                    Phase: row
                        .columns[512]
                        .into_u8()
                        .copied()
                        .expect("Expected column 512 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[39]
                        .into_u32()
                        .copied()
                        .expect("Expected column 39 to be a uint32!"),
                    FishParameter: row
                        .columns[118]
                        .into_u32()
                        .copied()
                        .expect("Expected column 118 to be a uint32!"),
                    ItemReceived: row
                        .columns[276]
                        .into_u32()
                        .copied()
                        .expect("Expected column 276 to be a uint32!"),
                    Reward: [
                        row
                            .columns[355]
                            .into_u16()
                            .copied()
                            .expect("Expected column 355 to be a uint16!"),
                        row
                            .columns[434]
                            .into_u16()
                            .copied()
                            .expect("Expected column 434 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[197]
                        .into_u8()
                        .copied()
                        .expect("Expected column 197 to be a uint8!"),
                    Phase: row
                        .columns[513]
                        .into_u8()
                        .copied()
                        .expect("Expected column 513 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[40]
                        .into_u32()
                        .copied()
                        .expect("Expected column 40 to be a uint32!"),
                    FishParameter: row
                        .columns[119]
                        .into_u32()
                        .copied()
                        .expect("Expected column 119 to be a uint32!"),
                    ItemReceived: row
                        .columns[277]
                        .into_u32()
                        .copied()
                        .expect("Expected column 277 to be a uint32!"),
                    Reward: [
                        row
                            .columns[356]
                            .into_u16()
                            .copied()
                            .expect("Expected column 356 to be a uint16!"),
                        row
                            .columns[435]
                            .into_u16()
                            .copied()
                            .expect("Expected column 435 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[198]
                        .into_u8()
                        .copied()
                        .expect("Expected column 198 to be a uint8!"),
                    Phase: row
                        .columns[514]
                        .into_u8()
                        .copied()
                        .expect("Expected column 514 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[41]
                        .into_u32()
                        .copied()
                        .expect("Expected column 41 to be a uint32!"),
                    FishParameter: row
                        .columns[120]
                        .into_u32()
                        .copied()
                        .expect("Expected column 120 to be a uint32!"),
                    ItemReceived: row
                        .columns[278]
                        .into_u32()
                        .copied()
                        .expect("Expected column 278 to be a uint32!"),
                    Reward: [
                        row
                            .columns[357]
                            .into_u16()
                            .copied()
                            .expect("Expected column 357 to be a uint16!"),
                        row
                            .columns[436]
                            .into_u16()
                            .copied()
                            .expect("Expected column 436 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[199]
                        .into_u8()
                        .copied()
                        .expect("Expected column 199 to be a uint8!"),
                    Phase: row
                        .columns[515]
                        .into_u8()
                        .copied()
                        .expect("Expected column 515 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[42]
                        .into_u32()
                        .copied()
                        .expect("Expected column 42 to be a uint32!"),
                    FishParameter: row
                        .columns[121]
                        .into_u32()
                        .copied()
                        .expect("Expected column 121 to be a uint32!"),
                    ItemReceived: row
                        .columns[279]
                        .into_u32()
                        .copied()
                        .expect("Expected column 279 to be a uint32!"),
                    Reward: [
                        row
                            .columns[358]
                            .into_u16()
                            .copied()
                            .expect("Expected column 358 to be a uint16!"),
                        row
                            .columns[437]
                            .into_u16()
                            .copied()
                            .expect("Expected column 437 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[200]
                        .into_u8()
                        .copied()
                        .expect("Expected column 200 to be a uint8!"),
                    Phase: row
                        .columns[516]
                        .into_u8()
                        .copied()
                        .expect("Expected column 516 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[43]
                        .into_u32()
                        .copied()
                        .expect("Expected column 43 to be a uint32!"),
                    FishParameter: row
                        .columns[122]
                        .into_u32()
                        .copied()
                        .expect("Expected column 122 to be a uint32!"),
                    ItemReceived: row
                        .columns[280]
                        .into_u32()
                        .copied()
                        .expect("Expected column 280 to be a uint32!"),
                    Reward: [
                        row
                            .columns[359]
                            .into_u16()
                            .copied()
                            .expect("Expected column 359 to be a uint16!"),
                        row
                            .columns[438]
                            .into_u16()
                            .copied()
                            .expect("Expected column 438 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[201]
                        .into_u8()
                        .copied()
                        .expect("Expected column 201 to be a uint8!"),
                    Phase: row
                        .columns[517]
                        .into_u8()
                        .copied()
                        .expect("Expected column 517 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[44]
                        .into_u32()
                        .copied()
                        .expect("Expected column 44 to be a uint32!"),
                    FishParameter: row
                        .columns[123]
                        .into_u32()
                        .copied()
                        .expect("Expected column 123 to be a uint32!"),
                    ItemReceived: row
                        .columns[281]
                        .into_u32()
                        .copied()
                        .expect("Expected column 281 to be a uint32!"),
                    Reward: [
                        row
                            .columns[360]
                            .into_u16()
                            .copied()
                            .expect("Expected column 360 to be a uint16!"),
                        row
                            .columns[439]
                            .into_u16()
                            .copied()
                            .expect("Expected column 439 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[202]
                        .into_u8()
                        .copied()
                        .expect("Expected column 202 to be a uint8!"),
                    Phase: row
                        .columns[518]
                        .into_u8()
                        .copied()
                        .expect("Expected column 518 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[45]
                        .into_u32()
                        .copied()
                        .expect("Expected column 45 to be a uint32!"),
                    FishParameter: row
                        .columns[124]
                        .into_u32()
                        .copied()
                        .expect("Expected column 124 to be a uint32!"),
                    ItemReceived: row
                        .columns[282]
                        .into_u32()
                        .copied()
                        .expect("Expected column 282 to be a uint32!"),
                    Reward: [
                        row
                            .columns[361]
                            .into_u16()
                            .copied()
                            .expect("Expected column 361 to be a uint16!"),
                        row
                            .columns[440]
                            .into_u16()
                            .copied()
                            .expect("Expected column 440 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[203]
                        .into_u8()
                        .copied()
                        .expect("Expected column 203 to be a uint8!"),
                    Phase: row
                        .columns[519]
                        .into_u8()
                        .copied()
                        .expect("Expected column 519 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[46]
                        .into_u32()
                        .copied()
                        .expect("Expected column 46 to be a uint32!"),
                    FishParameter: row
                        .columns[125]
                        .into_u32()
                        .copied()
                        .expect("Expected column 125 to be a uint32!"),
                    ItemReceived: row
                        .columns[283]
                        .into_u32()
                        .copied()
                        .expect("Expected column 283 to be a uint32!"),
                    Reward: [
                        row
                            .columns[362]
                            .into_u16()
                            .copied()
                            .expect("Expected column 362 to be a uint16!"),
                        row
                            .columns[441]
                            .into_u16()
                            .copied()
                            .expect("Expected column 441 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[204]
                        .into_u8()
                        .copied()
                        .expect("Expected column 204 to be a uint8!"),
                    Phase: row
                        .columns[520]
                        .into_u8()
                        .copied()
                        .expect("Expected column 520 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[47]
                        .into_u32()
                        .copied()
                        .expect("Expected column 47 to be a uint32!"),
                    FishParameter: row
                        .columns[126]
                        .into_u32()
                        .copied()
                        .expect("Expected column 126 to be a uint32!"),
                    ItemReceived: row
                        .columns[284]
                        .into_u32()
                        .copied()
                        .expect("Expected column 284 to be a uint32!"),
                    Reward: [
                        row
                            .columns[363]
                            .into_u16()
                            .copied()
                            .expect("Expected column 363 to be a uint16!"),
                        row
                            .columns[442]
                            .into_u16()
                            .copied()
                            .expect("Expected column 442 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[205]
                        .into_u8()
                        .copied()
                        .expect("Expected column 205 to be a uint8!"),
                    Phase: row
                        .columns[521]
                        .into_u8()
                        .copied()
                        .expect("Expected column 521 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[48]
                        .into_u32()
                        .copied()
                        .expect("Expected column 48 to be a uint32!"),
                    FishParameter: row
                        .columns[127]
                        .into_u32()
                        .copied()
                        .expect("Expected column 127 to be a uint32!"),
                    ItemReceived: row
                        .columns[285]
                        .into_u32()
                        .copied()
                        .expect("Expected column 285 to be a uint32!"),
                    Reward: [
                        row
                            .columns[364]
                            .into_u16()
                            .copied()
                            .expect("Expected column 364 to be a uint16!"),
                        row
                            .columns[443]
                            .into_u16()
                            .copied()
                            .expect("Expected column 443 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[206]
                        .into_u8()
                        .copied()
                        .expect("Expected column 206 to be a uint8!"),
                    Phase: row
                        .columns[522]
                        .into_u8()
                        .copied()
                        .expect("Expected column 522 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[49]
                        .into_u32()
                        .copied()
                        .expect("Expected column 49 to be a uint32!"),
                    FishParameter: row
                        .columns[128]
                        .into_u32()
                        .copied()
                        .expect("Expected column 128 to be a uint32!"),
                    ItemReceived: row
                        .columns[286]
                        .into_u32()
                        .copied()
                        .expect("Expected column 286 to be a uint32!"),
                    Reward: [
                        row
                            .columns[365]
                            .into_u16()
                            .copied()
                            .expect("Expected column 365 to be a uint16!"),
                        row
                            .columns[444]
                            .into_u16()
                            .copied()
                            .expect("Expected column 444 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[207]
                        .into_u8()
                        .copied()
                        .expect("Expected column 207 to be a uint8!"),
                    Phase: row
                        .columns[523]
                        .into_u8()
                        .copied()
                        .expect("Expected column 523 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[50]
                        .into_u32()
                        .copied()
                        .expect("Expected column 50 to be a uint32!"),
                    FishParameter: row
                        .columns[129]
                        .into_u32()
                        .copied()
                        .expect("Expected column 129 to be a uint32!"),
                    ItemReceived: row
                        .columns[287]
                        .into_u32()
                        .copied()
                        .expect("Expected column 287 to be a uint32!"),
                    Reward: [
                        row
                            .columns[366]
                            .into_u16()
                            .copied()
                            .expect("Expected column 366 to be a uint16!"),
                        row
                            .columns[445]
                            .into_u16()
                            .copied()
                            .expect("Expected column 445 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[208]
                        .into_u8()
                        .copied()
                        .expect("Expected column 208 to be a uint8!"),
                    Phase: row
                        .columns[524]
                        .into_u8()
                        .copied()
                        .expect("Expected column 524 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[51]
                        .into_u32()
                        .copied()
                        .expect("Expected column 51 to be a uint32!"),
                    FishParameter: row
                        .columns[130]
                        .into_u32()
                        .copied()
                        .expect("Expected column 130 to be a uint32!"),
                    ItemReceived: row
                        .columns[288]
                        .into_u32()
                        .copied()
                        .expect("Expected column 288 to be a uint32!"),
                    Reward: [
                        row
                            .columns[367]
                            .into_u16()
                            .copied()
                            .expect("Expected column 367 to be a uint16!"),
                        row
                            .columns[446]
                            .into_u16()
                            .copied()
                            .expect("Expected column 446 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[209]
                        .into_u8()
                        .copied()
                        .expect("Expected column 209 to be a uint8!"),
                    Phase: row
                        .columns[525]
                        .into_u8()
                        .copied()
                        .expect("Expected column 525 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[52]
                        .into_u32()
                        .copied()
                        .expect("Expected column 52 to be a uint32!"),
                    FishParameter: row
                        .columns[131]
                        .into_u32()
                        .copied()
                        .expect("Expected column 131 to be a uint32!"),
                    ItemReceived: row
                        .columns[289]
                        .into_u32()
                        .copied()
                        .expect("Expected column 289 to be a uint32!"),
                    Reward: [
                        row
                            .columns[368]
                            .into_u16()
                            .copied()
                            .expect("Expected column 368 to be a uint16!"),
                        row
                            .columns[447]
                            .into_u16()
                            .copied()
                            .expect("Expected column 447 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[210]
                        .into_u8()
                        .copied()
                        .expect("Expected column 210 to be a uint8!"),
                    Phase: row
                        .columns[526]
                        .into_u8()
                        .copied()
                        .expect("Expected column 526 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[53]
                        .into_u32()
                        .copied()
                        .expect("Expected column 53 to be a uint32!"),
                    FishParameter: row
                        .columns[132]
                        .into_u32()
                        .copied()
                        .expect("Expected column 132 to be a uint32!"),
                    ItemReceived: row
                        .columns[290]
                        .into_u32()
                        .copied()
                        .expect("Expected column 290 to be a uint32!"),
                    Reward: [
                        row
                            .columns[369]
                            .into_u16()
                            .copied()
                            .expect("Expected column 369 to be a uint16!"),
                        row
                            .columns[448]
                            .into_u16()
                            .copied()
                            .expect("Expected column 448 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[211]
                        .into_u8()
                        .copied()
                        .expect("Expected column 211 to be a uint8!"),
                    Phase: row
                        .columns[527]
                        .into_u8()
                        .copied()
                        .expect("Expected column 527 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[54]
                        .into_u32()
                        .copied()
                        .expect("Expected column 54 to be a uint32!"),
                    FishParameter: row
                        .columns[133]
                        .into_u32()
                        .copied()
                        .expect("Expected column 133 to be a uint32!"),
                    ItemReceived: row
                        .columns[291]
                        .into_u32()
                        .copied()
                        .expect("Expected column 291 to be a uint32!"),
                    Reward: [
                        row
                            .columns[370]
                            .into_u16()
                            .copied()
                            .expect("Expected column 370 to be a uint16!"),
                        row
                            .columns[449]
                            .into_u16()
                            .copied()
                            .expect("Expected column 449 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[212]
                        .into_u8()
                        .copied()
                        .expect("Expected column 212 to be a uint8!"),
                    Phase: row
                        .columns[528]
                        .into_u8()
                        .copied()
                        .expect("Expected column 528 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[55]
                        .into_u32()
                        .copied()
                        .expect("Expected column 55 to be a uint32!"),
                    FishParameter: row
                        .columns[134]
                        .into_u32()
                        .copied()
                        .expect("Expected column 134 to be a uint32!"),
                    ItemReceived: row
                        .columns[292]
                        .into_u32()
                        .copied()
                        .expect("Expected column 292 to be a uint32!"),
                    Reward: [
                        row
                            .columns[371]
                            .into_u16()
                            .copied()
                            .expect("Expected column 371 to be a uint16!"),
                        row
                            .columns[450]
                            .into_u16()
                            .copied()
                            .expect("Expected column 450 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[213]
                        .into_u8()
                        .copied()
                        .expect("Expected column 213 to be a uint8!"),
                    Phase: row
                        .columns[529]
                        .into_u8()
                        .copied()
                        .expect("Expected column 529 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[56]
                        .into_u32()
                        .copied()
                        .expect("Expected column 56 to be a uint32!"),
                    FishParameter: row
                        .columns[135]
                        .into_u32()
                        .copied()
                        .expect("Expected column 135 to be a uint32!"),
                    ItemReceived: row
                        .columns[293]
                        .into_u32()
                        .copied()
                        .expect("Expected column 293 to be a uint32!"),
                    Reward: [
                        row
                            .columns[372]
                            .into_u16()
                            .copied()
                            .expect("Expected column 372 to be a uint16!"),
                        row
                            .columns[451]
                            .into_u16()
                            .copied()
                            .expect("Expected column 451 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[214]
                        .into_u8()
                        .copied()
                        .expect("Expected column 214 to be a uint8!"),
                    Phase: row
                        .columns[530]
                        .into_u8()
                        .copied()
                        .expect("Expected column 530 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[57]
                        .into_u32()
                        .copied()
                        .expect("Expected column 57 to be a uint32!"),
                    FishParameter: row
                        .columns[136]
                        .into_u32()
                        .copied()
                        .expect("Expected column 136 to be a uint32!"),
                    ItemReceived: row
                        .columns[294]
                        .into_u32()
                        .copied()
                        .expect("Expected column 294 to be a uint32!"),
                    Reward: [
                        row
                            .columns[373]
                            .into_u16()
                            .copied()
                            .expect("Expected column 373 to be a uint16!"),
                        row
                            .columns[452]
                            .into_u16()
                            .copied()
                            .expect("Expected column 452 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[215]
                        .into_u8()
                        .copied()
                        .expect("Expected column 215 to be a uint8!"),
                    Phase: row
                        .columns[531]
                        .into_u8()
                        .copied()
                        .expect("Expected column 531 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[58]
                        .into_u32()
                        .copied()
                        .expect("Expected column 58 to be a uint32!"),
                    FishParameter: row
                        .columns[137]
                        .into_u32()
                        .copied()
                        .expect("Expected column 137 to be a uint32!"),
                    ItemReceived: row
                        .columns[295]
                        .into_u32()
                        .copied()
                        .expect("Expected column 295 to be a uint32!"),
                    Reward: [
                        row
                            .columns[374]
                            .into_u16()
                            .copied()
                            .expect("Expected column 374 to be a uint16!"),
                        row
                            .columns[453]
                            .into_u16()
                            .copied()
                            .expect("Expected column 453 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[216]
                        .into_u8()
                        .copied()
                        .expect("Expected column 216 to be a uint8!"),
                    Phase: row
                        .columns[532]
                        .into_u8()
                        .copied()
                        .expect("Expected column 532 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[59]
                        .into_u32()
                        .copied()
                        .expect("Expected column 59 to be a uint32!"),
                    FishParameter: row
                        .columns[138]
                        .into_u32()
                        .copied()
                        .expect("Expected column 138 to be a uint32!"),
                    ItemReceived: row
                        .columns[296]
                        .into_u32()
                        .copied()
                        .expect("Expected column 296 to be a uint32!"),
                    Reward: [
                        row
                            .columns[375]
                            .into_u16()
                            .copied()
                            .expect("Expected column 375 to be a uint16!"),
                        row
                            .columns[454]
                            .into_u16()
                            .copied()
                            .expect("Expected column 454 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[217]
                        .into_u8()
                        .copied()
                        .expect("Expected column 217 to be a uint8!"),
                    Phase: row
                        .columns[533]
                        .into_u8()
                        .copied()
                        .expect("Expected column 533 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[60]
                        .into_u32()
                        .copied()
                        .expect("Expected column 60 to be a uint32!"),
                    FishParameter: row
                        .columns[139]
                        .into_u32()
                        .copied()
                        .expect("Expected column 139 to be a uint32!"),
                    ItemReceived: row
                        .columns[297]
                        .into_u32()
                        .copied()
                        .expect("Expected column 297 to be a uint32!"),
                    Reward: [
                        row
                            .columns[376]
                            .into_u16()
                            .copied()
                            .expect("Expected column 376 to be a uint16!"),
                        row
                            .columns[455]
                            .into_u16()
                            .copied()
                            .expect("Expected column 455 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[218]
                        .into_u8()
                        .copied()
                        .expect("Expected column 218 to be a uint8!"),
                    Phase: row
                        .columns[534]
                        .into_u8()
                        .copied()
                        .expect("Expected column 534 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[61]
                        .into_u32()
                        .copied()
                        .expect("Expected column 61 to be a uint32!"),
                    FishParameter: row
                        .columns[140]
                        .into_u32()
                        .copied()
                        .expect("Expected column 140 to be a uint32!"),
                    ItemReceived: row
                        .columns[298]
                        .into_u32()
                        .copied()
                        .expect("Expected column 298 to be a uint32!"),
                    Reward: [
                        row
                            .columns[377]
                            .into_u16()
                            .copied()
                            .expect("Expected column 377 to be a uint16!"),
                        row
                            .columns[456]
                            .into_u16()
                            .copied()
                            .expect("Expected column 456 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[219]
                        .into_u8()
                        .copied()
                        .expect("Expected column 219 to be a uint8!"),
                    Phase: row
                        .columns[535]
                        .into_u8()
                        .copied()
                        .expect("Expected column 535 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[62]
                        .into_u32()
                        .copied()
                        .expect("Expected column 62 to be a uint32!"),
                    FishParameter: row
                        .columns[141]
                        .into_u32()
                        .copied()
                        .expect("Expected column 141 to be a uint32!"),
                    ItemReceived: row
                        .columns[299]
                        .into_u32()
                        .copied()
                        .expect("Expected column 299 to be a uint32!"),
                    Reward: [
                        row
                            .columns[378]
                            .into_u16()
                            .copied()
                            .expect("Expected column 378 to be a uint16!"),
                        row
                            .columns[457]
                            .into_u16()
                            .copied()
                            .expect("Expected column 457 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[220]
                        .into_u8()
                        .copied()
                        .expect("Expected column 220 to be a uint8!"),
                    Phase: row
                        .columns[536]
                        .into_u8()
                        .copied()
                        .expect("Expected column 536 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[63]
                        .into_u32()
                        .copied()
                        .expect("Expected column 63 to be a uint32!"),
                    FishParameter: row
                        .columns[142]
                        .into_u32()
                        .copied()
                        .expect("Expected column 142 to be a uint32!"),
                    ItemReceived: row
                        .columns[300]
                        .into_u32()
                        .copied()
                        .expect("Expected column 300 to be a uint32!"),
                    Reward: [
                        row
                            .columns[379]
                            .into_u16()
                            .copied()
                            .expect("Expected column 379 to be a uint16!"),
                        row
                            .columns[458]
                            .into_u16()
                            .copied()
                            .expect("Expected column 458 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[221]
                        .into_u8()
                        .copied()
                        .expect("Expected column 221 to be a uint8!"),
                    Phase: row
                        .columns[537]
                        .into_u8()
                        .copied()
                        .expect("Expected column 537 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[64]
                        .into_u32()
                        .copied()
                        .expect("Expected column 64 to be a uint32!"),
                    FishParameter: row
                        .columns[143]
                        .into_u32()
                        .copied()
                        .expect("Expected column 143 to be a uint32!"),
                    ItemReceived: row
                        .columns[301]
                        .into_u32()
                        .copied()
                        .expect("Expected column 301 to be a uint32!"),
                    Reward: [
                        row
                            .columns[380]
                            .into_u16()
                            .copied()
                            .expect("Expected column 380 to be a uint16!"),
                        row
                            .columns[459]
                            .into_u16()
                            .copied()
                            .expect("Expected column 459 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[222]
                        .into_u8()
                        .copied()
                        .expect("Expected column 222 to be a uint8!"),
                    Phase: row
                        .columns[538]
                        .into_u8()
                        .copied()
                        .expect("Expected column 538 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[65]
                        .into_u32()
                        .copied()
                        .expect("Expected column 65 to be a uint32!"),
                    FishParameter: row
                        .columns[144]
                        .into_u32()
                        .copied()
                        .expect("Expected column 144 to be a uint32!"),
                    ItemReceived: row
                        .columns[302]
                        .into_u32()
                        .copied()
                        .expect("Expected column 302 to be a uint32!"),
                    Reward: [
                        row
                            .columns[381]
                            .into_u16()
                            .copied()
                            .expect("Expected column 381 to be a uint16!"),
                        row
                            .columns[460]
                            .into_u16()
                            .copied()
                            .expect("Expected column 460 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[223]
                        .into_u8()
                        .copied()
                        .expect("Expected column 223 to be a uint8!"),
                    Phase: row
                        .columns[539]
                        .into_u8()
                        .copied()
                        .expect("Expected column 539 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[66]
                        .into_u32()
                        .copied()
                        .expect("Expected column 66 to be a uint32!"),
                    FishParameter: row
                        .columns[145]
                        .into_u32()
                        .copied()
                        .expect("Expected column 145 to be a uint32!"),
                    ItemReceived: row
                        .columns[303]
                        .into_u32()
                        .copied()
                        .expect("Expected column 303 to be a uint32!"),
                    Reward: [
                        row
                            .columns[382]
                            .into_u16()
                            .copied()
                            .expect("Expected column 382 to be a uint16!"),
                        row
                            .columns[461]
                            .into_u16()
                            .copied()
                            .expect("Expected column 461 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[224]
                        .into_u8()
                        .copied()
                        .expect("Expected column 224 to be a uint8!"),
                    Phase: row
                        .columns[540]
                        .into_u8()
                        .copied()
                        .expect("Expected column 540 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[67]
                        .into_u32()
                        .copied()
                        .expect("Expected column 67 to be a uint32!"),
                    FishParameter: row
                        .columns[146]
                        .into_u32()
                        .copied()
                        .expect("Expected column 146 to be a uint32!"),
                    ItemReceived: row
                        .columns[304]
                        .into_u32()
                        .copied()
                        .expect("Expected column 304 to be a uint32!"),
                    Reward: [
                        row
                            .columns[383]
                            .into_u16()
                            .copied()
                            .expect("Expected column 383 to be a uint16!"),
                        row
                            .columns[462]
                            .into_u16()
                            .copied()
                            .expect("Expected column 462 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[225]
                        .into_u8()
                        .copied()
                        .expect("Expected column 225 to be a uint8!"),
                    Phase: row
                        .columns[541]
                        .into_u8()
                        .copied()
                        .expect("Expected column 541 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[68]
                        .into_u32()
                        .copied()
                        .expect("Expected column 68 to be a uint32!"),
                    FishParameter: row
                        .columns[147]
                        .into_u32()
                        .copied()
                        .expect("Expected column 147 to be a uint32!"),
                    ItemReceived: row
                        .columns[305]
                        .into_u32()
                        .copied()
                        .expect("Expected column 305 to be a uint32!"),
                    Reward: [
                        row
                            .columns[384]
                            .into_u16()
                            .copied()
                            .expect("Expected column 384 to be a uint16!"),
                        row
                            .columns[463]
                            .into_u16()
                            .copied()
                            .expect("Expected column 463 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[226]
                        .into_u8()
                        .copied()
                        .expect("Expected column 226 to be a uint8!"),
                    Phase: row
                        .columns[542]
                        .into_u8()
                        .copied()
                        .expect("Expected column 542 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[69]
                        .into_u32()
                        .copied()
                        .expect("Expected column 69 to be a uint32!"),
                    FishParameter: row
                        .columns[148]
                        .into_u32()
                        .copied()
                        .expect("Expected column 148 to be a uint32!"),
                    ItemReceived: row
                        .columns[306]
                        .into_u32()
                        .copied()
                        .expect("Expected column 306 to be a uint32!"),
                    Reward: [
                        row
                            .columns[385]
                            .into_u16()
                            .copied()
                            .expect("Expected column 385 to be a uint16!"),
                        row
                            .columns[464]
                            .into_u16()
                            .copied()
                            .expect("Expected column 464 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[227]
                        .into_u8()
                        .copied()
                        .expect("Expected column 227 to be a uint8!"),
                    Phase: row
                        .columns[543]
                        .into_u8()
                        .copied()
                        .expect("Expected column 543 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[70]
                        .into_u32()
                        .copied()
                        .expect("Expected column 70 to be a uint32!"),
                    FishParameter: row
                        .columns[149]
                        .into_u32()
                        .copied()
                        .expect("Expected column 149 to be a uint32!"),
                    ItemReceived: row
                        .columns[307]
                        .into_u32()
                        .copied()
                        .expect("Expected column 307 to be a uint32!"),
                    Reward: [
                        row
                            .columns[386]
                            .into_u16()
                            .copied()
                            .expect("Expected column 386 to be a uint16!"),
                        row
                            .columns[465]
                            .into_u16()
                            .copied()
                            .expect("Expected column 465 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[228]
                        .into_u8()
                        .copied()
                        .expect("Expected column 228 to be a uint8!"),
                    Phase: row
                        .columns[544]
                        .into_u8()
                        .copied()
                        .expect("Expected column 544 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[71]
                        .into_u32()
                        .copied()
                        .expect("Expected column 71 to be a uint32!"),
                    FishParameter: row
                        .columns[150]
                        .into_u32()
                        .copied()
                        .expect("Expected column 150 to be a uint32!"),
                    ItemReceived: row
                        .columns[308]
                        .into_u32()
                        .copied()
                        .expect("Expected column 308 to be a uint32!"),
                    Reward: [
                        row
                            .columns[387]
                            .into_u16()
                            .copied()
                            .expect("Expected column 387 to be a uint16!"),
                        row
                            .columns[466]
                            .into_u16()
                            .copied()
                            .expect("Expected column 466 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[229]
                        .into_u8()
                        .copied()
                        .expect("Expected column 229 to be a uint8!"),
                    Phase: row
                        .columns[545]
                        .into_u8()
                        .copied()
                        .expect("Expected column 545 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[72]
                        .into_u32()
                        .copied()
                        .expect("Expected column 72 to be a uint32!"),
                    FishParameter: row
                        .columns[151]
                        .into_u32()
                        .copied()
                        .expect("Expected column 151 to be a uint32!"),
                    ItemReceived: row
                        .columns[309]
                        .into_u32()
                        .copied()
                        .expect("Expected column 309 to be a uint32!"),
                    Reward: [
                        row
                            .columns[388]
                            .into_u16()
                            .copied()
                            .expect("Expected column 388 to be a uint16!"),
                        row
                            .columns[467]
                            .into_u16()
                            .copied()
                            .expect("Expected column 467 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[230]
                        .into_u8()
                        .copied()
                        .expect("Expected column 230 to be a uint8!"),
                    Phase: row
                        .columns[546]
                        .into_u8()
                        .copied()
                        .expect("Expected column 546 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[73]
                        .into_u32()
                        .copied()
                        .expect("Expected column 73 to be a uint32!"),
                    FishParameter: row
                        .columns[152]
                        .into_u32()
                        .copied()
                        .expect("Expected column 152 to be a uint32!"),
                    ItemReceived: row
                        .columns[310]
                        .into_u32()
                        .copied()
                        .expect("Expected column 310 to be a uint32!"),
                    Reward: [
                        row
                            .columns[389]
                            .into_u16()
                            .copied()
                            .expect("Expected column 389 to be a uint16!"),
                        row
                            .columns[468]
                            .into_u16()
                            .copied()
                            .expect("Expected column 468 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[231]
                        .into_u8()
                        .copied()
                        .expect("Expected column 231 to be a uint8!"),
                    Phase: row
                        .columns[547]
                        .into_u8()
                        .copied()
                        .expect("Expected column 547 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[74]
                        .into_u32()
                        .copied()
                        .expect("Expected column 74 to be a uint32!"),
                    FishParameter: row
                        .columns[153]
                        .into_u32()
                        .copied()
                        .expect("Expected column 153 to be a uint32!"),
                    ItemReceived: row
                        .columns[311]
                        .into_u32()
                        .copied()
                        .expect("Expected column 311 to be a uint32!"),
                    Reward: [
                        row
                            .columns[390]
                            .into_u16()
                            .copied()
                            .expect("Expected column 390 to be a uint16!"),
                        row
                            .columns[469]
                            .into_u16()
                            .copied()
                            .expect("Expected column 469 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[232]
                        .into_u8()
                        .copied()
                        .expect("Expected column 232 to be a uint8!"),
                    Phase: row
                        .columns[548]
                        .into_u8()
                        .copied()
                        .expect("Expected column 548 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[75]
                        .into_u32()
                        .copied()
                        .expect("Expected column 75 to be a uint32!"),
                    FishParameter: row
                        .columns[154]
                        .into_u32()
                        .copied()
                        .expect("Expected column 154 to be a uint32!"),
                    ItemReceived: row
                        .columns[312]
                        .into_u32()
                        .copied()
                        .expect("Expected column 312 to be a uint32!"),
                    Reward: [
                        row
                            .columns[391]
                            .into_u16()
                            .copied()
                            .expect("Expected column 391 to be a uint16!"),
                        row
                            .columns[470]
                            .into_u16()
                            .copied()
                            .expect("Expected column 470 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[233]
                        .into_u8()
                        .copied()
                        .expect("Expected column 233 to be a uint8!"),
                    Phase: row
                        .columns[549]
                        .into_u8()
                        .copied()
                        .expect("Expected column 549 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[76]
                        .into_u32()
                        .copied()
                        .expect("Expected column 76 to be a uint32!"),
                    FishParameter: row
                        .columns[155]
                        .into_u32()
                        .copied()
                        .expect("Expected column 155 to be a uint32!"),
                    ItemReceived: row
                        .columns[313]
                        .into_u32()
                        .copied()
                        .expect("Expected column 313 to be a uint32!"),
                    Reward: [
                        row
                            .columns[392]
                            .into_u16()
                            .copied()
                            .expect("Expected column 392 to be a uint16!"),
                        row
                            .columns[471]
                            .into_u16()
                            .copied()
                            .expect("Expected column 471 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[234]
                        .into_u8()
                        .copied()
                        .expect("Expected column 234 to be a uint8!"),
                    Phase: row
                        .columns[550]
                        .into_u8()
                        .copied()
                        .expect("Expected column 550 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[77]
                        .into_u32()
                        .copied()
                        .expect("Expected column 77 to be a uint32!"),
                    FishParameter: row
                        .columns[156]
                        .into_u32()
                        .copied()
                        .expect("Expected column 156 to be a uint32!"),
                    ItemReceived: row
                        .columns[314]
                        .into_u32()
                        .copied()
                        .expect("Expected column 314 to be a uint32!"),
                    Reward: [
                        row
                            .columns[393]
                            .into_u16()
                            .copied()
                            .expect("Expected column 393 to be a uint16!"),
                        row
                            .columns[472]
                            .into_u16()
                            .copied()
                            .expect("Expected column 472 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[235]
                        .into_u8()
                        .copied()
                        .expect("Expected column 235 to be a uint8!"),
                    Phase: row
                        .columns[551]
                        .into_u8()
                        .copied()
                        .expect("Expected column 551 to be a uint8!"),
                },
                HWDGathererInspectionDataElement {
                    RequiredItem: row
                        .columns[78]
                        .into_u32()
                        .copied()
                        .expect("Expected column 78 to be a uint32!"),
                    FishParameter: row
                        .columns[157]
                        .into_u32()
                        .copied()
                        .expect("Expected column 157 to be a uint32!"),
                    ItemReceived: row
                        .columns[315]
                        .into_u32()
                        .copied()
                        .expect("Expected column 315 to be a uint32!"),
                    Reward: [
                        row
                            .columns[394]
                            .into_u16()
                            .copied()
                            .expect("Expected column 394 to be a uint16!"),
                        row
                            .columns[473]
                            .into_u16()
                            .copied()
                            .expect("Expected column 473 to be a uint16!"),
                    ],
                    AmountRequired: row
                        .columns[236]
                        .into_u8()
                        .copied()
                        .expect("Expected column 236 to be a uint8!"),
                    Phase: row
                        .columns[552]
                        .into_u8()
                        .copied()
                        .expect("Expected column 552 to be a uint8!"),
                },
            ],
        })
    }
}
impl<'a> IntoIterator for &'a HWDGathererInspectionSheet {
    type Item = (u32, Vec<(u16, HWDGathererInspectionRow)>);
    type IntoIter = StructuredSheetIterator<'a, HWDGathererInspectionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HWDGathererInspectionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HWDGathererInspectionRow {
    ///""
    pub HWDGathererInspectionData: [HWDGathererInspectionDataElement; 79],
}
