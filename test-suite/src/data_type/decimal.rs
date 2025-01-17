use {crate::*, gluesql_core::prelude::Value::*, rust_decimal::prelude::Decimal as D};

test_case!(decimal, async move {
    run!("CREATE TABLE DECIMAL_ITEM (v DECIMAL)");
    run!("INSERT INTO DECIMAL_ITEM VALUES (1)");

    test! {
        sql: "
            SELECT
                v AS a,
                v + 1 AS b,
                1 + v AS c,
                v - 1 AS d,
                1 - v AS e,
                v * 2 AS f,
                2 * v AS g
            FROM DECIMAL_ITEM
                ",
        expected: Ok(select!(
            a       | b       | c   | d       | e   | f       | g;
            Decimal | Decimal | I64 | Decimal | I64 | Decimal | I64;
            D::ONE    D::TWO    2     D::ZERO   0     D::TWO    2
        ))
    };

    test! {
        sql: "
            SELECT
                v / 2 AS h,
                2 / v AS i,
                2 % v AS j,
                v % 2 AS k
            FROM DECIMAL_ITEM
                ",
        expected: Ok(select!(
            h            | i   | j   | k;
            Decimal      | I64 | I64 | Decimal;
            D::new(5, 1)   2     0     D::ONE
        ))
    };

    run!("INSERT INTO DECIMAL_ITEM VALUES (1.5), (2.0), (25.12)");

    test! {
        sql: "SELECT v FROM DECIMAL_ITEM WHERE v > 1.5 AND v <= 25.12",
        expected: Ok(select!(
            v;
            Decimal;
            D::new(2, 0);
            D::new(2512, 2)
        ))
    }
});
