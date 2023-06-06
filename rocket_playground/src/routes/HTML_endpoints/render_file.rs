use rocket_dyn_templates::Template;

#[get("/basic_template")]
pub fn index() -> Template {
    let context = serde_json::json!({"logged-in": false});
    Template::render("index", &context)
}


/*
Template::render("index", context! {
        foo: 123,

 */