//! Integrates `ArrayString` with other crates' traits

#[cfg_attr(docs_rs_workaround, doc(cfg(feature = "diesel-traits")))]
#[cfg(feature = "diesel-traits")]
mod diesel_impl {
    pub use crate::prelude::*;

    #[cfg(feature = "std")]
    pub use std::io::Write;

    #[cfg(feature = "std")]
    pub use diesel::serialize::{self, Output, ToSql};

    pub use diesel::backend::Backend;
    pub use diesel::deserialize::{self, FromSql, FromSqlRow, Queryable};
    pub use diesel::{expression::*, prelude::*, query_builder::*, row::Row, sql_types::*};
    use diesel::backend::RawValue;

    impl<const N: usize, ST, DB> FromSql<ST, DB> for ArrayString<N>
        where
            DB: Backend,
            *const str: FromSql<ST, DB>,
    {
        fn from_sql(bytes: RawValue<'_, DB>) -> deserialize::Result<Self> {
            let ptr = <*const str as FromSql<ST, DB>>::from_sql(bytes)?;
            // We know that the pointer impl will never return null
            Ok(Self::from_str_truncate(unsafe { &*ptr }))
        }
    }

    #[cfg_attr(docs_rs_workaround, doc(cfg(feature = "std")))]
    #[cfg(feature = "std")]
    impl<const N: usize, DB> ToSql<VarChar, DB> for ArrayString<N>
        where
            DB: Backend,
            str: ToSql<VarChar, DB>,
    {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
            self.as_str().to_sql(out)
        }
    }

    impl<ST, DB> FromSql<ST, DB> for CacheString
        where
            DB: Backend,
            *const str: FromSql<ST, DB>,
    {
        #[inline]
        fn from_sql(bytes: RawValue<'_, DB>) -> deserialize::Result<Self> {
            Ok(CacheString(FromSql::from_sql(bytes)?))
        }
    }

    #[cfg_attr(docs_rs_workaround, doc(cfg(feature = "std")))]
    #[cfg(feature = "std")]
    impl<DB> ToSql<VarChar, DB> for CacheString
        where
            DB: Backend,
            str: ToSql<VarChar, DB>,
    {
        #[inline]
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
            ToSql::<VarChar, DB>::to_sql(&self.0, out)
        }
    }
}

mod serde_impl {
    pub use crate::prelude::*;

    #[cfg(feature = "serde-traits")]
    pub use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

