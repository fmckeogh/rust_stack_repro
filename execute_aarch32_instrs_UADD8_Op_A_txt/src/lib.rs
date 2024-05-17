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
use Bit::*;
use R_read::*;
use R_set::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_UADD8_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        sum4: i64,
        ga_348895: bool,
        ga_348889: bool,
        ga_348901: bool,
        sum3: i64,
        sum2: i64,
        ga_348907: bool,
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
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
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
        // D s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (s_0_25.value() as i128);
        // D s_0_27: cast reint s_0_26 -> i64
        let s_0_27: i64 = (s_0_26 as i64);
        // D s_0_28: cast zx s_0_13 -> i
        let s_0_28: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_29: cast zx s_0_27 -> i
        let s_0_29: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_30: add s_0_28 s_0_29
        let s_0_30: i128 = (s_0_28 + s_0_29);
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
        // D s_0_44: cast zx s_0_43 -> i
        let s_0_44: i128 = (s_0_43.value() as i128);
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
        // D s_0_58: cast zx s_0_57 -> i
        let s_0_58: i128 = (s_0_57.value() as i128);
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
        // D s_0_64: write-var sum2 <= s_0_63
        fn_state.sum2 = s_0_63;
        // D s_0_65: read-var n:i64
        let s_0_65: i64 = fn_state.n;
        // D s_0_66: cast zx s_0_65 -> i
        let s_0_66: i128 = (i128::try_from(s_0_65).unwrap());
        // D s_0_67: call R_read(s_0_66)
        let s_0_67: u32 = R_read(state, tracer, s_0_66);
        // C s_0_68: const #16s : i
        let s_0_68: i128 = 16;
        // D s_0_69: cast zx s_0_67 -> bv
        let s_0_69: Bits = Bits::new(s_0_67 as u128, 32u16);
        // C s_0_70: const #1s : i64
        let s_0_70: i64 = 1;
        // C s_0_71: cast zx s_0_70 -> i
        let s_0_71: i128 = (i128::try_from(s_0_70).unwrap());
        // C s_0_72: const #7s : i
        let s_0_72: i128 = 7;
        // C s_0_73: add s_0_72 s_0_71
        let s_0_73: i128 = (s_0_72 + s_0_71);
        // D s_0_74: bit-extract s_0_69 s_0_68 s_0_73
        let s_0_74: Bits = (Bits::new(
            ((s_0_69) >> (s_0_68)).value(),
            u16::try_from(s_0_73).unwrap(),
        ));
        // D s_0_75: cast reint s_0_74 -> u8
        let s_0_75: u8 = (s_0_74.value() as u8);
        // D s_0_76: cast zx s_0_75 -> bv
        let s_0_76: Bits = Bits::new(s_0_75 as u128, 8u16);
        // D s_0_77: cast zx s_0_76 -> i
        let s_0_77: i128 = (s_0_76.value() as i128);
        // D s_0_78: cast reint s_0_77 -> i64
        let s_0_78: i64 = (s_0_77 as i64);
        // D s_0_79: read-var m:i64
        let s_0_79: i64 = fn_state.m;
        // D s_0_80: cast zx s_0_79 -> i
        let s_0_80: i128 = (i128::try_from(s_0_79).unwrap());
        // D s_0_81: call R_read(s_0_80)
        let s_0_81: u32 = R_read(state, tracer, s_0_80);
        // C s_0_82: const #16s : i
        let s_0_82: i128 = 16;
        // D s_0_83: cast zx s_0_81 -> bv
        let s_0_83: Bits = Bits::new(s_0_81 as u128, 32u16);
        // C s_0_84: const #1s : i64
        let s_0_84: i64 = 1;
        // C s_0_85: cast zx s_0_84 -> i
        let s_0_85: i128 = (i128::try_from(s_0_84).unwrap());
        // C s_0_86: const #7s : i
        let s_0_86: i128 = 7;
        // C s_0_87: add s_0_86 s_0_85
        let s_0_87: i128 = (s_0_86 + s_0_85);
        // D s_0_88: bit-extract s_0_83 s_0_82 s_0_87
        let s_0_88: Bits = (Bits::new(
            ((s_0_83) >> (s_0_82)).value(),
            u16::try_from(s_0_87).unwrap(),
        ));
        // D s_0_89: cast reint s_0_88 -> u8
        let s_0_89: u8 = (s_0_88.value() as u8);
        // D s_0_90: cast zx s_0_89 -> bv
        let s_0_90: Bits = Bits::new(s_0_89 as u128, 8u16);
        // D s_0_91: cast zx s_0_90 -> i
        let s_0_91: i128 = (s_0_90.value() as i128);
        // D s_0_92: cast reint s_0_91 -> i64
        let s_0_92: i64 = (s_0_91 as i64);
        // D s_0_93: cast zx s_0_78 -> i
        let s_0_93: i128 = (i128::try_from(s_0_78).unwrap());
        // D s_0_94: cast zx s_0_92 -> i
        let s_0_94: i128 = (i128::try_from(s_0_92).unwrap());
        // D s_0_95: add s_0_93 s_0_94
        let s_0_95: i128 = (s_0_93 + s_0_94);
        // D s_0_96: cast reint s_0_95 -> i64
        let s_0_96: i64 = (s_0_95 as i64);
        // D s_0_97: write-var sum3 <= s_0_96
        fn_state.sum3 = s_0_96;
        // D s_0_98: read-var n:i64
        let s_0_98: i64 = fn_state.n;
        // D s_0_99: cast zx s_0_98 -> i
        let s_0_99: i128 = (i128::try_from(s_0_98).unwrap());
        // D s_0_100: call R_read(s_0_99)
        let s_0_100: u32 = R_read(state, tracer, s_0_99);
        // C s_0_101: const #24s : i
        let s_0_101: i128 = 24;
        // D s_0_102: cast zx s_0_100 -> bv
        let s_0_102: Bits = Bits::new(s_0_100 as u128, 32u16);
        // C s_0_103: const #1s : i64
        let s_0_103: i64 = 1;
        // C s_0_104: cast zx s_0_103 -> i
        let s_0_104: i128 = (i128::try_from(s_0_103).unwrap());
        // C s_0_105: const #7s : i
        let s_0_105: i128 = 7;
        // C s_0_106: add s_0_105 s_0_104
        let s_0_106: i128 = (s_0_105 + s_0_104);
        // D s_0_107: bit-extract s_0_102 s_0_101 s_0_106
        let s_0_107: Bits = (Bits::new(
            ((s_0_102) >> (s_0_101)).value(),
            u16::try_from(s_0_106).unwrap(),
        ));
        // D s_0_108: cast reint s_0_107 -> u8
        let s_0_108: u8 = (s_0_107.value() as u8);
        // D s_0_109: cast zx s_0_108 -> bv
        let s_0_109: Bits = Bits::new(s_0_108 as u128, 8u16);
        // D s_0_110: cast zx s_0_109 -> i
        let s_0_110: i128 = (s_0_109.value() as i128);
        // D s_0_111: cast reint s_0_110 -> i64
        let s_0_111: i64 = (s_0_110 as i64);
        // D s_0_112: read-var m:i64
        let s_0_112: i64 = fn_state.m;
        // D s_0_113: cast zx s_0_112 -> i
        let s_0_113: i128 = (i128::try_from(s_0_112).unwrap());
        // D s_0_114: call R_read(s_0_113)
        let s_0_114: u32 = R_read(state, tracer, s_0_113);
        // C s_0_115: const #24s : i
        let s_0_115: i128 = 24;
        // D s_0_116: cast zx s_0_114 -> bv
        let s_0_116: Bits = Bits::new(s_0_114 as u128, 32u16);
        // C s_0_117: const #1s : i64
        let s_0_117: i64 = 1;
        // C s_0_118: cast zx s_0_117 -> i
        let s_0_118: i128 = (i128::try_from(s_0_117).unwrap());
        // C s_0_119: const #7s : i
        let s_0_119: i128 = 7;
        // C s_0_120: add s_0_119 s_0_118
        let s_0_120: i128 = (s_0_119 + s_0_118);
        // D s_0_121: bit-extract s_0_116 s_0_115 s_0_120
        let s_0_121: Bits = (Bits::new(
            ((s_0_116) >> (s_0_115)).value(),
            u16::try_from(s_0_120).unwrap(),
        ));
        // D s_0_122: cast reint s_0_121 -> u8
        let s_0_122: u8 = (s_0_121.value() as u8);
        // D s_0_123: cast zx s_0_122 -> bv
        let s_0_123: Bits = Bits::new(s_0_122 as u128, 8u16);
        // D s_0_124: cast zx s_0_123 -> i
        let s_0_124: i128 = (s_0_123.value() as i128);
        // D s_0_125: cast reint s_0_124 -> i64
        let s_0_125: i64 = (s_0_124 as i64);
        // D s_0_126: cast zx s_0_111 -> i
        let s_0_126: i128 = (i128::try_from(s_0_111).unwrap());
        // D s_0_127: cast zx s_0_125 -> i
        let s_0_127: i128 = (i128::try_from(s_0_125).unwrap());
        // D s_0_128: add s_0_126 s_0_127
        let s_0_128: i128 = (s_0_126 + s_0_127);
        // D s_0_129: cast reint s_0_128 -> i64
        let s_0_129: i64 = (s_0_128 as i64);
        // D s_0_130: write-var sum4 <= s_0_129
        fn_state.sum4 = s_0_129;
        // D s_0_131: read-var d:i64
        let s_0_131: i64 = fn_state.d;
        // D s_0_132: cast zx s_0_131 -> i
        let s_0_132: i128 = (i128::try_from(s_0_131).unwrap());
        // D s_0_133: call R_read(s_0_132)
        let s_0_133: u32 = R_read(state, tracer, s_0_132);
        // C s_0_134: const #7s : i
        let s_0_134: i128 = 7;
        // C s_0_135: const #0s : i
        let s_0_135: i128 = 0;
        // D s_0_136: cast zx s_0_31 -> i
        let s_0_136: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_137: call integer_subrange(s_0_136, s_0_134, s_0_135)
        let s_0_137: Bits = integer_subrange(state, tracer, s_0_136, s_0_134, s_0_135);
        // D s_0_138: cast reint s_0_137 -> u8
        let s_0_138: u8 = (s_0_137.value() as u8);
        // C s_0_139: const #0s : i
        let s_0_139: i128 = 0;
        // D s_0_140: cast zx s_0_133 -> bv
        let s_0_140: Bits = Bits::new(s_0_133 as u128, 32u16);
        // D s_0_141: cast zx s_0_138 -> bv
        let s_0_141: Bits = Bits::new(s_0_138 as u128, 8u16);
        // C s_0_142: const #7s : i
        let s_0_142: i128 = 7;
        // C s_0_143: const #1u : u64
        let s_0_143: u64 = 1;
        // C s_0_144: cast zx s_0_143 -> bv
        let s_0_144: Bits = Bits::new(s_0_143 as u128, 64u16);
        // C s_0_145: lsl s_0_144 s_0_142
        let s_0_145: Bits = s_0_144 << s_0_142;
        // C s_0_146: sub s_0_145 s_0_144
        let s_0_146: Bits = ((s_0_145) - (s_0_144));
        // D s_0_147: and s_0_141 s_0_146
        let s_0_147: Bits = ((s_0_141) & (s_0_146));
        // D s_0_148: lsl s_0_147 s_0_139
        let s_0_148: Bits = s_0_147 << s_0_139;
        // C s_0_149: lsl s_0_146 s_0_139
        let s_0_149: Bits = s_0_146 << s_0_139;
        // C s_0_150: cmpl s_0_149
        let s_0_150: Bits = !s_0_149;
        // D s_0_151: and s_0_140 s_0_150
        let s_0_151: Bits = ((s_0_140) & (s_0_150));
        // D s_0_152: or s_0_151 s_0_148
        let s_0_152: Bits = ((s_0_151) | (s_0_148));
        // D s_0_153: cast reint s_0_152 -> u32
        let s_0_153: u32 = (s_0_152.value() as u32);
        // D s_0_154: read-var d:i64
        let s_0_154: i64 = fn_state.d;
        // D s_0_155: cast zx s_0_154 -> i
        let s_0_155: i128 = (i128::try_from(s_0_154).unwrap());
        // D s_0_156: call R_set(s_0_155, s_0_153)
        let s_0_156: () = R_set(state, tracer, s_0_155, s_0_153);
        // D s_0_157: read-var d:i64
        let s_0_157: i64 = fn_state.d;
        // D s_0_158: cast zx s_0_157 -> i
        let s_0_158: i128 = (i128::try_from(s_0_157).unwrap());
        // D s_0_159: call R_read(s_0_158)
        let s_0_159: u32 = R_read(state, tracer, s_0_158);
        // C s_0_160: const #7s : i
        let s_0_160: i128 = 7;
        // C s_0_161: const #0s : i
        let s_0_161: i128 = 0;
        // D s_0_162: read-var sum2:i64
        let s_0_162: i64 = fn_state.sum2;
        // D s_0_163: cast zx s_0_162 -> i
        let s_0_163: i128 = (i128::try_from(s_0_162).unwrap());
        // D s_0_164: call integer_subrange(s_0_163, s_0_160, s_0_161)
        let s_0_164: Bits = integer_subrange(state, tracer, s_0_163, s_0_160, s_0_161);
        // D s_0_165: cast reint s_0_164 -> u8
        let s_0_165: u8 = (s_0_164.value() as u8);
        // C s_0_166: const #8s : i
        let s_0_166: i128 = 8;
        // D s_0_167: cast zx s_0_159 -> bv
        let s_0_167: Bits = Bits::new(s_0_159 as u128, 32u16);
        // D s_0_168: cast zx s_0_165 -> bv
        let s_0_168: Bits = Bits::new(s_0_165 as u128, 8u16);
        // C s_0_169: const #7s : i
        let s_0_169: i128 = 7;
        // C s_0_170: const #1u : u64
        let s_0_170: u64 = 1;
        // C s_0_171: cast zx s_0_170 -> bv
        let s_0_171: Bits = Bits::new(s_0_170 as u128, 64u16);
        // C s_0_172: lsl s_0_171 s_0_169
        let s_0_172: Bits = s_0_171 << s_0_169;
        // C s_0_173: sub s_0_172 s_0_171
        let s_0_173: Bits = ((s_0_172) - (s_0_171));
        // D s_0_174: and s_0_168 s_0_173
        let s_0_174: Bits = ((s_0_168) & (s_0_173));
        // D s_0_175: lsl s_0_174 s_0_166
        let s_0_175: Bits = s_0_174 << s_0_166;
        // C s_0_176: lsl s_0_173 s_0_166
        let s_0_176: Bits = s_0_173 << s_0_166;
        // C s_0_177: cmpl s_0_176
        let s_0_177: Bits = !s_0_176;
        // D s_0_178: and s_0_167 s_0_177
        let s_0_178: Bits = ((s_0_167) & (s_0_177));
        // D s_0_179: or s_0_178 s_0_175
        let s_0_179: Bits = ((s_0_178) | (s_0_175));
        // D s_0_180: cast reint s_0_179 -> u32
        let s_0_180: u32 = (s_0_179.value() as u32);
        // D s_0_181: read-var d:i64
        let s_0_181: i64 = fn_state.d;
        // D s_0_182: cast zx s_0_181 -> i
        let s_0_182: i128 = (i128::try_from(s_0_181).unwrap());
        // D s_0_183: call R_set(s_0_182, s_0_180)
        let s_0_183: () = R_set(state, tracer, s_0_182, s_0_180);
        // D s_0_184: read-var d:i64
        let s_0_184: i64 = fn_state.d;
        // D s_0_185: cast zx s_0_184 -> i
        let s_0_185: i128 = (i128::try_from(s_0_184).unwrap());
        // D s_0_186: call R_read(s_0_185)
        let s_0_186: u32 = R_read(state, tracer, s_0_185);
        // C s_0_187: const #7s : i
        let s_0_187: i128 = 7;
        // C s_0_188: const #0s : i
        let s_0_188: i128 = 0;
        // D s_0_189: read-var sum3:i64
        let s_0_189: i64 = fn_state.sum3;
        // D s_0_190: cast zx s_0_189 -> i
        let s_0_190: i128 = (i128::try_from(s_0_189).unwrap());
        // D s_0_191: call integer_subrange(s_0_190, s_0_187, s_0_188)
        let s_0_191: Bits = integer_subrange(state, tracer, s_0_190, s_0_187, s_0_188);
        // D s_0_192: cast reint s_0_191 -> u8
        let s_0_192: u8 = (s_0_191.value() as u8);
        // C s_0_193: const #16s : i
        let s_0_193: i128 = 16;
        // D s_0_194: cast zx s_0_186 -> bv
        let s_0_194: Bits = Bits::new(s_0_186 as u128, 32u16);
        // D s_0_195: cast zx s_0_192 -> bv
        let s_0_195: Bits = Bits::new(s_0_192 as u128, 8u16);
        // C s_0_196: const #7s : i
        let s_0_196: i128 = 7;
        // C s_0_197: const #1u : u64
        let s_0_197: u64 = 1;
        // C s_0_198: cast zx s_0_197 -> bv
        let s_0_198: Bits = Bits::new(s_0_197 as u128, 64u16);
        // C s_0_199: lsl s_0_198 s_0_196
        let s_0_199: Bits = s_0_198 << s_0_196;
        // C s_0_200: sub s_0_199 s_0_198
        let s_0_200: Bits = ((s_0_199) - (s_0_198));
        // D s_0_201: and s_0_195 s_0_200
        let s_0_201: Bits = ((s_0_195) & (s_0_200));
        // D s_0_202: lsl s_0_201 s_0_193
        let s_0_202: Bits = s_0_201 << s_0_193;
        // C s_0_203: lsl s_0_200 s_0_193
        let s_0_203: Bits = s_0_200 << s_0_193;
        // C s_0_204: cmpl s_0_203
        let s_0_204: Bits = !s_0_203;
        // D s_0_205: and s_0_194 s_0_204
        let s_0_205: Bits = ((s_0_194) & (s_0_204));
        // D s_0_206: or s_0_205 s_0_202
        let s_0_206: Bits = ((s_0_205) | (s_0_202));
        // D s_0_207: cast reint s_0_206 -> u32
        let s_0_207: u32 = (s_0_206.value() as u32);
        // D s_0_208: read-var d:i64
        let s_0_208: i64 = fn_state.d;
        // D s_0_209: cast zx s_0_208 -> i
        let s_0_209: i128 = (i128::try_from(s_0_208).unwrap());
        // D s_0_210: call R_set(s_0_209, s_0_207)
        let s_0_210: () = R_set(state, tracer, s_0_209, s_0_207);
        // D s_0_211: read-var d:i64
        let s_0_211: i64 = fn_state.d;
        // D s_0_212: cast zx s_0_211 -> i
        let s_0_212: i128 = (i128::try_from(s_0_211).unwrap());
        // D s_0_213: call R_read(s_0_212)
        let s_0_213: u32 = R_read(state, tracer, s_0_212);
        // C s_0_214: const #7s : i
        let s_0_214: i128 = 7;
        // C s_0_215: const #0s : i
        let s_0_215: i128 = 0;
        // D s_0_216: read-var sum4:i64
        let s_0_216: i64 = fn_state.sum4;
        // D s_0_217: cast zx s_0_216 -> i
        let s_0_217: i128 = (i128::try_from(s_0_216).unwrap());
        // D s_0_218: call integer_subrange(s_0_217, s_0_214, s_0_215)
        let s_0_218: Bits = integer_subrange(state, tracer, s_0_217, s_0_214, s_0_215);
        // D s_0_219: cast reint s_0_218 -> u8
        let s_0_219: u8 = (s_0_218.value() as u8);
        // C s_0_220: const #24s : i
        let s_0_220: i128 = 24;
        // D s_0_221: cast zx s_0_213 -> bv
        let s_0_221: Bits = Bits::new(s_0_213 as u128, 32u16);
        // D s_0_222: cast zx s_0_219 -> bv
        let s_0_222: Bits = Bits::new(s_0_219 as u128, 8u16);
        // C s_0_223: const #7s : i
        let s_0_223: i128 = 7;
        // C s_0_224: const #1u : u64
        let s_0_224: u64 = 1;
        // C s_0_225: cast zx s_0_224 -> bv
        let s_0_225: Bits = Bits::new(s_0_224 as u128, 64u16);
        // C s_0_226: lsl s_0_225 s_0_223
        let s_0_226: Bits = s_0_225 << s_0_223;
        // C s_0_227: sub s_0_226 s_0_225
        let s_0_227: Bits = ((s_0_226) - (s_0_225));
        // D s_0_228: and s_0_222 s_0_227
        let s_0_228: Bits = ((s_0_222) & (s_0_227));
        // D s_0_229: lsl s_0_228 s_0_220
        let s_0_229: Bits = s_0_228 << s_0_220;
        // C s_0_230: lsl s_0_227 s_0_220
        let s_0_230: Bits = s_0_227 << s_0_220;
        // C s_0_231: cmpl s_0_230
        let s_0_231: Bits = !s_0_230;
        // D s_0_232: and s_0_221 s_0_231
        let s_0_232: Bits = ((s_0_221) & (s_0_231));
        // D s_0_233: or s_0_232 s_0_229
        let s_0_233: Bits = ((s_0_232) | (s_0_229));
        // D s_0_234: cast reint s_0_233 -> u32
        let s_0_234: u32 = (s_0_233.value() as u32);
        // D s_0_235: read-var d:i64
        let s_0_235: i64 = fn_state.d;
        // D s_0_236: cast zx s_0_235 -> i
        let s_0_236: i128 = (i128::try_from(s_0_235).unwrap());
        // D s_0_237: call R_set(s_0_236, s_0_234)
        let s_0_237: () = R_set(state, tracer, s_0_236, s_0_234);
        // C s_0_238: const #256u : u12
        let s_0_238: u16 = 256;
        // C s_0_239: cast zx s_0_238 -> bv
        let s_0_239: Bits = Bits::new(s_0_238 as u128, 12u16);
        // C s_0_240: cast zx s_0_239 -> i
        let s_0_240: i128 = (s_0_239.value() as i128);
        // C s_0_241: cast reint s_0_240 -> i64
        let s_0_241: i64 = (s_0_240 as i64);
        // D s_0_242: cast zx s_0_31 -> i
        let s_0_242: i128 = (i128::try_from(s_0_31).unwrap());
        // C s_0_243: cast zx s_0_241 -> i
        let s_0_243: i128 = (i128::try_from(s_0_241).unwrap());
        // D s_0_244: cmp-ge s_0_242 s_0_243
        let s_0_244: bool = ((s_0_242) >= (s_0_243));
        // N s_0_245: branch s_0_244 b12 b1
        if s_0_244 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var ga#348889 <= s_1_0
        fn_state.ga_348889 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#348889:u8
        let s_2_0: bool = fn_state.ga_348889;
        // D s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // C s_2_2: const #16968u : u32
        let s_2_2: u32 = 16968;
        // D s_2_3: read-reg s_2_2:struct
        let s_2_3: ProductTypec98939056e929b9c = {
            let value = state
                .read_register::<ProductTypec98939056e929b9c>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // C s_2_4: const #16968u : u32
        let s_2_4: u32 = 16968;
        // N s_2_5: write-reg s_2_4 <= s_2_3
        let s_2_5: () = {
            state.write_register::<ProductTypec98939056e929b9c>(s_2_4 as isize, s_2_3);
            tracer.write_register(s_2_4 as isize, s_2_3);
        };
        // C s_2_6: const #256u : u12
        let s_2_6: u16 = 256;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 12u16);
        // C s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // C s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var sum2:i64
        let s_2_10: i64 = fn_state.sum2;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // C s_2_12: cast zx s_2_9 -> i
        let s_2_12: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_13: cmp-ge s_2_11 s_2_12
        let s_2_13: bool = ((s_2_11) >= (s_2_12));
        // N s_2_14: branch s_2_13 b11 b3
        if s_2_13 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var ga#348895 <= s_3_0
        fn_state.ga_348895 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#348895:u8
        let s_4_0: bool = fn_state.ga_348895;
        // D s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // C s_4_2: const #16968u : u32
        let s_4_2: u32 = 16968;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductTypec98939056e929b9c = {
            let value = state
                .read_register::<ProductTypec98939056e929b9c>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #16968u : u32
        let s_4_4: u32 = 16968;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<ProductTypec98939056e929b9c>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // C s_4_6: const #256u : u12
        let s_4_6: u16 = 256;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 12u16);
        // C s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (s_4_7.value() as i128);
        // C s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: read-var sum3:i64
        let s_4_10: i64 = fn_state.sum3;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // C s_4_12: cast zx s_4_9 -> i
        let s_4_12: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_13: cmp-ge s_4_11 s_4_12
        let s_4_13: bool = ((s_4_11) >= (s_4_12));
        // N s_4_14: branch s_4_13 b10 b5
        if s_4_13 {
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
        // D s_5_1: write-var ga#348901 <= s_5_0
        fn_state.ga_348901 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#348901:u8
        let s_6_0: bool = fn_state.ga_348901;
        // D s_6_1: call Bit(s_6_0)
        let s_6_1: bool = Bit(state, tracer, s_6_0);
        // C s_6_2: const #16968u : u32
        let s_6_2: u32 = 16968;
        // D s_6_3: read-reg s_6_2:struct
        let s_6_3: ProductTypec98939056e929b9c = {
            let value = state
                .read_register::<ProductTypec98939056e929b9c>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // C s_6_4: const #16968u : u32
        let s_6_4: u32 = 16968;
        // N s_6_5: write-reg s_6_4 <= s_6_3
        let s_6_5: () = {
            state.write_register::<ProductTypec98939056e929b9c>(s_6_4 as isize, s_6_3);
            tracer.write_register(s_6_4 as isize, s_6_3);
        };
        // C s_6_6: const #256u : u12
        let s_6_6: u16 = 256;
        // C s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 12u16);
        // C s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (s_6_7.value() as i128);
        // C s_6_9: cast reint s_6_8 -> i64
        let s_6_9: i64 = (s_6_8 as i64);
        // D s_6_10: read-var sum4:i64
        let s_6_10: i64 = fn_state.sum4;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // C s_6_12: cast zx s_6_9 -> i
        let s_6_12: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_13: cmp-ge s_6_11 s_6_12
        let s_6_13: bool = ((s_6_11) >= (s_6_12));
        // N s_6_14: branch s_6_13 b9 b7
        if s_6_13 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var ga#348907 <= s_7_0
        fn_state.ga_348907 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#348907:u8
        let s_8_0: bool = fn_state.ga_348907;
        // D s_8_1: call Bit(s_8_0)
        let s_8_1: bool = Bit(state, tracer, s_8_0);
        // C s_8_2: const #16968u : u32
        let s_8_2: u32 = 16968;
        // D s_8_3: read-reg s_8_2:struct
        let s_8_3: ProductTypec98939056e929b9c = {
            let value = state
                .read_register::<ProductTypec98939056e929b9c>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // C s_8_4: const #16968u : u32
        let s_8_4: u32 = 16968;
        // N s_8_5: write-reg s_8_4 <= s_8_3
        let s_8_5: () = {
            state.write_register::<ProductTypec98939056e929b9c>(s_8_4 as isize, s_8_3);
            tracer.write_register(s_8_4 as isize, s_8_3);
        };
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var ga#348907 <= s_9_0
        fn_state.ga_348907 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var ga#348901 <= s_10_0
        fn_state.ga_348901 = s_10_0;
        // N s_10_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var ga#348895 <= s_11_0
        fn_state.ga_348895 = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var ga#348889 <= s_12_0
        fn_state.ga_348889 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
