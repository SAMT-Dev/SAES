use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Status {
    pub id: i8,
    pub display: String,
}

#[derive(Debug, Serialize)]
pub struct Statuses {
    pub uploaded: Status,
    pub accepted: Status,
    pub rejected: Status,
}

pub fn get_statuses() -> Statuses {
    Statuses {
        uploaded: Status {
            id: 1,
            display: "feltöltve".to_string(),
        },
        accepted: Status {
            id: 2,
            display: "feltöltve".to_string(),
        },
        rejected: Status {
            id: 3,
            display: "elutasítva".to_string(),
        },
    }
}

#[derive(Debug, Serialize)]
pub struct Tip {
    pub id: i8,
    pub display: String,
    pub plural: String,
}

#[derive(Debug, Serialize)]
pub struct Types {
    pub supplements: Tip, // pótlék
    pub hails: Tip,       // leintés
    pub bills: Tip,       // számla
}

pub fn get_types() -> Types {
    Types {
        supplements: Tip {
            id: 1,
            display: "pótlék".to_string(),
            plural: "pótlékok".to_string(),
        },
        hails: Tip {
            id: 2,
            display: "leintés".to_string(),
            plural: "leintések".to_string(),
        },
        bills: Tip {
            id: 3,
            display: "számla".to_string(),
            plural: "számlák".to_string(),
        },
    }
}

pub fn get_types_as_list() -> Vec<i8> {
    vec![1, 2, 3]
}

pub fn get_statuses_as_list() -> Vec<i8> {
    vec![1, 2, 3]
}
