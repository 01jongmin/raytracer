use actix_web::{
    http::header::{self, ContentType}, middleware,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer, Responder,
};

mod broadcast;
use broadcast::Broadcaster;

use raytracer::camera::Camera;
use raytracer::vec3::Vec3;
use raytracer::{ random_scene };

use image::{RgbImage, ImageOutputFormat};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let data = Broadcaster::create();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/render-event/{id}", web::get().to(new_client))
            .route("/render/{id}", web::get().to(broadcast))
            .route("/size", web::get().to(size))
    })
    .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn index() -> impl Responder {
    let index_html = include_str!("index.html");

    HttpResponse::Ok()
        .append_header(ContentType::html())
        .body(index_html)
}

async fn new_client(id: Path<u64>, broadcaster: Data<Broadcaster>) -> impl Responder {
    let rx = broadcaster.get_client(*id).unwrap(); 

    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx)
}

async fn size(broadcaster: Data<Broadcaster>) -> impl Responder {
    let test = format!("{}", broadcaster.get_number());
    HttpResponse::Ok()
        .body(test)
}

async fn broadcast(id: Path<u64>, broadcaster: Data<Broadcaster>) -> impl Responder {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 900;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 10;
    let max_depth = 50;

    let world = random_scene(10);

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);

    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vup, 20., aspect_ratio, aperture, dist_to_focus);

    let integer = Arc::new(Mutex::new(0));

    broadcaster.new_connection(*id);

    let buffer = raytracer::raytrace_buffer(image_width, image_height, samples_per_pixel, 
                                            max_depth, &world, &camera, 
                                            Some(&|_: String| {
                                                let mut lock = integer.lock().unwrap();
                                                *lock += 1;
                                                let t = *lock;
                                                if t % 1000 == 0 { broadcaster.send(*id, &t.to_string()); }
                                            }));

    broadcaster.close_sender(*id);

    let img = RgbImage::from_vec(image_width as u32, image_height as u32, buffer).unwrap();
    let mut w = Cursor::new(Vec::new());
    img.write_to(&mut w, ImageOutputFormat::Png).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(w.into_inner())
}