    #[cfg_attr(docs_rs_workaround, doc(cfg(feature = "serde-traits")))]
    #[cfg(feature = "serde-traits")]
    impl<const N: usize> Serialize for ArrayString<N> {
        #[inline]
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            Serialize::serialize(self.as_str(), ser)
        }
    }

    #[cfg_attr(docs_rs_workaround, doc(cfg(feature = "serde-traits")))]
    #[cfg(feature = "serde-traits")]
    impl<'a, const N: usize> Deserialize<'a> for ArrayString<N> {
        #[inline]
        fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
            <&str>::deserialize(des).map(Self::from_str_truncate)
        }
    }

    #[cfg_attr(docs_rs_workaround, doc(cfg(feature = "serde-traits")))]
    #[cfg(feature = "serde-traits")]
    impl Serialize for CacheString {
        #[inline]
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(ser)
        }
    }

    #[cfg_attr(docs_rs_workaround, doc(cfg(feature = "serde-traits")))]
    #[cfg(feature = "serde-traits")]
    impl<'a> Deserialize<'a> for CacheString {
        #[inline]
        fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
            Ok(CacheString(Deserialize::deserialize(des)?))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_import_braces)]

    use super::{diesel_impl::*, serde_impl::*};
    use crate::ArrayString;

    #[cfg(feature = "serde-traits")]
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct DeriveSerde(pub ArrayString<8>);

    #[cfg(feature = "serde-traits")]
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Derive2Serde(pub CacheString);

    #[test]
    #[cfg(feature = "serde-traits")]
    fn serde_derive_json() {
        let string =
            serde_json::to_string(&DeriveSerde(ArrayString::try_from_str("abcdefg").unwrap()))
                .unwrap();
        let s: DeriveSerde = serde_json::from_str(&string).unwrap();
        assert_eq!(
            s,
            DeriveSerde(ArrayString::try_from_str("abcdefg").unwrap())
        );
    }

    #[test]
    #[cfg(feature = "serde-traits")]
    fn serde_derive2_json() {
        let string = serde_json::to_string(&Derive2Serde(CacheString(
            ArrayString::try_from_str("abcdefg").unwrap(),
        )))
            .unwrap();
        let s: DeriveSerde = serde_json::from_str(&string).unwrap();
        assert_eq!(
            s,
            DeriveSerde(ArrayString::try_from_str("abcdefg").unwrap())
        );
    }

    #[test]
    #[cfg(feature = "serde-traits")]
    fn serde_json() {
        let string =
            serde_json::to_string(&ArrayString::<8>::try_from_str("abcdefg").unwrap())
                .unwrap();
        let s: ArrayString<8> = serde_json::from_str(&string).unwrap();
        assert_eq!(
            s,
            ArrayString::<8>::try_from_str("abcdefg").unwrap()
        );
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    use diesel::{debug_query, insert_into, sqlite, update};
    use diesel::{dsl, mysql, pg};

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    table! {
        derives (name) {
            id -> Integer,
            name -> VarChar,
        }
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    #[derive(AsChangeset, Identifiable, Queryable, QueryableByName, Insertable, Clone, Debug)]
    #[diesel(table_name = derives)]
    struct DeriveDiesel {
        pub id: i32,
        pub name: ArrayString<32>,
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    #[derive(AsChangeset, Identifiable, Queryable, QueryableByName, Insertable, Clone, Debug)]
    #[diesel(table_name = derives)]
    struct Derive2Diesel {
        pub id: i32,
        pub name: CacheString,
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    #[derive(AsChangeset, Identifiable, Queryable, QueryableByName, Insertable, Clone, Debug)]
    #[diesel(table_name = derives)]
    struct Derive3Diesel<'a> {
        pub id: i32,
        pub name: &'a str,
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    #[test]
    fn diesel_derive_query_compare_insert() {
        let array = DeriveDiesel {
            id: 0,
            name: ArrayString::try_from_str("Name1").unwrap(),
        };
        let cache = Derive2Diesel {
            id: 0,
            name: CacheString(ArrayString::try_from_str("Name1").unwrap()),
        };
        let string = Derive3Diesel { id: 0, name: "Name1" };

        let insert_array = insert_into(derives::table).values(&array);
        let insert_cache = insert_into(derives::table).values(&cache);
        let insert_string = insert_into(derives::table).values(&string);
        assert_eq!(
            debug_query::<pg::Pg, _>(&insert_array).to_string(),
            debug_query::<pg::Pg, _>(&insert_string).to_string()
        );
        assert_eq!(
            debug_query::<pg::Pg, _>(&insert_cache).to_string(),
            debug_query::<pg::Pg, _>(&insert_string).to_string()
        );
        assert_eq!(
            debug_query::<mysql::Mysql, _>(&insert_array).to_string(),
            debug_query::<mysql::Mysql, _>(&insert_string).to_string()
        );
        assert_eq!(
            debug_query::<mysql::Mysql, _>(&insert_cache).to_string(),
            debug_query::<mysql::Mysql, _>(&insert_string).to_string()
        );
        assert_eq!(
            debug_query::<sqlite::Sqlite, _>(&insert_array).to_string(),
            debug_query::<sqlite::Sqlite, _>(&insert_string).to_string()
        );
        assert_eq!(
            debug_query::<sqlite::Sqlite, _>(&insert_cache).to_string(),
            debug_query::<sqlite::Sqlite, _>(&insert_string).to_string()
        );
    }

    #[test]
    fn diesel_derive_query_compare_update() {
        let array = DeriveDiesel {
            id: 0,
            name: ArrayString::try_from_str("Name1").unwrap(),
        };
        let cache = Derive2Diesel {
            id: 0,
            name: CacheString(ArrayString::try_from_str("Name1").unwrap()),
        };
        let string = Derive3Diesel { id: 0, name: "Name1" };
        let update_array = update(derives::table).set(&array);
        let update_cache = update(derives::table).set(&array);
        let update_string = update(derives::table).set(&string);
        assert_eq!(
            debug_query::<pg::Pg, _>(&update_array).to_string(),
            debug_query::<pg::Pg, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<pg::Pg, _>(&update_cache).to_string(),
            debug_query::<pg::Pg, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<mysql::Mysql, _>(&update_array).to_string(),
            debug_query::<mysql::Mysql, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<mysql::Mysql, _>(&update_cache).to_string(),
            debug_query::<mysql::Mysql, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<sqlite::Sqlite, _>(&update_array).to_string(),
            debug_query::<sqlite::Sqlite, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<sqlite::Sqlite, _>(&update_cache).to_string(),
            debug_query::<sqlite::Sqlite, _>(&update_string).to_string()
        );

        let update_array = update(derives::table).set(derives::name.eq(&array.name));
        let update_cache = update(derives::table).set(derives::name.eq(&cache.name));
        let update_string = update(derives::table).set(derives::name.eq(&string.name));
        assert_eq!(
            debug_query::<pg::Pg, _>(&update_array).to_string(),
            debug_query::<pg::Pg, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<pg::Pg, _>(&update_cache).to_string(),
            debug_query::<pg::Pg, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<mysql::Mysql, _>(&update_array).to_string(),
            debug_query::<mysql::Mysql, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<mysql::Mysql, _>(&update_cache).to_string(),
            debug_query::<mysql::Mysql, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<sqlite::Sqlite, _>(&update_array).to_string(),
            debug_query::<sqlite::Sqlite, _>(&update_string).to_string()
        );
        assert_eq!(
            debug_query::<sqlite::Sqlite, _>(&update_cache).to_string(),
            debug_query::<sqlite::Sqlite, _>(&update_string).to_string()
        );
    }

    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn diesel_select_query_compiles() {
        let mut conn = pg::PgConnection::establish("").unwrap();
        let select_array: Vec<DeriveDiesel> = derives::table
            .select(derives::all_columns)
            .load(&mut conn)
            .unwrap();
        let select_cache: Vec<Derive2Diesel> = derives::table
            .select(derives::all_columns)
            .load(&mut conn)
            .unwrap();
        assert_eq!(
            select_cache
                .into_iter()
                .map(|d| d.name.to_string())
                .collect::<Vec<_>>(),
            select_array
                .into_iter()
                .map(|d| d.name.to_string())
                .collect::<Vec<_>>()
        );
        let _: std::time::SystemTime = derives::table.select(dsl::now).first(&mut conn).unwrap();
        let _: std::time::SystemTime = derives::table.select(dsl::now).first(&mut conn).unwrap();

        let mut conn = mysql::MysqlConnection::establish("").unwrap();
        let select_array: Vec<DeriveDiesel> = derives::table
            .select(derives::all_columns)
            .load(&mut conn)
            .unwrap();
        let select_cache: Vec<Derive2Diesel> = derives::table
            .select(derives::all_columns)
            .load(&mut conn)
            .unwrap();
        assert_eq!(
            select_array
                .into_iter()
                .map(|d| d.name.to_string())
                .collect::<Vec<_>>(),
            select_cache
                .into_iter()
                .map(|d| d.name.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    #[test]
    fn diesel_derive_query_sqlite() {
        let mut conn = diesel::sqlite::SqliteConnection::establish(":memory:").unwrap();
        let _ = diesel::sql_query("CREATE TABLE derives (id INTEGER, name VARCHAR(32));")
            .execute(&mut conn)
            .unwrap();
        let string = DeriveDiesel {
            id: 0,
            name: ArrayString::try_from_str("Name1").unwrap(),
        };

        let _ = insert_into(derives::table)
            .values(&string)
            .execute(&mut conn)
            .unwrap();

        let queried: DeriveDiesel = derives::table.first(&mut conn).unwrap();
        assert_eq!(queried.name.as_str(), "Name1");
    }

    #[cfg(all(feature = "diesel-traits", feature = "std"))]
    #[test]
    fn diesel_derive2_query_sqlite() {
        let mut conn = diesel::sqlite::SqliteConnection::establish(":memory:").unwrap();
        let _ = diesel::sql_query("CREATE TABLE derives (id INTEGER, name VARCHAR(32));")
            .execute(&mut conn)
            .unwrap();
        let string = Derive2Diesel {
            id: 0,
            name: CacheString(ArrayString::try_from_str("Name1").unwrap()),
        };

        let _ = insert_into(derives::table)
            .values(&string)
            .execute(&mut conn)
            .unwrap();

        let queried: Derive2Diesel = derives::table.first(&mut conn).unwrap();
        assert_eq!(queried.name.as_str(), "Name1");
    }
}
