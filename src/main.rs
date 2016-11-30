extern crate webplatform;

fn main() {
    let document = webplatform::init();
    let head = document.element_query("head").unwrap(); // Head should be available

    // Place the final compiled style here
    head.html_append(
        "<style>
            code {
                color: red;
            }
        </style>"
    );
}
