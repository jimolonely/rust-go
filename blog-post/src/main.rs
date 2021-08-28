use blog_post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我是个真男人");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("我是个真男人", post.content());
}
