use std::collections::VecDeque;

mod protocol;

#[derive(PartialEq, Eq)]
pub enum PollType { PollSendable, PollRecvable }
pub type PollCallback = fn();

pub struct RitmInstance
{
    pub sendable_cb: VecDeque<PollCallback>,
    pub recvable_cb: VecDeque<PollCallback>,
}

#[no_mangle]
pub unsafe extern fn ritm_init(pp_context: *mut *mut RitmInstance)
{
    
}

#[no_mangle]
pub unsafe extern fn ritm_free(pp_context: *mut *mut RitmInstance)
{
    
}

#[no_mangle]
pub unsafe extern fn ritm_poll(pp_context: *mut *mut RitmInstance, ptp: PollType, pcb: PollCallback)
{
    let ref mut context = **pp_context;
    if ptp == PollType::PollSendable
    {
        context.sendable_cb.push_back(pcb);
    }

    if ptp == PollType::PollRecvable
    {
        context.recvable_cb.push_back(pcb);
    }

    return;
}

#[no_mangle]
pub unsafe extern fn ritm_send(pp_context: *mut *mut RitmInstance)
{
    
}

#[no_mangle]
pub unsafe extern fn ritm_recv(pp_context: *mut *mut RitmInstance)
{

}