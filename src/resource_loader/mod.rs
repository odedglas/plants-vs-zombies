mod image;
mod model;

use std::collections::HashMap;
use std::rc::Rc;

use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Request, RequestInit, RequestMode, Response};

use crate::engine::EngineError;
use crate::model::{LevelData, SpriteCell};
use crate::resource_loader::model::{ResourceDataType, ResourceKind};
use crate::web_utils::window;

pub struct ResourceLoader;

pub struct Resources {
    pub cells: HashMap<String, Vec<SpriteCell>>,
    pub level_data: HashMap<String, LevelData>,
}

impl ResourceLoader {
    pub async fn load(&self) -> Resources {
        let cells = self
            .load_resources_kinds::<Vec<SpriteCell>>(
                vec![
                    ResourceKind::Card,
                    ResourceKind::Plant,
                    ResourceKind::Zombie,
                    ResourceKind::Interface,
                ],
                ResourceDataType::CELL,
            )
            .await;

        let level_data = self
            .load_resources_kinds::<LevelData>(vec![ResourceKind::Level], ResourceDataType::DATA)
            .await;

        Resources { cells, level_data }
    }

    async fn load_resources_kinds<T>(
        &self,
        resource_kinds: Vec<ResourceKind>,
        data_type: ResourceDataType,
    ) -> HashMap<String, T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let mut jsons_map = HashMap::new();

        for kind in resource_kinds.iter() {
            // Loads given asset kind associated with a data type.
            let result = &self.load_json(kind.value(), data_type.value()).await;

            if let Ok(value) = result {
                let json_items = self.convert_json_hashmap::<T>(value).unwrap();

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

    fn convert_json_hashmap<T>(&self, json: &JsValue) -> Result<HashMap<String, T>, EngineError>
    where
        for<'a> T: Deserialize<'a>,
    {
        let items = serde_wasm_bindgen::from_value(json.clone())?;

        Ok(items)
    }
}
