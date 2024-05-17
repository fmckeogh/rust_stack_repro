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
use neq_int::*;
use execute_aarch64_instrs_memory_rcws_swp::*;
use HaveTHExt::*;
use common::*;
pub fn decode_rcwsswp_aarch64_instrs_memory_rcws_swp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    opc: u8,
    o3: bool,
    Rs: u8,
    R: bool,
    A: bool,
    S: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        s: i64,
        n: i64,
        gs_166410: bool,
        Rt: u8,
        Rn: u8,
        opc: u8,
        o3: bool,
        Rs: u8,
        R: bool,
        A: bool,
        S: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        opc,
        o3,
        Rs,
        R,
        A,
        S,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveTHExt(s_0_0)
        let s_0_1: bool = HaveTHExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rt:u8
        let s_1_0: u8 = fn_state.Rt;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var t <= s_1_3
        fn_state.t = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var Rs:u8
        let s_1_10: u8 = fn_state.Rs;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var s <= s_1_13
        fn_state.s = s_1_13;
        // D s_1_15: read-var A:u8
        let s_1_15: bool = fn_state.A;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 1u16);
        // C s_1_17: const #1u : u8
        let s_1_17: bool = true;
        // C s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: cmp-eq s_1_16 s_1_18
        let s_1_19: bool = ((s_1_16) == (s_1_18));
        // N s_1_20: branch s_1_19 b4 b2
        if s_1_19 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#166410 <= s_2_0
        fn_state.gs_166410 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#166410:u8
        let s_3_0: bool = fn_state.gs_166410;
        // D s_3_1: read-var R:u8
        let s_3_1: bool = fn_state.R;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 1u16);
        // C s_3_3: const #1u : u8
        let s_3_3: bool = true;
        // C s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 1u16);
        // D s_3_5: cmp-eq s_3_2 s_3_4
        let s_3_5: bool = ((s_3_2) == (s_3_4));
        // C s_3_6: const #31s : i
        let s_3_6: i128 = 31;
        // D s_3_7: read-var n:i64
        let s_3_7: i64 = fn_state.n;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: call neq_int(s_3_8, s_3_6)
        let s_3_9: bool = neq_int(state, tracer, s_3_8, s_3_6);
        // D s_3_10: read-var n:i64
        let s_3_10: i64 = fn_state.n;
        // D s_3_11: read-var s:i64
        let s_3_11: i64 = fn_state.s;
        // C s_3_12: const #1u : u8
        let s_3_12: bool = true;
        // D s_3_13: read-var t:i64
        let s_3_13: i64 = fn_state.t;
        // D s_3_14: call execute_aarch64_instrs_memory_rcws_swp(s_3_0, s_3_10, s_3_5, s_3_11, s_3_12, s_3_13, s_3_9)
        let s_3_14: () = execute_aarch64_instrs_memory_rcws_swp(
            state,
            tracer,
            s_3_0,
            s_3_10,
            s_3_5,
            s_3_11,
            s_3_12,
            s_3_13,
            s_3_9,
        );
        // N s_3_15: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rt:u8
        let s_4_0: u8 = fn_state.Rt;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // C s_4_2: const #31u : u8
        let s_4_2: u8 = 31;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cmp-ne s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) != (s_4_3));
        // D s_4_5: write-var gs#166410 <= s_4_4
        fn_state.gs_166410 = s_4_4;
        // N s_4_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
