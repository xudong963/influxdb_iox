use std::ops::Deref;

use data_types::delete_predicate::{DeleteExpr, Op, Scalar};
use generated_types::influxdata::iox::predicate::v1 as proto;
use snafu::{OptionExt, ResultExt, Snafu};

pub(crate) fn expr_to_df(expr: DeleteExpr) -> datafusion::logical_plan::Expr {
    use datafusion::logical_plan::Expr;

    let column = datafusion::logical_plan::Column {
        relation: None,
        name: expr.column,
    };

    Expr::BinaryExpr {
        left: Box::new(Expr::Column(column)),
        op: op_to_df(expr.op),
        right: Box::new(Expr::Literal(scalar_to_df(expr.scalar))),
    }
}

#[derive(Debug, Snafu)]
pub enum ProtoToExprError {
    #[snafu(display("cannot deserialize operator: {}", source))]
    CannotDeserializeOperator {
        source: crate::delete_expr::ProtoToOpError,
    },

    #[snafu(display("illegal operator enum value: {}", value))]
    IllegalOperatorEnumValue { value: i32 },

    #[snafu(display("missing scalar"))]
    MissingScalar,

    #[snafu(display("cannot deserialize scalar: {}", source))]
    CannotDeserializeScalar {
        source: crate::delete_expr::ProtoToScalarError,
    },
}

pub(crate) fn proto_to_expr(expr: proto::Expr) -> Result<DeleteExpr, ProtoToExprError> {
    let op = proto_to_op(
        proto::Op::from_i32(expr.op).context(IllegalOperatorEnumValue { value: expr.op })?,
    )
    .context(CannotDeserializeOperator)?;

    let scalar = proto_to_scalar(expr.scalar.clone().context(MissingScalar)?)
        .context(CannotDeserializeScalar)?;

    Ok(DeleteExpr {
        column: expr.column,
        op,
        scalar,
    })
}

#[derive(Debug, Snafu)]
pub enum DataFusionToExprError {
    #[snafu(display("unsupported expression: {:?}", expr))]
    UnsupportedExpression {
        expr: datafusion::logical_plan::Expr,
    },

    #[snafu(display("unsupported operants: left {:?}; right {:?}", left, right))]
    UnsupportedOperants {
        left: datafusion::logical_plan::Expr,
        right: datafusion::logical_plan::Expr,
    },

    #[snafu(display("cannot convert datafusion operator: {}", source))]
    CannotConvertDataFusionOperator {
        source: crate::delete_expr::DataFusionToOpError,
    },

    #[snafu(display("cannot convert datafusion scalar value: {}", source))]
    CannotConvertDataFusionScalarValue {
        source: crate::delete_expr::DataFusionToScalarError,
    },
}

pub(crate) fn df_to_expr(
    expr: datafusion::logical_plan::Expr,
) -> Result<DeleteExpr, DataFusionToExprError> {
    match expr {
        datafusion::logical_plan::Expr::BinaryExpr { left, op, right } => {
            let (column, scalar) = match (left.deref(), right.deref()) {
                // The delete predicate parser currently only supports `<column><op><value>`, not `<value><op><column>`,
                // however this could can easily be extended to support the latter case as well.
                (
                    datafusion::logical_plan::Expr::Column(column),
                    datafusion::logical_plan::Expr::Literal(value),
                ) => {
                    let column = column.name.clone();

                    let scalar =
                        df_to_scalar(value.clone()).context(CannotConvertDataFusionScalarValue)?;

                    (column, scalar)
                }
                (other_left, other_right) => {
                    return Err(DataFusionToExprError::UnsupportedOperants {
                        left: other_left.clone(),
                        right: other_right.clone(),
                    });
                }
            };

            let op = df_to_op(op).context(CannotConvertDataFusionOperator)?;

            Ok(DeleteExpr { column, op, scalar })
        }
        other => Err(DataFusionToExprError::UnsupportedExpression { expr: other }),
    }
}

pub(crate) fn expr_to_proto(expr: DeleteExpr) -> proto::Expr {
    let op = op_to_proto(expr.op);

    proto::Expr {
        column: expr.column,
        op: op.into(),
        scalar: Some(scalar_to_proto(expr.scalar)),
    }
}

pub(crate) fn op_to_df(op: Op) -> datafusion::logical_plan::Operator {
    match op {
        Op::Eq => datafusion::logical_plan::Operator::Eq,
        Op::Ne => datafusion::logical_plan::Operator::NotEq,
    }
}

