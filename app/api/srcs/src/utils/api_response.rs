use actix_web::{body::BoxBody, http, web, HttpResponse, Responder};

#[allow(unused)]
#[derive(Debug)]
pub struct ApiResponse {
	pub status_code: u16,
	pub body: String,
	response_code: http::StatusCode,
}

impl ApiResponse {
	pub fn new(status_code: u16, body: String) -> Self {
		ApiResponse {
			status_code,
			body,
			response_code: http::StatusCode::from_u16(status_code).expect(&format!("invalid status code: {status_code}")),
		}
	}
}

#[allow(unused)]
impl Responder for ApiResponse {
	type Body = BoxBody;

	fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
		let body = BoxBody::new(
			web::BytesMut::from(self.body.as_bytes())
		);

		HttpResponse::new(self.response_code).set_body(body)
	}
}
