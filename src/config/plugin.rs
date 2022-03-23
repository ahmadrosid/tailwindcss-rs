use serde_json::Map;
use serde_json::Value;

// name: key -> [properties]
// value: inset -> [[String("top"), String("right"), String("bottom"), String("left")]]
// css: .inset-0 { top: 0px; right: 0px; bottom: 0px; left: 0px; }
pub type PluginValue = Map<String, Value>;

pub fn create_utility_plugin<'a>(
    name: &'a str,
    obj: &'a Map<String, Value>,
) -> Option<PluginValue> {
    let arr = obj.get(name)?.as_array()?;
    let mut data: PluginValue = Map::new();

    for item in arr {
        if item.get(0)?.is_string() {
            let key = item.get(0)?.as_str()?.to_string();
            let variants = item.get(1)?.clone();
            data.insert(key, variants);
        } else {
            for deep_item in item.as_array()? {
                let key = deep_item.get(0)?.as_str()?.to_string();
                let variants = deep_item.get(1)?.clone();
                data.insert(key, variants);
            }
        }
    }

    Some(data)
}
