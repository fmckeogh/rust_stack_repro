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
pub fn execute_aarch32_instrs_LDREXH_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        gs_879988: Bits,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
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
        // D s_0_3: write-var address <= s_0_2
        fn_state.address = s_0_2;
        // C s_0_4: const #2s : i
        let s_0_4: i128 = 2;
        // D s_0_5: read-var address:u32
        let s_0_5: u32 = fn_state.address;
        // D s_0_6: call AArch32_SetExclusiveMonitors(s_0_5, s_0_4)
        let s_0_6: () = AArch32_SetExclusiveMonitors(state, tracer, s_0_5, s_0_4);
        // N s_0_7: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i64
        let s_1_0: i64 = 2;
        // D s_1_1: read-var address:u32
        let s_1_1: u32 = fn_state.address;
        // D s_1_2: call MemA_read(s_1_1, s_1_0)
        let s_1_2: Bits = MemA_read(state, tracer, s_1_1, s_1_0);
        // D s_1_3: write-var gs#879988 <= s_1_2
        fn_state.gs_879988 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#879988:bv
        let s_2_0: Bits = fn_state.gs_879988;
        // D s_2_1: cast reint s_2_0 -> u16
        let s_2_1: u16 = (s_2_0.value() as u16);
        // C s_2_2: const #32s : i
        let s_2_2: i128 = 32;
        // D s_2_3: cast zx s_2_1 -> bv
        let s_2_3: Bits = Bits::new(s_2_1 as u128, 16u16);
        // D s_2_4: bits-cast zx s_2_3 -> bv length s_2_2
        let s_2_4: Bits = s_2_3.zero_extend(s_2_2);
        // D s_2_5: cast reint s_2_4 -> u32
        let s_2_5: u32 = (s_2_4.value() as u32);
        // D s_2_6: read-var t:i64
        let s_2_6: i64 = fn_state.t;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: call R_set(s_2_7, s_2_5)
        let s_2_8: () = R_set(state, tracer, s_2_7, s_2_5);
        // N s_2_9: return
        return;
    }
}
