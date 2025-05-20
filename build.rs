extern crate embed_resource;

fn main() {
    // Compile and link
    embed_resource::compile("dota2-timer.rc", embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
