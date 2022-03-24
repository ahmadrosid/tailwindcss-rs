use serde_json::Map;
use serde_json::Value;

// name: key -> [properties]
// value: inset -> [[String("top"), String("right"), String("bottom"), String("left")]]
// css: .inset-0 { top: 0px; right: 0px; bottom: 0px; left: 0px; }
pub type Utility = Map<String, Value>;

pub enum PluginMode {
    WithNegative,
    OnlyPositive,
}

fn build_value(data: &mut Utility, item: &Value, mode: &PluginMode) -> Option<()> {
    let key = item.get(0)?.as_str()?.to_string();
    let variants = item.get(1)?.clone();
    match mode {
        PluginMode::WithNegative => {
            data.insert(key.to_string(), variants.clone());
            data.insert(format!("-{}", &key), variants);
        }
        PluginMode::OnlyPositive => {
            data.insert(key, variants);
        }
    }
    Some(())
}

pub fn create_utility<'a>(
    name: &'a str,
    obj: &'a Map<String, Value>,
    mode: PluginMode,
) -> Option<Utility> {
    let plugin = obj.get("plugins")?.as_object()?;
    let arr = plugin.get(name)?.as_array()?;
    let mut data: Utility = Map::new();

    for item in arr {
        if item.get(0)?.is_string() {
            build_value(&mut data, item, &mode);
            continue;
        }

        for deep_item in item.as_array()? {
            build_value(&mut data, deep_item, &mode);
        }
    }

    Some(data)
}
