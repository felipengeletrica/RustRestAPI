use actix_web::{get,put,delete,Responder,HttpResponse,web};
#[path="service.rs"] mod service;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /edges ")
}

#[get("/edges")]
pub async fn list_edges() -> HttpResponse  {
    let edge = service::list_edges().await;
    HttpResponse::Ok().json(edge)
}

#[get("/edge/{id}")]
pub async fn get_edge_by_id(info:web::Path<String>) -> HttpResponse  {
    let id  = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let edge = service::get_edge_by_id(&new_string).await;
    HttpResponse::Ok().json(edge)
}

#[delete("/edge/{id}")]
pub async fn delete_edge_by_id(info:web::Path<String>) -> HttpResponse {
    let id  = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let news = service::delete_edge_by_id(&new_string).await;
    HttpResponse::Ok().json(news)
}

#[put("/edge/{url}/{desc}")]
pub async fn insert_edge(info:web::Path<(String, String)>) -> impl Responder {
    let url  = &info.0;
    let desc = &info.1;
    let edge = service::insert_edges(url,desc).await;
    HttpResponse::Ok().json(edge)
}