use super::extract_object;
use super::Object;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;

// name: key -> [properties]
// value: inset -> [[String("top"), String("right"), String("bottom"), String("left")]]
// css: .inset-0 { top: 0px; right: 0px; bottom: 0px; left: 0px; }
pub type Utility = Map<String, Value>;

#[derive(Clone, Copy)]
pub enum Mode {
    WithNegative,
    OnlyPositive,
}

fn build_value(data: &mut Utility, item: &Value, mode: Mode) -> Option<()> {
    let key = item.get(0)?.as_str()?.to_string();
    let variants = item.get(1)?.clone();
    match mode {
        Mode::WithNegative => {
            data.insert(key.to_string(), variants.clone());
            data.insert(format!("-{}", &key), variants);
        }
        Mode::OnlyPositive => {
            data.insert(key, variants);
        }
    }
    Some(())
}

pub fn create_utility<'a>(
    name: &'a str,
    obj: &'a Map<String, Value>,
    mode: Mode,
) -> Option<Utility> {
    let plugin = obj.get("plugins")?.as_object()?;
    let arr = plugin.get(name)?.as_array()?;
    let mut data: Utility = Map::new();

    for item in arr {
        if item.get(0)?.is_string() {
            build_value(&mut data, item, mode);
            continue;
        }

        for deep_item in item.as_array()? {
            build_value(&mut data, deep_item, mode);
        }
    }

    Some(data)
}

pub fn extract_base(obj: &'_ Map<String, Value>) -> (Object, Map<String, Value>) {
    let spacing = extract_object(obj, "spacing");
    let mut base: Object = HashMap::new();

    base.insert("basis".into(), {
        let mut data = extract_object(obj, "data");
        data.append(&mut spacing.clone());
        data
    });

    base.insert("margin".into(), {
        let mut data = extract_object(obj, "margin");
        data.append(&mut spacing.clone());
        data
    });

    base.insert("z_index".into(), {
        let mut data = extract_object(obj, "z_index");
        data.append(&mut spacing.clone());
        data
    });

    base.insert("width".into(), {
        let mut data = extract_object(obj, "width");
        data.append(&mut spacing.clone());
        data
    });

    base.insert("height".into(), {
        let mut data = extract_object(obj, "height");
        data.append(&mut spacing.clone());
        data
    });

    (base, spacing)
}
