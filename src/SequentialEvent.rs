//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct UnknownStructElement<'a> {
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
    pub Unknown3: &'a Field,
    pub Unknown4: &'a Field,
    pub Unknown5: &'a Field,
}
#[derive(Debug, Clone)]
pub struct SequentialEventSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl SequentialEventSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SequentialEvent")?;
        let sheet = resolver.read_excel_sheet(&exh, "SequentialEvent", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<SequentialEventRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SequentialEventRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SequentialEventSheet {
    type Row = SequentialEventRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a SequentialEventSheet {
    type Item = (u32, Vec<(u16, SequentialEventRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SequentialEventSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SequentialEventSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SequentialEventRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> SequentialEventRow<'a> {
    pub fn UnknownStruct(&'a self) -> [UnknownStructElement<'a>; 64] {
        [
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[0]],
                Unknown2: &self.row.columns[self.index_mapping[1]],
                Unknown3: &self.row.columns[self.index_mapping[2]],
                Unknown4: &self.row.columns[self.index_mapping[3]],
                Unknown5: &self.row.columns[self.index_mapping[4]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[5]],
                Unknown2: &self.row.columns[self.index_mapping[6]],
                Unknown3: &self.row.columns[self.index_mapping[7]],
                Unknown4: &self.row.columns[self.index_mapping[8]],
                Unknown5: &self.row.columns[self.index_mapping[9]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[10]],
                Unknown2: &self.row.columns[self.index_mapping[11]],
                Unknown3: &self.row.columns[self.index_mapping[12]],
                Unknown4: &self.row.columns[self.index_mapping[13]],
                Unknown5: &self.row.columns[self.index_mapping[14]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[15]],
                Unknown2: &self.row.columns[self.index_mapping[16]],
                Unknown3: &self.row.columns[self.index_mapping[17]],
                Unknown4: &self.row.columns[self.index_mapping[18]],
                Unknown5: &self.row.columns[self.index_mapping[19]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[20]],
                Unknown2: &self.row.columns[self.index_mapping[21]],
                Unknown3: &self.row.columns[self.index_mapping[22]],
                Unknown4: &self.row.columns[self.index_mapping[23]],
                Unknown5: &self.row.columns[self.index_mapping[24]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[25]],
                Unknown2: &self.row.columns[self.index_mapping[26]],
                Unknown3: &self.row.columns[self.index_mapping[27]],
                Unknown4: &self.row.columns[self.index_mapping[28]],
                Unknown5: &self.row.columns[self.index_mapping[29]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[30]],
                Unknown2: &self.row.columns[self.index_mapping[31]],
                Unknown3: &self.row.columns[self.index_mapping[32]],
                Unknown4: &self.row.columns[self.index_mapping[33]],
                Unknown5: &self.row.columns[self.index_mapping[34]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[35]],
                Unknown2: &self.row.columns[self.index_mapping[36]],
                Unknown3: &self.row.columns[self.index_mapping[37]],
                Unknown4: &self.row.columns[self.index_mapping[38]],
                Unknown5: &self.row.columns[self.index_mapping[39]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[40]],
                Unknown2: &self.row.columns[self.index_mapping[41]],
                Unknown3: &self.row.columns[self.index_mapping[42]],
                Unknown4: &self.row.columns[self.index_mapping[43]],
                Unknown5: &self.row.columns[self.index_mapping[44]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[45]],
                Unknown2: &self.row.columns[self.index_mapping[46]],
                Unknown3: &self.row.columns[self.index_mapping[47]],
                Unknown4: &self.row.columns[self.index_mapping[48]],
                Unknown5: &self.row.columns[self.index_mapping[49]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[50]],
                Unknown2: &self.row.columns[self.index_mapping[51]],
                Unknown3: &self.row.columns[self.index_mapping[52]],
                Unknown4: &self.row.columns[self.index_mapping[53]],
                Unknown5: &self.row.columns[self.index_mapping[54]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[55]],
                Unknown2: &self.row.columns[self.index_mapping[56]],
                Unknown3: &self.row.columns[self.index_mapping[57]],
                Unknown4: &self.row.columns[self.index_mapping[58]],
                Unknown5: &self.row.columns[self.index_mapping[59]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[60]],
                Unknown2: &self.row.columns[self.index_mapping[61]],
                Unknown3: &self.row.columns[self.index_mapping[62]],
                Unknown4: &self.row.columns[self.index_mapping[63]],
                Unknown5: &self.row.columns[self.index_mapping[64]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[65]],
                Unknown2: &self.row.columns[self.index_mapping[66]],
                Unknown3: &self.row.columns[self.index_mapping[67]],
                Unknown4: &self.row.columns[self.index_mapping[68]],
                Unknown5: &self.row.columns[self.index_mapping[69]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[70]],
                Unknown2: &self.row.columns[self.index_mapping[71]],
                Unknown3: &self.row.columns[self.index_mapping[72]],
                Unknown4: &self.row.columns[self.index_mapping[73]],
                Unknown5: &self.row.columns[self.index_mapping[74]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[75]],
                Unknown2: &self.row.columns[self.index_mapping[76]],
                Unknown3: &self.row.columns[self.index_mapping[77]],
                Unknown4: &self.row.columns[self.index_mapping[78]],
                Unknown5: &self.row.columns[self.index_mapping[79]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[80]],
                Unknown2: &self.row.columns[self.index_mapping[81]],
                Unknown3: &self.row.columns[self.index_mapping[82]],
                Unknown4: &self.row.columns[self.index_mapping[83]],
                Unknown5: &self.row.columns[self.index_mapping[84]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[85]],
                Unknown2: &self.row.columns[self.index_mapping[86]],
                Unknown3: &self.row.columns[self.index_mapping[87]],
                Unknown4: &self.row.columns[self.index_mapping[88]],
                Unknown5: &self.row.columns[self.index_mapping[89]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[90]],
                Unknown2: &self.row.columns[self.index_mapping[91]],
                Unknown3: &self.row.columns[self.index_mapping[92]],
                Unknown4: &self.row.columns[self.index_mapping[93]],
                Unknown5: &self.row.columns[self.index_mapping[94]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[95]],
                Unknown2: &self.row.columns[self.index_mapping[96]],
                Unknown3: &self.row.columns[self.index_mapping[97]],
                Unknown4: &self.row.columns[self.index_mapping[98]],
                Unknown5: &self.row.columns[self.index_mapping[99]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[100]],
                Unknown2: &self.row.columns[self.index_mapping[101]],
                Unknown3: &self.row.columns[self.index_mapping[102]],
                Unknown4: &self.row.columns[self.index_mapping[103]],
                Unknown5: &self.row.columns[self.index_mapping[104]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[105]],
                Unknown2: &self.row.columns[self.index_mapping[106]],
                Unknown3: &self.row.columns[self.index_mapping[107]],
                Unknown4: &self.row.columns[self.index_mapping[108]],
                Unknown5: &self.row.columns[self.index_mapping[109]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[110]],
                Unknown2: &self.row.columns[self.index_mapping[111]],
                Unknown3: &self.row.columns[self.index_mapping[112]],
                Unknown4: &self.row.columns[self.index_mapping[113]],
                Unknown5: &self.row.columns[self.index_mapping[114]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[115]],
                Unknown2: &self.row.columns[self.index_mapping[116]],
                Unknown3: &self.row.columns[self.index_mapping[117]],
                Unknown4: &self.row.columns[self.index_mapping[118]],
                Unknown5: &self.row.columns[self.index_mapping[119]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[120]],
                Unknown2: &self.row.columns[self.index_mapping[121]],
                Unknown3: &self.row.columns[self.index_mapping[122]],
                Unknown4: &self.row.columns[self.index_mapping[123]],
                Unknown5: &self.row.columns[self.index_mapping[124]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[125]],
                Unknown2: &self.row.columns[self.index_mapping[126]],
                Unknown3: &self.row.columns[self.index_mapping[127]],
                Unknown4: &self.row.columns[self.index_mapping[128]],
                Unknown5: &self.row.columns[self.index_mapping[129]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[130]],
                Unknown2: &self.row.columns[self.index_mapping[131]],
                Unknown3: &self.row.columns[self.index_mapping[132]],
                Unknown4: &self.row.columns[self.index_mapping[133]],
                Unknown5: &self.row.columns[self.index_mapping[134]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[135]],
                Unknown2: &self.row.columns[self.index_mapping[136]],
                Unknown3: &self.row.columns[self.index_mapping[137]],
                Unknown4: &self.row.columns[self.index_mapping[138]],
                Unknown5: &self.row.columns[self.index_mapping[139]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[140]],
                Unknown2: &self.row.columns[self.index_mapping[141]],
                Unknown3: &self.row.columns[self.index_mapping[142]],
                Unknown4: &self.row.columns[self.index_mapping[143]],
                Unknown5: &self.row.columns[self.index_mapping[144]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[145]],
                Unknown2: &self.row.columns[self.index_mapping[146]],
                Unknown3: &self.row.columns[self.index_mapping[147]],
                Unknown4: &self.row.columns[self.index_mapping[148]],
                Unknown5: &self.row.columns[self.index_mapping[149]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[150]],
                Unknown2: &self.row.columns[self.index_mapping[151]],
                Unknown3: &self.row.columns[self.index_mapping[152]],
                Unknown4: &self.row.columns[self.index_mapping[153]],
                Unknown5: &self.row.columns[self.index_mapping[154]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[155]],
                Unknown2: &self.row.columns[self.index_mapping[156]],
                Unknown3: &self.row.columns[self.index_mapping[157]],
                Unknown4: &self.row.columns[self.index_mapping[158]],
                Unknown5: &self.row.columns[self.index_mapping[159]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[160]],
                Unknown2: &self.row.columns[self.index_mapping[161]],
                Unknown3: &self.row.columns[self.index_mapping[162]],
                Unknown4: &self.row.columns[self.index_mapping[163]],
                Unknown5: &self.row.columns[self.index_mapping[164]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[165]],
                Unknown2: &self.row.columns[self.index_mapping[166]],
                Unknown3: &self.row.columns[self.index_mapping[167]],
                Unknown4: &self.row.columns[self.index_mapping[168]],
                Unknown5: &self.row.columns[self.index_mapping[169]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[170]],
                Unknown2: &self.row.columns[self.index_mapping[171]],
                Unknown3: &self.row.columns[self.index_mapping[172]],
                Unknown4: &self.row.columns[self.index_mapping[173]],
                Unknown5: &self.row.columns[self.index_mapping[174]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[175]],
                Unknown2: &self.row.columns[self.index_mapping[176]],
                Unknown3: &self.row.columns[self.index_mapping[177]],
                Unknown4: &self.row.columns[self.index_mapping[178]],
                Unknown5: &self.row.columns[self.index_mapping[179]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[180]],
                Unknown2: &self.row.columns[self.index_mapping[181]],
                Unknown3: &self.row.columns[self.index_mapping[182]],
                Unknown4: &self.row.columns[self.index_mapping[183]],
                Unknown5: &self.row.columns[self.index_mapping[184]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[185]],
                Unknown2: &self.row.columns[self.index_mapping[186]],
                Unknown3: &self.row.columns[self.index_mapping[187]],
                Unknown4: &self.row.columns[self.index_mapping[188]],
                Unknown5: &self.row.columns[self.index_mapping[189]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[190]],
                Unknown2: &self.row.columns[self.index_mapping[191]],
                Unknown3: &self.row.columns[self.index_mapping[192]],
                Unknown4: &self.row.columns[self.index_mapping[193]],
                Unknown5: &self.row.columns[self.index_mapping[194]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[195]],
                Unknown2: &self.row.columns[self.index_mapping[196]],
                Unknown3: &self.row.columns[self.index_mapping[197]],
                Unknown4: &self.row.columns[self.index_mapping[198]],
                Unknown5: &self.row.columns[self.index_mapping[199]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[200]],
                Unknown2: &self.row.columns[self.index_mapping[201]],
                Unknown3: &self.row.columns[self.index_mapping[202]],
                Unknown4: &self.row.columns[self.index_mapping[203]],
                Unknown5: &self.row.columns[self.index_mapping[204]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[205]],
                Unknown2: &self.row.columns[self.index_mapping[206]],
                Unknown3: &self.row.columns[self.index_mapping[207]],
                Unknown4: &self.row.columns[self.index_mapping[208]],
                Unknown5: &self.row.columns[self.index_mapping[209]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[210]],
                Unknown2: &self.row.columns[self.index_mapping[211]],
                Unknown3: &self.row.columns[self.index_mapping[212]],
                Unknown4: &self.row.columns[self.index_mapping[213]],
                Unknown5: &self.row.columns[self.index_mapping[214]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[215]],
                Unknown2: &self.row.columns[self.index_mapping[216]],
                Unknown3: &self.row.columns[self.index_mapping[217]],
                Unknown4: &self.row.columns[self.index_mapping[218]],
                Unknown5: &self.row.columns[self.index_mapping[219]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[220]],
                Unknown2: &self.row.columns[self.index_mapping[221]],
                Unknown3: &self.row.columns[self.index_mapping[222]],
                Unknown4: &self.row.columns[self.index_mapping[223]],
                Unknown5: &self.row.columns[self.index_mapping[224]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[225]],
                Unknown2: &self.row.columns[self.index_mapping[226]],
                Unknown3: &self.row.columns[self.index_mapping[227]],
                Unknown4: &self.row.columns[self.index_mapping[228]],
                Unknown5: &self.row.columns[self.index_mapping[229]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[230]],
                Unknown2: &self.row.columns[self.index_mapping[231]],
                Unknown3: &self.row.columns[self.index_mapping[232]],
                Unknown4: &self.row.columns[self.index_mapping[233]],
                Unknown5: &self.row.columns[self.index_mapping[234]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[235]],
                Unknown2: &self.row.columns[self.index_mapping[236]],
                Unknown3: &self.row.columns[self.index_mapping[237]],
                Unknown4: &self.row.columns[self.index_mapping[238]],
                Unknown5: &self.row.columns[self.index_mapping[239]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[240]],
                Unknown2: &self.row.columns[self.index_mapping[241]],
                Unknown3: &self.row.columns[self.index_mapping[242]],
                Unknown4: &self.row.columns[self.index_mapping[243]],
                Unknown5: &self.row.columns[self.index_mapping[244]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[245]],
                Unknown2: &self.row.columns[self.index_mapping[246]],
                Unknown3: &self.row.columns[self.index_mapping[247]],
                Unknown4: &self.row.columns[self.index_mapping[248]],
                Unknown5: &self.row.columns[self.index_mapping[249]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[250]],
                Unknown2: &self.row.columns[self.index_mapping[251]],
                Unknown3: &self.row.columns[self.index_mapping[252]],
                Unknown4: &self.row.columns[self.index_mapping[253]],
                Unknown5: &self.row.columns[self.index_mapping[254]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[255]],
                Unknown2: &self.row.columns[self.index_mapping[256]],
                Unknown3: &self.row.columns[self.index_mapping[257]],
                Unknown4: &self.row.columns[self.index_mapping[258]],
                Unknown5: &self.row.columns[self.index_mapping[259]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[260]],
                Unknown2: &self.row.columns[self.index_mapping[261]],
                Unknown3: &self.row.columns[self.index_mapping[262]],
                Unknown4: &self.row.columns[self.index_mapping[263]],
                Unknown5: &self.row.columns[self.index_mapping[264]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[265]],
                Unknown2: &self.row.columns[self.index_mapping[266]],
                Unknown3: &self.row.columns[self.index_mapping[267]],
                Unknown4: &self.row.columns[self.index_mapping[268]],
                Unknown5: &self.row.columns[self.index_mapping[269]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[270]],
                Unknown2: &self.row.columns[self.index_mapping[271]],
                Unknown3: &self.row.columns[self.index_mapping[272]],
                Unknown4: &self.row.columns[self.index_mapping[273]],
                Unknown5: &self.row.columns[self.index_mapping[274]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[275]],
                Unknown2: &self.row.columns[self.index_mapping[276]],
                Unknown3: &self.row.columns[self.index_mapping[277]],
                Unknown4: &self.row.columns[self.index_mapping[278]],
                Unknown5: &self.row.columns[self.index_mapping[279]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[280]],
                Unknown2: &self.row.columns[self.index_mapping[281]],
                Unknown3: &self.row.columns[self.index_mapping[282]],
                Unknown4: &self.row.columns[self.index_mapping[283]],
                Unknown5: &self.row.columns[self.index_mapping[284]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[285]],
                Unknown2: &self.row.columns[self.index_mapping[286]],
                Unknown3: &self.row.columns[self.index_mapping[287]],
                Unknown4: &self.row.columns[self.index_mapping[288]],
                Unknown5: &self.row.columns[self.index_mapping[289]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[290]],
                Unknown2: &self.row.columns[self.index_mapping[291]],
                Unknown3: &self.row.columns[self.index_mapping[292]],
                Unknown4: &self.row.columns[self.index_mapping[293]],
                Unknown5: &self.row.columns[self.index_mapping[294]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[295]],
                Unknown2: &self.row.columns[self.index_mapping[296]],
                Unknown3: &self.row.columns[self.index_mapping[297]],
                Unknown4: &self.row.columns[self.index_mapping[298]],
                Unknown5: &self.row.columns[self.index_mapping[299]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[300]],
                Unknown2: &self.row.columns[self.index_mapping[301]],
                Unknown3: &self.row.columns[self.index_mapping[302]],
                Unknown4: &self.row.columns[self.index_mapping[303]],
                Unknown5: &self.row.columns[self.index_mapping[304]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[305]],
                Unknown2: &self.row.columns[self.index_mapping[306]],
                Unknown3: &self.row.columns[self.index_mapping[307]],
                Unknown4: &self.row.columns[self.index_mapping[308]],
                Unknown5: &self.row.columns[self.index_mapping[309]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[310]],
                Unknown2: &self.row.columns[self.index_mapping[311]],
                Unknown3: &self.row.columns[self.index_mapping[312]],
                Unknown4: &self.row.columns[self.index_mapping[313]],
                Unknown5: &self.row.columns[self.index_mapping[314]],
            },
            UnknownStructElement {
                Unknown1: &self.row.columns[self.index_mapping[315]],
                Unknown2: &self.row.columns[self.index_mapping[316]],
                Unknown3: &self.row.columns[self.index_mapping[317]],
                Unknown4: &self.row.columns[self.index_mapping[318]],
                Unknown5: &self.row.columns[self.index_mapping[319]],
            },
        ]
    }
    pub fn Unknown320(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[320]]
    }
    pub fn Unknown_70(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[321]]
    }
    pub fn Unknown321(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[322]]
    }
}
