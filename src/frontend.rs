use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "order.pest"]
pub struct OrderParser;

#[cfg(test)]
mod tests;
