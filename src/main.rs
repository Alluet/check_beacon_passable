use std::io;
use std::fs::{self, File};
use serde_json::{json, to_writer};

fn main() -> io::Result<()> {
    let predicate = json!({
        "condition": "minecraft:inverted",
        "term": {
            "condition": "minecraft:alternative",
            "terms": ({
                let mut list = Vec::with_capacity(255);
                for (skip, offset) in (1..256).rev().zip(1..256) {
                    list.push(json!({
                        "condition": "minecraft:inverted",
                        "term": {
                            "condition": "minecraft:alternative",
                            "terms": [
                                {
                                    "condition": "minecraft:location_check",
                                    "predicate": {
                                        "position": {
                                            "y": {
                                                "min": skip
                                            }
                                        }
                                    }
                                },
                                {
                                    "condition": "minecraft:location_check",
                                    "offsetY": offset,
                                    "predicate": {
                                        "block": {
                                            "tag": "check_beacon_passable:beacon_passable"
                                        }
                                    }
                                },
                                {
                                    "condition": "minecraft:location_check",
                                    "offsetY": offset,
                                    "predicate": {
                                        "block": {
                                            "tag": "check_beacon_passable:piston",
                                            "state": {
                                                "extended": true
                                            }
                                        }
                                    }
                                },
                                {
                                    "condition": "minecraft:location_check",
                                    "offsetY": offset,
                                    "predicate": {
                                        "block": {
                                            "tag": "minecraft:slabs",
                                            "state": {
                                                "type": "bottom"
                                            }
                                        }
                                    }
                                },
                                {
                                    "condition": "minecraft:location_check",
                                    "offsetY": offset,
                                    "predicate": {
                                        "block": {
                                            "tag": "minecraft:slabs",
                                            "state": {
                                                "type": "top"
                                            }
                                        }
                                    }
                                }
                            ]
                        }
                    }));
                }
                list
            }),
        }
    });

    fs::create_dir_all("./predicates")?;
    let file = File::create("./predicates/is_beacon_passable.json")?;
    to_writer(file, &predicate)?;

    Ok(())
}
