/*
Last updated: 11-15-2023

Description: This crate defines the request handlers for the web server

Author: James Dean
*/
use hyper::{Request, Response, Body};

use config::VERSION;

// This method returns the version of the node
// Returns: Response<Body>
pub fn get_version() -> Response<Body> {
    Response::new(Body::from(VERSION))
}

// This method creates a transaction in the network
// Params: Request<Body>
// Returns: Response<Body>
pub fn create_transaction(req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(req.method().to_string()))
}

// This method searches a transaction in the network
// Params: Request<Body>
// Returns: Response<Body>
pub fn query_transaction(req: Request<Body>) -> Response<Body> {
    // Here you can extract information from the request if needed
    // For example, query parameters, headers, etc.

    // Then, return the appropriate response
    Response::new(Body::from(req.method().to_string()))
}
