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
use execute_aarch32_instrs_VQMOVN_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VQMOVN_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    op: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_316119: bool,
        D: bool,
        size: u8,
        Vd: u8,
        op: u8,
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
        // D s_2_0: read-var op:u8
        let s_2_0: u8 = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
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
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #3u : u8
        let s_3_2: u8 = 3;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b8 b4
        if s_3_4 {
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
        // D s_4_22: write-var gs#316119 <= s_4_21
        fn_state.gs_316119 = s_4_21;
        // N s_4_23: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#316119:u8
        let s_5_0: bool = fn_state.gs_316119;
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
        // D s_6_0: read-var op:u8
        let s_6_0: u8 = fn_state.op;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #3u : u8
        let s_6_2: u8 = 3;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // C s_6_5: const #0s : i
        let s_6_5: i128 = 0;
        // D s_6_6: read-var op:u8
        let s_6_6: u8 = fn_state.op;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 2u16);
        // C s_6_8: const #1u : u64
        let s_6_8: u64 = 1;
        // D s_6_9: bit-extract s_6_7 s_6_5 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_7) >> (s_6_5)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: bool = ((s_6_9.value()) != 0);
        // C s_6_11: const #0s : i
        let s_6_11: i128 = 0;
        // C s_6_12: const #0u : u64
        let s_6_12: u64 = 0;
        // D s_6_13: cast zx s_6_10 -> u64
        let s_6_13: u64 = (s_6_10 as u64);
        // C s_6_14: const #1u : u64
        let s_6_14: u64 = 1;
        // D s_6_15: and s_6_13 s_6_14
        let s_6_15: u64 = ((s_6_13) & (s_6_14));
        // D s_6_16: cmp-eq s_6_15 s_6_14
        let s_6_16: bool = ((s_6_15) == (s_6_14));
        // D s_6_17: lsl s_6_13 s_6_11
        let s_6_17: u64 = s_6_13 << s_6_11;
        // D s_6_18: or s_6_12 s_6_17
        let s_6_18: u64 = ((s_6_12) | (s_6_17));
        // D s_6_19: cmpl s_6_17
        let s_6_19: u64 = !s_6_17;
        // D s_6_20: and s_6_12 s_6_19
        let s_6_20: u64 = ((s_6_12) & (s_6_19));
        // D s_6_21: select s_6_16 s_6_18 s_6_20
        let s_6_21: u64 = if s_6_16 { s_6_18 } else { s_6_20 };
        // D s_6_22: cast trunc s_6_21 -> u8
        let s_6_22: bool = ((s_6_21) != 0);
        // D s_6_23: cast zx s_6_22 -> bv
        let s_6_23: Bits = Bits::new(s_6_22 as u128, 1u16);
        // C s_6_24: const #1u : u8
        let s_6_24: bool = true;
        // C s_6_25: cast zx s_6_24 -> bv
        let s_6_25: Bits = Bits::new(s_6_24 as u128, 1u16);
        // D s_6_26: cmp-eq s_6_23 s_6_25
        let s_6_26: bool = ((s_6_23) == (s_6_25));
        // D s_6_27: read-var size:u8
        let s_6_27: u8 = fn_state.size;
        // D s_6_28: cast zx s_6_27 -> bv
        let s_6_28: Bits = Bits::new(s_6_27 as u128, 2u16);
        // D s_6_29: cast zx s_6_28 -> i
        let s_6_29: i128 = (s_6_28.value() as i128);
        // D s_6_30: cast reint s_6_29 -> i64
        let s_6_30: i64 = (s_6_29 as i64);
        // C s_6_31: const #8s : i64
        let s_6_31: i64 = 8;
        // D s_6_32: lsl s_6_31 s_6_30
        let s_6_32: i64 = s_6_31 << s_6_30;
        // C s_6_33: const #64s : i
        let s_6_33: i128 = 64;
        // D s_6_34: cast zx s_6_32 -> i
        let s_6_34: i128 = (i128::try_from(s_6_32).unwrap());
        // D s_6_35: div s_6_33 s_6_34
        let s_6_35: i128 = ((s_6_33) / (s_6_34));
        // D s_6_36: cast reint s_6_35 -> i64
        let s_6_36: i64 = (s_6_35 as i64);
        // D s_6_37: read-var D:u8
        let s_6_37: bool = fn_state.D;
        // D s_6_38: cast zx s_6_37 -> bv
        let s_6_38: Bits = Bits::new(s_6_37 as u128, 1u16);
        // D s_6_39: read-var Vd:u8
        let s_6_39: u8 = fn_state.Vd;
        // D s_6_40: cast zx s_6_39 -> bv
        let s_6_40: Bits = Bits::new(s_6_39 as u128, 4u16);
        // D s_6_41: cast reint s_6_38 -> u128
        let s_6_41: u128 = (s_6_38.value() as u128);
        // D s_6_42: size-of s_6_38
        let s_6_42: u16 = s_6_38.length();
        // D s_6_43: cast reint s_6_40 -> u128
        let s_6_43: u128 = (s_6_40.value() as u128);
        // D s_6_44: size-of s_6_40
        let s_6_44: u16 = s_6_40.length();
        // D s_6_45: lsl s_6_41 s_6_44
        let s_6_45: u128 = s_6_41 << s_6_44;
        // D s_6_46: or s_6_45 s_6_43
        let s_6_46: u128 = ((s_6_45) | (s_6_43));
        // D s_6_47: add s_6_42 s_6_44
        let s_6_47: u16 = (s_6_42 + s_6_44);
        // D s_6_48: create-bits s_6_46 s_6_47
        let s_6_48: Bits = Bits::new(s_6_46, s_6_47);
        // D s_6_49: cast reint s_6_48 -> u8
        let s_6_49: u8 = (s_6_48.value() as u8);
        // D s_6_50: cast zx s_6_49 -> bv
        let s_6_50: Bits = Bits::new(s_6_49 as u128, 5u16);
        // D s_6_51: cast zx s_6_50 -> i
        let s_6_51: i128 = (s_6_50.value() as i128);
        // D s_6_52: cast reint s_6_51 -> i64
        let s_6_52: i64 = (s_6_51 as i64);
        // D s_6_53: read-var M:u8
        let s_6_53: bool = fn_state.M;
        // D s_6_54: cast zx s_6_53 -> bv
        let s_6_54: Bits = Bits::new(s_6_53 as u128, 1u16);
        // D s_6_55: read-var Vm:u8
        let s_6_55: u8 = fn_state.Vm;
        // D s_6_56: cast zx s_6_55 -> bv
        let s_6_56: Bits = Bits::new(s_6_55 as u128, 4u16);
        // D s_6_57: cast reint s_6_54 -> u128
        let s_6_57: u128 = (s_6_54.value() as u128);
        // D s_6_58: size-of s_6_54
        let s_6_58: u16 = s_6_54.length();
        // D s_6_59: cast reint s_6_56 -> u128
        let s_6_59: u128 = (s_6_56.value() as u128);
        // D s_6_60: size-of s_6_56
        let s_6_60: u16 = s_6_56.length();
        // D s_6_61: lsl s_6_57 s_6_60
        let s_6_61: u128 = s_6_57 << s_6_60;
        // D s_6_62: or s_6_61 s_6_59
        let s_6_62: u128 = ((s_6_61) | (s_6_59));
        // D s_6_63: add s_6_58 s_6_60
        let s_6_63: u16 = (s_6_58 + s_6_60);
        // D s_6_64: create-bits s_6_62 s_6_63
        let s_6_64: Bits = Bits::new(s_6_62, s_6_63);
        // D s_6_65: cast reint s_6_64 -> u8
        let s_6_65: u8 = (s_6_64.value() as u8);
        // D s_6_66: cast zx s_6_65 -> bv
        let s_6_66: Bits = Bits::new(s_6_65 as u128, 5u16);
        // D s_6_67: cast zx s_6_66 -> i
        let s_6_67: i128 = (s_6_66.value() as i128);
        // D s_6_68: cast reint s_6_67 -> i64
        let s_6_68: i64 = (s_6_67 as i64);
        // D s_6_69: cast zx s_6_32 -> i
        let s_6_69: i128 = (i128::try_from(s_6_32).unwrap());
        // D s_6_70: cast reint s_6_69 -> i64
        let s_6_70: i64 = (s_6_69 as i64);
        // D s_6_71: cast zx s_6_36 -> i
        let s_6_71: i128 = (i128::try_from(s_6_36).unwrap());
        // D s_6_72: call execute_aarch32_instrs_VQMOVN_Op_A_txt(s_6_52, s_6_26, s_6_71, s_6_70, s_6_68, s_6_4)
        let s_6_72: () = execute_aarch32_instrs_VQMOVN_Op_A_txt(
            state,
            tracer,
            s_6_52,
            s_6_26,
            s_6_71,
            s_6_70,
            s_6_68,
            s_6_4,
        );
        // N s_6_73: return
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
        // D s_8_1: write-var gs#316119 <= s_8_0
        fn_state.gs_316119 = s_8_0;
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
