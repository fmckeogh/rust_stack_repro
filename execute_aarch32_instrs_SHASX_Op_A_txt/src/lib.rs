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
use common::*;
pub fn execute_aarch32_instrs_SHASX_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #15s : i
        let s_0_7: i128 = 15;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u16
        let s_0_10: u16 = (s_0_9.value() as u16);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 16u16);
        // D s_0_12: cast sx s_0_11 -> i
        let s_0_12: i128 = {
            let sign_bit = s_0_11.length() - 1;
            let mut result = s_0_11.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call R_read(s_0_15)
        let s_0_16: u32 = R_read(state, tracer, s_0_15);
        // C s_0_17: const #16s : i
        let s_0_17: i128 = 16;
        // D s_0_18: cast zx s_0_16 -> bv
        let s_0_18: Bits = Bits::new(s_0_16 as u128, 32u16);
        // C s_0_19: const #1s : i64
        let s_0_19: i64 = 1;
        // C s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // C s_0_21: const #15s : i
        let s_0_21: i128 = 15;
        // C s_0_22: add s_0_21 s_0_20
        let s_0_22: i128 = (s_0_21 + s_0_20);
        // D s_0_23: bit-extract s_0_18 s_0_17 s_0_22
        let s_0_23: Bits = (Bits::new(
            ((s_0_18) >> (s_0_17)).value(),
            u16::try_from(s_0_22).unwrap(),
        ));
        // D s_0_24: cast reint s_0_23 -> u16
        let s_0_24: u16 = (s_0_23.value() as u16);
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 16u16);
        // D s_0_26: cast sx s_0_25 -> i
        let s_0_26: i128 = {
            let sign_bit = s_0_25.length() - 1;
            let mut result = s_0_25.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_27: cast reint s_0_26 -> i64
        let s_0_27: i64 = (s_0_26 as i64);
        // D s_0_28: cast zx s_0_13 -> i
        let s_0_28: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_29: cast zx s_0_27 -> i
        let s_0_29: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_30: sub s_0_28 s_0_29
        let s_0_30: i128 = ((s_0_28) - (s_0_29));
        // D s_0_31: cast reint s_0_30 -> i64
        let s_0_31: i64 = (s_0_30 as i64);
        // D s_0_32: read-var n:i64
        let s_0_32: i64 = fn_state.n;
        // D s_0_33: cast zx s_0_32 -> i
        let s_0_33: i128 = (i128::try_from(s_0_32).unwrap());
        // D s_0_34: call R_read(s_0_33)
        let s_0_34: u32 = R_read(state, tracer, s_0_33);
        // C s_0_35: const #16s : i
        let s_0_35: i128 = 16;
        // D s_0_36: cast zx s_0_34 -> bv
        let s_0_36: Bits = Bits::new(s_0_34 as u128, 32u16);
        // C s_0_37: const #1s : i64
        let s_0_37: i64 = 1;
        // C s_0_38: cast zx s_0_37 -> i
        let s_0_38: i128 = (i128::try_from(s_0_37).unwrap());
        // C s_0_39: const #15s : i
        let s_0_39: i128 = 15;
        // C s_0_40: add s_0_39 s_0_38
        let s_0_40: i128 = (s_0_39 + s_0_38);
        // D s_0_41: bit-extract s_0_36 s_0_35 s_0_40
        let s_0_41: Bits = (Bits::new(
            ((s_0_36) >> (s_0_35)).value(),
            u16::try_from(s_0_40).unwrap(),
        ));
        // D s_0_42: cast reint s_0_41 -> u16
        let s_0_42: u16 = (s_0_41.value() as u16);
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 16u16);
        // D s_0_44: cast sx s_0_43 -> i
        let s_0_44: i128 = {
            let sign_bit = s_0_43.length() - 1;
            let mut result = s_0_43.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_45: cast reint s_0_44 -> i64
        let s_0_45: i64 = (s_0_44 as i64);
        // D s_0_46: read-var m:i64
        let s_0_46: i64 = fn_state.m;
        // D s_0_47: cast zx s_0_46 -> i
        let s_0_47: i128 = (i128::try_from(s_0_46).unwrap());
        // D s_0_48: call R_read(s_0_47)
        let s_0_48: u32 = R_read(state, tracer, s_0_47);
        // C s_0_49: const #0s : i
        let s_0_49: i128 = 0;
        // D s_0_50: cast zx s_0_48 -> bv
        let s_0_50: Bits = Bits::new(s_0_48 as u128, 32u16);
        // C s_0_51: const #1s : i64
        let s_0_51: i64 = 1;
        // C s_0_52: cast zx s_0_51 -> i
        let s_0_52: i128 = (i128::try_from(s_0_51).unwrap());
        // C s_0_53: const #15s : i
        let s_0_53: i128 = 15;
        // C s_0_54: add s_0_53 s_0_52
        let s_0_54: i128 = (s_0_53 + s_0_52);
        // D s_0_55: bit-extract s_0_50 s_0_49 s_0_54
        let s_0_55: Bits = (Bits::new(
            ((s_0_50) >> (s_0_49)).value(),
            u16::try_from(s_0_54).unwrap(),
        ));
        // D s_0_56: cast reint s_0_55 -> u16
        let s_0_56: u16 = (s_0_55.value() as u16);
        // D s_0_57: cast zx s_0_56 -> bv
        let s_0_57: Bits = Bits::new(s_0_56 as u128, 16u16);
        // D s_0_58: cast sx s_0_57 -> i
        let s_0_58: i128 = {
            let sign_bit = s_0_57.length() - 1;
            let mut result = s_0_57.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_59: cast reint s_0_58 -> i64
        let s_0_59: i64 = (s_0_58 as i64);
        // D s_0_60: cast zx s_0_45 -> i
        let s_0_60: i128 = (i128::try_from(s_0_45).unwrap());
        // D s_0_61: cast zx s_0_59 -> i
        let s_0_61: i128 = (i128::try_from(s_0_59).unwrap());
        // D s_0_62: add s_0_60 s_0_61
        let s_0_62: i128 = (s_0_60 + s_0_61);
        // D s_0_63: cast reint s_0_62 -> i64
        let s_0_63: i64 = (s_0_62 as i64);
        // D s_0_64: read-var d:i64
        let s_0_64: i64 = fn_state.d;
        // D s_0_65: cast zx s_0_64 -> i
        let s_0_65: i128 = (i128::try_from(s_0_64).unwrap());
        // D s_0_66: call R_read(s_0_65)
        let s_0_66: u32 = R_read(state, tracer, s_0_65);
        // C s_0_67: const #16s : i
        let s_0_67: i128 = 16;
        // C s_0_68: const #1s : i
        let s_0_68: i128 = 1;
        // D s_0_69: cast zx s_0_31 -> i
        let s_0_69: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_70: call integer_subrange(s_0_69, s_0_67, s_0_68)
        let s_0_70: Bits = integer_subrange(state, tracer, s_0_69, s_0_67, s_0_68);
        // D s_0_71: cast reint s_0_70 -> u16
        let s_0_71: u16 = (s_0_70.value() as u16);
        // C s_0_72: const #0s : i
        let s_0_72: i128 = 0;
        // D s_0_73: cast zx s_0_66 -> bv
        let s_0_73: Bits = Bits::new(s_0_66 as u128, 32u16);
        // D s_0_74: cast zx s_0_71 -> bv
        let s_0_74: Bits = Bits::new(s_0_71 as u128, 16u16);
        // C s_0_75: const #15s : i
        let s_0_75: i128 = 15;
        // C s_0_76: const #1u : u64
        let s_0_76: u64 = 1;
        // C s_0_77: cast zx s_0_76 -> bv
        let s_0_77: Bits = Bits::new(s_0_76 as u128, 64u16);
        // C s_0_78: lsl s_0_77 s_0_75
        let s_0_78: Bits = s_0_77 << s_0_75;
        // C s_0_79: sub s_0_78 s_0_77
        let s_0_79: Bits = ((s_0_78) - (s_0_77));
        // D s_0_80: and s_0_74 s_0_79
        let s_0_80: Bits = ((s_0_74) & (s_0_79));
        // D s_0_81: lsl s_0_80 s_0_72
        let s_0_81: Bits = s_0_80 << s_0_72;
        // C s_0_82: lsl s_0_79 s_0_72
        let s_0_82: Bits = s_0_79 << s_0_72;
        // C s_0_83: cmpl s_0_82
        let s_0_83: Bits = !s_0_82;
        // D s_0_84: and s_0_73 s_0_83
        let s_0_84: Bits = ((s_0_73) & (s_0_83));
        // D s_0_85: or s_0_84 s_0_81
        let s_0_85: Bits = ((s_0_84) | (s_0_81));
        // D s_0_86: cast reint s_0_85 -> u32
        let s_0_86: u32 = (s_0_85.value() as u32);
        // D s_0_87: read-var d:i64
        let s_0_87: i64 = fn_state.d;
        // D s_0_88: cast zx s_0_87 -> i
        let s_0_88: i128 = (i128::try_from(s_0_87).unwrap());
        // D s_0_89: call R_set(s_0_88, s_0_86)
        let s_0_89: () = R_set(state, tracer, s_0_88, s_0_86);
        // D s_0_90: read-var d:i64
        let s_0_90: i64 = fn_state.d;
        // D s_0_91: cast zx s_0_90 -> i
        let s_0_91: i128 = (i128::try_from(s_0_90).unwrap());
        // D s_0_92: call R_read(s_0_91)
        let s_0_92: u32 = R_read(state, tracer, s_0_91);
        // C s_0_93: const #16s : i
        let s_0_93: i128 = 16;
        // C s_0_94: const #1s : i
        let s_0_94: i128 = 1;
        // D s_0_95: cast zx s_0_63 -> i
        let s_0_95: i128 = (i128::try_from(s_0_63).unwrap());
        // D s_0_96: call integer_subrange(s_0_95, s_0_93, s_0_94)
        let s_0_96: Bits = integer_subrange(state, tracer, s_0_95, s_0_93, s_0_94);
        // D s_0_97: cast reint s_0_96 -> u16
        let s_0_97: u16 = (s_0_96.value() as u16);
        // C s_0_98: const #16s : i
        let s_0_98: i128 = 16;
        // D s_0_99: cast zx s_0_92 -> bv
        let s_0_99: Bits = Bits::new(s_0_92 as u128, 32u16);
        // D s_0_100: cast zx s_0_97 -> bv
        let s_0_100: Bits = Bits::new(s_0_97 as u128, 16u16);
        // C s_0_101: const #15s : i
        let s_0_101: i128 = 15;
        // C s_0_102: const #1u : u64
        let s_0_102: u64 = 1;
        // C s_0_103: cast zx s_0_102 -> bv
        let s_0_103: Bits = Bits::new(s_0_102 as u128, 64u16);
        // C s_0_104: lsl s_0_103 s_0_101
        let s_0_104: Bits = s_0_103 << s_0_101;
        // C s_0_105: sub s_0_104 s_0_103
        let s_0_105: Bits = ((s_0_104) - (s_0_103));
        // D s_0_106: and s_0_100 s_0_105
        let s_0_106: Bits = ((s_0_100) & (s_0_105));
        // D s_0_107: lsl s_0_106 s_0_98
        let s_0_107: Bits = s_0_106 << s_0_98;
        // C s_0_108: lsl s_0_105 s_0_98
        let s_0_108: Bits = s_0_105 << s_0_98;
        // C s_0_109: cmpl s_0_108
        let s_0_109: Bits = !s_0_108;
        // D s_0_110: and s_0_99 s_0_109
        let s_0_110: Bits = ((s_0_99) & (s_0_109));
        // D s_0_111: or s_0_110 s_0_107
        let s_0_111: Bits = ((s_0_110) | (s_0_107));
        // D s_0_112: cast reint s_0_111 -> u32
        let s_0_112: u32 = (s_0_111.value() as u32);
        // D s_0_113: read-var d:i64
        let s_0_113: i64 = fn_state.d;
        // D s_0_114: cast zx s_0_113 -> i
        let s_0_114: i128 = (i128::try_from(s_0_113).unwrap());
        // D s_0_115: call R_set(s_0_114, s_0_112)
        let s_0_115: () = R_set(state, tracer, s_0_114, s_0_112);
        // N s_0_116: return
        return;
    }
}
