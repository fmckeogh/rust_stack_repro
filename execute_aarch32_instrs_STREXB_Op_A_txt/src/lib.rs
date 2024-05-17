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
use R_read::*;
use AArch32_ExclusiveMonitorsPass::*;
use R_set::*;
use MemA_set::*;
use common::*;
pub fn execute_aarch32_instrs_STREXB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_348002: bool,
        address: u32,
        d: i64,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // D s_0_5: read-var address:u32
        let s_0_5: u32 = fn_state.address;
        // D s_0_6: call AArch32_ExclusiveMonitorsPass(s_0_5, s_0_4)
        let s_0_6: bool = AArch32_ExclusiveMonitorsPass(state, tracer, s_0_5, s_0_4);
        // D s_0_7: write-var ga#348002 <= s_0_6
        fn_state.ga_348002 = s_0_6;
        // N s_0_8: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var ga#348002:u8
        let s_1_0: bool = fn_state.ga_348002;
        // N s_1_1: branch s_1_0 b3 b2
        if s_1_0 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #32s : i
        let s_2_0: i128 = 32;
        // C s_2_1: const #1u : u8
        let s_2_1: bool = true;
        // C s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // D s_2_3: bits-cast zx s_2_2 -> bv length s_2_0
        let s_2_3: Bits = s_2_2.zero_extend(s_2_0);
        // D s_2_4: cast reint s_2_3 -> u32
        let s_2_4: u32 = (s_2_3.value() as u32);
        // D s_2_5: read-var d:i64
        let s_2_5: i64 = fn_state.d;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: call R_set(s_2_6, s_2_4)
        let s_2_7: () = R_set(state, tracer, s_2_6, s_2_4);
        // N s_2_8: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var t:i64
        let s_3_0: i64 = fn_state.t;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // C s_3_5: const #1s : i64
        let s_3_5: i64 = 1;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_7: const #7s : i
        let s_3_7: i128 = 7;
        // C s_3_8: add s_3_7 s_3_6
        let s_3_8: i128 = (s_3_7 + s_3_6);
        // D s_3_9: bit-extract s_3_4 s_3_3 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_4) >> (s_3_3)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u8
        let s_3_10: u8 = (s_3_9.value() as u8);
        // C s_3_11: const #1s : i
        let s_3_11: i128 = 1;
        // D s_3_12: cast zx s_3_10 -> bv
        let s_3_12: Bits = Bits::new(s_3_10 as u128, 8u16);
        // D s_3_13: read-var address:u32
        let s_3_13: u32 = fn_state.address;
        // D s_3_14: call MemA_set(s_3_13, s_3_11, s_3_12)
        let s_3_14: () = MemA_set(state, tracer, s_3_13, s_3_11, s_3_12);
        // N s_3_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #32s : i
        let s_4_0: i128 = 32;
        // C s_4_1: const #0u : u8
        let s_4_1: bool = false;
        // C s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 1u16);
        // D s_4_3: bits-cast zx s_4_2 -> bv length s_4_0
        let s_4_3: Bits = s_4_2.zero_extend(s_4_0);
        // D s_4_4: cast reint s_4_3 -> u32
        let s_4_4: u32 = (s_4_3.value() as u32);
        // D s_4_5: read-var d:i64
        let s_4_5: i64 = fn_state.d;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call R_set(s_4_6, s_4_4)
        let s_4_7: () = R_set(state, tracer, s_4_6, s_4_4);
        // N s_4_8: return
        return;
    }
}
