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
use MemA_read::*;
use R_read::*;
use R_set::*;
use AArch32_SetExclusiveMonitors::*;
use common::*;
pub fn execute_aarch32_instrs_LDREX_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_879911: Bits,
        address: u32,
        imm32: u32,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        imm32,
        n,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var imm32:u32
        let s_0_4: u32 = fn_state.imm32;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_6: add s_0_3 s_0_5
        let s_0_6: Bits = (s_0_3 + s_0_5);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: write-var address <= s_0_7
        fn_state.address = s_0_7;
        // C s_0_9: const #4s : i
        let s_0_9: i128 = 4;
        // D s_0_10: read-var address:u32
        let s_0_10: u32 = fn_state.address;
        // D s_0_11: call AArch32_SetExclusiveMonitors(s_0_10, s_0_9)
        let s_0_11: () = AArch32_SetExclusiveMonitors(state, tracer, s_0_10, s_0_9);
        // N s_0_12: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #4s : i64
        let s_1_0: i64 = 4;
        // D s_1_1: read-var address:u32
        let s_1_1: u32 = fn_state.address;
        // D s_1_2: call MemA_read(s_1_1, s_1_0)
        let s_1_2: Bits = MemA_read(state, tracer, s_1_1, s_1_0);
        // D s_1_3: write-var gs#879911 <= s_1_2
        fn_state.gs_879911 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#879911:bv
        let s_2_0: Bits = fn_state.gs_879911;
        // D s_2_1: cast reint s_2_0 -> u32
        let s_2_1: u32 = (s_2_0.value() as u32);
        // D s_2_2: read-var t:i64
        let s_2_2: i64 = fn_state.t;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call R_set(s_2_3, s_2_1)
        let s_2_4: () = R_set(state, tracer, s_2_3, s_2_1);
        // N s_2_5: return
        return;
    }
}
