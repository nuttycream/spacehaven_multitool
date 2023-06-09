/*
/////////////////////////////////////////////// parallel version
// Extract data from nodeptrs using single-thread
let region_data: Vec<_> = regions
    .iter()
    .map(|region| {
        //unwrap for now.
        let name: String = get_attribute_value_node(&region, "n").unwrap();
        let x: usize = get_attribute_value_node(&region, "x").unwrap();
        let y: usize = get_attribute_value_node(&region, "y").unwrap();
        let w: usize = get_attribute_value_node(&region, "w").unwrap();
        let h: usize = get_attribute_value_node(&region, "h").unwrap();
        let page: String = get_attribute_value_node(&region, "t").unwrap();

        (name, x, y, w, h, page)
    })
    .collect();

// parallel operations for more performance
let cims_mutex = Arc::new(Mutex::new(HashMap::<String, Texture>::new()));

// HashSet for checking whether the page has been processed.
let processed_pages: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

let new_textures: Vec<(String, Texture)> = region_data
    .par_iter()
    .filter_map(|(_, _, _, _, _, page)| {
        let mut processed_pages = processed_pages.lock().unwrap();
        if !processed_pages.contains(page) {
            processed_pages.insert(page.clone());
            drop(processed_pages); // Release the lock as soon as possible.
            let cim_filename = format!("{}.cim", page);
            log::info!("Unpacking textures {}", cim_filename);
            let texture = Texture::new(
                &core_path.join("library").join(&cim_filename),
                false,
                None,
                None,
            );
            Some((page.clone(), texture))
        } else {
            None
        }
    })
    .collect();

// Insert new textures into cims single-threaded
let mut cims = cims_mutex.lock().unwrap();
for (page, texture) in &new_textures {
    if !cims.contains_key(page) {
        cims.insert(page.clone(), texture.clone());
    }
}

let exploded_textures_path = core_path.join("library").join("textures.exploded");

// export each region
region_data.par_iter().for_each(|(name, x, y, w, h, page)| {

    let texture_exploded_path = exploded_textures_path.join(page);

    if let Err(err) = std::fs::create_dir_all(&texture_exploded_path) {
        log::error!("Failed to create directory for {}", err);
    }

    let png_filename = exploded_textures_path
        .join(page)
        .join(format!("{}.png", name));

    if let Some(texture) = cims.get(page) {
        let _ = texture.export_png(png_filename.as_path(), *x, *y, Some(*w), Some(*h));
    }
});

// export each page
new_textures.par_iter().for_each(|(page, texture)| {
    let path = exploded_textures_path.join(format!("{}.png", page));
    texture
        .export_png(path.as_path(), 0, 0, None, None)
        .unwrap();
});
let png_count = new_textures.len() + region_data.len();
let expected = regions.len() + get_patchable_cim_files().len() + 1;
log::info!(
    "Exported {} of expected {} pngs",
    png_count,
    expected
);



////////////////////////////////////////////// single threaded version
for region in &regions {
    let name: String = get_attribute_value_node(&region, "n")?;

    let x = get_attribute_value_node(&region, "x")?;
    let y = get_attribute_value_node(&region, "y")?;
    let w = get_attribute_value_node(&region, "w")?;
    let h = get_attribute_value_node(&region, "h")?;

    let page: String = get_attribute_value_node(&region, "t")?;

    //log::info!("\nw {}\nh {}\npage {}", w, h, page);

    if !cims.contains_key(&page) {
        let cim_filename = format!("{}.cim", page);
        log::info!("Unpacking textures {}", cim_filename);
        cims.insert(
            page.clone(),
            Texture::new(
                &core_path.join("library").join(&cim_filename),
                false,
                None,
                None,
            ),
        );
    }

    let texture_exploded_path = core_path
        .join("library")
        .join("textures.exploded")
        .join(&page);
    if let Err(_) = std::fs::create_dir_all(&texture_exploded_path) {
        log::error!("What happened?");
    }

    let png_filename = core_path
        .join("library")
        .join("textures.exploded")
        .join(&page)
        .join(format!("{}.png", name));

    cims.get(&page)
        .unwrap()
        .export_png(png_filename.as_path(), x, y, Some(w), Some(h))?;

    region_count += 1;
}

for (page, _) in &cims {
    let path = core_path
        .join("library")
        .join("textures.exploded")
        .join(format!("{}.png", page));
    cims.get(page).ok_or("Could not find page")?.export_png(
        path.as_path(),
        0,
        0,
        None,
        None,
    )?;

    page_count += 1;
}
*/
