mod image;
mod model;

use std::collections::HashMap;
use std::rc::{Rc, Weak};

use futures::future::join_all;
use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Request, RequestInit, RequestMode, Response};

use crate::engine::EngineError;
use crate::model::{LevelData, SpriteCell, SpriteData};
use crate::resource_loader::image::ImageFuture;
pub use crate::resource_loader::model::{ResourceDataType, ResourceKind};
use crate::web_utils::window;

pub struct ResourceLoader;

pub struct Resource {
    pub key: String,
    pub cell: Vec<SpriteCell>,
    pub data: SpriteData,
    pub image: Option<Weak<HtmlImageElement>>,
}

pub struct Resources {
    pub cells: HashMap<String, Vec<SpriteCell>>,
    pub data: HashMap<String, SpriteData>,
    pub level_data: HashMap<String, LevelData>,
    pub images: HashMap<String, Rc<HtmlImageElement>>,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            cells: HashMap::new(),
            data: HashMap::new(),
            level_data: HashMap::new(),
            images: HashMap::new(),
        }
    }

    pub fn get_resource(&self, name: &str, kind: &ResourceKind) -> Resource {
        let resource_key = format!("{}/{}", kind.value(), name);

        let cell = self.cells.get(&resource_key).unwrap();
        let data = self.data.get(&resource_key).unwrap();
        let image = self.images.get(kind.value());

        Resource {
            key: resource_key,
            cell: cell.clone(),
            data: data.clone(),
            image: image.map(Rc::downgrade),
        }
    }

    pub fn get_cell(&self, name: &str, kind: &ResourceKind) -> Vec<SpriteCell> {
        let resource_key = format!("{}/{}", kind.value(), name);
        let cells = self.cells.get(&resource_key);

        match cells {
            None => vec![],
            Some(cells) => cells.to_vec(),
        }
    }

    pub fn get_level_data(&self, level_id: &str) -> LevelData {
        let resource_key = format!("{}/{}", ResourceKind::Level.value(), level_id);

        let level_data = self
            .level_data
            .get(&resource_key)
            .unwrap_or_else(|| panic!("Level data is expected to be preset {}", resource_key));

        level_data.clone()
    }
}

impl ResourceLoader {
    pub async fn load(&self) -> Resources {
        let cells = self
            .load_json_resources::<Vec<SpriteCell>>(
                vec![
                    ResourceKind::Card,
                    ResourceKind::Plant,
                    ResourceKind::Zombie,
                    ResourceKind::Interface,
                ],
                ResourceDataType::Cell,
            )
            .await;

        let data = self
            .load_json_resources::<SpriteData>(
                vec![
                    ResourceKind::Card,
                    ResourceKind::Plant,
                    ResourceKind::Zombie,
                    ResourceKind::Interface,
                ],
                ResourceDataType::Data,
            )
            .await;

        let level_data = self
            .load_json_resources::<LevelData>(vec![ResourceKind::Level], ResourceDataType::Data)
            .await;

        let images = self
            .load_image_resources(vec![
                ResourceKind::Card,
                ResourceKind::Plant,
                ResourceKind::Zombie,
                ResourceKind::Interface,
            ])
            .await;

        Resources {
            cells,
            data,
            level_data,
            images,
        }
    }

    async fn load_json_resources<T>(
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

    async fn load_image_resources(
        &self,
        kinds: Vec<ResourceKind>,
    ) -> HashMap<String, Rc<HtmlImageElement>> {
        let image_futures: Vec<ImageFuture> = kinds
            .iter()
            .map(|kind| ImageFuture::new(&format!("assets/image/{}.png", kind.value())))
            .collect();

        let promise_all = join_all(image_futures).await;

        let images: HashMap<String, Rc<HtmlImageElement>> = kinds
            .iter()
            .zip(promise_all.into_iter())
            .filter(|(_key, value)| (*value).is_ok())
            .map(|(key, value)| (key.value().to_string(), Rc::new(value.unwrap())))
            .collect();

        images
    }
}
