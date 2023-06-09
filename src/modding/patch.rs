use crate::utils::get_attribute_value_node;
use amxml::dom::NodePtr;
use std::{collections::HashMap, error::Error};


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
    let operation_type = binding.as_str();

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
    let value = patch_args.get("value").ok_or("value not found")?.clone();

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

fn patch_dispatch(
    p_type: &str,
    patch_args: &mut HashMap<&str, NodePtr>,
) -> Result<(), Box<dyn Error>> {
    match p_type {
        "AttributeSet" => attribute_set(patch_args),
        "AttributeAdd" => attribute_add(patch_args),
        "AttributeRemove" => attribute_remove(patch_args),
        "AttributeMath" => attribute_math(patch_args),
        "Add" => node_add(patch_args),
        "Insert" => node_insert(patch_args),
        "Remove" => node_remove(patch_args),
        "Replace" => node_replace(patch_args),
        _ => bad_op(patch_args),
    }
}

fn do_patch_type(
    core_library: HashMap<&str, NodePtr>,
    patch: &NodePtr,
    location: &str,
) -> Result<(), Box<dyn Error>> {
    let binding = patch
        .attribute_value("Class")
        .ok_or("Failed to find patch type class")?;
    let patch_type = binding.as_str();
    let xpath = patch
        .get_first_node("./xpath")
        .ok_or("Could not find xpath for patch")?
        .value();

    if let Some(current_core_lib) = core_library.get(location) {
        let current_core_lib_elements = current_core_lib.get_nodeset(&xpath)?;
        let mut patch_args: HashMap<&str, NodePtr> = HashMap::new();
        patch_args.insert("value", patch.get_first_node("value").unwrap());
        patch_args.insert("attribute", patch.get_first_node("attribute").unwrap());
        for core_lib_element in current_core_lib_elements {
            patch_args.insert("coreLibsElem", core_lib_element);
        }
        patch_dispatch(patch_type, &mut patch_args)?;

        // TODO: Replace Config Variables with user chosen value
        // if mod.variables { ... }
    }

    // # Replace Config Variables with user chosen value.
    // TODO: Prefer to do this elsewhere.
    //     if mod.variables:
    //         for var in mod.variables:
    //             patchArgs["value"].text = patchArgs["value"].text.replace( str(var.name), str(var.value) )

    Ok(())
}

pub fn do_patches(
    core_library: HashMap<&str, NodePtr>,
    mod_library: HashMap<&str, Vec<NodePtr>>,
) -> Result<(), Box<dyn Error>> {

    for (location, patches) in mod_library.iter() {
        for patch in patches {
            if patch.get_first_node("./Noload").is_some() {
                continue;
            }
            for patch_operation in patch.get_nodeset("./Operation")? {
                //TODO: why clone?
                do_patch_type(core_library.clone(), &patch_operation, location)?; 
            }
        }
    }
    
    Ok(())
}
