//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct StoryParamsElement<'a> {
    pub Instruction: &'a Field,
    pub Argument: &'a Field,
}
pub struct StoryDefineElement<'a> {
    pub CompletedQuest: [&'a Field; 3],
    pub AcceptedQuest: [&'a Field; 3],
    pub LayerSet: [&'a Field; 2],
    pub Sequence: &'a Field,
    pub CompletedQuestOperator: &'a Field,
    pub AcceptedQuestOperator: &'a Field,
    pub AcceptedQuestSequence: [&'a Field; 3],
}
pub struct StoryListenerElement<'a> {
    pub Listener: &'a Field,
    pub SequenceBegin: &'a Field,
    pub SequenceEnd: &'a Field,
}
#[derive(Debug, Clone)]
pub struct StorySheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl StorySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Story")?;
        let sheet = resolver.read_excel_sheet(&exh, "Story", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<StoryRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<StoryRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for StorySheet {
    type Row = StoryRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a StorySheet {
    type Item = (u32, Vec<(u16, StoryRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, StorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, StorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StoryRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> StoryRow<'a> {
    pub fn StoryParams(&'a self) -> [StoryParamsElement<'a>; 40] {
        [
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[0]],
                Argument: &self.row.columns[self.index_mapping[1]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[2]],
                Argument: &self.row.columns[self.index_mapping[3]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[4]],
                Argument: &self.row.columns[self.index_mapping[5]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[6]],
                Argument: &self.row.columns[self.index_mapping[7]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[8]],
                Argument: &self.row.columns[self.index_mapping[9]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[10]],
                Argument: &self.row.columns[self.index_mapping[11]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[12]],
                Argument: &self.row.columns[self.index_mapping[13]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[14]],
                Argument: &self.row.columns[self.index_mapping[15]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[16]],
                Argument: &self.row.columns[self.index_mapping[17]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[18]],
                Argument: &self.row.columns[self.index_mapping[19]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[20]],
                Argument: &self.row.columns[self.index_mapping[21]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[22]],
                Argument: &self.row.columns[self.index_mapping[23]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[24]],
                Argument: &self.row.columns[self.index_mapping[25]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[26]],
                Argument: &self.row.columns[self.index_mapping[27]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[28]],
                Argument: &self.row.columns[self.index_mapping[29]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[30]],
                Argument: &self.row.columns[self.index_mapping[31]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[32]],
                Argument: &self.row.columns[self.index_mapping[33]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[34]],
                Argument: &self.row.columns[self.index_mapping[35]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[36]],
                Argument: &self.row.columns[self.index_mapping[37]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[38]],
                Argument: &self.row.columns[self.index_mapping[39]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[40]],
                Argument: &self.row.columns[self.index_mapping[41]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[42]],
                Argument: &self.row.columns[self.index_mapping[43]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[44]],
                Argument: &self.row.columns[self.index_mapping[45]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[46]],
                Argument: &self.row.columns[self.index_mapping[47]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[48]],
                Argument: &self.row.columns[self.index_mapping[49]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[50]],
                Argument: &self.row.columns[self.index_mapping[51]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[52]],
                Argument: &self.row.columns[self.index_mapping[53]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[54]],
                Argument: &self.row.columns[self.index_mapping[55]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[56]],
                Argument: &self.row.columns[self.index_mapping[57]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[58]],
                Argument: &self.row.columns[self.index_mapping[59]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[60]],
                Argument: &self.row.columns[self.index_mapping[61]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[62]],
                Argument: &self.row.columns[self.index_mapping[63]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[64]],
                Argument: &self.row.columns[self.index_mapping[65]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[66]],
                Argument: &self.row.columns[self.index_mapping[67]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[68]],
                Argument: &self.row.columns[self.index_mapping[69]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[70]],
                Argument: &self.row.columns[self.index_mapping[71]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[72]],
                Argument: &self.row.columns[self.index_mapping[73]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[74]],
                Argument: &self.row.columns[self.index_mapping[75]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[76]],
                Argument: &self.row.columns[self.index_mapping[77]],
            },
            StoryParamsElement {
                Instruction: &self.row.columns[self.index_mapping[78]],
                Argument: &self.row.columns[self.index_mapping[79]],
            },
        ]
    }
    pub fn StoryDefine(&'a self) -> [StoryDefineElement<'a>; 110] {
        [
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[80]],
                    &self.row.columns[self.index_mapping[81]],
                    &self.row.columns[self.index_mapping[82]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[83]],
                    &self.row.columns[self.index_mapping[84]],
                    &self.row.columns[self.index_mapping[85]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[86]],
                    &self.row.columns[self.index_mapping[87]],
                ],
                Sequence: &self.row.columns[self.index_mapping[88]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[89]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[90]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[91]],
                    &self.row.columns[self.index_mapping[92]],
                    &self.row.columns[self.index_mapping[93]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[94]],
                    &self.row.columns[self.index_mapping[95]],
                    &self.row.columns[self.index_mapping[96]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[97]],
                    &self.row.columns[self.index_mapping[98]],
                    &self.row.columns[self.index_mapping[99]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[100]],
                    &self.row.columns[self.index_mapping[101]],
                ],
                Sequence: &self.row.columns[self.index_mapping[102]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[103]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[104]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[105]],
                    &self.row.columns[self.index_mapping[106]],
                    &self.row.columns[self.index_mapping[107]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[108]],
                    &self.row.columns[self.index_mapping[109]],
                    &self.row.columns[self.index_mapping[110]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[111]],
                    &self.row.columns[self.index_mapping[112]],
                    &self.row.columns[self.index_mapping[113]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[114]],
                    &self.row.columns[self.index_mapping[115]],
                ],
                Sequence: &self.row.columns[self.index_mapping[116]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[117]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[118]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[119]],
                    &self.row.columns[self.index_mapping[120]],
                    &self.row.columns[self.index_mapping[121]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[122]],
                    &self.row.columns[self.index_mapping[123]],
                    &self.row.columns[self.index_mapping[124]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[125]],
                    &self.row.columns[self.index_mapping[126]],
                    &self.row.columns[self.index_mapping[127]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[128]],
                    &self.row.columns[self.index_mapping[129]],
                ],
                Sequence: &self.row.columns[self.index_mapping[130]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[131]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[132]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[133]],
                    &self.row.columns[self.index_mapping[134]],
                    &self.row.columns[self.index_mapping[135]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[136]],
                    &self.row.columns[self.index_mapping[137]],
                    &self.row.columns[self.index_mapping[138]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[139]],
                    &self.row.columns[self.index_mapping[140]],
                    &self.row.columns[self.index_mapping[141]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[142]],
                    &self.row.columns[self.index_mapping[143]],
                ],
                Sequence: &self.row.columns[self.index_mapping[144]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[145]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[146]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[147]],
                    &self.row.columns[self.index_mapping[148]],
                    &self.row.columns[self.index_mapping[149]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[150]],
                    &self.row.columns[self.index_mapping[151]],
                    &self.row.columns[self.index_mapping[152]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[153]],
                    &self.row.columns[self.index_mapping[154]],
                    &self.row.columns[self.index_mapping[155]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[156]],
                    &self.row.columns[self.index_mapping[157]],
                ],
                Sequence: &self.row.columns[self.index_mapping[158]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[159]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[160]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[161]],
                    &self.row.columns[self.index_mapping[162]],
                    &self.row.columns[self.index_mapping[163]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[164]],
                    &self.row.columns[self.index_mapping[165]],
                    &self.row.columns[self.index_mapping[166]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[167]],
                    &self.row.columns[self.index_mapping[168]],
                    &self.row.columns[self.index_mapping[169]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[170]],
                    &self.row.columns[self.index_mapping[171]],
                ],
                Sequence: &self.row.columns[self.index_mapping[172]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[173]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[174]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[175]],
                    &self.row.columns[self.index_mapping[176]],
                    &self.row.columns[self.index_mapping[177]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[178]],
                    &self.row.columns[self.index_mapping[179]],
                    &self.row.columns[self.index_mapping[180]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[181]],
                    &self.row.columns[self.index_mapping[182]],
                    &self.row.columns[self.index_mapping[183]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[184]],
                    &self.row.columns[self.index_mapping[185]],
                ],
                Sequence: &self.row.columns[self.index_mapping[186]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[187]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[188]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[189]],
                    &self.row.columns[self.index_mapping[190]],
                    &self.row.columns[self.index_mapping[191]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[192]],
                    &self.row.columns[self.index_mapping[193]],
                    &self.row.columns[self.index_mapping[194]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[195]],
                    &self.row.columns[self.index_mapping[196]],
                    &self.row.columns[self.index_mapping[197]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[198]],
                    &self.row.columns[self.index_mapping[199]],
                ],
                Sequence: &self.row.columns[self.index_mapping[200]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[201]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[202]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[203]],
                    &self.row.columns[self.index_mapping[204]],
                    &self.row.columns[self.index_mapping[205]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[206]],
                    &self.row.columns[self.index_mapping[207]],
                    &self.row.columns[self.index_mapping[208]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[209]],
                    &self.row.columns[self.index_mapping[210]],
                    &self.row.columns[self.index_mapping[211]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[212]],
                    &self.row.columns[self.index_mapping[213]],
                ],
                Sequence: &self.row.columns[self.index_mapping[214]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[215]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[216]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[217]],
                    &self.row.columns[self.index_mapping[218]],
                    &self.row.columns[self.index_mapping[219]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[220]],
                    &self.row.columns[self.index_mapping[221]],
                    &self.row.columns[self.index_mapping[222]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[223]],
                    &self.row.columns[self.index_mapping[224]],
                    &self.row.columns[self.index_mapping[225]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[226]],
                    &self.row.columns[self.index_mapping[227]],
                ],
                Sequence: &self.row.columns[self.index_mapping[228]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[229]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[230]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[231]],
                    &self.row.columns[self.index_mapping[232]],
                    &self.row.columns[self.index_mapping[233]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[234]],
                    &self.row.columns[self.index_mapping[235]],
                    &self.row.columns[self.index_mapping[236]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[237]],
                    &self.row.columns[self.index_mapping[238]],
                    &self.row.columns[self.index_mapping[239]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[240]],
                    &self.row.columns[self.index_mapping[241]],
                ],
                Sequence: &self.row.columns[self.index_mapping[242]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[243]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[244]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[245]],
                    &self.row.columns[self.index_mapping[246]],
                    &self.row.columns[self.index_mapping[247]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[248]],
                    &self.row.columns[self.index_mapping[249]],
                    &self.row.columns[self.index_mapping[250]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[251]],
                    &self.row.columns[self.index_mapping[252]],
                    &self.row.columns[self.index_mapping[253]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[254]],
                    &self.row.columns[self.index_mapping[255]],
                ],
                Sequence: &self.row.columns[self.index_mapping[256]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[257]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[258]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[259]],
                    &self.row.columns[self.index_mapping[260]],
                    &self.row.columns[self.index_mapping[261]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[262]],
                    &self.row.columns[self.index_mapping[263]],
                    &self.row.columns[self.index_mapping[264]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[265]],
                    &self.row.columns[self.index_mapping[266]],
                    &self.row.columns[self.index_mapping[267]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[268]],
                    &self.row.columns[self.index_mapping[269]],
                ],
                Sequence: &self.row.columns[self.index_mapping[270]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[271]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[272]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[273]],
                    &self.row.columns[self.index_mapping[274]],
                    &self.row.columns[self.index_mapping[275]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[276]],
                    &self.row.columns[self.index_mapping[277]],
                    &self.row.columns[self.index_mapping[278]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[279]],
                    &self.row.columns[self.index_mapping[280]],
                    &self.row.columns[self.index_mapping[281]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[282]],
                    &self.row.columns[self.index_mapping[283]],
                ],
                Sequence: &self.row.columns[self.index_mapping[284]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[285]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[286]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[287]],
                    &self.row.columns[self.index_mapping[288]],
                    &self.row.columns[self.index_mapping[289]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[290]],
                    &self.row.columns[self.index_mapping[291]],
                    &self.row.columns[self.index_mapping[292]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[293]],
                    &self.row.columns[self.index_mapping[294]],
                    &self.row.columns[self.index_mapping[295]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[296]],
                    &self.row.columns[self.index_mapping[297]],
                ],
                Sequence: &self.row.columns[self.index_mapping[298]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[299]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[300]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[301]],
                    &self.row.columns[self.index_mapping[302]],
                    &self.row.columns[self.index_mapping[303]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[304]],
                    &self.row.columns[self.index_mapping[305]],
                    &self.row.columns[self.index_mapping[306]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[307]],
                    &self.row.columns[self.index_mapping[308]],
                    &self.row.columns[self.index_mapping[309]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[310]],
                    &self.row.columns[self.index_mapping[311]],
                ],
                Sequence: &self.row.columns[self.index_mapping[312]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[313]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[314]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[315]],
                    &self.row.columns[self.index_mapping[316]],
                    &self.row.columns[self.index_mapping[317]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[318]],
                    &self.row.columns[self.index_mapping[319]],
                    &self.row.columns[self.index_mapping[320]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[321]],
                    &self.row.columns[self.index_mapping[322]],
                    &self.row.columns[self.index_mapping[323]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[324]],
                    &self.row.columns[self.index_mapping[325]],
                ],
                Sequence: &self.row.columns[self.index_mapping[326]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[327]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[328]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[329]],
                    &self.row.columns[self.index_mapping[330]],
                    &self.row.columns[self.index_mapping[331]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[332]],
                    &self.row.columns[self.index_mapping[333]],
                    &self.row.columns[self.index_mapping[334]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[335]],
                    &self.row.columns[self.index_mapping[336]],
                    &self.row.columns[self.index_mapping[337]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[338]],
                    &self.row.columns[self.index_mapping[339]],
                ],
                Sequence: &self.row.columns[self.index_mapping[340]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[341]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[342]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[343]],
                    &self.row.columns[self.index_mapping[344]],
                    &self.row.columns[self.index_mapping[345]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[346]],
                    &self.row.columns[self.index_mapping[347]],
                    &self.row.columns[self.index_mapping[348]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[349]],
                    &self.row.columns[self.index_mapping[350]],
                    &self.row.columns[self.index_mapping[351]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[352]],
                    &self.row.columns[self.index_mapping[353]],
                ],
                Sequence: &self.row.columns[self.index_mapping[354]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[355]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[356]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[357]],
                    &self.row.columns[self.index_mapping[358]],
                    &self.row.columns[self.index_mapping[359]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[360]],
                    &self.row.columns[self.index_mapping[361]],
                    &self.row.columns[self.index_mapping[362]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[363]],
                    &self.row.columns[self.index_mapping[364]],
                    &self.row.columns[self.index_mapping[365]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[366]],
                    &self.row.columns[self.index_mapping[367]],
                ],
                Sequence: &self.row.columns[self.index_mapping[368]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[369]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[370]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[371]],
                    &self.row.columns[self.index_mapping[372]],
                    &self.row.columns[self.index_mapping[373]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[374]],
                    &self.row.columns[self.index_mapping[375]],
                    &self.row.columns[self.index_mapping[376]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[377]],
                    &self.row.columns[self.index_mapping[378]],
                    &self.row.columns[self.index_mapping[379]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[380]],
                    &self.row.columns[self.index_mapping[381]],
                ],
                Sequence: &self.row.columns[self.index_mapping[382]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[383]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[384]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[385]],
                    &self.row.columns[self.index_mapping[386]],
                    &self.row.columns[self.index_mapping[387]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[388]],
                    &self.row.columns[self.index_mapping[389]],
                    &self.row.columns[self.index_mapping[390]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[391]],
                    &self.row.columns[self.index_mapping[392]],
                    &self.row.columns[self.index_mapping[393]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[394]],
                    &self.row.columns[self.index_mapping[395]],
                ],
                Sequence: &self.row.columns[self.index_mapping[396]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[397]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[398]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[399]],
                    &self.row.columns[self.index_mapping[400]],
                    &self.row.columns[self.index_mapping[401]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[402]],
                    &self.row.columns[self.index_mapping[403]],
                    &self.row.columns[self.index_mapping[404]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[405]],
                    &self.row.columns[self.index_mapping[406]],
                    &self.row.columns[self.index_mapping[407]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[408]],
                    &self.row.columns[self.index_mapping[409]],
                ],
                Sequence: &self.row.columns[self.index_mapping[410]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[411]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[412]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[413]],
                    &self.row.columns[self.index_mapping[414]],
                    &self.row.columns[self.index_mapping[415]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[416]],
                    &self.row.columns[self.index_mapping[417]],
                    &self.row.columns[self.index_mapping[418]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[419]],
                    &self.row.columns[self.index_mapping[420]],
                    &self.row.columns[self.index_mapping[421]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[422]],
                    &self.row.columns[self.index_mapping[423]],
                ],
                Sequence: &self.row.columns[self.index_mapping[424]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[425]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[426]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[427]],
                    &self.row.columns[self.index_mapping[428]],
                    &self.row.columns[self.index_mapping[429]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[430]],
                    &self.row.columns[self.index_mapping[431]],
                    &self.row.columns[self.index_mapping[432]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[433]],
                    &self.row.columns[self.index_mapping[434]],
                    &self.row.columns[self.index_mapping[435]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[436]],
                    &self.row.columns[self.index_mapping[437]],
                ],
                Sequence: &self.row.columns[self.index_mapping[438]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[439]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[440]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[441]],
                    &self.row.columns[self.index_mapping[442]],
                    &self.row.columns[self.index_mapping[443]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[444]],
                    &self.row.columns[self.index_mapping[445]],
                    &self.row.columns[self.index_mapping[446]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[447]],
                    &self.row.columns[self.index_mapping[448]],
                    &self.row.columns[self.index_mapping[449]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[450]],
                    &self.row.columns[self.index_mapping[451]],
                ],
                Sequence: &self.row.columns[self.index_mapping[452]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[453]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[454]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[455]],
                    &self.row.columns[self.index_mapping[456]],
                    &self.row.columns[self.index_mapping[457]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[458]],
                    &self.row.columns[self.index_mapping[459]],
                    &self.row.columns[self.index_mapping[460]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[461]],
                    &self.row.columns[self.index_mapping[462]],
                    &self.row.columns[self.index_mapping[463]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[464]],
                    &self.row.columns[self.index_mapping[465]],
                ],
                Sequence: &self.row.columns[self.index_mapping[466]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[467]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[468]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[469]],
                    &self.row.columns[self.index_mapping[470]],
                    &self.row.columns[self.index_mapping[471]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[472]],
                    &self.row.columns[self.index_mapping[473]],
                    &self.row.columns[self.index_mapping[474]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[475]],
                    &self.row.columns[self.index_mapping[476]],
                    &self.row.columns[self.index_mapping[477]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[478]],
                    &self.row.columns[self.index_mapping[479]],
                ],
                Sequence: &self.row.columns[self.index_mapping[480]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[481]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[482]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[483]],
                    &self.row.columns[self.index_mapping[484]],
                    &self.row.columns[self.index_mapping[485]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[486]],
                    &self.row.columns[self.index_mapping[487]],
                    &self.row.columns[self.index_mapping[488]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[489]],
                    &self.row.columns[self.index_mapping[490]],
                    &self.row.columns[self.index_mapping[491]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[492]],
                    &self.row.columns[self.index_mapping[493]],
                ],
                Sequence: &self.row.columns[self.index_mapping[494]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[495]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[496]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[497]],
                    &self.row.columns[self.index_mapping[498]],
                    &self.row.columns[self.index_mapping[499]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[500]],
                    &self.row.columns[self.index_mapping[501]],
                    &self.row.columns[self.index_mapping[502]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[503]],
                    &self.row.columns[self.index_mapping[504]],
                    &self.row.columns[self.index_mapping[505]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[506]],
                    &self.row.columns[self.index_mapping[507]],
                ],
                Sequence: &self.row.columns[self.index_mapping[508]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[509]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[510]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[511]],
                    &self.row.columns[self.index_mapping[512]],
                    &self.row.columns[self.index_mapping[513]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[514]],
                    &self.row.columns[self.index_mapping[515]],
                    &self.row.columns[self.index_mapping[516]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[517]],
                    &self.row.columns[self.index_mapping[518]],
                    &self.row.columns[self.index_mapping[519]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[520]],
                    &self.row.columns[self.index_mapping[521]],
                ],
                Sequence: &self.row.columns[self.index_mapping[522]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[523]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[524]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[525]],
                    &self.row.columns[self.index_mapping[526]],
                    &self.row.columns[self.index_mapping[527]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[528]],
                    &self.row.columns[self.index_mapping[529]],
                    &self.row.columns[self.index_mapping[530]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[531]],
                    &self.row.columns[self.index_mapping[532]],
                    &self.row.columns[self.index_mapping[533]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[534]],
                    &self.row.columns[self.index_mapping[535]],
                ],
                Sequence: &self.row.columns[self.index_mapping[536]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[537]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[538]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[539]],
                    &self.row.columns[self.index_mapping[540]],
                    &self.row.columns[self.index_mapping[541]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[542]],
                    &self.row.columns[self.index_mapping[543]],
                    &self.row.columns[self.index_mapping[544]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[545]],
                    &self.row.columns[self.index_mapping[546]],
                    &self.row.columns[self.index_mapping[547]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[548]],
                    &self.row.columns[self.index_mapping[549]],
                ],
                Sequence: &self.row.columns[self.index_mapping[550]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[551]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[552]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[553]],
                    &self.row.columns[self.index_mapping[554]],
                    &self.row.columns[self.index_mapping[555]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[556]],
                    &self.row.columns[self.index_mapping[557]],
                    &self.row.columns[self.index_mapping[558]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[559]],
                    &self.row.columns[self.index_mapping[560]],
                    &self.row.columns[self.index_mapping[561]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[562]],
                    &self.row.columns[self.index_mapping[563]],
                ],
                Sequence: &self.row.columns[self.index_mapping[564]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[565]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[566]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[567]],
                    &self.row.columns[self.index_mapping[568]],
                    &self.row.columns[self.index_mapping[569]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[570]],
                    &self.row.columns[self.index_mapping[571]],
                    &self.row.columns[self.index_mapping[572]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[573]],
                    &self.row.columns[self.index_mapping[574]],
                    &self.row.columns[self.index_mapping[575]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[576]],
                    &self.row.columns[self.index_mapping[577]],
                ],
                Sequence: &self.row.columns[self.index_mapping[578]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[579]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[580]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[581]],
                    &self.row.columns[self.index_mapping[582]],
                    &self.row.columns[self.index_mapping[583]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[584]],
                    &self.row.columns[self.index_mapping[585]],
                    &self.row.columns[self.index_mapping[586]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[587]],
                    &self.row.columns[self.index_mapping[588]],
                    &self.row.columns[self.index_mapping[589]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[590]],
                    &self.row.columns[self.index_mapping[591]],
                ],
                Sequence: &self.row.columns[self.index_mapping[592]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[593]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[594]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[595]],
                    &self.row.columns[self.index_mapping[596]],
                    &self.row.columns[self.index_mapping[597]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[598]],
                    &self.row.columns[self.index_mapping[599]],
                    &self.row.columns[self.index_mapping[600]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[601]],
                    &self.row.columns[self.index_mapping[602]],
                    &self.row.columns[self.index_mapping[603]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[604]],
                    &self.row.columns[self.index_mapping[605]],
                ],
                Sequence: &self.row.columns[self.index_mapping[606]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[607]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[608]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[609]],
                    &self.row.columns[self.index_mapping[610]],
                    &self.row.columns[self.index_mapping[611]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[612]],
                    &self.row.columns[self.index_mapping[613]],
                    &self.row.columns[self.index_mapping[614]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[615]],
                    &self.row.columns[self.index_mapping[616]],
                    &self.row.columns[self.index_mapping[617]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[618]],
                    &self.row.columns[self.index_mapping[619]],
                ],
                Sequence: &self.row.columns[self.index_mapping[620]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[621]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[622]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[623]],
                    &self.row.columns[self.index_mapping[624]],
                    &self.row.columns[self.index_mapping[625]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[626]],
                    &self.row.columns[self.index_mapping[627]],
                    &self.row.columns[self.index_mapping[628]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[629]],
                    &self.row.columns[self.index_mapping[630]],
                    &self.row.columns[self.index_mapping[631]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[632]],
                    &self.row.columns[self.index_mapping[633]],
                ],
                Sequence: &self.row.columns[self.index_mapping[634]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[635]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[636]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[637]],
                    &self.row.columns[self.index_mapping[638]],
                    &self.row.columns[self.index_mapping[639]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[640]],
                    &self.row.columns[self.index_mapping[641]],
                    &self.row.columns[self.index_mapping[642]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[643]],
                    &self.row.columns[self.index_mapping[644]],
                    &self.row.columns[self.index_mapping[645]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[646]],
                    &self.row.columns[self.index_mapping[647]],
                ],
                Sequence: &self.row.columns[self.index_mapping[648]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[649]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[650]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[651]],
                    &self.row.columns[self.index_mapping[652]],
                    &self.row.columns[self.index_mapping[653]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[654]],
                    &self.row.columns[self.index_mapping[655]],
                    &self.row.columns[self.index_mapping[656]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[657]],
                    &self.row.columns[self.index_mapping[658]],
                    &self.row.columns[self.index_mapping[659]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[660]],
                    &self.row.columns[self.index_mapping[661]],
                ],
                Sequence: &self.row.columns[self.index_mapping[662]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[663]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[664]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[665]],
                    &self.row.columns[self.index_mapping[666]],
                    &self.row.columns[self.index_mapping[667]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[668]],
                    &self.row.columns[self.index_mapping[669]],
                    &self.row.columns[self.index_mapping[670]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[671]],
                    &self.row.columns[self.index_mapping[672]],
                    &self.row.columns[self.index_mapping[673]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[674]],
                    &self.row.columns[self.index_mapping[675]],
                ],
                Sequence: &self.row.columns[self.index_mapping[676]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[677]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[678]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[679]],
                    &self.row.columns[self.index_mapping[680]],
                    &self.row.columns[self.index_mapping[681]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[682]],
                    &self.row.columns[self.index_mapping[683]],
                    &self.row.columns[self.index_mapping[684]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[685]],
                    &self.row.columns[self.index_mapping[686]],
                    &self.row.columns[self.index_mapping[687]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[688]],
                    &self.row.columns[self.index_mapping[689]],
                ],
                Sequence: &self.row.columns[self.index_mapping[690]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[691]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[692]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[693]],
                    &self.row.columns[self.index_mapping[694]],
                    &self.row.columns[self.index_mapping[695]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[696]],
                    &self.row.columns[self.index_mapping[697]],
                    &self.row.columns[self.index_mapping[698]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[699]],
                    &self.row.columns[self.index_mapping[700]],
                    &self.row.columns[self.index_mapping[701]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[702]],
                    &self.row.columns[self.index_mapping[703]],
                ],
                Sequence: &self.row.columns[self.index_mapping[704]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[705]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[706]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[707]],
                    &self.row.columns[self.index_mapping[708]],
                    &self.row.columns[self.index_mapping[709]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[710]],
                    &self.row.columns[self.index_mapping[711]],
                    &self.row.columns[self.index_mapping[712]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[713]],
                    &self.row.columns[self.index_mapping[714]],
                    &self.row.columns[self.index_mapping[715]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[716]],
                    &self.row.columns[self.index_mapping[717]],
                ],
                Sequence: &self.row.columns[self.index_mapping[718]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[719]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[720]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[721]],
                    &self.row.columns[self.index_mapping[722]],
                    &self.row.columns[self.index_mapping[723]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[724]],
                    &self.row.columns[self.index_mapping[725]],
                    &self.row.columns[self.index_mapping[726]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[727]],
                    &self.row.columns[self.index_mapping[728]],
                    &self.row.columns[self.index_mapping[729]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[730]],
                    &self.row.columns[self.index_mapping[731]],
                ],
                Sequence: &self.row.columns[self.index_mapping[732]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[733]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[734]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[735]],
                    &self.row.columns[self.index_mapping[736]],
                    &self.row.columns[self.index_mapping[737]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[738]],
                    &self.row.columns[self.index_mapping[739]],
                    &self.row.columns[self.index_mapping[740]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[741]],
                    &self.row.columns[self.index_mapping[742]],
                    &self.row.columns[self.index_mapping[743]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[744]],
                    &self.row.columns[self.index_mapping[745]],
                ],
                Sequence: &self.row.columns[self.index_mapping[746]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[747]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[748]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[749]],
                    &self.row.columns[self.index_mapping[750]],
                    &self.row.columns[self.index_mapping[751]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[752]],
                    &self.row.columns[self.index_mapping[753]],
                    &self.row.columns[self.index_mapping[754]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[755]],
                    &self.row.columns[self.index_mapping[756]],
                    &self.row.columns[self.index_mapping[757]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[758]],
                    &self.row.columns[self.index_mapping[759]],
                ],
                Sequence: &self.row.columns[self.index_mapping[760]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[761]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[762]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[763]],
                    &self.row.columns[self.index_mapping[764]],
                    &self.row.columns[self.index_mapping[765]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[766]],
                    &self.row.columns[self.index_mapping[767]],
                    &self.row.columns[self.index_mapping[768]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[769]],
                    &self.row.columns[self.index_mapping[770]],
                    &self.row.columns[self.index_mapping[771]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[772]],
                    &self.row.columns[self.index_mapping[773]],
                ],
                Sequence: &self.row.columns[self.index_mapping[774]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[775]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[776]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[777]],
                    &self.row.columns[self.index_mapping[778]],
                    &self.row.columns[self.index_mapping[779]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[780]],
                    &self.row.columns[self.index_mapping[781]],
                    &self.row.columns[self.index_mapping[782]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[783]],
                    &self.row.columns[self.index_mapping[784]],
                    &self.row.columns[self.index_mapping[785]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[786]],
                    &self.row.columns[self.index_mapping[787]],
                ],
                Sequence: &self.row.columns[self.index_mapping[788]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[789]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[790]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[791]],
                    &self.row.columns[self.index_mapping[792]],
                    &self.row.columns[self.index_mapping[793]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[794]],
                    &self.row.columns[self.index_mapping[795]],
                    &self.row.columns[self.index_mapping[796]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[797]],
                    &self.row.columns[self.index_mapping[798]],
                    &self.row.columns[self.index_mapping[799]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[800]],
                    &self.row.columns[self.index_mapping[801]],
                ],
                Sequence: &self.row.columns[self.index_mapping[802]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[803]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[804]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[805]],
                    &self.row.columns[self.index_mapping[806]],
                    &self.row.columns[self.index_mapping[807]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[808]],
                    &self.row.columns[self.index_mapping[809]],
                    &self.row.columns[self.index_mapping[810]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[811]],
                    &self.row.columns[self.index_mapping[812]],
                    &self.row.columns[self.index_mapping[813]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[814]],
                    &self.row.columns[self.index_mapping[815]],
                ],
                Sequence: &self.row.columns[self.index_mapping[816]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[817]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[818]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[819]],
                    &self.row.columns[self.index_mapping[820]],
                    &self.row.columns[self.index_mapping[821]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[822]],
                    &self.row.columns[self.index_mapping[823]],
                    &self.row.columns[self.index_mapping[824]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[825]],
                    &self.row.columns[self.index_mapping[826]],
                    &self.row.columns[self.index_mapping[827]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[828]],
                    &self.row.columns[self.index_mapping[829]],
                ],
                Sequence: &self.row.columns[self.index_mapping[830]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[831]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[832]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[833]],
                    &self.row.columns[self.index_mapping[834]],
                    &self.row.columns[self.index_mapping[835]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[836]],
                    &self.row.columns[self.index_mapping[837]],
                    &self.row.columns[self.index_mapping[838]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[839]],
                    &self.row.columns[self.index_mapping[840]],
                    &self.row.columns[self.index_mapping[841]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[842]],
                    &self.row.columns[self.index_mapping[843]],
                ],
                Sequence: &self.row.columns[self.index_mapping[844]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[845]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[846]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[847]],
                    &self.row.columns[self.index_mapping[848]],
                    &self.row.columns[self.index_mapping[849]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[850]],
                    &self.row.columns[self.index_mapping[851]],
                    &self.row.columns[self.index_mapping[852]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[853]],
                    &self.row.columns[self.index_mapping[854]],
                    &self.row.columns[self.index_mapping[855]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[856]],
                    &self.row.columns[self.index_mapping[857]],
                ],
                Sequence: &self.row.columns[self.index_mapping[858]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[859]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[860]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[861]],
                    &self.row.columns[self.index_mapping[862]],
                    &self.row.columns[self.index_mapping[863]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[864]],
                    &self.row.columns[self.index_mapping[865]],
                    &self.row.columns[self.index_mapping[866]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[867]],
                    &self.row.columns[self.index_mapping[868]],
                    &self.row.columns[self.index_mapping[869]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[870]],
                    &self.row.columns[self.index_mapping[871]],
                ],
                Sequence: &self.row.columns[self.index_mapping[872]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[873]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[874]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[875]],
                    &self.row.columns[self.index_mapping[876]],
                    &self.row.columns[self.index_mapping[877]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[878]],
                    &self.row.columns[self.index_mapping[879]],
                    &self.row.columns[self.index_mapping[880]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[881]],
                    &self.row.columns[self.index_mapping[882]],
                    &self.row.columns[self.index_mapping[883]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[884]],
                    &self.row.columns[self.index_mapping[885]],
                ],
                Sequence: &self.row.columns[self.index_mapping[886]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[887]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[888]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[889]],
                    &self.row.columns[self.index_mapping[890]],
                    &self.row.columns[self.index_mapping[891]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[892]],
                    &self.row.columns[self.index_mapping[893]],
                    &self.row.columns[self.index_mapping[894]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[895]],
                    &self.row.columns[self.index_mapping[896]],
                    &self.row.columns[self.index_mapping[897]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[898]],
                    &self.row.columns[self.index_mapping[899]],
                ],
                Sequence: &self.row.columns[self.index_mapping[900]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[901]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[902]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[903]],
                    &self.row.columns[self.index_mapping[904]],
                    &self.row.columns[self.index_mapping[905]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[906]],
                    &self.row.columns[self.index_mapping[907]],
                    &self.row.columns[self.index_mapping[908]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[909]],
                    &self.row.columns[self.index_mapping[910]],
                    &self.row.columns[self.index_mapping[911]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[912]],
                    &self.row.columns[self.index_mapping[913]],
                ],
                Sequence: &self.row.columns[self.index_mapping[914]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[915]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[916]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[917]],
                    &self.row.columns[self.index_mapping[918]],
                    &self.row.columns[self.index_mapping[919]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[920]],
                    &self.row.columns[self.index_mapping[921]],
                    &self.row.columns[self.index_mapping[922]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[923]],
                    &self.row.columns[self.index_mapping[924]],
                    &self.row.columns[self.index_mapping[925]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[926]],
                    &self.row.columns[self.index_mapping[927]],
                ],
                Sequence: &self.row.columns[self.index_mapping[928]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[929]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[930]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[931]],
                    &self.row.columns[self.index_mapping[932]],
                    &self.row.columns[self.index_mapping[933]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[934]],
                    &self.row.columns[self.index_mapping[935]],
                    &self.row.columns[self.index_mapping[936]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[937]],
                    &self.row.columns[self.index_mapping[938]],
                    &self.row.columns[self.index_mapping[939]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[940]],
                    &self.row.columns[self.index_mapping[941]],
                ],
                Sequence: &self.row.columns[self.index_mapping[942]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[943]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[944]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[945]],
                    &self.row.columns[self.index_mapping[946]],
                    &self.row.columns[self.index_mapping[947]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[948]],
                    &self.row.columns[self.index_mapping[949]],
                    &self.row.columns[self.index_mapping[950]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[951]],
                    &self.row.columns[self.index_mapping[952]],
                    &self.row.columns[self.index_mapping[953]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[954]],
                    &self.row.columns[self.index_mapping[955]],
                ],
                Sequence: &self.row.columns[self.index_mapping[956]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[957]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[958]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[959]],
                    &self.row.columns[self.index_mapping[960]],
                    &self.row.columns[self.index_mapping[961]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[962]],
                    &self.row.columns[self.index_mapping[963]],
                    &self.row.columns[self.index_mapping[964]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[965]],
                    &self.row.columns[self.index_mapping[966]],
                    &self.row.columns[self.index_mapping[967]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[968]],
                    &self.row.columns[self.index_mapping[969]],
                ],
                Sequence: &self.row.columns[self.index_mapping[970]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[971]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[972]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[973]],
                    &self.row.columns[self.index_mapping[974]],
                    &self.row.columns[self.index_mapping[975]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[976]],
                    &self.row.columns[self.index_mapping[977]],
                    &self.row.columns[self.index_mapping[978]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[979]],
                    &self.row.columns[self.index_mapping[980]],
                    &self.row.columns[self.index_mapping[981]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[982]],
                    &self.row.columns[self.index_mapping[983]],
                ],
                Sequence: &self.row.columns[self.index_mapping[984]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[985]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[986]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[987]],
                    &self.row.columns[self.index_mapping[988]],
                    &self.row.columns[self.index_mapping[989]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[990]],
                    &self.row.columns[self.index_mapping[991]],
                    &self.row.columns[self.index_mapping[992]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[993]],
                    &self.row.columns[self.index_mapping[994]],
                    &self.row.columns[self.index_mapping[995]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[996]],
                    &self.row.columns[self.index_mapping[997]],
                ],
                Sequence: &self.row.columns[self.index_mapping[998]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[999]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1000]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1001]],
                    &self.row.columns[self.index_mapping[1002]],
                    &self.row.columns[self.index_mapping[1003]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1004]],
                    &self.row.columns[self.index_mapping[1005]],
                    &self.row.columns[self.index_mapping[1006]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1007]],
                    &self.row.columns[self.index_mapping[1008]],
                    &self.row.columns[self.index_mapping[1009]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1010]],
                    &self.row.columns[self.index_mapping[1011]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1012]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1013]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1014]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1015]],
                    &self.row.columns[self.index_mapping[1016]],
                    &self.row.columns[self.index_mapping[1017]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1018]],
                    &self.row.columns[self.index_mapping[1019]],
                    &self.row.columns[self.index_mapping[1020]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1021]],
                    &self.row.columns[self.index_mapping[1022]],
                    &self.row.columns[self.index_mapping[1023]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1024]],
                    &self.row.columns[self.index_mapping[1025]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1026]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1027]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1028]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1029]],
                    &self.row.columns[self.index_mapping[1030]],
                    &self.row.columns[self.index_mapping[1031]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1032]],
                    &self.row.columns[self.index_mapping[1033]],
                    &self.row.columns[self.index_mapping[1034]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1035]],
                    &self.row.columns[self.index_mapping[1036]],
                    &self.row.columns[self.index_mapping[1037]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1038]],
                    &self.row.columns[self.index_mapping[1039]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1040]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1041]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1042]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1043]],
                    &self.row.columns[self.index_mapping[1044]],
                    &self.row.columns[self.index_mapping[1045]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1046]],
                    &self.row.columns[self.index_mapping[1047]],
                    &self.row.columns[self.index_mapping[1048]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1049]],
                    &self.row.columns[self.index_mapping[1050]],
                    &self.row.columns[self.index_mapping[1051]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1052]],
                    &self.row.columns[self.index_mapping[1053]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1054]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1055]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1056]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1057]],
                    &self.row.columns[self.index_mapping[1058]],
                    &self.row.columns[self.index_mapping[1059]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1060]],
                    &self.row.columns[self.index_mapping[1061]],
                    &self.row.columns[self.index_mapping[1062]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1063]],
                    &self.row.columns[self.index_mapping[1064]],
                    &self.row.columns[self.index_mapping[1065]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1066]],
                    &self.row.columns[self.index_mapping[1067]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1068]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1069]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1070]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1071]],
                    &self.row.columns[self.index_mapping[1072]],
                    &self.row.columns[self.index_mapping[1073]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1074]],
                    &self.row.columns[self.index_mapping[1075]],
                    &self.row.columns[self.index_mapping[1076]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1077]],
                    &self.row.columns[self.index_mapping[1078]],
                    &self.row.columns[self.index_mapping[1079]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1080]],
                    &self.row.columns[self.index_mapping[1081]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1082]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1083]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1084]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1085]],
                    &self.row.columns[self.index_mapping[1086]],
                    &self.row.columns[self.index_mapping[1087]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1088]],
                    &self.row.columns[self.index_mapping[1089]],
                    &self.row.columns[self.index_mapping[1090]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1091]],
                    &self.row.columns[self.index_mapping[1092]],
                    &self.row.columns[self.index_mapping[1093]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1094]],
                    &self.row.columns[self.index_mapping[1095]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1096]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1097]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1098]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1099]],
                    &self.row.columns[self.index_mapping[1100]],
                    &self.row.columns[self.index_mapping[1101]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1102]],
                    &self.row.columns[self.index_mapping[1103]],
                    &self.row.columns[self.index_mapping[1104]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1105]],
                    &self.row.columns[self.index_mapping[1106]],
                    &self.row.columns[self.index_mapping[1107]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1108]],
                    &self.row.columns[self.index_mapping[1109]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1110]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1111]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1112]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1113]],
                    &self.row.columns[self.index_mapping[1114]],
                    &self.row.columns[self.index_mapping[1115]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1116]],
                    &self.row.columns[self.index_mapping[1117]],
                    &self.row.columns[self.index_mapping[1118]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1119]],
                    &self.row.columns[self.index_mapping[1120]],
                    &self.row.columns[self.index_mapping[1121]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1122]],
                    &self.row.columns[self.index_mapping[1123]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1124]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1125]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1126]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1127]],
                    &self.row.columns[self.index_mapping[1128]],
                    &self.row.columns[self.index_mapping[1129]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1130]],
                    &self.row.columns[self.index_mapping[1131]],
                    &self.row.columns[self.index_mapping[1132]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1133]],
                    &self.row.columns[self.index_mapping[1134]],
                    &self.row.columns[self.index_mapping[1135]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1136]],
                    &self.row.columns[self.index_mapping[1137]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1138]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1139]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1140]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1141]],
                    &self.row.columns[self.index_mapping[1142]],
                    &self.row.columns[self.index_mapping[1143]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1144]],
                    &self.row.columns[self.index_mapping[1145]],
                    &self.row.columns[self.index_mapping[1146]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1147]],
                    &self.row.columns[self.index_mapping[1148]],
                    &self.row.columns[self.index_mapping[1149]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1150]],
                    &self.row.columns[self.index_mapping[1151]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1152]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1153]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1154]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1155]],
                    &self.row.columns[self.index_mapping[1156]],
                    &self.row.columns[self.index_mapping[1157]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1158]],
                    &self.row.columns[self.index_mapping[1159]],
                    &self.row.columns[self.index_mapping[1160]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1161]],
                    &self.row.columns[self.index_mapping[1162]],
                    &self.row.columns[self.index_mapping[1163]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1164]],
                    &self.row.columns[self.index_mapping[1165]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1166]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1167]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1168]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1169]],
                    &self.row.columns[self.index_mapping[1170]],
                    &self.row.columns[self.index_mapping[1171]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1172]],
                    &self.row.columns[self.index_mapping[1173]],
                    &self.row.columns[self.index_mapping[1174]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1175]],
                    &self.row.columns[self.index_mapping[1176]],
                    &self.row.columns[self.index_mapping[1177]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1178]],
                    &self.row.columns[self.index_mapping[1179]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1180]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1181]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1182]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1183]],
                    &self.row.columns[self.index_mapping[1184]],
                    &self.row.columns[self.index_mapping[1185]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1186]],
                    &self.row.columns[self.index_mapping[1187]],
                    &self.row.columns[self.index_mapping[1188]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1189]],
                    &self.row.columns[self.index_mapping[1190]],
                    &self.row.columns[self.index_mapping[1191]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1192]],
                    &self.row.columns[self.index_mapping[1193]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1194]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1195]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1196]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1197]],
                    &self.row.columns[self.index_mapping[1198]],
                    &self.row.columns[self.index_mapping[1199]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1200]],
                    &self.row.columns[self.index_mapping[1201]],
                    &self.row.columns[self.index_mapping[1202]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1203]],
                    &self.row.columns[self.index_mapping[1204]],
                    &self.row.columns[self.index_mapping[1205]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1206]],
                    &self.row.columns[self.index_mapping[1207]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1208]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1209]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1210]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1211]],
                    &self.row.columns[self.index_mapping[1212]],
                    &self.row.columns[self.index_mapping[1213]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1214]],
                    &self.row.columns[self.index_mapping[1215]],
                    &self.row.columns[self.index_mapping[1216]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1217]],
                    &self.row.columns[self.index_mapping[1218]],
                    &self.row.columns[self.index_mapping[1219]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1220]],
                    &self.row.columns[self.index_mapping[1221]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1222]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1223]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1224]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1225]],
                    &self.row.columns[self.index_mapping[1226]],
                    &self.row.columns[self.index_mapping[1227]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1228]],
                    &self.row.columns[self.index_mapping[1229]],
                    &self.row.columns[self.index_mapping[1230]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1231]],
                    &self.row.columns[self.index_mapping[1232]],
                    &self.row.columns[self.index_mapping[1233]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1234]],
                    &self.row.columns[self.index_mapping[1235]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1236]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1237]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1238]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1239]],
                    &self.row.columns[self.index_mapping[1240]],
                    &self.row.columns[self.index_mapping[1241]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1242]],
                    &self.row.columns[self.index_mapping[1243]],
                    &self.row.columns[self.index_mapping[1244]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1245]],
                    &self.row.columns[self.index_mapping[1246]],
                    &self.row.columns[self.index_mapping[1247]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1248]],
                    &self.row.columns[self.index_mapping[1249]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1250]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1251]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1252]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1253]],
                    &self.row.columns[self.index_mapping[1254]],
                    &self.row.columns[self.index_mapping[1255]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1256]],
                    &self.row.columns[self.index_mapping[1257]],
                    &self.row.columns[self.index_mapping[1258]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1259]],
                    &self.row.columns[self.index_mapping[1260]],
                    &self.row.columns[self.index_mapping[1261]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1262]],
                    &self.row.columns[self.index_mapping[1263]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1264]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1265]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1266]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1267]],
                    &self.row.columns[self.index_mapping[1268]],
                    &self.row.columns[self.index_mapping[1269]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1270]],
                    &self.row.columns[self.index_mapping[1271]],
                    &self.row.columns[self.index_mapping[1272]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1273]],
                    &self.row.columns[self.index_mapping[1274]],
                    &self.row.columns[self.index_mapping[1275]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1276]],
                    &self.row.columns[self.index_mapping[1277]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1278]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1279]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1280]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1281]],
                    &self.row.columns[self.index_mapping[1282]],
                    &self.row.columns[self.index_mapping[1283]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1284]],
                    &self.row.columns[self.index_mapping[1285]],
                    &self.row.columns[self.index_mapping[1286]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1287]],
                    &self.row.columns[self.index_mapping[1288]],
                    &self.row.columns[self.index_mapping[1289]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1290]],
                    &self.row.columns[self.index_mapping[1291]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1292]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1293]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1294]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1295]],
                    &self.row.columns[self.index_mapping[1296]],
                    &self.row.columns[self.index_mapping[1297]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1298]],
                    &self.row.columns[self.index_mapping[1299]],
                    &self.row.columns[self.index_mapping[1300]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1301]],
                    &self.row.columns[self.index_mapping[1302]],
                    &self.row.columns[self.index_mapping[1303]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1304]],
                    &self.row.columns[self.index_mapping[1305]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1306]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1307]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1308]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1309]],
                    &self.row.columns[self.index_mapping[1310]],
                    &self.row.columns[self.index_mapping[1311]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1312]],
                    &self.row.columns[self.index_mapping[1313]],
                    &self.row.columns[self.index_mapping[1314]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1315]],
                    &self.row.columns[self.index_mapping[1316]],
                    &self.row.columns[self.index_mapping[1317]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1318]],
                    &self.row.columns[self.index_mapping[1319]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1320]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1321]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1322]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1323]],
                    &self.row.columns[self.index_mapping[1324]],
                    &self.row.columns[self.index_mapping[1325]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1326]],
                    &self.row.columns[self.index_mapping[1327]],
                    &self.row.columns[self.index_mapping[1328]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1329]],
                    &self.row.columns[self.index_mapping[1330]],
                    &self.row.columns[self.index_mapping[1331]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1332]],
                    &self.row.columns[self.index_mapping[1333]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1334]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1335]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1336]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1337]],
                    &self.row.columns[self.index_mapping[1338]],
                    &self.row.columns[self.index_mapping[1339]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1340]],
                    &self.row.columns[self.index_mapping[1341]],
                    &self.row.columns[self.index_mapping[1342]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1343]],
                    &self.row.columns[self.index_mapping[1344]],
                    &self.row.columns[self.index_mapping[1345]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1346]],
                    &self.row.columns[self.index_mapping[1347]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1348]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1349]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1350]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1351]],
                    &self.row.columns[self.index_mapping[1352]],
                    &self.row.columns[self.index_mapping[1353]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1354]],
                    &self.row.columns[self.index_mapping[1355]],
                    &self.row.columns[self.index_mapping[1356]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1357]],
                    &self.row.columns[self.index_mapping[1358]],
                    &self.row.columns[self.index_mapping[1359]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1360]],
                    &self.row.columns[self.index_mapping[1361]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1362]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1363]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1364]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1365]],
                    &self.row.columns[self.index_mapping[1366]],
                    &self.row.columns[self.index_mapping[1367]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1368]],
                    &self.row.columns[self.index_mapping[1369]],
                    &self.row.columns[self.index_mapping[1370]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1371]],
                    &self.row.columns[self.index_mapping[1372]],
                    &self.row.columns[self.index_mapping[1373]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1374]],
                    &self.row.columns[self.index_mapping[1375]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1376]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1377]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1378]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1379]],
                    &self.row.columns[self.index_mapping[1380]],
                    &self.row.columns[self.index_mapping[1381]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1382]],
                    &self.row.columns[self.index_mapping[1383]],
                    &self.row.columns[self.index_mapping[1384]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1385]],
                    &self.row.columns[self.index_mapping[1386]],
                    &self.row.columns[self.index_mapping[1387]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1388]],
                    &self.row.columns[self.index_mapping[1389]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1390]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1391]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1392]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1393]],
                    &self.row.columns[self.index_mapping[1394]],
                    &self.row.columns[self.index_mapping[1395]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1396]],
                    &self.row.columns[self.index_mapping[1397]],
                    &self.row.columns[self.index_mapping[1398]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1399]],
                    &self.row.columns[self.index_mapping[1400]],
                    &self.row.columns[self.index_mapping[1401]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1402]],
                    &self.row.columns[self.index_mapping[1403]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1404]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1405]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1406]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1407]],
                    &self.row.columns[self.index_mapping[1408]],
                    &self.row.columns[self.index_mapping[1409]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1410]],
                    &self.row.columns[self.index_mapping[1411]],
                    &self.row.columns[self.index_mapping[1412]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1413]],
                    &self.row.columns[self.index_mapping[1414]],
                    &self.row.columns[self.index_mapping[1415]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1416]],
                    &self.row.columns[self.index_mapping[1417]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1418]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1419]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1420]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1421]],
                    &self.row.columns[self.index_mapping[1422]],
                    &self.row.columns[self.index_mapping[1423]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1424]],
                    &self.row.columns[self.index_mapping[1425]],
                    &self.row.columns[self.index_mapping[1426]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1427]],
                    &self.row.columns[self.index_mapping[1428]],
                    &self.row.columns[self.index_mapping[1429]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1430]],
                    &self.row.columns[self.index_mapping[1431]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1432]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1433]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1434]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1435]],
                    &self.row.columns[self.index_mapping[1436]],
                    &self.row.columns[self.index_mapping[1437]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1438]],
                    &self.row.columns[self.index_mapping[1439]],
                    &self.row.columns[self.index_mapping[1440]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1441]],
                    &self.row.columns[self.index_mapping[1442]],
                    &self.row.columns[self.index_mapping[1443]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1444]],
                    &self.row.columns[self.index_mapping[1445]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1446]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1447]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1448]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1449]],
                    &self.row.columns[self.index_mapping[1450]],
                    &self.row.columns[self.index_mapping[1451]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1452]],
                    &self.row.columns[self.index_mapping[1453]],
                    &self.row.columns[self.index_mapping[1454]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1455]],
                    &self.row.columns[self.index_mapping[1456]],
                    &self.row.columns[self.index_mapping[1457]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1458]],
                    &self.row.columns[self.index_mapping[1459]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1460]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1461]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1462]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1463]],
                    &self.row.columns[self.index_mapping[1464]],
                    &self.row.columns[self.index_mapping[1465]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1466]],
                    &self.row.columns[self.index_mapping[1467]],
                    &self.row.columns[self.index_mapping[1468]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1469]],
                    &self.row.columns[self.index_mapping[1470]],
                    &self.row.columns[self.index_mapping[1471]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1472]],
                    &self.row.columns[self.index_mapping[1473]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1474]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1475]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1476]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1477]],
                    &self.row.columns[self.index_mapping[1478]],
                    &self.row.columns[self.index_mapping[1479]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1480]],
                    &self.row.columns[self.index_mapping[1481]],
                    &self.row.columns[self.index_mapping[1482]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1483]],
                    &self.row.columns[self.index_mapping[1484]],
                    &self.row.columns[self.index_mapping[1485]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1486]],
                    &self.row.columns[self.index_mapping[1487]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1488]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1489]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1490]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1491]],
                    &self.row.columns[self.index_mapping[1492]],
                    &self.row.columns[self.index_mapping[1493]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1494]],
                    &self.row.columns[self.index_mapping[1495]],
                    &self.row.columns[self.index_mapping[1496]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1497]],
                    &self.row.columns[self.index_mapping[1498]],
                    &self.row.columns[self.index_mapping[1499]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1500]],
                    &self.row.columns[self.index_mapping[1501]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1502]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1503]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1504]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1505]],
                    &self.row.columns[self.index_mapping[1506]],
                    &self.row.columns[self.index_mapping[1507]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1508]],
                    &self.row.columns[self.index_mapping[1509]],
                    &self.row.columns[self.index_mapping[1510]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1511]],
                    &self.row.columns[self.index_mapping[1512]],
                    &self.row.columns[self.index_mapping[1513]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1514]],
                    &self.row.columns[self.index_mapping[1515]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1516]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1517]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1518]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1519]],
                    &self.row.columns[self.index_mapping[1520]],
                    &self.row.columns[self.index_mapping[1521]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1522]],
                    &self.row.columns[self.index_mapping[1523]],
                    &self.row.columns[self.index_mapping[1524]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1525]],
                    &self.row.columns[self.index_mapping[1526]],
                    &self.row.columns[self.index_mapping[1527]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1528]],
                    &self.row.columns[self.index_mapping[1529]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1530]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1531]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1532]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1533]],
                    &self.row.columns[self.index_mapping[1534]],
                    &self.row.columns[self.index_mapping[1535]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1536]],
                    &self.row.columns[self.index_mapping[1537]],
                    &self.row.columns[self.index_mapping[1538]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1539]],
                    &self.row.columns[self.index_mapping[1540]],
                    &self.row.columns[self.index_mapping[1541]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1542]],
                    &self.row.columns[self.index_mapping[1543]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1544]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1545]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1546]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1547]],
                    &self.row.columns[self.index_mapping[1548]],
                    &self.row.columns[self.index_mapping[1549]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1550]],
                    &self.row.columns[self.index_mapping[1551]],
                    &self.row.columns[self.index_mapping[1552]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1553]],
                    &self.row.columns[self.index_mapping[1554]],
                    &self.row.columns[self.index_mapping[1555]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1556]],
                    &self.row.columns[self.index_mapping[1557]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1558]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1559]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1560]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1561]],
                    &self.row.columns[self.index_mapping[1562]],
                    &self.row.columns[self.index_mapping[1563]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1564]],
                    &self.row.columns[self.index_mapping[1565]],
                    &self.row.columns[self.index_mapping[1566]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1567]],
                    &self.row.columns[self.index_mapping[1568]],
                    &self.row.columns[self.index_mapping[1569]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1570]],
                    &self.row.columns[self.index_mapping[1571]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1572]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1573]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1574]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1575]],
                    &self.row.columns[self.index_mapping[1576]],
                    &self.row.columns[self.index_mapping[1577]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1578]],
                    &self.row.columns[self.index_mapping[1579]],
                    &self.row.columns[self.index_mapping[1580]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1581]],
                    &self.row.columns[self.index_mapping[1582]],
                    &self.row.columns[self.index_mapping[1583]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1584]],
                    &self.row.columns[self.index_mapping[1585]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1586]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1587]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1588]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1589]],
                    &self.row.columns[self.index_mapping[1590]],
                    &self.row.columns[self.index_mapping[1591]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1592]],
                    &self.row.columns[self.index_mapping[1593]],
                    &self.row.columns[self.index_mapping[1594]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1595]],
                    &self.row.columns[self.index_mapping[1596]],
                    &self.row.columns[self.index_mapping[1597]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1598]],
                    &self.row.columns[self.index_mapping[1599]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1600]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1601]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1602]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1603]],
                    &self.row.columns[self.index_mapping[1604]],
                    &self.row.columns[self.index_mapping[1605]],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.row.columns[self.index_mapping[1606]],
                    &self.row.columns[self.index_mapping[1607]],
                    &self.row.columns[self.index_mapping[1608]],
                ],
                AcceptedQuest: [
                    &self.row.columns[self.index_mapping[1609]],
                    &self.row.columns[self.index_mapping[1610]],
                    &self.row.columns[self.index_mapping[1611]],
                ],
                LayerSet: [
                    &self.row.columns[self.index_mapping[1612]],
                    &self.row.columns[self.index_mapping[1613]],
                ],
                Sequence: &self.row.columns[self.index_mapping[1614]],
                CompletedQuestOperator: &self.row.columns[self.index_mapping[1615]],
                AcceptedQuestOperator: &self.row.columns[self.index_mapping[1616]],
                AcceptedQuestSequence: [
                    &self.row.columns[self.index_mapping[1617]],
                    &self.row.columns[self.index_mapping[1618]],
                    &self.row.columns[self.index_mapping[1619]],
                ],
            },
        ]
    }
    pub fn StoryListener(&'a self) -> [StoryListenerElement<'a>; 80] {
        [
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1620]],
                SequenceBegin: &self.row.columns[self.index_mapping[1621]],
                SequenceEnd: &self.row.columns[self.index_mapping[1622]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1623]],
                SequenceBegin: &self.row.columns[self.index_mapping[1624]],
                SequenceEnd: &self.row.columns[self.index_mapping[1625]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1626]],
                SequenceBegin: &self.row.columns[self.index_mapping[1627]],
                SequenceEnd: &self.row.columns[self.index_mapping[1628]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1629]],
                SequenceBegin: &self.row.columns[self.index_mapping[1630]],
                SequenceEnd: &self.row.columns[self.index_mapping[1631]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1632]],
                SequenceBegin: &self.row.columns[self.index_mapping[1633]],
                SequenceEnd: &self.row.columns[self.index_mapping[1634]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1635]],
                SequenceBegin: &self.row.columns[self.index_mapping[1636]],
                SequenceEnd: &self.row.columns[self.index_mapping[1637]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1638]],
                SequenceBegin: &self.row.columns[self.index_mapping[1639]],
                SequenceEnd: &self.row.columns[self.index_mapping[1640]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1641]],
                SequenceBegin: &self.row.columns[self.index_mapping[1642]],
                SequenceEnd: &self.row.columns[self.index_mapping[1643]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1644]],
                SequenceBegin: &self.row.columns[self.index_mapping[1645]],
                SequenceEnd: &self.row.columns[self.index_mapping[1646]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1647]],
                SequenceBegin: &self.row.columns[self.index_mapping[1648]],
                SequenceEnd: &self.row.columns[self.index_mapping[1649]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1650]],
                SequenceBegin: &self.row.columns[self.index_mapping[1651]],
                SequenceEnd: &self.row.columns[self.index_mapping[1652]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1653]],
                SequenceBegin: &self.row.columns[self.index_mapping[1654]],
                SequenceEnd: &self.row.columns[self.index_mapping[1655]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1656]],
                SequenceBegin: &self.row.columns[self.index_mapping[1657]],
                SequenceEnd: &self.row.columns[self.index_mapping[1658]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1659]],
                SequenceBegin: &self.row.columns[self.index_mapping[1660]],
                SequenceEnd: &self.row.columns[self.index_mapping[1661]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1662]],
                SequenceBegin: &self.row.columns[self.index_mapping[1663]],
                SequenceEnd: &self.row.columns[self.index_mapping[1664]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1665]],
                SequenceBegin: &self.row.columns[self.index_mapping[1666]],
                SequenceEnd: &self.row.columns[self.index_mapping[1667]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1668]],
                SequenceBegin: &self.row.columns[self.index_mapping[1669]],
                SequenceEnd: &self.row.columns[self.index_mapping[1670]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1671]],
                SequenceBegin: &self.row.columns[self.index_mapping[1672]],
                SequenceEnd: &self.row.columns[self.index_mapping[1673]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1674]],
                SequenceBegin: &self.row.columns[self.index_mapping[1675]],
                SequenceEnd: &self.row.columns[self.index_mapping[1676]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1677]],
                SequenceBegin: &self.row.columns[self.index_mapping[1678]],
                SequenceEnd: &self.row.columns[self.index_mapping[1679]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1680]],
                SequenceBegin: &self.row.columns[self.index_mapping[1681]],
                SequenceEnd: &self.row.columns[self.index_mapping[1682]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1683]],
                SequenceBegin: &self.row.columns[self.index_mapping[1684]],
                SequenceEnd: &self.row.columns[self.index_mapping[1685]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1686]],
                SequenceBegin: &self.row.columns[self.index_mapping[1687]],
                SequenceEnd: &self.row.columns[self.index_mapping[1688]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1689]],
                SequenceBegin: &self.row.columns[self.index_mapping[1690]],
                SequenceEnd: &self.row.columns[self.index_mapping[1691]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1692]],
                SequenceBegin: &self.row.columns[self.index_mapping[1693]],
                SequenceEnd: &self.row.columns[self.index_mapping[1694]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1695]],
                SequenceBegin: &self.row.columns[self.index_mapping[1696]],
                SequenceEnd: &self.row.columns[self.index_mapping[1697]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1698]],
                SequenceBegin: &self.row.columns[self.index_mapping[1699]],
                SequenceEnd: &self.row.columns[self.index_mapping[1700]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1701]],
                SequenceBegin: &self.row.columns[self.index_mapping[1702]],
                SequenceEnd: &self.row.columns[self.index_mapping[1703]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1704]],
                SequenceBegin: &self.row.columns[self.index_mapping[1705]],
                SequenceEnd: &self.row.columns[self.index_mapping[1706]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1707]],
                SequenceBegin: &self.row.columns[self.index_mapping[1708]],
                SequenceEnd: &self.row.columns[self.index_mapping[1709]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1710]],
                SequenceBegin: &self.row.columns[self.index_mapping[1711]],
                SequenceEnd: &self.row.columns[self.index_mapping[1712]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1713]],
                SequenceBegin: &self.row.columns[self.index_mapping[1714]],
                SequenceEnd: &self.row.columns[self.index_mapping[1715]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1716]],
                SequenceBegin: &self.row.columns[self.index_mapping[1717]],
                SequenceEnd: &self.row.columns[self.index_mapping[1718]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1719]],
                SequenceBegin: &self.row.columns[self.index_mapping[1720]],
                SequenceEnd: &self.row.columns[self.index_mapping[1721]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1722]],
                SequenceBegin: &self.row.columns[self.index_mapping[1723]],
                SequenceEnd: &self.row.columns[self.index_mapping[1724]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1725]],
                SequenceBegin: &self.row.columns[self.index_mapping[1726]],
                SequenceEnd: &self.row.columns[self.index_mapping[1727]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1728]],
                SequenceBegin: &self.row.columns[self.index_mapping[1729]],
                SequenceEnd: &self.row.columns[self.index_mapping[1730]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1731]],
                SequenceBegin: &self.row.columns[self.index_mapping[1732]],
                SequenceEnd: &self.row.columns[self.index_mapping[1733]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1734]],
                SequenceBegin: &self.row.columns[self.index_mapping[1735]],
                SequenceEnd: &self.row.columns[self.index_mapping[1736]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1737]],
                SequenceBegin: &self.row.columns[self.index_mapping[1738]],
                SequenceEnd: &self.row.columns[self.index_mapping[1739]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1740]],
                SequenceBegin: &self.row.columns[self.index_mapping[1741]],
                SequenceEnd: &self.row.columns[self.index_mapping[1742]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1743]],
                SequenceBegin: &self.row.columns[self.index_mapping[1744]],
                SequenceEnd: &self.row.columns[self.index_mapping[1745]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1746]],
                SequenceBegin: &self.row.columns[self.index_mapping[1747]],
                SequenceEnd: &self.row.columns[self.index_mapping[1748]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1749]],
                SequenceBegin: &self.row.columns[self.index_mapping[1750]],
                SequenceEnd: &self.row.columns[self.index_mapping[1751]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1752]],
                SequenceBegin: &self.row.columns[self.index_mapping[1753]],
                SequenceEnd: &self.row.columns[self.index_mapping[1754]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1755]],
                SequenceBegin: &self.row.columns[self.index_mapping[1756]],
                SequenceEnd: &self.row.columns[self.index_mapping[1757]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1758]],
                SequenceBegin: &self.row.columns[self.index_mapping[1759]],
                SequenceEnd: &self.row.columns[self.index_mapping[1760]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1761]],
                SequenceBegin: &self.row.columns[self.index_mapping[1762]],
                SequenceEnd: &self.row.columns[self.index_mapping[1763]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1764]],
                SequenceBegin: &self.row.columns[self.index_mapping[1765]],
                SequenceEnd: &self.row.columns[self.index_mapping[1766]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1767]],
                SequenceBegin: &self.row.columns[self.index_mapping[1768]],
                SequenceEnd: &self.row.columns[self.index_mapping[1769]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1770]],
                SequenceBegin: &self.row.columns[self.index_mapping[1771]],
                SequenceEnd: &self.row.columns[self.index_mapping[1772]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1773]],
                SequenceBegin: &self.row.columns[self.index_mapping[1774]],
                SequenceEnd: &self.row.columns[self.index_mapping[1775]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1776]],
                SequenceBegin: &self.row.columns[self.index_mapping[1777]],
                SequenceEnd: &self.row.columns[self.index_mapping[1778]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1779]],
                SequenceBegin: &self.row.columns[self.index_mapping[1780]],
                SequenceEnd: &self.row.columns[self.index_mapping[1781]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1782]],
                SequenceBegin: &self.row.columns[self.index_mapping[1783]],
                SequenceEnd: &self.row.columns[self.index_mapping[1784]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1785]],
                SequenceBegin: &self.row.columns[self.index_mapping[1786]],
                SequenceEnd: &self.row.columns[self.index_mapping[1787]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1788]],
                SequenceBegin: &self.row.columns[self.index_mapping[1789]],
                SequenceEnd: &self.row.columns[self.index_mapping[1790]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1791]],
                SequenceBegin: &self.row.columns[self.index_mapping[1792]],
                SequenceEnd: &self.row.columns[self.index_mapping[1793]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1794]],
                SequenceBegin: &self.row.columns[self.index_mapping[1795]],
                SequenceEnd: &self.row.columns[self.index_mapping[1796]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1797]],
                SequenceBegin: &self.row.columns[self.index_mapping[1798]],
                SequenceEnd: &self.row.columns[self.index_mapping[1799]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1800]],
                SequenceBegin: &self.row.columns[self.index_mapping[1801]],
                SequenceEnd: &self.row.columns[self.index_mapping[1802]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1803]],
                SequenceBegin: &self.row.columns[self.index_mapping[1804]],
                SequenceEnd: &self.row.columns[self.index_mapping[1805]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1806]],
                SequenceBegin: &self.row.columns[self.index_mapping[1807]],
                SequenceEnd: &self.row.columns[self.index_mapping[1808]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1809]],
                SequenceBegin: &self.row.columns[self.index_mapping[1810]],
                SequenceEnd: &self.row.columns[self.index_mapping[1811]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1812]],
                SequenceBegin: &self.row.columns[self.index_mapping[1813]],
                SequenceEnd: &self.row.columns[self.index_mapping[1814]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1815]],
                SequenceBegin: &self.row.columns[self.index_mapping[1816]],
                SequenceEnd: &self.row.columns[self.index_mapping[1817]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1818]],
                SequenceBegin: &self.row.columns[self.index_mapping[1819]],
                SequenceEnd: &self.row.columns[self.index_mapping[1820]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1821]],
                SequenceBegin: &self.row.columns[self.index_mapping[1822]],
                SequenceEnd: &self.row.columns[self.index_mapping[1823]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1824]],
                SequenceBegin: &self.row.columns[self.index_mapping[1825]],
                SequenceEnd: &self.row.columns[self.index_mapping[1826]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1827]],
                SequenceBegin: &self.row.columns[self.index_mapping[1828]],
                SequenceEnd: &self.row.columns[self.index_mapping[1829]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1830]],
                SequenceBegin: &self.row.columns[self.index_mapping[1831]],
                SequenceEnd: &self.row.columns[self.index_mapping[1832]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1833]],
                SequenceBegin: &self.row.columns[self.index_mapping[1834]],
                SequenceEnd: &self.row.columns[self.index_mapping[1835]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1836]],
                SequenceBegin: &self.row.columns[self.index_mapping[1837]],
                SequenceEnd: &self.row.columns[self.index_mapping[1838]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1839]],
                SequenceBegin: &self.row.columns[self.index_mapping[1840]],
                SequenceEnd: &self.row.columns[self.index_mapping[1841]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1842]],
                SequenceBegin: &self.row.columns[self.index_mapping[1843]],
                SequenceEnd: &self.row.columns[self.index_mapping[1844]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1845]],
                SequenceBegin: &self.row.columns[self.index_mapping[1846]],
                SequenceEnd: &self.row.columns[self.index_mapping[1847]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1848]],
                SequenceBegin: &self.row.columns[self.index_mapping[1849]],
                SequenceEnd: &self.row.columns[self.index_mapping[1850]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1851]],
                SequenceBegin: &self.row.columns[self.index_mapping[1852]],
                SequenceEnd: &self.row.columns[self.index_mapping[1853]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1854]],
                SequenceBegin: &self.row.columns[self.index_mapping[1855]],
                SequenceEnd: &self.row.columns[self.index_mapping[1856]],
            },
            StoryListenerElement {
                Listener: &self.row.columns[self.index_mapping[1857]],
                SequenceBegin: &self.row.columns[self.index_mapping[1858]],
                SequenceEnd: &self.row.columns[self.index_mapping[1859]],
            },
        ]
    }
    pub fn Script(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1860]]
    }
    pub fn LayerSetTerritoryType(&'a self) -> [&'a Field; 2] {
        [
            &self.row.columns[self.index_mapping[1861]],
            &self.row.columns[self.index_mapping[1862]],
        ]
    }
}
