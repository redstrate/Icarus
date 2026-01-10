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
pub struct BNpcPartsSheet {
    sheet: Sheet,
}
impl BNpcPartsSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcParts")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcParts", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcPartsRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcPartsRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BNpcPartsSheet {
    type Row = BNpcPartsRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a BNpcPartsSheet {
    type Item = (u32, Vec<(u16, BNpcPartsRow)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcPartsSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcPartsSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BNpcPartsRow {
    columns: Vec<Field>,
}
impl BNpcPartsRow {
    pub fn X1<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn X2<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn X3<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn X4<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn X5<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Y1<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Y2<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Y3<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Y4<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Y5<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Z1<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Z2<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Z3<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Z4<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Z5<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Scale1<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Scale2<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn Scale4<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn Scale5<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn BNpcBase1<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn BNpcBase2<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn BNpcBase3<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn BNpcBase4<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn BNpcBase5<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn Scale3<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn PartSlot1<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn PartSlot2<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn PartSlot3<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn PartSlot4<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn PartSlot5<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a Field {
        &self.columns[47]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a Field {
        &self.columns[48]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a Field {
        &self.columns[49]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a Field {
        &self.columns[50]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a Field {
        &self.columns[51]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a Field {
        &self.columns[52]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a Field {
        &self.columns[53]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a Field {
        &self.columns[54]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a Field {
        &self.columns[55]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn Unknown31<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn Unknown32<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
    pub fn Unknown33<'a>(&'a self) -> &'a Field {
        &self.columns[63]
    }
    pub fn Unknown34<'a>(&'a self) -> &'a Field {
        &self.columns[64]
    }
    pub fn Unknown35<'a>(&'a self) -> &'a Field {
        &self.columns[65]
    }
    pub fn Unknown36<'a>(&'a self) -> &'a Field {
        &self.columns[66]
    }
}
