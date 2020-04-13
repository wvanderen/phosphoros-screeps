use screeps::{find, prelude::*, Part, ResourceType, ReturnCode, RoomObjectProperties};

pub fn run () {
    debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        let name = creep.name();
        debug!("running creep {}", name);
        if creep.spawning() {
            continue;
        }
        if creep.memory().bool("taskAssigned") {
            
        }

        if creep.memory().bool("harvesting") {
            if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
                creep.memory().set("harvesting", false);
            }
        } else {
            if creep.store_used_capacity(None) == 0 {
                creep.memory().set("harvesting", true);
            }
        }

        if creep.memory().bool("harvesting") {
            let source = &creep.room().find(find::SOURCES)[1];
            if creep.pos().is_near_to(source) {
                let r = creep.harvest(source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(source);
            }
        } else {
            let target = &creep.room().find(find::MY_CONSTRUCTION_SITES)[0];
            if creep.pos().is_near_to(target) {
                let r =  creep.build(target);
                if r != ReturnCode::Ok {
                    warn!("couldn't build {:?}", r);
                } 
            } else {
                creep.move_to(target);
            }
        }
    }

}