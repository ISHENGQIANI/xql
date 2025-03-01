use crate::clause;
use crate::expr::Expr;
use crate::ops::and;
use crate::stmt::result::Result;

/// `SELECT` statement builder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Select<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) fields: clause::Select<'a>,
    pub(crate) tables: Option<clause::From<'a>>,
    pub(crate) filter: Option<clause::Where<'a>>,
    pub(crate) groups: Option<clause::GroupBy<'a>>,
    pub(crate) having: Option<clause::Having<'a>>,
    pub(crate) orders: Option<clause::OrderBy<'a>>,
}

stmt_common!(Select);

crate::macros::gen_display!(Select<'_>);

impl<'a> Select<'a> {
    /// Add more column(s) to `SELECT` clause.
    ///
    /// # Examples
    ///
    /// ```
    /// use qians_xql::select;
    ///
    /// let value = "value".to_string();
    /// let query1 = select(("id", &value, 2));
    ///
    /// let query2 = select(["id"]).select([&value]).select([2]);
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn select<F>(mut self, fields: F) -> Select<'a>
    where
        F: Into<clause::Select<'a>>,
    {
        self.fields.0.extend(fields.into().0);
        self
    }

    /// Add more table(s) to `FROM` clause.
    ///
    /// # Examples
    ///
    /// ```
    /// use qians_xql::select;
    /// use qians_xql::eq;
    ///
    /// let query1 = select([("book", "id"), ("author", "id")])
    ///     .from(["book", "author"])
    ///     .filter(eq(("book", "id"), ("author", "id")));
    ///
    /// let query2 = select([("book", "id"), ("author", "id")])
    ///     .from("book")
    ///     .from("author")
    ///     .filter(eq(("book", "id"), ("author", "id")));
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn from<T>(mut self, tables: T) -> Select<'a>
    where
        T: Into<clause::From<'a>>,
    {
        self.tables = match self.tables.take() {
            Some(mut inner) => {
                inner.0.extend(tables.into().0);
                Some(inner)
            }
            None => Some(tables.into()),
        };
        self
    }

    /// Set condition to `WHERE` clause.
    ///
    /// Successive calls combine new condition with previous condition with
    /// [`and`](crate::ops::and).
    ///
    /// # Examples
    ///
    /// ```
    /// use qians_xql::select;
    /// use qians_xql::and;
    /// use qians_xql::ge;
    ///
    /// let query1 = select(["id", "year", "name"])
    ///     .from("book")
    ///     .filter(and(ge("id", 1), ge("year", 1970)));
    ///
    /// let query2 = select(["id", "year", "name"])
    ///     .from("book")
    ///     .filter(ge("id", 1))
    ///     .filter(ge("year", 1970));
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn filter<E>(mut self, expr: E) -> Select<'a>
    where
        E: Into<Expr<'a>>,
    {
        self.filter = match self.filter.take() {
            Some(inner) => Some(and(inner.0, expr.into()).into()),
            None => Some(expr.into().into()),
        };
        self
    }

    /// Add more condition(s) to `GROUP BY` clause.
    ///
    /// # Examples
    ///
    /// ```
    /// use qians_xql::select;
    /// use qians_xql::eq;
    ///
    /// let query1 = select(["id", "title"])
    ///     .from("book")
    ///     .group_by("id")
    ///     .group_by("title");
    ///
    /// let query2 = select(["id", "title"])
    ///     .from("book")
    ///     .group_by(["id", "title"]);
    ///
    /// ```
    pub fn group_by<G>(mut self, groups: G) -> Select<'a>
    where
        G: Into<clause::GroupBy<'a>>,
    {
        self.groups = match self.groups.take() {
            Some(mut inner) => {
                inner.0.extend(groups.into().0);
                Some(inner)
            }
            None => Some(groups.into()),
        };
        self
    }

    /// Set condition to `HAVING` clause.
    ///
    /// Successive calls combine new condition with previous condition with
    /// [`and`](crate::ops::and).
    ///
    /// # Examples
    ///
    /// ```
    /// use qians_xql::select;
    /// use qians_xql::and;
    /// use qians_xql::ge;
    ///
    /// let query1 = select(["id", "year", "name"])
    ///     .from("book")
    ///     .having(and(ge("id", 1), ge("year", 1970)));
    ///
    /// let query2 = select(["id", "year", "name"])
    ///     .from("book")
    ///     .having(ge("id", 1))
    ///     .having(ge("year", 1970));
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn having<E>(mut self, expr: E) -> Select<'a>
    where
        E: Into<Expr<'a>>,
    {
        self.having = match self.having.take() {
            Some(inner) => Some(and(inner.0, expr.into()).into()),
            None => Some(expr.into().into()),
        };
        self
    }

    pub fn order_by<O>(mut self, orders: O) -> Select<'a>
    where
        O: Into<clause::OrderBy<'a>>,
    {
        self.orders = match self.orders.take() {
            Some(mut inner) => {
                inner.0.extend(orders.into().0);
                Some(inner)
            }
            None => Some(orders.into()),
        };
        self
    }

    pub fn pagination(self, limit: u32, offset: u32) -> Result<'a> {
        Result {
            data: self.into(),
            limit: Some(clause::Limit(limit)),
            offset: Some(clause::Offset(offset)),
            ..Default::default()
        }
    }

    pub fn limit(self, limit: u32) -> Result<'a> {
        Result {
            data: self.into(),
            limit: Some(clause::Limit(limit)),
            ..Default::default()
        }
    }

    pub fn offset(self, offset: u32) -> Result<'a> {
        Result {
            data: self.into(),
            offset: Some(clause::Offset(offset)),
            ..Default::default()
        }
    }
}

#[test]
#[cfg(test)]
fn test() {
    use crate::func;
    use crate::ops;
    use crate::stmt::select;

    let mut query = select([("data", "id"), ("data", "value")]);
    query = query.select([
        func::count("id"),
        func::max("age"),
        func::min("age"),
        func::avg("age"),
    ]);

    let name = &"name".to_string();
    query = query
        .from(("public", "data"))
        .from(func("unnest", [("data", "value")]))
        .filter(and(
            ops::eq(("data", "id"), 1),
            ops::eq(("data", "name"), name),
        ))
        .group_by([("data", "id")])
        .having(true)
        .order_by([ops::desc(("data", "id"))]);
    let expect = "SELECT data.id, data.value, COUNT(id), MAX(age), MIN(age), AVG(age) FROM public.data, unnest(data.value) WHERE data.id = 1 AND data.name = \'name\' GROUP BY data.id HAVING true ORDER BY data.id DESC";
    assert_eq!(query.to_string(), expect);
}
