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
pub fn execute_aarch32_instrs_SHSUB8_Op_A_txt<T: Tracer>(
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
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 8u16);
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
        // C s_0_17: const #0s : i
        let s_0_17: i128 = 0;
        // D s_0_18: cast zx s_0_16 -> bv
        let s_0_18: Bits = Bits::new(s_0_16 as u128, 32u16);
        // C s_0_19: const #1s : i64
        let s_0_19: i64 = 1;
        // C s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // C s_0_21: const #7s : i
        let s_0_21: i128 = 7;
        // C s_0_22: add s_0_21 s_0_20
        let s_0_22: i128 = (s_0_21 + s_0_20);
        // D s_0_23: bit-extract s_0_18 s_0_17 s_0_22
        let s_0_23: Bits = (Bits::new(
            ((s_0_18) >> (s_0_17)).value(),
            u16::try_from(s_0_22).unwrap(),
        ));
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: u8 = (s_0_23.value() as u8);
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 8u16);
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
        // C s_0_35: const #8s : i
        let s_0_35: i128 = 8;
        // D s_0_36: cast zx s_0_34 -> bv
        let s_0_36: Bits = Bits::new(s_0_34 as u128, 32u16);
        // C s_0_37: const #1s : i64
        let s_0_37: i64 = 1;
        // C s_0_38: cast zx s_0_37 -> i
        let s_0_38: i128 = (i128::try_from(s_0_37).unwrap());
        // C s_0_39: const #7s : i
        let s_0_39: i128 = 7;
        // C s_0_40: add s_0_39 s_0_38
        let s_0_40: i128 = (s_0_39 + s_0_38);
        // D s_0_41: bit-extract s_0_36 s_0_35 s_0_40
        let s_0_41: Bits = (Bits::new(
            ((s_0_36) >> (s_0_35)).value(),
            u16::try_from(s_0_40).unwrap(),
        ));
        // D s_0_42: cast reint s_0_41 -> u8
        let s_0_42: u8 = (s_0_41.value() as u8);
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 8u16);
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
        // C s_0_49: const #8s : i
        let s_0_49: i128 = 8;
        // D s_0_50: cast zx s_0_48 -> bv
        let s_0_50: Bits = Bits::new(s_0_48 as u128, 32u16);
        // C s_0_51: const #1s : i64
        let s_0_51: i64 = 1;
        // C s_0_52: cast zx s_0_51 -> i
        let s_0_52: i128 = (i128::try_from(s_0_51).unwrap());
        // C s_0_53: const #7s : i
        let s_0_53: i128 = 7;
        // C s_0_54: add s_0_53 s_0_52
        let s_0_54: i128 = (s_0_53 + s_0_52);
        // D s_0_55: bit-extract s_0_50 s_0_49 s_0_54
        let s_0_55: Bits = (Bits::new(
            ((s_0_50) >> (s_0_49)).value(),
            u16::try_from(s_0_54).unwrap(),
        ));
        // D s_0_56: cast reint s_0_55 -> u8
        let s_0_56: u8 = (s_0_55.value() as u8);
        // D s_0_57: cast zx s_0_56 -> bv
        let s_0_57: Bits = Bits::new(s_0_56 as u128, 8u16);
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
        // D s_0_62: sub s_0_60 s_0_61
        let s_0_62: i128 = ((s_0_60) - (s_0_61));
        // D s_0_63: cast reint s_0_62 -> i64
        let s_0_63: i64 = (s_0_62 as i64);
        // D s_0_64: read-var n:i64
        let s_0_64: i64 = fn_state.n;
        // D s_0_65: cast zx s_0_64 -> i
        let s_0_65: i128 = (i128::try_from(s_0_64).unwrap());
        // D s_0_66: call R_read(s_0_65)
        let s_0_66: u32 = R_read(state, tracer, s_0_65);
        // C s_0_67: const #16s : i
        let s_0_67: i128 = 16;
        // D s_0_68: cast zx s_0_66 -> bv
        let s_0_68: Bits = Bits::new(s_0_66 as u128, 32u16);
        // C s_0_69: const #1s : i64
        let s_0_69: i64 = 1;
        // C s_0_70: cast zx s_0_69 -> i
        let s_0_70: i128 = (i128::try_from(s_0_69).unwrap());
        // C s_0_71: const #7s : i
        let s_0_71: i128 = 7;
        // C s_0_72: add s_0_71 s_0_70
        let s_0_72: i128 = (s_0_71 + s_0_70);
        // D s_0_73: bit-extract s_0_68 s_0_67 s_0_72
        let s_0_73: Bits = (Bits::new(
            ((s_0_68) >> (s_0_67)).value(),
            u16::try_from(s_0_72).unwrap(),
        ));
        // D s_0_74: cast reint s_0_73 -> u8
        let s_0_74: u8 = (s_0_73.value() as u8);
        // D s_0_75: cast zx s_0_74 -> bv
        let s_0_75: Bits = Bits::new(s_0_74 as u128, 8u16);
        // D s_0_76: cast sx s_0_75 -> i
        let s_0_76: i128 = {
            let sign_bit = s_0_75.length() - 1;
            let mut result = s_0_75.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_77: cast reint s_0_76 -> i64
        let s_0_77: i64 = (s_0_76 as i64);
        // D s_0_78: read-var m:i64
        let s_0_78: i64 = fn_state.m;
        // D s_0_79: cast zx s_0_78 -> i
        let s_0_79: i128 = (i128::try_from(s_0_78).unwrap());
        // D s_0_80: call R_read(s_0_79)
        let s_0_80: u32 = R_read(state, tracer, s_0_79);
        // C s_0_81: const #16s : i
        let s_0_81: i128 = 16;
        // D s_0_82: cast zx s_0_80 -> bv
        let s_0_82: Bits = Bits::new(s_0_80 as u128, 32u16);
        // C s_0_83: const #1s : i64
        let s_0_83: i64 = 1;
        // C s_0_84: cast zx s_0_83 -> i
        let s_0_84: i128 = (i128::try_from(s_0_83).unwrap());
        // C s_0_85: const #7s : i
        let s_0_85: i128 = 7;
        // C s_0_86: add s_0_85 s_0_84
        let s_0_86: i128 = (s_0_85 + s_0_84);
        // D s_0_87: bit-extract s_0_82 s_0_81 s_0_86
        let s_0_87: Bits = (Bits::new(
            ((s_0_82) >> (s_0_81)).value(),
            u16::try_from(s_0_86).unwrap(),
        ));
        // D s_0_88: cast reint s_0_87 -> u8
        let s_0_88: u8 = (s_0_87.value() as u8);
        // D s_0_89: cast zx s_0_88 -> bv
        let s_0_89: Bits = Bits::new(s_0_88 as u128, 8u16);
        // D s_0_90: cast sx s_0_89 -> i
        let s_0_90: i128 = {
            let sign_bit = s_0_89.length() - 1;
            let mut result = s_0_89.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_91: cast reint s_0_90 -> i64
        let s_0_91: i64 = (s_0_90 as i64);
        // D s_0_92: cast zx s_0_77 -> i
        let s_0_92: i128 = (i128::try_from(s_0_77).unwrap());
        // D s_0_93: cast zx s_0_91 -> i
        let s_0_93: i128 = (i128::try_from(s_0_91).unwrap());
        // D s_0_94: sub s_0_92 s_0_93
        let s_0_94: i128 = ((s_0_92) - (s_0_93));
        // D s_0_95: cast reint s_0_94 -> i64
        let s_0_95: i64 = (s_0_94 as i64);
        // D s_0_96: read-var n:i64
        let s_0_96: i64 = fn_state.n;
        // D s_0_97: cast zx s_0_96 -> i
        let s_0_97: i128 = (i128::try_from(s_0_96).unwrap());
        // D s_0_98: call R_read(s_0_97)
        let s_0_98: u32 = R_read(state, tracer, s_0_97);
        // C s_0_99: const #24s : i
        let s_0_99: i128 = 24;
        // D s_0_100: cast zx s_0_98 -> bv
        let s_0_100: Bits = Bits::new(s_0_98 as u128, 32u16);
        // C s_0_101: const #1s : i64
        let s_0_101: i64 = 1;
        // C s_0_102: cast zx s_0_101 -> i
        let s_0_102: i128 = (i128::try_from(s_0_101).unwrap());
        // C s_0_103: const #7s : i
        let s_0_103: i128 = 7;
        // C s_0_104: add s_0_103 s_0_102
        let s_0_104: i128 = (s_0_103 + s_0_102);
        // D s_0_105: bit-extract s_0_100 s_0_99 s_0_104
        let s_0_105: Bits = (Bits::new(
            ((s_0_100) >> (s_0_99)).value(),
            u16::try_from(s_0_104).unwrap(),
        ));
        // D s_0_106: cast reint s_0_105 -> u8
        let s_0_106: u8 = (s_0_105.value() as u8);
        // D s_0_107: cast zx s_0_106 -> bv
        let s_0_107: Bits = Bits::new(s_0_106 as u128, 8u16);
        // D s_0_108: cast sx s_0_107 -> i
        let s_0_108: i128 = {
            let sign_bit = s_0_107.length() - 1;
            let mut result = s_0_107.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_109: cast reint s_0_108 -> i64
        let s_0_109: i64 = (s_0_108 as i64);
        // D s_0_110: read-var m:i64
        let s_0_110: i64 = fn_state.m;
        // D s_0_111: cast zx s_0_110 -> i
        let s_0_111: i128 = (i128::try_from(s_0_110).unwrap());
        // D s_0_112: call R_read(s_0_111)
        let s_0_112: u32 = R_read(state, tracer, s_0_111);
        // C s_0_113: const #24s : i
        let s_0_113: i128 = 24;
        // D s_0_114: cast zx s_0_112 -> bv
        let s_0_114: Bits = Bits::new(s_0_112 as u128, 32u16);
        // C s_0_115: const #1s : i64
        let s_0_115: i64 = 1;
        // C s_0_116: cast zx s_0_115 -> i
        let s_0_116: i128 = (i128::try_from(s_0_115).unwrap());
        // C s_0_117: const #7s : i
        let s_0_117: i128 = 7;
        // C s_0_118: add s_0_117 s_0_116
        let s_0_118: i128 = (s_0_117 + s_0_116);
        // D s_0_119: bit-extract s_0_114 s_0_113 s_0_118
        let s_0_119: Bits = (Bits::new(
            ((s_0_114) >> (s_0_113)).value(),
            u16::try_from(s_0_118).unwrap(),
        ));
        // D s_0_120: cast reint s_0_119 -> u8
        let s_0_120: u8 = (s_0_119.value() as u8);
        // D s_0_121: cast zx s_0_120 -> bv
        let s_0_121: Bits = Bits::new(s_0_120 as u128, 8u16);
        // D s_0_122: cast sx s_0_121 -> i
        let s_0_122: i128 = {
            let sign_bit = s_0_121.length() - 1;
            let mut result = s_0_121.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_123: cast reint s_0_122 -> i64
        let s_0_123: i64 = (s_0_122 as i64);
        // D s_0_124: cast zx s_0_109 -> i
        let s_0_124: i128 = (i128::try_from(s_0_109).unwrap());
        // D s_0_125: cast zx s_0_123 -> i
        let s_0_125: i128 = (i128::try_from(s_0_123).unwrap());
        // D s_0_126: sub s_0_124 s_0_125
        let s_0_126: i128 = ((s_0_124) - (s_0_125));
        // D s_0_127: cast reint s_0_126 -> i64
        let s_0_127: i64 = (s_0_126 as i64);
        // D s_0_128: read-var d:i64
        let s_0_128: i64 = fn_state.d;
        // D s_0_129: cast zx s_0_128 -> i
        let s_0_129: i128 = (i128::try_from(s_0_128).unwrap());
        // D s_0_130: call R_read(s_0_129)
        let s_0_130: u32 = R_read(state, tracer, s_0_129);
        // C s_0_131: const #8s : i
        let s_0_131: i128 = 8;
        // C s_0_132: const #1s : i
        let s_0_132: i128 = 1;
        // D s_0_133: cast zx s_0_31 -> i
        let s_0_133: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_134: call integer_subrange(s_0_133, s_0_131, s_0_132)
        let s_0_134: Bits = integer_subrange(state, tracer, s_0_133, s_0_131, s_0_132);
        // D s_0_135: cast reint s_0_134 -> u8
        let s_0_135: u8 = (s_0_134.value() as u8);
        // C s_0_136: const #0s : i
        let s_0_136: i128 = 0;
        // D s_0_137: cast zx s_0_130 -> bv
        let s_0_137: Bits = Bits::new(s_0_130 as u128, 32u16);
        // D s_0_138: cast zx s_0_135 -> bv
        let s_0_138: Bits = Bits::new(s_0_135 as u128, 8u16);
        // C s_0_139: const #7s : i
        let s_0_139: i128 = 7;
        // C s_0_140: const #1u : u64
        let s_0_140: u64 = 1;
        // C s_0_141: cast zx s_0_140 -> bv
        let s_0_141: Bits = Bits::new(s_0_140 as u128, 64u16);
        // C s_0_142: lsl s_0_141 s_0_139
        let s_0_142: Bits = s_0_141 << s_0_139;
        // C s_0_143: sub s_0_142 s_0_141
        let s_0_143: Bits = ((s_0_142) - (s_0_141));
        // D s_0_144: and s_0_138 s_0_143
        let s_0_144: Bits = ((s_0_138) & (s_0_143));
        // D s_0_145: lsl s_0_144 s_0_136
        let s_0_145: Bits = s_0_144 << s_0_136;
        // C s_0_146: lsl s_0_143 s_0_136
        let s_0_146: Bits = s_0_143 << s_0_136;
        // C s_0_147: cmpl s_0_146
        let s_0_147: Bits = !s_0_146;
        // D s_0_148: and s_0_137 s_0_147
        let s_0_148: Bits = ((s_0_137) & (s_0_147));
        // D s_0_149: or s_0_148 s_0_145
        let s_0_149: Bits = ((s_0_148) | (s_0_145));
        // D s_0_150: cast reint s_0_149 -> u32
        let s_0_150: u32 = (s_0_149.value() as u32);
        // D s_0_151: read-var d:i64
        let s_0_151: i64 = fn_state.d;
        // D s_0_152: cast zx s_0_151 -> i
        let s_0_152: i128 = (i128::try_from(s_0_151).unwrap());
        // D s_0_153: call R_set(s_0_152, s_0_150)
        let s_0_153: () = R_set(state, tracer, s_0_152, s_0_150);
        // D s_0_154: read-var d:i64
        let s_0_154: i64 = fn_state.d;
        // D s_0_155: cast zx s_0_154 -> i
        let s_0_155: i128 = (i128::try_from(s_0_154).unwrap());
        // D s_0_156: call R_read(s_0_155)
        let s_0_156: u32 = R_read(state, tracer, s_0_155);
        // C s_0_157: const #8s : i
        let s_0_157: i128 = 8;
        // C s_0_158: const #1s : i
        let s_0_158: i128 = 1;
        // D s_0_159: cast zx s_0_63 -> i
        let s_0_159: i128 = (i128::try_from(s_0_63).unwrap());
        // D s_0_160: call integer_subrange(s_0_159, s_0_157, s_0_158)
        let s_0_160: Bits = integer_subrange(state, tracer, s_0_159, s_0_157, s_0_158);
        // D s_0_161: cast reint s_0_160 -> u8
        let s_0_161: u8 = (s_0_160.value() as u8);
        // C s_0_162: const #8s : i
        let s_0_162: i128 = 8;
        // D s_0_163: cast zx s_0_156 -> bv
        let s_0_163: Bits = Bits::new(s_0_156 as u128, 32u16);
        // D s_0_164: cast zx s_0_161 -> bv
        let s_0_164: Bits = Bits::new(s_0_161 as u128, 8u16);
        // C s_0_165: const #7s : i
        let s_0_165: i128 = 7;
        // C s_0_166: const #1u : u64
        let s_0_166: u64 = 1;
        // C s_0_167: cast zx s_0_166 -> bv
        let s_0_167: Bits = Bits::new(s_0_166 as u128, 64u16);
        // C s_0_168: lsl s_0_167 s_0_165
        let s_0_168: Bits = s_0_167 << s_0_165;
        // C s_0_169: sub s_0_168 s_0_167
        let s_0_169: Bits = ((s_0_168) - (s_0_167));
        // D s_0_170: and s_0_164 s_0_169
        let s_0_170: Bits = ((s_0_164) & (s_0_169));
        // D s_0_171: lsl s_0_170 s_0_162
        let s_0_171: Bits = s_0_170 << s_0_162;
        // C s_0_172: lsl s_0_169 s_0_162
        let s_0_172: Bits = s_0_169 << s_0_162;
        // C s_0_173: cmpl s_0_172
        let s_0_173: Bits = !s_0_172;
        // D s_0_174: and s_0_163 s_0_173
        let s_0_174: Bits = ((s_0_163) & (s_0_173));
        // D s_0_175: or s_0_174 s_0_171
        let s_0_175: Bits = ((s_0_174) | (s_0_171));
        // D s_0_176: cast reint s_0_175 -> u32
        let s_0_176: u32 = (s_0_175.value() as u32);
        // D s_0_177: read-var d:i64
        let s_0_177: i64 = fn_state.d;
        // D s_0_178: cast zx s_0_177 -> i
        let s_0_178: i128 = (i128::try_from(s_0_177).unwrap());
        // D s_0_179: call R_set(s_0_178, s_0_176)
        let s_0_179: () = R_set(state, tracer, s_0_178, s_0_176);
        // D s_0_180: read-var d:i64
        let s_0_180: i64 = fn_state.d;
        // D s_0_181: cast zx s_0_180 -> i
        let s_0_181: i128 = (i128::try_from(s_0_180).unwrap());
        // D s_0_182: call R_read(s_0_181)
        let s_0_182: u32 = R_read(state, tracer, s_0_181);
        // C s_0_183: const #8s : i
        let s_0_183: i128 = 8;
        // C s_0_184: const #1s : i
        let s_0_184: i128 = 1;
        // D s_0_185: cast zx s_0_95 -> i
        let s_0_185: i128 = (i128::try_from(s_0_95).unwrap());
        // D s_0_186: call integer_subrange(s_0_185, s_0_183, s_0_184)
        let s_0_186: Bits = integer_subrange(state, tracer, s_0_185, s_0_183, s_0_184);
        // D s_0_187: cast reint s_0_186 -> u8
        let s_0_187: u8 = (s_0_186.value() as u8);
        // C s_0_188: const #16s : i
        let s_0_188: i128 = 16;
        // D s_0_189: cast zx s_0_182 -> bv
        let s_0_189: Bits = Bits::new(s_0_182 as u128, 32u16);
        // D s_0_190: cast zx s_0_187 -> bv
        let s_0_190: Bits = Bits::new(s_0_187 as u128, 8u16);
        // C s_0_191: const #7s : i
        let s_0_191: i128 = 7;
        // C s_0_192: const #1u : u64
        let s_0_192: u64 = 1;
        // C s_0_193: cast zx s_0_192 -> bv
        let s_0_193: Bits = Bits::new(s_0_192 as u128, 64u16);
        // C s_0_194: lsl s_0_193 s_0_191
        let s_0_194: Bits = s_0_193 << s_0_191;
        // C s_0_195: sub s_0_194 s_0_193
        let s_0_195: Bits = ((s_0_194) - (s_0_193));
        // D s_0_196: and s_0_190 s_0_195
        let s_0_196: Bits = ((s_0_190) & (s_0_195));
        // D s_0_197: lsl s_0_196 s_0_188
        let s_0_197: Bits = s_0_196 << s_0_188;
        // C s_0_198: lsl s_0_195 s_0_188
        let s_0_198: Bits = s_0_195 << s_0_188;
        // C s_0_199: cmpl s_0_198
        let s_0_199: Bits = !s_0_198;
        // D s_0_200: and s_0_189 s_0_199
        let s_0_200: Bits = ((s_0_189) & (s_0_199));
        // D s_0_201: or s_0_200 s_0_197
        let s_0_201: Bits = ((s_0_200) | (s_0_197));
        // D s_0_202: cast reint s_0_201 -> u32
        let s_0_202: u32 = (s_0_201.value() as u32);
        // D s_0_203: read-var d:i64
        let s_0_203: i64 = fn_state.d;
        // D s_0_204: cast zx s_0_203 -> i
        let s_0_204: i128 = (i128::try_from(s_0_203).unwrap());
        // D s_0_205: call R_set(s_0_204, s_0_202)
        let s_0_205: () = R_set(state, tracer, s_0_204, s_0_202);
        // D s_0_206: read-var d:i64
        let s_0_206: i64 = fn_state.d;
        // D s_0_207: cast zx s_0_206 -> i
        let s_0_207: i128 = (i128::try_from(s_0_206).unwrap());
        // D s_0_208: call R_read(s_0_207)
        let s_0_208: u32 = R_read(state, tracer, s_0_207);
        // C s_0_209: const #8s : i
        let s_0_209: i128 = 8;
        // C s_0_210: const #1s : i
        let s_0_210: i128 = 1;
        // D s_0_211: cast zx s_0_127 -> i
        let s_0_211: i128 = (i128::try_from(s_0_127).unwrap());
        // D s_0_212: call integer_subrange(s_0_211, s_0_209, s_0_210)
        let s_0_212: Bits = integer_subrange(state, tracer, s_0_211, s_0_209, s_0_210);
        // D s_0_213: cast reint s_0_212 -> u8
        let s_0_213: u8 = (s_0_212.value() as u8);
        // C s_0_214: const #24s : i
        let s_0_214: i128 = 24;
        // D s_0_215: cast zx s_0_208 -> bv
        let s_0_215: Bits = Bits::new(s_0_208 as u128, 32u16);
        // D s_0_216: cast zx s_0_213 -> bv
        let s_0_216: Bits = Bits::new(s_0_213 as u128, 8u16);
        // C s_0_217: const #7s : i
        let s_0_217: i128 = 7;
        // C s_0_218: const #1u : u64
        let s_0_218: u64 = 1;
        // C s_0_219: cast zx s_0_218 -> bv
        let s_0_219: Bits = Bits::new(s_0_218 as u128, 64u16);
        // C s_0_220: lsl s_0_219 s_0_217
        let s_0_220: Bits = s_0_219 << s_0_217;
        // C s_0_221: sub s_0_220 s_0_219
        let s_0_221: Bits = ((s_0_220) - (s_0_219));
        // D s_0_222: and s_0_216 s_0_221
        let s_0_222: Bits = ((s_0_216) & (s_0_221));
        // D s_0_223: lsl s_0_222 s_0_214
        let s_0_223: Bits = s_0_222 << s_0_214;
        // C s_0_224: lsl s_0_221 s_0_214
        let s_0_224: Bits = s_0_221 << s_0_214;
        // C s_0_225: cmpl s_0_224
        let s_0_225: Bits = !s_0_224;
        // D s_0_226: and s_0_215 s_0_225
        let s_0_226: Bits = ((s_0_215) & (s_0_225));
        // D s_0_227: or s_0_226 s_0_223
        let s_0_227: Bits = ((s_0_226) | (s_0_223));
        // D s_0_228: cast reint s_0_227 -> u32
        let s_0_228: u32 = (s_0_227.value() as u32);
        // D s_0_229: read-var d:i64
        let s_0_229: i64 = fn_state.d;
        // D s_0_230: cast zx s_0_229 -> i
        let s_0_230: i128 = (i128::try_from(s_0_229).unwrap());
        // D s_0_231: call R_set(s_0_230, s_0_228)
        let s_0_231: () = R_set(state, tracer, s_0_230, s_0_228);
        // N s_0_232: return
        return;
    }
}
