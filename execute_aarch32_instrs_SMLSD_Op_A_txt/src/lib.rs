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
use R_read::*;
use R_set::*;
use integer_subrange::*;
use ROR::*;
use common::*;
pub fn execute_aarch32_instrs_SMLSD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    m_swap: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: u32,
        a: i64,
        d: i64,
        m: i64,
        m_swap: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        a,
        d,
        m,
        m_swap,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m_swap:u8
        let s_0_0: bool = fn_state.m_swap;
        // N s_0_1: branch s_0_0 b5 b1
        if s_0_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: write-var operand2 <= s_1_2
        fn_state.operand2 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // D s_2_4: cast zx s_2_2 -> bv
        let s_2_4: Bits = Bits::new(s_2_2 as u128, 32u16);
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #15s : i
        let s_2_7: i128 = 15;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_3 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_3)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u16
        let s_2_10: u16 = (s_2_9.value() as u16);
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 16u16);
        // D s_2_12: cast sx s_2_11 -> i
        let s_2_12: i128 = {
            let sign_bit = s_2_11.length() - 1;
            let mut result = s_2_11.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // C s_2_14: const #0s : i
        let s_2_14: i128 = 0;
        // D s_2_15: read-var operand2:u32
        let s_2_15: u32 = fn_state.operand2;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 32u16);
        // C s_2_17: const #1s : i64
        let s_2_17: i64 = 1;
        // C s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // C s_2_19: const #15s : i
        let s_2_19: i128 = 15;
        // C s_2_20: add s_2_19 s_2_18
        let s_2_20: i128 = (s_2_19 + s_2_18);
        // D s_2_21: bit-extract s_2_16 s_2_14 s_2_20
        let s_2_21: Bits = (Bits::new(
            ((s_2_16) >> (s_2_14)).value(),
            u16::try_from(s_2_20).unwrap(),
        ));
        // D s_2_22: cast reint s_2_21 -> u16
        let s_2_22: u16 = (s_2_21.value() as u16);
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 16u16);
        // D s_2_24: cast sx s_2_23 -> i
        let s_2_24: i128 = {
            let sign_bit = s_2_23.length() - 1;
            let mut result = s_2_23.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_25: cast reint s_2_24 -> i64
        let s_2_25: i64 = (s_2_24 as i64);
        // D s_2_26: cast zx s_2_13 -> i
        let s_2_26: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_27: cast zx s_2_25 -> i
        let s_2_27: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_28: mul s_2_26 s_2_27
        let s_2_28: i128 = ((s_2_26) * (s_2_27));
        // D s_2_29: cast reint s_2_28 -> i64
        let s_2_29: i64 = (s_2_28 as i64);
        // D s_2_30: read-var n:i64
        let s_2_30: i64 = fn_state.n;
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_32: call R_read(s_2_31)
        let s_2_32: u32 = R_read(state, tracer, s_2_31);
        // C s_2_33: const #16s : i
        let s_2_33: i128 = 16;
        // D s_2_34: cast zx s_2_32 -> bv
        let s_2_34: Bits = Bits::new(s_2_32 as u128, 32u16);
        // C s_2_35: const #1s : i64
        let s_2_35: i64 = 1;
        // C s_2_36: cast zx s_2_35 -> i
        let s_2_36: i128 = (i128::try_from(s_2_35).unwrap());
        // C s_2_37: const #15s : i
        let s_2_37: i128 = 15;
        // C s_2_38: add s_2_37 s_2_36
        let s_2_38: i128 = (s_2_37 + s_2_36);
        // D s_2_39: bit-extract s_2_34 s_2_33 s_2_38
        let s_2_39: Bits = (Bits::new(
            ((s_2_34) >> (s_2_33)).value(),
            u16::try_from(s_2_38).unwrap(),
        ));
        // D s_2_40: cast reint s_2_39 -> u16
        let s_2_40: u16 = (s_2_39.value() as u16);
        // D s_2_41: cast zx s_2_40 -> bv
        let s_2_41: Bits = Bits::new(s_2_40 as u128, 16u16);
        // D s_2_42: cast sx s_2_41 -> i
        let s_2_42: i128 = {
            let sign_bit = s_2_41.length() - 1;
            let mut result = s_2_41.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_43: cast reint s_2_42 -> i64
        let s_2_43: i64 = (s_2_42 as i64);
        // C s_2_44: const #16s : i
        let s_2_44: i128 = 16;
        // D s_2_45: read-var operand2:u32
        let s_2_45: u32 = fn_state.operand2;
        // D s_2_46: cast zx s_2_45 -> bv
        let s_2_46: Bits = Bits::new(s_2_45 as u128, 32u16);
        // C s_2_47: const #1s : i64
        let s_2_47: i64 = 1;
        // C s_2_48: cast zx s_2_47 -> i
        let s_2_48: i128 = (i128::try_from(s_2_47).unwrap());
        // C s_2_49: const #15s : i
        let s_2_49: i128 = 15;
        // C s_2_50: add s_2_49 s_2_48
        let s_2_50: i128 = (s_2_49 + s_2_48);
        // D s_2_51: bit-extract s_2_46 s_2_44 s_2_50
        let s_2_51: Bits = (Bits::new(
            ((s_2_46) >> (s_2_44)).value(),
            u16::try_from(s_2_50).unwrap(),
        ));
        // D s_2_52: cast reint s_2_51 -> u16
        let s_2_52: u16 = (s_2_51.value() as u16);
        // D s_2_53: cast zx s_2_52 -> bv
        let s_2_53: Bits = Bits::new(s_2_52 as u128, 16u16);
        // D s_2_54: cast sx s_2_53 -> i
        let s_2_54: i128 = {
            let sign_bit = s_2_53.length() - 1;
            let mut result = s_2_53.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_55: cast reint s_2_54 -> i64
        let s_2_55: i64 = (s_2_54 as i64);
        // D s_2_56: cast zx s_2_43 -> i
        let s_2_56: i128 = (i128::try_from(s_2_43).unwrap());
        // D s_2_57: cast zx s_2_55 -> i
        let s_2_57: i128 = (i128::try_from(s_2_55).unwrap());
        // D s_2_58: mul s_2_56 s_2_57
        let s_2_58: i128 = ((s_2_56) * (s_2_57));
        // D s_2_59: cast reint s_2_58 -> i64
        let s_2_59: i64 = (s_2_58 as i64);
        // D s_2_60: cast zx s_2_29 -> i
        let s_2_60: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_61: cast zx s_2_59 -> i
        let s_2_61: i128 = (i128::try_from(s_2_59).unwrap());
        // D s_2_62: sub s_2_60 s_2_61
        let s_2_62: i128 = ((s_2_60) - (s_2_61));
        // D s_2_63: cast reint s_2_62 -> i64
        let s_2_63: i64 = (s_2_62 as i64);
        // D s_2_64: read-var a:i64
        let s_2_64: i64 = fn_state.a;
        // D s_2_65: cast zx s_2_64 -> i
        let s_2_65: i128 = (i128::try_from(s_2_64).unwrap());
        // D s_2_66: call R_read(s_2_65)
        let s_2_66: u32 = R_read(state, tracer, s_2_65);
        // D s_2_67: cast zx s_2_66 -> bv
        let s_2_67: Bits = Bits::new(s_2_66 as u128, 32u16);
        // D s_2_68: cast sx s_2_67 -> i
        let s_2_68: i128 = {
            let sign_bit = s_2_67.length() - 1;
            let mut result = s_2_67.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_69: cast reint s_2_68 -> i64
        let s_2_69: i64 = (s_2_68 as i64);
        // D s_2_70: cast zx s_2_63 -> i
        let s_2_70: i128 = (i128::try_from(s_2_63).unwrap());
        // D s_2_71: cast zx s_2_69 -> i
        let s_2_71: i128 = (i128::try_from(s_2_69).unwrap());
        // D s_2_72: add s_2_70 s_2_71
        let s_2_72: i128 = (s_2_70 + s_2_71);
        // D s_2_73: cast reint s_2_72 -> i64
        let s_2_73: i64 = (s_2_72 as i64);
        // C s_2_74: const #31s : i
        let s_2_74: i128 = 31;
        // C s_2_75: const #0s : i
        let s_2_75: i128 = 0;
        // D s_2_76: cast zx s_2_73 -> i
        let s_2_76: i128 = (i128::try_from(s_2_73).unwrap());
        // D s_2_77: call integer_subrange(s_2_76, s_2_74, s_2_75)
        let s_2_77: Bits = integer_subrange(state, tracer, s_2_76, s_2_74, s_2_75);
        // D s_2_78: cast reint s_2_77 -> u32
        let s_2_78: u32 = (s_2_77.value() as u32);
        // D s_2_79: read-var d:i64
        let s_2_79: i64 = fn_state.d;
        // D s_2_80: cast zx s_2_79 -> i
        let s_2_80: i128 = (i128::try_from(s_2_79).unwrap());
        // D s_2_81: call R_set(s_2_80, s_2_78)
        let s_2_81: () = R_set(state, tracer, s_2_80, s_2_78);
        // C s_2_82: const #31s : i
        let s_2_82: i128 = 31;
        // C s_2_83: const #0s : i
        let s_2_83: i128 = 0;
        // D s_2_84: cast zx s_2_73 -> i
        let s_2_84: i128 = (i128::try_from(s_2_73).unwrap());
        // D s_2_85: call integer_subrange(s_2_84, s_2_82, s_2_83)
        let s_2_85: Bits = integer_subrange(state, tracer, s_2_84, s_2_82, s_2_83);
        // D s_2_86: cast reint s_2_85 -> u32
        let s_2_86: u32 = (s_2_85.value() as u32);
        // D s_2_87: cast zx s_2_86 -> bv
        let s_2_87: Bits = Bits::new(s_2_86 as u128, 32u16);
        // D s_2_88: cast sx s_2_87 -> i
        let s_2_88: i128 = {
            let sign_bit = s_2_87.length() - 1;
            let mut result = s_2_87.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_89: cast reint s_2_88 -> i64
        let s_2_89: i64 = (s_2_88 as i64);
        // D s_2_90: cast zx s_2_73 -> i
        let s_2_90: i128 = (i128::try_from(s_2_73).unwrap());
        // D s_2_91: cast zx s_2_89 -> i
        let s_2_91: i128 = (i128::try_from(s_2_89).unwrap());
        // D s_2_92: call neq_int(s_2_90, s_2_91)
        let s_2_92: bool = neq_int(state, tracer, s_2_90, s_2_91);
        // N s_2_93: branch s_2_92 b4 b3
        if s_2_92 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #16988u : u32
        let s_4_1: u32 = 16988;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call R_read(s_5_1)
        let s_5_2: u32 = R_read(state, tracer, s_5_1);
        // C s_5_3: const #16s : i
        let s_5_3: i128 = 16;
        // D s_5_4: cast zx s_5_2 -> bv
        let s_5_4: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_5: call ROR(s_5_4, s_5_3)
        let s_5_5: Bits = ROR(state, tracer, s_5_4, s_5_3);
        // D s_5_6: cast reint s_5_5 -> u32
        let s_5_6: u32 = (s_5_5.value() as u32);
        // D s_5_7: write-var operand2 <= s_5_6
        fn_state.operand2 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
