use lambda_http::{lambda, Request, RequestExt};

fn main() {
    lambda!(
        |request: Request, _context| Ok(
            format!(
                "hello {}",
                request.query_string_parameters()
                    .get("name")
                    .unwrap_or_else(|| "stranger")
            )
        )
    )
}
