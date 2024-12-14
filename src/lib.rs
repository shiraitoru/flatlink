use std::{error::Error, fs, path};

struct Link {
    src_path: path::PathBuf,
    link_name: String,
}

pub fn link(src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    let _ = dst;

    let mut name_list: Vec<Link> = Vec::new();
    search_file_list(src, &mut name_list)?;
    path_to_link_name(src, &mut name_list)?;

    for name in name_list.iter() {
        println!(
            "path: {}, name: {}",
            name.src_path.to_str().unwrap(),
            name.link_name
        );
    }

    create_hard_link(&name_list, dst)?;

    Ok(())
}

fn create_hard_link(name_list: &Vec<Link>, dst: &str) -> Result<(), Box<dyn Error>> {
    for item in name_list {
        let link_path = path::PathBuf::from(dst);
        let link_path = link_path.join(&item.link_name);
        if !link_path.is_file() {
            fs::hard_link(&item.src_path, link_path)?;
        }
    }

    Ok(())
}

fn path_to_link_name(base_dir: &str, name_list: &mut Vec<Link>) -> Result<(), Box<dyn Error>> {
    for item in name_list {
        let relative_path = item.src_path.strip_prefix(base_dir)?;

        let mut link_name = String::new();
        for elem in relative_path {
            link_name += &("_".to_string() + elem.to_str().unwrap_or_default());
        }

        item.link_name = link_name[1..].to_string();
    }

    Ok(())
}

fn search_file_list(dir: &str, name_list: &mut Vec<Link>) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(dir)?;
    for item in dir.into_iter() {
        let path = item?.path();
        if path.is_dir() {
            let curr_dir = path.to_str().unwrap();
            search_file_list(curr_dir, name_list)?;
        } else {
            name_list.push(Link {
                src_path: path,
                link_name: "".to_string(),
            });
        }
    }
    Ok(())
}
