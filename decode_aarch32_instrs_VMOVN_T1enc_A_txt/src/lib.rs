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
use execute_aarch32_instrs_VMOVN_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOVN_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        D: bool,
        size: u8,
        Vd: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
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
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b6 b3
        if s_2_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var Vm:u8
        let s_3_1: u8 = fn_state.Vm;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #1u : u8
        let s_3_19: bool = true;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-eq s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) == (s_3_20));
        // N s_3_22: branch s_3_21 b5 b4
        if s_3_21 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #8s : i64
        let s_4_4: i64 = 8;
        // D s_4_5: lsl s_4_4 s_4_3
        let s_4_5: i64 = s_4_4 << s_4_3;
        // C s_4_6: const #64s : i
        let s_4_6: i128 = 64;
        // D s_4_7: cast zx s_4_5 -> i
        let s_4_7: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_8: div s_4_6 s_4_7
        let s_4_8: i128 = ((s_4_6) / (s_4_7));
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: read-var D:u8
        let s_4_10: bool = fn_state.D;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 1u16);
        // D s_4_12: read-var Vd:u8
        let s_4_12: u8 = fn_state.Vd;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 4u16);
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // D s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: lsl s_4_14 s_4_17
        let s_4_18: u128 = s_4_14 << s_4_17;
        // D s_4_19: or s_4_18 s_4_16
        let s_4_19: u128 = ((s_4_18) | (s_4_16));
        // D s_4_20: add s_4_15 s_4_17
        let s_4_20: u16 = (s_4_15 + s_4_17);
        // D s_4_21: create-bits s_4_19 s_4_20
        let s_4_21: Bits = Bits::new(s_4_19, s_4_20);
        // D s_4_22: cast reint s_4_21 -> u8
        let s_4_22: u8 = (s_4_21.value() as u8);
        // D s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 5u16);
        // D s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (s_4_23.value() as i128);
        // D s_4_25: cast reint s_4_24 -> i64
        let s_4_25: i64 = (s_4_24 as i64);
        // D s_4_26: read-var M:u8
        let s_4_26: bool = fn_state.M;
        // D s_4_27: cast zx s_4_26 -> bv
        let s_4_27: Bits = Bits::new(s_4_26 as u128, 1u16);
        // D s_4_28: read-var Vm:u8
        let s_4_28: u8 = fn_state.Vm;
        // D s_4_29: cast zx s_4_28 -> bv
        let s_4_29: Bits = Bits::new(s_4_28 as u128, 4u16);
        // D s_4_30: cast reint s_4_27 -> u128
        let s_4_30: u128 = (s_4_27.value() as u128);
        // D s_4_31: size-of s_4_27
        let s_4_31: u16 = s_4_27.length();
        // D s_4_32: cast reint s_4_29 -> u128
        let s_4_32: u128 = (s_4_29.value() as u128);
        // D s_4_33: size-of s_4_29
        let s_4_33: u16 = s_4_29.length();
        // D s_4_34: lsl s_4_30 s_4_33
        let s_4_34: u128 = s_4_30 << s_4_33;
        // D s_4_35: or s_4_34 s_4_32
        let s_4_35: u128 = ((s_4_34) | (s_4_32));
        // D s_4_36: add s_4_31 s_4_33
        let s_4_36: u16 = (s_4_31 + s_4_33);
        // D s_4_37: create-bits s_4_35 s_4_36
        let s_4_37: Bits = Bits::new(s_4_35, s_4_36);
        // D s_4_38: cast reint s_4_37 -> u8
        let s_4_38: u8 = (s_4_37.value() as u8);
        // D s_4_39: cast zx s_4_38 -> bv
        let s_4_39: Bits = Bits::new(s_4_38 as u128, 5u16);
        // D s_4_40: cast zx s_4_39 -> i
        let s_4_40: i128 = (s_4_39.value() as i128);
        // D s_4_41: cast reint s_4_40 -> i64
        let s_4_41: i64 = (s_4_40 as i64);
        // D s_4_42: cast zx s_4_5 -> i
        let s_4_42: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_43: cast reint s_4_42 -> i64
        let s_4_43: i64 = (s_4_42 as i64);
        // D s_4_44: cast zx s_4_9 -> i
        let s_4_44: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_45: call execute_aarch32_instrs_VMOVN_Op_A_txt(s_4_25, s_4_44, s_4_43, s_4_41)
        let s_4_45: () = execute_aarch32_instrs_VMOVN_Op_A_txt(
            state,
            tracer,
            s_4_25,
            s_4_44,
            s_4_43,
            s_4_41,
        );
        // N s_4_46: return
        return;
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
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
