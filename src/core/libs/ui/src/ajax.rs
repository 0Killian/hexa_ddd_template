use yew::virtual_dom::VTag;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct AjaxProps {
    pub method: Option<AjaxMethod>,
    pub url: Option<String>,
    pub encoding: Option<AjaxEncoding>,
    pub indicator: Option<String>,
    pub target: Option<String>,
    pub target_error: Option<String>,
    pub trigger: Option<String>,
    pub confirm: Option<String>,
    pub vals: Option<String>,
    pub on_click: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AjaxMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AjaxEncoding {
    MultipartFormData,
    UrlEncoded,
}

impl AjaxProps {
    pub fn add_to_vtag(&self, vtag: &mut VTag) {
        let htmx_props = HtmxProps::from(self);

        for (prop, attr) in [
            (htmx_props.get, "hx-get"),
            (htmx_props.post, "hx-post"),
            (htmx_props.put, "hx-put"),
            (htmx_props.delete, "hx-delete"),
            (htmx_props.patch, "hx-patch"),
            (htmx_props.indicator, "hx-indicator"),
            (htmx_props.target, "hx-target"),
            (htmx_props.target_error, "hx-target-error"),
            (htmx_props.encoding, "hx-encoding"),
            (htmx_props.trigger, "hx-trigger"),
            (htmx_props.confirm, "hx-confirm"),
            (htmx_props.vals, "hx-vals"),
            (htmx_props.on_click, "hx-on:click"),
        ] {
            if let Some(prop) = prop {
                vtag.add_attribute(attr, prop);
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct HtmxProps {
    pub get: Option<String>,
    pub post: Option<String>,
    pub put: Option<String>,
    pub delete: Option<String>,
    pub patch: Option<String>,
    pub indicator: Option<String>,
    pub target: Option<String>,
    pub target_error: Option<String>,
    pub encoding: Option<String>,
    pub trigger: Option<String>,
    pub confirm: Option<String>,
    pub vals: Option<String>,
    pub on_click: Option<String>,
}

impl From<&AjaxProps> for HtmxProps {
    fn from(ajax_props: &AjaxProps) -> Self {
        let (get, post, put, delete, patch) = match ajax_props.method {
            Some(AjaxMethod::Get) => (ajax_props.url.clone(), None, None, None, None),
            Some(AjaxMethod::Post) => (None, ajax_props.url.clone(), None, None, None),
            Some(AjaxMethod::Put) => (None, None, ajax_props.url.clone(), None, None),
            Some(AjaxMethod::Delete) => (None, None, None, ajax_props.url.clone(), None),
            Some(AjaxMethod::Patch) => (None, None, None, None, ajax_props.url.clone()),
            None => (None, None, None, None, None),
        };

        let encoding = match ajax_props.encoding {
            Some(AjaxEncoding::MultipartFormData) => Some(String::from("multipart/form-data")),
            Some(AjaxEncoding::UrlEncoded) => {
                Some(String::from("application/x-www-form-urlencoded"))
            }
            None => None,
        };

        HtmxProps {
            get,
            post,
            put,
            delete,
            patch,
            indicator: ajax_props.indicator.clone(),
            target: ajax_props.target.clone(),
            target_error: ajax_props.target_error.clone(),
            encoding,
            trigger: ajax_props.trigger.clone(),
            confirm: ajax_props.confirm.clone(),
            vals: ajax_props.vals.clone(),
            on_click: ajax_props.on_click.clone(),
        }
    }
}
