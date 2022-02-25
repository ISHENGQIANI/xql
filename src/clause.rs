use crate::expr::Expr;
use crate::item::Cte;
use crate::item::Field;
use crate::item::Ident;
use crate::item::Order;
use crate::item::Row;
use crate::item::Table;
use crate::item::TableRef;
use crate::utils::join;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct With<'a>(pub(crate) bool, pub(crate) Vec<Cte<'a>>);

impl std::fmt::Display for With<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 {
            write!(f, "WITH RECURSIVE {}", join(&self.1, ", "))
        } else {
            write!(f, "WITH {}", join(&self.1, ", "))
        }
    }
}

impl<'a, T> std::convert::From<Vec<T>> for With<'a>
where
    T: Into<Cte<'a>>,
{
    #[inline]
    fn from(val: Vec<T>) -> Self {
        With(false, val.into_iter().map(Into::into).collect())
    }
}

impl<'a, T, const N: usize> std::convert::From<[T; N]> for With<'a>
where
    T: Into<Cte<'a>>,
{
    #[inline]
    fn from(val: [T; N]) -> Self {
        With(false, val.into_iter().map(Into::into).collect())
    }
}

/// Represent a `SELECT` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Select<'a>(pub(crate) Vec<Field<'a>>);

impl std::fmt::Display for Select<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SELECT {}", join(&self.0, ", "))
    }
}

crate::macros::gen_impl_from_arr!(Select[Field]<'a>);
crate::macros::gen_impl_from_vec!(Select[Field]<'a>);
crate::macros::gen_impl_from_tup!(Select[Field]<'a>);

/// Represent a `FROM` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct From<'a>(pub(crate) Vec<Table<'a>>);

impl std::fmt::Display for From<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FROM {}", join(&self.0, ", "))
    }
}

impl<'a, T> std::convert::From<T> for From<'a>
where
    T: Into<Table<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        From(vec![val.into()])
    }
}

crate::macros::gen_impl_from_arr!(From[Table]<'a>);
crate::macros::gen_impl_from_vec!(From[Table]<'a>);

/// Represent a `WHERE` clause.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Where<'a>(pub(crate) Expr<'a>);

impl std::fmt::Display for Where<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WHERE {}", self.0)
    }
}

impl<'a, E> std::convert::From<E> for Where<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(expr: E) -> Self {
        Where(expr.into())
    }
}

/// Represent a `GROUP BY` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct GroupBy<'a>(pub(crate) Vec<Expr<'a>>);

impl std::fmt::Display for GroupBy<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GROUP BY {}", join(&self.0, ", "))
    }
}

impl<'a, T> std::convert::From<T> for GroupBy<'a>
where
    T: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        GroupBy(vec![val.into()])
    }
}

crate::macros::gen_impl_from_arr!(GroupBy[Expr]<'a>);
crate::macros::gen_impl_from_vec!(GroupBy[Expr]<'a>);

/// Represent a `HAVING` clause.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Having<'a>(pub(crate) Expr<'a>);

impl std::fmt::Display for Having<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HAVING {}", self.0)
    }
}

impl<'a, E> std::convert::From<E> for Having<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(expr: E) -> Self {
        Having(expr.into())
    }
}

/// Represent a `ORDER BY` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct OrderBy<'a>(pub(crate) Vec<Order<'a>>);

impl std::fmt::Display for OrderBy<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ORDER BY {}", join(&self.0, ", "))
    }
}

impl<'a, T> std::convert::From<T> for OrderBy<'a>
where
    T: Into<Order<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        OrderBy(vec![val.into()])
    }
}

crate::macros::gen_impl_from_arr!(OrderBy[Order]<'a>);
crate::macros::gen_impl_from_vec!(OrderBy[Order]<'a>);

/// Represent a `INSERT` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Insert<'a>(pub(crate) TableRef<'a>, pub(crate) Vec<Ident<'a>>);

impl std::fmt::Display for Insert<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "INSERT INTO {}", self.0)?;
        if !self.1.is_empty() {
            write!(f, "({})", join(&self.1, ", "))?;
        };
        Ok(())
    }
}

/// Represent a `VALUES` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Values<'a>(pub(crate) Vec<Row<'a>>);

impl std::fmt::Display for Values<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VALUES {}", join(&self.0, ", "))
    }
}

crate::macros::gen_impl_from_arr!(Values[Row]<'a>);
crate::macros::gen_impl_from_vec!(Values[Row]<'a>);
crate::macros::gen_impl_from_tup!(Values[Row]<'a>);

/// Represent a `VALUES` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Returning<'a>(pub(crate) Vec<Field<'a>>);

impl std::fmt::Display for Returning<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RETURNING {}", join(&self.0, ", "))
    }
}

crate::macros::gen_impl_from_arr!(Returning[Field]<'a>);
crate::macros::gen_impl_from_vec!(Returning[Field]<'a>);
crate::macros::gen_impl_from_tup!(Returning[Field]<'a>);

/// Represent a `DELETE` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Delete<'a>(pub(crate) TableRef<'a>);

impl std::fmt::Display for Delete<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DELETE FROM {}", self.0)
    }
}

impl<'a, T> std::convert::From<T> for Delete<'a>
where
    T: Into<TableRef<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        Delete(val.into())
    }
}

/// Represent a `UPDATE` clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Update<'a>(pub(crate) TableRef<'a>);

impl std::fmt::Display for Update<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UPDATE {}", self.0)
    }
}

impl<'a, T> std::convert::From<T> for Update<'a>
where
    T: Into<TableRef<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        Update(val.into())
    }
}

