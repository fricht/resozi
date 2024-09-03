use std::fs;
mod slide_creator;

fn main() {
    let test_slide = slide_creator::SlidePresentation {
        slides: vec![
            slide_creator::SlideData {
                name: "Hello World!".to_string(),
                notes: "Nothing to say here".to_string(),
                items: vec![slide_creator::ItemData {
                    id: "svg1".to_string(),
                    transition: "0.5s ease-in-out".to_string(),
                    opacity: 1.,
                    position: [0., 0.],
                    rotation: 0.,
                    scale: 10.,
                }],
            },
            slide_creator::SlideData {
                name: "Slide 2".to_string(),
                notes: "Nothing to say here, again".to_string(),
                items: vec![slide_creator::ItemData {
                    id: "svg1".to_string(),
                    transition: "1s ease-in-out".to_string(),
                    opacity: 0.8,
                    position: [-50., -50.],
                    rotation: 0.,
                    scale: 50.,
                }],
            },
        ],
    };
    let test_svg = slide_creator::SVG {
        data: "<svg class=\"svg-layer\" viewBox=\"0 0 100 100\"><circle cx=\"50\" cy=\"50\" r=\"45\" stroke=\"black\" stroke-width=\"3\" fill=\"red\" /></svg>".to_string(),
        id: "svg1".to_string(),
    };
    match slide_creator::create_web(vec![test_svg], test_slide) {
        Ok(html) => {
            if let Err(e) = fs::write("output-presentation.html", html) {
                panic!("{}", e)
            }
        }
        Err(e) => panic!("{}", e),
    }
}
