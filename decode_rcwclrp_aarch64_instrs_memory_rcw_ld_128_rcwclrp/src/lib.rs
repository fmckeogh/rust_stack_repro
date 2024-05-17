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
use HaveTHExt::*;
use Have128BitDescriptorExt::*;
use execute_aarch64_instrs_memory_rcw_ld_128_rcwclrp::*;
use common::*;
pub fn decode_rcwclrp_aarch64_instrs_memory_rcw_ld_128_rcwclrp<T: Tracer>(
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
        release: bool,
        gs_166027: bool,
        t: i64,
        t2: i64,
        op: u32,
        n: i64,
        acquire: bool,
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
        // S s_0_1: call Have128BitDescriptorExt(s_0_0)
        let s_0_1: bool = Have128BitDescriptorExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b12 b1
        if s_0_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveTHExt(s_1_0)
        let s_1_1: bool = HaveTHExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#166027 <= s_1_2
        fn_state.gs_166027 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#166027:u8
        let s_2_0: bool = fn_state.gs_166027;
        // N s_2_1: branch s_2_0 b11 b3
        if s_2_0 {
            return block_11(state, tracer, fn_state);
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
        // C s_3_2: const #31u : u8
        let s_3_2: u8 = 31;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b10 b4
        if s_3_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rt2:u8
        let s_4_0: u8 = fn_state.Rt2;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // C s_4_2: const #31u : u8
        let s_4_2: u8 = 31;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b9 b5
        if s_4_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rt:u8
        let s_5_0: u8 = fn_state.Rt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var t <= s_5_3
        fn_state.t = s_5_3;
        // C s_5_5: const #31s : i
        let s_5_5: i128 = 31;
        // D s_5_6: read-var t:i64
        let s_5_6: i64 = fn_state.t;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: call neq_int(s_5_7, s_5_5)
        let s_5_8: bool = neq_int(state, tracer, s_5_7, s_5_5);
        // N s_5_9: assert s_5_8
        let s_5_9: () = assert!(s_5_8);
        // D s_5_10: read-var Rt2:u8
        let s_5_10: u8 = fn_state.Rt2;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 5u16);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var t2 <= s_5_13
        fn_state.t2 = s_5_13;
        // D s_5_15: read-var Rn:u8
        let s_5_15: u8 = fn_state.Rn;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 5u16);
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (s_5_16.value() as i128);
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: write-var n <= s_5_18
        fn_state.n = s_5_18;
        // D s_5_20: read-var A:u8
        let s_5_20: bool = fn_state.A;
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 1u16);
        // C s_5_22: const #1u : u8
        let s_5_22: bool = true;
        // C s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 1u16);
        // D s_5_24: cmp-eq s_5_21 s_5_23
        let s_5_24: bool = ((s_5_21) == (s_5_23));
        // D s_5_25: write-var acquire <= s_5_24
        fn_state.acquire = s_5_24;
        // D s_5_26: read-var R:u8
        let s_5_26: bool = fn_state.R;
        // D s_5_27: cast zx s_5_26 -> bv
        let s_5_27: Bits = Bits::new(s_5_26 as u128, 1u16);
        // C s_5_28: const #1u : u8
        let s_5_28: bool = true;
        // C s_5_29: cast zx s_5_28 -> bv
        let s_5_29: Bits = Bits::new(s_5_28 as u128, 1u16);
        // D s_5_30: cmp-eq s_5_27 s_5_29
        let s_5_30: bool = ((s_5_27) == (s_5_29));
        // D s_5_31: write-var release <= s_5_30
        fn_state.release = s_5_30;
        // D s_5_32: read-var opc:u8
        let s_5_32: u8 = fn_state.opc;
        // D s_5_33: cast zx s_5_32 -> bv
        let s_5_33: Bits = Bits::new(s_5_32 as u128, 3u16);
        // C s_5_34: const #1u : u8
        let s_5_34: u8 = 1;
        // C s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 3u16);
        // D s_5_36: cmp-eq s_5_33 s_5_35
        let s_5_36: bool = ((s_5_33) == (s_5_35));
        // N s_5_37: branch s_5_36 b8 b6
        if s_5_36 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4u : u32
        let s_6_0: u32 = 4;
        // D s_6_1: write-var op <= s_6_0
        fn_state.op = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #31s : i
        let s_7_0: i128 = 31;
        // D s_7_1: read-var n:i64
        let s_7_1: i64 = fn_state.n;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call neq_int(s_7_2, s_7_0)
        let s_7_3: bool = neq_int(state, tracer, s_7_2, s_7_0);
        // D s_7_4: read-var acquire:u8
        let s_7_4: bool = fn_state.acquire;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: read-var op:u32
        let s_7_6: u32 = fn_state.op;
        // D s_7_7: read-var release:u8
        let s_7_7: bool = fn_state.release;
        // C s_7_8: const #0u : u8
        let s_7_8: bool = false;
        // D s_7_9: read-var t:i64
        let s_7_9: i64 = fn_state.t;
        // D s_7_10: read-var t2:i64
        let s_7_10: i64 = fn_state.t2;
        // D s_7_11: call execute_aarch64_instrs_memory_rcw_ld_128_rcwclrp(s_7_4, s_7_5, s_7_6, s_7_7, s_7_8, s_7_9, s_7_10, s_7_3)
        let s_7_11: () = execute_aarch64_instrs_memory_rcw_ld_128_rcwclrp(
            state,
            tracer,
            s_7_4,
            s_7_5,
            s_7_6,
            s_7_7,
            s_7_8,
            s_7_9,
            s_7_10,
            s_7_3,
        );
        // N s_7_12: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // D s_8_1: write-var op <= s_8_0
        fn_state.op = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
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
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#166027 <= s_12_0
        fn_state.gs_166027 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
