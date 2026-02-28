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
pub struct ENpcDressUpDressSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ENpcDressUpDressSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ENpcDressUpDress")?;
        let sheet = resolver.read_excel_sheet(&exh, "ENpcDressUpDress", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ENpcDressUpDressSheet {
    type Row = ENpcDressUpDressRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ENpcDressUpDressSheet {
    type Item = (u32, Vec<(u16, ENpcDressUpDressRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ENpcDressUpDressSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ENpcDressUpDressSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ENpcDressUpDressRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ENpcDressUpDressRow<'a> {
    pub fn ModelMainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn ModelOffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn ENpc(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn ModelHead(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn ModelBody(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn ModelHands(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn ModelLegs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn ModelFeet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Behavior(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn Unknown21(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn Unknown22(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Unknown24(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn Unknown25(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Unknown26(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn Unknown27(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn Unknown28(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Unknown29(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown30(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown31(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown32(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown33(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn Unknown34(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn DyeMainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn Dye2MainHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn DyeOffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn Dye2OffHand(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn DyeHead(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn DyeBody(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn DyeHands(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn DyeLegs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn DyeFeet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn DyeEars(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn DyeNeck(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn DyeWrists(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn DyeLeftRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn DyeRightRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn Dye2Head(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn Dye2Body(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn Dye2Hands(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn Dye2Legs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn Dye2Feet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn Dye2Ears(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn Dye2Neck(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn Dye2Wrists(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Dye2LeftRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Dye2RightRing(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn Unknown40(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Unknown41(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Unknown42(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Unknown43(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn Unknown44(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
}
