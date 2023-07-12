use crate::*;

impl Element {
    // htmx
    pub fn hx_get(self, value: impl Render) -> Self {
        self.attr("hx-get", value)
    }

    pub fn hx_post(self, value: impl Render) -> Self {
        self.attr("hx-post", value)
    }

    pub fn hx_patch(self, value: impl Render) -> Self {
        self.attr("hx-patch", value)
    }

    pub fn hx_delete(self, value: impl Render) -> Self {
        self.attr("hx-delete", value)
    }

    pub fn hx_put(self, value: impl Render) -> Self {
        self.attr("hx-put", value)
    }

    pub fn hx_trigger(self, value: impl Render) -> Self {
        self.attr("hx-trigger", value)
    }

    pub fn hx_target(self, value: impl Render) -> Self {
        self.attr("hx-target", value)
    }

    pub fn hx_swap(self, value: impl Render) -> Self {
        self.attr("hx-swap", value)
    }

    pub fn hx_swap_oob(self, value: impl Render) -> Self {
        self.attr("hx-swap-oob", value)
    }

    pub fn hx_indicator(self, value: impl Render) -> Self {
        self.attr("hx-indicator", value)
    }

    pub fn hx_confirm(self, value: impl Render) -> Self {
        self.attr("hx-confirm", value)
    }

    pub fn hx_headers(self, value: impl Render) -> Self {
        self.attr("hx-headers", value)
    }

    pub fn hx_params(self, value: impl Render) -> Self {
        self.attr("hx-params", value)
    }

    pub fn hx_timeout(self, value: impl Render) -> Self {
        self.attr("hx-timeout", value)
    }

    pub fn hx_ws(self, value: impl Render) -> Self {
        self.attr("hx-ws", value)
    }

    pub fn hx_ws_reconnect(self, value: impl Render) -> Self {
        self.attr("hx-ws-reconnect", value)
    }

    pub fn hx_ws_elt(self, value: impl Render) -> Self {
        self.attr("hx-ws-elt", value)
    }
}
