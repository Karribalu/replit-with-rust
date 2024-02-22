use actix_web::HttpResponse;

pub async fn search(query: String) -> Result<HttpResponse, actix_web::Error> {

    Ok(HttpResponse::from(HttpResponse::Ok().content_type("text/plain").body("hello")))
}