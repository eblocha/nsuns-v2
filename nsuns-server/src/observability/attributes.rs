use std::net::IpAddr;

use axum::extract::{ConnectInfo, MatchedPath};
use http::Version;

use crate::server::ClientInfo;

/// Attributes computed from an HTTP request. Used for telemetry spans and metrics.
#[derive(Debug, Clone)]
pub struct HttpRequestAttributes<'r> {
    pub http_request_method: http::Method,
    pub http_route: Option<&'r str>,
    pub user_agent_original: Option<&'r str>,
    pub client_address: Option<String>,
    pub network_local_address: Option<IpAddr>,
    pub network_local_port: Option<u16>,
    pub network_peer_address: Option<IpAddr>,
    pub network_peer_port: Option<u16>,
    pub network_protocol_name: &'static str,
    pub network_protocol_version: Option<&'static str>,
    pub network_type: Option<&'static str>,
    pub server_address: Option<&'r str>,
    pub url_path: &'r str,
    pub url_query: Option<&'r str>,
    pub url_scheme: Option<&'r str>,
}

impl<'r, B> From<&'r http::Request<B>> for HttpRequestAttributes<'r> {
    fn from(request: &'r http::Request<B>) -> Self {
        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(|path| path.as_str());

        let client_info = request
            .extensions()
            .get::<ConnectInfo<ClientInfo>>()
            .map(|c| c.0.clone());

        let peer_address = client_info.as_ref().map(network_peer_ip);

        let client_address = forwarded_for(request)
            .map(ToOwned::to_owned)
            .or(peer_address.map(|ip| ip.to_string()));

        let server_address = http_host(request).or_else(|| forwarded_host(request));

        let url_scheme = request
            .uri()
            .scheme_str()
            .or_else(|| forwarded_proto(request));

        Self {
            http_request_method: request.method().clone(),
            http_route: matched_path,
            user_agent_original: user_agent(request),
            client_address,
            network_local_address: client_info.as_ref().map(network_local_ip),
            network_local_port: client_info.as_ref().map(network_local_port),
            network_peer_address: peer_address,
            network_peer_port: client_info.as_ref().map(network_peer_port),
            network_protocol_name: "http",
            network_protocol_version: protocol_version(request),
            network_type: client_info.as_ref().map(network_type),
            server_address,
            url_path: request.uri().path(),
            url_query: request.uri().query(),
            url_scheme,
        }
    }
}

#[inline]
fn user_agent<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get(http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
}

#[inline]
fn protocol_version<B>(req: &http::Request<B>) -> Option<&'static str> {
    match req.version() {
        Version::HTTP_09 => Some("0.9"),
        Version::HTTP_10 => Some("1.0"),
        Version::HTTP_11 => Some("1.1"),
        Version::HTTP_2 => Some("2.0"),
        Version::HTTP_3 => Some("3.0"),
        _ => None,
    }
}

#[inline]
fn http_host<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get(http::header::HOST)
        .map_or(req.uri().host(), |h| h.to_str().ok())
}

// Client Info

#[inline]
fn network_peer_ip(client_info: &ClientInfo) -> IpAddr {
    client_info.remote_addr.ip()
}

#[inline]
fn network_peer_port(client_info: &ClientInfo) -> u16 {
    client_info.remote_addr.port()
}

#[inline]
fn network_local_ip(client_info: &ClientInfo) -> IpAddr {
    client_info.local_addr.ip()
}

#[inline]
fn network_local_port(client_info: &ClientInfo) -> u16 {
    client_info.local_addr.port()
}

#[inline]
fn network_type(client_info: &ClientInfo) -> &'static str {
    if network_peer_ip(client_info).is_ipv4() {
        "ipv4"
    } else {
        "ipv6"
    }
}

fn forwarded_for<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.split(',').next())
}

fn forwarded_host<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get("X-Forwarded-Host")
        .and_then(|h| h.to_str().ok())
}

fn forwarded_proto<B>(req: &http::Request<B>) -> Option<&str> {
    req.headers()
        .get("X-Forwarded-Proto")
        .and_then(|h| h.to_str().ok())
}
