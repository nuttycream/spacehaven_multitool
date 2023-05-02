use crate::utils::{get_attribute_value_node, set_attribute_value_node};
use amxml::dom::*;
use std::{collections::HashMap, error::Error};

type PatchFunction = fn(&mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>>;

fn attribute_set(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let attribute = patch_args
        .get("attribute")
        .ok_or("attribute not found")?
        .value();
    let value = patch_args.get("value").ok_or("value not found")?.value();

    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        element.set_attribute(&attribute, &value);
    }

    Ok(())
}

fn attribute_add(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let attribute = patch_args
        .get("attribute")
        .ok_or("attribute not found")?
        .value();
    let value = patch_args.get("value").ok_or("value not found")?.value();

    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        if element
            .attributes()
            .iter()
            .find(|e| e.name() == attribute)
            .is_none()
        {
            element.set_attribute(&attribute, &value);
        }
    }

    Ok(())
}

fn attribute_remove(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let attribute = patch_args
        .get("attribute")
        .ok_or("attribute not found")?
        .value();

    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        element.delete_attribute(&attribute);
    }

    Ok(())
}

fn attribute_math(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let attribute = patch_args
        .get("attribute")
        .ok_or("attribute not found")?
        .value();
    let value = patch_args.get("value").ok_or("value not found")?;
    let value_as_float = value.value().parse::<f32>()?;
    let binding = value
        .attribute_value("opType")
        .ok_or("Could not get Operation Type for AttributeMath")?;
    let operation_type = binding
        .as_str();

    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        let start_value = get_attribute_value_node::<f32>(&element, &attribute)?;
        let new_value = match operation_type {
            "add" => start_value + value_as_float,
            "subtract" => start_value - value_as_float,
            "multiply" => start_value * value_as_float,
            "divide" => start_value / value_as_float,
            _ => return Err("Invalid Operation Type".into()),
        };
        let string = if new_value.fract() == 0.0 {
            format!("{}", new_value as i32)
        } else {
            format!("{:.1}", new_value)
        };
        element.set_attribute(&attribute, &string);
    }

    Ok(())
}

// <Operation Class="Add">
//     <xpath>/data/TechTree/tree[@id="2535"]/items</xpath>
//     <value>
//         <i  tid="2585" x="53" y="210" />
//     </value>
// </Operation>
fn node_add(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let values: Vec<NodePtr> = patch_args
        .iter()
        .filter(|(key, _value)| key.contains("value"))
        .map(|(_key, value)| value.clone())
        .collect();

    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        for node in &values {
            element.append_child(&node);
        }
    }

    Ok(())
}

fn node_insert(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let value = patch_args
        .get("value")
        .ok_or("value not found")?
        .clone();

    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        element.replace_with(&value);
    }

    Ok(())
}

fn node_remove(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        let parent = element.parent().ok_or("Node has no parent.")?;
        parent.delete_child(&element);
    }

    Ok(())
}

fn node_replace(patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    let current_core_lib_elements: Vec<&mut NodePtr> = patch_args
        .iter_mut()
        .filter(|(key, _value)| key.contains("coreLibsElem"))
        .map(|(_key, value)| value)
        .collect();

    for element in current_core_lib_elements {
        let parent = element.parent().ok_or("Node has no parent.")?;
        parent.delete_child(&element);
    }

    Ok(())
}

fn bad_op(_patch_args: &mut HashMap<&str, NodePtr>) -> Result<(), Box<dyn Error>> {
    Err("Unrecognized Patch Operation".into())
}

fn patch_dispatch(p_type: &str) -> PatchFunction {
    match p_type {
        "AttributeSet" => attribute_set,
        "AttributeAdd" => attribute_add,
        "AttributeRemove" => attribute_remove,
        "AttributeMath" => attribute_math,
        "Add" => node_add,
        "Insert" => node_insert,
        "Remove" => node_remove,
        "Replace" => node_replace,
        _ => bad_op,
    }
}
