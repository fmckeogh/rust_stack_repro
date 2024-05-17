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
use R_set::*;
use integer_subrange::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_USAD8_Op_A_txt<T: Tracer>(
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
        // D s_0_30: sub s_0_28 s_0_29
        let s_0_30: i128 = ((s_0_28) - (s_0_29));
        // D s_0_31: cast reint s_0_30 -> i64
        let s_0_31: i64 = (s_0_30 as i64);
        // D s_0_32: cast zx s_0_31 -> i
        let s_0_32: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_33: abs s_0_32
        let s_0_33: i128 = (s_0_32).abs();
        // D s_0_34: cast reint s_0_33 -> i64
        let s_0_34: i64 = (s_0_33 as i64);
        // D s_0_35: read-var n:i64
        let s_0_35: i64 = fn_state.n;
        // D s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // D s_0_37: call R_read(s_0_36)
        let s_0_37: u32 = R_read(state, tracer, s_0_36);
        // C s_0_38: const #8s : i
        let s_0_38: i128 = 8;
        // D s_0_39: cast zx s_0_37 -> bv
        let s_0_39: Bits = Bits::new(s_0_37 as u128, 32u16);
        // C s_0_40: const #1s : i64
        let s_0_40: i64 = 1;
        // C s_0_41: cast zx s_0_40 -> i
        let s_0_41: i128 = (i128::try_from(s_0_40).unwrap());
        // C s_0_42: const #7s : i
        let s_0_42: i128 = 7;
        // C s_0_43: add s_0_42 s_0_41
        let s_0_43: i128 = (s_0_42 + s_0_41);
        // D s_0_44: bit-extract s_0_39 s_0_38 s_0_43
        let s_0_44: Bits = (Bits::new(
            ((s_0_39) >> (s_0_38)).value(),
            u16::try_from(s_0_43).unwrap(),
        ));
        // D s_0_45: cast reint s_0_44 -> u8
        let s_0_45: u8 = (s_0_44.value() as u8);
        // D s_0_46: cast zx s_0_45 -> bv
        let s_0_46: Bits = Bits::new(s_0_45 as u128, 8u16);
        // D s_0_47: cast zx s_0_46 -> i
        let s_0_47: i128 = (s_0_46.value() as i128);
        // D s_0_48: cast reint s_0_47 -> i64
        let s_0_48: i64 = (s_0_47 as i64);
        // D s_0_49: read-var m:i64
        let s_0_49: i64 = fn_state.m;
        // D s_0_50: cast zx s_0_49 -> i
        let s_0_50: i128 = (i128::try_from(s_0_49).unwrap());
        // D s_0_51: call R_read(s_0_50)
        let s_0_51: u32 = R_read(state, tracer, s_0_50);
        // C s_0_52: const #8s : i
        let s_0_52: i128 = 8;
        // D s_0_53: cast zx s_0_51 -> bv
        let s_0_53: Bits = Bits::new(s_0_51 as u128, 32u16);
        // C s_0_54: const #1s : i64
        let s_0_54: i64 = 1;
        // C s_0_55: cast zx s_0_54 -> i
        let s_0_55: i128 = (i128::try_from(s_0_54).unwrap());
        // C s_0_56: const #7s : i
        let s_0_56: i128 = 7;
        // C s_0_57: add s_0_56 s_0_55
        let s_0_57: i128 = (s_0_56 + s_0_55);
        // D s_0_58: bit-extract s_0_53 s_0_52 s_0_57
        let s_0_58: Bits = (Bits::new(
            ((s_0_53) >> (s_0_52)).value(),
            u16::try_from(s_0_57).unwrap(),
        ));
        // D s_0_59: cast reint s_0_58 -> u8
        let s_0_59: u8 = (s_0_58.value() as u8);
        // D s_0_60: cast zx s_0_59 -> bv
        let s_0_60: Bits = Bits::new(s_0_59 as u128, 8u16);
        // D s_0_61: cast zx s_0_60 -> i
        let s_0_61: i128 = (s_0_60.value() as i128);
        // D s_0_62: cast reint s_0_61 -> i64
        let s_0_62: i64 = (s_0_61 as i64);
        // D s_0_63: cast zx s_0_48 -> i
        let s_0_63: i128 = (i128::try_from(s_0_48).unwrap());
        // D s_0_64: cast zx s_0_62 -> i
        let s_0_64: i128 = (i128::try_from(s_0_62).unwrap());
        // D s_0_65: sub s_0_63 s_0_64
        let s_0_65: i128 = ((s_0_63) - (s_0_64));
        // D s_0_66: cast reint s_0_65 -> i64
        let s_0_66: i64 = (s_0_65 as i64);
        // D s_0_67: cast zx s_0_66 -> i
        let s_0_67: i128 = (i128::try_from(s_0_66).unwrap());
        // D s_0_68: abs s_0_67
        let s_0_68: i128 = (s_0_67).abs();
        // D s_0_69: cast reint s_0_68 -> i64
        let s_0_69: i64 = (s_0_68 as i64);
        // D s_0_70: read-var n:i64
        let s_0_70: i64 = fn_state.n;
        // D s_0_71: cast zx s_0_70 -> i
        let s_0_71: i128 = (i128::try_from(s_0_70).unwrap());
        // D s_0_72: call R_read(s_0_71)
        let s_0_72: u32 = R_read(state, tracer, s_0_71);
        // C s_0_73: const #16s : i
        let s_0_73: i128 = 16;
        // D s_0_74: cast zx s_0_72 -> bv
        let s_0_74: Bits = Bits::new(s_0_72 as u128, 32u16);
        // C s_0_75: const #1s : i64
        let s_0_75: i64 = 1;
        // C s_0_76: cast zx s_0_75 -> i
        let s_0_76: i128 = (i128::try_from(s_0_75).unwrap());
        // C s_0_77: const #7s : i
        let s_0_77: i128 = 7;
        // C s_0_78: add s_0_77 s_0_76
        let s_0_78: i128 = (s_0_77 + s_0_76);
        // D s_0_79: bit-extract s_0_74 s_0_73 s_0_78
        let s_0_79: Bits = (Bits::new(
            ((s_0_74) >> (s_0_73)).value(),
            u16::try_from(s_0_78).unwrap(),
        ));
        // D s_0_80: cast reint s_0_79 -> u8
        let s_0_80: u8 = (s_0_79.value() as u8);
        // D s_0_81: cast zx s_0_80 -> bv
        let s_0_81: Bits = Bits::new(s_0_80 as u128, 8u16);
        // D s_0_82: cast zx s_0_81 -> i
        let s_0_82: i128 = (s_0_81.value() as i128);
        // D s_0_83: cast reint s_0_82 -> i64
        let s_0_83: i64 = (s_0_82 as i64);
        // D s_0_84: read-var m:i64
        let s_0_84: i64 = fn_state.m;
        // D s_0_85: cast zx s_0_84 -> i
        let s_0_85: i128 = (i128::try_from(s_0_84).unwrap());
        // D s_0_86: call R_read(s_0_85)
        let s_0_86: u32 = R_read(state, tracer, s_0_85);
        // C s_0_87: const #16s : i
        let s_0_87: i128 = 16;
        // D s_0_88: cast zx s_0_86 -> bv
        let s_0_88: Bits = Bits::new(s_0_86 as u128, 32u16);
        // C s_0_89: const #1s : i64
        let s_0_89: i64 = 1;
        // C s_0_90: cast zx s_0_89 -> i
        let s_0_90: i128 = (i128::try_from(s_0_89).unwrap());
        // C s_0_91: const #7s : i
        let s_0_91: i128 = 7;
        // C s_0_92: add s_0_91 s_0_90
        let s_0_92: i128 = (s_0_91 + s_0_90);
        // D s_0_93: bit-extract s_0_88 s_0_87 s_0_92
        let s_0_93: Bits = (Bits::new(
            ((s_0_88) >> (s_0_87)).value(),
            u16::try_from(s_0_92).unwrap(),
        ));
        // D s_0_94: cast reint s_0_93 -> u8
        let s_0_94: u8 = (s_0_93.value() as u8);
        // D s_0_95: cast zx s_0_94 -> bv
        let s_0_95: Bits = Bits::new(s_0_94 as u128, 8u16);
        // D s_0_96: cast zx s_0_95 -> i
        let s_0_96: i128 = (s_0_95.value() as i128);
        // D s_0_97: cast reint s_0_96 -> i64
        let s_0_97: i64 = (s_0_96 as i64);
        // D s_0_98: cast zx s_0_83 -> i
        let s_0_98: i128 = (i128::try_from(s_0_83).unwrap());
        // D s_0_99: cast zx s_0_97 -> i
        let s_0_99: i128 = (i128::try_from(s_0_97).unwrap());
        // D s_0_100: sub s_0_98 s_0_99
        let s_0_100: i128 = ((s_0_98) - (s_0_99));
        // D s_0_101: cast reint s_0_100 -> i64
        let s_0_101: i64 = (s_0_100 as i64);
        // D s_0_102: cast zx s_0_101 -> i
        let s_0_102: i128 = (i128::try_from(s_0_101).unwrap());
        // D s_0_103: abs s_0_102
        let s_0_103: i128 = (s_0_102).abs();
        // D s_0_104: cast reint s_0_103 -> i64
        let s_0_104: i64 = (s_0_103 as i64);
        // D s_0_105: read-var n:i64
        let s_0_105: i64 = fn_state.n;
        // D s_0_106: cast zx s_0_105 -> i
        let s_0_106: i128 = (i128::try_from(s_0_105).unwrap());
        // D s_0_107: call R_read(s_0_106)
        let s_0_107: u32 = R_read(state, tracer, s_0_106);
        // C s_0_108: const #24s : i
        let s_0_108: i128 = 24;
        // D s_0_109: cast zx s_0_107 -> bv
        let s_0_109: Bits = Bits::new(s_0_107 as u128, 32u16);
        // C s_0_110: const #1s : i64
        let s_0_110: i64 = 1;
        // C s_0_111: cast zx s_0_110 -> i
        let s_0_111: i128 = (i128::try_from(s_0_110).unwrap());
        // C s_0_112: const #7s : i
        let s_0_112: i128 = 7;
        // C s_0_113: add s_0_112 s_0_111
        let s_0_113: i128 = (s_0_112 + s_0_111);
        // D s_0_114: bit-extract s_0_109 s_0_108 s_0_113
        let s_0_114: Bits = (Bits::new(
            ((s_0_109) >> (s_0_108)).value(),
            u16::try_from(s_0_113).unwrap(),
        ));
        // D s_0_115: cast reint s_0_114 -> u8
        let s_0_115: u8 = (s_0_114.value() as u8);
        // D s_0_116: cast zx s_0_115 -> bv
        let s_0_116: Bits = Bits::new(s_0_115 as u128, 8u16);
        // D s_0_117: cast zx s_0_116 -> i
        let s_0_117: i128 = (s_0_116.value() as i128);
        // D s_0_118: cast reint s_0_117 -> i64
        let s_0_118: i64 = (s_0_117 as i64);
        // D s_0_119: read-var m:i64
        let s_0_119: i64 = fn_state.m;
        // D s_0_120: cast zx s_0_119 -> i
        let s_0_120: i128 = (i128::try_from(s_0_119).unwrap());
        // D s_0_121: call R_read(s_0_120)
        let s_0_121: u32 = R_read(state, tracer, s_0_120);
        // C s_0_122: const #24s : i
        let s_0_122: i128 = 24;
        // D s_0_123: cast zx s_0_121 -> bv
        let s_0_123: Bits = Bits::new(s_0_121 as u128, 32u16);
        // C s_0_124: const #1s : i64
        let s_0_124: i64 = 1;
        // C s_0_125: cast zx s_0_124 -> i
        let s_0_125: i128 = (i128::try_from(s_0_124).unwrap());
        // C s_0_126: const #7s : i
        let s_0_126: i128 = 7;
        // C s_0_127: add s_0_126 s_0_125
        let s_0_127: i128 = (s_0_126 + s_0_125);
        // D s_0_128: bit-extract s_0_123 s_0_122 s_0_127
        let s_0_128: Bits = (Bits::new(
            ((s_0_123) >> (s_0_122)).value(),
            u16::try_from(s_0_127).unwrap(),
        ));
        // D s_0_129: cast reint s_0_128 -> u8
        let s_0_129: u8 = (s_0_128.value() as u8);
        // D s_0_130: cast zx s_0_129 -> bv
        let s_0_130: Bits = Bits::new(s_0_129 as u128, 8u16);
        // D s_0_131: cast zx s_0_130 -> i
        let s_0_131: i128 = (s_0_130.value() as i128);
        // D s_0_132: cast reint s_0_131 -> i64
        let s_0_132: i64 = (s_0_131 as i64);
        // D s_0_133: cast zx s_0_118 -> i
        let s_0_133: i128 = (i128::try_from(s_0_118).unwrap());
        // D s_0_134: cast zx s_0_132 -> i
        let s_0_134: i128 = (i128::try_from(s_0_132).unwrap());
        // D s_0_135: sub s_0_133 s_0_134
        let s_0_135: i128 = ((s_0_133) - (s_0_134));
        // D s_0_136: cast reint s_0_135 -> i64
        let s_0_136: i64 = (s_0_135 as i64);
        // D s_0_137: cast zx s_0_136 -> i
        let s_0_137: i128 = (i128::try_from(s_0_136).unwrap());
        // D s_0_138: abs s_0_137
        let s_0_138: i128 = (s_0_137).abs();
        // D s_0_139: cast reint s_0_138 -> i64
        let s_0_139: i64 = (s_0_138 as i64);
        // D s_0_140: cast zx s_0_34 -> i
        let s_0_140: i128 = (i128::try_from(s_0_34).unwrap());
        // D s_0_141: cast zx s_0_69 -> i
        let s_0_141: i128 = (i128::try_from(s_0_69).unwrap());
        // D s_0_142: add s_0_140 s_0_141
        let s_0_142: i128 = (s_0_140 + s_0_141);
        // D s_0_143: cast reint s_0_142 -> i64
        let s_0_143: i64 = (s_0_142 as i64);
        // D s_0_144: cast zx s_0_143 -> i
        let s_0_144: i128 = (i128::try_from(s_0_143).unwrap());
        // D s_0_145: cast zx s_0_104 -> i
        let s_0_145: i128 = (i128::try_from(s_0_104).unwrap());
        // D s_0_146: add s_0_144 s_0_145
        let s_0_146: i128 = (s_0_144 + s_0_145);
        // D s_0_147: cast reint s_0_146 -> i64
        let s_0_147: i64 = (s_0_146 as i64);
        // D s_0_148: cast zx s_0_147 -> i
        let s_0_148: i128 = (i128::try_from(s_0_147).unwrap());
        // D s_0_149: cast zx s_0_139 -> i
        let s_0_149: i128 = (i128::try_from(s_0_139).unwrap());
        // D s_0_150: add s_0_148 s_0_149
        let s_0_150: i128 = (s_0_148 + s_0_149);
        // D s_0_151: cast reint s_0_150 -> i64
        let s_0_151: i64 = (s_0_150 as i64);
        // C s_0_152: const #31s : i
        let s_0_152: i128 = 31;
        // C s_0_153: const #0s : i
        let s_0_153: i128 = 0;
        // D s_0_154: cast zx s_0_151 -> i
        let s_0_154: i128 = (i128::try_from(s_0_151).unwrap());
        // D s_0_155: call integer_subrange(s_0_154, s_0_152, s_0_153)
        let s_0_155: Bits = integer_subrange(state, tracer, s_0_154, s_0_152, s_0_153);
        // D s_0_156: cast reint s_0_155 -> u32
        let s_0_156: u32 = (s_0_155.value() as u32);
        // D s_0_157: read-var d:i64
        let s_0_157: i64 = fn_state.d;
        // D s_0_158: cast zx s_0_157 -> i
        let s_0_158: i128 = (i128::try_from(s_0_157).unwrap());
        // D s_0_159: call R_set(s_0_158, s_0_156)
        let s_0_159: () = R_set(state, tracer, s_0_158, s_0_156);
        // N s_0_160: return
        return;
    }
}
