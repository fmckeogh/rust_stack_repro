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
use R_read::*;
use R_set::*;
use integer_subrange::*;
use ROR::*;
use common::*;
pub fn execute_aarch32_instrs_SMLSLD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    dHi: i64,
    dLo: i64,
    m: i64,
    m_swap: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: u32,
        dHi: i64,
        dLo: i64,
        m: i64,
        m_swap: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        dHi,
        dLo,
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
        // N s_0_1: branch s_0_0 b3 b1
        if s_0_0 {
            return block_3(state, tracer, fn_state);
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
        // D s_2_64: read-var dHi:i64
        let s_2_64: i64 = fn_state.dHi;
        // D s_2_65: cast zx s_2_64 -> i
        let s_2_65: i128 = (i128::try_from(s_2_64).unwrap());
        // D s_2_66: call R_read(s_2_65)
        let s_2_66: u32 = R_read(state, tracer, s_2_65);
        // D s_2_67: read-var dLo:i64
        let s_2_67: i64 = fn_state.dLo;
        // D s_2_68: cast zx s_2_67 -> i
        let s_2_68: i128 = (i128::try_from(s_2_67).unwrap());
        // D s_2_69: call R_read(s_2_68)
        let s_2_69: u32 = R_read(state, tracer, s_2_68);
        // D s_2_70: cast zx s_2_66 -> bv
        let s_2_70: Bits = Bits::new(s_2_66 as u128, 32u16);
        // D s_2_71: cast zx s_2_69 -> bv
        let s_2_71: Bits = Bits::new(s_2_69 as u128, 32u16);
        // D s_2_72: cast reint s_2_70 -> u128
        let s_2_72: u128 = (s_2_70.value() as u128);
        // D s_2_73: size-of s_2_70
        let s_2_73: u16 = s_2_70.length();
        // D s_2_74: cast reint s_2_71 -> u128
        let s_2_74: u128 = (s_2_71.value() as u128);
        // D s_2_75: size-of s_2_71
        let s_2_75: u16 = s_2_71.length();
        // D s_2_76: lsl s_2_72 s_2_75
        let s_2_76: u128 = s_2_72 << s_2_75;
        // D s_2_77: or s_2_76 s_2_74
        let s_2_77: u128 = ((s_2_76) | (s_2_74));
        // D s_2_78: add s_2_73 s_2_75
        let s_2_78: u16 = (s_2_73 + s_2_75);
        // D s_2_79: create-bits s_2_77 s_2_78
        let s_2_79: Bits = Bits::new(s_2_77, s_2_78);
        // D s_2_80: cast reint s_2_79 -> u64
        let s_2_80: u64 = (s_2_79.value() as u64);
        // D s_2_81: cast zx s_2_80 -> bv
        let s_2_81: Bits = Bits::new(s_2_80 as u128, 64u16);
        // D s_2_82: cast sx s_2_81 -> i
        let s_2_82: i128 = {
            let sign_bit = s_2_81.length() - 1;
            let mut result = s_2_81.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_83: cast reint s_2_82 -> i64
        let s_2_83: i64 = (s_2_82 as i64);
        // D s_2_84: cast zx s_2_63 -> i
        let s_2_84: i128 = (i128::try_from(s_2_63).unwrap());
        // D s_2_85: cast zx s_2_83 -> i
        let s_2_85: i128 = (i128::try_from(s_2_83).unwrap());
        // D s_2_86: add s_2_84 s_2_85
        let s_2_86: i128 = (s_2_84 + s_2_85);
        // C s_2_87: const #63s : i
        let s_2_87: i128 = 63;
        // C s_2_88: const #32s : i
        let s_2_88: i128 = 32;
        // D s_2_89: call integer_subrange(s_2_86, s_2_87, s_2_88)
        let s_2_89: Bits = integer_subrange(state, tracer, s_2_86, s_2_87, s_2_88);
        // D s_2_90: cast reint s_2_89 -> u32
        let s_2_90: u32 = (s_2_89.value() as u32);
        // D s_2_91: read-var dHi:i64
        let s_2_91: i64 = fn_state.dHi;
        // D s_2_92: cast zx s_2_91 -> i
        let s_2_92: i128 = (i128::try_from(s_2_91).unwrap());
        // D s_2_93: call R_set(s_2_92, s_2_90)
        let s_2_93: () = R_set(state, tracer, s_2_92, s_2_90);
        // C s_2_94: const #31s : i
        let s_2_94: i128 = 31;
        // C s_2_95: const #0s : i
        let s_2_95: i128 = 0;
        // D s_2_96: call integer_subrange(s_2_86, s_2_94, s_2_95)
        let s_2_96: Bits = integer_subrange(state, tracer, s_2_86, s_2_94, s_2_95);
        // D s_2_97: cast reint s_2_96 -> u32
        let s_2_97: u32 = (s_2_96.value() as u32);
        // D s_2_98: read-var dLo:i64
        let s_2_98: i64 = fn_state.dLo;
        // D s_2_99: cast zx s_2_98 -> i
        let s_2_99: i128 = (i128::try_from(s_2_98).unwrap());
        // D s_2_100: call R_set(s_2_99, s_2_97)
        let s_2_100: () = R_set(state, tracer, s_2_99, s_2_97);
        // N s_2_101: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // C s_3_3: const #16s : i
        let s_3_3: i128 = 16;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_5: call ROR(s_3_4, s_3_3)
        let s_3_5: Bits = ROR(state, tracer, s_3_4, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // D s_3_7: write-var operand2 <= s_3_6
        fn_state.operand2 = s_3_6;
        // N s_3_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
