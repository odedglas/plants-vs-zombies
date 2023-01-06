mod image;
mod model;

use std::collections::HashMap;

use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::engine::EngineError;
use crate::model::SpriteCell;
use crate::resource_loader::model::ResourceKind;
use crate::web_utils::window;

pub struct ResourceLoader;

pub struct Resources {
    pub cells: HashMap<String, Vec<SpriteCell>>,
}

impl ResourceLoader {
    pub async fn load(&self) -> Resources {
        let cells = self
            .load_jsons::<SpriteCell>(
                vec![
                    ResourceKind::Card,
                    ResourceKind::Plant,
                    ResourceKind::Zombie,
                    ResourceKind::Interface,
                ],
                "cell",
            )
            .await;

        Resources { cells }
    }

    async fn load_jsons<T>(
        &self,
        resource_kinds: Vec<ResourceKind>,
        data_type: &str,
    ) -> HashMap<String, Vec<T>>
    where
        for<'a> T: Deserialize<'a>,
    {
        let mut jsons_map = HashMap::new();

        for kind in resource_kinds.iter() {
            // Loads given asset kind associated with a data type.
            let result = &self.load_json(kind.value(), data_type).await;

            if let Ok(value) = result {
                let json_items: HashMap<String, Vec<T>> = self.convert_json::<T>(value).unwrap();

                for (key, value) in json_items {
                    jsons_map.insert(format!("{}/{}", kind.value(), key), value);
                }
            }
        }

        jsons_map
    }

    async fn load_json(&self, path: &str, data_type: &str) -> Result<JsValue, EngineError> {
        // https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
        let qualified_path = format!("/assets/json/{}-{}.json", path, data_type);
        let mut opts = RequestInit::new();

        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&qualified_path, &opts)?;

        let window = window();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;
        Ok(json)
    }

    fn convert_json<T>(&self, json: &JsValue) -> Result<HashMap<String, Vec<T>>, EngineError>
    where
        for<'a> T: Deserialize<'a>,
    {
        let items = serde_wasm_bindgen::from_value(json.clone())?;

        Ok(items)
    }
}
