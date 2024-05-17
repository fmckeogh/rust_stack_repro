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
use execute_aarch32_instrs_VSUBHN_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSUBHN_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_322584: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vn,
        Vd,
        N,
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
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var Vn:u8
        let s_3_1: u8 = fn_state.Vn;
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
        // N s_3_22: branch s_3_21 b8 b4
        if s_3_21 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var Vm:u8
        let s_4_1: u8 = fn_state.Vm;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // D s_4_22: write-var gs#322584 <= s_4_21
        fn_state.gs_322584 = s_4_21;
        // N s_4_23: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#322584:u8
        let s_5_0: bool = fn_state.gs_322584;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #8s : i64
        let s_6_4: i64 = 8;
        // D s_6_5: lsl s_6_4 s_6_3
        let s_6_5: i64 = s_6_4 << s_6_3;
        // C s_6_6: const #64s : i
        let s_6_6: i128 = 64;
        // D s_6_7: cast zx s_6_5 -> i
        let s_6_7: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_8: div s_6_6 s_6_7
        let s_6_8: i128 = ((s_6_6) / (s_6_7));
        // D s_6_9: cast reint s_6_8 -> i64
        let s_6_9: i64 = (s_6_8 as i64);
        // D s_6_10: read-var D:u8
        let s_6_10: bool = fn_state.D;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 1u16);
        // D s_6_12: read-var Vd:u8
        let s_6_12: u8 = fn_state.Vd;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 4u16);
        // D s_6_14: cast reint s_6_11 -> u128
        let s_6_14: u128 = (s_6_11.value() as u128);
        // D s_6_15: size-of s_6_11
        let s_6_15: u16 = s_6_11.length();
        // D s_6_16: cast reint s_6_13 -> u128
        let s_6_16: u128 = (s_6_13.value() as u128);
        // D s_6_17: size-of s_6_13
        let s_6_17: u16 = s_6_13.length();
        // D s_6_18: lsl s_6_14 s_6_17
        let s_6_18: u128 = s_6_14 << s_6_17;
        // D s_6_19: or s_6_18 s_6_16
        let s_6_19: u128 = ((s_6_18) | (s_6_16));
        // D s_6_20: add s_6_15 s_6_17
        let s_6_20: u16 = (s_6_15 + s_6_17);
        // D s_6_21: create-bits s_6_19 s_6_20
        let s_6_21: Bits = Bits::new(s_6_19, s_6_20);
        // D s_6_22: cast reint s_6_21 -> u8
        let s_6_22: u8 = (s_6_21.value() as u8);
        // D s_6_23: cast zx s_6_22 -> bv
        let s_6_23: Bits = Bits::new(s_6_22 as u128, 5u16);
        // D s_6_24: cast zx s_6_23 -> i
        let s_6_24: i128 = (s_6_23.value() as i128);
        // D s_6_25: cast reint s_6_24 -> i64
        let s_6_25: i64 = (s_6_24 as i64);
        // D s_6_26: read-var N:u8
        let s_6_26: bool = fn_state.N;
        // D s_6_27: cast zx s_6_26 -> bv
        let s_6_27: Bits = Bits::new(s_6_26 as u128, 1u16);
        // D s_6_28: read-var Vn:u8
        let s_6_28: u8 = fn_state.Vn;
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 4u16);
        // D s_6_30: cast reint s_6_27 -> u128
        let s_6_30: u128 = (s_6_27.value() as u128);
        // D s_6_31: size-of s_6_27
        let s_6_31: u16 = s_6_27.length();
        // D s_6_32: cast reint s_6_29 -> u128
        let s_6_32: u128 = (s_6_29.value() as u128);
        // D s_6_33: size-of s_6_29
        let s_6_33: u16 = s_6_29.length();
        // D s_6_34: lsl s_6_30 s_6_33
        let s_6_34: u128 = s_6_30 << s_6_33;
        // D s_6_35: or s_6_34 s_6_32
        let s_6_35: u128 = ((s_6_34) | (s_6_32));
        // D s_6_36: add s_6_31 s_6_33
        let s_6_36: u16 = (s_6_31 + s_6_33);
        // D s_6_37: create-bits s_6_35 s_6_36
        let s_6_37: Bits = Bits::new(s_6_35, s_6_36);
        // D s_6_38: cast reint s_6_37 -> u8
        let s_6_38: u8 = (s_6_37.value() as u8);
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 5u16);
        // D s_6_40: cast zx s_6_39 -> i
        let s_6_40: i128 = (s_6_39.value() as i128);
        // D s_6_41: cast reint s_6_40 -> i64
        let s_6_41: i64 = (s_6_40 as i64);
        // D s_6_42: read-var M:u8
        let s_6_42: bool = fn_state.M;
        // D s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 1u16);
        // D s_6_44: read-var Vm:u8
        let s_6_44: u8 = fn_state.Vm;
        // D s_6_45: cast zx s_6_44 -> bv
        let s_6_45: Bits = Bits::new(s_6_44 as u128, 4u16);
        // D s_6_46: cast reint s_6_43 -> u128
        let s_6_46: u128 = (s_6_43.value() as u128);
        // D s_6_47: size-of s_6_43
        let s_6_47: u16 = s_6_43.length();
        // D s_6_48: cast reint s_6_45 -> u128
        let s_6_48: u128 = (s_6_45.value() as u128);
        // D s_6_49: size-of s_6_45
        let s_6_49: u16 = s_6_45.length();
        // D s_6_50: lsl s_6_46 s_6_49
        let s_6_50: u128 = s_6_46 << s_6_49;
        // D s_6_51: or s_6_50 s_6_48
        let s_6_51: u128 = ((s_6_50) | (s_6_48));
        // D s_6_52: add s_6_47 s_6_49
        let s_6_52: u16 = (s_6_47 + s_6_49);
        // D s_6_53: create-bits s_6_51 s_6_52
        let s_6_53: Bits = Bits::new(s_6_51, s_6_52);
        // D s_6_54: cast reint s_6_53 -> u8
        let s_6_54: u8 = (s_6_53.value() as u8);
        // D s_6_55: cast zx s_6_54 -> bv
        let s_6_55: Bits = Bits::new(s_6_54 as u128, 5u16);
        // D s_6_56: cast zx s_6_55 -> i
        let s_6_56: i128 = (s_6_55.value() as i128);
        // D s_6_57: cast reint s_6_56 -> i64
        let s_6_57: i64 = (s_6_56 as i64);
        // D s_6_58: cast zx s_6_5 -> i
        let s_6_58: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_59: cast reint s_6_58 -> i64
        let s_6_59: i64 = (s_6_58 as i64);
        // D s_6_60: cast zx s_6_9 -> i
        let s_6_60: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_61: call execute_aarch32_instrs_VSUBHN_Op_A_txt(s_6_25, s_6_60, s_6_59, s_6_57, s_6_41)
        let s_6_61: () = execute_aarch32_instrs_VSUBHN_Op_A_txt(
            state,
            tracer,
            s_6_25,
            s_6_60,
            s_6_59,
            s_6_57,
            s_6_41,
        );
        // N s_6_62: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#322584 <= s_8_0
        fn_state.gs_322584 = s_8_0;
        // N s_8_2: jump b5
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
}
