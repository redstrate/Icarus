//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct ENpcDressUpDressSheet {
    sheet: ExcelSheet,
}
impl ENpcDressUpDressSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ENpcDressUpDress")?;
        let sheet = resolver.read_excel_sheet(exh, "ENpcDressUpDress", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ENpcDressUpDressRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(ENpcDressUpDressRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct ENpcDressUpDressRow {
    columns: Vec<ColumnData>,
}
impl ENpcDressUpDressRow {
    pub fn ModelMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn ModelOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn ENpc<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn ModelHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn ModelBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn ModelHands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn ModelLegs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn ModelFeet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Behavior<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown31<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown32<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown33<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn Unknown34<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn DyeMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn Dye2MainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn DyeOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn Dye2OffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn DyeHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn DyeBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn DyeHands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn DyeLegs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn DyeFeet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
    pub fn DyeEars<'a>(&'a self) -> &'a ColumnData {
        &self.columns[53]
    }
    pub fn DyeNeck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[54]
    }
    pub fn DyeWrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[55]
    }
    pub fn DyeLeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn DyeRightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn Dye2Head<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn Dye2Body<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn Dye2Hands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn Dye2Legs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn Dye2Feet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn Dye2Ears<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn Dye2Neck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn Dye2Wrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Dye2LeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn Dye2RightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn Unknown40<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Unknown41<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Unknown42<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Unknown43<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
    pub fn Unknown44<'a>(&'a self) -> &'a ColumnData {
        &self.columns[72]
    }
}
