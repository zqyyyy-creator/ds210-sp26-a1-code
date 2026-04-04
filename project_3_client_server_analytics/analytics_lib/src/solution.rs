use std::collections::HashMap;
use std::hash::Hash;
use std::result;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn row_matches(dataset: &Dataset, row: &Row, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(column_name,value) => {
            let column_index = dataset.column_index(column_name);
            return row.get_value(column_index) == value;
        },
        Condition::Not(inner) => {
            return !row_matches(dataset, row, inner)
        },
        Condition::And(left, right) => {
            return row_matches(dataset, row, left) && row_matches(dataset, row, right);
        },
        Condition::Or(left, right) => {
            return row_matches(dataset, row, left) || row_matches(dataset, row, right);
        }

    }

}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if row_matches(dataset, row, filter) {
            result.add_row(row.clone());
        }
    }
    return result;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let column_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    let mut result = HashMap::new();
    for row in dataset.into_iter() {
        let key = row.get_value(column_index).clone();
        if !result.contains_key(&key) {
            result.insert(key.clone(), Dataset::new(columns.clone()));
        }
        result.get_mut(&key).unwrap().add_row(row);
    }
    return result;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}