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
use common::*;
pub fn PACCellShuffle<T: Tracer>(state: &mut State, tracer: &T, indata: u64) -> u64 {
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
        // C s_0_0: const #52s : i
        let s_0_0: i128 = 52;
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
        // C s_0_26: const #24s : i
        let s_0_26: i128 = 24;
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
        // C s_0_51: const #44s : i
        let s_0_51: i128 = 44;
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
        // C s_0_60: const #8s : i
        let s_0_60: i128 = 8;
        // D s_0_61: cast zx s_0_49 -> bv
        let s_0_61: Bits = Bits::new(s_0_49 as u128, 64u16);
        // D s_0_62: cast zx s_0_59 -> bv
        let s_0_62: Bits = Bits::new(s_0_59 as u128, 4u16);
        // C s_0_63: const #3s : i
        let s_0_63: i128 = 3;
        // C s_0_64: const #1u : u64
        let s_0_64: u64 = 1;
        // C s_0_65: cast zx s_0_64 -> bv
        let s_0_65: Bits = Bits::new(s_0_64 as u128, 64u16);
        // C s_0_66: lsl s_0_65 s_0_63
        let s_0_66: Bits = s_0_65 << s_0_63;
        // C s_0_67: sub s_0_66 s_0_65
        let s_0_67: Bits = ((s_0_66) - (s_0_65));
        // D s_0_68: and s_0_62 s_0_67
        let s_0_68: Bits = ((s_0_62) & (s_0_67));
        // D s_0_69: lsl s_0_68 s_0_60
        let s_0_69: Bits = s_0_68 << s_0_60;
        // C s_0_70: lsl s_0_67 s_0_60
        let s_0_70: Bits = s_0_67 << s_0_60;
        // C s_0_71: cmpl s_0_70
        let s_0_71: Bits = !s_0_70;
        // D s_0_72: and s_0_61 s_0_71
        let s_0_72: Bits = ((s_0_61) & (s_0_71));
        // D s_0_73: or s_0_72 s_0_69
        let s_0_73: Bits = ((s_0_72) | (s_0_69));
        // D s_0_74: cast reint s_0_73 -> u64
        let s_0_74: u64 = (s_0_73.value() as u64);
        // D s_0_75: write-var outdata <= s_0_74
        fn_state.outdata = s_0_74;
        // C s_0_76: const #0s : i
        let s_0_76: i128 = 0;
        // D s_0_77: read-var indata:u64
        let s_0_77: u64 = fn_state.indata;
        // D s_0_78: cast zx s_0_77 -> bv
        let s_0_78: Bits = Bits::new(s_0_77 as u128, 64u16);
        // C s_0_79: const #1s : i64
        let s_0_79: i64 = 1;
        // C s_0_80: cast zx s_0_79 -> i
        let s_0_80: i128 = (i128::try_from(s_0_79).unwrap());
        // C s_0_81: const #3s : i
        let s_0_81: i128 = 3;
        // C s_0_82: add s_0_81 s_0_80
        let s_0_82: i128 = (s_0_81 + s_0_80);
        // D s_0_83: bit-extract s_0_78 s_0_76 s_0_82
        let s_0_83: Bits = (Bits::new(
            ((s_0_78) >> (s_0_76)).value(),
            u16::try_from(s_0_82).unwrap(),
        ));
        // D s_0_84: cast reint s_0_83 -> u8
        let s_0_84: u8 = (s_0_83.value() as u8);
        // C s_0_85: const #12s : i
        let s_0_85: i128 = 12;
        // D s_0_86: cast zx s_0_74 -> bv
        let s_0_86: Bits = Bits::new(s_0_74 as u128, 64u16);
        // D s_0_87: cast zx s_0_84 -> bv
        let s_0_87: Bits = Bits::new(s_0_84 as u128, 4u16);
        // C s_0_88: const #3s : i
        let s_0_88: i128 = 3;
        // C s_0_89: const #1u : u64
        let s_0_89: u64 = 1;
        // C s_0_90: cast zx s_0_89 -> bv
        let s_0_90: Bits = Bits::new(s_0_89 as u128, 64u16);
        // C s_0_91: lsl s_0_90 s_0_88
        let s_0_91: Bits = s_0_90 << s_0_88;
        // C s_0_92: sub s_0_91 s_0_90
        let s_0_92: Bits = ((s_0_91) - (s_0_90));
        // D s_0_93: and s_0_87 s_0_92
        let s_0_93: Bits = ((s_0_87) & (s_0_92));
        // D s_0_94: lsl s_0_93 s_0_85
        let s_0_94: Bits = s_0_93 << s_0_85;
        // C s_0_95: lsl s_0_92 s_0_85
        let s_0_95: Bits = s_0_92 << s_0_85;
        // C s_0_96: cmpl s_0_95
        let s_0_96: Bits = !s_0_95;
        // D s_0_97: and s_0_86 s_0_96
        let s_0_97: Bits = ((s_0_86) & (s_0_96));
        // D s_0_98: or s_0_97 s_0_94
        let s_0_98: Bits = ((s_0_97) | (s_0_94));
        // D s_0_99: cast reint s_0_98 -> u64
        let s_0_99: u64 = (s_0_98.value() as u64);
        // D s_0_100: write-var outdata <= s_0_99
        fn_state.outdata = s_0_99;
        // C s_0_101: const #28s : i
        let s_0_101: i128 = 28;
        // D s_0_102: read-var indata:u64
        let s_0_102: u64 = fn_state.indata;
        // D s_0_103: cast zx s_0_102 -> bv
        let s_0_103: Bits = Bits::new(s_0_102 as u128, 64u16);
        // C s_0_104: const #1s : i64
        let s_0_104: i64 = 1;
        // C s_0_105: cast zx s_0_104 -> i
        let s_0_105: i128 = (i128::try_from(s_0_104).unwrap());
        // C s_0_106: const #3s : i
        let s_0_106: i128 = 3;
        // C s_0_107: add s_0_106 s_0_105
        let s_0_107: i128 = (s_0_106 + s_0_105);
        // D s_0_108: bit-extract s_0_103 s_0_101 s_0_107
        let s_0_108: Bits = (Bits::new(
            ((s_0_103) >> (s_0_101)).value(),
            u16::try_from(s_0_107).unwrap(),
        ));
        // D s_0_109: cast reint s_0_108 -> u8
        let s_0_109: u8 = (s_0_108.value() as u8);
        // C s_0_110: const #16s : i
        let s_0_110: i128 = 16;
        // D s_0_111: cast zx s_0_99 -> bv
        let s_0_111: Bits = Bits::new(s_0_99 as u128, 64u16);
        // D s_0_112: cast zx s_0_109 -> bv
        let s_0_112: Bits = Bits::new(s_0_109 as u128, 4u16);
        // C s_0_113: const #3s : i
        let s_0_113: i128 = 3;
        // C s_0_114: const #1u : u64
        let s_0_114: u64 = 1;
        // C s_0_115: cast zx s_0_114 -> bv
        let s_0_115: Bits = Bits::new(s_0_114 as u128, 64u16);
        // C s_0_116: lsl s_0_115 s_0_113
        let s_0_116: Bits = s_0_115 << s_0_113;
        // C s_0_117: sub s_0_116 s_0_115
        let s_0_117: Bits = ((s_0_116) - (s_0_115));
        // D s_0_118: and s_0_112 s_0_117
        let s_0_118: Bits = ((s_0_112) & (s_0_117));
        // D s_0_119: lsl s_0_118 s_0_110
        let s_0_119: Bits = s_0_118 << s_0_110;
        // C s_0_120: lsl s_0_117 s_0_110
        let s_0_120: Bits = s_0_117 << s_0_110;
        // C s_0_121: cmpl s_0_120
        let s_0_121: Bits = !s_0_120;
        // D s_0_122: and s_0_111 s_0_121
        let s_0_122: Bits = ((s_0_111) & (s_0_121));
        // D s_0_123: or s_0_122 s_0_119
        let s_0_123: Bits = ((s_0_122) | (s_0_119));
        // D s_0_124: cast reint s_0_123 -> u64
        let s_0_124: u64 = (s_0_123.value() as u64);
        // D s_0_125: write-var outdata <= s_0_124
        fn_state.outdata = s_0_124;
        // C s_0_126: const #48s : i
        let s_0_126: i128 = 48;
        // D s_0_127: read-var indata:u64
        let s_0_127: u64 = fn_state.indata;
        // D s_0_128: cast zx s_0_127 -> bv
        let s_0_128: Bits = Bits::new(s_0_127 as u128, 64u16);
        // C s_0_129: const #1s : i64
        let s_0_129: i64 = 1;
        // C s_0_130: cast zx s_0_129 -> i
        let s_0_130: i128 = (i128::try_from(s_0_129).unwrap());
        // C s_0_131: const #3s : i
        let s_0_131: i128 = 3;
        // C s_0_132: add s_0_131 s_0_130
        let s_0_132: i128 = (s_0_131 + s_0_130);
        // D s_0_133: bit-extract s_0_128 s_0_126 s_0_132
        let s_0_133: Bits = (Bits::new(
            ((s_0_128) >> (s_0_126)).value(),
            u16::try_from(s_0_132).unwrap(),
        ));
        // D s_0_134: cast reint s_0_133 -> u8
        let s_0_134: u8 = (s_0_133.value() as u8);
        // C s_0_135: const #20s : i
        let s_0_135: i128 = 20;
        // D s_0_136: cast zx s_0_124 -> bv
        let s_0_136: Bits = Bits::new(s_0_124 as u128, 64u16);
        // D s_0_137: cast zx s_0_134 -> bv
        let s_0_137: Bits = Bits::new(s_0_134 as u128, 4u16);
        // C s_0_138: const #3s : i
        let s_0_138: i128 = 3;
        // C s_0_139: const #1u : u64
        let s_0_139: u64 = 1;
        // C s_0_140: cast zx s_0_139 -> bv
        let s_0_140: Bits = Bits::new(s_0_139 as u128, 64u16);
        // C s_0_141: lsl s_0_140 s_0_138
        let s_0_141: Bits = s_0_140 << s_0_138;
        // C s_0_142: sub s_0_141 s_0_140
        let s_0_142: Bits = ((s_0_141) - (s_0_140));
        // D s_0_143: and s_0_137 s_0_142
        let s_0_143: Bits = ((s_0_137) & (s_0_142));
        // D s_0_144: lsl s_0_143 s_0_135
        let s_0_144: Bits = s_0_143 << s_0_135;
        // C s_0_145: lsl s_0_142 s_0_135
        let s_0_145: Bits = s_0_142 << s_0_135;
        // C s_0_146: cmpl s_0_145
        let s_0_146: Bits = !s_0_145;
        // D s_0_147: and s_0_136 s_0_146
        let s_0_147: Bits = ((s_0_136) & (s_0_146));
        // D s_0_148: or s_0_147 s_0_144
        let s_0_148: Bits = ((s_0_147) | (s_0_144));
        // D s_0_149: cast reint s_0_148 -> u64
        let s_0_149: u64 = (s_0_148.value() as u64);
        // D s_0_150: write-var outdata <= s_0_149
        fn_state.outdata = s_0_149;
        // C s_0_151: const #4s : i
        let s_0_151: i128 = 4;
        // D s_0_152: read-var indata:u64
        let s_0_152: u64 = fn_state.indata;
        // D s_0_153: cast zx s_0_152 -> bv
        let s_0_153: Bits = Bits::new(s_0_152 as u128, 64u16);
        // C s_0_154: const #1s : i64
        let s_0_154: i64 = 1;
        // C s_0_155: cast zx s_0_154 -> i
        let s_0_155: i128 = (i128::try_from(s_0_154).unwrap());
        // C s_0_156: const #3s : i
        let s_0_156: i128 = 3;
        // C s_0_157: add s_0_156 s_0_155
        let s_0_157: i128 = (s_0_156 + s_0_155);
        // D s_0_158: bit-extract s_0_153 s_0_151 s_0_157
        let s_0_158: Bits = (Bits::new(
            ((s_0_153) >> (s_0_151)).value(),
            u16::try_from(s_0_157).unwrap(),
        ));
        // D s_0_159: cast reint s_0_158 -> u8
        let s_0_159: u8 = (s_0_158.value() as u8);
        // C s_0_160: const #24s : i
        let s_0_160: i128 = 24;
        // D s_0_161: cast zx s_0_149 -> bv
        let s_0_161: Bits = Bits::new(s_0_149 as u128, 64u16);
        // D s_0_162: cast zx s_0_159 -> bv
        let s_0_162: Bits = Bits::new(s_0_159 as u128, 4u16);
        // C s_0_163: const #3s : i
        let s_0_163: i128 = 3;
        // C s_0_164: const #1u : u64
        let s_0_164: u64 = 1;
        // C s_0_165: cast zx s_0_164 -> bv
        let s_0_165: Bits = Bits::new(s_0_164 as u128, 64u16);
        // C s_0_166: lsl s_0_165 s_0_163
        let s_0_166: Bits = s_0_165 << s_0_163;
        // C s_0_167: sub s_0_166 s_0_165
        let s_0_167: Bits = ((s_0_166) - (s_0_165));
        // D s_0_168: and s_0_162 s_0_167
        let s_0_168: Bits = ((s_0_162) & (s_0_167));
        // D s_0_169: lsl s_0_168 s_0_160
        let s_0_169: Bits = s_0_168 << s_0_160;
        // C s_0_170: lsl s_0_167 s_0_160
        let s_0_170: Bits = s_0_167 << s_0_160;
        // C s_0_171: cmpl s_0_170
        let s_0_171: Bits = !s_0_170;
        // D s_0_172: and s_0_161 s_0_171
        let s_0_172: Bits = ((s_0_161) & (s_0_171));
        // D s_0_173: or s_0_172 s_0_169
        let s_0_173: Bits = ((s_0_172) | (s_0_169));
        // D s_0_174: cast reint s_0_173 -> u64
        let s_0_174: u64 = (s_0_173.value() as u64);
        // D s_0_175: write-var outdata <= s_0_174
        fn_state.outdata = s_0_174;
        // C s_0_176: const #40s : i
        let s_0_176: i128 = 40;
        // D s_0_177: read-var indata:u64
        let s_0_177: u64 = fn_state.indata;
        // D s_0_178: cast zx s_0_177 -> bv
        let s_0_178: Bits = Bits::new(s_0_177 as u128, 64u16);
        // C s_0_179: const #1s : i64
        let s_0_179: i64 = 1;
        // C s_0_180: cast zx s_0_179 -> i
        let s_0_180: i128 = (i128::try_from(s_0_179).unwrap());
        // C s_0_181: const #3s : i
        let s_0_181: i128 = 3;
        // C s_0_182: add s_0_181 s_0_180
        let s_0_182: i128 = (s_0_181 + s_0_180);
        // D s_0_183: bit-extract s_0_178 s_0_176 s_0_182
        let s_0_183: Bits = (Bits::new(
            ((s_0_178) >> (s_0_176)).value(),
            u16::try_from(s_0_182).unwrap(),
        ));
        // D s_0_184: cast reint s_0_183 -> u8
        let s_0_184: u8 = (s_0_183.value() as u8);
        // C s_0_185: const #28s : i
        let s_0_185: i128 = 28;
        // D s_0_186: cast zx s_0_174 -> bv
        let s_0_186: Bits = Bits::new(s_0_174 as u128, 64u16);
        // D s_0_187: cast zx s_0_184 -> bv
        let s_0_187: Bits = Bits::new(s_0_184 as u128, 4u16);
        // C s_0_188: const #3s : i
        let s_0_188: i128 = 3;
        // C s_0_189: const #1u : u64
        let s_0_189: u64 = 1;
        // C s_0_190: cast zx s_0_189 -> bv
        let s_0_190: Bits = Bits::new(s_0_189 as u128, 64u16);
        // C s_0_191: lsl s_0_190 s_0_188
        let s_0_191: Bits = s_0_190 << s_0_188;
        // C s_0_192: sub s_0_191 s_0_190
        let s_0_192: Bits = ((s_0_191) - (s_0_190));
        // D s_0_193: and s_0_187 s_0_192
        let s_0_193: Bits = ((s_0_187) & (s_0_192));
        // D s_0_194: lsl s_0_193 s_0_185
        let s_0_194: Bits = s_0_193 << s_0_185;
        // C s_0_195: lsl s_0_192 s_0_185
        let s_0_195: Bits = s_0_192 << s_0_185;
        // C s_0_196: cmpl s_0_195
        let s_0_196: Bits = !s_0_195;
        // D s_0_197: and s_0_186 s_0_196
        let s_0_197: Bits = ((s_0_186) & (s_0_196));
        // D s_0_198: or s_0_197 s_0_194
        let s_0_198: Bits = ((s_0_197) | (s_0_194));
        // D s_0_199: cast reint s_0_198 -> u64
        let s_0_199: u64 = (s_0_198.value() as u64);
        // D s_0_200: write-var outdata <= s_0_199
        fn_state.outdata = s_0_199;
        // C s_0_201: const #32s : i
        let s_0_201: i128 = 32;
        // D s_0_202: read-var indata:u64
        let s_0_202: u64 = fn_state.indata;
        // D s_0_203: cast zx s_0_202 -> bv
        let s_0_203: Bits = Bits::new(s_0_202 as u128, 64u16);
        // C s_0_204: const #1s : i64
        let s_0_204: i64 = 1;
        // C s_0_205: cast zx s_0_204 -> i
        let s_0_205: i128 = (i128::try_from(s_0_204).unwrap());
        // C s_0_206: const #3s : i
        let s_0_206: i128 = 3;
        // C s_0_207: add s_0_206 s_0_205
        let s_0_207: i128 = (s_0_206 + s_0_205);
        // D s_0_208: bit-extract s_0_203 s_0_201 s_0_207
        let s_0_208: Bits = (Bits::new(
            ((s_0_203) >> (s_0_201)).value(),
            u16::try_from(s_0_207).unwrap(),
        ));
        // D s_0_209: cast reint s_0_208 -> u8
        let s_0_209: u8 = (s_0_208.value() as u8);
        // C s_0_210: const #32s : i
        let s_0_210: i128 = 32;
        // D s_0_211: cast zx s_0_199 -> bv
        let s_0_211: Bits = Bits::new(s_0_199 as u128, 64u16);
        // D s_0_212: cast zx s_0_209 -> bv
        let s_0_212: Bits = Bits::new(s_0_209 as u128, 4u16);
        // C s_0_213: const #3s : i
        let s_0_213: i128 = 3;
        // C s_0_214: const #1u : u64
        let s_0_214: u64 = 1;
        // C s_0_215: cast zx s_0_214 -> bv
        let s_0_215: Bits = Bits::new(s_0_214 as u128, 64u16);
        // C s_0_216: lsl s_0_215 s_0_213
        let s_0_216: Bits = s_0_215 << s_0_213;
        // C s_0_217: sub s_0_216 s_0_215
        let s_0_217: Bits = ((s_0_216) - (s_0_215));
        // D s_0_218: and s_0_212 s_0_217
        let s_0_218: Bits = ((s_0_212) & (s_0_217));
        // D s_0_219: lsl s_0_218 s_0_210
        let s_0_219: Bits = s_0_218 << s_0_210;
        // C s_0_220: lsl s_0_217 s_0_210
        let s_0_220: Bits = s_0_217 << s_0_210;
        // C s_0_221: cmpl s_0_220
        let s_0_221: Bits = !s_0_220;
        // D s_0_222: and s_0_211 s_0_221
        let s_0_222: Bits = ((s_0_211) & (s_0_221));
        // D s_0_223: or s_0_222 s_0_219
        let s_0_223: Bits = ((s_0_222) | (s_0_219));
        // D s_0_224: cast reint s_0_223 -> u64
        let s_0_224: u64 = (s_0_223.value() as u64);
        // D s_0_225: write-var outdata <= s_0_224
        fn_state.outdata = s_0_224;
        // C s_0_226: const #12s : i
        let s_0_226: i128 = 12;
        // D s_0_227: read-var indata:u64
        let s_0_227: u64 = fn_state.indata;
        // D s_0_228: cast zx s_0_227 -> bv
        let s_0_228: Bits = Bits::new(s_0_227 as u128, 64u16);
        // C s_0_229: const #1s : i64
        let s_0_229: i64 = 1;
        // C s_0_230: cast zx s_0_229 -> i
        let s_0_230: i128 = (i128::try_from(s_0_229).unwrap());
        // C s_0_231: const #3s : i
        let s_0_231: i128 = 3;
        // C s_0_232: add s_0_231 s_0_230
        let s_0_232: i128 = (s_0_231 + s_0_230);
        // D s_0_233: bit-extract s_0_228 s_0_226 s_0_232
        let s_0_233: Bits = (Bits::new(
            ((s_0_228) >> (s_0_226)).value(),
            u16::try_from(s_0_232).unwrap(),
        ));
        // D s_0_234: cast reint s_0_233 -> u8
        let s_0_234: u8 = (s_0_233.value() as u8);
        // C s_0_235: const #36s : i
        let s_0_235: i128 = 36;
        // D s_0_236: cast zx s_0_224 -> bv
        let s_0_236: Bits = Bits::new(s_0_224 as u128, 64u16);
        // D s_0_237: cast zx s_0_234 -> bv
        let s_0_237: Bits = Bits::new(s_0_234 as u128, 4u16);
        // C s_0_238: const #3s : i
        let s_0_238: i128 = 3;
        // C s_0_239: const #1u : u64
        let s_0_239: u64 = 1;
        // C s_0_240: cast zx s_0_239 -> bv
        let s_0_240: Bits = Bits::new(s_0_239 as u128, 64u16);
        // C s_0_241: lsl s_0_240 s_0_238
        let s_0_241: Bits = s_0_240 << s_0_238;
        // C s_0_242: sub s_0_241 s_0_240
        let s_0_242: Bits = ((s_0_241) - (s_0_240));
        // D s_0_243: and s_0_237 s_0_242
        let s_0_243: Bits = ((s_0_237) & (s_0_242));
        // D s_0_244: lsl s_0_243 s_0_235
        let s_0_244: Bits = s_0_243 << s_0_235;
        // C s_0_245: lsl s_0_242 s_0_235
        let s_0_245: Bits = s_0_242 << s_0_235;
        // C s_0_246: cmpl s_0_245
        let s_0_246: Bits = !s_0_245;
        // D s_0_247: and s_0_236 s_0_246
        let s_0_247: Bits = ((s_0_236) & (s_0_246));
        // D s_0_248: or s_0_247 s_0_244
        let s_0_248: Bits = ((s_0_247) | (s_0_244));
        // D s_0_249: cast reint s_0_248 -> u64
        let s_0_249: u64 = (s_0_248.value() as u64);
        // D s_0_250: write-var outdata <= s_0_249
        fn_state.outdata = s_0_249;
        // C s_0_251: const #56s : i
        let s_0_251: i128 = 56;
        // D s_0_252: read-var indata:u64
        let s_0_252: u64 = fn_state.indata;
        // D s_0_253: cast zx s_0_252 -> bv
        let s_0_253: Bits = Bits::new(s_0_252 as u128, 64u16);
        // C s_0_254: const #1s : i64
        let s_0_254: i64 = 1;
        // C s_0_255: cast zx s_0_254 -> i
        let s_0_255: i128 = (i128::try_from(s_0_254).unwrap());
        // C s_0_256: const #3s : i
        let s_0_256: i128 = 3;
        // C s_0_257: add s_0_256 s_0_255
        let s_0_257: i128 = (s_0_256 + s_0_255);
        // D s_0_258: bit-extract s_0_253 s_0_251 s_0_257
        let s_0_258: Bits = (Bits::new(
            ((s_0_253) >> (s_0_251)).value(),
            u16::try_from(s_0_257).unwrap(),
        ));
        // D s_0_259: cast reint s_0_258 -> u8
        let s_0_259: u8 = (s_0_258.value() as u8);
        // C s_0_260: const #40s : i
        let s_0_260: i128 = 40;
        // D s_0_261: cast zx s_0_249 -> bv
        let s_0_261: Bits = Bits::new(s_0_249 as u128, 64u16);
        // D s_0_262: cast zx s_0_259 -> bv
        let s_0_262: Bits = Bits::new(s_0_259 as u128, 4u16);
        // C s_0_263: const #3s : i
        let s_0_263: i128 = 3;
        // C s_0_264: const #1u : u64
        let s_0_264: u64 = 1;
        // C s_0_265: cast zx s_0_264 -> bv
        let s_0_265: Bits = Bits::new(s_0_264 as u128, 64u16);
        // C s_0_266: lsl s_0_265 s_0_263
        let s_0_266: Bits = s_0_265 << s_0_263;
        // C s_0_267: sub s_0_266 s_0_265
        let s_0_267: Bits = ((s_0_266) - (s_0_265));
        // D s_0_268: and s_0_262 s_0_267
        let s_0_268: Bits = ((s_0_262) & (s_0_267));
        // D s_0_269: lsl s_0_268 s_0_260
        let s_0_269: Bits = s_0_268 << s_0_260;
        // C s_0_270: lsl s_0_267 s_0_260
        let s_0_270: Bits = s_0_267 << s_0_260;
        // C s_0_271: cmpl s_0_270
        let s_0_271: Bits = !s_0_270;
        // D s_0_272: and s_0_261 s_0_271
        let s_0_272: Bits = ((s_0_261) & (s_0_271));
        // D s_0_273: or s_0_272 s_0_269
        let s_0_273: Bits = ((s_0_272) | (s_0_269));
        // D s_0_274: cast reint s_0_273 -> u64
        let s_0_274: u64 = (s_0_273.value() as u64);
        // D s_0_275: write-var outdata <= s_0_274
        fn_state.outdata = s_0_274;
        // C s_0_276: const #20s : i
        let s_0_276: i128 = 20;
        // D s_0_277: read-var indata:u64
        let s_0_277: u64 = fn_state.indata;
        // D s_0_278: cast zx s_0_277 -> bv
        let s_0_278: Bits = Bits::new(s_0_277 as u128, 64u16);
        // C s_0_279: const #1s : i64
        let s_0_279: i64 = 1;
        // C s_0_280: cast zx s_0_279 -> i
        let s_0_280: i128 = (i128::try_from(s_0_279).unwrap());
        // C s_0_281: const #3s : i
        let s_0_281: i128 = 3;
        // C s_0_282: add s_0_281 s_0_280
        let s_0_282: i128 = (s_0_281 + s_0_280);
        // D s_0_283: bit-extract s_0_278 s_0_276 s_0_282
        let s_0_283: Bits = (Bits::new(
            ((s_0_278) >> (s_0_276)).value(),
            u16::try_from(s_0_282).unwrap(),
        ));
        // D s_0_284: cast reint s_0_283 -> u8
        let s_0_284: u8 = (s_0_283.value() as u8);
        // C s_0_285: const #44s : i
        let s_0_285: i128 = 44;
        // D s_0_286: cast zx s_0_274 -> bv
        let s_0_286: Bits = Bits::new(s_0_274 as u128, 64u16);
        // D s_0_287: cast zx s_0_284 -> bv
        let s_0_287: Bits = Bits::new(s_0_284 as u128, 4u16);
        // C s_0_288: const #3s : i
        let s_0_288: i128 = 3;
        // C s_0_289: const #1u : u64
        let s_0_289: u64 = 1;
        // C s_0_290: cast zx s_0_289 -> bv
        let s_0_290: Bits = Bits::new(s_0_289 as u128, 64u16);
        // C s_0_291: lsl s_0_290 s_0_288
        let s_0_291: Bits = s_0_290 << s_0_288;
        // C s_0_292: sub s_0_291 s_0_290
        let s_0_292: Bits = ((s_0_291) - (s_0_290));
        // D s_0_293: and s_0_287 s_0_292
        let s_0_293: Bits = ((s_0_287) & (s_0_292));
        // D s_0_294: lsl s_0_293 s_0_285
        let s_0_294: Bits = s_0_293 << s_0_285;
        // C s_0_295: lsl s_0_292 s_0_285
        let s_0_295: Bits = s_0_292 << s_0_285;
        // C s_0_296: cmpl s_0_295
        let s_0_296: Bits = !s_0_295;
        // D s_0_297: and s_0_286 s_0_296
        let s_0_297: Bits = ((s_0_286) & (s_0_296));
        // D s_0_298: or s_0_297 s_0_294
        let s_0_298: Bits = ((s_0_297) | (s_0_294));
        // D s_0_299: cast reint s_0_298 -> u64
        let s_0_299: u64 = (s_0_298.value() as u64);
        // D s_0_300: write-var outdata <= s_0_299
        fn_state.outdata = s_0_299;
        // C s_0_301: const #8s : i
        let s_0_301: i128 = 8;
        // D s_0_302: read-var indata:u64
        let s_0_302: u64 = fn_state.indata;
        // D s_0_303: cast zx s_0_302 -> bv
        let s_0_303: Bits = Bits::new(s_0_302 as u128, 64u16);
        // C s_0_304: const #1s : i64
        let s_0_304: i64 = 1;
        // C s_0_305: cast zx s_0_304 -> i
        let s_0_305: i128 = (i128::try_from(s_0_304).unwrap());
        // C s_0_306: const #3s : i
        let s_0_306: i128 = 3;
        // C s_0_307: add s_0_306 s_0_305
        let s_0_307: i128 = (s_0_306 + s_0_305);
        // D s_0_308: bit-extract s_0_303 s_0_301 s_0_307
        let s_0_308: Bits = (Bits::new(
            ((s_0_303) >> (s_0_301)).value(),
            u16::try_from(s_0_307).unwrap(),
        ));
        // D s_0_309: cast reint s_0_308 -> u8
        let s_0_309: u8 = (s_0_308.value() as u8);
        // C s_0_310: const #48s : i
        let s_0_310: i128 = 48;
        // D s_0_311: cast zx s_0_299 -> bv
        let s_0_311: Bits = Bits::new(s_0_299 as u128, 64u16);
        // D s_0_312: cast zx s_0_309 -> bv
        let s_0_312: Bits = Bits::new(s_0_309 as u128, 4u16);
        // C s_0_313: const #3s : i
        let s_0_313: i128 = 3;
        // C s_0_314: const #1u : u64
        let s_0_314: u64 = 1;
        // C s_0_315: cast zx s_0_314 -> bv
        let s_0_315: Bits = Bits::new(s_0_314 as u128, 64u16);
        // C s_0_316: lsl s_0_315 s_0_313
        let s_0_316: Bits = s_0_315 << s_0_313;
        // C s_0_317: sub s_0_316 s_0_315
        let s_0_317: Bits = ((s_0_316) - (s_0_315));
        // D s_0_318: and s_0_312 s_0_317
        let s_0_318: Bits = ((s_0_312) & (s_0_317));
        // D s_0_319: lsl s_0_318 s_0_310
        let s_0_319: Bits = s_0_318 << s_0_310;
        // C s_0_320: lsl s_0_317 s_0_310
        let s_0_320: Bits = s_0_317 << s_0_310;
        // C s_0_321: cmpl s_0_320
        let s_0_321: Bits = !s_0_320;
        // D s_0_322: and s_0_311 s_0_321
        let s_0_322: Bits = ((s_0_311) & (s_0_321));
        // D s_0_323: or s_0_322 s_0_319
        let s_0_323: Bits = ((s_0_322) | (s_0_319));
        // D s_0_324: cast reint s_0_323 -> u64
        let s_0_324: u64 = (s_0_323.value() as u64);
        // D s_0_325: write-var outdata <= s_0_324
        fn_state.outdata = s_0_324;
        // C s_0_326: const #36s : i
        let s_0_326: i128 = 36;
        // D s_0_327: read-var indata:u64
        let s_0_327: u64 = fn_state.indata;
        // D s_0_328: cast zx s_0_327 -> bv
        let s_0_328: Bits = Bits::new(s_0_327 as u128, 64u16);
        // C s_0_329: const #1s : i64
        let s_0_329: i64 = 1;
        // C s_0_330: cast zx s_0_329 -> i
        let s_0_330: i128 = (i128::try_from(s_0_329).unwrap());
        // C s_0_331: const #3s : i
        let s_0_331: i128 = 3;
        // C s_0_332: add s_0_331 s_0_330
        let s_0_332: i128 = (s_0_331 + s_0_330);
        // D s_0_333: bit-extract s_0_328 s_0_326 s_0_332
        let s_0_333: Bits = (Bits::new(
            ((s_0_328) >> (s_0_326)).value(),
            u16::try_from(s_0_332).unwrap(),
        ));
        // D s_0_334: cast reint s_0_333 -> u8
        let s_0_334: u8 = (s_0_333.value() as u8);
        // C s_0_335: const #52s : i
        let s_0_335: i128 = 52;
        // D s_0_336: cast zx s_0_324 -> bv
        let s_0_336: Bits = Bits::new(s_0_324 as u128, 64u16);
        // D s_0_337: cast zx s_0_334 -> bv
        let s_0_337: Bits = Bits::new(s_0_334 as u128, 4u16);
        // C s_0_338: const #3s : i
        let s_0_338: i128 = 3;
        // C s_0_339: const #1u : u64
        let s_0_339: u64 = 1;
        // C s_0_340: cast zx s_0_339 -> bv
        let s_0_340: Bits = Bits::new(s_0_339 as u128, 64u16);
        // C s_0_341: lsl s_0_340 s_0_338
        let s_0_341: Bits = s_0_340 << s_0_338;
        // C s_0_342: sub s_0_341 s_0_340
        let s_0_342: Bits = ((s_0_341) - (s_0_340));
        // D s_0_343: and s_0_337 s_0_342
        let s_0_343: Bits = ((s_0_337) & (s_0_342));
        // D s_0_344: lsl s_0_343 s_0_335
        let s_0_344: Bits = s_0_343 << s_0_335;
        // C s_0_345: lsl s_0_342 s_0_335
        let s_0_345: Bits = s_0_342 << s_0_335;
        // C s_0_346: cmpl s_0_345
        let s_0_346: Bits = !s_0_345;
        // D s_0_347: and s_0_336 s_0_346
        let s_0_347: Bits = ((s_0_336) & (s_0_346));
        // D s_0_348: or s_0_347 s_0_344
        let s_0_348: Bits = ((s_0_347) | (s_0_344));
        // D s_0_349: cast reint s_0_348 -> u64
        let s_0_349: u64 = (s_0_348.value() as u64);
        // D s_0_350: write-var outdata <= s_0_349
        fn_state.outdata = s_0_349;
        // C s_0_351: const #16s : i
        let s_0_351: i128 = 16;
        // D s_0_352: read-var indata:u64
        let s_0_352: u64 = fn_state.indata;
        // D s_0_353: cast zx s_0_352 -> bv
        let s_0_353: Bits = Bits::new(s_0_352 as u128, 64u16);
        // C s_0_354: const #1s : i64
        let s_0_354: i64 = 1;
        // C s_0_355: cast zx s_0_354 -> i
        let s_0_355: i128 = (i128::try_from(s_0_354).unwrap());
        // C s_0_356: const #3s : i
        let s_0_356: i128 = 3;
        // C s_0_357: add s_0_356 s_0_355
        let s_0_357: i128 = (s_0_356 + s_0_355);
        // D s_0_358: bit-extract s_0_353 s_0_351 s_0_357
        let s_0_358: Bits = (Bits::new(
            ((s_0_353) >> (s_0_351)).value(),
            u16::try_from(s_0_357).unwrap(),
        ));
        // D s_0_359: cast reint s_0_358 -> u8
        let s_0_359: u8 = (s_0_358.value() as u8);
        // C s_0_360: const #56s : i
        let s_0_360: i128 = 56;
        // D s_0_361: cast zx s_0_349 -> bv
        let s_0_361: Bits = Bits::new(s_0_349 as u128, 64u16);
        // D s_0_362: cast zx s_0_359 -> bv
        let s_0_362: Bits = Bits::new(s_0_359 as u128, 4u16);
        // C s_0_363: const #3s : i
        let s_0_363: i128 = 3;
        // C s_0_364: const #1u : u64
        let s_0_364: u64 = 1;
        // C s_0_365: cast zx s_0_364 -> bv
        let s_0_365: Bits = Bits::new(s_0_364 as u128, 64u16);
        // C s_0_366: lsl s_0_365 s_0_363
        let s_0_366: Bits = s_0_365 << s_0_363;
        // C s_0_367: sub s_0_366 s_0_365
        let s_0_367: Bits = ((s_0_366) - (s_0_365));
        // D s_0_368: and s_0_362 s_0_367
        let s_0_368: Bits = ((s_0_362) & (s_0_367));
        // D s_0_369: lsl s_0_368 s_0_360
        let s_0_369: Bits = s_0_368 << s_0_360;
        // C s_0_370: lsl s_0_367 s_0_360
        let s_0_370: Bits = s_0_367 << s_0_360;
        // C s_0_371: cmpl s_0_370
        let s_0_371: Bits = !s_0_370;
        // D s_0_372: and s_0_361 s_0_371
        let s_0_372: Bits = ((s_0_361) & (s_0_371));
        // D s_0_373: or s_0_372 s_0_369
        let s_0_373: Bits = ((s_0_372) | (s_0_369));
        // D s_0_374: cast reint s_0_373 -> u64
        let s_0_374: u64 = (s_0_373.value() as u64);
        // D s_0_375: write-var outdata <= s_0_374
        fn_state.outdata = s_0_374;
        // C s_0_376: const #60s : i
        let s_0_376: i128 = 60;
        // D s_0_377: read-var indata:u64
        let s_0_377: u64 = fn_state.indata;
        // D s_0_378: cast zx s_0_377 -> bv
        let s_0_378: Bits = Bits::new(s_0_377 as u128, 64u16);
        // C s_0_379: const #1s : i64
        let s_0_379: i64 = 1;
        // C s_0_380: cast zx s_0_379 -> i
        let s_0_380: i128 = (i128::try_from(s_0_379).unwrap());
        // C s_0_381: const #3s : i
        let s_0_381: i128 = 3;
        // C s_0_382: add s_0_381 s_0_380
        let s_0_382: i128 = (s_0_381 + s_0_380);
        // D s_0_383: bit-extract s_0_378 s_0_376 s_0_382
        let s_0_383: Bits = (Bits::new(
            ((s_0_378) >> (s_0_376)).value(),
            u16::try_from(s_0_382).unwrap(),
        ));
        // D s_0_384: cast reint s_0_383 -> u8
        let s_0_384: u8 = (s_0_383.value() as u8);
        // C s_0_385: const #60s : i
        let s_0_385: i128 = 60;
        // D s_0_386: cast zx s_0_374 -> bv
        let s_0_386: Bits = Bits::new(s_0_374 as u128, 64u16);
        // D s_0_387: cast zx s_0_384 -> bv
        let s_0_387: Bits = Bits::new(s_0_384 as u128, 4u16);
        // C s_0_388: const #3s : i
        let s_0_388: i128 = 3;
        // C s_0_389: const #1u : u64
        let s_0_389: u64 = 1;
        // C s_0_390: cast zx s_0_389 -> bv
        let s_0_390: Bits = Bits::new(s_0_389 as u128, 64u16);
        // C s_0_391: lsl s_0_390 s_0_388
        let s_0_391: Bits = s_0_390 << s_0_388;
        // C s_0_392: sub s_0_391 s_0_390
        let s_0_392: Bits = ((s_0_391) - (s_0_390));
        // D s_0_393: and s_0_387 s_0_392
        let s_0_393: Bits = ((s_0_387) & (s_0_392));
        // D s_0_394: lsl s_0_393 s_0_385
        let s_0_394: Bits = s_0_393 << s_0_385;
        // C s_0_395: lsl s_0_392 s_0_385
        let s_0_395: Bits = s_0_392 << s_0_385;
        // C s_0_396: cmpl s_0_395
        let s_0_396: Bits = !s_0_395;
        // D s_0_397: and s_0_386 s_0_396
        let s_0_397: Bits = ((s_0_386) & (s_0_396));
        // D s_0_398: or s_0_397 s_0_394
        let s_0_398: Bits = ((s_0_397) | (s_0_394));
        // D s_0_399: cast reint s_0_398 -> u64
        let s_0_399: u64 = (s_0_398.value() as u64);
        // D s_0_400: write-var outdata <= s_0_399
        fn_state.outdata = s_0_399;
        // N s_0_401: return s_0_399
        return s_0_399;
    }
}
