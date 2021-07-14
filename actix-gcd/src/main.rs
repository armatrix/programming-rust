use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParams {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            // 既然是web服务器，这里我们可以对REST API做简单的介绍 参考： https://cloud.google.com/apis/design?hl=zh-cn
            .route("/", web::get().to(get_index))
            .route("/", web::post().to(gcd_post))
    });
    let addr = "127.0.0.1:6060";
    println!("serving on: {}", addr);
    server
        .bind(addr)
        .expect("error while binding")
        .run()
        .expect("error while running");
    // 注意 这里是不输出的
    println!("serving on test");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

fn gcd_post(form: web::Form<GcdParams>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("computing the GCD with zero is boring.");
    }

    let response = format!(
        "the greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m = m % n;
    }
    n
}

// 可能用到的命令
// ab -n 1000 -c 100 http://127.0.0.1:6060/
