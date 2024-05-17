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
use HaveLSE128::*;
use execute_aarch64_instrs_memory_atomicops_ld_128_ldsetp::*;
use common::*;
pub fn decode_ldsetp_aarch64_instrs_memory_atomicops_ld_128_ldsetp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    opc: u8,
    o3: bool,
    Rt2: u8,
    R: bool,
    A: bool,
    S: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        t2: i64,
        op: u32,
        n: i64,
        Rt: u8,
        Rn: u8,
        opc: u8,
        o3: bool,
        Rt2: u8,
        R: bool,
        A: bool,
        S: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        opc,
        o3,
        Rt2,
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
        // S s_0_1: call HaveLSE128(s_0_0)
        let s_0_1: bool = HaveLSE128(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b11 b1
        if s_0_2 {
            return block_11(state, tracer, fn_state);
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
        // C s_1_2: const #31u : u8
        let s_1_2: u8 = 31;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 5u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b10 b2
        if s_1_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rt2:u8
        let s_2_0: u8 = fn_state.Rt2;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 5u16);
        // C s_2_2: const #31u : u8
        let s_2_2: u8 = 31;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 5u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b9 b3
        if s_2_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // C s_3_5: const #31s : i
        let s_3_5: i128 = 31;
        // D s_3_6: read-var t:i64
        let s_3_6: i64 = fn_state.t;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: call neq_int(s_3_7, s_3_5)
        let s_3_8: bool = neq_int(state, tracer, s_3_7, s_3_5);
        // N s_3_9: assert s_3_8
        let s_3_9: () = assert!(s_3_8);
        // D s_3_10: read-var Rt2:u8
        let s_3_10: u8 = fn_state.Rt2;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 5u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var t2 <= s_3_13
        fn_state.t2 = s_3_13;
        // D s_3_15: read-var Rn:u8
        let s_3_15: u8 = fn_state.Rn;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 5u16);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (s_3_16.value() as i128);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: write-var n <= s_3_18
        fn_state.n = s_3_18;
        // D s_3_20: read-var opc:u8
        let s_3_20: u8 = fn_state.opc;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 3u16);
        // C s_3_22: const #1u : u8
        let s_3_22: u8 = 1;
        // C s_3_23: cast zx s_3_22 -> bv
        let s_3_23: Bits = Bits::new(s_3_22 as u128, 3u16);
        // D s_3_24: cmp-eq s_3_21 s_3_23
        let s_3_24: bool = ((s_3_21) == (s_3_23));
        // D s_3_25: not s_3_24
        let s_3_25: bool = !s_3_24;
        // N s_3_26: branch s_3_25 b6 b4
        if s_3_25 {
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
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
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
        // D s_5_0: read-var A:u8
        let s_5_0: bool = fn_state.A;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: read-var R:u8
        let s_5_5: bool = fn_state.R;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 1u16);
        // C s_5_7: const #1u : u8
        let s_5_7: bool = true;
        // C s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 1u16);
        // D s_5_9: cmp-eq s_5_6 s_5_8
        let s_5_9: bool = ((s_5_6) == (s_5_8));
        // C s_5_10: const #31s : i
        let s_5_10: i128 = 31;
        // D s_5_11: read-var n:i64
        let s_5_11: i64 = fn_state.n;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: call neq_int(s_5_12, s_5_10)
        let s_5_13: bool = neq_int(state, tracer, s_5_12, s_5_10);
        // D s_5_14: read-var n:i64
        let s_5_14: i64 = fn_state.n;
        // D s_5_15: read-var op:u32
        let s_5_15: u32 = fn_state.op;
        // D s_5_16: read-var t:i64
        let s_5_16: i64 = fn_state.t;
        // D s_5_17: read-var t2:i64
        let s_5_17: i64 = fn_state.t2;
        // D s_5_18: call execute_aarch64_instrs_memory_atomicops_ld_128_ldsetp(s_5_4, s_5_14, s_5_15, s_5_9, s_5_16, s_5_17, s_5_13)
        let s_5_18: () = execute_aarch64_instrs_memory_atomicops_ld_128_ldsetp(
            state,
            tracer,
            s_5_4,
            s_5_14,
            s_5_15,
            s_5_9,
            s_5_16,
            s_5_17,
            s_5_13,
        );
        // N s_5_19: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var opc:u8
        let s_6_0: u8 = fn_state.opc;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 3u16);
        // C s_6_2: const #3u : u8
        let s_6_2: u8 = 3;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 3u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #4u : u32
        let s_7_0: u32 = 4;
        // D s_7_1: write-var op <= s_7_0
        fn_state.op = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
}
