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
use execute_aarch32_instrs_VSUBL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSUBL_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    op: bool,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_322693: bool,
        gs_322694: bool,
        U: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        op: bool,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        size,
        Vn,
        Vd,
        op,
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
        // N s_2_5: branch s_2_4 b12 b3
        if s_2_4 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_1: read-var Vd:u8
        let s_3_1: u8 = fn_state.Vd;
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
        // N s_3_22: branch s_3_21 b11 b4
        if s_3_21 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op:u8
        let s_4_0: bool = fn_state.op;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b10 b5
        if s_4_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#322693 <= s_5_0
        fn_state.gs_322693 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#322693:u8
        let s_6_0: bool = fn_state.gs_322693;
        // D s_6_1: write-var gs#322694 <= s_6_0
        fn_state.gs_322694 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#322694:u8
        let s_7_0: bool = fn_state.gs_322694;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var U:u8
        let s_8_0: bool = fn_state.U;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: read-var size:u8
        let s_8_5: u8 = fn_state.size;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 2u16);
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (s_8_6.value() as i128);
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // C s_8_9: const #8s : i64
        let s_8_9: i64 = 8;
        // D s_8_10: lsl s_8_9 s_8_8
        let s_8_10: i64 = s_8_9 << s_8_8;
        // C s_8_11: const #64s : i
        let s_8_11: i128 = 64;
        // D s_8_12: cast zx s_8_10 -> i
        let s_8_12: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_13: div s_8_11 s_8_12
        let s_8_13: i128 = ((s_8_11) / (s_8_12));
        // D s_8_14: cast reint s_8_13 -> i64
        let s_8_14: i64 = (s_8_13 as i64);
        // D s_8_15: read-var op:u8
        let s_8_15: bool = fn_state.op;
        // D s_8_16: cast zx s_8_15 -> bv
        let s_8_16: Bits = Bits::new(s_8_15 as u128, 1u16);
        // C s_8_17: const #1u : u8
        let s_8_17: bool = true;
        // C s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // D s_8_19: cmp-eq s_8_16 s_8_18
        let s_8_19: bool = ((s_8_16) == (s_8_18));
        // D s_8_20: read-var D:u8
        let s_8_20: bool = fn_state.D;
        // D s_8_21: cast zx s_8_20 -> bv
        let s_8_21: Bits = Bits::new(s_8_20 as u128, 1u16);
        // D s_8_22: read-var Vd:u8
        let s_8_22: u8 = fn_state.Vd;
        // D s_8_23: cast zx s_8_22 -> bv
        let s_8_23: Bits = Bits::new(s_8_22 as u128, 4u16);
        // D s_8_24: cast reint s_8_21 -> u128
        let s_8_24: u128 = (s_8_21.value() as u128);
        // D s_8_25: size-of s_8_21
        let s_8_25: u16 = s_8_21.length();
        // D s_8_26: cast reint s_8_23 -> u128
        let s_8_26: u128 = (s_8_23.value() as u128);
        // D s_8_27: size-of s_8_23
        let s_8_27: u16 = s_8_23.length();
        // D s_8_28: lsl s_8_24 s_8_27
        let s_8_28: u128 = s_8_24 << s_8_27;
        // D s_8_29: or s_8_28 s_8_26
        let s_8_29: u128 = ((s_8_28) | (s_8_26));
        // D s_8_30: add s_8_25 s_8_27
        let s_8_30: u16 = (s_8_25 + s_8_27);
        // D s_8_31: create-bits s_8_29 s_8_30
        let s_8_31: Bits = Bits::new(s_8_29, s_8_30);
        // D s_8_32: cast reint s_8_31 -> u8
        let s_8_32: u8 = (s_8_31.value() as u8);
        // D s_8_33: cast zx s_8_32 -> bv
        let s_8_33: Bits = Bits::new(s_8_32 as u128, 5u16);
        // D s_8_34: cast zx s_8_33 -> i
        let s_8_34: i128 = (s_8_33.value() as i128);
        // D s_8_35: cast reint s_8_34 -> i64
        let s_8_35: i64 = (s_8_34 as i64);
        // D s_8_36: read-var N:u8
        let s_8_36: bool = fn_state.N;
        // D s_8_37: cast zx s_8_36 -> bv
        let s_8_37: Bits = Bits::new(s_8_36 as u128, 1u16);
        // D s_8_38: read-var Vn:u8
        let s_8_38: u8 = fn_state.Vn;
        // D s_8_39: cast zx s_8_38 -> bv
        let s_8_39: Bits = Bits::new(s_8_38 as u128, 4u16);
        // D s_8_40: cast reint s_8_37 -> u128
        let s_8_40: u128 = (s_8_37.value() as u128);
        // D s_8_41: size-of s_8_37
        let s_8_41: u16 = s_8_37.length();
        // D s_8_42: cast reint s_8_39 -> u128
        let s_8_42: u128 = (s_8_39.value() as u128);
        // D s_8_43: size-of s_8_39
        let s_8_43: u16 = s_8_39.length();
        // D s_8_44: lsl s_8_40 s_8_43
        let s_8_44: u128 = s_8_40 << s_8_43;
        // D s_8_45: or s_8_44 s_8_42
        let s_8_45: u128 = ((s_8_44) | (s_8_42));
        // D s_8_46: add s_8_41 s_8_43
        let s_8_46: u16 = (s_8_41 + s_8_43);
        // D s_8_47: create-bits s_8_45 s_8_46
        let s_8_47: Bits = Bits::new(s_8_45, s_8_46);
        // D s_8_48: cast reint s_8_47 -> u8
        let s_8_48: u8 = (s_8_47.value() as u8);
        // D s_8_49: cast zx s_8_48 -> bv
        let s_8_49: Bits = Bits::new(s_8_48 as u128, 5u16);
        // D s_8_50: cast zx s_8_49 -> i
        let s_8_50: i128 = (s_8_49.value() as i128);
        // D s_8_51: cast reint s_8_50 -> i64
        let s_8_51: i64 = (s_8_50 as i64);
        // D s_8_52: read-var M:u8
        let s_8_52: bool = fn_state.M;
        // D s_8_53: cast zx s_8_52 -> bv
        let s_8_53: Bits = Bits::new(s_8_52 as u128, 1u16);
        // D s_8_54: read-var Vm:u8
        let s_8_54: u8 = fn_state.Vm;
        // D s_8_55: cast zx s_8_54 -> bv
        let s_8_55: Bits = Bits::new(s_8_54 as u128, 4u16);
        // D s_8_56: cast reint s_8_53 -> u128
        let s_8_56: u128 = (s_8_53.value() as u128);
        // D s_8_57: size-of s_8_53
        let s_8_57: u16 = s_8_53.length();
        // D s_8_58: cast reint s_8_55 -> u128
        let s_8_58: u128 = (s_8_55.value() as u128);
        // D s_8_59: size-of s_8_55
        let s_8_59: u16 = s_8_55.length();
        // D s_8_60: lsl s_8_56 s_8_59
        let s_8_60: u128 = s_8_56 << s_8_59;
        // D s_8_61: or s_8_60 s_8_58
        let s_8_61: u128 = ((s_8_60) | (s_8_58));
        // D s_8_62: add s_8_57 s_8_59
        let s_8_62: u16 = (s_8_57 + s_8_59);
        // D s_8_63: create-bits s_8_61 s_8_62
        let s_8_63: Bits = Bits::new(s_8_61, s_8_62);
        // D s_8_64: cast reint s_8_63 -> u8
        let s_8_64: u8 = (s_8_63.value() as u8);
        // D s_8_65: cast zx s_8_64 -> bv
        let s_8_65: Bits = Bits::new(s_8_64 as u128, 5u16);
        // D s_8_66: cast zx s_8_65 -> i
        let s_8_66: i128 = (s_8_65.value() as i128);
        // D s_8_67: cast reint s_8_66 -> i64
        let s_8_67: i64 = (s_8_66 as i64);
        // D s_8_68: cast zx s_8_10 -> i
        let s_8_68: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_69: cast reint s_8_68 -> i64
        let s_8_69: i64 = (s_8_68 as i64);
        // D s_8_70: cast zx s_8_14 -> i
        let s_8_70: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_71: call execute_aarch32_instrs_VSUBL_Op_A_txt(s_8_35, s_8_70, s_8_69, s_8_19, s_8_67, s_8_51, s_8_4)
        let s_8_71: () = execute_aarch32_instrs_VSUBL_Op_A_txt(
            state,
            tracer,
            s_8_35,
            s_8_70,
            s_8_69,
            s_8_19,
            s_8_67,
            s_8_51,
            s_8_4,
        );
        // N s_8_72: return
        return;
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
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var Vn:u8
        let s_10_1: u8 = fn_state.Vn;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 4u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // D s_10_22: write-var gs#322693 <= s_10_21
        fn_state.gs_322693 = s_10_21;
        // N s_10_23: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#322694 <= s_11_0
        fn_state.gs_322694 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
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
}
