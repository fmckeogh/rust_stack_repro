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
use execute_aarch32_instrs_VPADD_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VPADD_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_314980: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vn,
        Vd,
        N,
        Q,
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
        // N s_2_5: branch s_2_4 b7 b3
        if s_2_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Q:u8
        let s_3_0: bool = fn_state.Q;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var gs#314980 <= s_3_4
        fn_state.gs_314980 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#314980:u8
        let s_4_0: bool = fn_state.gs_314980;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #8s : i64
        let s_5_4: i64 = 8;
        // D s_5_5: lsl s_5_4 s_5_3
        let s_5_5: i64 = s_5_4 << s_5_3;
        // C s_5_6: const #64s : i
        let s_5_6: i128 = 64;
        // D s_5_7: cast zx s_5_5 -> i
        let s_5_7: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_8: div s_5_6 s_5_7
        let s_5_8: i128 = ((s_5_6) / (s_5_7));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: read-var D:u8
        let s_5_10: bool = fn_state.D;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 1u16);
        // D s_5_12: read-var Vd:u8
        let s_5_12: u8 = fn_state.Vd;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 4u16);
        // D s_5_14: cast reint s_5_11 -> u128
        let s_5_14: u128 = (s_5_11.value() as u128);
        // D s_5_15: size-of s_5_11
        let s_5_15: u16 = s_5_11.length();
        // D s_5_16: cast reint s_5_13 -> u128
        let s_5_16: u128 = (s_5_13.value() as u128);
        // D s_5_17: size-of s_5_13
        let s_5_17: u16 = s_5_13.length();
        // D s_5_18: lsl s_5_14 s_5_17
        let s_5_18: u128 = s_5_14 << s_5_17;
        // D s_5_19: or s_5_18 s_5_16
        let s_5_19: u128 = ((s_5_18) | (s_5_16));
        // D s_5_20: add s_5_15 s_5_17
        let s_5_20: u16 = (s_5_15 + s_5_17);
        // D s_5_21: create-bits s_5_19 s_5_20
        let s_5_21: Bits = Bits::new(s_5_19, s_5_20);
        // D s_5_22: cast reint s_5_21 -> u8
        let s_5_22: u8 = (s_5_21.value() as u8);
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 5u16);
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (s_5_23.value() as i128);
        // D s_5_25: cast reint s_5_24 -> i64
        let s_5_25: i64 = (s_5_24 as i64);
        // D s_5_26: read-var N:u8
        let s_5_26: bool = fn_state.N;
        // D s_5_27: cast zx s_5_26 -> bv
        let s_5_27: Bits = Bits::new(s_5_26 as u128, 1u16);
        // D s_5_28: read-var Vn:u8
        let s_5_28: u8 = fn_state.Vn;
        // D s_5_29: cast zx s_5_28 -> bv
        let s_5_29: Bits = Bits::new(s_5_28 as u128, 4u16);
        // D s_5_30: cast reint s_5_27 -> u128
        let s_5_30: u128 = (s_5_27.value() as u128);
        // D s_5_31: size-of s_5_27
        let s_5_31: u16 = s_5_27.length();
        // D s_5_32: cast reint s_5_29 -> u128
        let s_5_32: u128 = (s_5_29.value() as u128);
        // D s_5_33: size-of s_5_29
        let s_5_33: u16 = s_5_29.length();
        // D s_5_34: lsl s_5_30 s_5_33
        let s_5_34: u128 = s_5_30 << s_5_33;
        // D s_5_35: or s_5_34 s_5_32
        let s_5_35: u128 = ((s_5_34) | (s_5_32));
        // D s_5_36: add s_5_31 s_5_33
        let s_5_36: u16 = (s_5_31 + s_5_33);
        // D s_5_37: create-bits s_5_35 s_5_36
        let s_5_37: Bits = Bits::new(s_5_35, s_5_36);
        // D s_5_38: cast reint s_5_37 -> u8
        let s_5_38: u8 = (s_5_37.value() as u8);
        // D s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 5u16);
        // D s_5_40: cast zx s_5_39 -> i
        let s_5_40: i128 = (s_5_39.value() as i128);
        // D s_5_41: cast reint s_5_40 -> i64
        let s_5_41: i64 = (s_5_40 as i64);
        // D s_5_42: read-var M:u8
        let s_5_42: bool = fn_state.M;
        // D s_5_43: cast zx s_5_42 -> bv
        let s_5_43: Bits = Bits::new(s_5_42 as u128, 1u16);
        // D s_5_44: read-var Vm:u8
        let s_5_44: u8 = fn_state.Vm;
        // D s_5_45: cast zx s_5_44 -> bv
        let s_5_45: Bits = Bits::new(s_5_44 as u128, 4u16);
        // D s_5_46: cast reint s_5_43 -> u128
        let s_5_46: u128 = (s_5_43.value() as u128);
        // D s_5_47: size-of s_5_43
        let s_5_47: u16 = s_5_43.length();
        // D s_5_48: cast reint s_5_45 -> u128
        let s_5_48: u128 = (s_5_45.value() as u128);
        // D s_5_49: size-of s_5_45
        let s_5_49: u16 = s_5_45.length();
        // D s_5_50: lsl s_5_46 s_5_49
        let s_5_50: u128 = s_5_46 << s_5_49;
        // D s_5_51: or s_5_50 s_5_48
        let s_5_51: u128 = ((s_5_50) | (s_5_48));
        // D s_5_52: add s_5_47 s_5_49
        let s_5_52: u16 = (s_5_47 + s_5_49);
        // D s_5_53: create-bits s_5_51 s_5_52
        let s_5_53: Bits = Bits::new(s_5_51, s_5_52);
        // D s_5_54: cast reint s_5_53 -> u8
        let s_5_54: u8 = (s_5_53.value() as u8);
        // D s_5_55: cast zx s_5_54 -> bv
        let s_5_55: Bits = Bits::new(s_5_54 as u128, 5u16);
        // D s_5_56: cast zx s_5_55 -> i
        let s_5_56: i128 = (s_5_55.value() as i128);
        // D s_5_57: cast reint s_5_56 -> i64
        let s_5_57: i64 = (s_5_56 as i64);
        // D s_5_58: cast zx s_5_5 -> i
        let s_5_58: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_59: cast reint s_5_58 -> i64
        let s_5_59: i64 = (s_5_58 as i64);
        // D s_5_60: cast zx s_5_9 -> i
        let s_5_60: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_61: call execute_aarch32_instrs_VPADD_i_Op_A_txt(s_5_25, s_5_60, s_5_59, s_5_57, s_5_41)
        let s_5_61: () = execute_aarch32_instrs_VPADD_i_Op_A_txt(
            state,
            tracer,
            s_5_25,
            s_5_60,
            s_5_59,
            s_5_57,
            s_5_41,
        );
        // N s_5_62: return
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
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#314980 <= s_7_0
        fn_state.gs_314980 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