/// Represent a `SET` clause inside `UPDATE` statement.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Set<'a>(pub(crate) Vec<(Ident<'a>, Expr<'a>)>);

impl std::fmt::Display for Set<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SET {}",
            join(
                self.0.iter().map(|(col, val)| format!("{col} = {val}")),
                ", "
            )
        )
    }
}

impl<'a, C, E> std::convert::From<Vec<(C, E)>> for Set<'a>
where
    C: Into<Ident<'a>>,
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: Vec<(C, E)>) -> Self {
        Set(val
            .into_iter()
            .map(|(col, exp)| (col.into(), exp.into()))
            .collect())
    }
}

impl<'a, C, E, const N: usize> std::convert::From<[(C, E); N]> for Set<'a>
where
    C: Into<Ident<'a>>,
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: [(C, E); N]) -> Self {
        Set(val
            .into_iter()
            .map(|(col, exp)| (col.into(), exp.into()))
            .collect())
    }
}

/// Represent a `LIMIT` clause.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Limit<'a>(pub(crate) Expr<'a>);

impl std::fmt::Display for Limit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LIMIT {}", self.0)
    }
}

impl<'a, E> std::convert::From<E> for Limit<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(expr: E) -> Self {
        Limit(expr.into())
    }
}

/// Represent a `OFFSET` clause.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Offset<'a>(pub(crate) Expr<'a>);

impl std::fmt::Display for Offset<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OFFSET {}", self.0)
    }
}

impl<'a, E> std::convert::From<E> for Offset<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(expr: E) -> Self {
        Offset(expr.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::clause::From;
    use crate::clause::GroupBy;
    use crate::clause::Having;
    use crate::clause::OrderBy;
    use crate::clause::Select;
    use crate::clause::Where;
    use crate::expr::Expr;
    use crate::item::ColumnRef;
    use crate::item::Field;
    use crate::item::Ident;
    use crate::item::Order;
    use crate::item::Sort;
    use crate::item::Table;
    use crate::item::TableRef;
    use crate::table_expr::TableExpr;
    use crate::value::Value;

    #[test]
    fn select() {
        let mut clause: Select = ["id"].into();
        assert_eq!(
            clause,
            Select(vec![Field {
                alias: None,
                expr: Expr::Column(ColumnRef::Column(Ident("id")))
            }])
        );
        assert_eq!(clause.to_string(), "SELECT id");

        clause.0.extend([("user", "name").into()]);
        assert_eq!(
            clause,
            Select(vec![
                Field {
                    alias: None,
                    expr: Expr::Column(ColumnRef::Column(Ident("id"))),
                },
                Field {
                    alias: None,
                    expr: Expr::Column(ColumnRef::TableColumn(Ident("user"), Ident("name"),)),
                },
            ])
        );
        assert_eq!(clause.to_string(), "SELECT id, user.name");
    }

    #[test]
    fn from() {
        let mut clause: From = ["user"].into();
        assert_eq!(
            clause,
            From(vec![Table {
                alias: None,
                table: TableExpr::TableRef(TableRef::Table(Ident("user"))),
            }])
        );
        assert_eq!(clause.to_string(), "FROM user");

        clause.0.extend([("public", "contact").into()]);
        assert_eq!(
            clause,
            From(vec![
                Table {
                    alias: None,
                    table: TableExpr::TableRef(TableRef::Table(Ident("user")))
                },
                Table {
                    alias: None,
                    table: TableExpr::TableRef(TableRef::SchemaTable(
                        Ident("public"),
                        Ident("contact")
                    ))
                },
            ])
        );
        assert_eq!(clause.to_string(), "FROM user, public.contact")
    }

    #[test]
    fn where_() {
        let clause: Where = true.into();
        assert_eq!(clause, Where(Expr::Literal(Value::Bool(true))));
        assert_eq!(clause.to_string(), "WHERE true");
    }

    #[test]
    fn group_by() {
        let mut clause: GroupBy = ["id"].into();
        assert_eq!(
            clause,
            GroupBy(vec![Expr::Column(ColumnRef::Column(Ident("id")))])
        );
        assert_eq!(clause.to_string(), "GROUP BY id");

        clause.0.extend([("user", "name").into()]);
        assert_eq!(
            clause,
            GroupBy(vec![
                Expr::Column(ColumnRef::Column(Ident("id"))),
                Expr::Column(ColumnRef::TableColumn(Ident("user"), Ident("name")))
            ])
        );
        assert_eq!(clause.to_string(), "GROUP BY id, user.name")
    }

    #[test]
    fn having() {
        let clause: Having = true.into();
        assert_eq!(clause, Having(Expr::Literal(Value::Bool(true))));
        assert_eq!(clause.to_string(), "HAVING true");
    }

    #[test]
    fn order_by() {
        let mut clause: OrderBy = ["id"].into();
        assert_eq!(
            clause,
            OrderBy(vec![Order(
                Expr::Column(ColumnRef::Column(Ident("id"))),
                None
            )]),
        );
        assert_eq!(clause.to_string(), "ORDER BY id");

        clause
            .0
            .extend([(("user", "name"), Some(Sort::Desc)).into()]);

        assert_eq!(
            clause,
            OrderBy(vec![
                Order(Expr::Column(ColumnRef::Column(Ident("id"))), None),
                Order(
                    Expr::Column(ColumnRef::TableColumn(Ident("user"), Ident("name"),)),
                    Some(Sort::Desc)
                )
            ]),
        );
        assert_eq!(clause.to_string(), "ORDER BY id, user.name DESC")
    }
}
