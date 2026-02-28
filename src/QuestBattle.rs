//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct QuestBattleParamsElement<'a> {
    pub ScriptInstruction: &'a Field,
    pub ScriptValue: &'a Field,
}
#[derive(Debug, Clone)]
pub struct QuestBattleSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl QuestBattleSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestBattle")?;
        let sheet = resolver.read_excel_sheet(&exh, "QuestBattle", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
impl<'a> StructuredSheet<'a> for QuestBattleSheet {
    type Row = QuestBattleRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a QuestBattleSheet {
    type Item = (u32, Vec<(u16, QuestBattleRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, QuestBattleSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestBattleSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestBattleRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> QuestBattleRow<'a> {
    pub fn QuestBattleParams(&'a self) -> [QuestBattleParamsElement<'a>; 220] {
        [
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[0]],
                ScriptValue: &self.row.columns[self.index_mapping[1]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[2]],
                ScriptValue: &self.row.columns[self.index_mapping[3]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[4]],
                ScriptValue: &self.row.columns[self.index_mapping[5]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[6]],
                ScriptValue: &self.row.columns[self.index_mapping[7]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[8]],
                ScriptValue: &self.row.columns[self.index_mapping[9]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[10]],
                ScriptValue: &self.row.columns[self.index_mapping[11]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[12]],
                ScriptValue: &self.row.columns[self.index_mapping[13]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[14]],
                ScriptValue: &self.row.columns[self.index_mapping[15]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[16]],
                ScriptValue: &self.row.columns[self.index_mapping[17]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[18]],
                ScriptValue: &self.row.columns[self.index_mapping[19]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[20]],
                ScriptValue: &self.row.columns[self.index_mapping[21]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[22]],
                ScriptValue: &self.row.columns[self.index_mapping[23]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[24]],
                ScriptValue: &self.row.columns[self.index_mapping[25]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[26]],
                ScriptValue: &self.row.columns[self.index_mapping[27]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[28]],
                ScriptValue: &self.row.columns[self.index_mapping[29]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[30]],
                ScriptValue: &self.row.columns[self.index_mapping[31]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[32]],
                ScriptValue: &self.row.columns[self.index_mapping[33]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[34]],
                ScriptValue: &self.row.columns[self.index_mapping[35]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[36]],
                ScriptValue: &self.row.columns[self.index_mapping[37]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[38]],
                ScriptValue: &self.row.columns[self.index_mapping[39]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[40]],
                ScriptValue: &self.row.columns[self.index_mapping[41]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[42]],
                ScriptValue: &self.row.columns[self.index_mapping[43]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[44]],
                ScriptValue: &self.row.columns[self.index_mapping[45]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[46]],
                ScriptValue: &self.row.columns[self.index_mapping[47]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[48]],
                ScriptValue: &self.row.columns[self.index_mapping[49]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[50]],
                ScriptValue: &self.row.columns[self.index_mapping[51]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[52]],
                ScriptValue: &self.row.columns[self.index_mapping[53]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[54]],
                ScriptValue: &self.row.columns[self.index_mapping[55]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[56]],
                ScriptValue: &self.row.columns[self.index_mapping[57]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[58]],
                ScriptValue: &self.row.columns[self.index_mapping[59]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[60]],
                ScriptValue: &self.row.columns[self.index_mapping[61]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[62]],
                ScriptValue: &self.row.columns[self.index_mapping[63]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[64]],
                ScriptValue: &self.row.columns[self.index_mapping[65]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[66]],
                ScriptValue: &self.row.columns[self.index_mapping[67]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[68]],
                ScriptValue: &self.row.columns[self.index_mapping[69]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[70]],
                ScriptValue: &self.row.columns[self.index_mapping[71]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[72]],
                ScriptValue: &self.row.columns[self.index_mapping[73]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[74]],
                ScriptValue: &self.row.columns[self.index_mapping[75]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[76]],
                ScriptValue: &self.row.columns[self.index_mapping[77]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[78]],
                ScriptValue: &self.row.columns[self.index_mapping[79]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[80]],
                ScriptValue: &self.row.columns[self.index_mapping[81]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[82]],
                ScriptValue: &self.row.columns[self.index_mapping[83]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[84]],
                ScriptValue: &self.row.columns[self.index_mapping[85]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[86]],
                ScriptValue: &self.row.columns[self.index_mapping[87]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[88]],
                ScriptValue: &self.row.columns[self.index_mapping[89]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[90]],
                ScriptValue: &self.row.columns[self.index_mapping[91]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[92]],
                ScriptValue: &self.row.columns[self.index_mapping[93]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[94]],
                ScriptValue: &self.row.columns[self.index_mapping[95]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[96]],
                ScriptValue: &self.row.columns[self.index_mapping[97]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[98]],
                ScriptValue: &self.row.columns[self.index_mapping[99]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[100]],
                ScriptValue: &self.row.columns[self.index_mapping[101]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[102]],
                ScriptValue: &self.row.columns[self.index_mapping[103]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[104]],
                ScriptValue: &self.row.columns[self.index_mapping[105]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[106]],
                ScriptValue: &self.row.columns[self.index_mapping[107]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[108]],
                ScriptValue: &self.row.columns[self.index_mapping[109]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[110]],
                ScriptValue: &self.row.columns[self.index_mapping[111]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[112]],
                ScriptValue: &self.row.columns[self.index_mapping[113]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[114]],
                ScriptValue: &self.row.columns[self.index_mapping[115]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[116]],
                ScriptValue: &self.row.columns[self.index_mapping[117]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[118]],
                ScriptValue: &self.row.columns[self.index_mapping[119]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[120]],
                ScriptValue: &self.row.columns[self.index_mapping[121]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[122]],
                ScriptValue: &self.row.columns[self.index_mapping[123]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[124]],
                ScriptValue: &self.row.columns[self.index_mapping[125]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[126]],
                ScriptValue: &self.row.columns[self.index_mapping[127]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[128]],
                ScriptValue: &self.row.columns[self.index_mapping[129]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[130]],
                ScriptValue: &self.row.columns[self.index_mapping[131]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[132]],
                ScriptValue: &self.row.columns[self.index_mapping[133]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[134]],
                ScriptValue: &self.row.columns[self.index_mapping[135]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[136]],
                ScriptValue: &self.row.columns[self.index_mapping[137]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[138]],
                ScriptValue: &self.row.columns[self.index_mapping[139]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[140]],
                ScriptValue: &self.row.columns[self.index_mapping[141]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[142]],
                ScriptValue: &self.row.columns[self.index_mapping[143]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[144]],
                ScriptValue: &self.row.columns[self.index_mapping[145]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[146]],
                ScriptValue: &self.row.columns[self.index_mapping[147]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[148]],
                ScriptValue: &self.row.columns[self.index_mapping[149]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[150]],
                ScriptValue: &self.row.columns[self.index_mapping[151]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[152]],
                ScriptValue: &self.row.columns[self.index_mapping[153]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[154]],
                ScriptValue: &self.row.columns[self.index_mapping[155]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[156]],
                ScriptValue: &self.row.columns[self.index_mapping[157]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[158]],
                ScriptValue: &self.row.columns[self.index_mapping[159]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[160]],
                ScriptValue: &self.row.columns[self.index_mapping[161]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[162]],
                ScriptValue: &self.row.columns[self.index_mapping[163]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[164]],
                ScriptValue: &self.row.columns[self.index_mapping[165]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[166]],
                ScriptValue: &self.row.columns[self.index_mapping[167]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[168]],
                ScriptValue: &self.row.columns[self.index_mapping[169]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[170]],
                ScriptValue: &self.row.columns[self.index_mapping[171]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[172]],
                ScriptValue: &self.row.columns[self.index_mapping[173]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[174]],
                ScriptValue: &self.row.columns[self.index_mapping[175]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[176]],
                ScriptValue: &self.row.columns[self.index_mapping[177]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[178]],
                ScriptValue: &self.row.columns[self.index_mapping[179]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[180]],
                ScriptValue: &self.row.columns[self.index_mapping[181]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[182]],
                ScriptValue: &self.row.columns[self.index_mapping[183]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[184]],
                ScriptValue: &self.row.columns[self.index_mapping[185]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[186]],
                ScriptValue: &self.row.columns[self.index_mapping[187]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[188]],
                ScriptValue: &self.row.columns[self.index_mapping[189]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[190]],
                ScriptValue: &self.row.columns[self.index_mapping[191]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[192]],
                ScriptValue: &self.row.columns[self.index_mapping[193]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[194]],
                ScriptValue: &self.row.columns[self.index_mapping[195]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[196]],
                ScriptValue: &self.row.columns[self.index_mapping[197]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[198]],
                ScriptValue: &self.row.columns[self.index_mapping[199]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[200]],
                ScriptValue: &self.row.columns[self.index_mapping[201]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[202]],
                ScriptValue: &self.row.columns[self.index_mapping[203]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[204]],
                ScriptValue: &self.row.columns[self.index_mapping[205]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[206]],
                ScriptValue: &self.row.columns[self.index_mapping[207]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[208]],
                ScriptValue: &self.row.columns[self.index_mapping[209]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[210]],
                ScriptValue: &self.row.columns[self.index_mapping[211]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[212]],
                ScriptValue: &self.row.columns[self.index_mapping[213]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[214]],
                ScriptValue: &self.row.columns[self.index_mapping[215]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[216]],
                ScriptValue: &self.row.columns[self.index_mapping[217]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[218]],
                ScriptValue: &self.row.columns[self.index_mapping[219]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[220]],
                ScriptValue: &self.row.columns[self.index_mapping[221]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[222]],
                ScriptValue: &self.row.columns[self.index_mapping[223]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[224]],
                ScriptValue: &self.row.columns[self.index_mapping[225]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[226]],
                ScriptValue: &self.row.columns[self.index_mapping[227]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[228]],
                ScriptValue: &self.row.columns[self.index_mapping[229]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[230]],
                ScriptValue: &self.row.columns[self.index_mapping[231]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[232]],
                ScriptValue: &self.row.columns[self.index_mapping[233]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[234]],
                ScriptValue: &self.row.columns[self.index_mapping[235]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[236]],
                ScriptValue: &self.row.columns[self.index_mapping[237]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[238]],
                ScriptValue: &self.row.columns[self.index_mapping[239]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[240]],
                ScriptValue: &self.row.columns[self.index_mapping[241]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[242]],
                ScriptValue: &self.row.columns[self.index_mapping[243]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[244]],
                ScriptValue: &self.row.columns[self.index_mapping[245]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[246]],
                ScriptValue: &self.row.columns[self.index_mapping[247]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[248]],
                ScriptValue: &self.row.columns[self.index_mapping[249]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[250]],
                ScriptValue: &self.row.columns[self.index_mapping[251]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[252]],
                ScriptValue: &self.row.columns[self.index_mapping[253]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[254]],
                ScriptValue: &self.row.columns[self.index_mapping[255]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[256]],
                ScriptValue: &self.row.columns[self.index_mapping[257]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[258]],
                ScriptValue: &self.row.columns[self.index_mapping[259]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[260]],
                ScriptValue: &self.row.columns[self.index_mapping[261]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[262]],
                ScriptValue: &self.row.columns[self.index_mapping[263]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[264]],
                ScriptValue: &self.row.columns[self.index_mapping[265]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[266]],
                ScriptValue: &self.row.columns[self.index_mapping[267]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[268]],
                ScriptValue: &self.row.columns[self.index_mapping[269]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[270]],
                ScriptValue: &self.row.columns[self.index_mapping[271]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[272]],
                ScriptValue: &self.row.columns[self.index_mapping[273]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[274]],
                ScriptValue: &self.row.columns[self.index_mapping[275]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[276]],
                ScriptValue: &self.row.columns[self.index_mapping[277]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[278]],
                ScriptValue: &self.row.columns[self.index_mapping[279]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[280]],
                ScriptValue: &self.row.columns[self.index_mapping[281]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[282]],
                ScriptValue: &self.row.columns[self.index_mapping[283]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[284]],
                ScriptValue: &self.row.columns[self.index_mapping[285]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[286]],
                ScriptValue: &self.row.columns[self.index_mapping[287]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[288]],
                ScriptValue: &self.row.columns[self.index_mapping[289]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[290]],
                ScriptValue: &self.row.columns[self.index_mapping[291]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[292]],
                ScriptValue: &self.row.columns[self.index_mapping[293]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[294]],
                ScriptValue: &self.row.columns[self.index_mapping[295]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[296]],
                ScriptValue: &self.row.columns[self.index_mapping[297]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[298]],
                ScriptValue: &self.row.columns[self.index_mapping[299]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[300]],
                ScriptValue: &self.row.columns[self.index_mapping[301]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[302]],
                ScriptValue: &self.row.columns[self.index_mapping[303]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[304]],
                ScriptValue: &self.row.columns[self.index_mapping[305]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[306]],
                ScriptValue: &self.row.columns[self.index_mapping[307]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[308]],
                ScriptValue: &self.row.columns[self.index_mapping[309]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[310]],
                ScriptValue: &self.row.columns[self.index_mapping[311]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[312]],
                ScriptValue: &self.row.columns[self.index_mapping[313]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[314]],
                ScriptValue: &self.row.columns[self.index_mapping[315]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[316]],
                ScriptValue: &self.row.columns[self.index_mapping[317]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[318]],
                ScriptValue: &self.row.columns[self.index_mapping[319]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[320]],
                ScriptValue: &self.row.columns[self.index_mapping[321]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[322]],
                ScriptValue: &self.row.columns[self.index_mapping[323]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[324]],
                ScriptValue: &self.row.columns[self.index_mapping[325]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[326]],
                ScriptValue: &self.row.columns[self.index_mapping[327]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[328]],
                ScriptValue: &self.row.columns[self.index_mapping[329]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[330]],
                ScriptValue: &self.row.columns[self.index_mapping[331]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[332]],
                ScriptValue: &self.row.columns[self.index_mapping[333]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[334]],
                ScriptValue: &self.row.columns[self.index_mapping[335]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[336]],
                ScriptValue: &self.row.columns[self.index_mapping[337]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[338]],
                ScriptValue: &self.row.columns[self.index_mapping[339]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[340]],
                ScriptValue: &self.row.columns[self.index_mapping[341]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[342]],
                ScriptValue: &self.row.columns[self.index_mapping[343]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[344]],
                ScriptValue: &self.row.columns[self.index_mapping[345]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[346]],
                ScriptValue: &self.row.columns[self.index_mapping[347]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[348]],
                ScriptValue: &self.row.columns[self.index_mapping[349]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[350]],
                ScriptValue: &self.row.columns[self.index_mapping[351]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[352]],
                ScriptValue: &self.row.columns[self.index_mapping[353]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[354]],
                ScriptValue: &self.row.columns[self.index_mapping[355]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[356]],
                ScriptValue: &self.row.columns[self.index_mapping[357]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[358]],
                ScriptValue: &self.row.columns[self.index_mapping[359]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[360]],
                ScriptValue: &self.row.columns[self.index_mapping[361]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[362]],
                ScriptValue: &self.row.columns[self.index_mapping[363]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[364]],
                ScriptValue: &self.row.columns[self.index_mapping[365]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[366]],
                ScriptValue: &self.row.columns[self.index_mapping[367]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[368]],
                ScriptValue: &self.row.columns[self.index_mapping[369]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[370]],
                ScriptValue: &self.row.columns[self.index_mapping[371]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[372]],
                ScriptValue: &self.row.columns[self.index_mapping[373]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[374]],
                ScriptValue: &self.row.columns[self.index_mapping[375]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[376]],
                ScriptValue: &self.row.columns[self.index_mapping[377]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[378]],
                ScriptValue: &self.row.columns[self.index_mapping[379]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[380]],
                ScriptValue: &self.row.columns[self.index_mapping[381]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[382]],
                ScriptValue: &self.row.columns[self.index_mapping[383]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[384]],
                ScriptValue: &self.row.columns[self.index_mapping[385]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[386]],
                ScriptValue: &self.row.columns[self.index_mapping[387]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[388]],
                ScriptValue: &self.row.columns[self.index_mapping[389]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[390]],
                ScriptValue: &self.row.columns[self.index_mapping[391]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[392]],
                ScriptValue: &self.row.columns[self.index_mapping[393]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[394]],
                ScriptValue: &self.row.columns[self.index_mapping[395]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[396]],
                ScriptValue: &self.row.columns[self.index_mapping[397]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[398]],
                ScriptValue: &self.row.columns[self.index_mapping[399]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[400]],
                ScriptValue: &self.row.columns[self.index_mapping[401]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[402]],
                ScriptValue: &self.row.columns[self.index_mapping[403]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[404]],
                ScriptValue: &self.row.columns[self.index_mapping[405]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[406]],
                ScriptValue: &self.row.columns[self.index_mapping[407]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[408]],
                ScriptValue: &self.row.columns[self.index_mapping[409]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[410]],
                ScriptValue: &self.row.columns[self.index_mapping[411]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[412]],
                ScriptValue: &self.row.columns[self.index_mapping[413]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[414]],
                ScriptValue: &self.row.columns[self.index_mapping[415]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[416]],
                ScriptValue: &self.row.columns[self.index_mapping[417]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[418]],
                ScriptValue: &self.row.columns[self.index_mapping[419]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[420]],
                ScriptValue: &self.row.columns[self.index_mapping[421]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[422]],
                ScriptValue: &self.row.columns[self.index_mapping[423]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[424]],
                ScriptValue: &self.row.columns[self.index_mapping[425]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[426]],
                ScriptValue: &self.row.columns[self.index_mapping[427]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[428]],
                ScriptValue: &self.row.columns[self.index_mapping[429]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[430]],
                ScriptValue: &self.row.columns[self.index_mapping[431]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[432]],
                ScriptValue: &self.row.columns[self.index_mapping[433]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[434]],
                ScriptValue: &self.row.columns[self.index_mapping[435]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[436]],
                ScriptValue: &self.row.columns[self.index_mapping[437]],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[438]],
                ScriptValue: &self.row.columns[self.index_mapping[439]],
            },
        ]
    }
    pub fn Quest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[440]]
    }
    pub fn TimeLimit(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[441]]
    }
    pub fn LevelSync(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[442]]
    }
    pub fn QuestBattleScene(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[443]]
    }
}
