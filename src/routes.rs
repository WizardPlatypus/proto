use rocket::{form::Form, request::FromParam};

#[get("/login")]
pub fn get_login() {
    todo!();
}

#[post("/login")]
pub fn post_login() {
    todo!();
}

#[get("/register")]
pub fn get_register() {
    todo!();
}

#[post("/register")]
pub fn post_register() {
    todo!();
}

#[get("/logout")]
pub fn get_logout() {
    todo!();
}

pub struct Locator {
    todo: usize,
}

impl<'a> FromParam<'a> for Locator {
    type Error = ();
    fn from_param(_param: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[get("/post/<_locator>")]
pub fn get_post_locator(_locator: Locator) {
    todo!()
}

#[get("/post/<_id>")]
pub fn get_post_id(_id: usize) {
    todo!()
}

#[derive(FromForm)]
pub struct PatchForm {
    todo: usize,
}

#[patch("/post/<_id>", data="<_form>")]
pub fn patch_post_id(_id: usize, _form: Form<PatchForm>) {
    todo!()
}

#[delete("/post/<_id>")]
pub fn delete_post_id(_id: usize) {
    todo!()
}

#[get("/post/new")]
pub fn get_post_new() {
    todo!()
}

#[derive(FromForm)]
pub struct NewForm {
    todo: usize,
}

#[post("/post/new", data="<_form>")]
pub fn post_new(_form: Form<NewForm>) {
    todo!()
}

#[put("/post/<_id>?<_tag>&<_agree>")]
pub fn put_post_id_tag_agree(_id: usize, _tag: &str, _agree: Option<bool>) {
    todo!()
}

#[delete("/post/<_id>?<_tag>&<_agree>")]
pub fn delete_post_id_tag_agree(_id: usize, _tag: &str, _agree: Option<bool>) {
    todo!()
}