#[derive(Debug, Snafu)]
#[allow(missing_copy_implementations)] // allow extensions
pub enum DataFusionToOpError {
    #[snafu(display("unsupported operator: {:?}", op))]
    UnsupportedOperator {
        op: datafusion::logical_plan::Operator,
    },
}

pub(crate) fn df_to_op(op: datafusion::logical_plan::Operator) -> Result<Op, DataFusionToOpError> {
    match op {
        datafusion::logical_plan::Operator::Eq => Ok(Op::Eq),
        datafusion::logical_plan::Operator::NotEq => Ok(Op::Ne),
        other => Err(DataFusionToOpError::UnsupportedOperator { op: other }),
    }
}

pub(crate) fn op_to_proto(op: Op) -> proto::Op {
    match op {
        Op::Eq => proto::Op::Eq,
        Op::Ne => proto::Op::Ne,
    }
}

#[derive(Debug, Snafu)]
#[allow(missing_copy_implementations)] // allow extensions
pub enum ProtoToOpError {
    #[snafu(display("unspecified operator"))]
    UnspecifiedOperator,
}

pub(crate) fn proto_to_op(op: proto::Op) -> Result<Op, ProtoToOpError> {
    match op {
        proto::Op::Unspecified => Err(ProtoToOpError::UnspecifiedOperator),
        proto::Op::Eq => Ok(Op::Eq),
        proto::Op::Ne => Ok(Op::Ne),
    }
}

pub(crate) fn scalar_to_df(scalar: Scalar) -> datafusion::scalar::ScalarValue {
    use datafusion::scalar::ScalarValue;
    match scalar {
        Scalar::Bool(value) => ScalarValue::Boolean(Some(value)),
        Scalar::I64(value) => ScalarValue::Int64(Some(value)),
        Scalar::F64(value) => ScalarValue::Float64(Some(value.into())),
        Scalar::String(value) => ScalarValue::Utf8(Some(value)),
    }
}

#[derive(Debug, Snafu)]
#[allow(missing_copy_implementations)] // allow extensions
pub enum ProtoToScalarError {
    #[snafu(display("missing scalar value"))]
    MissingScalarValue,
}

pub(crate) fn proto_to_scalar(scalar: proto::Scalar) -> Result<Scalar, ProtoToScalarError> {
    match scalar.value.context(MissingScalarValue)? {
        proto::scalar::Value::ValueBool(value) => Ok(Scalar::Bool(value)),
        proto::scalar::Value::ValueI64(value) => Ok(Scalar::I64(value)),
        proto::scalar::Value::ValueF64(value) => Ok(Scalar::F64(value.into())),
        proto::scalar::Value::ValueString(value) => Ok(Scalar::String(value)),
    }
}

#[derive(Debug, Snafu)]
pub enum DataFusionToScalarError {
    #[snafu(display("unsupported scalar value: {:?}", value))]
    UnsupportedScalarValue {
        value: datafusion::scalar::ScalarValue,
    },
}

pub(crate) fn df_to_scalar(
    scalar: datafusion::scalar::ScalarValue,
) -> Result<Scalar, DataFusionToScalarError> {
    use datafusion::scalar::ScalarValue;
    match scalar {
        ScalarValue::Utf8(Some(value)) => Ok(Scalar::String(value)),
        ScalarValue::Int64(Some(value)) => Ok(Scalar::I64(value)),
        ScalarValue::Float64(Some(value)) => Ok(Scalar::F64(value.into())),
        ScalarValue::Boolean(Some(value)) => Ok(Scalar::Bool(value)),
        other => Err(DataFusionToScalarError::UnsupportedScalarValue { value: other }),
    }
}

