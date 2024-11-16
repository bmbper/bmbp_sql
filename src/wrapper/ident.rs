use serde_json::to_string;

pub trait RdbcModel {
    fn schema() -> String {
        "".to_string()
    }
    fn table() -> String {
        "".to_string()
    }
    fn table_alias() -> String {
        "".to_string()
    }
    fn columns() -> Vec<dyn RdbcColumnIdent> {
        vec![]
    }
}
pub trait RdbcTableIdent {
    fn schema(&self) -> String {
        "".to_string()
    }
    fn table(&self) -> String {
        "".to_string()
    }
    fn table_alias(&self) -> String {
        "".to_string()
    }
}
pub trait RdbcColumnIdent: RdbcTableIdent {
    fn column(&self) -> String {
        "".to_string()
    }
    fn column_alias(&self) -> String {
        "".to_string()
    }
}

impl<T> RdbcTableIdent for T
where
    T: ToString,
{
    fn table(&self) -> String {
        self.to_string()
    }
}
impl<T> RdbcTableIdent for Option<T>
where
    T: ToString,
{
    fn table(&self) -> String {
        match self {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T> RdbcTableIdent for Option<&T>
where
    T: ToString,
{
    fn table(&self) -> String {
        match self {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}

impl<T, A> RdbcTableIdent for (T, A)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        self.0.to_string()
    }

    fn table_alias(&self) -> String {
        self.1.to_string()
    }
}
impl<T, A> RdbcTableIdent for (T, &A)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        self.0.to_string()
    }

    fn table_alias(&self) -> String {
        self.1.to_string()
    }
}
impl<T, A> RdbcTableIdent for (&T, A)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        self.0.to_string()
    }

    fn table_alias(&self) -> String {
        self.1.to_string()
    }
}
impl<T, A> RdbcTableIdent for (&T, &A)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        self.0.to_string()
    }

    fn table_alias(&self) -> String {
        self.1.to_string()
    }
}

impl<T, A> RdbcTableIdent for (Option<T>, Option<A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T, A> RdbcTableIdent for (Option<T>, Option<&A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T, A> RdbcTableIdent for (Option<&T>, Option<A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T, A> RdbcTableIdent for (Option<&T>, Option<&A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}

impl<T, A> RdbcTableIdent for (&Option<T>, &Option<A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T, A> RdbcTableIdent for (&Option<T>, Option<&A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T, A> RdbcTableIdent for (Option<&T>, &Option<A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}
impl<T, A> RdbcTableIdent for (&Option<&T>, &Option<&A>)
where
    T: ToString,
    A: ToString,
{
    fn table(&self) -> String {
        match self.0.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    fn table_alias(&self) -> String {
        match self.1.as_ref() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}

impl<S, T, A> RdbcTableIdent for (S, T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (S, &T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (S, T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, &T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (S, &T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, &T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}

impl<S, T, A> RdbcTableIdent for (Option<S>, T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<&S>, T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<S>, &T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<S>, T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<&S>, &T, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<S>, &T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<&S>, T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (Option<&S>, &T, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}

impl<S, T, A> RdbcTableIdent for (S, Option<T>, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, Option<T>, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (S, Option<T>, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, Option<&T>, A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, Option<T>, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}
impl<S, T, A> RdbcTableIdent for (&S, Option<&T>, &A)
where
    S: ToString,
    T: ToString,
    A: ToString,
{
    fn schema(&self) -> String {
        self.0.to_string()
    }
    fn table(&self) -> String {
        self.1.to_string()
    }

    fn table_alias(&self) -> String {
        self.2.to_string()
    }
}

impl<C> RdbcColumnIdent for C
where
    C: ToString,
{
    fn column(&self) -> String {
        self.to_string()
    }
}
impl<C> RdbcColumnIdent for Option<C>
where
    C: ToString,
{
    fn column(&self) -> String {
        self.to_string()
    }
}
impl<C> RdbcColumnIdent for &C
where
    C: ToString,
{
    fn column(&self) -> String {
        self.to_string()
    }
}

impl<C> RdbcColumnIdent for Option<&C>
where
    C: ToString,
{
    fn column(&self) -> String {
        self.to_string()
    }
}
