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
use FPAdd_BF16::*;
use Din_read::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use BFMulH::*;
use D_set::*;
use common::*;
pub fn execute_aarch32_instrs_VDOT_bf16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_326819: i64,
        result: u64,
        operand1: u64,
        operand2: u64,
        d: i64,
        m: i64,
        n: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        regs,
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
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#326819 <= s_1_5
        fn_state.gs_326819 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#326819:i64
        let s_2_1: i64 = fn_state.gs_326819;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
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
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: call Din_read(s_3_6)
        let s_3_7: u64 = Din_read(state, tracer, s_3_6);
        // D s_3_8: write-var operand1 <= s_3_7
        fn_state.operand1 = s_3_7;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var r:i64
        let s_3_11: i64 = fn_state.r;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: add s_3_10 s_3_12
        let s_3_13: i128 = (s_3_10 + s_3_12);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: call Din_read(s_3_15)
        let s_3_16: u64 = Din_read(state, tracer, s_3_15);
        // D s_3_17: write-var operand2 <= s_3_16
        fn_state.operand2 = s_3_16;
        // D s_3_18: read-var d:i64
        let s_3_18: i64 = fn_state.d;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: read-var r:i64
        let s_3_20: i64 = fn_state.r;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: add s_3_19 s_3_21
        let s_3_22: i128 = (s_3_19 + s_3_21);
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: call Din_read(s_3_24)
        let s_3_25: u64 = Din_read(state, tracer, s_3_24);
        // D s_3_26: write-var result <= s_3_25
        fn_state.result = s_3_25;
        // C s_3_27: const #0s : i64
        let s_3_27: i64 = 0;
        // D s_3_28: write-var e <= s_3_27
        fn_state.e = s_3_27;
        // N s_3_29: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
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
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // C s_5_9: const #16s : i64
        let s_5_9: i64 = 16;
        // D s_5_10: read-var operand1:u64
        let s_5_10: u64 = fn_state.operand1;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 64u16);
        // D s_5_12: cast zx s_5_8 -> i
        let s_5_12: i128 = (i128::try_from(s_5_8).unwrap());
        // C s_5_13: cast zx s_5_9 -> i
        let s_5_13: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_14: call Elem_read(s_5_11, s_5_12, s_5_13)
        let s_5_14: Bits = Elem_read(state, tracer, s_5_11, s_5_12, s_5_13);
        // D s_5_15: cast reint s_5_14 -> u16
        let s_5_15: u16 = (s_5_14.value() as u16);
        // C s_5_16: const #2s : i
        let s_5_16: i128 = 2;
        // D s_5_17: read-var e:i64
        let s_5_17: i64 = fn_state.e;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: mul s_5_16 s_5_18
        let s_5_19: i128 = ((s_5_16) * (s_5_18));
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // C s_5_21: const #1s : i
        let s_5_21: i128 = 1;
        // D s_5_22: cast zx s_5_20 -> i
        let s_5_22: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_23: add s_5_22 s_5_21
        let s_5_23: i128 = (s_5_22 + s_5_21);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // C s_5_25: const #16s : i64
        let s_5_25: i64 = 16;
        // D s_5_26: read-var operand1:u64
        let s_5_26: u64 = fn_state.operand1;
        // D s_5_27: cast zx s_5_26 -> bv
        let s_5_27: Bits = Bits::new(s_5_26 as u128, 64u16);
        // D s_5_28: cast zx s_5_24 -> i
        let s_5_28: i128 = (i128::try_from(s_5_24).unwrap());
        // C s_5_29: cast zx s_5_25 -> i
        let s_5_29: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_30: call Elem_read(s_5_27, s_5_28, s_5_29)
        let s_5_30: Bits = Elem_read(state, tracer, s_5_27, s_5_28, s_5_29);
        // D s_5_31: cast reint s_5_30 -> u16
        let s_5_31: u16 = (s_5_30.value() as u16);
        // C s_5_32: const #2s : i
        let s_5_32: i128 = 2;
        // D s_5_33: read-var e:i64
        let s_5_33: i64 = fn_state.e;
        // D s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_35: mul s_5_32 s_5_34
        let s_5_35: i128 = ((s_5_32) * (s_5_34));
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // C s_5_37: const #0s : i
        let s_5_37: i128 = 0;
        // D s_5_38: cast zx s_5_36 -> i
        let s_5_38: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_39: add s_5_38 s_5_37
        let s_5_39: i128 = (s_5_38 + s_5_37);
        // D s_5_40: cast reint s_5_39 -> i64
        let s_5_40: i64 = (s_5_39 as i64);
        // C s_5_41: const #16s : i64
        let s_5_41: i64 = 16;
        // D s_5_42: read-var operand2:u64
        let s_5_42: u64 = fn_state.operand2;
        // D s_5_43: cast zx s_5_42 -> bv
        let s_5_43: Bits = Bits::new(s_5_42 as u128, 64u16);
        // D s_5_44: cast zx s_5_40 -> i
        let s_5_44: i128 = (i128::try_from(s_5_40).unwrap());
        // C s_5_45: cast zx s_5_41 -> i
        let s_5_45: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_46: call Elem_read(s_5_43, s_5_44, s_5_45)
        let s_5_46: Bits = Elem_read(state, tracer, s_5_43, s_5_44, s_5_45);
        // D s_5_47: cast reint s_5_46 -> u16
        let s_5_47: u16 = (s_5_46.value() as u16);
        // C s_5_48: const #2s : i
        let s_5_48: i128 = 2;
        // D s_5_49: read-var e:i64
        let s_5_49: i64 = fn_state.e;
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (i128::try_from(s_5_49).unwrap());
        // D s_5_51: mul s_5_48 s_5_50
        let s_5_51: i128 = ((s_5_48) * (s_5_50));
        // D s_5_52: cast reint s_5_51 -> i64
        let s_5_52: i64 = (s_5_51 as i64);
        // C s_5_53: const #1s : i
        let s_5_53: i128 = 1;
        // D s_5_54: cast zx s_5_52 -> i
        let s_5_54: i128 = (i128::try_from(s_5_52).unwrap());
        // D s_5_55: add s_5_54 s_5_53
        let s_5_55: i128 = (s_5_54 + s_5_53);
        // D s_5_56: cast reint s_5_55 -> i64
        let s_5_56: i64 = (s_5_55 as i64);
        // C s_5_57: const #16s : i64
        let s_5_57: i64 = 16;
        // D s_5_58: read-var operand2:u64
        let s_5_58: u64 = fn_state.operand2;
        // D s_5_59: cast zx s_5_58 -> bv
        let s_5_59: Bits = Bits::new(s_5_58 as u128, 64u16);
        // D s_5_60: cast zx s_5_56 -> i
        let s_5_60: i128 = (i128::try_from(s_5_56).unwrap());
        // C s_5_61: cast zx s_5_57 -> i
        let s_5_61: i128 = (i128::try_from(s_5_57).unwrap());
        // D s_5_62: call Elem_read(s_5_59, s_5_60, s_5_61)
        let s_5_62: Bits = Elem_read(state, tracer, s_5_59, s_5_60, s_5_61);
        // D s_5_63: cast reint s_5_62 -> u16
        let s_5_63: u16 = (s_5_62.value() as u16);
        // D s_5_64: cast zx s_5_15 -> bv
        let s_5_64: Bits = Bits::new(s_5_15 as u128, 16u16);
        // D s_5_65: cast zx s_5_47 -> bv
        let s_5_65: Bits = Bits::new(s_5_47 as u128, 16u16);
        // D s_5_66: call BFMulH(s_5_64, s_5_65)
        let s_5_66: Bits = BFMulH(state, tracer, s_5_64, s_5_65);
        // D s_5_67: cast reint s_5_66 -> u32
        let s_5_67: u32 = (s_5_66.value() as u32);
        // D s_5_68: cast zx s_5_31 -> bv
        let s_5_68: Bits = Bits::new(s_5_31 as u128, 16u16);
        // D s_5_69: cast zx s_5_63 -> bv
        let s_5_69: Bits = Bits::new(s_5_63 as u128, 16u16);
        // D s_5_70: call BFMulH(s_5_68, s_5_69)
        let s_5_70: Bits = BFMulH(state, tracer, s_5_68, s_5_69);
        // D s_5_71: cast reint s_5_70 -> u32
        let s_5_71: u32 = (s_5_70.value() as u32);
        // D s_5_72: cast zx s_5_67 -> bv
        let s_5_72: Bits = Bits::new(s_5_67 as u128, 32u16);
        // D s_5_73: cast zx s_5_71 -> bv
        let s_5_73: Bits = Bits::new(s_5_71 as u128, 32u16);
        // D s_5_74: call FPAdd_BF16(s_5_72, s_5_73)
        let s_5_74: Bits = FPAdd_BF16(state, tracer, s_5_72, s_5_73);
        // D s_5_75: cast reint s_5_74 -> u32
        let s_5_75: u32 = (s_5_74.value() as u32);
        // C s_5_76: const #32s : i64
        let s_5_76: i64 = 32;
        // C s_5_77: const #32s : i64
        let s_5_77: i64 = 32;
        // D s_5_78: read-var result:u64
        let s_5_78: u64 = fn_state.result;
        // D s_5_79: cast zx s_5_78 -> bv
        let s_5_79: Bits = Bits::new(s_5_78 as u128, 64u16);
        // D s_5_80: read-var e:i64
        let s_5_80: i64 = fn_state.e;
        // D s_5_81: cast zx s_5_80 -> i
        let s_5_81: i128 = (i128::try_from(s_5_80).unwrap());
        // C s_5_82: cast zx s_5_77 -> i
        let s_5_82: i128 = (i128::try_from(s_5_77).unwrap());
        // D s_5_83: call Elem_read(s_5_79, s_5_81, s_5_82)
        let s_5_83: Bits = Elem_read(state, tracer, s_5_79, s_5_81, s_5_82);
        // D s_5_84: cast reint s_5_83 -> u32
        let s_5_84: u32 = (s_5_83.value() as u32);
        // D s_5_85: cast zx s_5_84 -> bv
        let s_5_85: Bits = Bits::new(s_5_84 as u128, 32u16);
        // D s_5_86: cast zx s_5_75 -> bv
        let s_5_86: Bits = Bits::new(s_5_75 as u128, 32u16);
        // D s_5_87: call FPAdd_BF16(s_5_85, s_5_86)
        let s_5_87: Bits = FPAdd_BF16(state, tracer, s_5_85, s_5_86);
        // D s_5_88: cast reint s_5_87 -> u32
        let s_5_88: u32 = (s_5_87.value() as u32);
        // D s_5_89: read-var result:u64
        let s_5_89: u64 = fn_state.result;
        // D s_5_90: cast zx s_5_89 -> bv
        let s_5_90: Bits = Bits::new(s_5_89 as u128, 64u16);
        // D s_5_91: read-var e:i64
        let s_5_91: i64 = fn_state.e;
        // D s_5_92: cast zx s_5_91 -> i
        let s_5_92: i128 = (i128::try_from(s_5_91).unwrap());
        // C s_5_93: cast zx s_5_76 -> i
        let s_5_93: i128 = (i128::try_from(s_5_76).unwrap());
        // D s_5_94: cast zx s_5_88 -> bv
        let s_5_94: Bits = Bits::new(s_5_88 as u128, 32u16);
        // D s_5_95: call Elem_set(s_5_90, s_5_92, s_5_93, s_5_94)
        let s_5_95: Bits = Elem_set(state, tracer, s_5_90, s_5_92, s_5_93, s_5_94);
        // D s_5_96: cast reint s_5_95 -> u64
        let s_5_96: u64 = (s_5_95.value() as u64);
        // D s_5_97: write-var result <= s_5_96
        fn_state.result = s_5_96;
        // D s_5_98: read-var e:i64
        let s_5_98: i64 = fn_state.e;
        // C s_5_99: const #1s : i64
        let s_5_99: i64 = 1;
        // D s_5_100: add s_5_98 s_5_99
        let s_5_100: i64 = (s_5_98 + s_5_99);
        // D s_5_101: write-var e <= s_5_100
        fn_state.e = s_5_100;
        // N s_5_102: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var result:u64
        let s_6_7: u64 = fn_state.result;
        // D s_6_8: call D_set(s_6_6, s_6_7)
        let s_6_8: () = D_set(state, tracer, s_6_6, s_6_7);
        // D s_6_9: read-var r:i64
        let s_6_9: i64 = fn_state.r;
        // C s_6_10: const #1s : i64
        let s_6_10: i64 = 1;
        // D s_6_11: add s_6_9 s_6_10
        let s_6_11: i64 = (s_6_9 + s_6_10);
        // D s_6_12: write-var r <= s_6_11
        fn_state.r = s_6_11;
        // N s_6_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
