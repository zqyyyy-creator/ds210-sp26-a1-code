extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::Query;
use analytics_lib::query::{Aggregation,Condition};
use analytics_lib::dataset::Value;
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    let (left, right) = input.split_once("GROUP BY").unwrap();
    let condition_text = left.strip_prefix("FILTER").unwrap().trim();
    let right = right.trim();
    let mut parts = right.splitn(2, " ");
    let group_by_text = parts.next().unwrap();
    let aggregation_text = parts.next().unwrap();
    let group_by = parse_group_by(group_by_text);
    let aggregation = parse_aggregation(aggregation_text);
    let condition = parse_condition(condition_text);
    return Query::new(condition, group_by, aggregation);
}

fn parse_group_by(text: &str) -> String {
    return text.trim().to_string();
}

fn parse_aggregation(text: &str) -> Aggregation {
    let parts = text.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        panic!("Invalid aggregation format: {}", text);
    }
    match parts[0] {
        "COUNT" => Aggregation::Count(parts[1].to_string()),
        "SUM" => Aggregation::Sum(parts[1].to_string()),
        "AVERAGE" => Aggregation::Average(parts[1].to_string()),
        _ => panic!("Unknown aggregation type: {}", text),
    }
}

fn parse_condition(input: &str) -> Condition {
    let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        panic!("only simple == conditions are supported");
    }

    let column = parts[0].trim().to_string();
    let raw_value = parts[1].trim();

    let value = if raw_value.starts_with('"') && raw_value.ends_with('"') {
        Value::String(raw_value.trim_matches('"').to_string())
    } else if let Ok(num) = raw_value.parse::<i32>() {
        Value::Integer(num)
    } else {
        Value::String(raw_value.to_string())
    };

    Condition::Equal(column, value)
}


   

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}