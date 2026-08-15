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
pub struct QuestBattleParamsElement {
    pub ScriptInstruction: String,
    pub ScriptValue: u32,
}
#[derive(Debug, Clone)]
pub struct QuestBattleSheet {
    sheet: Sheet,
}
impl QuestBattleSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestBattle")?;
        let sheet = resolver.read_excel_sheet(&exh, "QuestBattle", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestBattleRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestBattleRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for QuestBattleSheet {
    type Row = QuestBattleRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            QuestBattleParams: [
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[4]
                        .into_string()
                        .cloned()
                        .expect("Expected column 4 to be a string!"),
                    ScriptValue: row
                        .columns[224]
                        .into_u32()
                        .copied()
                        .expect("Expected column 224 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[5]
                        .into_string()
                        .cloned()
                        .expect("Expected column 5 to be a string!"),
                    ScriptValue: row
                        .columns[225]
                        .into_u32()
                        .copied()
                        .expect("Expected column 225 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[6]
                        .into_string()
                        .cloned()
                        .expect("Expected column 6 to be a string!"),
                    ScriptValue: row
                        .columns[226]
                        .into_u32()
                        .copied()
                        .expect("Expected column 226 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[7]
                        .into_string()
                        .cloned()
                        .expect("Expected column 7 to be a string!"),
                    ScriptValue: row
                        .columns[227]
                        .into_u32()
                        .copied()
                        .expect("Expected column 227 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[8]
                        .into_string()
                        .cloned()
                        .expect("Expected column 8 to be a string!"),
                    ScriptValue: row
                        .columns[228]
                        .into_u32()
                        .copied()
                        .expect("Expected column 228 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[9]
                        .into_string()
                        .cloned()
                        .expect("Expected column 9 to be a string!"),
                    ScriptValue: row
                        .columns[229]
                        .into_u32()
                        .copied()
                        .expect("Expected column 229 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[10]
                        .into_string()
                        .cloned()
                        .expect("Expected column 10 to be a string!"),
                    ScriptValue: row
                        .columns[230]
                        .into_u32()
                        .copied()
                        .expect("Expected column 230 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[11]
                        .into_string()
                        .cloned()
                        .expect("Expected column 11 to be a string!"),
                    ScriptValue: row
                        .columns[231]
                        .into_u32()
                        .copied()
                        .expect("Expected column 231 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[12]
                        .into_string()
                        .cloned()
                        .expect("Expected column 12 to be a string!"),
                    ScriptValue: row
                        .columns[232]
                        .into_u32()
                        .copied()
                        .expect("Expected column 232 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[13]
                        .into_string()
                        .cloned()
                        .expect("Expected column 13 to be a string!"),
                    ScriptValue: row
                        .columns[233]
                        .into_u32()
                        .copied()
                        .expect("Expected column 233 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[14]
                        .into_string()
                        .cloned()
                        .expect("Expected column 14 to be a string!"),
                    ScriptValue: row
                        .columns[234]
                        .into_u32()
                        .copied()
                        .expect("Expected column 234 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[15]
                        .into_string()
                        .cloned()
                        .expect("Expected column 15 to be a string!"),
                    ScriptValue: row
                        .columns[235]
                        .into_u32()
                        .copied()
                        .expect("Expected column 235 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[16]
                        .into_string()
                        .cloned()
                        .expect("Expected column 16 to be a string!"),
                    ScriptValue: row
                        .columns[236]
                        .into_u32()
                        .copied()
                        .expect("Expected column 236 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[17]
                        .into_string()
                        .cloned()
                        .expect("Expected column 17 to be a string!"),
                    ScriptValue: row
                        .columns[237]
                        .into_u32()
                        .copied()
                        .expect("Expected column 237 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[18]
                        .into_string()
                        .cloned()
                        .expect("Expected column 18 to be a string!"),
                    ScriptValue: row
                        .columns[238]
                        .into_u32()
                        .copied()
                        .expect("Expected column 238 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[19]
                        .into_string()
                        .cloned()
                        .expect("Expected column 19 to be a string!"),
                    ScriptValue: row
                        .columns[239]
                        .into_u32()
                        .copied()
                        .expect("Expected column 239 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[20]
                        .into_string()
                        .cloned()
                        .expect("Expected column 20 to be a string!"),
                    ScriptValue: row
                        .columns[240]
                        .into_u32()
                        .copied()
                        .expect("Expected column 240 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[21]
                        .into_string()
                        .cloned()
                        .expect("Expected column 21 to be a string!"),
                    ScriptValue: row
                        .columns[241]
                        .into_u32()
                        .copied()
                        .expect("Expected column 241 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[22]
                        .into_string()
                        .cloned()
                        .expect("Expected column 22 to be a string!"),
                    ScriptValue: row
                        .columns[242]
                        .into_u32()
                        .copied()
                        .expect("Expected column 242 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[23]
                        .into_string()
                        .cloned()
                        .expect("Expected column 23 to be a string!"),
                    ScriptValue: row
                        .columns[243]
                        .into_u32()
                        .copied()
                        .expect("Expected column 243 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[24]
                        .into_string()
                        .cloned()
                        .expect("Expected column 24 to be a string!"),
                    ScriptValue: row
                        .columns[244]
                        .into_u32()
                        .copied()
                        .expect("Expected column 244 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[25]
                        .into_string()
                        .cloned()
                        .expect("Expected column 25 to be a string!"),
                    ScriptValue: row
                        .columns[245]
                        .into_u32()
                        .copied()
                        .expect("Expected column 245 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[26]
                        .into_string()
                        .cloned()
                        .expect("Expected column 26 to be a string!"),
                    ScriptValue: row
                        .columns[246]
                        .into_u32()
                        .copied()
                        .expect("Expected column 246 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[27]
                        .into_string()
                        .cloned()
                        .expect("Expected column 27 to be a string!"),
                    ScriptValue: row
                        .columns[247]
                        .into_u32()
                        .copied()
                        .expect("Expected column 247 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[28]
                        .into_string()
                        .cloned()
                        .expect("Expected column 28 to be a string!"),
                    ScriptValue: row
                        .columns[248]
                        .into_u32()
                        .copied()
                        .expect("Expected column 248 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[29]
                        .into_string()
                        .cloned()
                        .expect("Expected column 29 to be a string!"),
                    ScriptValue: row
                        .columns[249]
                        .into_u32()
                        .copied()
                        .expect("Expected column 249 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[30]
                        .into_string()
                        .cloned()
                        .expect("Expected column 30 to be a string!"),
                    ScriptValue: row
                        .columns[250]
                        .into_u32()
                        .copied()
                        .expect("Expected column 250 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[31]
                        .into_string()
                        .cloned()
                        .expect("Expected column 31 to be a string!"),
                    ScriptValue: row
                        .columns[251]
                        .into_u32()
                        .copied()
                        .expect("Expected column 251 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[32]
                        .into_string()
                        .cloned()
                        .expect("Expected column 32 to be a string!"),
                    ScriptValue: row
                        .columns[252]
                        .into_u32()
                        .copied()
                        .expect("Expected column 252 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[33]
                        .into_string()
                        .cloned()
                        .expect("Expected column 33 to be a string!"),
                    ScriptValue: row
                        .columns[253]
                        .into_u32()
                        .copied()
                        .expect("Expected column 253 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[34]
                        .into_string()
                        .cloned()
                        .expect("Expected column 34 to be a string!"),
                    ScriptValue: row
                        .columns[254]
                        .into_u32()
                        .copied()
                        .expect("Expected column 254 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[35]
                        .into_string()
                        .cloned()
                        .expect("Expected column 35 to be a string!"),
                    ScriptValue: row
                        .columns[255]
                        .into_u32()
                        .copied()
                        .expect("Expected column 255 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[36]
                        .into_string()
                        .cloned()
                        .expect("Expected column 36 to be a string!"),
                    ScriptValue: row
                        .columns[256]
                        .into_u32()
                        .copied()
                        .expect("Expected column 256 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[37]
                        .into_string()
                        .cloned()
                        .expect("Expected column 37 to be a string!"),
                    ScriptValue: row
                        .columns[257]
                        .into_u32()
                        .copied()
                        .expect("Expected column 257 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[38]
                        .into_string()
                        .cloned()
                        .expect("Expected column 38 to be a string!"),
                    ScriptValue: row
                        .columns[258]
                        .into_u32()
                        .copied()
                        .expect("Expected column 258 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[39]
                        .into_string()
                        .cloned()
                        .expect("Expected column 39 to be a string!"),
                    ScriptValue: row
                        .columns[259]
                        .into_u32()
                        .copied()
                        .expect("Expected column 259 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[40]
                        .into_string()
                        .cloned()
                        .expect("Expected column 40 to be a string!"),
                    ScriptValue: row
                        .columns[260]
                        .into_u32()
                        .copied()
                        .expect("Expected column 260 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[41]
                        .into_string()
                        .cloned()
                        .expect("Expected column 41 to be a string!"),
                    ScriptValue: row
                        .columns[261]
                        .into_u32()
                        .copied()
                        .expect("Expected column 261 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[42]
                        .into_string()
                        .cloned()
                        .expect("Expected column 42 to be a string!"),
                    ScriptValue: row
                        .columns[262]
                        .into_u32()
                        .copied()
                        .expect("Expected column 262 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[43]
                        .into_string()
                        .cloned()
                        .expect("Expected column 43 to be a string!"),
                    ScriptValue: row
                        .columns[263]
                        .into_u32()
                        .copied()
                        .expect("Expected column 263 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[44]
                        .into_string()
                        .cloned()
                        .expect("Expected column 44 to be a string!"),
                    ScriptValue: row
                        .columns[264]
                        .into_u32()
                        .copied()
                        .expect("Expected column 264 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[45]
                        .into_string()
                        .cloned()
                        .expect("Expected column 45 to be a string!"),
                    ScriptValue: row
                        .columns[265]
                        .into_u32()
                        .copied()
                        .expect("Expected column 265 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[46]
                        .into_string()
                        .cloned()
                        .expect("Expected column 46 to be a string!"),
                    ScriptValue: row
                        .columns[266]
                        .into_u32()
                        .copied()
                        .expect("Expected column 266 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[47]
                        .into_string()
                        .cloned()
                        .expect("Expected column 47 to be a string!"),
                    ScriptValue: row
                        .columns[267]
                        .into_u32()
                        .copied()
                        .expect("Expected column 267 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[48]
                        .into_string()
                        .cloned()
                        .expect("Expected column 48 to be a string!"),
                    ScriptValue: row
                        .columns[268]
                        .into_u32()
                        .copied()
                        .expect("Expected column 268 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[49]
                        .into_string()
                        .cloned()
                        .expect("Expected column 49 to be a string!"),
                    ScriptValue: row
                        .columns[269]
                        .into_u32()
                        .copied()
                        .expect("Expected column 269 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[50]
                        .into_string()
                        .cloned()
                        .expect("Expected column 50 to be a string!"),
                    ScriptValue: row
                        .columns[270]
                        .into_u32()
                        .copied()
                        .expect("Expected column 270 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[51]
                        .into_string()
                        .cloned()
                        .expect("Expected column 51 to be a string!"),
                    ScriptValue: row
                        .columns[271]
                        .into_u32()
                        .copied()
                        .expect("Expected column 271 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[52]
                        .into_string()
                        .cloned()
                        .expect("Expected column 52 to be a string!"),
                    ScriptValue: row
                        .columns[272]
                        .into_u32()
                        .copied()
                        .expect("Expected column 272 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[53]
                        .into_string()
                        .cloned()
                        .expect("Expected column 53 to be a string!"),
                    ScriptValue: row
                        .columns[273]
                        .into_u32()
                        .copied()
                        .expect("Expected column 273 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[54]
                        .into_string()
                        .cloned()
                        .expect("Expected column 54 to be a string!"),
                    ScriptValue: row
                        .columns[274]
                        .into_u32()
                        .copied()
                        .expect("Expected column 274 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[55]
                        .into_string()
                        .cloned()
                        .expect("Expected column 55 to be a string!"),
                    ScriptValue: row
                        .columns[275]
                        .into_u32()
                        .copied()
                        .expect("Expected column 275 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[56]
                        .into_string()
                        .cloned()
                        .expect("Expected column 56 to be a string!"),
                    ScriptValue: row
                        .columns[276]
                        .into_u32()
                        .copied()
                        .expect("Expected column 276 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[57]
                        .into_string()
                        .cloned()
                        .expect("Expected column 57 to be a string!"),
                    ScriptValue: row
                        .columns[277]
                        .into_u32()
                        .copied()
                        .expect("Expected column 277 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[58]
                        .into_string()
                        .cloned()
                        .expect("Expected column 58 to be a string!"),
                    ScriptValue: row
                        .columns[278]
                        .into_u32()
                        .copied()
                        .expect("Expected column 278 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[59]
                        .into_string()
                        .cloned()
                        .expect("Expected column 59 to be a string!"),
                    ScriptValue: row
                        .columns[279]
                        .into_u32()
                        .copied()
                        .expect("Expected column 279 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[60]
                        .into_string()
                        .cloned()
                        .expect("Expected column 60 to be a string!"),
                    ScriptValue: row
                        .columns[280]
                        .into_u32()
                        .copied()
                        .expect("Expected column 280 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[61]
                        .into_string()
                        .cloned()
                        .expect("Expected column 61 to be a string!"),
                    ScriptValue: row
                        .columns[281]
                        .into_u32()
                        .copied()
                        .expect("Expected column 281 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[62]
                        .into_string()
                        .cloned()
                        .expect("Expected column 62 to be a string!"),
                    ScriptValue: row
                        .columns[282]
                        .into_u32()
                        .copied()
                        .expect("Expected column 282 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[63]
                        .into_string()
                        .cloned()
                        .expect("Expected column 63 to be a string!"),
                    ScriptValue: row
                        .columns[283]
                        .into_u32()
                        .copied()
                        .expect("Expected column 283 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[64]
                        .into_string()
                        .cloned()
                        .expect("Expected column 64 to be a string!"),
                    ScriptValue: row
                        .columns[284]
                        .into_u32()
                        .copied()
                        .expect("Expected column 284 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[65]
                        .into_string()
                        .cloned()
                        .expect("Expected column 65 to be a string!"),
                    ScriptValue: row
                        .columns[285]
                        .into_u32()
                        .copied()
                        .expect("Expected column 285 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[66]
                        .into_string()
                        .cloned()
                        .expect("Expected column 66 to be a string!"),
                    ScriptValue: row
                        .columns[286]
                        .into_u32()
                        .copied()
                        .expect("Expected column 286 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[67]
                        .into_string()
                        .cloned()
                        .expect("Expected column 67 to be a string!"),
                    ScriptValue: row
                        .columns[287]
                        .into_u32()
                        .copied()
                        .expect("Expected column 287 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[68]
                        .into_string()
                        .cloned()
                        .expect("Expected column 68 to be a string!"),
                    ScriptValue: row
                        .columns[288]
                        .into_u32()
                        .copied()
                        .expect("Expected column 288 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[69]
                        .into_string()
                        .cloned()
                        .expect("Expected column 69 to be a string!"),
                    ScriptValue: row
                        .columns[289]
                        .into_u32()
                        .copied()
                        .expect("Expected column 289 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[70]
                        .into_string()
                        .cloned()
                        .expect("Expected column 70 to be a string!"),
                    ScriptValue: row
                        .columns[290]
                        .into_u32()
                        .copied()
                        .expect("Expected column 290 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[71]
                        .into_string()
                        .cloned()
                        .expect("Expected column 71 to be a string!"),
                    ScriptValue: row
                        .columns[291]
                        .into_u32()
                        .copied()
                        .expect("Expected column 291 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[72]
                        .into_string()
                        .cloned()
                        .expect("Expected column 72 to be a string!"),
                    ScriptValue: row
                        .columns[292]
                        .into_u32()
                        .copied()
                        .expect("Expected column 292 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[73]
                        .into_string()
                        .cloned()
                        .expect("Expected column 73 to be a string!"),
                    ScriptValue: row
                        .columns[293]
                        .into_u32()
                        .copied()
                        .expect("Expected column 293 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[74]
                        .into_string()
                        .cloned()
                        .expect("Expected column 74 to be a string!"),
                    ScriptValue: row
                        .columns[294]
                        .into_u32()
                        .copied()
                        .expect("Expected column 294 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[75]
                        .into_string()
                        .cloned()
                        .expect("Expected column 75 to be a string!"),
                    ScriptValue: row
                        .columns[295]
                        .into_u32()
                        .copied()
                        .expect("Expected column 295 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[76]
                        .into_string()
                        .cloned()
                        .expect("Expected column 76 to be a string!"),
                    ScriptValue: row
                        .columns[296]
                        .into_u32()
                        .copied()
                        .expect("Expected column 296 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[77]
                        .into_string()
                        .cloned()
                        .expect("Expected column 77 to be a string!"),
                    ScriptValue: row
                        .columns[297]
                        .into_u32()
                        .copied()
                        .expect("Expected column 297 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[78]
                        .into_string()
                        .cloned()
                        .expect("Expected column 78 to be a string!"),
                    ScriptValue: row
                        .columns[298]
                        .into_u32()
                        .copied()
                        .expect("Expected column 298 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[79]
                        .into_string()
                        .cloned()
                        .expect("Expected column 79 to be a string!"),
                    ScriptValue: row
                        .columns[299]
                        .into_u32()
                        .copied()
                        .expect("Expected column 299 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[80]
                        .into_string()
                        .cloned()
                        .expect("Expected column 80 to be a string!"),
                    ScriptValue: row
                        .columns[300]
                        .into_u32()
                        .copied()
                        .expect("Expected column 300 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[81]
                        .into_string()
                        .cloned()
                        .expect("Expected column 81 to be a string!"),
                    ScriptValue: row
                        .columns[301]
                        .into_u32()
                        .copied()
                        .expect("Expected column 301 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[82]
                        .into_string()
                        .cloned()
                        .expect("Expected column 82 to be a string!"),
                    ScriptValue: row
                        .columns[302]
                        .into_u32()
                        .copied()
                        .expect("Expected column 302 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[83]
                        .into_string()
                        .cloned()
                        .expect("Expected column 83 to be a string!"),
                    ScriptValue: row
                        .columns[303]
                        .into_u32()
                        .copied()
                        .expect("Expected column 303 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[84]
                        .into_string()
                        .cloned()
                        .expect("Expected column 84 to be a string!"),
                    ScriptValue: row
                        .columns[304]
                        .into_u32()
                        .copied()
                        .expect("Expected column 304 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[85]
                        .into_string()
                        .cloned()
                        .expect("Expected column 85 to be a string!"),
                    ScriptValue: row
                        .columns[305]
                        .into_u32()
                        .copied()
                        .expect("Expected column 305 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[86]
                        .into_string()
                        .cloned()
                        .expect("Expected column 86 to be a string!"),
                    ScriptValue: row
                        .columns[306]
                        .into_u32()
                        .copied()
                        .expect("Expected column 306 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[87]
                        .into_string()
                        .cloned()
                        .expect("Expected column 87 to be a string!"),
                    ScriptValue: row
                        .columns[307]
                        .into_u32()
                        .copied()
                        .expect("Expected column 307 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[88]
                        .into_string()
                        .cloned()
                        .expect("Expected column 88 to be a string!"),
                    ScriptValue: row
                        .columns[308]
                        .into_u32()
                        .copied()
                        .expect("Expected column 308 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[89]
                        .into_string()
                        .cloned()
                        .expect("Expected column 89 to be a string!"),
                    ScriptValue: row
                        .columns[309]
                        .into_u32()
                        .copied()
                        .expect("Expected column 309 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[90]
                        .into_string()
                        .cloned()
                        .expect("Expected column 90 to be a string!"),
                    ScriptValue: row
                        .columns[310]
                        .into_u32()
                        .copied()
                        .expect("Expected column 310 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[91]
                        .into_string()
                        .cloned()
                        .expect("Expected column 91 to be a string!"),
                    ScriptValue: row
                        .columns[311]
                        .into_u32()
                        .copied()
                        .expect("Expected column 311 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[92]
                        .into_string()
                        .cloned()
                        .expect("Expected column 92 to be a string!"),
                    ScriptValue: row
                        .columns[312]
                        .into_u32()
                        .copied()
                        .expect("Expected column 312 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[93]
                        .into_string()
                        .cloned()
                        .expect("Expected column 93 to be a string!"),
                    ScriptValue: row
                        .columns[313]
                        .into_u32()
                        .copied()
                        .expect("Expected column 313 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[94]
                        .into_string()
                        .cloned()
                        .expect("Expected column 94 to be a string!"),
                    ScriptValue: row
                        .columns[314]
                        .into_u32()
                        .copied()
                        .expect("Expected column 314 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[95]
                        .into_string()
                        .cloned()
                        .expect("Expected column 95 to be a string!"),
                    ScriptValue: row
                        .columns[315]
                        .into_u32()
                        .copied()
                        .expect("Expected column 315 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[96]
                        .into_string()
                        .cloned()
                        .expect("Expected column 96 to be a string!"),
                    ScriptValue: row
                        .columns[316]
                        .into_u32()
                        .copied()
                        .expect("Expected column 316 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[97]
                        .into_string()
                        .cloned()
                        .expect("Expected column 97 to be a string!"),
                    ScriptValue: row
                        .columns[317]
                        .into_u32()
                        .copied()
                        .expect("Expected column 317 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[98]
                        .into_string()
                        .cloned()
                        .expect("Expected column 98 to be a string!"),
                    ScriptValue: row
                        .columns[318]
                        .into_u32()
                        .copied()
                        .expect("Expected column 318 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[99]
                        .into_string()
                        .cloned()
                        .expect("Expected column 99 to be a string!"),
                    ScriptValue: row
                        .columns[319]
                        .into_u32()
                        .copied()
                        .expect("Expected column 319 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[100]
                        .into_string()
                        .cloned()
                        .expect("Expected column 100 to be a string!"),
                    ScriptValue: row
                        .columns[320]
                        .into_u32()
                        .copied()
                        .expect("Expected column 320 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[101]
                        .into_string()
                        .cloned()
                        .expect("Expected column 101 to be a string!"),
                    ScriptValue: row
                        .columns[321]
                        .into_u32()
                        .copied()
                        .expect("Expected column 321 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[102]
                        .into_string()
                        .cloned()
                        .expect("Expected column 102 to be a string!"),
                    ScriptValue: row
                        .columns[322]
                        .into_u32()
                        .copied()
                        .expect("Expected column 322 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[103]
                        .into_string()
                        .cloned()
                        .expect("Expected column 103 to be a string!"),
                    ScriptValue: row
                        .columns[323]
                        .into_u32()
                        .copied()
                        .expect("Expected column 323 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[104]
                        .into_string()
                        .cloned()
                        .expect("Expected column 104 to be a string!"),
                    ScriptValue: row
                        .columns[324]
                        .into_u32()
                        .copied()
                        .expect("Expected column 324 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[105]
                        .into_string()
                        .cloned()
                        .expect("Expected column 105 to be a string!"),
                    ScriptValue: row
                        .columns[325]
                        .into_u32()
                        .copied()
                        .expect("Expected column 325 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[106]
                        .into_string()
                        .cloned()
                        .expect("Expected column 106 to be a string!"),
                    ScriptValue: row
                        .columns[326]
                        .into_u32()
                        .copied()
                        .expect("Expected column 326 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[107]
                        .into_string()
                        .cloned()
                        .expect("Expected column 107 to be a string!"),
                    ScriptValue: row
                        .columns[327]
                        .into_u32()
                        .copied()
                        .expect("Expected column 327 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[108]
                        .into_string()
                        .cloned()
                        .expect("Expected column 108 to be a string!"),
                    ScriptValue: row
                        .columns[328]
                        .into_u32()
                        .copied()
                        .expect("Expected column 328 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[109]
                        .into_string()
                        .cloned()
                        .expect("Expected column 109 to be a string!"),
                    ScriptValue: row
                        .columns[329]
                        .into_u32()
                        .copied()
                        .expect("Expected column 329 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[110]
                        .into_string()
                        .cloned()
                        .expect("Expected column 110 to be a string!"),
                    ScriptValue: row
                        .columns[330]
                        .into_u32()
                        .copied()
                        .expect("Expected column 330 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[111]
                        .into_string()
                        .cloned()
                        .expect("Expected column 111 to be a string!"),
                    ScriptValue: row
                        .columns[331]
                        .into_u32()
                        .copied()
                        .expect("Expected column 331 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[112]
                        .into_string()
                        .cloned()
                        .expect("Expected column 112 to be a string!"),
                    ScriptValue: row
                        .columns[332]
                        .into_u32()
                        .copied()
                        .expect("Expected column 332 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[113]
                        .into_string()
                        .cloned()
                        .expect("Expected column 113 to be a string!"),
                    ScriptValue: row
                        .columns[333]
                        .into_u32()
                        .copied()
                        .expect("Expected column 333 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[114]
                        .into_string()
                        .cloned()
                        .expect("Expected column 114 to be a string!"),
                    ScriptValue: row
                        .columns[334]
                        .into_u32()
                        .copied()
                        .expect("Expected column 334 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[115]
                        .into_string()
                        .cloned()
                        .expect("Expected column 115 to be a string!"),
                    ScriptValue: row
                        .columns[335]
                        .into_u32()
                        .copied()
                        .expect("Expected column 335 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[116]
                        .into_string()
                        .cloned()
                        .expect("Expected column 116 to be a string!"),
                    ScriptValue: row
                        .columns[336]
                        .into_u32()
                        .copied()
                        .expect("Expected column 336 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[117]
                        .into_string()
                        .cloned()
                        .expect("Expected column 117 to be a string!"),
                    ScriptValue: row
                        .columns[337]
                        .into_u32()
                        .copied()
                        .expect("Expected column 337 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[118]
                        .into_string()
                        .cloned()
                        .expect("Expected column 118 to be a string!"),
                    ScriptValue: row
                        .columns[338]
                        .into_u32()
                        .copied()
                        .expect("Expected column 338 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[119]
                        .into_string()
                        .cloned()
                        .expect("Expected column 119 to be a string!"),
                    ScriptValue: row
                        .columns[339]
                        .into_u32()
                        .copied()
                        .expect("Expected column 339 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[120]
                        .into_string()
                        .cloned()
                        .expect("Expected column 120 to be a string!"),
                    ScriptValue: row
                        .columns[340]
                        .into_u32()
                        .copied()
                        .expect("Expected column 340 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[121]
                        .into_string()
                        .cloned()
                        .expect("Expected column 121 to be a string!"),
                    ScriptValue: row
                        .columns[341]
                        .into_u32()
                        .copied()
                        .expect("Expected column 341 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[122]
                        .into_string()
                        .cloned()
                        .expect("Expected column 122 to be a string!"),
                    ScriptValue: row
                        .columns[342]
                        .into_u32()
                        .copied()
                        .expect("Expected column 342 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[123]
                        .into_string()
                        .cloned()
                        .expect("Expected column 123 to be a string!"),
                    ScriptValue: row
                        .columns[343]
                        .into_u32()
                        .copied()
                        .expect("Expected column 343 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[124]
                        .into_string()
                        .cloned()
                        .expect("Expected column 124 to be a string!"),
                    ScriptValue: row
                        .columns[344]
                        .into_u32()
                        .copied()
                        .expect("Expected column 344 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[125]
                        .into_string()
                        .cloned()
                        .expect("Expected column 125 to be a string!"),
                    ScriptValue: row
                        .columns[345]
                        .into_u32()
                        .copied()
                        .expect("Expected column 345 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[126]
                        .into_string()
                        .cloned()
                        .expect("Expected column 126 to be a string!"),
                    ScriptValue: row
                        .columns[346]
                        .into_u32()
                        .copied()
                        .expect("Expected column 346 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[127]
                        .into_string()
                        .cloned()
                        .expect("Expected column 127 to be a string!"),
                    ScriptValue: row
                        .columns[347]
                        .into_u32()
                        .copied()
                        .expect("Expected column 347 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[128]
                        .into_string()
                        .cloned()
                        .expect("Expected column 128 to be a string!"),
                    ScriptValue: row
                        .columns[348]
                        .into_u32()
                        .copied()
                        .expect("Expected column 348 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[129]
                        .into_string()
                        .cloned()
                        .expect("Expected column 129 to be a string!"),
                    ScriptValue: row
                        .columns[349]
                        .into_u32()
                        .copied()
                        .expect("Expected column 349 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[130]
                        .into_string()
                        .cloned()
                        .expect("Expected column 130 to be a string!"),
                    ScriptValue: row
                        .columns[350]
                        .into_u32()
                        .copied()
                        .expect("Expected column 350 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[131]
                        .into_string()
                        .cloned()
                        .expect("Expected column 131 to be a string!"),
                    ScriptValue: row
                        .columns[351]
                        .into_u32()
                        .copied()
                        .expect("Expected column 351 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[132]
                        .into_string()
                        .cloned()
                        .expect("Expected column 132 to be a string!"),
                    ScriptValue: row
                        .columns[352]
                        .into_u32()
                        .copied()
                        .expect("Expected column 352 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[133]
                        .into_string()
                        .cloned()
                        .expect("Expected column 133 to be a string!"),
                    ScriptValue: row
                        .columns[353]
                        .into_u32()
                        .copied()
                        .expect("Expected column 353 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[134]
                        .into_string()
                        .cloned()
                        .expect("Expected column 134 to be a string!"),
                    ScriptValue: row
                        .columns[354]
                        .into_u32()
                        .copied()
                        .expect("Expected column 354 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[135]
                        .into_string()
                        .cloned()
                        .expect("Expected column 135 to be a string!"),
                    ScriptValue: row
                        .columns[355]
                        .into_u32()
                        .copied()
                        .expect("Expected column 355 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[136]
                        .into_string()
                        .cloned()
                        .expect("Expected column 136 to be a string!"),
                    ScriptValue: row
                        .columns[356]
                        .into_u32()
                        .copied()
                        .expect("Expected column 356 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[137]
                        .into_string()
                        .cloned()
                        .expect("Expected column 137 to be a string!"),
                    ScriptValue: row
                        .columns[357]
                        .into_u32()
                        .copied()
                        .expect("Expected column 357 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[138]
                        .into_string()
                        .cloned()
                        .expect("Expected column 138 to be a string!"),
                    ScriptValue: row
                        .columns[358]
                        .into_u32()
                        .copied()
                        .expect("Expected column 358 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[139]
                        .into_string()
                        .cloned()
                        .expect("Expected column 139 to be a string!"),
                    ScriptValue: row
                        .columns[359]
                        .into_u32()
                        .copied()
                        .expect("Expected column 359 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[140]
                        .into_string()
                        .cloned()
                        .expect("Expected column 140 to be a string!"),
                    ScriptValue: row
                        .columns[360]
                        .into_u32()
                        .copied()
                        .expect("Expected column 360 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[141]
                        .into_string()
                        .cloned()
                        .expect("Expected column 141 to be a string!"),
                    ScriptValue: row
                        .columns[361]
                        .into_u32()
                        .copied()
                        .expect("Expected column 361 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[142]
                        .into_string()
                        .cloned()
                        .expect("Expected column 142 to be a string!"),
                    ScriptValue: row
                        .columns[362]
                        .into_u32()
                        .copied()
                        .expect("Expected column 362 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[143]
                        .into_string()
                        .cloned()
                        .expect("Expected column 143 to be a string!"),
                    ScriptValue: row
                        .columns[363]
                        .into_u32()
                        .copied()
                        .expect("Expected column 363 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[144]
                        .into_string()
                        .cloned()
                        .expect("Expected column 144 to be a string!"),
                    ScriptValue: row
                        .columns[364]
                        .into_u32()
                        .copied()
                        .expect("Expected column 364 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[145]
                        .into_string()
                        .cloned()
                        .expect("Expected column 145 to be a string!"),
                    ScriptValue: row
                        .columns[365]
                        .into_u32()
                        .copied()
                        .expect("Expected column 365 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[146]
                        .into_string()
                        .cloned()
                        .expect("Expected column 146 to be a string!"),
                    ScriptValue: row
                        .columns[366]
                        .into_u32()
                        .copied()
                        .expect("Expected column 366 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[147]
                        .into_string()
                        .cloned()
                        .expect("Expected column 147 to be a string!"),
                    ScriptValue: row
                        .columns[367]
                        .into_u32()
                        .copied()
                        .expect("Expected column 367 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[148]
                        .into_string()
                        .cloned()
                        .expect("Expected column 148 to be a string!"),
                    ScriptValue: row
                        .columns[368]
                        .into_u32()
                        .copied()
                        .expect("Expected column 368 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[149]
                        .into_string()
                        .cloned()
                        .expect("Expected column 149 to be a string!"),
                    ScriptValue: row
                        .columns[369]
                        .into_u32()
                        .copied()
                        .expect("Expected column 369 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[150]
                        .into_string()
                        .cloned()
                        .expect("Expected column 150 to be a string!"),
                    ScriptValue: row
                        .columns[370]
                        .into_u32()
                        .copied()
                        .expect("Expected column 370 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[151]
                        .into_string()
                        .cloned()
                        .expect("Expected column 151 to be a string!"),
                    ScriptValue: row
                        .columns[371]
                        .into_u32()
                        .copied()
                        .expect("Expected column 371 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[152]
                        .into_string()
                        .cloned()
                        .expect("Expected column 152 to be a string!"),
                    ScriptValue: row
                        .columns[372]
                        .into_u32()
                        .copied()
                        .expect("Expected column 372 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[153]
                        .into_string()
                        .cloned()
                        .expect("Expected column 153 to be a string!"),
                    ScriptValue: row
                        .columns[373]
                        .into_u32()
                        .copied()
                        .expect("Expected column 373 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[154]
                        .into_string()
                        .cloned()
                        .expect("Expected column 154 to be a string!"),
                    ScriptValue: row
                        .columns[374]
                        .into_u32()
                        .copied()
                        .expect("Expected column 374 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[155]
                        .into_string()
                        .cloned()
                        .expect("Expected column 155 to be a string!"),
                    ScriptValue: row
                        .columns[375]
                        .into_u32()
                        .copied()
                        .expect("Expected column 375 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[156]
                        .into_string()
                        .cloned()
                        .expect("Expected column 156 to be a string!"),
                    ScriptValue: row
                        .columns[376]
                        .into_u32()
                        .copied()
                        .expect("Expected column 376 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[157]
                        .into_string()
                        .cloned()
                        .expect("Expected column 157 to be a string!"),
                    ScriptValue: row
                        .columns[377]
                        .into_u32()
                        .copied()
                        .expect("Expected column 377 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[158]
                        .into_string()
                        .cloned()
                        .expect("Expected column 158 to be a string!"),
                    ScriptValue: row
                        .columns[378]
                        .into_u32()
                        .copied()
                        .expect("Expected column 378 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[159]
                        .into_string()
                        .cloned()
                        .expect("Expected column 159 to be a string!"),
                    ScriptValue: row
                        .columns[379]
                        .into_u32()
                        .copied()
                        .expect("Expected column 379 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[160]
                        .into_string()
                        .cloned()
                        .expect("Expected column 160 to be a string!"),
                    ScriptValue: row
                        .columns[380]
                        .into_u32()
                        .copied()
                        .expect("Expected column 380 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[161]
                        .into_string()
                        .cloned()
                        .expect("Expected column 161 to be a string!"),
                    ScriptValue: row
                        .columns[381]
                        .into_u32()
                        .copied()
                        .expect("Expected column 381 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[162]
                        .into_string()
                        .cloned()
                        .expect("Expected column 162 to be a string!"),
                    ScriptValue: row
                        .columns[382]
                        .into_u32()
                        .copied()
                        .expect("Expected column 382 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[163]
                        .into_string()
                        .cloned()
                        .expect("Expected column 163 to be a string!"),
                    ScriptValue: row
                        .columns[383]
                        .into_u32()
                        .copied()
                        .expect("Expected column 383 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[164]
                        .into_string()
                        .cloned()
                        .expect("Expected column 164 to be a string!"),
                    ScriptValue: row
                        .columns[384]
                        .into_u32()
                        .copied()
                        .expect("Expected column 384 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[165]
                        .into_string()
                        .cloned()
                        .expect("Expected column 165 to be a string!"),
                    ScriptValue: row
                        .columns[385]
                        .into_u32()
                        .copied()
                        .expect("Expected column 385 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[166]
                        .into_string()
                        .cloned()
                        .expect("Expected column 166 to be a string!"),
                    ScriptValue: row
                        .columns[386]
                        .into_u32()
                        .copied()
                        .expect("Expected column 386 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[167]
                        .into_string()
                        .cloned()
                        .expect("Expected column 167 to be a string!"),
                    ScriptValue: row
                        .columns[387]
                        .into_u32()
                        .copied()
                        .expect("Expected column 387 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[168]
                        .into_string()
                        .cloned()
                        .expect("Expected column 168 to be a string!"),
                    ScriptValue: row
                        .columns[388]
                        .into_u32()
                        .copied()
                        .expect("Expected column 388 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[169]
                        .into_string()
                        .cloned()
                        .expect("Expected column 169 to be a string!"),
                    ScriptValue: row
                        .columns[389]
                        .into_u32()
                        .copied()
                        .expect("Expected column 389 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[170]
                        .into_string()
                        .cloned()
                        .expect("Expected column 170 to be a string!"),
                    ScriptValue: row
                        .columns[390]
                        .into_u32()
                        .copied()
                        .expect("Expected column 390 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[171]
                        .into_string()
                        .cloned()
                        .expect("Expected column 171 to be a string!"),
                    ScriptValue: row
                        .columns[391]
                        .into_u32()
                        .copied()
                        .expect("Expected column 391 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[172]
                        .into_string()
                        .cloned()
                        .expect("Expected column 172 to be a string!"),
                    ScriptValue: row
                        .columns[392]
                        .into_u32()
                        .copied()
                        .expect("Expected column 392 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[173]
                        .into_string()
                        .cloned()
                        .expect("Expected column 173 to be a string!"),
                    ScriptValue: row
                        .columns[393]
                        .into_u32()
                        .copied()
                        .expect("Expected column 393 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[174]
                        .into_string()
                        .cloned()
                        .expect("Expected column 174 to be a string!"),
                    ScriptValue: row
                        .columns[394]
                        .into_u32()
                        .copied()
                        .expect("Expected column 394 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[175]
                        .into_string()
                        .cloned()
                        .expect("Expected column 175 to be a string!"),
                    ScriptValue: row
                        .columns[395]
                        .into_u32()
                        .copied()
                        .expect("Expected column 395 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[176]
                        .into_string()
                        .cloned()
                        .expect("Expected column 176 to be a string!"),
                    ScriptValue: row
                        .columns[396]
                        .into_u32()
                        .copied()
                        .expect("Expected column 396 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[177]
                        .into_string()
                        .cloned()
                        .expect("Expected column 177 to be a string!"),
                    ScriptValue: row
                        .columns[397]
                        .into_u32()
                        .copied()
                        .expect("Expected column 397 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[178]
                        .into_string()
                        .cloned()
                        .expect("Expected column 178 to be a string!"),
                    ScriptValue: row
                        .columns[398]
                        .into_u32()
                        .copied()
                        .expect("Expected column 398 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[179]
                        .into_string()
                        .cloned()
                        .expect("Expected column 179 to be a string!"),
                    ScriptValue: row
                        .columns[399]
                        .into_u32()
                        .copied()
                        .expect("Expected column 399 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[180]
                        .into_string()
                        .cloned()
                        .expect("Expected column 180 to be a string!"),
                    ScriptValue: row
                        .columns[400]
                        .into_u32()
                        .copied()
                        .expect("Expected column 400 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[181]
                        .into_string()
                        .cloned()
                        .expect("Expected column 181 to be a string!"),
                    ScriptValue: row
                        .columns[401]
                        .into_u32()
                        .copied()
                        .expect("Expected column 401 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[182]
                        .into_string()
                        .cloned()
                        .expect("Expected column 182 to be a string!"),
                    ScriptValue: row
                        .columns[402]
                        .into_u32()
                        .copied()
                        .expect("Expected column 402 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[183]
                        .into_string()
                        .cloned()
                        .expect("Expected column 183 to be a string!"),
                    ScriptValue: row
                        .columns[403]
                        .into_u32()
                        .copied()
                        .expect("Expected column 403 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[184]
                        .into_string()
                        .cloned()
                        .expect("Expected column 184 to be a string!"),
                    ScriptValue: row
                        .columns[404]
                        .into_u32()
                        .copied()
                        .expect("Expected column 404 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[185]
                        .into_string()
                        .cloned()
                        .expect("Expected column 185 to be a string!"),
                    ScriptValue: row
                        .columns[405]
                        .into_u32()
                        .copied()
                        .expect("Expected column 405 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[186]
                        .into_string()
                        .cloned()
                        .expect("Expected column 186 to be a string!"),
                    ScriptValue: row
                        .columns[406]
                        .into_u32()
                        .copied()
                        .expect("Expected column 406 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[187]
                        .into_string()
                        .cloned()
                        .expect("Expected column 187 to be a string!"),
                    ScriptValue: row
                        .columns[407]
                        .into_u32()
                        .copied()
                        .expect("Expected column 407 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[188]
                        .into_string()
                        .cloned()
                        .expect("Expected column 188 to be a string!"),
                    ScriptValue: row
                        .columns[408]
                        .into_u32()
                        .copied()
                        .expect("Expected column 408 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[189]
                        .into_string()
                        .cloned()
                        .expect("Expected column 189 to be a string!"),
                    ScriptValue: row
                        .columns[409]
                        .into_u32()
                        .copied()
                        .expect("Expected column 409 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[190]
                        .into_string()
                        .cloned()
                        .expect("Expected column 190 to be a string!"),
                    ScriptValue: row
                        .columns[410]
                        .into_u32()
                        .copied()
                        .expect("Expected column 410 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[191]
                        .into_string()
                        .cloned()
                        .expect("Expected column 191 to be a string!"),
                    ScriptValue: row
                        .columns[411]
                        .into_u32()
                        .copied()
                        .expect("Expected column 411 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[192]
                        .into_string()
                        .cloned()
                        .expect("Expected column 192 to be a string!"),
                    ScriptValue: row
                        .columns[412]
                        .into_u32()
                        .copied()
                        .expect("Expected column 412 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[193]
                        .into_string()
                        .cloned()
                        .expect("Expected column 193 to be a string!"),
                    ScriptValue: row
                        .columns[413]
                        .into_u32()
                        .copied()
                        .expect("Expected column 413 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[194]
                        .into_string()
                        .cloned()
                        .expect("Expected column 194 to be a string!"),
                    ScriptValue: row
                        .columns[414]
                        .into_u32()
                        .copied()
                        .expect("Expected column 414 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[195]
                        .into_string()
                        .cloned()
                        .expect("Expected column 195 to be a string!"),
                    ScriptValue: row
                        .columns[415]
                        .into_u32()
                        .copied()
                        .expect("Expected column 415 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[196]
                        .into_string()
                        .cloned()
                        .expect("Expected column 196 to be a string!"),
                    ScriptValue: row
                        .columns[416]
                        .into_u32()
                        .copied()
                        .expect("Expected column 416 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[197]
                        .into_string()
                        .cloned()
                        .expect("Expected column 197 to be a string!"),
                    ScriptValue: row
                        .columns[417]
                        .into_u32()
                        .copied()
                        .expect("Expected column 417 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[198]
                        .into_string()
                        .cloned()
                        .expect("Expected column 198 to be a string!"),
                    ScriptValue: row
                        .columns[418]
                        .into_u32()
                        .copied()
                        .expect("Expected column 418 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[199]
                        .into_string()
                        .cloned()
                        .expect("Expected column 199 to be a string!"),
                    ScriptValue: row
                        .columns[419]
                        .into_u32()
                        .copied()
                        .expect("Expected column 419 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[200]
                        .into_string()
                        .cloned()
                        .expect("Expected column 200 to be a string!"),
                    ScriptValue: row
                        .columns[420]
                        .into_u32()
                        .copied()
                        .expect("Expected column 420 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[201]
                        .into_string()
                        .cloned()
                        .expect("Expected column 201 to be a string!"),
                    ScriptValue: row
                        .columns[421]
                        .into_u32()
                        .copied()
                        .expect("Expected column 421 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[202]
                        .into_string()
                        .cloned()
                        .expect("Expected column 202 to be a string!"),
                    ScriptValue: row
                        .columns[422]
                        .into_u32()
                        .copied()
                        .expect("Expected column 422 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[203]
                        .into_string()
                        .cloned()
                        .expect("Expected column 203 to be a string!"),
                    ScriptValue: row
                        .columns[423]
                        .into_u32()
                        .copied()
                        .expect("Expected column 423 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[204]
                        .into_string()
                        .cloned()
                        .expect("Expected column 204 to be a string!"),
                    ScriptValue: row
                        .columns[424]
                        .into_u32()
                        .copied()
                        .expect("Expected column 424 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[205]
                        .into_string()
                        .cloned()
                        .expect("Expected column 205 to be a string!"),
                    ScriptValue: row
                        .columns[425]
                        .into_u32()
                        .copied()
                        .expect("Expected column 425 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[206]
                        .into_string()
                        .cloned()
                        .expect("Expected column 206 to be a string!"),
                    ScriptValue: row
                        .columns[426]
                        .into_u32()
                        .copied()
                        .expect("Expected column 426 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[207]
                        .into_string()
                        .cloned()
                        .expect("Expected column 207 to be a string!"),
                    ScriptValue: row
                        .columns[427]
                        .into_u32()
                        .copied()
                        .expect("Expected column 427 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[208]
                        .into_string()
                        .cloned()
                        .expect("Expected column 208 to be a string!"),
                    ScriptValue: row
                        .columns[428]
                        .into_u32()
                        .copied()
                        .expect("Expected column 428 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[209]
                        .into_string()
                        .cloned()
                        .expect("Expected column 209 to be a string!"),
                    ScriptValue: row
                        .columns[429]
                        .into_u32()
                        .copied()
                        .expect("Expected column 429 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[210]
                        .into_string()
                        .cloned()
                        .expect("Expected column 210 to be a string!"),
                    ScriptValue: row
                        .columns[430]
                        .into_u32()
                        .copied()
                        .expect("Expected column 430 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[211]
                        .into_string()
                        .cloned()
                        .expect("Expected column 211 to be a string!"),
                    ScriptValue: row
                        .columns[431]
                        .into_u32()
                        .copied()
                        .expect("Expected column 431 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[212]
                        .into_string()
                        .cloned()
                        .expect("Expected column 212 to be a string!"),
                    ScriptValue: row
                        .columns[432]
                        .into_u32()
                        .copied()
                        .expect("Expected column 432 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[213]
                        .into_string()
                        .cloned()
                        .expect("Expected column 213 to be a string!"),
                    ScriptValue: row
                        .columns[433]
                        .into_u32()
                        .copied()
                        .expect("Expected column 433 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[214]
                        .into_string()
                        .cloned()
                        .expect("Expected column 214 to be a string!"),
                    ScriptValue: row
                        .columns[434]
                        .into_u32()
                        .copied()
                        .expect("Expected column 434 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[215]
                        .into_string()
                        .cloned()
                        .expect("Expected column 215 to be a string!"),
                    ScriptValue: row
                        .columns[435]
                        .into_u32()
                        .copied()
                        .expect("Expected column 435 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[216]
                        .into_string()
                        .cloned()
                        .expect("Expected column 216 to be a string!"),
                    ScriptValue: row
                        .columns[436]
                        .into_u32()
                        .copied()
                        .expect("Expected column 436 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[217]
                        .into_string()
                        .cloned()
                        .expect("Expected column 217 to be a string!"),
                    ScriptValue: row
                        .columns[437]
                        .into_u32()
                        .copied()
                        .expect("Expected column 437 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[218]
                        .into_string()
                        .cloned()
                        .expect("Expected column 218 to be a string!"),
                    ScriptValue: row
                        .columns[438]
                        .into_u32()
                        .copied()
                        .expect("Expected column 438 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[219]
                        .into_string()
                        .cloned()
                        .expect("Expected column 219 to be a string!"),
                    ScriptValue: row
                        .columns[439]
                        .into_u32()
                        .copied()
                        .expect("Expected column 439 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[220]
                        .into_string()
                        .cloned()
                        .expect("Expected column 220 to be a string!"),
                    ScriptValue: row
                        .columns[440]
                        .into_u32()
                        .copied()
                        .expect("Expected column 440 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[221]
                        .into_string()
                        .cloned()
                        .expect("Expected column 221 to be a string!"),
                    ScriptValue: row
                        .columns[441]
                        .into_u32()
                        .copied()
                        .expect("Expected column 441 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[222]
                        .into_string()
                        .cloned()
                        .expect("Expected column 222 to be a string!"),
                    ScriptValue: row
                        .columns[442]
                        .into_u32()
                        .copied()
                        .expect("Expected column 442 to be a uint32!"),
                },
                QuestBattleParamsElement {
                    ScriptInstruction: row
                        .columns[223]
                        .into_string()
                        .cloned()
                        .expect("Expected column 223 to be a string!"),
                    ScriptValue: row
                        .columns[443]
                        .into_u32()
                        .copied()
                        .expect("Expected column 443 to be a uint32!"),
                },
            ],
            Quest: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            TimeLimit: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            LevelSync: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            QuestBattleScene: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a QuestBattleSheet {
    type Item = (u32, Vec<(u16, QuestBattleRow)>);
    type IntoIter = StructuredSheetIterator<'a, QuestBattleSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestBattleSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuestBattleRow {
    ///""
    pub QuestBattleParams: [QuestBattleParamsElement; 220],
    ///""
    pub Quest: i32,
    ///""
    pub TimeLimit: u16,
    ///""
    pub LevelSync: u16,
    ///""
    pub QuestBattleScene: u8,
}
