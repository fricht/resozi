use serde::Serialize;


#[derive(Serialize)]
pub struct SlidePresentation {
    pub slides: Vec<SlideData>
}

#[derive(Serialize)]
pub struct SlideData {
    pub name: String,
    pub notes: String,
    pub items: Vec<ItemData>
}

#[derive(Serialize)]
pub struct ItemData {
    pub id: String,
    pub transition: String,
    pub opacity: f64,
    pub position: [f64; 2],
    pub rotation: f64,
    pub scale: f64
}
