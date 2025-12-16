#[derive(Debug, Clone)]
pub struct HTML {
    pub class: String,
    pub id: String,
    pub value: String
}

impl HTML {
    pub fn new(class: String, id: String, value: String) -> HTML {
        HTML {
            class: class,
            id: id,
            value: value
        }
    }

    pub fn conv_to_string(&self) -> String {
        let string = format!("<div class={} id={}> {} </div>", self.class, self.id, self.value);
        string
    }

    pub fn append_value(mut html: HTML, string: String) -> HTML {
        html.value.push_str(string.as_str());

        html
    }

    pub fn set_value(mut html: HTML, string: String) -> HTML {
        html.value = string;
        html
    }
}
