use std::sync::Arc;

use crate::objects;

use super::{GameObjectId, GameObjectRef, GameObjectType};

pub trait GameObject {
    fn get_id(&self) -> GameObjectId;
    fn get_object_type(&self) -> GameObjectType;
    fn get_name(&self) -> &str;

    fn get_adjective(&self) -> &str {
        ""
    }

    fn get_definite_article(&self) -> &str {
        ""
    }

    fn get_plural(&self) -> String {
        self.get_name().to_owned()
    }

    fn get_ref(&self) -> GameObjectRef {
        GameObjectRef(self.get_object_type(), self.get_id())
    }

    fn to_class(&self) -> Option<objects::Class> {
        None
    }

    fn to_item(&self) -> Option<objects::Item> {
        None
    }

    fn to_player(&self) -> Option<objects::Player> {
        None
    }

    fn to_portal(&self) -> Option<objects::Portal> {
        None
    }

    fn to_race(&self) -> Option<objects::Race> {
        None
    }

    fn to_realm(&self) -> Option<objects::Realm> {
        None
    }

    fn to_room(&self) -> Option<objects::Room> {
        None
    }
}

pub fn hydrate(object_ref: GameObjectRef, content: &str) -> Result<Arc<dyn GameObject>, String> {
    let hydrate = match object_ref.get_type() {
        GameObjectType::Class => objects::Class::hydrate,
        GameObjectType::Item => objects::Item::hydrate,
        GameObjectType::Player => objects::Player::hydrate,
        GameObjectType::Portal => objects::Portal::hydrate,
        GameObjectType::Race => objects::Race::hydrate,
        GameObjectType::Room => objects::Room::hydrate,
        _ => panic!("not implemented"),
    };

    hydrate(object_ref.get_id(), content)
}
