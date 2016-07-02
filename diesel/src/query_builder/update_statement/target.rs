use associations::Identifiable;
use query_dsl::FindDsl;
use query_source::Table;

#[doc(hidden)]
#[derive(Debug)]
pub struct UpdateTarget<Table, WhereClause> {
    pub table: Table,
    pub where_clause: Option<WhereClause>,
}

#[doc(hidden)]
pub trait IntoUpdateTarget {
    type Table: Table;
    type WhereClause;

    fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause>;
}

impl<'a, T: Identifiable, V> IntoUpdateTarget for &'a T where
    <T::Table as FindDsl<T::Id>>::Output: IntoUpdateTarget<Table=T::Table, WhereClause=V>,
{
    type Table = T::Table;
    type WhereClause = V;

    fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
        T::table().find(self.id()).into_update_target()
    }
}
