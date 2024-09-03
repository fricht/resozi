use serde::Serialize;
use serde_json;

const HTML_TEMPLATE: &str = include_str!("ressources/mainpage.html");
const BREAK_CHAR: &str = "¿";

#[derive(Serialize)]
pub struct SlidePresentation {
    pub slides: Vec<SlideData>,
}

#[derive(Serialize)]
pub struct SlideData {
    pub name: String,
    pub notes: String,
    pub items: Vec<ItemData>,
}

#[derive(Serialize)]
pub struct ItemData {
    pub id: String,
    pub transition: String,
    pub opacity: f64,
    pub position: [f64; 2],
    pub rotation: f64,
    pub scale: f64,
}

pub struct SVG {
    pub data: String,
    pub id: String,
}

fn svg_to_str(svg: &SVG) -> String {
    let mut data = "<div class=\"svg-container\"  id=\"".to_string();
    data.push_str(&svg.id);
    data.push_str("\">");
    data.push_str(&svg.data);
    data.push_str("</div>");
    data
}

fn svgs_to_str(svgs: &Vec<SVG>) -> String {
    let mut data = "".to_string();
    for svg in svgs {
        data.push_str(&svg_to_str(&svg));
    }
    data
}

pub fn create_web(svgs: Vec<SVG>, slides: SlidePresentation) -> Result<String, String> {
    let slides_data = serde_json::to_string(&slides).map_err(|e| e.to_string())?;
    let html_page_template: Vec<&str> = HTML_TEMPLATE.split(BREAK_CHAR).collect();
    if html_page_template.len() != 3 {
        return Err("Failed to load template.".to_string());
    }
    let mut html_page = html_page_template[0].to_string();
    html_page.push_str(&svgs_to_str(&svgs));
    html_page.push_str(html_page_template[1]);
    html_page.push_str(&slides_data);
    html_page.push_str(html_page_template[2]);
    Ok(html_page)
}
