//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct QuestParamsElement<'a> {
    pub ScriptInstruction: &'a Field,
    pub ScriptArg: &'a Field,
}
pub struct QuestListenerParamsElement<'a> {
    pub Listener: &'a Field,
    pub ConditionValue: &'a Field,
    pub Behavior: &'a Field,
    pub ActorSpawnSeq: &'a Field,
    pub ActorDespawnSeq: &'a Field,
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
    pub QuestUInt8A: &'a Field,
    pub ConditionType: &'a Field,
    pub ConditionOperator: &'a Field,
    pub VisibleBool: &'a Field,
    pub ConditionBool: &'a Field,
    pub ItemBool: &'a Field,
    pub AnnounceBool: &'a Field,
    pub BehaviorBool: &'a Field,
    pub AcceptBool: &'a Field,
    pub QualifiedBool: &'a Field,
    pub CanTargetBool: &'a Field,
}
pub struct TodoParamsElement<'a> {
    pub ToDoLocation: [&'a Field; 8],
    pub ToDoCompleteSeq: &'a Field,
    pub ToDoQty: &'a Field,
    pub CountableNum: &'a Field,
}
#[derive(Debug, Clone)]
pub struct QuestSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl QuestSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Quest")?;
        let sheet = resolver.read_excel_sheet(&exh, "Quest", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<QuestRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for QuestSheet {
    type Row = QuestRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a QuestSheet {
    type Item = (u32, Vec<(u16, QuestRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, QuestSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> QuestRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn QuestParams(&'a self) -> [QuestParamsElement<'a>; 50] {
        [
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[1]],
                ScriptArg: &self.row.columns[self.index_mapping[2]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[3]],
                ScriptArg: &self.row.columns[self.index_mapping[4]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[5]],
                ScriptArg: &self.row.columns[self.index_mapping[6]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[7]],
                ScriptArg: &self.row.columns[self.index_mapping[8]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[9]],
                ScriptArg: &self.row.columns[self.index_mapping[10]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[11]],
                ScriptArg: &self.row.columns[self.index_mapping[12]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[13]],
                ScriptArg: &self.row.columns[self.index_mapping[14]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[15]],
                ScriptArg: &self.row.columns[self.index_mapping[16]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[17]],
                ScriptArg: &self.row.columns[self.index_mapping[18]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[19]],
                ScriptArg: &self.row.columns[self.index_mapping[20]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[21]],
                ScriptArg: &self.row.columns[self.index_mapping[22]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[23]],
                ScriptArg: &self.row.columns[self.index_mapping[24]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[25]],
                ScriptArg: &self.row.columns[self.index_mapping[26]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[27]],
                ScriptArg: &self.row.columns[self.index_mapping[28]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[29]],
                ScriptArg: &self.row.columns[self.index_mapping[30]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[31]],
                ScriptArg: &self.row.columns[self.index_mapping[32]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[33]],
                ScriptArg: &self.row.columns[self.index_mapping[34]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[35]],
                ScriptArg: &self.row.columns[self.index_mapping[36]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[37]],
                ScriptArg: &self.row.columns[self.index_mapping[38]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[39]],
                ScriptArg: &self.row.columns[self.index_mapping[40]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[41]],
                ScriptArg: &self.row.columns[self.index_mapping[42]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[43]],
                ScriptArg: &self.row.columns[self.index_mapping[44]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[45]],
                ScriptArg: &self.row.columns[self.index_mapping[46]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[47]],
                ScriptArg: &self.row.columns[self.index_mapping[48]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[49]],
                ScriptArg: &self.row.columns[self.index_mapping[50]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[51]],
                ScriptArg: &self.row.columns[self.index_mapping[52]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[53]],
                ScriptArg: &self.row.columns[self.index_mapping[54]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[55]],
                ScriptArg: &self.row.columns[self.index_mapping[56]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[57]],
                ScriptArg: &self.row.columns[self.index_mapping[58]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[59]],
                ScriptArg: &self.row.columns[self.index_mapping[60]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[61]],
                ScriptArg: &self.row.columns[self.index_mapping[62]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[63]],
                ScriptArg: &self.row.columns[self.index_mapping[64]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[65]],
                ScriptArg: &self.row.columns[self.index_mapping[66]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[67]],
                ScriptArg: &self.row.columns[self.index_mapping[68]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[69]],
                ScriptArg: &self.row.columns[self.index_mapping[70]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[71]],
                ScriptArg: &self.row.columns[self.index_mapping[72]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[73]],
                ScriptArg: &self.row.columns[self.index_mapping[74]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[75]],
                ScriptArg: &self.row.columns[self.index_mapping[76]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[77]],
                ScriptArg: &self.row.columns[self.index_mapping[78]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[79]],
                ScriptArg: &self.row.columns[self.index_mapping[80]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[81]],
                ScriptArg: &self.row.columns[self.index_mapping[82]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[83]],
                ScriptArg: &self.row.columns[self.index_mapping[84]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[85]],
                ScriptArg: &self.row.columns[self.index_mapping[86]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[87]],
                ScriptArg: &self.row.columns[self.index_mapping[88]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[89]],
                ScriptArg: &self.row.columns[self.index_mapping[90]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[91]],
                ScriptArg: &self.row.columns[self.index_mapping[92]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[93]],
                ScriptArg: &self.row.columns[self.index_mapping[94]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[95]],
                ScriptArg: &self.row.columns[self.index_mapping[96]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[97]],
                ScriptArg: &self.row.columns[self.index_mapping[98]],
            },
            QuestParamsElement {
                ScriptInstruction: &self.row.columns[self.index_mapping[99]],
                ScriptArg: &self.row.columns[self.index_mapping[100]],
            },
        ]
    }
    pub fn QuestListenerParams(&'a self) -> [QuestListenerParamsElement<'a>; 64] {
        [
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[101]],
                ConditionValue: &self.row.columns[self.index_mapping[102]],
                Behavior: &self.row.columns[self.index_mapping[103]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[104]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[105]],
                Unknown0: &self.row.columns[self.index_mapping[106]],
                Unknown1: &self.row.columns[self.index_mapping[107]],
                QuestUInt8A: &self.row.columns[self.index_mapping[108]],
                ConditionType: &self.row.columns[self.index_mapping[109]],
                ConditionOperator: &self.row.columns[self.index_mapping[110]],
                VisibleBool: &self.row.columns[self.index_mapping[111]],
                ConditionBool: &self.row.columns[self.index_mapping[112]],
                ItemBool: &self.row.columns[self.index_mapping[113]],
                AnnounceBool: &self.row.columns[self.index_mapping[114]],
                BehaviorBool: &self.row.columns[self.index_mapping[115]],
                AcceptBool: &self.row.columns[self.index_mapping[116]],
                QualifiedBool: &self.row.columns[self.index_mapping[117]],
                CanTargetBool: &self.row.columns[self.index_mapping[118]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[119]],
                ConditionValue: &self.row.columns[self.index_mapping[120]],
                Behavior: &self.row.columns[self.index_mapping[121]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[122]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[123]],
                Unknown0: &self.row.columns[self.index_mapping[124]],
                Unknown1: &self.row.columns[self.index_mapping[125]],
                QuestUInt8A: &self.row.columns[self.index_mapping[126]],
                ConditionType: &self.row.columns[self.index_mapping[127]],
                ConditionOperator: &self.row.columns[self.index_mapping[128]],
                VisibleBool: &self.row.columns[self.index_mapping[129]],
                ConditionBool: &self.row.columns[self.index_mapping[130]],
                ItemBool: &self.row.columns[self.index_mapping[131]],
                AnnounceBool: &self.row.columns[self.index_mapping[132]],
                BehaviorBool: &self.row.columns[self.index_mapping[133]],
                AcceptBool: &self.row.columns[self.index_mapping[134]],
                QualifiedBool: &self.row.columns[self.index_mapping[135]],
                CanTargetBool: &self.row.columns[self.index_mapping[136]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[137]],
                ConditionValue: &self.row.columns[self.index_mapping[138]],
                Behavior: &self.row.columns[self.index_mapping[139]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[140]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[141]],
                Unknown0: &self.row.columns[self.index_mapping[142]],
                Unknown1: &self.row.columns[self.index_mapping[143]],
                QuestUInt8A: &self.row.columns[self.index_mapping[144]],
                ConditionType: &self.row.columns[self.index_mapping[145]],
                ConditionOperator: &self.row.columns[self.index_mapping[146]],
                VisibleBool: &self.row.columns[self.index_mapping[147]],
                ConditionBool: &self.row.columns[self.index_mapping[148]],
                ItemBool: &self.row.columns[self.index_mapping[149]],
                AnnounceBool: &self.row.columns[self.index_mapping[150]],
                BehaviorBool: &self.row.columns[self.index_mapping[151]],
                AcceptBool: &self.row.columns[self.index_mapping[152]],
                QualifiedBool: &self.row.columns[self.index_mapping[153]],
                CanTargetBool: &self.row.columns[self.index_mapping[154]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[155]],
                ConditionValue: &self.row.columns[self.index_mapping[156]],
                Behavior: &self.row.columns[self.index_mapping[157]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[158]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[159]],
                Unknown0: &self.row.columns[self.index_mapping[160]],
                Unknown1: &self.row.columns[self.index_mapping[161]],
                QuestUInt8A: &self.row.columns[self.index_mapping[162]],
                ConditionType: &self.row.columns[self.index_mapping[163]],
                ConditionOperator: &self.row.columns[self.index_mapping[164]],
                VisibleBool: &self.row.columns[self.index_mapping[165]],
                ConditionBool: &self.row.columns[self.index_mapping[166]],
                ItemBool: &self.row.columns[self.index_mapping[167]],
                AnnounceBool: &self.row.columns[self.index_mapping[168]],
                BehaviorBool: &self.row.columns[self.index_mapping[169]],
                AcceptBool: &self.row.columns[self.index_mapping[170]],
                QualifiedBool: &self.row.columns[self.index_mapping[171]],
                CanTargetBool: &self.row.columns[self.index_mapping[172]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[173]],
                ConditionValue: &self.row.columns[self.index_mapping[174]],
                Behavior: &self.row.columns[self.index_mapping[175]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[176]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[177]],
                Unknown0: &self.row.columns[self.index_mapping[178]],
                Unknown1: &self.row.columns[self.index_mapping[179]],
                QuestUInt8A: &self.row.columns[self.index_mapping[180]],
                ConditionType: &self.row.columns[self.index_mapping[181]],
                ConditionOperator: &self.row.columns[self.index_mapping[182]],
                VisibleBool: &self.row.columns[self.index_mapping[183]],
                ConditionBool: &self.row.columns[self.index_mapping[184]],
                ItemBool: &self.row.columns[self.index_mapping[185]],
                AnnounceBool: &self.row.columns[self.index_mapping[186]],
                BehaviorBool: &self.row.columns[self.index_mapping[187]],
                AcceptBool: &self.row.columns[self.index_mapping[188]],
                QualifiedBool: &self.row.columns[self.index_mapping[189]],
                CanTargetBool: &self.row.columns[self.index_mapping[190]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[191]],
                ConditionValue: &self.row.columns[self.index_mapping[192]],
                Behavior: &self.row.columns[self.index_mapping[193]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[194]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[195]],
                Unknown0: &self.row.columns[self.index_mapping[196]],
                Unknown1: &self.row.columns[self.index_mapping[197]],
                QuestUInt8A: &self.row.columns[self.index_mapping[198]],
                ConditionType: &self.row.columns[self.index_mapping[199]],
                ConditionOperator: &self.row.columns[self.index_mapping[200]],
                VisibleBool: &self.row.columns[self.index_mapping[201]],
                ConditionBool: &self.row.columns[self.index_mapping[202]],
                ItemBool: &self.row.columns[self.index_mapping[203]],
                AnnounceBool: &self.row.columns[self.index_mapping[204]],
                BehaviorBool: &self.row.columns[self.index_mapping[205]],
                AcceptBool: &self.row.columns[self.index_mapping[206]],
                QualifiedBool: &self.row.columns[self.index_mapping[207]],
                CanTargetBool: &self.row.columns[self.index_mapping[208]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[209]],
                ConditionValue: &self.row.columns[self.index_mapping[210]],
                Behavior: &self.row.columns[self.index_mapping[211]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[212]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[213]],
                Unknown0: &self.row.columns[self.index_mapping[214]],
                Unknown1: &self.row.columns[self.index_mapping[215]],
                QuestUInt8A: &self.row.columns[self.index_mapping[216]],
                ConditionType: &self.row.columns[self.index_mapping[217]],
                ConditionOperator: &self.row.columns[self.index_mapping[218]],
                VisibleBool: &self.row.columns[self.index_mapping[219]],
                ConditionBool: &self.row.columns[self.index_mapping[220]],
                ItemBool: &self.row.columns[self.index_mapping[221]],
                AnnounceBool: &self.row.columns[self.index_mapping[222]],
                BehaviorBool: &self.row.columns[self.index_mapping[223]],
                AcceptBool: &self.row.columns[self.index_mapping[224]],
                QualifiedBool: &self.row.columns[self.index_mapping[225]],
                CanTargetBool: &self.row.columns[self.index_mapping[226]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[227]],
                ConditionValue: &self.row.columns[self.index_mapping[228]],
                Behavior: &self.row.columns[self.index_mapping[229]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[230]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[231]],
                Unknown0: &self.row.columns[self.index_mapping[232]],
                Unknown1: &self.row.columns[self.index_mapping[233]],
                QuestUInt8A: &self.row.columns[self.index_mapping[234]],
                ConditionType: &self.row.columns[self.index_mapping[235]],
                ConditionOperator: &self.row.columns[self.index_mapping[236]],
                VisibleBool: &self.row.columns[self.index_mapping[237]],
                ConditionBool: &self.row.columns[self.index_mapping[238]],
                ItemBool: &self.row.columns[self.index_mapping[239]],
                AnnounceBool: &self.row.columns[self.index_mapping[240]],
                BehaviorBool: &self.row.columns[self.index_mapping[241]],
                AcceptBool: &self.row.columns[self.index_mapping[242]],
                QualifiedBool: &self.row.columns[self.index_mapping[243]],
                CanTargetBool: &self.row.columns[self.index_mapping[244]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[245]],
                ConditionValue: &self.row.columns[self.index_mapping[246]],
                Behavior: &self.row.columns[self.index_mapping[247]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[248]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[249]],
                Unknown0: &self.row.columns[self.index_mapping[250]],
                Unknown1: &self.row.columns[self.index_mapping[251]],
                QuestUInt8A: &self.row.columns[self.index_mapping[252]],
                ConditionType: &self.row.columns[self.index_mapping[253]],
                ConditionOperator: &self.row.columns[self.index_mapping[254]],
                VisibleBool: &self.row.columns[self.index_mapping[255]],
                ConditionBool: &self.row.columns[self.index_mapping[256]],
                ItemBool: &self.row.columns[self.index_mapping[257]],
                AnnounceBool: &self.row.columns[self.index_mapping[258]],
                BehaviorBool: &self.row.columns[self.index_mapping[259]],
                AcceptBool: &self.row.columns[self.index_mapping[260]],
                QualifiedBool: &self.row.columns[self.index_mapping[261]],
                CanTargetBool: &self.row.columns[self.index_mapping[262]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[263]],
                ConditionValue: &self.row.columns[self.index_mapping[264]],
                Behavior: &self.row.columns[self.index_mapping[265]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[266]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[267]],
                Unknown0: &self.row.columns[self.index_mapping[268]],
                Unknown1: &self.row.columns[self.index_mapping[269]],
                QuestUInt8A: &self.row.columns[self.index_mapping[270]],
                ConditionType: &self.row.columns[self.index_mapping[271]],
                ConditionOperator: &self.row.columns[self.index_mapping[272]],
                VisibleBool: &self.row.columns[self.index_mapping[273]],
                ConditionBool: &self.row.columns[self.index_mapping[274]],
                ItemBool: &self.row.columns[self.index_mapping[275]],
                AnnounceBool: &self.row.columns[self.index_mapping[276]],
                BehaviorBool: &self.row.columns[self.index_mapping[277]],
                AcceptBool: &self.row.columns[self.index_mapping[278]],
                QualifiedBool: &self.row.columns[self.index_mapping[279]],
                CanTargetBool: &self.row.columns[self.index_mapping[280]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[281]],
                ConditionValue: &self.row.columns[self.index_mapping[282]],
                Behavior: &self.row.columns[self.index_mapping[283]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[284]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[285]],
                Unknown0: &self.row.columns[self.index_mapping[286]],
                Unknown1: &self.row.columns[self.index_mapping[287]],
                QuestUInt8A: &self.row.columns[self.index_mapping[288]],
                ConditionType: &self.row.columns[self.index_mapping[289]],
                ConditionOperator: &self.row.columns[self.index_mapping[290]],
                VisibleBool: &self.row.columns[self.index_mapping[291]],
                ConditionBool: &self.row.columns[self.index_mapping[292]],
                ItemBool: &self.row.columns[self.index_mapping[293]],
                AnnounceBool: &self.row.columns[self.index_mapping[294]],
                BehaviorBool: &self.row.columns[self.index_mapping[295]],
                AcceptBool: &self.row.columns[self.index_mapping[296]],
                QualifiedBool: &self.row.columns[self.index_mapping[297]],
                CanTargetBool: &self.row.columns[self.index_mapping[298]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[299]],
                ConditionValue: &self.row.columns[self.index_mapping[300]],
                Behavior: &self.row.columns[self.index_mapping[301]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[302]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[303]],
                Unknown0: &self.row.columns[self.index_mapping[304]],
                Unknown1: &self.row.columns[self.index_mapping[305]],
                QuestUInt8A: &self.row.columns[self.index_mapping[306]],
                ConditionType: &self.row.columns[self.index_mapping[307]],
                ConditionOperator: &self.row.columns[self.index_mapping[308]],
                VisibleBool: &self.row.columns[self.index_mapping[309]],
                ConditionBool: &self.row.columns[self.index_mapping[310]],
                ItemBool: &self.row.columns[self.index_mapping[311]],
                AnnounceBool: &self.row.columns[self.index_mapping[312]],
                BehaviorBool: &self.row.columns[self.index_mapping[313]],
                AcceptBool: &self.row.columns[self.index_mapping[314]],
                QualifiedBool: &self.row.columns[self.index_mapping[315]],
                CanTargetBool: &self.row.columns[self.index_mapping[316]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[317]],
                ConditionValue: &self.row.columns[self.index_mapping[318]],
                Behavior: &self.row.columns[self.index_mapping[319]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[320]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[321]],
                Unknown0: &self.row.columns[self.index_mapping[322]],
                Unknown1: &self.row.columns[self.index_mapping[323]],
                QuestUInt8A: &self.row.columns[self.index_mapping[324]],
                ConditionType: &self.row.columns[self.index_mapping[325]],
                ConditionOperator: &self.row.columns[self.index_mapping[326]],
                VisibleBool: &self.row.columns[self.index_mapping[327]],
                ConditionBool: &self.row.columns[self.index_mapping[328]],
                ItemBool: &self.row.columns[self.index_mapping[329]],
                AnnounceBool: &self.row.columns[self.index_mapping[330]],
                BehaviorBool: &self.row.columns[self.index_mapping[331]],
                AcceptBool: &self.row.columns[self.index_mapping[332]],
                QualifiedBool: &self.row.columns[self.index_mapping[333]],
                CanTargetBool: &self.row.columns[self.index_mapping[334]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[335]],
                ConditionValue: &self.row.columns[self.index_mapping[336]],
                Behavior: &self.row.columns[self.index_mapping[337]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[338]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[339]],
                Unknown0: &self.row.columns[self.index_mapping[340]],
                Unknown1: &self.row.columns[self.index_mapping[341]],
                QuestUInt8A: &self.row.columns[self.index_mapping[342]],
                ConditionType: &self.row.columns[self.index_mapping[343]],
                ConditionOperator: &self.row.columns[self.index_mapping[344]],
                VisibleBool: &self.row.columns[self.index_mapping[345]],
                ConditionBool: &self.row.columns[self.index_mapping[346]],
                ItemBool: &self.row.columns[self.index_mapping[347]],
                AnnounceBool: &self.row.columns[self.index_mapping[348]],
                BehaviorBool: &self.row.columns[self.index_mapping[349]],
                AcceptBool: &self.row.columns[self.index_mapping[350]],
                QualifiedBool: &self.row.columns[self.index_mapping[351]],
                CanTargetBool: &self.row.columns[self.index_mapping[352]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[353]],
                ConditionValue: &self.row.columns[self.index_mapping[354]],
                Behavior: &self.row.columns[self.index_mapping[355]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[356]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[357]],
                Unknown0: &self.row.columns[self.index_mapping[358]],
                Unknown1: &self.row.columns[self.index_mapping[359]],
                QuestUInt8A: &self.row.columns[self.index_mapping[360]],
                ConditionType: &self.row.columns[self.index_mapping[361]],
                ConditionOperator: &self.row.columns[self.index_mapping[362]],
                VisibleBool: &self.row.columns[self.index_mapping[363]],
                ConditionBool: &self.row.columns[self.index_mapping[364]],
                ItemBool: &self.row.columns[self.index_mapping[365]],
                AnnounceBool: &self.row.columns[self.index_mapping[366]],
                BehaviorBool: &self.row.columns[self.index_mapping[367]],
                AcceptBool: &self.row.columns[self.index_mapping[368]],
                QualifiedBool: &self.row.columns[self.index_mapping[369]],
                CanTargetBool: &self.row.columns[self.index_mapping[370]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[371]],
                ConditionValue: &self.row.columns[self.index_mapping[372]],
                Behavior: &self.row.columns[self.index_mapping[373]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[374]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[375]],
                Unknown0: &self.row.columns[self.index_mapping[376]],
                Unknown1: &self.row.columns[self.index_mapping[377]],
                QuestUInt8A: &self.row.columns[self.index_mapping[378]],
                ConditionType: &self.row.columns[self.index_mapping[379]],
                ConditionOperator: &self.row.columns[self.index_mapping[380]],
                VisibleBool: &self.row.columns[self.index_mapping[381]],
                ConditionBool: &self.row.columns[self.index_mapping[382]],
                ItemBool: &self.row.columns[self.index_mapping[383]],
                AnnounceBool: &self.row.columns[self.index_mapping[384]],
                BehaviorBool: &self.row.columns[self.index_mapping[385]],
                AcceptBool: &self.row.columns[self.index_mapping[386]],
                QualifiedBool: &self.row.columns[self.index_mapping[387]],
                CanTargetBool: &self.row.columns[self.index_mapping[388]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[389]],
                ConditionValue: &self.row.columns[self.index_mapping[390]],
                Behavior: &self.row.columns[self.index_mapping[391]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[392]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[393]],
                Unknown0: &self.row.columns[self.index_mapping[394]],
                Unknown1: &self.row.columns[self.index_mapping[395]],
                QuestUInt8A: &self.row.columns[self.index_mapping[396]],
                ConditionType: &self.row.columns[self.index_mapping[397]],
                ConditionOperator: &self.row.columns[self.index_mapping[398]],
                VisibleBool: &self.row.columns[self.index_mapping[399]],
                ConditionBool: &self.row.columns[self.index_mapping[400]],
                ItemBool: &self.row.columns[self.index_mapping[401]],
                AnnounceBool: &self.row.columns[self.index_mapping[402]],
                BehaviorBool: &self.row.columns[self.index_mapping[403]],
                AcceptBool: &self.row.columns[self.index_mapping[404]],
                QualifiedBool: &self.row.columns[self.index_mapping[405]],
                CanTargetBool: &self.row.columns[self.index_mapping[406]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[407]],
                ConditionValue: &self.row.columns[self.index_mapping[408]],
                Behavior: &self.row.columns[self.index_mapping[409]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[410]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[411]],
                Unknown0: &self.row.columns[self.index_mapping[412]],
                Unknown1: &self.row.columns[self.index_mapping[413]],
                QuestUInt8A: &self.row.columns[self.index_mapping[414]],
                ConditionType: &self.row.columns[self.index_mapping[415]],
                ConditionOperator: &self.row.columns[self.index_mapping[416]],
                VisibleBool: &self.row.columns[self.index_mapping[417]],
                ConditionBool: &self.row.columns[self.index_mapping[418]],
                ItemBool: &self.row.columns[self.index_mapping[419]],
                AnnounceBool: &self.row.columns[self.index_mapping[420]],
                BehaviorBool: &self.row.columns[self.index_mapping[421]],
                AcceptBool: &self.row.columns[self.index_mapping[422]],
                QualifiedBool: &self.row.columns[self.index_mapping[423]],
                CanTargetBool: &self.row.columns[self.index_mapping[424]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[425]],
                ConditionValue: &self.row.columns[self.index_mapping[426]],
                Behavior: &self.row.columns[self.index_mapping[427]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[428]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[429]],
                Unknown0: &self.row.columns[self.index_mapping[430]],
                Unknown1: &self.row.columns[self.index_mapping[431]],
                QuestUInt8A: &self.row.columns[self.index_mapping[432]],
                ConditionType: &self.row.columns[self.index_mapping[433]],
                ConditionOperator: &self.row.columns[self.index_mapping[434]],
                VisibleBool: &self.row.columns[self.index_mapping[435]],
                ConditionBool: &self.row.columns[self.index_mapping[436]],
                ItemBool: &self.row.columns[self.index_mapping[437]],
                AnnounceBool: &self.row.columns[self.index_mapping[438]],
                BehaviorBool: &self.row.columns[self.index_mapping[439]],
                AcceptBool: &self.row.columns[self.index_mapping[440]],
                QualifiedBool: &self.row.columns[self.index_mapping[441]],
                CanTargetBool: &self.row.columns[self.index_mapping[442]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[443]],
                ConditionValue: &self.row.columns[self.index_mapping[444]],
                Behavior: &self.row.columns[self.index_mapping[445]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[446]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[447]],
                Unknown0: &self.row.columns[self.index_mapping[448]],
                Unknown1: &self.row.columns[self.index_mapping[449]],
                QuestUInt8A: &self.row.columns[self.index_mapping[450]],
                ConditionType: &self.row.columns[self.index_mapping[451]],
                ConditionOperator: &self.row.columns[self.index_mapping[452]],
                VisibleBool: &self.row.columns[self.index_mapping[453]],
                ConditionBool: &self.row.columns[self.index_mapping[454]],
                ItemBool: &self.row.columns[self.index_mapping[455]],
                AnnounceBool: &self.row.columns[self.index_mapping[456]],
                BehaviorBool: &self.row.columns[self.index_mapping[457]],
                AcceptBool: &self.row.columns[self.index_mapping[458]],
                QualifiedBool: &self.row.columns[self.index_mapping[459]],
                CanTargetBool: &self.row.columns[self.index_mapping[460]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[461]],
                ConditionValue: &self.row.columns[self.index_mapping[462]],
                Behavior: &self.row.columns[self.index_mapping[463]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[464]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[465]],
                Unknown0: &self.row.columns[self.index_mapping[466]],
                Unknown1: &self.row.columns[self.index_mapping[467]],
                QuestUInt8A: &self.row.columns[self.index_mapping[468]],
                ConditionType: &self.row.columns[self.index_mapping[469]],
                ConditionOperator: &self.row.columns[self.index_mapping[470]],
                VisibleBool: &self.row.columns[self.index_mapping[471]],
                ConditionBool: &self.row.columns[self.index_mapping[472]],
                ItemBool: &self.row.columns[self.index_mapping[473]],
                AnnounceBool: &self.row.columns[self.index_mapping[474]],
                BehaviorBool: &self.row.columns[self.index_mapping[475]],
                AcceptBool: &self.row.columns[self.index_mapping[476]],
                QualifiedBool: &self.row.columns[self.index_mapping[477]],
                CanTargetBool: &self.row.columns[self.index_mapping[478]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[479]],
                ConditionValue: &self.row.columns[self.index_mapping[480]],
                Behavior: &self.row.columns[self.index_mapping[481]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[482]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[483]],
                Unknown0: &self.row.columns[self.index_mapping[484]],
                Unknown1: &self.row.columns[self.index_mapping[485]],
                QuestUInt8A: &self.row.columns[self.index_mapping[486]],
                ConditionType: &self.row.columns[self.index_mapping[487]],
                ConditionOperator: &self.row.columns[self.index_mapping[488]],
                VisibleBool: &self.row.columns[self.index_mapping[489]],
                ConditionBool: &self.row.columns[self.index_mapping[490]],
                ItemBool: &self.row.columns[self.index_mapping[491]],
                AnnounceBool: &self.row.columns[self.index_mapping[492]],
                BehaviorBool: &self.row.columns[self.index_mapping[493]],
                AcceptBool: &self.row.columns[self.index_mapping[494]],
                QualifiedBool: &self.row.columns[self.index_mapping[495]],
                CanTargetBool: &self.row.columns[self.index_mapping[496]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[497]],
                ConditionValue: &self.row.columns[self.index_mapping[498]],
                Behavior: &self.row.columns[self.index_mapping[499]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[500]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[501]],
                Unknown0: &self.row.columns[self.index_mapping[502]],
                Unknown1: &self.row.columns[self.index_mapping[503]],
                QuestUInt8A: &self.row.columns[self.index_mapping[504]],
                ConditionType: &self.row.columns[self.index_mapping[505]],
                ConditionOperator: &self.row.columns[self.index_mapping[506]],
                VisibleBool: &self.row.columns[self.index_mapping[507]],
                ConditionBool: &self.row.columns[self.index_mapping[508]],
                ItemBool: &self.row.columns[self.index_mapping[509]],
                AnnounceBool: &self.row.columns[self.index_mapping[510]],
                BehaviorBool: &self.row.columns[self.index_mapping[511]],
                AcceptBool: &self.row.columns[self.index_mapping[512]],
                QualifiedBool: &self.row.columns[self.index_mapping[513]],
                CanTargetBool: &self.row.columns[self.index_mapping[514]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[515]],
                ConditionValue: &self.row.columns[self.index_mapping[516]],
                Behavior: &self.row.columns[self.index_mapping[517]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[518]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[519]],
                Unknown0: &self.row.columns[self.index_mapping[520]],
                Unknown1: &self.row.columns[self.index_mapping[521]],
                QuestUInt8A: &self.row.columns[self.index_mapping[522]],
                ConditionType: &self.row.columns[self.index_mapping[523]],
                ConditionOperator: &self.row.columns[self.index_mapping[524]],
                VisibleBool: &self.row.columns[self.index_mapping[525]],
                ConditionBool: &self.row.columns[self.index_mapping[526]],
                ItemBool: &self.row.columns[self.index_mapping[527]],
                AnnounceBool: &self.row.columns[self.index_mapping[528]],
                BehaviorBool: &self.row.columns[self.index_mapping[529]],
                AcceptBool: &self.row.columns[self.index_mapping[530]],
                QualifiedBool: &self.row.columns[self.index_mapping[531]],
                CanTargetBool: &self.row.columns[self.index_mapping[532]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[533]],
                ConditionValue: &self.row.columns[self.index_mapping[534]],
                Behavior: &self.row.columns[self.index_mapping[535]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[536]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[537]],
                Unknown0: &self.row.columns[self.index_mapping[538]],
                Unknown1: &self.row.columns[self.index_mapping[539]],
                QuestUInt8A: &self.row.columns[self.index_mapping[540]],
                ConditionType: &self.row.columns[self.index_mapping[541]],
                ConditionOperator: &self.row.columns[self.index_mapping[542]],
                VisibleBool: &self.row.columns[self.index_mapping[543]],
                ConditionBool: &self.row.columns[self.index_mapping[544]],
                ItemBool: &self.row.columns[self.index_mapping[545]],
                AnnounceBool: &self.row.columns[self.index_mapping[546]],
                BehaviorBool: &self.row.columns[self.index_mapping[547]],
                AcceptBool: &self.row.columns[self.index_mapping[548]],
                QualifiedBool: &self.row.columns[self.index_mapping[549]],
                CanTargetBool: &self.row.columns[self.index_mapping[550]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[551]],
                ConditionValue: &self.row.columns[self.index_mapping[552]],
                Behavior: &self.row.columns[self.index_mapping[553]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[554]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[555]],
                Unknown0: &self.row.columns[self.index_mapping[556]],
                Unknown1: &self.row.columns[self.index_mapping[557]],
                QuestUInt8A: &self.row.columns[self.index_mapping[558]],
                ConditionType: &self.row.columns[self.index_mapping[559]],
                ConditionOperator: &self.row.columns[self.index_mapping[560]],
                VisibleBool: &self.row.columns[self.index_mapping[561]],
                ConditionBool: &self.row.columns[self.index_mapping[562]],
                ItemBool: &self.row.columns[self.index_mapping[563]],
                AnnounceBool: &self.row.columns[self.index_mapping[564]],
                BehaviorBool: &self.row.columns[self.index_mapping[565]],
                AcceptBool: &self.row.columns[self.index_mapping[566]],
                QualifiedBool: &self.row.columns[self.index_mapping[567]],
                CanTargetBool: &self.row.columns[self.index_mapping[568]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[569]],
                ConditionValue: &self.row.columns[self.index_mapping[570]],
                Behavior: &self.row.columns[self.index_mapping[571]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[572]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[573]],
                Unknown0: &self.row.columns[self.index_mapping[574]],
                Unknown1: &self.row.columns[self.index_mapping[575]],
                QuestUInt8A: &self.row.columns[self.index_mapping[576]],
                ConditionType: &self.row.columns[self.index_mapping[577]],
                ConditionOperator: &self.row.columns[self.index_mapping[578]],
                VisibleBool: &self.row.columns[self.index_mapping[579]],
                ConditionBool: &self.row.columns[self.index_mapping[580]],
                ItemBool: &self.row.columns[self.index_mapping[581]],
                AnnounceBool: &self.row.columns[self.index_mapping[582]],
                BehaviorBool: &self.row.columns[self.index_mapping[583]],
                AcceptBool: &self.row.columns[self.index_mapping[584]],
                QualifiedBool: &self.row.columns[self.index_mapping[585]],
                CanTargetBool: &self.row.columns[self.index_mapping[586]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[587]],
                ConditionValue: &self.row.columns[self.index_mapping[588]],
                Behavior: &self.row.columns[self.index_mapping[589]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[590]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[591]],
                Unknown0: &self.row.columns[self.index_mapping[592]],
                Unknown1: &self.row.columns[self.index_mapping[593]],
                QuestUInt8A: &self.row.columns[self.index_mapping[594]],
                ConditionType: &self.row.columns[self.index_mapping[595]],
                ConditionOperator: &self.row.columns[self.index_mapping[596]],
                VisibleBool: &self.row.columns[self.index_mapping[597]],
                ConditionBool: &self.row.columns[self.index_mapping[598]],
                ItemBool: &self.row.columns[self.index_mapping[599]],
                AnnounceBool: &self.row.columns[self.index_mapping[600]],
                BehaviorBool: &self.row.columns[self.index_mapping[601]],
                AcceptBool: &self.row.columns[self.index_mapping[602]],
                QualifiedBool: &self.row.columns[self.index_mapping[603]],
                CanTargetBool: &self.row.columns[self.index_mapping[604]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[605]],
                ConditionValue: &self.row.columns[self.index_mapping[606]],
                Behavior: &self.row.columns[self.index_mapping[607]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[608]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[609]],
                Unknown0: &self.row.columns[self.index_mapping[610]],
                Unknown1: &self.row.columns[self.index_mapping[611]],
                QuestUInt8A: &self.row.columns[self.index_mapping[612]],
                ConditionType: &self.row.columns[self.index_mapping[613]],
                ConditionOperator: &self.row.columns[self.index_mapping[614]],
                VisibleBool: &self.row.columns[self.index_mapping[615]],
                ConditionBool: &self.row.columns[self.index_mapping[616]],
                ItemBool: &self.row.columns[self.index_mapping[617]],
                AnnounceBool: &self.row.columns[self.index_mapping[618]],
                BehaviorBool: &self.row.columns[self.index_mapping[619]],
                AcceptBool: &self.row.columns[self.index_mapping[620]],
                QualifiedBool: &self.row.columns[self.index_mapping[621]],
                CanTargetBool: &self.row.columns[self.index_mapping[622]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[623]],
                ConditionValue: &self.row.columns[self.index_mapping[624]],
                Behavior: &self.row.columns[self.index_mapping[625]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[626]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[627]],
                Unknown0: &self.row.columns[self.index_mapping[628]],
                Unknown1: &self.row.columns[self.index_mapping[629]],
                QuestUInt8A: &self.row.columns[self.index_mapping[630]],
                ConditionType: &self.row.columns[self.index_mapping[631]],
                ConditionOperator: &self.row.columns[self.index_mapping[632]],
                VisibleBool: &self.row.columns[self.index_mapping[633]],
                ConditionBool: &self.row.columns[self.index_mapping[634]],
                ItemBool: &self.row.columns[self.index_mapping[635]],
                AnnounceBool: &self.row.columns[self.index_mapping[636]],
                BehaviorBool: &self.row.columns[self.index_mapping[637]],
                AcceptBool: &self.row.columns[self.index_mapping[638]],
                QualifiedBool: &self.row.columns[self.index_mapping[639]],
                CanTargetBool: &self.row.columns[self.index_mapping[640]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[641]],
                ConditionValue: &self.row.columns[self.index_mapping[642]],
                Behavior: &self.row.columns[self.index_mapping[643]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[644]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[645]],
                Unknown0: &self.row.columns[self.index_mapping[646]],
                Unknown1: &self.row.columns[self.index_mapping[647]],
                QuestUInt8A: &self.row.columns[self.index_mapping[648]],
                ConditionType: &self.row.columns[self.index_mapping[649]],
                ConditionOperator: &self.row.columns[self.index_mapping[650]],
                VisibleBool: &self.row.columns[self.index_mapping[651]],
                ConditionBool: &self.row.columns[self.index_mapping[652]],
                ItemBool: &self.row.columns[self.index_mapping[653]],
                AnnounceBool: &self.row.columns[self.index_mapping[654]],
                BehaviorBool: &self.row.columns[self.index_mapping[655]],
                AcceptBool: &self.row.columns[self.index_mapping[656]],
                QualifiedBool: &self.row.columns[self.index_mapping[657]],
                CanTargetBool: &self.row.columns[self.index_mapping[658]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[659]],
                ConditionValue: &self.row.columns[self.index_mapping[660]],
                Behavior: &self.row.columns[self.index_mapping[661]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[662]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[663]],
                Unknown0: &self.row.columns[self.index_mapping[664]],
                Unknown1: &self.row.columns[self.index_mapping[665]],
                QuestUInt8A: &self.row.columns[self.index_mapping[666]],
                ConditionType: &self.row.columns[self.index_mapping[667]],
                ConditionOperator: &self.row.columns[self.index_mapping[668]],
                VisibleBool: &self.row.columns[self.index_mapping[669]],
                ConditionBool: &self.row.columns[self.index_mapping[670]],
                ItemBool: &self.row.columns[self.index_mapping[671]],
                AnnounceBool: &self.row.columns[self.index_mapping[672]],
                BehaviorBool: &self.row.columns[self.index_mapping[673]],
                AcceptBool: &self.row.columns[self.index_mapping[674]],
                QualifiedBool: &self.row.columns[self.index_mapping[675]],
                CanTargetBool: &self.row.columns[self.index_mapping[676]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[677]],
                ConditionValue: &self.row.columns[self.index_mapping[678]],
                Behavior: &self.row.columns[self.index_mapping[679]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[680]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[681]],
                Unknown0: &self.row.columns[self.index_mapping[682]],
                Unknown1: &self.row.columns[self.index_mapping[683]],
                QuestUInt8A: &self.row.columns[self.index_mapping[684]],
                ConditionType: &self.row.columns[self.index_mapping[685]],
                ConditionOperator: &self.row.columns[self.index_mapping[686]],
                VisibleBool: &self.row.columns[self.index_mapping[687]],
                ConditionBool: &self.row.columns[self.index_mapping[688]],
                ItemBool: &self.row.columns[self.index_mapping[689]],
                AnnounceBool: &self.row.columns[self.index_mapping[690]],
                BehaviorBool: &self.row.columns[self.index_mapping[691]],
                AcceptBool: &self.row.columns[self.index_mapping[692]],
                QualifiedBool: &self.row.columns[self.index_mapping[693]],
                CanTargetBool: &self.row.columns[self.index_mapping[694]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[695]],
                ConditionValue: &self.row.columns[self.index_mapping[696]],
                Behavior: &self.row.columns[self.index_mapping[697]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[698]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[699]],
                Unknown0: &self.row.columns[self.index_mapping[700]],
                Unknown1: &self.row.columns[self.index_mapping[701]],
                QuestUInt8A: &self.row.columns[self.index_mapping[702]],
                ConditionType: &self.row.columns[self.index_mapping[703]],
                ConditionOperator: &self.row.columns[self.index_mapping[704]],
                VisibleBool: &self.row.columns[self.index_mapping[705]],
                ConditionBool: &self.row.columns[self.index_mapping[706]],
                ItemBool: &self.row.columns[self.index_mapping[707]],
                AnnounceBool: &self.row.columns[self.index_mapping[708]],
                BehaviorBool: &self.row.columns[self.index_mapping[709]],
                AcceptBool: &self.row.columns[self.index_mapping[710]],
                QualifiedBool: &self.row.columns[self.index_mapping[711]],
                CanTargetBool: &self.row.columns[self.index_mapping[712]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[713]],
                ConditionValue: &self.row.columns[self.index_mapping[714]],
                Behavior: &self.row.columns[self.index_mapping[715]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[716]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[717]],
                Unknown0: &self.row.columns[self.index_mapping[718]],
                Unknown1: &self.row.columns[self.index_mapping[719]],
                QuestUInt8A: &self.row.columns[self.index_mapping[720]],
                ConditionType: &self.row.columns[self.index_mapping[721]],
                ConditionOperator: &self.row.columns[self.index_mapping[722]],
                VisibleBool: &self.row.columns[self.index_mapping[723]],
                ConditionBool: &self.row.columns[self.index_mapping[724]],
                ItemBool: &self.row.columns[self.index_mapping[725]],
                AnnounceBool: &self.row.columns[self.index_mapping[726]],
                BehaviorBool: &self.row.columns[self.index_mapping[727]],
                AcceptBool: &self.row.columns[self.index_mapping[728]],
                QualifiedBool: &self.row.columns[self.index_mapping[729]],
                CanTargetBool: &self.row.columns[self.index_mapping[730]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[731]],
                ConditionValue: &self.row.columns[self.index_mapping[732]],
                Behavior: &self.row.columns[self.index_mapping[733]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[734]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[735]],
                Unknown0: &self.row.columns[self.index_mapping[736]],
                Unknown1: &self.row.columns[self.index_mapping[737]],
                QuestUInt8A: &self.row.columns[self.index_mapping[738]],
                ConditionType: &self.row.columns[self.index_mapping[739]],
                ConditionOperator: &self.row.columns[self.index_mapping[740]],
                VisibleBool: &self.row.columns[self.index_mapping[741]],
                ConditionBool: &self.row.columns[self.index_mapping[742]],
                ItemBool: &self.row.columns[self.index_mapping[743]],
                AnnounceBool: &self.row.columns[self.index_mapping[744]],
                BehaviorBool: &self.row.columns[self.index_mapping[745]],
                AcceptBool: &self.row.columns[self.index_mapping[746]],
                QualifiedBool: &self.row.columns[self.index_mapping[747]],
                CanTargetBool: &self.row.columns[self.index_mapping[748]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[749]],
                ConditionValue: &self.row.columns[self.index_mapping[750]],
                Behavior: &self.row.columns[self.index_mapping[751]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[752]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[753]],
                Unknown0: &self.row.columns[self.index_mapping[754]],
                Unknown1: &self.row.columns[self.index_mapping[755]],
                QuestUInt8A: &self.row.columns[self.index_mapping[756]],
                ConditionType: &self.row.columns[self.index_mapping[757]],
                ConditionOperator: &self.row.columns[self.index_mapping[758]],
                VisibleBool: &self.row.columns[self.index_mapping[759]],
                ConditionBool: &self.row.columns[self.index_mapping[760]],
                ItemBool: &self.row.columns[self.index_mapping[761]],
                AnnounceBool: &self.row.columns[self.index_mapping[762]],
                BehaviorBool: &self.row.columns[self.index_mapping[763]],
                AcceptBool: &self.row.columns[self.index_mapping[764]],
                QualifiedBool: &self.row.columns[self.index_mapping[765]],
                CanTargetBool: &self.row.columns[self.index_mapping[766]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[767]],
                ConditionValue: &self.row.columns[self.index_mapping[768]],
                Behavior: &self.row.columns[self.index_mapping[769]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[770]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[771]],
                Unknown0: &self.row.columns[self.index_mapping[772]],
                Unknown1: &self.row.columns[self.index_mapping[773]],
                QuestUInt8A: &self.row.columns[self.index_mapping[774]],
                ConditionType: &self.row.columns[self.index_mapping[775]],
                ConditionOperator: &self.row.columns[self.index_mapping[776]],
                VisibleBool: &self.row.columns[self.index_mapping[777]],
                ConditionBool: &self.row.columns[self.index_mapping[778]],
                ItemBool: &self.row.columns[self.index_mapping[779]],
                AnnounceBool: &self.row.columns[self.index_mapping[780]],
                BehaviorBool: &self.row.columns[self.index_mapping[781]],
                AcceptBool: &self.row.columns[self.index_mapping[782]],
                QualifiedBool: &self.row.columns[self.index_mapping[783]],
                CanTargetBool: &self.row.columns[self.index_mapping[784]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[785]],
                ConditionValue: &self.row.columns[self.index_mapping[786]],
                Behavior: &self.row.columns[self.index_mapping[787]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[788]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[789]],
                Unknown0: &self.row.columns[self.index_mapping[790]],
                Unknown1: &self.row.columns[self.index_mapping[791]],
                QuestUInt8A: &self.row.columns[self.index_mapping[792]],
                ConditionType: &self.row.columns[self.index_mapping[793]],
                ConditionOperator: &self.row.columns[self.index_mapping[794]],
                VisibleBool: &self.row.columns[self.index_mapping[795]],
                ConditionBool: &self.row.columns[self.index_mapping[796]],
                ItemBool: &self.row.columns[self.index_mapping[797]],
                AnnounceBool: &self.row.columns[self.index_mapping[798]],
                BehaviorBool: &self.row.columns[self.index_mapping[799]],
                AcceptBool: &self.row.columns[self.index_mapping[800]],
                QualifiedBool: &self.row.columns[self.index_mapping[801]],
                CanTargetBool: &self.row.columns[self.index_mapping[802]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[803]],
                ConditionValue: &self.row.columns[self.index_mapping[804]],
                Behavior: &self.row.columns[self.index_mapping[805]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[806]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[807]],
                Unknown0: &self.row.columns[self.index_mapping[808]],
                Unknown1: &self.row.columns[self.index_mapping[809]],
                QuestUInt8A: &self.row.columns[self.index_mapping[810]],
                ConditionType: &self.row.columns[self.index_mapping[811]],
                ConditionOperator: &self.row.columns[self.index_mapping[812]],
                VisibleBool: &self.row.columns[self.index_mapping[813]],
                ConditionBool: &self.row.columns[self.index_mapping[814]],
                ItemBool: &self.row.columns[self.index_mapping[815]],
                AnnounceBool: &self.row.columns[self.index_mapping[816]],
                BehaviorBool: &self.row.columns[self.index_mapping[817]],
                AcceptBool: &self.row.columns[self.index_mapping[818]],
                QualifiedBool: &self.row.columns[self.index_mapping[819]],
                CanTargetBool: &self.row.columns[self.index_mapping[820]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[821]],
                ConditionValue: &self.row.columns[self.index_mapping[822]],
                Behavior: &self.row.columns[self.index_mapping[823]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[824]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[825]],
                Unknown0: &self.row.columns[self.index_mapping[826]],
                Unknown1: &self.row.columns[self.index_mapping[827]],
                QuestUInt8A: &self.row.columns[self.index_mapping[828]],
                ConditionType: &self.row.columns[self.index_mapping[829]],
                ConditionOperator: &self.row.columns[self.index_mapping[830]],
                VisibleBool: &self.row.columns[self.index_mapping[831]],
                ConditionBool: &self.row.columns[self.index_mapping[832]],
                ItemBool: &self.row.columns[self.index_mapping[833]],
                AnnounceBool: &self.row.columns[self.index_mapping[834]],
                BehaviorBool: &self.row.columns[self.index_mapping[835]],
                AcceptBool: &self.row.columns[self.index_mapping[836]],
                QualifiedBool: &self.row.columns[self.index_mapping[837]],
                CanTargetBool: &self.row.columns[self.index_mapping[838]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[839]],
                ConditionValue: &self.row.columns[self.index_mapping[840]],
                Behavior: &self.row.columns[self.index_mapping[841]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[842]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[843]],
                Unknown0: &self.row.columns[self.index_mapping[844]],
                Unknown1: &self.row.columns[self.index_mapping[845]],
                QuestUInt8A: &self.row.columns[self.index_mapping[846]],
                ConditionType: &self.row.columns[self.index_mapping[847]],
                ConditionOperator: &self.row.columns[self.index_mapping[848]],
                VisibleBool: &self.row.columns[self.index_mapping[849]],
                ConditionBool: &self.row.columns[self.index_mapping[850]],
                ItemBool: &self.row.columns[self.index_mapping[851]],
                AnnounceBool: &self.row.columns[self.index_mapping[852]],
                BehaviorBool: &self.row.columns[self.index_mapping[853]],
                AcceptBool: &self.row.columns[self.index_mapping[854]],
                QualifiedBool: &self.row.columns[self.index_mapping[855]],
                CanTargetBool: &self.row.columns[self.index_mapping[856]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[857]],
                ConditionValue: &self.row.columns[self.index_mapping[858]],
                Behavior: &self.row.columns[self.index_mapping[859]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[860]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[861]],
                Unknown0: &self.row.columns[self.index_mapping[862]],
                Unknown1: &self.row.columns[self.index_mapping[863]],
                QuestUInt8A: &self.row.columns[self.index_mapping[864]],
                ConditionType: &self.row.columns[self.index_mapping[865]],
                ConditionOperator: &self.row.columns[self.index_mapping[866]],
                VisibleBool: &self.row.columns[self.index_mapping[867]],
                ConditionBool: &self.row.columns[self.index_mapping[868]],
                ItemBool: &self.row.columns[self.index_mapping[869]],
                AnnounceBool: &self.row.columns[self.index_mapping[870]],
                BehaviorBool: &self.row.columns[self.index_mapping[871]],
                AcceptBool: &self.row.columns[self.index_mapping[872]],
                QualifiedBool: &self.row.columns[self.index_mapping[873]],
                CanTargetBool: &self.row.columns[self.index_mapping[874]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[875]],
                ConditionValue: &self.row.columns[self.index_mapping[876]],
                Behavior: &self.row.columns[self.index_mapping[877]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[878]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[879]],
                Unknown0: &self.row.columns[self.index_mapping[880]],
                Unknown1: &self.row.columns[self.index_mapping[881]],
                QuestUInt8A: &self.row.columns[self.index_mapping[882]],
                ConditionType: &self.row.columns[self.index_mapping[883]],
                ConditionOperator: &self.row.columns[self.index_mapping[884]],
                VisibleBool: &self.row.columns[self.index_mapping[885]],
                ConditionBool: &self.row.columns[self.index_mapping[886]],
                ItemBool: &self.row.columns[self.index_mapping[887]],
                AnnounceBool: &self.row.columns[self.index_mapping[888]],
                BehaviorBool: &self.row.columns[self.index_mapping[889]],
                AcceptBool: &self.row.columns[self.index_mapping[890]],
                QualifiedBool: &self.row.columns[self.index_mapping[891]],
                CanTargetBool: &self.row.columns[self.index_mapping[892]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[893]],
                ConditionValue: &self.row.columns[self.index_mapping[894]],
                Behavior: &self.row.columns[self.index_mapping[895]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[896]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[897]],
                Unknown0: &self.row.columns[self.index_mapping[898]],
                Unknown1: &self.row.columns[self.index_mapping[899]],
                QuestUInt8A: &self.row.columns[self.index_mapping[900]],
                ConditionType: &self.row.columns[self.index_mapping[901]],
                ConditionOperator: &self.row.columns[self.index_mapping[902]],
                VisibleBool: &self.row.columns[self.index_mapping[903]],
                ConditionBool: &self.row.columns[self.index_mapping[904]],
                ItemBool: &self.row.columns[self.index_mapping[905]],
                AnnounceBool: &self.row.columns[self.index_mapping[906]],
                BehaviorBool: &self.row.columns[self.index_mapping[907]],
                AcceptBool: &self.row.columns[self.index_mapping[908]],
                QualifiedBool: &self.row.columns[self.index_mapping[909]],
                CanTargetBool: &self.row.columns[self.index_mapping[910]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[911]],
                ConditionValue: &self.row.columns[self.index_mapping[912]],
                Behavior: &self.row.columns[self.index_mapping[913]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[914]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[915]],
                Unknown0: &self.row.columns[self.index_mapping[916]],
                Unknown1: &self.row.columns[self.index_mapping[917]],
                QuestUInt8A: &self.row.columns[self.index_mapping[918]],
                ConditionType: &self.row.columns[self.index_mapping[919]],
                ConditionOperator: &self.row.columns[self.index_mapping[920]],
                VisibleBool: &self.row.columns[self.index_mapping[921]],
                ConditionBool: &self.row.columns[self.index_mapping[922]],
                ItemBool: &self.row.columns[self.index_mapping[923]],
                AnnounceBool: &self.row.columns[self.index_mapping[924]],
                BehaviorBool: &self.row.columns[self.index_mapping[925]],
                AcceptBool: &self.row.columns[self.index_mapping[926]],
                QualifiedBool: &self.row.columns[self.index_mapping[927]],
                CanTargetBool: &self.row.columns[self.index_mapping[928]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[929]],
                ConditionValue: &self.row.columns[self.index_mapping[930]],
                Behavior: &self.row.columns[self.index_mapping[931]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[932]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[933]],
                Unknown0: &self.row.columns[self.index_mapping[934]],
                Unknown1: &self.row.columns[self.index_mapping[935]],
                QuestUInt8A: &self.row.columns[self.index_mapping[936]],
                ConditionType: &self.row.columns[self.index_mapping[937]],
                ConditionOperator: &self.row.columns[self.index_mapping[938]],
                VisibleBool: &self.row.columns[self.index_mapping[939]],
                ConditionBool: &self.row.columns[self.index_mapping[940]],
                ItemBool: &self.row.columns[self.index_mapping[941]],
                AnnounceBool: &self.row.columns[self.index_mapping[942]],
                BehaviorBool: &self.row.columns[self.index_mapping[943]],
                AcceptBool: &self.row.columns[self.index_mapping[944]],
                QualifiedBool: &self.row.columns[self.index_mapping[945]],
                CanTargetBool: &self.row.columns[self.index_mapping[946]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[947]],
                ConditionValue: &self.row.columns[self.index_mapping[948]],
                Behavior: &self.row.columns[self.index_mapping[949]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[950]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[951]],
                Unknown0: &self.row.columns[self.index_mapping[952]],
                Unknown1: &self.row.columns[self.index_mapping[953]],
                QuestUInt8A: &self.row.columns[self.index_mapping[954]],
                ConditionType: &self.row.columns[self.index_mapping[955]],
                ConditionOperator: &self.row.columns[self.index_mapping[956]],
                VisibleBool: &self.row.columns[self.index_mapping[957]],
                ConditionBool: &self.row.columns[self.index_mapping[958]],
                ItemBool: &self.row.columns[self.index_mapping[959]],
                AnnounceBool: &self.row.columns[self.index_mapping[960]],
                BehaviorBool: &self.row.columns[self.index_mapping[961]],
                AcceptBool: &self.row.columns[self.index_mapping[962]],
                QualifiedBool: &self.row.columns[self.index_mapping[963]],
                CanTargetBool: &self.row.columns[self.index_mapping[964]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[965]],
                ConditionValue: &self.row.columns[self.index_mapping[966]],
                Behavior: &self.row.columns[self.index_mapping[967]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[968]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[969]],
                Unknown0: &self.row.columns[self.index_mapping[970]],
                Unknown1: &self.row.columns[self.index_mapping[971]],
                QuestUInt8A: &self.row.columns[self.index_mapping[972]],
                ConditionType: &self.row.columns[self.index_mapping[973]],
                ConditionOperator: &self.row.columns[self.index_mapping[974]],
                VisibleBool: &self.row.columns[self.index_mapping[975]],
                ConditionBool: &self.row.columns[self.index_mapping[976]],
                ItemBool: &self.row.columns[self.index_mapping[977]],
                AnnounceBool: &self.row.columns[self.index_mapping[978]],
                BehaviorBool: &self.row.columns[self.index_mapping[979]],
                AcceptBool: &self.row.columns[self.index_mapping[980]],
                QualifiedBool: &self.row.columns[self.index_mapping[981]],
                CanTargetBool: &self.row.columns[self.index_mapping[982]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[983]],
                ConditionValue: &self.row.columns[self.index_mapping[984]],
                Behavior: &self.row.columns[self.index_mapping[985]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[986]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[987]],
                Unknown0: &self.row.columns[self.index_mapping[988]],
                Unknown1: &self.row.columns[self.index_mapping[989]],
                QuestUInt8A: &self.row.columns[self.index_mapping[990]],
                ConditionType: &self.row.columns[self.index_mapping[991]],
                ConditionOperator: &self.row.columns[self.index_mapping[992]],
                VisibleBool: &self.row.columns[self.index_mapping[993]],
                ConditionBool: &self.row.columns[self.index_mapping[994]],
                ItemBool: &self.row.columns[self.index_mapping[995]],
                AnnounceBool: &self.row.columns[self.index_mapping[996]],
                BehaviorBool: &self.row.columns[self.index_mapping[997]],
                AcceptBool: &self.row.columns[self.index_mapping[998]],
                QualifiedBool: &self.row.columns[self.index_mapping[999]],
                CanTargetBool: &self.row.columns[self.index_mapping[1000]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1001]],
                ConditionValue: &self.row.columns[self.index_mapping[1002]],
                Behavior: &self.row.columns[self.index_mapping[1003]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1004]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1005]],
                Unknown0: &self.row.columns[self.index_mapping[1006]],
                Unknown1: &self.row.columns[self.index_mapping[1007]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1008]],
                ConditionType: &self.row.columns[self.index_mapping[1009]],
                ConditionOperator: &self.row.columns[self.index_mapping[1010]],
                VisibleBool: &self.row.columns[self.index_mapping[1011]],
                ConditionBool: &self.row.columns[self.index_mapping[1012]],
                ItemBool: &self.row.columns[self.index_mapping[1013]],
                AnnounceBool: &self.row.columns[self.index_mapping[1014]],
                BehaviorBool: &self.row.columns[self.index_mapping[1015]],
                AcceptBool: &self.row.columns[self.index_mapping[1016]],
                QualifiedBool: &self.row.columns[self.index_mapping[1017]],
                CanTargetBool: &self.row.columns[self.index_mapping[1018]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1019]],
                ConditionValue: &self.row.columns[self.index_mapping[1020]],
                Behavior: &self.row.columns[self.index_mapping[1021]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1022]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1023]],
                Unknown0: &self.row.columns[self.index_mapping[1024]],
                Unknown1: &self.row.columns[self.index_mapping[1025]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1026]],
                ConditionType: &self.row.columns[self.index_mapping[1027]],
                ConditionOperator: &self.row.columns[self.index_mapping[1028]],
                VisibleBool: &self.row.columns[self.index_mapping[1029]],
                ConditionBool: &self.row.columns[self.index_mapping[1030]],
                ItemBool: &self.row.columns[self.index_mapping[1031]],
                AnnounceBool: &self.row.columns[self.index_mapping[1032]],
                BehaviorBool: &self.row.columns[self.index_mapping[1033]],
                AcceptBool: &self.row.columns[self.index_mapping[1034]],
                QualifiedBool: &self.row.columns[self.index_mapping[1035]],
                CanTargetBool: &self.row.columns[self.index_mapping[1036]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1037]],
                ConditionValue: &self.row.columns[self.index_mapping[1038]],
                Behavior: &self.row.columns[self.index_mapping[1039]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1040]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1041]],
                Unknown0: &self.row.columns[self.index_mapping[1042]],
                Unknown1: &self.row.columns[self.index_mapping[1043]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1044]],
                ConditionType: &self.row.columns[self.index_mapping[1045]],
                ConditionOperator: &self.row.columns[self.index_mapping[1046]],
                VisibleBool: &self.row.columns[self.index_mapping[1047]],
                ConditionBool: &self.row.columns[self.index_mapping[1048]],
                ItemBool: &self.row.columns[self.index_mapping[1049]],
                AnnounceBool: &self.row.columns[self.index_mapping[1050]],
                BehaviorBool: &self.row.columns[self.index_mapping[1051]],
                AcceptBool: &self.row.columns[self.index_mapping[1052]],
                QualifiedBool: &self.row.columns[self.index_mapping[1053]],
                CanTargetBool: &self.row.columns[self.index_mapping[1054]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1055]],
                ConditionValue: &self.row.columns[self.index_mapping[1056]],
                Behavior: &self.row.columns[self.index_mapping[1057]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1058]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1059]],
                Unknown0: &self.row.columns[self.index_mapping[1060]],
                Unknown1: &self.row.columns[self.index_mapping[1061]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1062]],
                ConditionType: &self.row.columns[self.index_mapping[1063]],
                ConditionOperator: &self.row.columns[self.index_mapping[1064]],
                VisibleBool: &self.row.columns[self.index_mapping[1065]],
                ConditionBool: &self.row.columns[self.index_mapping[1066]],
                ItemBool: &self.row.columns[self.index_mapping[1067]],
                AnnounceBool: &self.row.columns[self.index_mapping[1068]],
                BehaviorBool: &self.row.columns[self.index_mapping[1069]],
                AcceptBool: &self.row.columns[self.index_mapping[1070]],
                QualifiedBool: &self.row.columns[self.index_mapping[1071]],
                CanTargetBool: &self.row.columns[self.index_mapping[1072]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1073]],
                ConditionValue: &self.row.columns[self.index_mapping[1074]],
                Behavior: &self.row.columns[self.index_mapping[1075]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1076]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1077]],
                Unknown0: &self.row.columns[self.index_mapping[1078]],
                Unknown1: &self.row.columns[self.index_mapping[1079]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1080]],
                ConditionType: &self.row.columns[self.index_mapping[1081]],
                ConditionOperator: &self.row.columns[self.index_mapping[1082]],
                VisibleBool: &self.row.columns[self.index_mapping[1083]],
                ConditionBool: &self.row.columns[self.index_mapping[1084]],
                ItemBool: &self.row.columns[self.index_mapping[1085]],
                AnnounceBool: &self.row.columns[self.index_mapping[1086]],
                BehaviorBool: &self.row.columns[self.index_mapping[1087]],
                AcceptBool: &self.row.columns[self.index_mapping[1088]],
                QualifiedBool: &self.row.columns[self.index_mapping[1089]],
                CanTargetBool: &self.row.columns[self.index_mapping[1090]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1091]],
                ConditionValue: &self.row.columns[self.index_mapping[1092]],
                Behavior: &self.row.columns[self.index_mapping[1093]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1094]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1095]],
                Unknown0: &self.row.columns[self.index_mapping[1096]],
                Unknown1: &self.row.columns[self.index_mapping[1097]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1098]],
                ConditionType: &self.row.columns[self.index_mapping[1099]],
                ConditionOperator: &self.row.columns[self.index_mapping[1100]],
                VisibleBool: &self.row.columns[self.index_mapping[1101]],
                ConditionBool: &self.row.columns[self.index_mapping[1102]],
                ItemBool: &self.row.columns[self.index_mapping[1103]],
                AnnounceBool: &self.row.columns[self.index_mapping[1104]],
                BehaviorBool: &self.row.columns[self.index_mapping[1105]],
                AcceptBool: &self.row.columns[self.index_mapping[1106]],
                QualifiedBool: &self.row.columns[self.index_mapping[1107]],
                CanTargetBool: &self.row.columns[self.index_mapping[1108]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1109]],
                ConditionValue: &self.row.columns[self.index_mapping[1110]],
                Behavior: &self.row.columns[self.index_mapping[1111]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1112]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1113]],
                Unknown0: &self.row.columns[self.index_mapping[1114]],
                Unknown1: &self.row.columns[self.index_mapping[1115]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1116]],
                ConditionType: &self.row.columns[self.index_mapping[1117]],
                ConditionOperator: &self.row.columns[self.index_mapping[1118]],
                VisibleBool: &self.row.columns[self.index_mapping[1119]],
                ConditionBool: &self.row.columns[self.index_mapping[1120]],
                ItemBool: &self.row.columns[self.index_mapping[1121]],
                AnnounceBool: &self.row.columns[self.index_mapping[1122]],
                BehaviorBool: &self.row.columns[self.index_mapping[1123]],
                AcceptBool: &self.row.columns[self.index_mapping[1124]],
                QualifiedBool: &self.row.columns[self.index_mapping[1125]],
                CanTargetBool: &self.row.columns[self.index_mapping[1126]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1127]],
                ConditionValue: &self.row.columns[self.index_mapping[1128]],
                Behavior: &self.row.columns[self.index_mapping[1129]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1130]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1131]],
                Unknown0: &self.row.columns[self.index_mapping[1132]],
                Unknown1: &self.row.columns[self.index_mapping[1133]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1134]],
                ConditionType: &self.row.columns[self.index_mapping[1135]],
                ConditionOperator: &self.row.columns[self.index_mapping[1136]],
                VisibleBool: &self.row.columns[self.index_mapping[1137]],
                ConditionBool: &self.row.columns[self.index_mapping[1138]],
                ItemBool: &self.row.columns[self.index_mapping[1139]],
                AnnounceBool: &self.row.columns[self.index_mapping[1140]],
                BehaviorBool: &self.row.columns[self.index_mapping[1141]],
                AcceptBool: &self.row.columns[self.index_mapping[1142]],
                QualifiedBool: &self.row.columns[self.index_mapping[1143]],
                CanTargetBool: &self.row.columns[self.index_mapping[1144]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1145]],
                ConditionValue: &self.row.columns[self.index_mapping[1146]],
                Behavior: &self.row.columns[self.index_mapping[1147]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1148]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1149]],
                Unknown0: &self.row.columns[self.index_mapping[1150]],
                Unknown1: &self.row.columns[self.index_mapping[1151]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1152]],
                ConditionType: &self.row.columns[self.index_mapping[1153]],
                ConditionOperator: &self.row.columns[self.index_mapping[1154]],
                VisibleBool: &self.row.columns[self.index_mapping[1155]],
                ConditionBool: &self.row.columns[self.index_mapping[1156]],
                ItemBool: &self.row.columns[self.index_mapping[1157]],
                AnnounceBool: &self.row.columns[self.index_mapping[1158]],
                BehaviorBool: &self.row.columns[self.index_mapping[1159]],
                AcceptBool: &self.row.columns[self.index_mapping[1160]],
                QualifiedBool: &self.row.columns[self.index_mapping[1161]],
                CanTargetBool: &self.row.columns[self.index_mapping[1162]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1163]],
                ConditionValue: &self.row.columns[self.index_mapping[1164]],
                Behavior: &self.row.columns[self.index_mapping[1165]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1166]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1167]],
                Unknown0: &self.row.columns[self.index_mapping[1168]],
                Unknown1: &self.row.columns[self.index_mapping[1169]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1170]],
                ConditionType: &self.row.columns[self.index_mapping[1171]],
                ConditionOperator: &self.row.columns[self.index_mapping[1172]],
                VisibleBool: &self.row.columns[self.index_mapping[1173]],
                ConditionBool: &self.row.columns[self.index_mapping[1174]],
                ItemBool: &self.row.columns[self.index_mapping[1175]],
                AnnounceBool: &self.row.columns[self.index_mapping[1176]],
                BehaviorBool: &self.row.columns[self.index_mapping[1177]],
                AcceptBool: &self.row.columns[self.index_mapping[1178]],
                QualifiedBool: &self.row.columns[self.index_mapping[1179]],
                CanTargetBool: &self.row.columns[self.index_mapping[1180]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1181]],
                ConditionValue: &self.row.columns[self.index_mapping[1182]],
                Behavior: &self.row.columns[self.index_mapping[1183]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1184]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1185]],
                Unknown0: &self.row.columns[self.index_mapping[1186]],
                Unknown1: &self.row.columns[self.index_mapping[1187]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1188]],
                ConditionType: &self.row.columns[self.index_mapping[1189]],
                ConditionOperator: &self.row.columns[self.index_mapping[1190]],
                VisibleBool: &self.row.columns[self.index_mapping[1191]],
                ConditionBool: &self.row.columns[self.index_mapping[1192]],
                ItemBool: &self.row.columns[self.index_mapping[1193]],
                AnnounceBool: &self.row.columns[self.index_mapping[1194]],
                BehaviorBool: &self.row.columns[self.index_mapping[1195]],
                AcceptBool: &self.row.columns[self.index_mapping[1196]],
                QualifiedBool: &self.row.columns[self.index_mapping[1197]],
                CanTargetBool: &self.row.columns[self.index_mapping[1198]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1199]],
                ConditionValue: &self.row.columns[self.index_mapping[1200]],
                Behavior: &self.row.columns[self.index_mapping[1201]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1202]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1203]],
                Unknown0: &self.row.columns[self.index_mapping[1204]],
                Unknown1: &self.row.columns[self.index_mapping[1205]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1206]],
                ConditionType: &self.row.columns[self.index_mapping[1207]],
                ConditionOperator: &self.row.columns[self.index_mapping[1208]],
                VisibleBool: &self.row.columns[self.index_mapping[1209]],
                ConditionBool: &self.row.columns[self.index_mapping[1210]],
                ItemBool: &self.row.columns[self.index_mapping[1211]],
                AnnounceBool: &self.row.columns[self.index_mapping[1212]],
                BehaviorBool: &self.row.columns[self.index_mapping[1213]],
                AcceptBool: &self.row.columns[self.index_mapping[1214]],
                QualifiedBool: &self.row.columns[self.index_mapping[1215]],
                CanTargetBool: &self.row.columns[self.index_mapping[1216]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1217]],
                ConditionValue: &self.row.columns[self.index_mapping[1218]],
                Behavior: &self.row.columns[self.index_mapping[1219]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1220]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1221]],
                Unknown0: &self.row.columns[self.index_mapping[1222]],
                Unknown1: &self.row.columns[self.index_mapping[1223]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1224]],
                ConditionType: &self.row.columns[self.index_mapping[1225]],
                ConditionOperator: &self.row.columns[self.index_mapping[1226]],
                VisibleBool: &self.row.columns[self.index_mapping[1227]],
                ConditionBool: &self.row.columns[self.index_mapping[1228]],
                ItemBool: &self.row.columns[self.index_mapping[1229]],
                AnnounceBool: &self.row.columns[self.index_mapping[1230]],
                BehaviorBool: &self.row.columns[self.index_mapping[1231]],
                AcceptBool: &self.row.columns[self.index_mapping[1232]],
                QualifiedBool: &self.row.columns[self.index_mapping[1233]],
                CanTargetBool: &self.row.columns[self.index_mapping[1234]],
            },
            QuestListenerParamsElement {
                Listener: &self.row.columns[self.index_mapping[1235]],
                ConditionValue: &self.row.columns[self.index_mapping[1236]],
                Behavior: &self.row.columns[self.index_mapping[1237]],
                ActorSpawnSeq: &self.row.columns[self.index_mapping[1238]],
                ActorDespawnSeq: &self.row.columns[self.index_mapping[1239]],
                Unknown0: &self.row.columns[self.index_mapping[1240]],
                Unknown1: &self.row.columns[self.index_mapping[1241]],
                QuestUInt8A: &self.row.columns[self.index_mapping[1242]],
                ConditionType: &self.row.columns[self.index_mapping[1243]],
                ConditionOperator: &self.row.columns[self.index_mapping[1244]],
                VisibleBool: &self.row.columns[self.index_mapping[1245]],
                ConditionBool: &self.row.columns[self.index_mapping[1246]],
                ItemBool: &self.row.columns[self.index_mapping[1247]],
                AnnounceBool: &self.row.columns[self.index_mapping[1248]],
                BehaviorBool: &self.row.columns[self.index_mapping[1249]],
                AcceptBool: &self.row.columns[self.index_mapping[1250]],
                QualifiedBool: &self.row.columns[self.index_mapping[1251]],
                CanTargetBool: &self.row.columns[self.index_mapping[1252]],
            },
        ]
    }
    pub fn TodoParams(&'a self) -> [TodoParamsElement<'a>; 24] {
        [
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1253]],
                    &self.row.columns[self.index_mapping[1254]],
                    &self.row.columns[self.index_mapping[1255]],
                    &self.row.columns[self.index_mapping[1256]],
                    &self.row.columns[self.index_mapping[1257]],
                    &self.row.columns[self.index_mapping[1258]],
                    &self.row.columns[self.index_mapping[1259]],
                    &self.row.columns[self.index_mapping[1260]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1261]],
                ToDoQty: &self.row.columns[self.index_mapping[1262]],
                CountableNum: &self.row.columns[self.index_mapping[1263]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1264]],
                    &self.row.columns[self.index_mapping[1265]],
                    &self.row.columns[self.index_mapping[1266]],
                    &self.row.columns[self.index_mapping[1267]],
                    &self.row.columns[self.index_mapping[1268]],
                    &self.row.columns[self.index_mapping[1269]],
                    &self.row.columns[self.index_mapping[1270]],
                    &self.row.columns[self.index_mapping[1271]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1272]],
                ToDoQty: &self.row.columns[self.index_mapping[1273]],
                CountableNum: &self.row.columns[self.index_mapping[1274]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1275]],
                    &self.row.columns[self.index_mapping[1276]],
                    &self.row.columns[self.index_mapping[1277]],
                    &self.row.columns[self.index_mapping[1278]],
                    &self.row.columns[self.index_mapping[1279]],
                    &self.row.columns[self.index_mapping[1280]],
                    &self.row.columns[self.index_mapping[1281]],
                    &self.row.columns[self.index_mapping[1282]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1283]],
                ToDoQty: &self.row.columns[self.index_mapping[1284]],
                CountableNum: &self.row.columns[self.index_mapping[1285]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1286]],
                    &self.row.columns[self.index_mapping[1287]],
                    &self.row.columns[self.index_mapping[1288]],
                    &self.row.columns[self.index_mapping[1289]],
                    &self.row.columns[self.index_mapping[1290]],
                    &self.row.columns[self.index_mapping[1291]],
                    &self.row.columns[self.index_mapping[1292]],
                    &self.row.columns[self.index_mapping[1293]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1294]],
                ToDoQty: &self.row.columns[self.index_mapping[1295]],
                CountableNum: &self.row.columns[self.index_mapping[1296]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1297]],
                    &self.row.columns[self.index_mapping[1298]],
                    &self.row.columns[self.index_mapping[1299]],
                    &self.row.columns[self.index_mapping[1300]],
                    &self.row.columns[self.index_mapping[1301]],
                    &self.row.columns[self.index_mapping[1302]],
                    &self.row.columns[self.index_mapping[1303]],
                    &self.row.columns[self.index_mapping[1304]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1305]],
                ToDoQty: &self.row.columns[self.index_mapping[1306]],
                CountableNum: &self.row.columns[self.index_mapping[1307]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1308]],
                    &self.row.columns[self.index_mapping[1309]],
                    &self.row.columns[self.index_mapping[1310]],
                    &self.row.columns[self.index_mapping[1311]],
                    &self.row.columns[self.index_mapping[1312]],
                    &self.row.columns[self.index_mapping[1313]],
                    &self.row.columns[self.index_mapping[1314]],
                    &self.row.columns[self.index_mapping[1315]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1316]],
                ToDoQty: &self.row.columns[self.index_mapping[1317]],
                CountableNum: &self.row.columns[self.index_mapping[1318]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1319]],
                    &self.row.columns[self.index_mapping[1320]],
                    &self.row.columns[self.index_mapping[1321]],
                    &self.row.columns[self.index_mapping[1322]],
                    &self.row.columns[self.index_mapping[1323]],
                    &self.row.columns[self.index_mapping[1324]],
                    &self.row.columns[self.index_mapping[1325]],
                    &self.row.columns[self.index_mapping[1326]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1327]],
                ToDoQty: &self.row.columns[self.index_mapping[1328]],
                CountableNum: &self.row.columns[self.index_mapping[1329]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1330]],
                    &self.row.columns[self.index_mapping[1331]],
                    &self.row.columns[self.index_mapping[1332]],
                    &self.row.columns[self.index_mapping[1333]],
                    &self.row.columns[self.index_mapping[1334]],
                    &self.row.columns[self.index_mapping[1335]],
                    &self.row.columns[self.index_mapping[1336]],
                    &self.row.columns[self.index_mapping[1337]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1338]],
                ToDoQty: &self.row.columns[self.index_mapping[1339]],
                CountableNum: &self.row.columns[self.index_mapping[1340]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1341]],
                    &self.row.columns[self.index_mapping[1342]],
                    &self.row.columns[self.index_mapping[1343]],
                    &self.row.columns[self.index_mapping[1344]],
                    &self.row.columns[self.index_mapping[1345]],
                    &self.row.columns[self.index_mapping[1346]],
                    &self.row.columns[self.index_mapping[1347]],
                    &self.row.columns[self.index_mapping[1348]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1349]],
                ToDoQty: &self.row.columns[self.index_mapping[1350]],
                CountableNum: &self.row.columns[self.index_mapping[1351]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1352]],
                    &self.row.columns[self.index_mapping[1353]],
                    &self.row.columns[self.index_mapping[1354]],
                    &self.row.columns[self.index_mapping[1355]],
                    &self.row.columns[self.index_mapping[1356]],
                    &self.row.columns[self.index_mapping[1357]],
                    &self.row.columns[self.index_mapping[1358]],
                    &self.row.columns[self.index_mapping[1359]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1360]],
                ToDoQty: &self.row.columns[self.index_mapping[1361]],
                CountableNum: &self.row.columns[self.index_mapping[1362]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1363]],
                    &self.row.columns[self.index_mapping[1364]],
                    &self.row.columns[self.index_mapping[1365]],
                    &self.row.columns[self.index_mapping[1366]],
                    &self.row.columns[self.index_mapping[1367]],
                    &self.row.columns[self.index_mapping[1368]],
                    &self.row.columns[self.index_mapping[1369]],
                    &self.row.columns[self.index_mapping[1370]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1371]],
                ToDoQty: &self.row.columns[self.index_mapping[1372]],
                CountableNum: &self.row.columns[self.index_mapping[1373]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1374]],
                    &self.row.columns[self.index_mapping[1375]],
                    &self.row.columns[self.index_mapping[1376]],
                    &self.row.columns[self.index_mapping[1377]],
                    &self.row.columns[self.index_mapping[1378]],
                    &self.row.columns[self.index_mapping[1379]],
                    &self.row.columns[self.index_mapping[1380]],
                    &self.row.columns[self.index_mapping[1381]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1382]],
                ToDoQty: &self.row.columns[self.index_mapping[1383]],
                CountableNum: &self.row.columns[self.index_mapping[1384]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1385]],
                    &self.row.columns[self.index_mapping[1386]],
                    &self.row.columns[self.index_mapping[1387]],
                    &self.row.columns[self.index_mapping[1388]],
                    &self.row.columns[self.index_mapping[1389]],
                    &self.row.columns[self.index_mapping[1390]],
                    &self.row.columns[self.index_mapping[1391]],
                    &self.row.columns[self.index_mapping[1392]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1393]],
                ToDoQty: &self.row.columns[self.index_mapping[1394]],
                CountableNum: &self.row.columns[self.index_mapping[1395]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1396]],
                    &self.row.columns[self.index_mapping[1397]],
                    &self.row.columns[self.index_mapping[1398]],
                    &self.row.columns[self.index_mapping[1399]],
                    &self.row.columns[self.index_mapping[1400]],
                    &self.row.columns[self.index_mapping[1401]],
                    &self.row.columns[self.index_mapping[1402]],
                    &self.row.columns[self.index_mapping[1403]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1404]],
                ToDoQty: &self.row.columns[self.index_mapping[1405]],
                CountableNum: &self.row.columns[self.index_mapping[1406]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1407]],
                    &self.row.columns[self.index_mapping[1408]],
                    &self.row.columns[self.index_mapping[1409]],
                    &self.row.columns[self.index_mapping[1410]],
                    &self.row.columns[self.index_mapping[1411]],
                    &self.row.columns[self.index_mapping[1412]],
                    &self.row.columns[self.index_mapping[1413]],
                    &self.row.columns[self.index_mapping[1414]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1415]],
                ToDoQty: &self.row.columns[self.index_mapping[1416]],
                CountableNum: &self.row.columns[self.index_mapping[1417]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1418]],
                    &self.row.columns[self.index_mapping[1419]],
                    &self.row.columns[self.index_mapping[1420]],
                    &self.row.columns[self.index_mapping[1421]],
                    &self.row.columns[self.index_mapping[1422]],
                    &self.row.columns[self.index_mapping[1423]],
                    &self.row.columns[self.index_mapping[1424]],
                    &self.row.columns[self.index_mapping[1425]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1426]],
                ToDoQty: &self.row.columns[self.index_mapping[1427]],
                CountableNum: &self.row.columns[self.index_mapping[1428]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1429]],
                    &self.row.columns[self.index_mapping[1430]],
                    &self.row.columns[self.index_mapping[1431]],
                    &self.row.columns[self.index_mapping[1432]],
                    &self.row.columns[self.index_mapping[1433]],
                    &self.row.columns[self.index_mapping[1434]],
                    &self.row.columns[self.index_mapping[1435]],
                    &self.row.columns[self.index_mapping[1436]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1437]],
                ToDoQty: &self.row.columns[self.index_mapping[1438]],
                CountableNum: &self.row.columns[self.index_mapping[1439]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1440]],
                    &self.row.columns[self.index_mapping[1441]],
                    &self.row.columns[self.index_mapping[1442]],
                    &self.row.columns[self.index_mapping[1443]],
                    &self.row.columns[self.index_mapping[1444]],
                    &self.row.columns[self.index_mapping[1445]],
                    &self.row.columns[self.index_mapping[1446]],
                    &self.row.columns[self.index_mapping[1447]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1448]],
                ToDoQty: &self.row.columns[self.index_mapping[1449]],
                CountableNum: &self.row.columns[self.index_mapping[1450]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1451]],
                    &self.row.columns[self.index_mapping[1452]],
                    &self.row.columns[self.index_mapping[1453]],
                    &self.row.columns[self.index_mapping[1454]],
                    &self.row.columns[self.index_mapping[1455]],
                    &self.row.columns[self.index_mapping[1456]],
                    &self.row.columns[self.index_mapping[1457]],
                    &self.row.columns[self.index_mapping[1458]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1459]],
                ToDoQty: &self.row.columns[self.index_mapping[1460]],
                CountableNum: &self.row.columns[self.index_mapping[1461]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1462]],
                    &self.row.columns[self.index_mapping[1463]],
                    &self.row.columns[self.index_mapping[1464]],
                    &self.row.columns[self.index_mapping[1465]],
                    &self.row.columns[self.index_mapping[1466]],
                    &self.row.columns[self.index_mapping[1467]],
                    &self.row.columns[self.index_mapping[1468]],
                    &self.row.columns[self.index_mapping[1469]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1470]],
                ToDoQty: &self.row.columns[self.index_mapping[1471]],
                CountableNum: &self.row.columns[self.index_mapping[1472]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1473]],
                    &self.row.columns[self.index_mapping[1474]],
                    &self.row.columns[self.index_mapping[1475]],
                    &self.row.columns[self.index_mapping[1476]],
                    &self.row.columns[self.index_mapping[1477]],
                    &self.row.columns[self.index_mapping[1478]],
                    &self.row.columns[self.index_mapping[1479]],
                    &self.row.columns[self.index_mapping[1480]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1481]],
                ToDoQty: &self.row.columns[self.index_mapping[1482]],
                CountableNum: &self.row.columns[self.index_mapping[1483]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1484]],
                    &self.row.columns[self.index_mapping[1485]],
                    &self.row.columns[self.index_mapping[1486]],
                    &self.row.columns[self.index_mapping[1487]],
                    &self.row.columns[self.index_mapping[1488]],
                    &self.row.columns[self.index_mapping[1489]],
                    &self.row.columns[self.index_mapping[1490]],
                    &self.row.columns[self.index_mapping[1491]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1492]],
                ToDoQty: &self.row.columns[self.index_mapping[1493]],
                CountableNum: &self.row.columns[self.index_mapping[1494]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1495]],
                    &self.row.columns[self.index_mapping[1496]],
                    &self.row.columns[self.index_mapping[1497]],
                    &self.row.columns[self.index_mapping[1498]],
                    &self.row.columns[self.index_mapping[1499]],
                    &self.row.columns[self.index_mapping[1500]],
                    &self.row.columns[self.index_mapping[1501]],
                    &self.row.columns[self.index_mapping[1502]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1503]],
                ToDoQty: &self.row.columns[self.index_mapping[1504]],
                CountableNum: &self.row.columns[self.index_mapping[1505]],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.row.columns[self.index_mapping[1506]],
                    &self.row.columns[self.index_mapping[1507]],
                    &self.row.columns[self.index_mapping[1508]],
                    &self.row.columns[self.index_mapping[1509]],
                    &self.row.columns[self.index_mapping[1510]],
                    &self.row.columns[self.index_mapping[1511]],
                    &self.row.columns[self.index_mapping[1512]],
                    &self.row.columns[self.index_mapping[1513]],
                ],
                ToDoCompleteSeq: &self.row.columns[self.index_mapping[1514]],
                ToDoQty: &self.row.columns[self.index_mapping[1515]],
                CountableNum: &self.row.columns[self.index_mapping[1516]],
            },
        ]
    }
    pub fn GilReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1517]]
    }
    pub fn CurrencyReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1518]]
    }
    pub fn CurrencyRewardCount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1519]]
    }
    pub fn Reward(&'a self) -> [&'a Field; 7] {
        [
            &self.row.columns[self.index_mapping[1520]],
            &self.row.columns[self.index_mapping[1521]],
            &self.row.columns[self.index_mapping[1522]],
            &self.row.columns[self.index_mapping[1523]],
            &self.row.columns[self.index_mapping[1524]],
            &self.row.columns[self.index_mapping[1525]],
            &self.row.columns[self.index_mapping[1526]],
        ]
    }
    pub fn OptionalItemReward(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[1527]],
            &self.row.columns[self.index_mapping[1528]],
            &self.row.columns[self.index_mapping[1529]],
            &self.row.columns[self.index_mapping[1530]],
            &self.row.columns[self.index_mapping[1531]],
        ]
    }
    pub fn InstanceContentUnlock(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1532]]
    }
    pub fn ExpFactor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1533]]
    }
    pub fn EmoteReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1534]]
    }
    pub fn ActionReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1535]]
    }
    pub fn SystemReward(&'a self) -> [&'a Field; 2] {
        [
            &self.row.columns[self.index_mapping[1536]],
            &self.row.columns[self.index_mapping[1537]],
        ]
    }
    pub fn GCTypeReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1538]]
    }
    pub fn ItemCatalyst(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[1539]],
            &self.row.columns[self.index_mapping[1540]],
            &self.row.columns[self.index_mapping[1541]],
        ]
    }
    pub fn ItemCountCatalyst(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[1542]],
            &self.row.columns[self.index_mapping[1543]],
            &self.row.columns[self.index_mapping[1544]],
        ]
    }
    pub fn ItemRewardType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1545]]
    }
    pub fn ItemCountReward(&'a self) -> [&'a Field; 7] {
        [
            &self.row.columns[self.index_mapping[1546]],
            &self.row.columns[self.index_mapping[1547]],
            &self.row.columns[self.index_mapping[1548]],
            &self.row.columns[self.index_mapping[1549]],
            &self.row.columns[self.index_mapping[1550]],
            &self.row.columns[self.index_mapping[1551]],
            &self.row.columns[self.index_mapping[1552]],
        ]
    }
    pub fn RewardStain(&'a self) -> [&'a Field; 7] {
        [
            &self.row.columns[self.index_mapping[1553]],
            &self.row.columns[self.index_mapping[1554]],
            &self.row.columns[self.index_mapping[1555]],
            &self.row.columns[self.index_mapping[1556]],
            &self.row.columns[self.index_mapping[1557]],
            &self.row.columns[self.index_mapping[1558]],
            &self.row.columns[self.index_mapping[1559]],
        ]
    }
    pub fn OptionalItemCountReward(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[1560]],
            &self.row.columns[self.index_mapping[1561]],
            &self.row.columns[self.index_mapping[1562]],
            &self.row.columns[self.index_mapping[1563]],
            &self.row.columns[self.index_mapping[1564]],
        ]
    }
    pub fn OptionalItemStainReward(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[1565]],
            &self.row.columns[self.index_mapping[1566]],
            &self.row.columns[self.index_mapping[1567]],
            &self.row.columns[self.index_mapping[1568]],
            &self.row.columns[self.index_mapping[1569]],
        ]
    }
    pub fn GeneralActionReward(&'a self) -> [&'a Field; 2] {
        [
            &self.row.columns[self.index_mapping[1570]],
            &self.row.columns[self.index_mapping[1571]],
        ]
    }
    pub fn OtherReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1572]]
    }
    pub fn Tomestone(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1573]]
    }
    pub fn TomestoneReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1574]]
    }
    pub fn TomestoneCountReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1575]]
    }
    pub fn ReputationReward(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1576]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1577]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1578]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1579]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1580]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1581]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1582]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1583]]
    }
    pub fn OptionalItemIsHQReward(&'a self) -> [&'a Field; 5] {
        [
            &self.row.columns[self.index_mapping[1584]],
            &self.row.columns[self.index_mapping[1585]],
            &self.row.columns[self.index_mapping[1586]],
            &self.row.columns[self.index_mapping[1587]],
            &self.row.columns[self.index_mapping[1588]],
        ]
    }
    pub fn Id(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1589]]
    }
    pub fn PreviousQuest(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[1590]],
            &self.row.columns[self.index_mapping[1591]],
            &self.row.columns[self.index_mapping[1592]],
        ]
    }
    pub fn QuestLock(&'a self) -> [&'a Field; 2] {
        [
            &self.row.columns[self.index_mapping[1593]],
            &self.row.columns[self.index_mapping[1594]],
        ]
    }
    pub fn InstanceContent(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[1595]],
            &self.row.columns[self.index_mapping[1596]],
            &self.row.columns[self.index_mapping[1597]],
        ]
    }
    pub fn IssuerStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1598]]
    }
    pub fn IssuerLocation(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1599]]
    }
    pub fn TargetEnd(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1600]]
    }
    pub fn JournalGenre(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1601]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1602]]
    }
    pub fn IconSpecial(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1603]]
    }
    pub fn MountRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1604]]
    }
    pub fn ClassJobLevel(&'a self) -> [&'a Field; 2] {
        [
            &self.row.columns[self.index_mapping[1605]],
            &self.row.columns[self.index_mapping[1606]],
        ]
    }
    pub fn Header(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1607]]
    }
    pub fn Festival(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1608]]
    }
    pub fn BellStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1609]]
    }
    pub fn BellEnd(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1610]]
    }
    pub fn BeastReputationValue(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1611]]
    }
    pub fn ClientBehavior(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1612]]
    }
    pub fn QuestClassJobSupply(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1613]]
    }
    pub fn PlaceName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1614]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1615]]
    }
    pub fn Expansion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1616]]
    }
    pub fn ClassJobCategory0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1617]]
    }
    pub fn QuestLevelOffset(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1618]]
    }
    pub fn ClassJobCategory1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1619]]
    }
    pub fn PreviousQuestJoin(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1620]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1621]]
    }
    pub fn QuestLockJoin(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1622]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1623]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1624]]
    }
    pub fn ClassJobUnlock(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1625]]
    }
    pub fn GrandCompany(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1626]]
    }
    pub fn GrandCompanyRank(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1627]]
    }
    pub fn InstanceContentJoin(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1628]]
    }
    pub fn FestivalBegin(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1629]]
    }
    pub fn FestivalEnd(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1630]]
    }
    pub fn BeastTribe(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1631]]
    }
    pub fn BeastReputationRank(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1632]]
    }
    pub fn SatisfactionNpc(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1633]]
    }
    pub fn SatisfactionLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1634]]
    }
    pub fn DeliveryQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1635]]
    }
    pub fn RepeatIntervalType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1636]]
    }
    pub fn QuestRepeatFlag(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1637]]
    }
    pub fn Type(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1638]]
    }
    pub fn Unknown_70(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1639]]
    }
    pub fn LevelMax(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1640]]
    }
    pub fn ClassJobRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1641]]
    }
    pub fn QuestRewardOtherDisplay(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1642]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1643]]
    }
    pub fn EventIconType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1644]]
    }
    /// 1/2 - normal daily beast tribe quests, 3 - 'exclusive' (if player's rank is not greater than max rank requirement of quests offered by npc, exactly one of the available quests will be from this pool)
    pub fn DailyQuestPool(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1645]]
    }
    pub fn IsHouseRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1646]]
    }
    pub fn IsRepeatable(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1647]]
    }
    pub fn CanCancel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1648]]
    }
    pub fn Introduction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1649]]
    }
    pub fn HideOfferIcon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1650]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1651]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1652]]
    }
}
