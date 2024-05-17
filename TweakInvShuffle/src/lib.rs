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
use TweakCellInvRot::*;
use common::*;
pub fn TweakInvShuffle<T: Tracer>(state: &mut State, tracer: &T, indata: u64) -> u64 {
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
        // C s_0_0: const #48s : i
        let s_0_0: i128 = 48;
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
        // D s_0_9: call TweakCellInvRot(s_0_8)
        let s_0_9: u8 = TweakCellInvRot(state, tracer, s_0_8);
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // D s_0_11: read-var outdata:u64
        let s_0_11: u64 = fn_state.outdata;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 64u16);
        // D s_0_13: cast zx s_0_9 -> bv
        let s_0_13: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_14: const #3s : i
        let s_0_14: i128 = 3;
        // C s_0_15: const #1u : u64
        let s_0_15: u64 = 1;
        // C s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 64u16);
        // C s_0_17: lsl s_0_16 s_0_14
        let s_0_17: Bits = s_0_16 << s_0_14;
        // C s_0_18: sub s_0_17 s_0_16
        let s_0_18: Bits = ((s_0_17) - (s_0_16));
        // D s_0_19: and s_0_13 s_0_18
        let s_0_19: Bits = ((s_0_13) & (s_0_18));
        // D s_0_20: lsl s_0_19 s_0_10
        let s_0_20: Bits = s_0_19 << s_0_10;
        // C s_0_21: lsl s_0_18 s_0_10
        let s_0_21: Bits = s_0_18 << s_0_10;
        // C s_0_22: cmpl s_0_21
        let s_0_22: Bits = !s_0_21;
        // D s_0_23: and s_0_12 s_0_22
        let s_0_23: Bits = ((s_0_12) & (s_0_22));
        // D s_0_24: or s_0_23 s_0_20
        let s_0_24: Bits = ((s_0_23) | (s_0_20));
        // D s_0_25: cast reint s_0_24 -> u64
        let s_0_25: u64 = (s_0_24.value() as u64);
        // D s_0_26: write-var outdata <= s_0_25
        fn_state.outdata = s_0_25;
        // C s_0_27: const #52s : i
        let s_0_27: i128 = 52;
        // D s_0_28: read-var indata:u64
        let s_0_28: u64 = fn_state.indata;
        // D s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 64u16);
        // C s_0_30: const #1s : i64
        let s_0_30: i64 = 1;
        // C s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (i128::try_from(s_0_30).unwrap());
        // C s_0_32: const #3s : i
        let s_0_32: i128 = 3;
        // C s_0_33: add s_0_32 s_0_31
        let s_0_33: i128 = (s_0_32 + s_0_31);
        // D s_0_34: bit-extract s_0_29 s_0_27 s_0_33
        let s_0_34: Bits = (Bits::new(
            ((s_0_29) >> (s_0_27)).value(),
            u16::try_from(s_0_33).unwrap(),
        ));
        // D s_0_35: cast reint s_0_34 -> u8
        let s_0_35: u8 = (s_0_34.value() as u8);
        // C s_0_36: const #4s : i
        let s_0_36: i128 = 4;
        // D s_0_37: cast zx s_0_25 -> bv
        let s_0_37: Bits = Bits::new(s_0_25 as u128, 64u16);
        // D s_0_38: cast zx s_0_35 -> bv
        let s_0_38: Bits = Bits::new(s_0_35 as u128, 4u16);
        // C s_0_39: const #3s : i
        let s_0_39: i128 = 3;
        // C s_0_40: const #1u : u64
        let s_0_40: u64 = 1;
        // C s_0_41: cast zx s_0_40 -> bv
        let s_0_41: Bits = Bits::new(s_0_40 as u128, 64u16);
        // C s_0_42: lsl s_0_41 s_0_39
        let s_0_42: Bits = s_0_41 << s_0_39;
        // C s_0_43: sub s_0_42 s_0_41
        let s_0_43: Bits = ((s_0_42) - (s_0_41));
        // D s_0_44: and s_0_38 s_0_43
        let s_0_44: Bits = ((s_0_38) & (s_0_43));
        // D s_0_45: lsl s_0_44 s_0_36
        let s_0_45: Bits = s_0_44 << s_0_36;
        // C s_0_46: lsl s_0_43 s_0_36
        let s_0_46: Bits = s_0_43 << s_0_36;
        // C s_0_47: cmpl s_0_46
        let s_0_47: Bits = !s_0_46;
        // D s_0_48: and s_0_37 s_0_47
        let s_0_48: Bits = ((s_0_37) & (s_0_47));
        // D s_0_49: or s_0_48 s_0_45
        let s_0_49: Bits = ((s_0_48) | (s_0_45));
        // D s_0_50: cast reint s_0_49 -> u64
        let s_0_50: u64 = (s_0_49.value() as u64);
        // D s_0_51: write-var outdata <= s_0_50
        fn_state.outdata = s_0_50;
        // C s_0_52: const #20s : i
        let s_0_52: i128 = 20;
        // D s_0_53: read-var indata:u64
        let s_0_53: u64 = fn_state.indata;
        // D s_0_54: cast zx s_0_53 -> bv
        let s_0_54: Bits = Bits::new(s_0_53 as u128, 64u16);
        // C s_0_55: const #1s : i64
        let s_0_55: i64 = 1;
        // C s_0_56: cast zx s_0_55 -> i
        let s_0_56: i128 = (i128::try_from(s_0_55).unwrap());
        // C s_0_57: const #3s : i
        let s_0_57: i128 = 3;
        // C s_0_58: add s_0_57 s_0_56
        let s_0_58: i128 = (s_0_57 + s_0_56);
        // D s_0_59: bit-extract s_0_54 s_0_52 s_0_58
        let s_0_59: Bits = (Bits::new(
            ((s_0_54) >> (s_0_52)).value(),
            u16::try_from(s_0_58).unwrap(),
        ));
        // D s_0_60: cast reint s_0_59 -> u8
        let s_0_60: u8 = (s_0_59.value() as u8);
        // C s_0_61: const #8s : i
        let s_0_61: i128 = 8;
        // D s_0_62: cast zx s_0_50 -> bv
        let s_0_62: Bits = Bits::new(s_0_50 as u128, 64u16);
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
        // C s_0_77: const #24s : i
        let s_0_77: i128 = 24;
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
        // C s_0_102: const #0s : i
        let s_0_102: i128 = 0;
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
        // C s_0_111: const #16s : i
        let s_0_111: i128 = 16;
        // D s_0_112: cast zx s_0_100 -> bv
        let s_0_112: Bits = Bits::new(s_0_100 as u128, 64u16);
        // D s_0_113: cast zx s_0_110 -> bv
        let s_0_113: Bits = Bits::new(s_0_110 as u128, 4u16);
        // C s_0_114: const #3s : i
        let s_0_114: i128 = 3;
        // C s_0_115: const #1u : u64
        let s_0_115: u64 = 1;
        // C s_0_116: cast zx s_0_115 -> bv
        let s_0_116: Bits = Bits::new(s_0_115 as u128, 64u16);
        // C s_0_117: lsl s_0_116 s_0_114
        let s_0_117: Bits = s_0_116 << s_0_114;
        // C s_0_118: sub s_0_117 s_0_116
        let s_0_118: Bits = ((s_0_117) - (s_0_116));
        // D s_0_119: and s_0_113 s_0_118
        let s_0_119: Bits = ((s_0_113) & (s_0_118));
        // D s_0_120: lsl s_0_119 s_0_111
        let s_0_120: Bits = s_0_119 << s_0_111;
        // C s_0_121: lsl s_0_118 s_0_111
        let s_0_121: Bits = s_0_118 << s_0_111;
        // C s_0_122: cmpl s_0_121
        let s_0_122: Bits = !s_0_121;
        // D s_0_123: and s_0_112 s_0_122
        let s_0_123: Bits = ((s_0_112) & (s_0_122));
        // D s_0_124: or s_0_123 s_0_120
        let s_0_124: Bits = ((s_0_123) | (s_0_120));
        // D s_0_125: cast reint s_0_124 -> u64
        let s_0_125: u64 = (s_0_124.value() as u64);
        // D s_0_126: write-var outdata <= s_0_125
        fn_state.outdata = s_0_125;
        // C s_0_127: const #4s : i
        let s_0_127: i128 = 4;
        // D s_0_128: read-var indata:u64
        let s_0_128: u64 = fn_state.indata;
        // D s_0_129: cast zx s_0_128 -> bv
        let s_0_129: Bits = Bits::new(s_0_128 as u128, 64u16);
        // C s_0_130: const #1s : i64
        let s_0_130: i64 = 1;
        // C s_0_131: cast zx s_0_130 -> i
        let s_0_131: i128 = (i128::try_from(s_0_130).unwrap());
        // C s_0_132: const #3s : i
        let s_0_132: i128 = 3;
        // C s_0_133: add s_0_132 s_0_131
        let s_0_133: i128 = (s_0_132 + s_0_131);
        // D s_0_134: bit-extract s_0_129 s_0_127 s_0_133
        let s_0_134: Bits = (Bits::new(
            ((s_0_129) >> (s_0_127)).value(),
            u16::try_from(s_0_133).unwrap(),
        ));
        // D s_0_135: cast reint s_0_134 -> u8
        let s_0_135: u8 = (s_0_134.value() as u8);
        // C s_0_136: const #20s : i
        let s_0_136: i128 = 20;
        // D s_0_137: cast zx s_0_125 -> bv
        let s_0_137: Bits = Bits::new(s_0_125 as u128, 64u16);
        // D s_0_138: cast zx s_0_135 -> bv
        let s_0_138: Bits = Bits::new(s_0_135 as u128, 4u16);
        // C s_0_139: const #3s : i
        let s_0_139: i128 = 3;
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
        // D s_0_150: cast reint s_0_149 -> u64
        let s_0_150: u64 = (s_0_149.value() as u64);
        // D s_0_151: write-var outdata <= s_0_150
        fn_state.outdata = s_0_150;
        // C s_0_152: const #8s : i
        let s_0_152: i128 = 8;
        // D s_0_153: read-var indata:u64
        let s_0_153: u64 = fn_state.indata;
        // D s_0_154: cast zx s_0_153 -> bv
        let s_0_154: Bits = Bits::new(s_0_153 as u128, 64u16);
        // C s_0_155: const #1s : i64
        let s_0_155: i64 = 1;
        // C s_0_156: cast zx s_0_155 -> i
        let s_0_156: i128 = (i128::try_from(s_0_155).unwrap());
        // C s_0_157: const #3s : i
        let s_0_157: i128 = 3;
        // C s_0_158: add s_0_157 s_0_156
        let s_0_158: i128 = (s_0_157 + s_0_156);
        // D s_0_159: bit-extract s_0_154 s_0_152 s_0_158
        let s_0_159: Bits = (Bits::new(
            ((s_0_154) >> (s_0_152)).value(),
            u16::try_from(s_0_158).unwrap(),
        ));
        // D s_0_160: cast reint s_0_159 -> u8
        let s_0_160: u8 = (s_0_159.value() as u8);
        // D s_0_161: call TweakCellInvRot(s_0_160)
        let s_0_161: u8 = TweakCellInvRot(state, tracer, s_0_160);
        // C s_0_162: const #24s : i
        let s_0_162: i128 = 24;
        // D s_0_163: cast zx s_0_150 -> bv
        let s_0_163: Bits = Bits::new(s_0_150 as u128, 64u16);
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
        // C s_0_178: const #12s : i
        let s_0_178: i128 = 12;
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
        // C s_0_187: const #28s : i
        let s_0_187: i128 = 28;
        // D s_0_188: cast zx s_0_176 -> bv
        let s_0_188: Bits = Bits::new(s_0_176 as u128, 64u16);
        // D s_0_189: cast zx s_0_186 -> bv
        let s_0_189: Bits = Bits::new(s_0_186 as u128, 4u16);
        // C s_0_190: const #3s : i
        let s_0_190: i128 = 3;
        // C s_0_191: const #1u : u64
        let s_0_191: u64 = 1;
        // C s_0_192: cast zx s_0_191 -> bv
        let s_0_192: Bits = Bits::new(s_0_191 as u128, 64u16);
        // C s_0_193: lsl s_0_192 s_0_190
        let s_0_193: Bits = s_0_192 << s_0_190;
        // C s_0_194: sub s_0_193 s_0_192
        let s_0_194: Bits = ((s_0_193) - (s_0_192));
        // D s_0_195: and s_0_189 s_0_194
        let s_0_195: Bits = ((s_0_189) & (s_0_194));
        // D s_0_196: lsl s_0_195 s_0_187
        let s_0_196: Bits = s_0_195 << s_0_187;
        // C s_0_197: lsl s_0_194 s_0_187
        let s_0_197: Bits = s_0_194 << s_0_187;
        // C s_0_198: cmpl s_0_197
        let s_0_198: Bits = !s_0_197;
        // D s_0_199: and s_0_188 s_0_198
        let s_0_199: Bits = ((s_0_188) & (s_0_198));
        // D s_0_200: or s_0_199 s_0_196
        let s_0_200: Bits = ((s_0_199) | (s_0_196));
        // D s_0_201: cast reint s_0_200 -> u64
        let s_0_201: u64 = (s_0_200.value() as u64);
        // D s_0_202: write-var outdata <= s_0_201
        fn_state.outdata = s_0_201;
        // C s_0_203: const #28s : i
        let s_0_203: i128 = 28;
        // D s_0_204: read-var indata:u64
        let s_0_204: u64 = fn_state.indata;
        // D s_0_205: cast zx s_0_204 -> bv
        let s_0_205: Bits = Bits::new(s_0_204 as u128, 64u16);
        // C s_0_206: const #1s : i64
        let s_0_206: i64 = 1;
        // C s_0_207: cast zx s_0_206 -> i
        let s_0_207: i128 = (i128::try_from(s_0_206).unwrap());
        // C s_0_208: const #3s : i
        let s_0_208: i128 = 3;
        // C s_0_209: add s_0_208 s_0_207
        let s_0_209: i128 = (s_0_208 + s_0_207);
        // D s_0_210: bit-extract s_0_205 s_0_203 s_0_209
        let s_0_210: Bits = (Bits::new(
            ((s_0_205) >> (s_0_203)).value(),
            u16::try_from(s_0_209).unwrap(),
        ));
        // D s_0_211: cast reint s_0_210 -> u8
        let s_0_211: u8 = (s_0_210.value() as u8);
        // D s_0_212: call TweakCellInvRot(s_0_211)
        let s_0_212: u8 = TweakCellInvRot(state, tracer, s_0_211);
        // C s_0_213: const #32s : i
        let s_0_213: i128 = 32;
        // D s_0_214: cast zx s_0_201 -> bv
        let s_0_214: Bits = Bits::new(s_0_201 as u128, 64u16);
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
        // C s_0_229: const #60s : i
        let s_0_229: i128 = 60;
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
        // D s_0_238: call TweakCellInvRot(s_0_237)
        let s_0_238: u8 = TweakCellInvRot(state, tracer, s_0_237);
        // C s_0_239: const #36s : i
        let s_0_239: i128 = 36;
        // D s_0_240: cast zx s_0_227 -> bv
        let s_0_240: Bits = Bits::new(s_0_227 as u128, 64u16);
        // D s_0_241: cast zx s_0_238 -> bv
        let s_0_241: Bits = Bits::new(s_0_238 as u128, 4u16);
        // C s_0_242: const #3s : i
        let s_0_242: i128 = 3;
        // C s_0_243: const #1u : u64
        let s_0_243: u64 = 1;
        // C s_0_244: cast zx s_0_243 -> bv
        let s_0_244: Bits = Bits::new(s_0_243 as u128, 64u16);
        // C s_0_245: lsl s_0_244 s_0_242
        let s_0_245: Bits = s_0_244 << s_0_242;
        // C s_0_246: sub s_0_245 s_0_244
        let s_0_246: Bits = ((s_0_245) - (s_0_244));
        // D s_0_247: and s_0_241 s_0_246
        let s_0_247: Bits = ((s_0_241) & (s_0_246));
        // D s_0_248: lsl s_0_247 s_0_239
        let s_0_248: Bits = s_0_247 << s_0_239;
        // C s_0_249: lsl s_0_246 s_0_239
        let s_0_249: Bits = s_0_246 << s_0_239;
        // C s_0_250: cmpl s_0_249
        let s_0_250: Bits = !s_0_249;
        // D s_0_251: and s_0_240 s_0_250
        let s_0_251: Bits = ((s_0_240) & (s_0_250));
        // D s_0_252: or s_0_251 s_0_248
        let s_0_252: Bits = ((s_0_251) | (s_0_248));
        // D s_0_253: cast reint s_0_252 -> u64
        let s_0_253: u64 = (s_0_252.value() as u64);
        // D s_0_254: write-var outdata <= s_0_253
        fn_state.outdata = s_0_253;
        // C s_0_255: const #56s : i
        let s_0_255: i128 = 56;
        // D s_0_256: read-var indata:u64
        let s_0_256: u64 = fn_state.indata;
        // D s_0_257: cast zx s_0_256 -> bv
        let s_0_257: Bits = Bits::new(s_0_256 as u128, 64u16);
        // C s_0_258: const #1s : i64
        let s_0_258: i64 = 1;
        // C s_0_259: cast zx s_0_258 -> i
        let s_0_259: i128 = (i128::try_from(s_0_258).unwrap());
        // C s_0_260: const #3s : i
        let s_0_260: i128 = 3;
        // C s_0_261: add s_0_260 s_0_259
        let s_0_261: i128 = (s_0_260 + s_0_259);
        // D s_0_262: bit-extract s_0_257 s_0_255 s_0_261
        let s_0_262: Bits = (Bits::new(
            ((s_0_257) >> (s_0_255)).value(),
            u16::try_from(s_0_261).unwrap(),
        ));
        // D s_0_263: cast reint s_0_262 -> u8
        let s_0_263: u8 = (s_0_262.value() as u8);
        // D s_0_264: call TweakCellInvRot(s_0_263)
        let s_0_264: u8 = TweakCellInvRot(state, tracer, s_0_263);
        // C s_0_265: const #40s : i
        let s_0_265: i128 = 40;
        // D s_0_266: cast zx s_0_253 -> bv
        let s_0_266: Bits = Bits::new(s_0_253 as u128, 64u16);
        // D s_0_267: cast zx s_0_264 -> bv
        let s_0_267: Bits = Bits::new(s_0_264 as u128, 4u16);
        // C s_0_268: const #3s : i
        let s_0_268: i128 = 3;
        // C s_0_269: const #1u : u64
        let s_0_269: u64 = 1;
        // C s_0_270: cast zx s_0_269 -> bv
        let s_0_270: Bits = Bits::new(s_0_269 as u128, 64u16);
        // C s_0_271: lsl s_0_270 s_0_268
        let s_0_271: Bits = s_0_270 << s_0_268;
        // C s_0_272: sub s_0_271 s_0_270
        let s_0_272: Bits = ((s_0_271) - (s_0_270));
        // D s_0_273: and s_0_267 s_0_272
        let s_0_273: Bits = ((s_0_267) & (s_0_272));
        // D s_0_274: lsl s_0_273 s_0_265
        let s_0_274: Bits = s_0_273 << s_0_265;
        // C s_0_275: lsl s_0_272 s_0_265
        let s_0_275: Bits = s_0_272 << s_0_265;
        // C s_0_276: cmpl s_0_275
        let s_0_276: Bits = !s_0_275;
        // D s_0_277: and s_0_266 s_0_276
        let s_0_277: Bits = ((s_0_266) & (s_0_276));
        // D s_0_278: or s_0_277 s_0_274
        let s_0_278: Bits = ((s_0_277) | (s_0_274));
        // D s_0_279: cast reint s_0_278 -> u64
        let s_0_279: u64 = (s_0_278.value() as u64);
        // D s_0_280: write-var outdata <= s_0_279
        fn_state.outdata = s_0_279;
        // C s_0_281: const #16s : i
        let s_0_281: i128 = 16;
        // D s_0_282: read-var indata:u64
        let s_0_282: u64 = fn_state.indata;
        // D s_0_283: cast zx s_0_282 -> bv
        let s_0_283: Bits = Bits::new(s_0_282 as u128, 64u16);
        // C s_0_284: const #1s : i64
        let s_0_284: i64 = 1;
        // C s_0_285: cast zx s_0_284 -> i
        let s_0_285: i128 = (i128::try_from(s_0_284).unwrap());
        // C s_0_286: const #3s : i
        let s_0_286: i128 = 3;
        // C s_0_287: add s_0_286 s_0_285
        let s_0_287: i128 = (s_0_286 + s_0_285);
        // D s_0_288: bit-extract s_0_283 s_0_281 s_0_287
        let s_0_288: Bits = (Bits::new(
            ((s_0_283) >> (s_0_281)).value(),
            u16::try_from(s_0_287).unwrap(),
        ));
        // D s_0_289: cast reint s_0_288 -> u8
        let s_0_289: u8 = (s_0_288.value() as u8);
        // D s_0_290: call TweakCellInvRot(s_0_289)
        let s_0_290: u8 = TweakCellInvRot(state, tracer, s_0_289);
        // C s_0_291: const #44s : i
        let s_0_291: i128 = 44;
        // D s_0_292: cast zx s_0_279 -> bv
        let s_0_292: Bits = Bits::new(s_0_279 as u128, 64u16);
        // D s_0_293: cast zx s_0_290 -> bv
        let s_0_293: Bits = Bits::new(s_0_290 as u128, 4u16);
        // C s_0_294: const #3s : i
        let s_0_294: i128 = 3;
        // C s_0_295: const #1u : u64
        let s_0_295: u64 = 1;
        // C s_0_296: cast zx s_0_295 -> bv
        let s_0_296: Bits = Bits::new(s_0_295 as u128, 64u16);
        // C s_0_297: lsl s_0_296 s_0_294
        let s_0_297: Bits = s_0_296 << s_0_294;
        // C s_0_298: sub s_0_297 s_0_296
        let s_0_298: Bits = ((s_0_297) - (s_0_296));
        // D s_0_299: and s_0_293 s_0_298
        let s_0_299: Bits = ((s_0_293) & (s_0_298));
        // D s_0_300: lsl s_0_299 s_0_291
        let s_0_300: Bits = s_0_299 << s_0_291;
        // C s_0_301: lsl s_0_298 s_0_291
        let s_0_301: Bits = s_0_298 << s_0_291;
        // C s_0_302: cmpl s_0_301
        let s_0_302: Bits = !s_0_301;
        // D s_0_303: and s_0_292 s_0_302
        let s_0_303: Bits = ((s_0_292) & (s_0_302));
        // D s_0_304: or s_0_303 s_0_300
        let s_0_304: Bits = ((s_0_303) | (s_0_300));
        // D s_0_305: cast reint s_0_304 -> u64
        let s_0_305: u64 = (s_0_304.value() as u64);
        // D s_0_306: write-var outdata <= s_0_305
        fn_state.outdata = s_0_305;
        // C s_0_307: const #32s : i
        let s_0_307: i128 = 32;
        // D s_0_308: read-var indata:u64
        let s_0_308: u64 = fn_state.indata;
        // D s_0_309: cast zx s_0_308 -> bv
        let s_0_309: Bits = Bits::new(s_0_308 as u128, 64u16);
        // C s_0_310: const #1s : i64
        let s_0_310: i64 = 1;
        // C s_0_311: cast zx s_0_310 -> i
        let s_0_311: i128 = (i128::try_from(s_0_310).unwrap());
        // C s_0_312: const #3s : i
        let s_0_312: i128 = 3;
        // C s_0_313: add s_0_312 s_0_311
        let s_0_313: i128 = (s_0_312 + s_0_311);
        // D s_0_314: bit-extract s_0_309 s_0_307 s_0_313
        let s_0_314: Bits = (Bits::new(
            ((s_0_309) >> (s_0_307)).value(),
            u16::try_from(s_0_313).unwrap(),
        ));
        // D s_0_315: cast reint s_0_314 -> u8
        let s_0_315: u8 = (s_0_314.value() as u8);
        // C s_0_316: const #48s : i
        let s_0_316: i128 = 48;
        // D s_0_317: cast zx s_0_305 -> bv
        let s_0_317: Bits = Bits::new(s_0_305 as u128, 64u16);
        // D s_0_318: cast zx s_0_315 -> bv
        let s_0_318: Bits = Bits::new(s_0_315 as u128, 4u16);
        // C s_0_319: const #3s : i
        let s_0_319: i128 = 3;
        // C s_0_320: const #1u : u64
        let s_0_320: u64 = 1;
        // C s_0_321: cast zx s_0_320 -> bv
        let s_0_321: Bits = Bits::new(s_0_320 as u128, 64u16);
        // C s_0_322: lsl s_0_321 s_0_319
        let s_0_322: Bits = s_0_321 << s_0_319;
        // C s_0_323: sub s_0_322 s_0_321
        let s_0_323: Bits = ((s_0_322) - (s_0_321));
        // D s_0_324: and s_0_318 s_0_323
        let s_0_324: Bits = ((s_0_318) & (s_0_323));
        // D s_0_325: lsl s_0_324 s_0_316
        let s_0_325: Bits = s_0_324 << s_0_316;
        // C s_0_326: lsl s_0_323 s_0_316
        let s_0_326: Bits = s_0_323 << s_0_316;
        // C s_0_327: cmpl s_0_326
        let s_0_327: Bits = !s_0_326;
        // D s_0_328: and s_0_317 s_0_327
        let s_0_328: Bits = ((s_0_317) & (s_0_327));
        // D s_0_329: or s_0_328 s_0_325
        let s_0_329: Bits = ((s_0_328) | (s_0_325));
        // D s_0_330: cast reint s_0_329 -> u64
        let s_0_330: u64 = (s_0_329.value() as u64);
        // D s_0_331: write-var outdata <= s_0_330
        fn_state.outdata = s_0_330;
        // C s_0_332: const #36s : i
        let s_0_332: i128 = 36;
        // D s_0_333: read-var indata:u64
        let s_0_333: u64 = fn_state.indata;
        // D s_0_334: cast zx s_0_333 -> bv
        let s_0_334: Bits = Bits::new(s_0_333 as u128, 64u16);
        // C s_0_335: const #1s : i64
        let s_0_335: i64 = 1;
        // C s_0_336: cast zx s_0_335 -> i
        let s_0_336: i128 = (i128::try_from(s_0_335).unwrap());
        // C s_0_337: const #3s : i
        let s_0_337: i128 = 3;
        // C s_0_338: add s_0_337 s_0_336
        let s_0_338: i128 = (s_0_337 + s_0_336);
        // D s_0_339: bit-extract s_0_334 s_0_332 s_0_338
        let s_0_339: Bits = (Bits::new(
            ((s_0_334) >> (s_0_332)).value(),
            u16::try_from(s_0_338).unwrap(),
        ));
        // D s_0_340: cast reint s_0_339 -> u8
        let s_0_340: u8 = (s_0_339.value() as u8);
        // C s_0_341: const #52s : i
        let s_0_341: i128 = 52;
        // D s_0_342: cast zx s_0_330 -> bv
        let s_0_342: Bits = Bits::new(s_0_330 as u128, 64u16);
        // D s_0_343: cast zx s_0_340 -> bv
        let s_0_343: Bits = Bits::new(s_0_340 as u128, 4u16);
        // C s_0_344: const #3s : i
        let s_0_344: i128 = 3;
        // C s_0_345: const #1u : u64
        let s_0_345: u64 = 1;
        // C s_0_346: cast zx s_0_345 -> bv
        let s_0_346: Bits = Bits::new(s_0_345 as u128, 64u16);
        // C s_0_347: lsl s_0_346 s_0_344
        let s_0_347: Bits = s_0_346 << s_0_344;
        // C s_0_348: sub s_0_347 s_0_346
        let s_0_348: Bits = ((s_0_347) - (s_0_346));
        // D s_0_349: and s_0_343 s_0_348
        let s_0_349: Bits = ((s_0_343) & (s_0_348));
        // D s_0_350: lsl s_0_349 s_0_341
        let s_0_350: Bits = s_0_349 << s_0_341;
        // C s_0_351: lsl s_0_348 s_0_341
        let s_0_351: Bits = s_0_348 << s_0_341;
        // C s_0_352: cmpl s_0_351
        let s_0_352: Bits = !s_0_351;
        // D s_0_353: and s_0_342 s_0_352
        let s_0_353: Bits = ((s_0_342) & (s_0_352));
        // D s_0_354: or s_0_353 s_0_350
        let s_0_354: Bits = ((s_0_353) | (s_0_350));
        // D s_0_355: cast reint s_0_354 -> u64
        let s_0_355: u64 = (s_0_354.value() as u64);
        // D s_0_356: write-var outdata <= s_0_355
        fn_state.outdata = s_0_355;
        // C s_0_357: const #40s : i
        let s_0_357: i128 = 40;
        // D s_0_358: read-var indata:u64
        let s_0_358: u64 = fn_state.indata;
        // D s_0_359: cast zx s_0_358 -> bv
        let s_0_359: Bits = Bits::new(s_0_358 as u128, 64u16);
        // C s_0_360: const #1s : i64
        let s_0_360: i64 = 1;
        // C s_0_361: cast zx s_0_360 -> i
        let s_0_361: i128 = (i128::try_from(s_0_360).unwrap());
        // C s_0_362: const #3s : i
        let s_0_362: i128 = 3;
        // C s_0_363: add s_0_362 s_0_361
        let s_0_363: i128 = (s_0_362 + s_0_361);
        // D s_0_364: bit-extract s_0_359 s_0_357 s_0_363
        let s_0_364: Bits = (Bits::new(
            ((s_0_359) >> (s_0_357)).value(),
            u16::try_from(s_0_363).unwrap(),
        ));
        // D s_0_365: cast reint s_0_364 -> u8
        let s_0_365: u8 = (s_0_364.value() as u8);
        // C s_0_366: const #56s : i
        let s_0_366: i128 = 56;
        // D s_0_367: cast zx s_0_355 -> bv
        let s_0_367: Bits = Bits::new(s_0_355 as u128, 64u16);
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
        // C s_0_382: const #44s : i
        let s_0_382: i128 = 44;
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
        // D s_0_391: call TweakCellInvRot(s_0_390)
        let s_0_391: u8 = TweakCellInvRot(state, tracer, s_0_390);
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
