use crate::traits::controler::Control;
use crate::traits::draw::Draw;
use crate::traits::update::Update;

pub trait BaseGameFlow: Update + Draw + Control {}