pub(crate) fn scalar_to_proto(scalar: Scalar) -> proto::Scalar {
    match scalar {
        Scalar::Bool(value) => proto::Scalar {
            value: Some(proto::scalar::Value::ValueBool(value)),
        },
        Scalar::I64(value) => proto::Scalar {
            value: Some(proto::scalar::Value::ValueI64(value)),
        },
        Scalar::F64(value) => proto::Scalar {
            value: Some(proto::scalar::Value::ValueF64(value.into())),
        },
        Scalar::String(value) => proto::Scalar {
            value: Some(proto::scalar::Value::ValueString(value)),
        },
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::assert_contains;

    use super::*;

    #[test]
    fn test_roundtrips() {
        assert_expr_works(
            DeleteExpr {
                column: "foo".to_string(),
                op: Op::Eq,
                scalar: Scalar::Bool(true),
            },
            "foo=true",
        );
        assert_expr_works(
            DeleteExpr {
                column: "bar".to_string(),
                op: Op::Ne,
                scalar: Scalar::I64(-1),
            },
            "bar!=-1",
        );
        assert_expr_works(
            DeleteExpr {
                column: "baz".to_string(),
                op: Op::Eq,
                scalar: Scalar::F64((-1.1).into()),
            },
            "baz=-1.1",
        );
        assert_expr_works(
            DeleteExpr {
                column: "col".to_string(),
                op: Op::Eq,
                scalar: Scalar::String("foo".to_string()),
            },
            "col='foo'",
        );
    }

    fn assert_expr_works(expr: DeleteExpr, display: &str) {
        let df_expr = expr_to_df(expr.clone());
        let expr2 = df_to_expr(df_expr).unwrap();
        assert_eq!(expr2, expr);

        let proto_expr: proto::Expr = expr_to_proto(expr.clone());
        let expr3 = proto_to_expr(proto_expr).unwrap();
        assert_eq!(expr3, expr);

        assert_eq!(expr.to_string(), display);
    }

    #[test]
    fn test_unsupported_expression() {
        let expr = datafusion::logical_plan::Expr::Not(Box::new(
            datafusion::logical_plan::Expr::BinaryExpr {
                left: Box::new(datafusion::logical_plan::Expr::Column(
                    datafusion::logical_plan::Column {
                        relation: None,
                        name: "foo".to_string(),
                    },
                )),
                op: datafusion::logical_plan::Operator::Eq,
                right: Box::new(datafusion::logical_plan::Expr::Literal(
                    datafusion::scalar::ScalarValue::Utf8(Some("x".to_string())),
                )),
            },
        ));
        let res = df_to_expr(expr);
        assert_contains!(res.unwrap_err().to_string(), "unsupported expression:");
    }

    #[test]
    fn test_unsupported_operants() {
        let expr = datafusion::logical_plan::Expr::BinaryExpr {
            left: Box::new(datafusion::logical_plan::Expr::Column(
                datafusion::logical_plan::Column {
                    relation: None,
                    name: "foo".to_string(),
                },
            )),
            op: datafusion::logical_plan::Operator::Eq,
            right: Box::new(datafusion::logical_plan::Expr::Column(
                datafusion::logical_plan::Column {
                    relation: None,
                    name: "bar".to_string(),
                },
            )),
        };
        let res = df_to_expr(expr);
        assert_contains!(res.unwrap_err().to_string(), "unsupported operants:");
    }

    #[test]
    fn test_unsupported_scalar_value() {
        let scalar = datafusion::scalar::ScalarValue::List(
            Some(Box::new(vec![])),
            Box::new(arrow::datatypes::DataType::Float64),
        );
        let res = df_to_scalar(scalar);
        assert_contains!(res.unwrap_err().to_string(), "unsupported scalar value:");
    }

    #[test]
    fn test_unsupported_scalar_value_in_expr() {
        let expr = datafusion::logical_plan::Expr::BinaryExpr {
            left: Box::new(datafusion::logical_plan::Expr::Column(
                datafusion::logical_plan::Column {
                    relation: None,
                    name: "foo".to_string(),
                },
            )),
            op: datafusion::logical_plan::Operator::Eq,
            right: Box::new(datafusion::logical_plan::Expr::Literal(
                datafusion::scalar::ScalarValue::List(
                    Some(Box::new(vec![])),
                    Box::new(arrow::datatypes::DataType::Float64),
                ),
            )),
        };
        let res = df_to_expr(expr);
        assert_contains!(res.unwrap_err().to_string(), "unsupported scalar value:");
    }

    #[test]
    fn test_unsupported_operator() {
        let res = df_to_op(datafusion::logical_plan::Operator::Like);
        assert_contains!(res.unwrap_err().to_string(), "unsupported operator:");
    }

    #[test]
    fn test_unsupported_operator_in_expr() {
        let expr = datafusion::logical_plan::Expr::BinaryExpr {
            left: Box::new(datafusion::logical_plan::Expr::Column(
                datafusion::logical_plan::Column {
                    relation: None,
                    name: "foo".to_string(),
                },
            )),
            op: datafusion::logical_plan::Operator::Like,
            right: Box::new(datafusion::logical_plan::Expr::Literal(
                datafusion::scalar::ScalarValue::Utf8(Some("x".to_string())),
            )),
        };
        let res = df_to_expr(expr);
        assert_contains!(res.unwrap_err().to_string(), "unsupported operator:");
    }
}
