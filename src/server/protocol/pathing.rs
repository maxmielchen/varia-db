use std::io::{Error, ErrorKind};

pub fn put_pathing(path: String) -> Result<String, Error> {
    let segments = path.split("/");
    let slice_all = segments.clone().collect::<Vec<&str>>();
    if slice_all.len() != 3 {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    if slice_all.get(1).unwrap() != &"put" {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    Ok(slice_all.get(2).unwrap().to_string())
}

pub enum GetPathing {
    Get(String),
    List,
}

pub fn get_pathing(path: String) -> Result<GetPathing, Error> {
    let segments = path.split("/");
    let slice_all = segments.clone().collect::<Vec<&str>>();
    let operator = slice_all.get(1);
    if operator.is_none() {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    let operator = operator.unwrap();
    match operator {
        &"get" => {
            if slice_all.len() != 3 {
                return Err(
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid path: {}", path),
                    ),
                );
            }
            Ok(GetPathing::Get(slice_all.get(2).unwrap().to_string()))
        },
        &"list" => {
            Ok(GetPathing::List)
        },
        _ => {
            Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid path: {}", path),
                ),
            )
        }
    }
}

pub fn del_pathing(path: String) -> Result<String, Error> {
    let segments = path.split("/");
    let slice_all = segments.clone().collect::<Vec<&str>>();
    if slice_all.len() != 3 {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    if slice_all.get(1).unwrap() != &"del" {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    Ok(slice_all[2].to_string())
}