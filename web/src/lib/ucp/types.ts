export const item_types = {
    1: ["pótlék", "pótlékok"],
    2: ["leintés", "leintések"],
    3: ["számla", "számlák"],
};

export const item_statuses = {
    1: "feltöltve",
    2: "elfogadva",
    3: "elutasítva",
};

export function get_type_number(tip: string) {
    switch (tip) {
        case "pótlék":
            return 1;
        case "leintés":
            return 2;
        case "számla":
            return 3;
        default:
            return 0;
    }
}

export function get_type_string(tip: number) {
    switch (tip) {
        case 1:
            return "pótlék";
        case 2:
            return "leintés";
        case 3:
            return "számla";
        default:
            return "hiba";
    }
}

export function get_type_en_string(tip: number) {
    switch (tip) {
        case 1:
            return "supplements";
        case 2:
            return "hails";
        case 3:
            return "bills";
        default:
            return "supplements";
    }
}

export function get_status_number(tip: string) {
    switch (tip) {
        case "feltöltve":
            return 1;
        case "elfogadva":
            return 2;
        case "elutasítva":
            return 3;
        default:
            return 0;
    }
}
export function get_status_string(tip: number) {
    switch (tip) {
        case 1:
            return "feltöltve";
        case 2:
            return "elfogadva";
        case 3:
            return "elutasítva";
        default:
            return "hiba";
    }
}
