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
use ConditionPassed::*;
use execute_aarch32_instrs_VCVT_hs_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCVT_hs_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_307957: bool,
        gs_307960: bool,
        half_to_single: bool,
        D: bool,
        size: u8,
        Vd: u8,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        op,
        M,
        Vm,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #1u : u8
        let s_2_2: u8 = 1;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: branch s_2_4 b14 b3
        if s_2_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op:u8
        let s_3_0: bool = fn_state.op;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var half_to_single <= s_3_4
        fn_state.half_to_single = s_3_4;
        // D s_3_6: read-var half_to_single:u8
        let s_3_6: bool = fn_state.half_to_single;
        // N s_3_7: branch s_3_6 b13 b4
        if s_3_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#307957 <= s_4_0
        fn_state.gs_307957 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#307957:u8
        let s_5_0: bool = fn_state.gs_307957;
        // N s_5_1: branch s_5_0 b12 b6
        if s_5_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var half_to_single:u8
        let s_6_0: bool = fn_state.half_to_single;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b11 b7
        if s_6_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#307960 <= s_7_0
        fn_state.gs_307960 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#307960:u8
        let s_8_0: bool = fn_state.gs_307960;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #4s : i64
        let s_9_0: i64 = 4;
        // D s_9_1: read-var M:u8
        let s_9_1: bool = fn_state.M;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 1u16);
        // D s_9_3: read-var Vm:u8
        let s_9_3: u8 = fn_state.Vm;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 4u16);
        // D s_9_5: cast reint s_9_2 -> u128
        let s_9_5: u128 = (s_9_2.value() as u128);
        // D s_9_6: size-of s_9_2
        let s_9_6: u16 = s_9_2.length();
        // D s_9_7: cast reint s_9_4 -> u128
        let s_9_7: u128 = (s_9_4.value() as u128);
        // D s_9_8: size-of s_9_4
        let s_9_8: u16 = s_9_4.length();
        // D s_9_9: lsl s_9_5 s_9_8
        let s_9_9: u128 = s_9_5 << s_9_8;
        // D s_9_10: or s_9_9 s_9_7
        let s_9_10: u128 = ((s_9_9) | (s_9_7));
        // D s_9_11: add s_9_6 s_9_8
        let s_9_11: u16 = (s_9_6 + s_9_8);
        // D s_9_12: create-bits s_9_10 s_9_11
        let s_9_12: Bits = Bits::new(s_9_10, s_9_11);
        // D s_9_13: cast reint s_9_12 -> u8
        let s_9_13: u8 = (s_9_12.value() as u8);
        // D s_9_14: cast zx s_9_13 -> bv
        let s_9_14: Bits = Bits::new(s_9_13 as u128, 5u16);
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (s_9_14.value() as i128);
        // D s_9_16: cast reint s_9_15 -> i64
        let s_9_16: i64 = (s_9_15 as i64);
        // D s_9_17: read-var D:u8
        let s_9_17: bool = fn_state.D;
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // D s_9_19: read-var Vd:u8
        let s_9_19: u8 = fn_state.Vd;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 4u16);
        // D s_9_21: cast reint s_9_18 -> u128
        let s_9_21: u128 = (s_9_18.value() as u128);
        // D s_9_22: size-of s_9_18
        let s_9_22: u16 = s_9_18.length();
        // D s_9_23: cast reint s_9_20 -> u128
        let s_9_23: u128 = (s_9_20.value() as u128);
        // D s_9_24: size-of s_9_20
        let s_9_24: u16 = s_9_20.length();
        // D s_9_25: lsl s_9_21 s_9_24
        let s_9_25: u128 = s_9_21 << s_9_24;
        // D s_9_26: or s_9_25 s_9_23
        let s_9_26: u128 = ((s_9_25) | (s_9_23));
        // D s_9_27: add s_9_22 s_9_24
        let s_9_27: u16 = (s_9_22 + s_9_24);
        // D s_9_28: create-bits s_9_26 s_9_27
        let s_9_28: Bits = Bits::new(s_9_26, s_9_27);
        // D s_9_29: cast reint s_9_28 -> u8
        let s_9_29: u8 = (s_9_28.value() as u8);
        // D s_9_30: cast zx s_9_29 -> bv
        let s_9_30: Bits = Bits::new(s_9_29 as u128, 5u16);
        // D s_9_31: cast zx s_9_30 -> i
        let s_9_31: i128 = (s_9_30.value() as i128);
        // D s_9_32: cast reint s_9_31 -> i64
        let s_9_32: i64 = (s_9_31 as i64);
        // D s_9_33: read-var half_to_single:u8
        let s_9_33: bool = fn_state.half_to_single;
        // D s_9_34: call execute_aarch32_instrs_VCVT_hs_Op_A_txt(s_9_32, s_9_0, s_9_33, s_9_16)
        let s_9_34: () = execute_aarch32_instrs_VCVT_hs_Op_A_txt(
            state,
            tracer,
            s_9_32,
            s_9_0,
            s_9_33,
            s_9_16,
        );
        // N s_9_35: return
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
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var Vm:u8
        let s_11_1: u8 = fn_state.Vm;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 4u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // D s_11_22: write-var gs#307960 <= s_11_21
        fn_state.gs_307960 = s_11_21;
        // N s_11_23: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vd:u8
        let s_13_1: u8 = fn_state.Vd;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#307957 <= s_13_21
        fn_state.gs_307957 = s_13_21;
        // N s_13_23: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
}
