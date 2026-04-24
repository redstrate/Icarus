//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuestBattleParamsElement<'a> {
    pub ScriptInstruction: &'a str,
    pub ScriptValue: u32,
}
#[derive(Debug, Clone)]
pub struct QuestBattleSheet {
    sheet: Sheet,
}
impl QuestBattleSheet {
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
impl<'a> StructuredSheet<'a> for QuestBattleSheet {
    type Row = QuestBattleRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
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
}
impl<'a> QuestBattleRow<'a> {
    pub fn QuestBattleParams(&'a self) -> [QuestBattleParamsElement<'a>; 220] {
        [
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[4].into_string().unwrap(),
                ScriptValue: self.row.columns[224].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[5].into_string().unwrap(),
                ScriptValue: self.row.columns[225].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[6].into_string().unwrap(),
                ScriptValue: self.row.columns[226].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[7].into_string().unwrap(),
                ScriptValue: self.row.columns[227].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[8].into_string().unwrap(),
                ScriptValue: self.row.columns[228].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[9].into_string().unwrap(),
                ScriptValue: self.row.columns[229].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[10].into_string().unwrap(),
                ScriptValue: self.row.columns[230].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[11].into_string().unwrap(),
                ScriptValue: self.row.columns[231].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[12].into_string().unwrap(),
                ScriptValue: self.row.columns[232].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[13].into_string().unwrap(),
                ScriptValue: self.row.columns[233].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[14].into_string().unwrap(),
                ScriptValue: self.row.columns[234].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[15].into_string().unwrap(),
                ScriptValue: self.row.columns[235].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[16].into_string().unwrap(),
                ScriptValue: self.row.columns[236].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[17].into_string().unwrap(),
                ScriptValue: self.row.columns[237].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[18].into_string().unwrap(),
                ScriptValue: self.row.columns[238].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[19].into_string().unwrap(),
                ScriptValue: self.row.columns[239].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[20].into_string().unwrap(),
                ScriptValue: self.row.columns[240].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[21].into_string().unwrap(),
                ScriptValue: self.row.columns[241].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[22].into_string().unwrap(),
                ScriptValue: self.row.columns[242].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[23].into_string().unwrap(),
                ScriptValue: self.row.columns[243].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[24].into_string().unwrap(),
                ScriptValue: self.row.columns[244].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[25].into_string().unwrap(),
                ScriptValue: self.row.columns[245].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[26].into_string().unwrap(),
                ScriptValue: self.row.columns[246].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[27].into_string().unwrap(),
                ScriptValue: self.row.columns[247].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[28].into_string().unwrap(),
                ScriptValue: self.row.columns[248].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[29].into_string().unwrap(),
                ScriptValue: self.row.columns[249].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[30].into_string().unwrap(),
                ScriptValue: self.row.columns[250].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[31].into_string().unwrap(),
                ScriptValue: self.row.columns[251].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[32].into_string().unwrap(),
                ScriptValue: self.row.columns[252].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[33].into_string().unwrap(),
                ScriptValue: self.row.columns[253].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[34].into_string().unwrap(),
                ScriptValue: self.row.columns[254].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[35].into_string().unwrap(),
                ScriptValue: self.row.columns[255].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[36].into_string().unwrap(),
                ScriptValue: self.row.columns[256].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[37].into_string().unwrap(),
                ScriptValue: self.row.columns[257].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[38].into_string().unwrap(),
                ScriptValue: self.row.columns[258].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[39].into_string().unwrap(),
                ScriptValue: self.row.columns[259].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[40].into_string().unwrap(),
                ScriptValue: self.row.columns[260].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[41].into_string().unwrap(),
                ScriptValue: self.row.columns[261].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[42].into_string().unwrap(),
                ScriptValue: self.row.columns[262].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[43].into_string().unwrap(),
                ScriptValue: self.row.columns[263].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[44].into_string().unwrap(),
                ScriptValue: self.row.columns[264].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[45].into_string().unwrap(),
                ScriptValue: self.row.columns[265].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[46].into_string().unwrap(),
                ScriptValue: self.row.columns[266].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[47].into_string().unwrap(),
                ScriptValue: self.row.columns[267].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[48].into_string().unwrap(),
                ScriptValue: self.row.columns[268].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[49].into_string().unwrap(),
                ScriptValue: self.row.columns[269].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[50].into_string().unwrap(),
                ScriptValue: self.row.columns[270].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[51].into_string().unwrap(),
                ScriptValue: self.row.columns[271].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[52].into_string().unwrap(),
                ScriptValue: self.row.columns[272].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[53].into_string().unwrap(),
                ScriptValue: self.row.columns[273].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[54].into_string().unwrap(),
                ScriptValue: self.row.columns[274].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[55].into_string().unwrap(),
                ScriptValue: self.row.columns[275].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[56].into_string().unwrap(),
                ScriptValue: self.row.columns[276].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[57].into_string().unwrap(),
                ScriptValue: self.row.columns[277].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[58].into_string().unwrap(),
                ScriptValue: self.row.columns[278].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[59].into_string().unwrap(),
                ScriptValue: self.row.columns[279].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[60].into_string().unwrap(),
                ScriptValue: self.row.columns[280].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[61].into_string().unwrap(),
                ScriptValue: self.row.columns[281].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[62].into_string().unwrap(),
                ScriptValue: self.row.columns[282].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[63].into_string().unwrap(),
                ScriptValue: self.row.columns[283].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[64].into_string().unwrap(),
                ScriptValue: self.row.columns[284].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[65].into_string().unwrap(),
                ScriptValue: self.row.columns[285].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[66].into_string().unwrap(),
                ScriptValue: self.row.columns[286].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[67].into_string().unwrap(),
                ScriptValue: self.row.columns[287].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[68].into_string().unwrap(),
                ScriptValue: self.row.columns[288].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[69].into_string().unwrap(),
                ScriptValue: self.row.columns[289].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[70].into_string().unwrap(),
                ScriptValue: self.row.columns[290].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[71].into_string().unwrap(),
                ScriptValue: self.row.columns[291].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[72].into_string().unwrap(),
                ScriptValue: self.row.columns[292].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[73].into_string().unwrap(),
                ScriptValue: self.row.columns[293].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[74].into_string().unwrap(),
                ScriptValue: self.row.columns[294].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[75].into_string().unwrap(),
                ScriptValue: self.row.columns[295].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[76].into_string().unwrap(),
                ScriptValue: self.row.columns[296].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[77].into_string().unwrap(),
                ScriptValue: self.row.columns[297].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[78].into_string().unwrap(),
                ScriptValue: self.row.columns[298].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[79].into_string().unwrap(),
                ScriptValue: self.row.columns[299].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[80].into_string().unwrap(),
                ScriptValue: self.row.columns[300].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[81].into_string().unwrap(),
                ScriptValue: self.row.columns[301].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[82].into_string().unwrap(),
                ScriptValue: self.row.columns[302].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[83].into_string().unwrap(),
                ScriptValue: self.row.columns[303].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[84].into_string().unwrap(),
                ScriptValue: self.row.columns[304].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[85].into_string().unwrap(),
                ScriptValue: self.row.columns[305].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[86].into_string().unwrap(),
                ScriptValue: self.row.columns[306].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[87].into_string().unwrap(),
                ScriptValue: self.row.columns[307].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[88].into_string().unwrap(),
                ScriptValue: self.row.columns[308].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[89].into_string().unwrap(),
                ScriptValue: self.row.columns[309].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[90].into_string().unwrap(),
                ScriptValue: self.row.columns[310].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[91].into_string().unwrap(),
                ScriptValue: self.row.columns[311].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[92].into_string().unwrap(),
                ScriptValue: self.row.columns[312].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[93].into_string().unwrap(),
                ScriptValue: self.row.columns[313].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[94].into_string().unwrap(),
                ScriptValue: self.row.columns[314].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[95].into_string().unwrap(),
                ScriptValue: self.row.columns[315].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[96].into_string().unwrap(),
                ScriptValue: self.row.columns[316].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[97].into_string().unwrap(),
                ScriptValue: self.row.columns[317].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[98].into_string().unwrap(),
                ScriptValue: self.row.columns[318].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[99].into_string().unwrap(),
                ScriptValue: self.row.columns[319].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[100].into_string().unwrap(),
                ScriptValue: self.row.columns[320].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[101].into_string().unwrap(),
                ScriptValue: self.row.columns[321].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[102].into_string().unwrap(),
                ScriptValue: self.row.columns[322].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[103].into_string().unwrap(),
                ScriptValue: self.row.columns[323].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[104].into_string().unwrap(),
                ScriptValue: self.row.columns[324].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[105].into_string().unwrap(),
                ScriptValue: self.row.columns[325].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[106].into_string().unwrap(),
                ScriptValue: self.row.columns[326].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[107].into_string().unwrap(),
                ScriptValue: self.row.columns[327].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[108].into_string().unwrap(),
                ScriptValue: self.row.columns[328].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[109].into_string().unwrap(),
                ScriptValue: self.row.columns[329].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[110].into_string().unwrap(),
                ScriptValue: self.row.columns[330].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[111].into_string().unwrap(),
                ScriptValue: self.row.columns[331].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[112].into_string().unwrap(),
                ScriptValue: self.row.columns[332].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[113].into_string().unwrap(),
                ScriptValue: self.row.columns[333].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[114].into_string().unwrap(),
                ScriptValue: self.row.columns[334].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[115].into_string().unwrap(),
                ScriptValue: self.row.columns[335].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[116].into_string().unwrap(),
                ScriptValue: self.row.columns[336].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[117].into_string().unwrap(),
                ScriptValue: self.row.columns[337].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[118].into_string().unwrap(),
                ScriptValue: self.row.columns[338].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[119].into_string().unwrap(),
                ScriptValue: self.row.columns[339].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[120].into_string().unwrap(),
                ScriptValue: self.row.columns[340].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[121].into_string().unwrap(),
                ScriptValue: self.row.columns[341].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[122].into_string().unwrap(),
                ScriptValue: self.row.columns[342].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[123].into_string().unwrap(),
                ScriptValue: self.row.columns[343].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[124].into_string().unwrap(),
                ScriptValue: self.row.columns[344].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[125].into_string().unwrap(),
                ScriptValue: self.row.columns[345].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[126].into_string().unwrap(),
                ScriptValue: self.row.columns[346].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[127].into_string().unwrap(),
                ScriptValue: self.row.columns[347].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[128].into_string().unwrap(),
                ScriptValue: self.row.columns[348].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[129].into_string().unwrap(),
                ScriptValue: self.row.columns[349].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[130].into_string().unwrap(),
                ScriptValue: self.row.columns[350].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[131].into_string().unwrap(),
                ScriptValue: self.row.columns[351].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[132].into_string().unwrap(),
                ScriptValue: self.row.columns[352].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[133].into_string().unwrap(),
                ScriptValue: self.row.columns[353].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[134].into_string().unwrap(),
                ScriptValue: self.row.columns[354].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[135].into_string().unwrap(),
                ScriptValue: self.row.columns[355].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[136].into_string().unwrap(),
                ScriptValue: self.row.columns[356].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[137].into_string().unwrap(),
                ScriptValue: self.row.columns[357].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[138].into_string().unwrap(),
                ScriptValue: self.row.columns[358].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[139].into_string().unwrap(),
                ScriptValue: self.row.columns[359].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[140].into_string().unwrap(),
                ScriptValue: self.row.columns[360].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[141].into_string().unwrap(),
                ScriptValue: self.row.columns[361].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[142].into_string().unwrap(),
                ScriptValue: self.row.columns[362].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[143].into_string().unwrap(),
                ScriptValue: self.row.columns[363].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[144].into_string().unwrap(),
                ScriptValue: self.row.columns[364].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[145].into_string().unwrap(),
                ScriptValue: self.row.columns[365].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[146].into_string().unwrap(),
                ScriptValue: self.row.columns[366].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[147].into_string().unwrap(),
                ScriptValue: self.row.columns[367].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[148].into_string().unwrap(),
                ScriptValue: self.row.columns[368].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[149].into_string().unwrap(),
                ScriptValue: self.row.columns[369].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[150].into_string().unwrap(),
                ScriptValue: self.row.columns[370].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[151].into_string().unwrap(),
                ScriptValue: self.row.columns[371].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[152].into_string().unwrap(),
                ScriptValue: self.row.columns[372].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[153].into_string().unwrap(),
                ScriptValue: self.row.columns[373].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[154].into_string().unwrap(),
                ScriptValue: self.row.columns[374].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[155].into_string().unwrap(),
                ScriptValue: self.row.columns[375].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[156].into_string().unwrap(),
                ScriptValue: self.row.columns[376].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[157].into_string().unwrap(),
                ScriptValue: self.row.columns[377].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[158].into_string().unwrap(),
                ScriptValue: self.row.columns[378].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[159].into_string().unwrap(),
                ScriptValue: self.row.columns[379].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[160].into_string().unwrap(),
                ScriptValue: self.row.columns[380].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[161].into_string().unwrap(),
                ScriptValue: self.row.columns[381].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[162].into_string().unwrap(),
                ScriptValue: self.row.columns[382].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[163].into_string().unwrap(),
                ScriptValue: self.row.columns[383].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[164].into_string().unwrap(),
                ScriptValue: self.row.columns[384].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[165].into_string().unwrap(),
                ScriptValue: self.row.columns[385].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[166].into_string().unwrap(),
                ScriptValue: self.row.columns[386].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[167].into_string().unwrap(),
                ScriptValue: self.row.columns[387].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[168].into_string().unwrap(),
                ScriptValue: self.row.columns[388].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[169].into_string().unwrap(),
                ScriptValue: self.row.columns[389].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[170].into_string().unwrap(),
                ScriptValue: self.row.columns[390].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[171].into_string().unwrap(),
                ScriptValue: self.row.columns[391].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[172].into_string().unwrap(),
                ScriptValue: self.row.columns[392].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[173].into_string().unwrap(),
                ScriptValue: self.row.columns[393].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[174].into_string().unwrap(),
                ScriptValue: self.row.columns[394].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[175].into_string().unwrap(),
                ScriptValue: self.row.columns[395].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[176].into_string().unwrap(),
                ScriptValue: self.row.columns[396].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[177].into_string().unwrap(),
                ScriptValue: self.row.columns[397].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[178].into_string().unwrap(),
                ScriptValue: self.row.columns[398].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[179].into_string().unwrap(),
                ScriptValue: self.row.columns[399].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[180].into_string().unwrap(),
                ScriptValue: self.row.columns[400].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[181].into_string().unwrap(),
                ScriptValue: self.row.columns[401].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[182].into_string().unwrap(),
                ScriptValue: self.row.columns[402].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[183].into_string().unwrap(),
                ScriptValue: self.row.columns[403].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[184].into_string().unwrap(),
                ScriptValue: self.row.columns[404].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[185].into_string().unwrap(),
                ScriptValue: self.row.columns[405].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[186].into_string().unwrap(),
                ScriptValue: self.row.columns[406].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[187].into_string().unwrap(),
                ScriptValue: self.row.columns[407].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[188].into_string().unwrap(),
                ScriptValue: self.row.columns[408].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[189].into_string().unwrap(),
                ScriptValue: self.row.columns[409].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[190].into_string().unwrap(),
                ScriptValue: self.row.columns[410].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[191].into_string().unwrap(),
                ScriptValue: self.row.columns[411].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[192].into_string().unwrap(),
                ScriptValue: self.row.columns[412].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[193].into_string().unwrap(),
                ScriptValue: self.row.columns[413].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[194].into_string().unwrap(),
                ScriptValue: self.row.columns[414].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[195].into_string().unwrap(),
                ScriptValue: self.row.columns[415].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[196].into_string().unwrap(),
                ScriptValue: self.row.columns[416].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[197].into_string().unwrap(),
                ScriptValue: self.row.columns[417].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[198].into_string().unwrap(),
                ScriptValue: self.row.columns[418].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[199].into_string().unwrap(),
                ScriptValue: self.row.columns[419].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[200].into_string().unwrap(),
                ScriptValue: self.row.columns[420].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[201].into_string().unwrap(),
                ScriptValue: self.row.columns[421].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[202].into_string().unwrap(),
                ScriptValue: self.row.columns[422].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[203].into_string().unwrap(),
                ScriptValue: self.row.columns[423].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[204].into_string().unwrap(),
                ScriptValue: self.row.columns[424].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[205].into_string().unwrap(),
                ScriptValue: self.row.columns[425].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[206].into_string().unwrap(),
                ScriptValue: self.row.columns[426].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[207].into_string().unwrap(),
                ScriptValue: self.row.columns[427].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[208].into_string().unwrap(),
                ScriptValue: self.row.columns[428].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[209].into_string().unwrap(),
                ScriptValue: self.row.columns[429].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[210].into_string().unwrap(),
                ScriptValue: self.row.columns[430].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[211].into_string().unwrap(),
                ScriptValue: self.row.columns[431].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[212].into_string().unwrap(),
                ScriptValue: self.row.columns[432].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[213].into_string().unwrap(),
                ScriptValue: self.row.columns[433].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[214].into_string().unwrap(),
                ScriptValue: self.row.columns[434].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[215].into_string().unwrap(),
                ScriptValue: self.row.columns[435].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[216].into_string().unwrap(),
                ScriptValue: self.row.columns[436].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[217].into_string().unwrap(),
                ScriptValue: self.row.columns[437].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[218].into_string().unwrap(),
                ScriptValue: self.row.columns[438].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[219].into_string().unwrap(),
                ScriptValue: self.row.columns[439].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[220].into_string().unwrap(),
                ScriptValue: self.row.columns[440].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[221].into_string().unwrap(),
                ScriptValue: self.row.columns[441].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[222].into_string().unwrap(),
                ScriptValue: self.row.columns[442].into_u32().copied().unwrap(),
            },
            QuestBattleParamsElement {
                ScriptInstruction: self.row.columns[223].into_string().unwrap(),
                ScriptValue: self.row.columns[443].into_u32().copied().unwrap(),
            },
        ]
    }
    pub fn Quest(&'a self) -> i32 {
        self.row.columns[0].into_i32().copied().unwrap()
    }
    pub fn TimeLimit(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn LevelSync(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn QuestBattleScene(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
}
