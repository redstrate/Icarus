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
pub struct HWDCrafterSupplyParamsElement {
    pub ItemTradeIn: u32,
    pub BaseCollectableRating: u16,
    pub MidCollectableRating: u16,
    pub HighCollectableRating: u16,
    pub BaseCollectableReward: u16,
    pub MidCollectableReward: u16,
    pub HighCollectableReward: u16,
    pub BaseCollectableRewardPostPhase: u16,
    pub MidCollectableRewardPostPhase: u16,
    pub HighCollectableRewardPostPhase: u16,
    pub Level: u8,
    pub LevelMax: u8,
    pub Unknown0: u8,
    pub TermName: u8,
}
#[derive(Debug, Clone)]
pub struct HWDCrafterSupplySheet {
    sheet: Sheet,
}
impl HWDCrafterSupplySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HWDCrafterSupply")?;
        let sheet = resolver.read_excel_sheet(&exh, "HWDCrafterSupply", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HWDCrafterSupplyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HWDCrafterSupplyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for HWDCrafterSupplySheet {
    type Row = HWDCrafterSupplyRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            HWDCrafterSupplyParams: [
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[0]
                        .into_u32()
                        .copied()
                        .expect("Expected column 0 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[92]
                        .into_u16()
                        .copied()
                        .expect("Expected column 92 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[115]
                        .into_u16()
                        .copied()
                        .expect("Expected column 115 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[138]
                        .into_u16()
                        .copied()
                        .expect("Expected column 138 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[161]
                        .into_u16()
                        .copied()
                        .expect("Expected column 161 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[184]
                        .into_u16()
                        .copied()
                        .expect("Expected column 184 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[207]
                        .into_u16()
                        .copied()
                        .expect("Expected column 207 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[230]
                        .into_u16()
                        .copied()
                        .expect("Expected column 230 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[253]
                        .into_u16()
                        .copied()
                        .expect("Expected column 253 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[276]
                        .into_u16()
                        .copied()
                        .expect("Expected column 276 to be a uint16!"),
                    Level: row
                        .columns[23]
                        .into_u8()
                        .copied()
                        .expect("Expected column 23 to be a uint8!"),
                    LevelMax: row
                        .columns[46]
                        .into_u8()
                        .copied()
                        .expect("Expected column 46 to be a uint8!"),
                    Unknown0: row
                        .columns[69]
                        .into_u8()
                        .copied()
                        .expect("Expected column 69 to be a uint8!"),
                    TermName: row
                        .columns[299]
                        .into_u8()
                        .copied()
                        .expect("Expected column 299 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[1]
                        .into_u32()
                        .copied()
                        .expect("Expected column 1 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[93]
                        .into_u16()
                        .copied()
                        .expect("Expected column 93 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[116]
                        .into_u16()
                        .copied()
                        .expect("Expected column 116 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[139]
                        .into_u16()
                        .copied()
                        .expect("Expected column 139 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[162]
                        .into_u16()
                        .copied()
                        .expect("Expected column 162 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[185]
                        .into_u16()
                        .copied()
                        .expect("Expected column 185 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[208]
                        .into_u16()
                        .copied()
                        .expect("Expected column 208 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[231]
                        .into_u16()
                        .copied()
                        .expect("Expected column 231 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[254]
                        .into_u16()
                        .copied()
                        .expect("Expected column 254 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[277]
                        .into_u16()
                        .copied()
                        .expect("Expected column 277 to be a uint16!"),
                    Level: row
                        .columns[24]
                        .into_u8()
                        .copied()
                        .expect("Expected column 24 to be a uint8!"),
                    LevelMax: row
                        .columns[47]
                        .into_u8()
                        .copied()
                        .expect("Expected column 47 to be a uint8!"),
                    Unknown0: row
                        .columns[70]
                        .into_u8()
                        .copied()
                        .expect("Expected column 70 to be a uint8!"),
                    TermName: row
                        .columns[300]
                        .into_u8()
                        .copied()
                        .expect("Expected column 300 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[2]
                        .into_u32()
                        .copied()
                        .expect("Expected column 2 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[94]
                        .into_u16()
                        .copied()
                        .expect("Expected column 94 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[117]
                        .into_u16()
                        .copied()
                        .expect("Expected column 117 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[140]
                        .into_u16()
                        .copied()
                        .expect("Expected column 140 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[163]
                        .into_u16()
                        .copied()
                        .expect("Expected column 163 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[186]
                        .into_u16()
                        .copied()
                        .expect("Expected column 186 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[209]
                        .into_u16()
                        .copied()
                        .expect("Expected column 209 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[232]
                        .into_u16()
                        .copied()
                        .expect("Expected column 232 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[255]
                        .into_u16()
                        .copied()
                        .expect("Expected column 255 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[278]
                        .into_u16()
                        .copied()
                        .expect("Expected column 278 to be a uint16!"),
                    Level: row
                        .columns[25]
                        .into_u8()
                        .copied()
                        .expect("Expected column 25 to be a uint8!"),
                    LevelMax: row
                        .columns[48]
                        .into_u8()
                        .copied()
                        .expect("Expected column 48 to be a uint8!"),
                    Unknown0: row
                        .columns[71]
                        .into_u8()
                        .copied()
                        .expect("Expected column 71 to be a uint8!"),
                    TermName: row
                        .columns[301]
                        .into_u8()
                        .copied()
                        .expect("Expected column 301 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[3]
                        .into_u32()
                        .copied()
                        .expect("Expected column 3 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[95]
                        .into_u16()
                        .copied()
                        .expect("Expected column 95 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[118]
                        .into_u16()
                        .copied()
                        .expect("Expected column 118 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[141]
                        .into_u16()
                        .copied()
                        .expect("Expected column 141 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[164]
                        .into_u16()
                        .copied()
                        .expect("Expected column 164 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[187]
                        .into_u16()
                        .copied()
                        .expect("Expected column 187 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[210]
                        .into_u16()
                        .copied()
                        .expect("Expected column 210 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[233]
                        .into_u16()
                        .copied()
                        .expect("Expected column 233 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[256]
                        .into_u16()
                        .copied()
                        .expect("Expected column 256 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[279]
                        .into_u16()
                        .copied()
                        .expect("Expected column 279 to be a uint16!"),
                    Level: row
                        .columns[26]
                        .into_u8()
                        .copied()
                        .expect("Expected column 26 to be a uint8!"),
                    LevelMax: row
                        .columns[49]
                        .into_u8()
                        .copied()
                        .expect("Expected column 49 to be a uint8!"),
                    Unknown0: row
                        .columns[72]
                        .into_u8()
                        .copied()
                        .expect("Expected column 72 to be a uint8!"),
                    TermName: row
                        .columns[302]
                        .into_u8()
                        .copied()
                        .expect("Expected column 302 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[4]
                        .into_u32()
                        .copied()
                        .expect("Expected column 4 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[96]
                        .into_u16()
                        .copied()
                        .expect("Expected column 96 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[119]
                        .into_u16()
                        .copied()
                        .expect("Expected column 119 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[142]
                        .into_u16()
                        .copied()
                        .expect("Expected column 142 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[165]
                        .into_u16()
                        .copied()
                        .expect("Expected column 165 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[188]
                        .into_u16()
                        .copied()
                        .expect("Expected column 188 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[211]
                        .into_u16()
                        .copied()
                        .expect("Expected column 211 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[234]
                        .into_u16()
                        .copied()
                        .expect("Expected column 234 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[257]
                        .into_u16()
                        .copied()
                        .expect("Expected column 257 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[280]
                        .into_u16()
                        .copied()
                        .expect("Expected column 280 to be a uint16!"),
                    Level: row
                        .columns[27]
                        .into_u8()
                        .copied()
                        .expect("Expected column 27 to be a uint8!"),
                    LevelMax: row
                        .columns[50]
                        .into_u8()
                        .copied()
                        .expect("Expected column 50 to be a uint8!"),
                    Unknown0: row
                        .columns[73]
                        .into_u8()
                        .copied()
                        .expect("Expected column 73 to be a uint8!"),
                    TermName: row
                        .columns[303]
                        .into_u8()
                        .copied()
                        .expect("Expected column 303 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[5]
                        .into_u32()
                        .copied()
                        .expect("Expected column 5 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[97]
                        .into_u16()
                        .copied()
                        .expect("Expected column 97 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[120]
                        .into_u16()
                        .copied()
                        .expect("Expected column 120 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[143]
                        .into_u16()
                        .copied()
                        .expect("Expected column 143 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[166]
                        .into_u16()
                        .copied()
                        .expect("Expected column 166 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[189]
                        .into_u16()
                        .copied()
                        .expect("Expected column 189 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[212]
                        .into_u16()
                        .copied()
                        .expect("Expected column 212 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[235]
                        .into_u16()
                        .copied()
                        .expect("Expected column 235 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[258]
                        .into_u16()
                        .copied()
                        .expect("Expected column 258 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[281]
                        .into_u16()
                        .copied()
                        .expect("Expected column 281 to be a uint16!"),
                    Level: row
                        .columns[28]
                        .into_u8()
                        .copied()
                        .expect("Expected column 28 to be a uint8!"),
                    LevelMax: row
                        .columns[51]
                        .into_u8()
                        .copied()
                        .expect("Expected column 51 to be a uint8!"),
                    Unknown0: row
                        .columns[74]
                        .into_u8()
                        .copied()
                        .expect("Expected column 74 to be a uint8!"),
                    TermName: row
                        .columns[304]
                        .into_u8()
                        .copied()
                        .expect("Expected column 304 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[98]
                        .into_u16()
                        .copied()
                        .expect("Expected column 98 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[121]
                        .into_u16()
                        .copied()
                        .expect("Expected column 121 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[144]
                        .into_u16()
                        .copied()
                        .expect("Expected column 144 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[167]
                        .into_u16()
                        .copied()
                        .expect("Expected column 167 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[190]
                        .into_u16()
                        .copied()
                        .expect("Expected column 190 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[213]
                        .into_u16()
                        .copied()
                        .expect("Expected column 213 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[236]
                        .into_u16()
                        .copied()
                        .expect("Expected column 236 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[259]
                        .into_u16()
                        .copied()
                        .expect("Expected column 259 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[282]
                        .into_u16()
                        .copied()
                        .expect("Expected column 282 to be a uint16!"),
                    Level: row
                        .columns[29]
                        .into_u8()
                        .copied()
                        .expect("Expected column 29 to be a uint8!"),
                    LevelMax: row
                        .columns[52]
                        .into_u8()
                        .copied()
                        .expect("Expected column 52 to be a uint8!"),
                    Unknown0: row
                        .columns[75]
                        .into_u8()
                        .copied()
                        .expect("Expected column 75 to be a uint8!"),
                    TermName: row
                        .columns[305]
                        .into_u8()
                        .copied()
                        .expect("Expected column 305 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[99]
                        .into_u16()
                        .copied()
                        .expect("Expected column 99 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[122]
                        .into_u16()
                        .copied()
                        .expect("Expected column 122 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[145]
                        .into_u16()
                        .copied()
                        .expect("Expected column 145 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[168]
                        .into_u16()
                        .copied()
                        .expect("Expected column 168 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[191]
                        .into_u16()
                        .copied()
                        .expect("Expected column 191 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[214]
                        .into_u16()
                        .copied()
                        .expect("Expected column 214 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[237]
                        .into_u16()
                        .copied()
                        .expect("Expected column 237 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[260]
                        .into_u16()
                        .copied()
                        .expect("Expected column 260 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[283]
                        .into_u16()
                        .copied()
                        .expect("Expected column 283 to be a uint16!"),
                    Level: row
                        .columns[30]
                        .into_u8()
                        .copied()
                        .expect("Expected column 30 to be a uint8!"),
                    LevelMax: row
                        .columns[53]
                        .into_u8()
                        .copied()
                        .expect("Expected column 53 to be a uint8!"),
                    Unknown0: row
                        .columns[76]
                        .into_u8()
                        .copied()
                        .expect("Expected column 76 to be a uint8!"),
                    TermName: row
                        .columns[306]
                        .into_u8()
                        .copied()
                        .expect("Expected column 306 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[100]
                        .into_u16()
                        .copied()
                        .expect("Expected column 100 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[123]
                        .into_u16()
                        .copied()
                        .expect("Expected column 123 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[146]
                        .into_u16()
                        .copied()
                        .expect("Expected column 146 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[169]
                        .into_u16()
                        .copied()
                        .expect("Expected column 169 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[192]
                        .into_u16()
                        .copied()
                        .expect("Expected column 192 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[215]
                        .into_u16()
                        .copied()
                        .expect("Expected column 215 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[238]
                        .into_u16()
                        .copied()
                        .expect("Expected column 238 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[261]
                        .into_u16()
                        .copied()
                        .expect("Expected column 261 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[284]
                        .into_u16()
                        .copied()
                        .expect("Expected column 284 to be a uint16!"),
                    Level: row
                        .columns[31]
                        .into_u8()
                        .copied()
                        .expect("Expected column 31 to be a uint8!"),
                    LevelMax: row
                        .columns[54]
                        .into_u8()
                        .copied()
                        .expect("Expected column 54 to be a uint8!"),
                    Unknown0: row
                        .columns[77]
                        .into_u8()
                        .copied()
                        .expect("Expected column 77 to be a uint8!"),
                    TermName: row
                        .columns[307]
                        .into_u8()
                        .copied()
                        .expect("Expected column 307 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[101]
                        .into_u16()
                        .copied()
                        .expect("Expected column 101 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[124]
                        .into_u16()
                        .copied()
                        .expect("Expected column 124 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[147]
                        .into_u16()
                        .copied()
                        .expect("Expected column 147 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[170]
                        .into_u16()
                        .copied()
                        .expect("Expected column 170 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[193]
                        .into_u16()
                        .copied()
                        .expect("Expected column 193 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[216]
                        .into_u16()
                        .copied()
                        .expect("Expected column 216 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[239]
                        .into_u16()
                        .copied()
                        .expect("Expected column 239 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[262]
                        .into_u16()
                        .copied()
                        .expect("Expected column 262 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[285]
                        .into_u16()
                        .copied()
                        .expect("Expected column 285 to be a uint16!"),
                    Level: row
                        .columns[32]
                        .into_u8()
                        .copied()
                        .expect("Expected column 32 to be a uint8!"),
                    LevelMax: row
                        .columns[55]
                        .into_u8()
                        .copied()
                        .expect("Expected column 55 to be a uint8!"),
                    Unknown0: row
                        .columns[78]
                        .into_u8()
                        .copied()
                        .expect("Expected column 78 to be a uint8!"),
                    TermName: row
                        .columns[308]
                        .into_u8()
                        .copied()
                        .expect("Expected column 308 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[102]
                        .into_u16()
                        .copied()
                        .expect("Expected column 102 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[125]
                        .into_u16()
                        .copied()
                        .expect("Expected column 125 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[148]
                        .into_u16()
                        .copied()
                        .expect("Expected column 148 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[171]
                        .into_u16()
                        .copied()
                        .expect("Expected column 171 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[194]
                        .into_u16()
                        .copied()
                        .expect("Expected column 194 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[217]
                        .into_u16()
                        .copied()
                        .expect("Expected column 217 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[240]
                        .into_u16()
                        .copied()
                        .expect("Expected column 240 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[263]
                        .into_u16()
                        .copied()
                        .expect("Expected column 263 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[286]
                        .into_u16()
                        .copied()
                        .expect("Expected column 286 to be a uint16!"),
                    Level: row
                        .columns[33]
                        .into_u8()
                        .copied()
                        .expect("Expected column 33 to be a uint8!"),
                    LevelMax: row
                        .columns[56]
                        .into_u8()
                        .copied()
                        .expect("Expected column 56 to be a uint8!"),
                    Unknown0: row
                        .columns[79]
                        .into_u8()
                        .copied()
                        .expect("Expected column 79 to be a uint8!"),
                    TermName: row
                        .columns[309]
                        .into_u8()
                        .copied()
                        .expect("Expected column 309 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[103]
                        .into_u16()
                        .copied()
                        .expect("Expected column 103 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[126]
                        .into_u16()
                        .copied()
                        .expect("Expected column 126 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[149]
                        .into_u16()
                        .copied()
                        .expect("Expected column 149 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[172]
                        .into_u16()
                        .copied()
                        .expect("Expected column 172 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[195]
                        .into_u16()
                        .copied()
                        .expect("Expected column 195 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[218]
                        .into_u16()
                        .copied()
                        .expect("Expected column 218 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[241]
                        .into_u16()
                        .copied()
                        .expect("Expected column 241 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[264]
                        .into_u16()
                        .copied()
                        .expect("Expected column 264 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[287]
                        .into_u16()
                        .copied()
                        .expect("Expected column 287 to be a uint16!"),
                    Level: row
                        .columns[34]
                        .into_u8()
                        .copied()
                        .expect("Expected column 34 to be a uint8!"),
                    LevelMax: row
                        .columns[57]
                        .into_u8()
                        .copied()
                        .expect("Expected column 57 to be a uint8!"),
                    Unknown0: row
                        .columns[80]
                        .into_u8()
                        .copied()
                        .expect("Expected column 80 to be a uint8!"),
                    TermName: row
                        .columns[310]
                        .into_u8()
                        .copied()
                        .expect("Expected column 310 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[12]
                        .into_u32()
                        .copied()
                        .expect("Expected column 12 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[104]
                        .into_u16()
                        .copied()
                        .expect("Expected column 104 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[127]
                        .into_u16()
                        .copied()
                        .expect("Expected column 127 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[150]
                        .into_u16()
                        .copied()
                        .expect("Expected column 150 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[173]
                        .into_u16()
                        .copied()
                        .expect("Expected column 173 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[196]
                        .into_u16()
                        .copied()
                        .expect("Expected column 196 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[219]
                        .into_u16()
                        .copied()
                        .expect("Expected column 219 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[242]
                        .into_u16()
                        .copied()
                        .expect("Expected column 242 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[265]
                        .into_u16()
                        .copied()
                        .expect("Expected column 265 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[288]
                        .into_u16()
                        .copied()
                        .expect("Expected column 288 to be a uint16!"),
                    Level: row
                        .columns[35]
                        .into_u8()
                        .copied()
                        .expect("Expected column 35 to be a uint8!"),
                    LevelMax: row
                        .columns[58]
                        .into_u8()
                        .copied()
                        .expect("Expected column 58 to be a uint8!"),
                    Unknown0: row
                        .columns[81]
                        .into_u8()
                        .copied()
                        .expect("Expected column 81 to be a uint8!"),
                    TermName: row
                        .columns[311]
                        .into_u8()
                        .copied()
                        .expect("Expected column 311 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[13]
                        .into_u32()
                        .copied()
                        .expect("Expected column 13 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[105]
                        .into_u16()
                        .copied()
                        .expect("Expected column 105 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[128]
                        .into_u16()
                        .copied()
                        .expect("Expected column 128 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[151]
                        .into_u16()
                        .copied()
                        .expect("Expected column 151 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[174]
                        .into_u16()
                        .copied()
                        .expect("Expected column 174 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[197]
                        .into_u16()
                        .copied()
                        .expect("Expected column 197 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[220]
                        .into_u16()
                        .copied()
                        .expect("Expected column 220 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[243]
                        .into_u16()
                        .copied()
                        .expect("Expected column 243 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[266]
                        .into_u16()
                        .copied()
                        .expect("Expected column 266 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[289]
                        .into_u16()
                        .copied()
                        .expect("Expected column 289 to be a uint16!"),
                    Level: row
                        .columns[36]
                        .into_u8()
                        .copied()
                        .expect("Expected column 36 to be a uint8!"),
                    LevelMax: row
                        .columns[59]
                        .into_u8()
                        .copied()
                        .expect("Expected column 59 to be a uint8!"),
                    Unknown0: row
                        .columns[82]
                        .into_u8()
                        .copied()
                        .expect("Expected column 82 to be a uint8!"),
                    TermName: row
                        .columns[312]
                        .into_u8()
                        .copied()
                        .expect("Expected column 312 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[14]
                        .into_u32()
                        .copied()
                        .expect("Expected column 14 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[106]
                        .into_u16()
                        .copied()
                        .expect("Expected column 106 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[129]
                        .into_u16()
                        .copied()
                        .expect("Expected column 129 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[152]
                        .into_u16()
                        .copied()
                        .expect("Expected column 152 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[175]
                        .into_u16()
                        .copied()
                        .expect("Expected column 175 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[198]
                        .into_u16()
                        .copied()
                        .expect("Expected column 198 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[221]
                        .into_u16()
                        .copied()
                        .expect("Expected column 221 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[244]
                        .into_u16()
                        .copied()
                        .expect("Expected column 244 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[267]
                        .into_u16()
                        .copied()
                        .expect("Expected column 267 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[290]
                        .into_u16()
                        .copied()
                        .expect("Expected column 290 to be a uint16!"),
                    Level: row
                        .columns[37]
                        .into_u8()
                        .copied()
                        .expect("Expected column 37 to be a uint8!"),
                    LevelMax: row
                        .columns[60]
                        .into_u8()
                        .copied()
                        .expect("Expected column 60 to be a uint8!"),
                    Unknown0: row
                        .columns[83]
                        .into_u8()
                        .copied()
                        .expect("Expected column 83 to be a uint8!"),
                    TermName: row
                        .columns[313]
                        .into_u8()
                        .copied()
                        .expect("Expected column 313 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[15]
                        .into_u32()
                        .copied()
                        .expect("Expected column 15 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[107]
                        .into_u16()
                        .copied()
                        .expect("Expected column 107 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[130]
                        .into_u16()
                        .copied()
                        .expect("Expected column 130 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[153]
                        .into_u16()
                        .copied()
                        .expect("Expected column 153 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[176]
                        .into_u16()
                        .copied()
                        .expect("Expected column 176 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[199]
                        .into_u16()
                        .copied()
                        .expect("Expected column 199 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[222]
                        .into_u16()
                        .copied()
                        .expect("Expected column 222 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[245]
                        .into_u16()
                        .copied()
                        .expect("Expected column 245 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[268]
                        .into_u16()
                        .copied()
                        .expect("Expected column 268 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[291]
                        .into_u16()
                        .copied()
                        .expect("Expected column 291 to be a uint16!"),
                    Level: row
                        .columns[38]
                        .into_u8()
                        .copied()
                        .expect("Expected column 38 to be a uint8!"),
                    LevelMax: row
                        .columns[61]
                        .into_u8()
                        .copied()
                        .expect("Expected column 61 to be a uint8!"),
                    Unknown0: row
                        .columns[84]
                        .into_u8()
                        .copied()
                        .expect("Expected column 84 to be a uint8!"),
                    TermName: row
                        .columns[314]
                        .into_u8()
                        .copied()
                        .expect("Expected column 314 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[108]
                        .into_u16()
                        .copied()
                        .expect("Expected column 108 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[131]
                        .into_u16()
                        .copied()
                        .expect("Expected column 131 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[154]
                        .into_u16()
                        .copied()
                        .expect("Expected column 154 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[177]
                        .into_u16()
                        .copied()
                        .expect("Expected column 177 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[200]
                        .into_u16()
                        .copied()
                        .expect("Expected column 200 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[223]
                        .into_u16()
                        .copied()
                        .expect("Expected column 223 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[246]
                        .into_u16()
                        .copied()
                        .expect("Expected column 246 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[269]
                        .into_u16()
                        .copied()
                        .expect("Expected column 269 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[292]
                        .into_u16()
                        .copied()
                        .expect("Expected column 292 to be a uint16!"),
                    Level: row
                        .columns[39]
                        .into_u8()
                        .copied()
                        .expect("Expected column 39 to be a uint8!"),
                    LevelMax: row
                        .columns[62]
                        .into_u8()
                        .copied()
                        .expect("Expected column 62 to be a uint8!"),
                    Unknown0: row
                        .columns[85]
                        .into_u8()
                        .copied()
                        .expect("Expected column 85 to be a uint8!"),
                    TermName: row
                        .columns[315]
                        .into_u8()
                        .copied()
                        .expect("Expected column 315 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[109]
                        .into_u16()
                        .copied()
                        .expect("Expected column 109 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[132]
                        .into_u16()
                        .copied()
                        .expect("Expected column 132 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[155]
                        .into_u16()
                        .copied()
                        .expect("Expected column 155 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[178]
                        .into_u16()
                        .copied()
                        .expect("Expected column 178 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[201]
                        .into_u16()
                        .copied()
                        .expect("Expected column 201 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[224]
                        .into_u16()
                        .copied()
                        .expect("Expected column 224 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[247]
                        .into_u16()
                        .copied()
                        .expect("Expected column 247 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[270]
                        .into_u16()
                        .copied()
                        .expect("Expected column 270 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[293]
                        .into_u16()
                        .copied()
                        .expect("Expected column 293 to be a uint16!"),
                    Level: row
                        .columns[40]
                        .into_u8()
                        .copied()
                        .expect("Expected column 40 to be a uint8!"),
                    LevelMax: row
                        .columns[63]
                        .into_u8()
                        .copied()
                        .expect("Expected column 63 to be a uint8!"),
                    Unknown0: row
                        .columns[86]
                        .into_u8()
                        .copied()
                        .expect("Expected column 86 to be a uint8!"),
                    TermName: row
                        .columns[316]
                        .into_u8()
                        .copied()
                        .expect("Expected column 316 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[110]
                        .into_u16()
                        .copied()
                        .expect("Expected column 110 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[133]
                        .into_u16()
                        .copied()
                        .expect("Expected column 133 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[156]
                        .into_u16()
                        .copied()
                        .expect("Expected column 156 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[179]
                        .into_u16()
                        .copied()
                        .expect("Expected column 179 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[202]
                        .into_u16()
                        .copied()
                        .expect("Expected column 202 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[225]
                        .into_u16()
                        .copied()
                        .expect("Expected column 225 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[248]
                        .into_u16()
                        .copied()
                        .expect("Expected column 248 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[271]
                        .into_u16()
                        .copied()
                        .expect("Expected column 271 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[294]
                        .into_u16()
                        .copied()
                        .expect("Expected column 294 to be a uint16!"),
                    Level: row
                        .columns[41]
                        .into_u8()
                        .copied()
                        .expect("Expected column 41 to be a uint8!"),
                    LevelMax: row
                        .columns[64]
                        .into_u8()
                        .copied()
                        .expect("Expected column 64 to be a uint8!"),
                    Unknown0: row
                        .columns[87]
                        .into_u8()
                        .copied()
                        .expect("Expected column 87 to be a uint8!"),
                    TermName: row
                        .columns[317]
                        .into_u8()
                        .copied()
                        .expect("Expected column 317 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[111]
                        .into_u16()
                        .copied()
                        .expect("Expected column 111 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[134]
                        .into_u16()
                        .copied()
                        .expect("Expected column 134 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[157]
                        .into_u16()
                        .copied()
                        .expect("Expected column 157 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[180]
                        .into_u16()
                        .copied()
                        .expect("Expected column 180 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[203]
                        .into_u16()
                        .copied()
                        .expect("Expected column 203 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[226]
                        .into_u16()
                        .copied()
                        .expect("Expected column 226 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[249]
                        .into_u16()
                        .copied()
                        .expect("Expected column 249 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[272]
                        .into_u16()
                        .copied()
                        .expect("Expected column 272 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[295]
                        .into_u16()
                        .copied()
                        .expect("Expected column 295 to be a uint16!"),
                    Level: row
                        .columns[42]
                        .into_u8()
                        .copied()
                        .expect("Expected column 42 to be a uint8!"),
                    LevelMax: row
                        .columns[65]
                        .into_u8()
                        .copied()
                        .expect("Expected column 65 to be a uint8!"),
                    Unknown0: row
                        .columns[88]
                        .into_u8()
                        .copied()
                        .expect("Expected column 88 to be a uint8!"),
                    TermName: row
                        .columns[318]
                        .into_u8()
                        .copied()
                        .expect("Expected column 318 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[112]
                        .into_u16()
                        .copied()
                        .expect("Expected column 112 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[135]
                        .into_u16()
                        .copied()
                        .expect("Expected column 135 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[158]
                        .into_u16()
                        .copied()
                        .expect("Expected column 158 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[181]
                        .into_u16()
                        .copied()
                        .expect("Expected column 181 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[204]
                        .into_u16()
                        .copied()
                        .expect("Expected column 204 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[227]
                        .into_u16()
                        .copied()
                        .expect("Expected column 227 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[250]
                        .into_u16()
                        .copied()
                        .expect("Expected column 250 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[273]
                        .into_u16()
                        .copied()
                        .expect("Expected column 273 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[296]
                        .into_u16()
                        .copied()
                        .expect("Expected column 296 to be a uint16!"),
                    Level: row
                        .columns[43]
                        .into_u8()
                        .copied()
                        .expect("Expected column 43 to be a uint8!"),
                    LevelMax: row
                        .columns[66]
                        .into_u8()
                        .copied()
                        .expect("Expected column 66 to be a uint8!"),
                    Unknown0: row
                        .columns[89]
                        .into_u8()
                        .copied()
                        .expect("Expected column 89 to be a uint8!"),
                    TermName: row
                        .columns[319]
                        .into_u8()
                        .copied()
                        .expect("Expected column 319 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[21]
                        .into_u32()
                        .copied()
                        .expect("Expected column 21 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[113]
                        .into_u16()
                        .copied()
                        .expect("Expected column 113 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[136]
                        .into_u16()
                        .copied()
                        .expect("Expected column 136 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[159]
                        .into_u16()
                        .copied()
                        .expect("Expected column 159 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[182]
                        .into_u16()
                        .copied()
                        .expect("Expected column 182 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[205]
                        .into_u16()
                        .copied()
                        .expect("Expected column 205 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[228]
                        .into_u16()
                        .copied()
                        .expect("Expected column 228 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[251]
                        .into_u16()
                        .copied()
                        .expect("Expected column 251 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[274]
                        .into_u16()
                        .copied()
                        .expect("Expected column 274 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[297]
                        .into_u16()
                        .copied()
                        .expect("Expected column 297 to be a uint16!"),
                    Level: row
                        .columns[44]
                        .into_u8()
                        .copied()
                        .expect("Expected column 44 to be a uint8!"),
                    LevelMax: row
                        .columns[67]
                        .into_u8()
                        .copied()
                        .expect("Expected column 67 to be a uint8!"),
                    Unknown0: row
                        .columns[90]
                        .into_u8()
                        .copied()
                        .expect("Expected column 90 to be a uint8!"),
                    TermName: row
                        .columns[320]
                        .into_u8()
                        .copied()
                        .expect("Expected column 320 to be a uint8!"),
                },
                HWDCrafterSupplyParamsElement {
                    ItemTradeIn: row
                        .columns[22]
                        .into_u32()
                        .copied()
                        .expect("Expected column 22 to be a uint32!"),
                    BaseCollectableRating: row
                        .columns[114]
                        .into_u16()
                        .copied()
                        .expect("Expected column 114 to be a uint16!"),
                    MidCollectableRating: row
                        .columns[137]
                        .into_u16()
                        .copied()
                        .expect("Expected column 137 to be a uint16!"),
                    HighCollectableRating: row
                        .columns[160]
                        .into_u16()
                        .copied()
                        .expect("Expected column 160 to be a uint16!"),
                    BaseCollectableReward: row
                        .columns[183]
                        .into_u16()
                        .copied()
                        .expect("Expected column 183 to be a uint16!"),
                    MidCollectableReward: row
                        .columns[206]
                        .into_u16()
                        .copied()
                        .expect("Expected column 206 to be a uint16!"),
                    HighCollectableReward: row
                        .columns[229]
                        .into_u16()
                        .copied()
                        .expect("Expected column 229 to be a uint16!"),
                    BaseCollectableRewardPostPhase: row
                        .columns[252]
                        .into_u16()
                        .copied()
                        .expect("Expected column 252 to be a uint16!"),
                    MidCollectableRewardPostPhase: row
                        .columns[275]
                        .into_u16()
                        .copied()
                        .expect("Expected column 275 to be a uint16!"),
                    HighCollectableRewardPostPhase: row
                        .columns[298]
                        .into_u16()
                        .copied()
                        .expect("Expected column 298 to be a uint16!"),
                    Level: row
                        .columns[45]
                        .into_u8()
                        .copied()
                        .expect("Expected column 45 to be a uint8!"),
                    LevelMax: row
                        .columns[68]
                        .into_u8()
                        .copied()
                        .expect("Expected column 68 to be a uint8!"),
                    Unknown0: row
                        .columns[91]
                        .into_u8()
                        .copied()
                        .expect("Expected column 91 to be a uint8!"),
                    TermName: row
                        .columns[321]
                        .into_u8()
                        .copied()
                        .expect("Expected column 321 to be a uint8!"),
                },
            ],
        })
    }
}
impl<'a> IntoIterator for &'a HWDCrafterSupplySheet {
    type Item = (u32, Vec<(u16, HWDCrafterSupplyRow)>);
    type IntoIter = StructuredSheetIterator<'a, HWDCrafterSupplySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HWDCrafterSupplySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HWDCrafterSupplyRow {
    ///""
    pub HWDCrafterSupplyParams: [HWDCrafterSupplyParamsElement; 23],
}
