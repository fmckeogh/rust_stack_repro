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
use execute_aarch64_instrs_memory_rcw_ld_rcwclr::*;
use HaveTHExt::*;
use common::*;
pub fn decode_rcwclr_aarch64_instrs_memory_rcw_ld_rcwclr<T: Tracer>(
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
        release: bool,
        gs_165984: bool,
        t: i64,
        s: i64,
        op: u32,
        n: i64,
        acquire: bool,
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
        // N s_0_3: branch s_0_2 b8 b1
        if s_0_2 {
            return block_8(state, tracer, fn_state);
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
        // N s_1_20: branch s_1_19 b7 b2
        if s_1_19 {
            return block_7(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#165984 <= s_2_0
        fn_state.gs_165984 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#165984:u8
        let s_3_0: bool = fn_state.gs_165984;
        // D s_3_1: write-var acquire <= s_3_0
        fn_state.acquire = s_3_0;
        // D s_3_2: read-var R:u8
        let s_3_2: bool = fn_state.R;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: write-var release <= s_3_6
        fn_state.release = s_3_6;
        // D s_3_8: read-var opc:u8
        let s_3_8: u8 = fn_state.opc;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 3u16);
        // C s_3_10: const #1u : u8
        let s_3_10: u8 = 1;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 3u16);
        // D s_3_12: cmp-eq s_3_9 s_3_11
        let s_3_12: bool = ((s_3_9) == (s_3_11));
        // N s_3_13: branch s_3_12 b6 b4
        if s_3_12 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #4u : u32
        let s_4_0: u32 = 4;
        // D s_4_1: write-var op <= s_4_0
        fn_state.op = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call neq_int(s_5_2, s_5_0)
        let s_5_3: bool = neq_int(state, tracer, s_5_2, s_5_0);
        // D s_5_4: read-var acquire:u8
        let s_5_4: bool = fn_state.acquire;
        // D s_5_5: read-var n:i64
        let s_5_5: i64 = fn_state.n;
        // D s_5_6: read-var op:u32
        let s_5_6: u32 = fn_state.op;
        // D s_5_7: read-var release:u8
        let s_5_7: bool = fn_state.release;
        // D s_5_8: read-var s:i64
        let s_5_8: i64 = fn_state.s;
        // C s_5_9: const #0u : u8
        let s_5_9: bool = false;
        // D s_5_10: read-var t:i64
        let s_5_10: i64 = fn_state.t;
        // D s_5_11: call execute_aarch64_instrs_memory_rcw_ld_rcwclr(s_5_4, s_5_5, s_5_6, s_5_7, s_5_8, s_5_9, s_5_10, s_5_3)
        let s_5_11: () = execute_aarch64_instrs_memory_rcw_ld_rcwclr(
            state,
            tracer,
            s_5_4,
            s_5_5,
            s_5_6,
            s_5_7,
            s_5_8,
            s_5_9,
            s_5_10,
            s_5_3,
        );
        // N s_5_12: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
        // D s_6_1: write-var op <= s_6_0
        fn_state.op = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Rt:u8
        let s_7_0: u8 = fn_state.Rt;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 5u16);
        // C s_7_2: const #31u : u8
        let s_7_2: u8 = 31;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 5u16);
        // D s_7_4: cmp-ne s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) != (s_7_3));
        // D s_7_5: write-var gs#165984 <= s_7_4
        fn_state.gs_165984 = s_7_4;
        // N s_7_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
