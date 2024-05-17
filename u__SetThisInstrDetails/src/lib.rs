#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use common::*;
pub fn u__SetThisInstrDetails<T: Tracer>(
    state: &mut State,
    tracer: &T,
    enc: u32,
    opcode: u32,
    cond: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        enc: u32,
        opcode: u32,
        cond: u8,
    }
    let fn_state = FunctionState {
        enc,
        opcode,
        cond,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var enc:u32
        let s_0_0: u32 = fn_state.enc;
        // C s_0_1: const #13600u : u32
        let s_0_1: u32 = 13600;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<u32>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // D s_0_3: read-var opcode:u32
        let s_0_3: u32 = fn_state.opcode;
        // C s_0_4: const #104696u : u32
        let s_0_4: u32 = 104696;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<u32>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // D s_0_6: read-var cond:u8
        let s_0_6: u8 = fn_state.cond;
        // C s_0_7: const #21920u : u32
        let s_0_7: u32 = 21920;
        // N s_0_8: write-reg s_0_7 <= s_0_6
        let s_0_8: () = {
            state.write_register::<u8>(s_0_7 as isize, s_0_6);
            tracer.write_register(s_0_7 as isize, s_0_6);
        };
        // N s_0_9: return
        return;
    }
}
