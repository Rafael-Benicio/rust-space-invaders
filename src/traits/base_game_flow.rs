use crate::traits::controler::Control;
use crate::traits::draw::Draw;
use crate::traits::update::Update;

use crate::Uuid;

pub trait BaseGameFlow: Update + Draw + Control {
    fn get_id(&self) -> Uuid;
}
