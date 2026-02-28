//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct MountSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl MountSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Mount")?;
        let sheet = resolver.read_excel_sheet(&exh, "Mount", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<MountRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MountSheet {
    type Row = MountRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a MountSheet {
    type Item = (u32, Vec<(u16, MountRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MountSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MountSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MountRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> MountRow<'a> {
    pub fn Singular(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Plural(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Adjective(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn PossessivePronoun(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn StartsWithVowel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Pronoun(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Article(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn ModelChara(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn EquipHead(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn EquipBody(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn EquipLeg(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn EquipFoot(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn MoveControl(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn RideBGM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn UIPriority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn MountAction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn Unknown_70_1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn Order(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn FlyingCondition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn IsFlying(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn MountCustomize(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn ExitMoveDist(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn ExitMoveSpeed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn RadiusRate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn BaseMotionSpeed_Run(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn BaseMotionSpeed_Walk(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn ExtraSeats(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn IsEmote(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn IsAirborne(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn ExHotbarEnableConfig(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn UseEP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn IsImmobile(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn HideHeadgear(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
}
