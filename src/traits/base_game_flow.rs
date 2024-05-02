use crate::traits::controler::Control;
use crate::traits::update::Update;
use crate::traits::draw::Draw;

pub trait BaseGameFlow: Update + Draw + Control {}