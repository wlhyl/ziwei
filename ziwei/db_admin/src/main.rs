use std::{env, error::Error, net::SocketAddrV4};

use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use db_admin::{args, routers::routes, state::AppState};
use sea_orm::Database;
use tera::Tera;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    dotenv::dotenv().ok();

    let log4rs_config = env::var("LOG4RS_CONFIG")
        .expect("没设置 LOG4RS_CONFIG 环境变量，可在.env文件中设置或export LOG4RS_CONFIG=...");

    log4rs::init_file(log4rs_config, Default::default()).unwrap();

    let database_url = env::var("DATABASE_URL")
        .expect("没设置 DATABASE_URL 环境变量，可在.env文件中设置或export DATABASE_URL=...");

    let db = Database::connect(database_url)
        .await
        .expect("连接到数据库失败");
    // 不能将args放在db之后，会有报错
    // let args = args::Args::parse();

    let shared_data = web::Data::new(AppState { db });
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html"))?;
    // tera.full_reload()?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(shared_data.clone())
            .configure(routes)
            .wrap(Logger::default())
    })
    .workers(args.n)
    .bind(SocketAddrV4::new(args.ip, args.port))?
    .run()
    .await?;
    Ok(())
}
