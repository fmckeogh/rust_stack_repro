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
use TweakCellRot::*;
use common::*;
pub fn TweakShuffle<T: Tracer>(state: &mut State, tracer: &T, indata: u64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        outdata: u64,
        indata: u64,
    }
    let fn_state = FunctionState {
        indata,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #16s : i
        let s_0_0: i128 = 16;
        // D s_0_1: read-var indata:u64
        let s_0_1: u64 = fn_state.indata;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 64u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #3s : i
        let s_0_5: i128 = 3;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // C s_0_9: const #0s : i
        let s_0_9: i128 = 0;
        // D s_0_10: read-var outdata:u64
        let s_0_10: u64 = fn_state.outdata;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 64u16);
        // D s_0_12: cast zx s_0_8 -> bv
        let s_0_12: Bits = Bits::new(s_0_8 as u128, 4u16);
        // C s_0_13: const #3s : i
        let s_0_13: i128 = 3;
        // C s_0_14: const #1u : u64
        let s_0_14: u64 = 1;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 64u16);
        // C s_0_16: lsl s_0_15 s_0_13
        let s_0_16: Bits = s_0_15 << s_0_13;
        // C s_0_17: sub s_0_16 s_0_15
        let s_0_17: Bits = ((s_0_16) - (s_0_15));
        // D s_0_18: and s_0_12 s_0_17
        let s_0_18: Bits = ((s_0_12) & (s_0_17));
        // D s_0_19: lsl s_0_18 s_0_9
        let s_0_19: Bits = s_0_18 << s_0_9;
        // C s_0_20: lsl s_0_17 s_0_9
        let s_0_20: Bits = s_0_17 << s_0_9;
        // C s_0_21: cmpl s_0_20
        let s_0_21: Bits = !s_0_20;
        // D s_0_22: and s_0_11 s_0_21
        let s_0_22: Bits = ((s_0_11) & (s_0_21));
        // D s_0_23: or s_0_22 s_0_19
        let s_0_23: Bits = ((s_0_22) | (s_0_19));
        // D s_0_24: cast reint s_0_23 -> u64
        let s_0_24: u64 = (s_0_23.value() as u64);
        // D s_0_25: write-var outdata <= s_0_24
        fn_state.outdata = s_0_24;
        // C s_0_26: const #20s : i
        let s_0_26: i128 = 20;
        // D s_0_27: read-var indata:u64
        let s_0_27: u64 = fn_state.indata;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 64u16);
        // C s_0_29: const #1s : i64
        let s_0_29: i64 = 1;
        // C s_0_30: cast zx s_0_29 -> i
        let s_0_30: i128 = (i128::try_from(s_0_29).unwrap());
        // C s_0_31: const #3s : i
        let s_0_31: i128 = 3;
        // C s_0_32: add s_0_31 s_0_30
        let s_0_32: i128 = (s_0_31 + s_0_30);
        // D s_0_33: bit-extract s_0_28 s_0_26 s_0_32
        let s_0_33: Bits = (Bits::new(
            ((s_0_28) >> (s_0_26)).value(),
            u16::try_from(s_0_32).unwrap(),
        ));
        // D s_0_34: cast reint s_0_33 -> u8
        let s_0_34: u8 = (s_0_33.value() as u8);
        // C s_0_35: const #4s : i
        let s_0_35: i128 = 4;
        // D s_0_36: cast zx s_0_24 -> bv
        let s_0_36: Bits = Bits::new(s_0_24 as u128, 64u16);
        // D s_0_37: cast zx s_0_34 -> bv
        let s_0_37: Bits = Bits::new(s_0_34 as u128, 4u16);
        // C s_0_38: const #3s : i
        let s_0_38: i128 = 3;
        // C s_0_39: const #1u : u64
        let s_0_39: u64 = 1;
        // C s_0_40: cast zx s_0_39 -> bv
        let s_0_40: Bits = Bits::new(s_0_39 as u128, 64u16);
        // C s_0_41: lsl s_0_40 s_0_38
        let s_0_41: Bits = s_0_40 << s_0_38;
        // C s_0_42: sub s_0_41 s_0_40
        let s_0_42: Bits = ((s_0_41) - (s_0_40));
        // D s_0_43: and s_0_37 s_0_42
        let s_0_43: Bits = ((s_0_37) & (s_0_42));
        // D s_0_44: lsl s_0_43 s_0_35
        let s_0_44: Bits = s_0_43 << s_0_35;
        // C s_0_45: lsl s_0_42 s_0_35
        let s_0_45: Bits = s_0_42 << s_0_35;
        // C s_0_46: cmpl s_0_45
        let s_0_46: Bits = !s_0_45;
        // D s_0_47: and s_0_36 s_0_46
        let s_0_47: Bits = ((s_0_36) & (s_0_46));
        // D s_0_48: or s_0_47 s_0_44
        let s_0_48: Bits = ((s_0_47) | (s_0_44));
        // D s_0_49: cast reint s_0_48 -> u64
        let s_0_49: u64 = (s_0_48.value() as u64);
        // D s_0_50: write-var outdata <= s_0_49
        fn_state.outdata = s_0_49;
        // C s_0_51: const #24s : i
        let s_0_51: i128 = 24;
        // D s_0_52: read-var indata:u64
        let s_0_52: u64 = fn_state.indata;
        // D s_0_53: cast zx s_0_52 -> bv
        let s_0_53: Bits = Bits::new(s_0_52 as u128, 64u16);
        // C s_0_54: const #1s : i64
        let s_0_54: i64 = 1;
        // C s_0_55: cast zx s_0_54 -> i
        let s_0_55: i128 = (i128::try_from(s_0_54).unwrap());
        // C s_0_56: const #3s : i
        let s_0_56: i128 = 3;
        // C s_0_57: add s_0_56 s_0_55
        let s_0_57: i128 = (s_0_56 + s_0_55);
        // D s_0_58: bit-extract s_0_53 s_0_51 s_0_57
        let s_0_58: Bits = (Bits::new(
            ((s_0_53) >> (s_0_51)).value(),
            u16::try_from(s_0_57).unwrap(),
        ));
        // D s_0_59: cast reint s_0_58 -> u8
        let s_0_59: u8 = (s_0_58.value() as u8);
        // D s_0_60: call TweakCellRot(s_0_59)
        let s_0_60: u8 = TweakCellRot(state, tracer, s_0_59);
        // C s_0_61: const #8s : i
        let s_0_61: i128 = 8;
        // D s_0_62: cast zx s_0_49 -> bv
        let s_0_62: Bits = Bits::new(s_0_49 as u128, 64u16);
        // D s_0_63: cast zx s_0_60 -> bv
        let s_0_63: Bits = Bits::new(s_0_60 as u128, 4u16);
        // C s_0_64: const #3s : i
        let s_0_64: i128 = 3;
        // C s_0_65: const #1u : u64
        let s_0_65: u64 = 1;
        // C s_0_66: cast zx s_0_65 -> bv
        let s_0_66: Bits = Bits::new(s_0_65 as u128, 64u16);
        // C s_0_67: lsl s_0_66 s_0_64
        let s_0_67: Bits = s_0_66 << s_0_64;
        // C s_0_68: sub s_0_67 s_0_66
        let s_0_68: Bits = ((s_0_67) - (s_0_66));
        // D s_0_69: and s_0_63 s_0_68
        let s_0_69: Bits = ((s_0_63) & (s_0_68));
        // D s_0_70: lsl s_0_69 s_0_61
        let s_0_70: Bits = s_0_69 << s_0_61;
        // C s_0_71: lsl s_0_68 s_0_61
        let s_0_71: Bits = s_0_68 << s_0_61;
        // C s_0_72: cmpl s_0_71
        let s_0_72: Bits = !s_0_71;
        // D s_0_73: and s_0_62 s_0_72
        let s_0_73: Bits = ((s_0_62) & (s_0_72));
        // D s_0_74: or s_0_73 s_0_70
        let s_0_74: Bits = ((s_0_73) | (s_0_70));
        // D s_0_75: cast reint s_0_74 -> u64
        let s_0_75: u64 = (s_0_74.value() as u64);
        // D s_0_76: write-var outdata <= s_0_75
        fn_state.outdata = s_0_75;
        // C s_0_77: const #28s : i
        let s_0_77: i128 = 28;
        // D s_0_78: read-var indata:u64
        let s_0_78: u64 = fn_state.indata;
        // D s_0_79: cast zx s_0_78 -> bv
        let s_0_79: Bits = Bits::new(s_0_78 as u128, 64u16);
        // C s_0_80: const #1s : i64
        let s_0_80: i64 = 1;
        // C s_0_81: cast zx s_0_80 -> i
        let s_0_81: i128 = (i128::try_from(s_0_80).unwrap());
        // C s_0_82: const #3s : i
        let s_0_82: i128 = 3;
        // C s_0_83: add s_0_82 s_0_81
        let s_0_83: i128 = (s_0_82 + s_0_81);
        // D s_0_84: bit-extract s_0_79 s_0_77 s_0_83
        let s_0_84: Bits = (Bits::new(
            ((s_0_79) >> (s_0_77)).value(),
            u16::try_from(s_0_83).unwrap(),
        ));
        // D s_0_85: cast reint s_0_84 -> u8
        let s_0_85: u8 = (s_0_84.value() as u8);
        // C s_0_86: const #12s : i
        let s_0_86: i128 = 12;
        // D s_0_87: cast zx s_0_75 -> bv
        let s_0_87: Bits = Bits::new(s_0_75 as u128, 64u16);
        // D s_0_88: cast zx s_0_85 -> bv
        let s_0_88: Bits = Bits::new(s_0_85 as u128, 4u16);
        // C s_0_89: const #3s : i
        let s_0_89: i128 = 3;
        // C s_0_90: const #1u : u64
        let s_0_90: u64 = 1;
        // C s_0_91: cast zx s_0_90 -> bv
        let s_0_91: Bits = Bits::new(s_0_90 as u128, 64u16);
        // C s_0_92: lsl s_0_91 s_0_89
        let s_0_92: Bits = s_0_91 << s_0_89;
        // C s_0_93: sub s_0_92 s_0_91
        let s_0_93: Bits = ((s_0_92) - (s_0_91));
        // D s_0_94: and s_0_88 s_0_93
        let s_0_94: Bits = ((s_0_88) & (s_0_93));
        // D s_0_95: lsl s_0_94 s_0_86
        let s_0_95: Bits = s_0_94 << s_0_86;
        // C s_0_96: lsl s_0_93 s_0_86
        let s_0_96: Bits = s_0_93 << s_0_86;
        // C s_0_97: cmpl s_0_96
        let s_0_97: Bits = !s_0_96;
        // D s_0_98: and s_0_87 s_0_97
        let s_0_98: Bits = ((s_0_87) & (s_0_97));
        // D s_0_99: or s_0_98 s_0_95
        let s_0_99: Bits = ((s_0_98) | (s_0_95));
        // D s_0_100: cast reint s_0_99 -> u64
        let s_0_100: u64 = (s_0_99.value() as u64);
        // D s_0_101: write-var outdata <= s_0_100
        fn_state.outdata = s_0_100;
        // C s_0_102: const #44s : i
        let s_0_102: i128 = 44;
        // D s_0_103: read-var indata:u64
        let s_0_103: u64 = fn_state.indata;
        // D s_0_104: cast zx s_0_103 -> bv
        let s_0_104: Bits = Bits::new(s_0_103 as u128, 64u16);
        // C s_0_105: const #1s : i64
        let s_0_105: i64 = 1;
        // C s_0_106: cast zx s_0_105 -> i
        let s_0_106: i128 = (i128::try_from(s_0_105).unwrap());
        // C s_0_107: const #3s : i
        let s_0_107: i128 = 3;
        // C s_0_108: add s_0_107 s_0_106
        let s_0_108: i128 = (s_0_107 + s_0_106);
        // D s_0_109: bit-extract s_0_104 s_0_102 s_0_108
        let s_0_109: Bits = (Bits::new(
            ((s_0_104) >> (s_0_102)).value(),
            u16::try_from(s_0_108).unwrap(),
        ));
        // D s_0_110: cast reint s_0_109 -> u8
        let s_0_110: u8 = (s_0_109.value() as u8);
        // D s_0_111: call TweakCellRot(s_0_110)
        let s_0_111: u8 = TweakCellRot(state, tracer, s_0_110);
        // C s_0_112: const #16s : i
        let s_0_112: i128 = 16;
        // D s_0_113: cast zx s_0_100 -> bv
        let s_0_113: Bits = Bits::new(s_0_100 as u128, 64u16);
        // D s_0_114: cast zx s_0_111 -> bv
        let s_0_114: Bits = Bits::new(s_0_111 as u128, 4u16);
        // C s_0_115: const #3s : i
        let s_0_115: i128 = 3;
        // C s_0_116: const #1u : u64
        let s_0_116: u64 = 1;
        // C s_0_117: cast zx s_0_116 -> bv
        let s_0_117: Bits = Bits::new(s_0_116 as u128, 64u16);
        // C s_0_118: lsl s_0_117 s_0_115
        let s_0_118: Bits = s_0_117 << s_0_115;
        // C s_0_119: sub s_0_118 s_0_117
        let s_0_119: Bits = ((s_0_118) - (s_0_117));
        // D s_0_120: and s_0_114 s_0_119
        let s_0_120: Bits = ((s_0_114) & (s_0_119));
        // D s_0_121: lsl s_0_120 s_0_112
        let s_0_121: Bits = s_0_120 << s_0_112;
        // C s_0_122: lsl s_0_119 s_0_112
        let s_0_122: Bits = s_0_119 << s_0_112;
        // C s_0_123: cmpl s_0_122
        let s_0_123: Bits = !s_0_122;
        // D s_0_124: and s_0_113 s_0_123
        let s_0_124: Bits = ((s_0_113) & (s_0_123));
        // D s_0_125: or s_0_124 s_0_121
        let s_0_125: Bits = ((s_0_124) | (s_0_121));
        // D s_0_126: cast reint s_0_125 -> u64
        let s_0_126: u64 = (s_0_125.value() as u64);
        // D s_0_127: write-var outdata <= s_0_126
        fn_state.outdata = s_0_126;
        // C s_0_128: const #8s : i
        let s_0_128: i128 = 8;
        // D s_0_129: read-var indata:u64
        let s_0_129: u64 = fn_state.indata;
        // D s_0_130: cast zx s_0_129 -> bv
        let s_0_130: Bits = Bits::new(s_0_129 as u128, 64u16);
        // C s_0_131: const #1s : i64
        let s_0_131: i64 = 1;
        // C s_0_132: cast zx s_0_131 -> i
        let s_0_132: i128 = (i128::try_from(s_0_131).unwrap());
        // C s_0_133: const #3s : i
        let s_0_133: i128 = 3;
        // C s_0_134: add s_0_133 s_0_132
        let s_0_134: i128 = (s_0_133 + s_0_132);
        // D s_0_135: bit-extract s_0_130 s_0_128 s_0_134
        let s_0_135: Bits = (Bits::new(
            ((s_0_130) >> (s_0_128)).value(),
            u16::try_from(s_0_134).unwrap(),
        ));
        // D s_0_136: cast reint s_0_135 -> u8
        let s_0_136: u8 = (s_0_135.value() as u8);
        // C s_0_137: const #20s : i
        let s_0_137: i128 = 20;
        // D s_0_138: cast zx s_0_126 -> bv
        let s_0_138: Bits = Bits::new(s_0_126 as u128, 64u16);
        // D s_0_139: cast zx s_0_136 -> bv
        let s_0_139: Bits = Bits::new(s_0_136 as u128, 4u16);
        // C s_0_140: const #3s : i
        let s_0_140: i128 = 3;
        // C s_0_141: const #1u : u64
        let s_0_141: u64 = 1;
        // C s_0_142: cast zx s_0_141 -> bv
        let s_0_142: Bits = Bits::new(s_0_141 as u128, 64u16);
        // C s_0_143: lsl s_0_142 s_0_140
        let s_0_143: Bits = s_0_142 << s_0_140;
        // C s_0_144: sub s_0_143 s_0_142
        let s_0_144: Bits = ((s_0_143) - (s_0_142));
        // D s_0_145: and s_0_139 s_0_144
        let s_0_145: Bits = ((s_0_139) & (s_0_144));
        // D s_0_146: lsl s_0_145 s_0_137
        let s_0_146: Bits = s_0_145 << s_0_137;
        // C s_0_147: lsl s_0_144 s_0_137
        let s_0_147: Bits = s_0_144 << s_0_137;
        // C s_0_148: cmpl s_0_147
        let s_0_148: Bits = !s_0_147;
        // D s_0_149: and s_0_138 s_0_148
        let s_0_149: Bits = ((s_0_138) & (s_0_148));
        // D s_0_150: or s_0_149 s_0_146
        let s_0_150: Bits = ((s_0_149) | (s_0_146));
        // D s_0_151: cast reint s_0_150 -> u64
        let s_0_151: u64 = (s_0_150.value() as u64);
        // D s_0_152: write-var outdata <= s_0_151
        fn_state.outdata = s_0_151;
        // C s_0_153: const #12s : i
        let s_0_153: i128 = 12;
        // D s_0_154: read-var indata:u64
        let s_0_154: u64 = fn_state.indata;
        // D s_0_155: cast zx s_0_154 -> bv
        let s_0_155: Bits = Bits::new(s_0_154 as u128, 64u16);
        // C s_0_156: const #1s : i64
        let s_0_156: i64 = 1;
        // C s_0_157: cast zx s_0_156 -> i
        let s_0_157: i128 = (i128::try_from(s_0_156).unwrap());
        // C s_0_158: const #3s : i
        let s_0_158: i128 = 3;
        // C s_0_159: add s_0_158 s_0_157
        let s_0_159: i128 = (s_0_158 + s_0_157);
        // D s_0_160: bit-extract s_0_155 s_0_153 s_0_159
        let s_0_160: Bits = (Bits::new(
            ((s_0_155) >> (s_0_153)).value(),
            u16::try_from(s_0_159).unwrap(),
        ));
        // D s_0_161: cast reint s_0_160 -> u8
        let s_0_161: u8 = (s_0_160.value() as u8);
        // C s_0_162: const #24s : i
        let s_0_162: i128 = 24;
        // D s_0_163: cast zx s_0_151 -> bv
        let s_0_163: Bits = Bits::new(s_0_151 as u128, 64u16);
        // D s_0_164: cast zx s_0_161 -> bv
        let s_0_164: Bits = Bits::new(s_0_161 as u128, 4u16);
        // C s_0_165: const #3s : i
        let s_0_165: i128 = 3;
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
        // D s_0_176: cast reint s_0_175 -> u64
        let s_0_176: u64 = (s_0_175.value() as u64);
        // D s_0_177: write-var outdata <= s_0_176
        fn_state.outdata = s_0_176;
        // C s_0_178: const #32s : i
        let s_0_178: i128 = 32;
        // D s_0_179: read-var indata:u64
        let s_0_179: u64 = fn_state.indata;
        // D s_0_180: cast zx s_0_179 -> bv
        let s_0_180: Bits = Bits::new(s_0_179 as u128, 64u16);
        // C s_0_181: const #1s : i64
        let s_0_181: i64 = 1;
        // C s_0_182: cast zx s_0_181 -> i
        let s_0_182: i128 = (i128::try_from(s_0_181).unwrap());
        // C s_0_183: const #3s : i
        let s_0_183: i128 = 3;
        // C s_0_184: add s_0_183 s_0_182
        let s_0_184: i128 = (s_0_183 + s_0_182);
        // D s_0_185: bit-extract s_0_180 s_0_178 s_0_184
        let s_0_185: Bits = (Bits::new(
            ((s_0_180) >> (s_0_178)).value(),
            u16::try_from(s_0_184).unwrap(),
        ));
        // D s_0_186: cast reint s_0_185 -> u8
        let s_0_186: u8 = (s_0_185.value() as u8);
        // D s_0_187: call TweakCellRot(s_0_186)
        let s_0_187: u8 = TweakCellRot(state, tracer, s_0_186);
        // C s_0_188: const #28s : i
        let s_0_188: i128 = 28;
        // D s_0_189: cast zx s_0_176 -> bv
        let s_0_189: Bits = Bits::new(s_0_176 as u128, 64u16);
        // D s_0_190: cast zx s_0_187 -> bv
        let s_0_190: Bits = Bits::new(s_0_187 as u128, 4u16);
        // C s_0_191: const #3s : i
        let s_0_191: i128 = 3;
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
        // D s_0_202: cast reint s_0_201 -> u64
        let s_0_202: u64 = (s_0_201.value() as u64);
        // D s_0_203: write-var outdata <= s_0_202
        fn_state.outdata = s_0_202;
        // C s_0_204: const #48s : i
        let s_0_204: i128 = 48;
        // D s_0_205: read-var indata:u64
        let s_0_205: u64 = fn_state.indata;
        // D s_0_206: cast zx s_0_205 -> bv
        let s_0_206: Bits = Bits::new(s_0_205 as u128, 64u16);
        // C s_0_207: const #1s : i64
        let s_0_207: i64 = 1;
        // C s_0_208: cast zx s_0_207 -> i
        let s_0_208: i128 = (i128::try_from(s_0_207).unwrap());
        // C s_0_209: const #3s : i
        let s_0_209: i128 = 3;
        // C s_0_210: add s_0_209 s_0_208
        let s_0_210: i128 = (s_0_209 + s_0_208);
        // D s_0_211: bit-extract s_0_206 s_0_204 s_0_210
        let s_0_211: Bits = (Bits::new(
            ((s_0_206) >> (s_0_204)).value(),
            u16::try_from(s_0_210).unwrap(),
        ));
        // D s_0_212: cast reint s_0_211 -> u8
        let s_0_212: u8 = (s_0_211.value() as u8);
        // C s_0_213: const #32s : i
        let s_0_213: i128 = 32;
        // D s_0_214: cast zx s_0_202 -> bv
        let s_0_214: Bits = Bits::new(s_0_202 as u128, 64u16);
        // D s_0_215: cast zx s_0_212 -> bv
        let s_0_215: Bits = Bits::new(s_0_212 as u128, 4u16);
        // C s_0_216: const #3s : i
        let s_0_216: i128 = 3;
        // C s_0_217: const #1u : u64
        let s_0_217: u64 = 1;
        // C s_0_218: cast zx s_0_217 -> bv
        let s_0_218: Bits = Bits::new(s_0_217 as u128, 64u16);
        // C s_0_219: lsl s_0_218 s_0_216
        let s_0_219: Bits = s_0_218 << s_0_216;
        // C s_0_220: sub s_0_219 s_0_218
        let s_0_220: Bits = ((s_0_219) - (s_0_218));
        // D s_0_221: and s_0_215 s_0_220
        let s_0_221: Bits = ((s_0_215) & (s_0_220));
        // D s_0_222: lsl s_0_221 s_0_213
        let s_0_222: Bits = s_0_221 << s_0_213;
        // C s_0_223: lsl s_0_220 s_0_213
        let s_0_223: Bits = s_0_220 << s_0_213;
        // C s_0_224: cmpl s_0_223
        let s_0_224: Bits = !s_0_223;
        // D s_0_225: and s_0_214 s_0_224
        let s_0_225: Bits = ((s_0_214) & (s_0_224));
        // D s_0_226: or s_0_225 s_0_222
        let s_0_226: Bits = ((s_0_225) | (s_0_222));
        // D s_0_227: cast reint s_0_226 -> u64
        let s_0_227: u64 = (s_0_226.value() as u64);
        // D s_0_228: write-var outdata <= s_0_227
        fn_state.outdata = s_0_227;
        // C s_0_229: const #52s : i
        let s_0_229: i128 = 52;
        // D s_0_230: read-var indata:u64
        let s_0_230: u64 = fn_state.indata;
        // D s_0_231: cast zx s_0_230 -> bv
        let s_0_231: Bits = Bits::new(s_0_230 as u128, 64u16);
        // C s_0_232: const #1s : i64
        let s_0_232: i64 = 1;
        // C s_0_233: cast zx s_0_232 -> i
        let s_0_233: i128 = (i128::try_from(s_0_232).unwrap());
        // C s_0_234: const #3s : i
        let s_0_234: i128 = 3;
        // C s_0_235: add s_0_234 s_0_233
        let s_0_235: i128 = (s_0_234 + s_0_233);
        // D s_0_236: bit-extract s_0_231 s_0_229 s_0_235
        let s_0_236: Bits = (Bits::new(
            ((s_0_231) >> (s_0_229)).value(),
            u16::try_from(s_0_235).unwrap(),
        ));
        // D s_0_237: cast reint s_0_236 -> u8
        let s_0_237: u8 = (s_0_236.value() as u8);
        // C s_0_238: const #36s : i
        let s_0_238: i128 = 36;
        // D s_0_239: cast zx s_0_227 -> bv
        let s_0_239: Bits = Bits::new(s_0_227 as u128, 64u16);
        // D s_0_240: cast zx s_0_237 -> bv
        let s_0_240: Bits = Bits::new(s_0_237 as u128, 4u16);
        // C s_0_241: const #3s : i
        let s_0_241: i128 = 3;
        // C s_0_242: const #1u : u64
        let s_0_242: u64 = 1;
        // C s_0_243: cast zx s_0_242 -> bv
        let s_0_243: Bits = Bits::new(s_0_242 as u128, 64u16);
        // C s_0_244: lsl s_0_243 s_0_241
        let s_0_244: Bits = s_0_243 << s_0_241;
        // C s_0_245: sub s_0_244 s_0_243
        let s_0_245: Bits = ((s_0_244) - (s_0_243));
        // D s_0_246: and s_0_240 s_0_245
        let s_0_246: Bits = ((s_0_240) & (s_0_245));
        // D s_0_247: lsl s_0_246 s_0_238
        let s_0_247: Bits = s_0_246 << s_0_238;
        // C s_0_248: lsl s_0_245 s_0_238
        let s_0_248: Bits = s_0_245 << s_0_238;
        // C s_0_249: cmpl s_0_248
        let s_0_249: Bits = !s_0_248;
        // D s_0_250: and s_0_239 s_0_249
        let s_0_250: Bits = ((s_0_239) & (s_0_249));
        // D s_0_251: or s_0_250 s_0_247
        let s_0_251: Bits = ((s_0_250) | (s_0_247));
        // D s_0_252: cast reint s_0_251 -> u64
        let s_0_252: u64 = (s_0_251.value() as u64);
        // D s_0_253: write-var outdata <= s_0_252
        fn_state.outdata = s_0_252;
        // C s_0_254: const #56s : i
        let s_0_254: i128 = 56;
        // D s_0_255: read-var indata:u64
        let s_0_255: u64 = fn_state.indata;
        // D s_0_256: cast zx s_0_255 -> bv
        let s_0_256: Bits = Bits::new(s_0_255 as u128, 64u16);
        // C s_0_257: const #1s : i64
        let s_0_257: i64 = 1;
        // C s_0_258: cast zx s_0_257 -> i
        let s_0_258: i128 = (i128::try_from(s_0_257).unwrap());
        // C s_0_259: const #3s : i
        let s_0_259: i128 = 3;
        // C s_0_260: add s_0_259 s_0_258
        let s_0_260: i128 = (s_0_259 + s_0_258);
        // D s_0_261: bit-extract s_0_256 s_0_254 s_0_260
        let s_0_261: Bits = (Bits::new(
            ((s_0_256) >> (s_0_254)).value(),
            u16::try_from(s_0_260).unwrap(),
        ));
        // D s_0_262: cast reint s_0_261 -> u8
        let s_0_262: u8 = (s_0_261.value() as u8);
        // C s_0_263: const #40s : i
        let s_0_263: i128 = 40;
        // D s_0_264: cast zx s_0_252 -> bv
        let s_0_264: Bits = Bits::new(s_0_252 as u128, 64u16);
        // D s_0_265: cast zx s_0_262 -> bv
        let s_0_265: Bits = Bits::new(s_0_262 as u128, 4u16);
        // C s_0_266: const #3s : i
        let s_0_266: i128 = 3;
        // C s_0_267: const #1u : u64
        let s_0_267: u64 = 1;
        // C s_0_268: cast zx s_0_267 -> bv
        let s_0_268: Bits = Bits::new(s_0_267 as u128, 64u16);
        // C s_0_269: lsl s_0_268 s_0_266
        let s_0_269: Bits = s_0_268 << s_0_266;
        // C s_0_270: sub s_0_269 s_0_268
        let s_0_270: Bits = ((s_0_269) - (s_0_268));
        // D s_0_271: and s_0_265 s_0_270
        let s_0_271: Bits = ((s_0_265) & (s_0_270));
        // D s_0_272: lsl s_0_271 s_0_263
        let s_0_272: Bits = s_0_271 << s_0_263;
        // C s_0_273: lsl s_0_270 s_0_263
        let s_0_273: Bits = s_0_270 << s_0_263;
        // C s_0_274: cmpl s_0_273
        let s_0_274: Bits = !s_0_273;
        // D s_0_275: and s_0_264 s_0_274
        let s_0_275: Bits = ((s_0_264) & (s_0_274));
        // D s_0_276: or s_0_275 s_0_272
        let s_0_276: Bits = ((s_0_275) | (s_0_272));
        // D s_0_277: cast reint s_0_276 -> u64
        let s_0_277: u64 = (s_0_276.value() as u64);
        // D s_0_278: write-var outdata <= s_0_277
        fn_state.outdata = s_0_277;
        // C s_0_279: const #60s : i
        let s_0_279: i128 = 60;
        // D s_0_280: read-var indata:u64
        let s_0_280: u64 = fn_state.indata;
        // D s_0_281: cast zx s_0_280 -> bv
        let s_0_281: Bits = Bits::new(s_0_280 as u128, 64u16);
        // C s_0_282: const #1s : i64
        let s_0_282: i64 = 1;
        // C s_0_283: cast zx s_0_282 -> i
        let s_0_283: i128 = (i128::try_from(s_0_282).unwrap());
        // C s_0_284: const #3s : i
        let s_0_284: i128 = 3;
        // C s_0_285: add s_0_284 s_0_283
        let s_0_285: i128 = (s_0_284 + s_0_283);
        // D s_0_286: bit-extract s_0_281 s_0_279 s_0_285
        let s_0_286: Bits = (Bits::new(
            ((s_0_281) >> (s_0_279)).value(),
            u16::try_from(s_0_285).unwrap(),
        ));
        // D s_0_287: cast reint s_0_286 -> u8
        let s_0_287: u8 = (s_0_286.value() as u8);
        // D s_0_288: call TweakCellRot(s_0_287)
        let s_0_288: u8 = TweakCellRot(state, tracer, s_0_287);
        // C s_0_289: const #44s : i
        let s_0_289: i128 = 44;
        // D s_0_290: cast zx s_0_277 -> bv
        let s_0_290: Bits = Bits::new(s_0_277 as u128, 64u16);
        // D s_0_291: cast zx s_0_288 -> bv
        let s_0_291: Bits = Bits::new(s_0_288 as u128, 4u16);
        // C s_0_292: const #3s : i
        let s_0_292: i128 = 3;
        // C s_0_293: const #1u : u64
        let s_0_293: u64 = 1;
        // C s_0_294: cast zx s_0_293 -> bv
        let s_0_294: Bits = Bits::new(s_0_293 as u128, 64u16);
        // C s_0_295: lsl s_0_294 s_0_292
        let s_0_295: Bits = s_0_294 << s_0_292;
        // C s_0_296: sub s_0_295 s_0_294
        let s_0_296: Bits = ((s_0_295) - (s_0_294));
        // D s_0_297: and s_0_291 s_0_296
        let s_0_297: Bits = ((s_0_291) & (s_0_296));
        // D s_0_298: lsl s_0_297 s_0_289
        let s_0_298: Bits = s_0_297 << s_0_289;
        // C s_0_299: lsl s_0_296 s_0_289
        let s_0_299: Bits = s_0_296 << s_0_289;
        // C s_0_300: cmpl s_0_299
        let s_0_300: Bits = !s_0_299;
        // D s_0_301: and s_0_290 s_0_300
        let s_0_301: Bits = ((s_0_290) & (s_0_300));
        // D s_0_302: or s_0_301 s_0_298
        let s_0_302: Bits = ((s_0_301) | (s_0_298));
        // D s_0_303: cast reint s_0_302 -> u64
        let s_0_303: u64 = (s_0_302.value() as u64);
        // D s_0_304: write-var outdata <= s_0_303
        fn_state.outdata = s_0_303;
        // C s_0_305: const #0s : i
        let s_0_305: i128 = 0;
        // D s_0_306: read-var indata:u64
        let s_0_306: u64 = fn_state.indata;
        // D s_0_307: cast zx s_0_306 -> bv
        let s_0_307: Bits = Bits::new(s_0_306 as u128, 64u16);
        // C s_0_308: const #1s : i64
        let s_0_308: i64 = 1;
        // C s_0_309: cast zx s_0_308 -> i
        let s_0_309: i128 = (i128::try_from(s_0_308).unwrap());
        // C s_0_310: const #3s : i
        let s_0_310: i128 = 3;
        // C s_0_311: add s_0_310 s_0_309
        let s_0_311: i128 = (s_0_310 + s_0_309);
        // D s_0_312: bit-extract s_0_307 s_0_305 s_0_311
        let s_0_312: Bits = (Bits::new(
            ((s_0_307) >> (s_0_305)).value(),
            u16::try_from(s_0_311).unwrap(),
        ));
        // D s_0_313: cast reint s_0_312 -> u8
        let s_0_313: u8 = (s_0_312.value() as u8);
        // D s_0_314: call TweakCellRot(s_0_313)
        let s_0_314: u8 = TweakCellRot(state, tracer, s_0_313);
        // C s_0_315: const #48s : i
        let s_0_315: i128 = 48;
        // D s_0_316: cast zx s_0_303 -> bv
        let s_0_316: Bits = Bits::new(s_0_303 as u128, 64u16);
        // D s_0_317: cast zx s_0_314 -> bv
        let s_0_317: Bits = Bits::new(s_0_314 as u128, 4u16);
        // C s_0_318: const #3s : i
        let s_0_318: i128 = 3;
        // C s_0_319: const #1u : u64
        let s_0_319: u64 = 1;
        // C s_0_320: cast zx s_0_319 -> bv
        let s_0_320: Bits = Bits::new(s_0_319 as u128, 64u16);
        // C s_0_321: lsl s_0_320 s_0_318
        let s_0_321: Bits = s_0_320 << s_0_318;
        // C s_0_322: sub s_0_321 s_0_320
        let s_0_322: Bits = ((s_0_321) - (s_0_320));
        // D s_0_323: and s_0_317 s_0_322
        let s_0_323: Bits = ((s_0_317) & (s_0_322));
        // D s_0_324: lsl s_0_323 s_0_315
        let s_0_324: Bits = s_0_323 << s_0_315;
        // C s_0_325: lsl s_0_322 s_0_315
        let s_0_325: Bits = s_0_322 << s_0_315;
        // C s_0_326: cmpl s_0_325
        let s_0_326: Bits = !s_0_325;
        // D s_0_327: and s_0_316 s_0_326
        let s_0_327: Bits = ((s_0_316) & (s_0_326));
        // D s_0_328: or s_0_327 s_0_324
        let s_0_328: Bits = ((s_0_327) | (s_0_324));
        // D s_0_329: cast reint s_0_328 -> u64
        let s_0_329: u64 = (s_0_328.value() as u64);
        // D s_0_330: write-var outdata <= s_0_329
        fn_state.outdata = s_0_329;
        // C s_0_331: const #4s : i
        let s_0_331: i128 = 4;
        // D s_0_332: read-var indata:u64
        let s_0_332: u64 = fn_state.indata;
        // D s_0_333: cast zx s_0_332 -> bv
        let s_0_333: Bits = Bits::new(s_0_332 as u128, 64u16);
        // C s_0_334: const #1s : i64
        let s_0_334: i64 = 1;
        // C s_0_335: cast zx s_0_334 -> i
        let s_0_335: i128 = (i128::try_from(s_0_334).unwrap());
        // C s_0_336: const #3s : i
        let s_0_336: i128 = 3;
        // C s_0_337: add s_0_336 s_0_335
        let s_0_337: i128 = (s_0_336 + s_0_335);
        // D s_0_338: bit-extract s_0_333 s_0_331 s_0_337
        let s_0_338: Bits = (Bits::new(
            ((s_0_333) >> (s_0_331)).value(),
            u16::try_from(s_0_337).unwrap(),
        ));
        // D s_0_339: cast reint s_0_338 -> u8
        let s_0_339: u8 = (s_0_338.value() as u8);
        // C s_0_340: const #52s : i
        let s_0_340: i128 = 52;
        // D s_0_341: cast zx s_0_329 -> bv
        let s_0_341: Bits = Bits::new(s_0_329 as u128, 64u16);
        // D s_0_342: cast zx s_0_339 -> bv
        let s_0_342: Bits = Bits::new(s_0_339 as u128, 4u16);
        // C s_0_343: const #3s : i
        let s_0_343: i128 = 3;
        // C s_0_344: const #1u : u64
        let s_0_344: u64 = 1;
        // C s_0_345: cast zx s_0_344 -> bv
        let s_0_345: Bits = Bits::new(s_0_344 as u128, 64u16);
        // C s_0_346: lsl s_0_345 s_0_343
        let s_0_346: Bits = s_0_345 << s_0_343;
        // C s_0_347: sub s_0_346 s_0_345
        let s_0_347: Bits = ((s_0_346) - (s_0_345));
        // D s_0_348: and s_0_342 s_0_347
        let s_0_348: Bits = ((s_0_342) & (s_0_347));
        // D s_0_349: lsl s_0_348 s_0_340
        let s_0_349: Bits = s_0_348 << s_0_340;
        // C s_0_350: lsl s_0_347 s_0_340
        let s_0_350: Bits = s_0_347 << s_0_340;
        // C s_0_351: cmpl s_0_350
        let s_0_351: Bits = !s_0_350;
        // D s_0_352: and s_0_341 s_0_351
        let s_0_352: Bits = ((s_0_341) & (s_0_351));
        // D s_0_353: or s_0_352 s_0_349
        let s_0_353: Bits = ((s_0_352) | (s_0_349));
        // D s_0_354: cast reint s_0_353 -> u64
        let s_0_354: u64 = (s_0_353.value() as u64);
        // D s_0_355: write-var outdata <= s_0_354
        fn_state.outdata = s_0_354;
        // C s_0_356: const #40s : i
        let s_0_356: i128 = 40;
        // D s_0_357: read-var indata:u64
        let s_0_357: u64 = fn_state.indata;
        // D s_0_358: cast zx s_0_357 -> bv
        let s_0_358: Bits = Bits::new(s_0_357 as u128, 64u16);
        // C s_0_359: const #1s : i64
        let s_0_359: i64 = 1;
        // C s_0_360: cast zx s_0_359 -> i
        let s_0_360: i128 = (i128::try_from(s_0_359).unwrap());
        // C s_0_361: const #3s : i
        let s_0_361: i128 = 3;
        // C s_0_362: add s_0_361 s_0_360
        let s_0_362: i128 = (s_0_361 + s_0_360);
        // D s_0_363: bit-extract s_0_358 s_0_356 s_0_362
        let s_0_363: Bits = (Bits::new(
            ((s_0_358) >> (s_0_356)).value(),
            u16::try_from(s_0_362).unwrap(),
        ));
        // D s_0_364: cast reint s_0_363 -> u8
        let s_0_364: u8 = (s_0_363.value() as u8);
        // D s_0_365: call TweakCellRot(s_0_364)
        let s_0_365: u8 = TweakCellRot(state, tracer, s_0_364);
        // C s_0_366: const #56s : i
        let s_0_366: i128 = 56;
        // D s_0_367: cast zx s_0_354 -> bv
        let s_0_367: Bits = Bits::new(s_0_354 as u128, 64u16);
        // D s_0_368: cast zx s_0_365 -> bv
        let s_0_368: Bits = Bits::new(s_0_365 as u128, 4u16);
        // C s_0_369: const #3s : i
        let s_0_369: i128 = 3;
        // C s_0_370: const #1u : u64
        let s_0_370: u64 = 1;
        // C s_0_371: cast zx s_0_370 -> bv
        let s_0_371: Bits = Bits::new(s_0_370 as u128, 64u16);
        // C s_0_372: lsl s_0_371 s_0_369
        let s_0_372: Bits = s_0_371 << s_0_369;
        // C s_0_373: sub s_0_372 s_0_371
        let s_0_373: Bits = ((s_0_372) - (s_0_371));
        // D s_0_374: and s_0_368 s_0_373
        let s_0_374: Bits = ((s_0_368) & (s_0_373));
        // D s_0_375: lsl s_0_374 s_0_366
        let s_0_375: Bits = s_0_374 << s_0_366;
        // C s_0_376: lsl s_0_373 s_0_366
        let s_0_376: Bits = s_0_373 << s_0_366;
        // C s_0_377: cmpl s_0_376
        let s_0_377: Bits = !s_0_376;
        // D s_0_378: and s_0_367 s_0_377
        let s_0_378: Bits = ((s_0_367) & (s_0_377));
        // D s_0_379: or s_0_378 s_0_375
        let s_0_379: Bits = ((s_0_378) | (s_0_375));
        // D s_0_380: cast reint s_0_379 -> u64
        let s_0_380: u64 = (s_0_379.value() as u64);
        // D s_0_381: write-var outdata <= s_0_380
        fn_state.outdata = s_0_380;
        // C s_0_382: const #36s : i
        let s_0_382: i128 = 36;
        // D s_0_383: read-var indata:u64
        let s_0_383: u64 = fn_state.indata;
        // D s_0_384: cast zx s_0_383 -> bv
        let s_0_384: Bits = Bits::new(s_0_383 as u128, 64u16);
        // C s_0_385: const #1s : i64
        let s_0_385: i64 = 1;
        // C s_0_386: cast zx s_0_385 -> i
        let s_0_386: i128 = (i128::try_from(s_0_385).unwrap());
        // C s_0_387: const #3s : i
        let s_0_387: i128 = 3;
        // C s_0_388: add s_0_387 s_0_386
        let s_0_388: i128 = (s_0_387 + s_0_386);
        // D s_0_389: bit-extract s_0_384 s_0_382 s_0_388
        let s_0_389: Bits = (Bits::new(
            ((s_0_384) >> (s_0_382)).value(),
            u16::try_from(s_0_388).unwrap(),
        ));
        // D s_0_390: cast reint s_0_389 -> u8
        let s_0_390: u8 = (s_0_389.value() as u8);
        // D s_0_391: call TweakCellRot(s_0_390)
        let s_0_391: u8 = TweakCellRot(state, tracer, s_0_390);
        // C s_0_392: const #60s : i
        let s_0_392: i128 = 60;
        // D s_0_393: cast zx s_0_380 -> bv
        let s_0_393: Bits = Bits::new(s_0_380 as u128, 64u16);
        // D s_0_394: cast zx s_0_391 -> bv
        let s_0_394: Bits = Bits::new(s_0_391 as u128, 4u16);
        // C s_0_395: const #3s : i
        let s_0_395: i128 = 3;
        // C s_0_396: const #1u : u64
        let s_0_396: u64 = 1;
        // C s_0_397: cast zx s_0_396 -> bv
        let s_0_397: Bits = Bits::new(s_0_396 as u128, 64u16);
        // C s_0_398: lsl s_0_397 s_0_395
        let s_0_398: Bits = s_0_397 << s_0_395;
        // C s_0_399: sub s_0_398 s_0_397
        let s_0_399: Bits = ((s_0_398) - (s_0_397));
        // D s_0_400: and s_0_394 s_0_399
        let s_0_400: Bits = ((s_0_394) & (s_0_399));
        // D s_0_401: lsl s_0_400 s_0_392
        let s_0_401: Bits = s_0_400 << s_0_392;
        // C s_0_402: lsl s_0_399 s_0_392
        let s_0_402: Bits = s_0_399 << s_0_392;
        // C s_0_403: cmpl s_0_402
        let s_0_403: Bits = !s_0_402;
        // D s_0_404: and s_0_393 s_0_403
        let s_0_404: Bits = ((s_0_393) & (s_0_403));
        // D s_0_405: or s_0_404 s_0_401
        let s_0_405: Bits = ((s_0_404) | (s_0_401));
        // D s_0_406: cast reint s_0_405 -> u64
        let s_0_406: u64 = (s_0_405.value() as u64);
        // D s_0_407: write-var outdata <= s_0_406
        fn_state.outdata = s_0_406;
        // N s_0_408: return s_0_406
        return s_0_406;
    }
}
