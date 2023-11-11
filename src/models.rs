use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::liquidations)]
pub struct NewLiquidation {
    pub symbol: String,
    pub side: String,
    pub quantity: BigDecimal,
}
