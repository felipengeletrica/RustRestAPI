extern crate contract;
extern crate dao;

use contract::Edges;

pub async fn get_edge_by_id(id:&String) -> Option<Edges> {
  if id.is_empty() {
    return None
  }
  return dao::get_edge_by_id(id).await;
}

pub async fn delete_edge_by_id(id:&String) -> Option<bool> {
  if id.is_empty() {
    return None
  }
  return dao::delete_edge_by_id(id).await;
}

pub async fn list_edges() -> Option<Vec<Edges>> {
  return dao::list_edges().await;
}

pub async fn insert_edges(url:&String,desc:&String) -> Option<Edges> {
  if url.is_empty() || desc.is_empty() {
    return None
  }
  return dao::insert_edges(&url,&desc).await;
}