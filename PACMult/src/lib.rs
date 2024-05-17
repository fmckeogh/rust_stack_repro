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
use RotCell::*;
use vector_update_subrange_from_subrange::*;
use common::*;
pub fn PACMult<T: Tracer>(state: &mut State, tracer: &T, Sinput: u64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        t2: u8,
        Soutput: u64,
        t1: u8,
        i: i64,
        t0: u8,
        t3: u8,
        Sinput: u64,
    }
    let fn_state = FunctionState {
        Sinput,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #3s : i64
        let s_1_1: i64 = 3;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #8s : i
        let s_2_0: i128 = 8;
        // D s_2_1: read-var i:i64
        let s_2_1: i64 = fn_state.i;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: add s_2_2 s_2_0
        let s_2_3: i128 = (s_2_2 + s_2_0);
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // C s_2_5: const #4s : i
        let s_2_5: i128 = 4;
        // D s_2_6: cast zx s_2_4 -> i
        let s_2_6: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_7: mul s_2_5 s_2_6
        let s_2_7: i128 = ((s_2_5) * (s_2_6));
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // C s_2_9: const #4s : i
        let s_2_9: i128 = 4;
        // D s_2_10: read-var Sinput:u64
        let s_2_10: u64 = fn_state.Sinput;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 64u16);
        // D s_2_12: cast zx s_2_8 -> i
        let s_2_12: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_13: bit-extract s_2_11 s_2_12 s_2_9
        let s_2_13: Bits = (Bits::new(
            ((s_2_11) >> (s_2_12)).value(),
            u16::try_from(s_2_9).unwrap(),
        ));
        // D s_2_14: cast reint s_2_13 -> u8
        let s_2_14: u8 = (s_2_13.value() as u8);
        // C s_2_15: const #1s : i64
        let s_2_15: i64 = 1;
        // D s_2_16: call RotCell(s_2_14, s_2_15)
        let s_2_16: u8 = RotCell(state, tracer, s_2_14, s_2_15);
        // C s_2_17: const #4s : i
        let s_2_17: i128 = 4;
        // D s_2_18: read-var i:i64
        let s_2_18: i64 = fn_state.i;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: add s_2_19 s_2_17
        let s_2_20: i128 = (s_2_19 + s_2_17);
        // D s_2_21: cast reint s_2_20 -> i64
        let s_2_21: i64 = (s_2_20 as i64);
        // C s_2_22: const #4s : i
        let s_2_22: i128 = 4;
        // D s_2_23: cast zx s_2_21 -> i
        let s_2_23: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_24: mul s_2_22 s_2_23
        let s_2_24: i128 = ((s_2_22) * (s_2_23));
        // D s_2_25: cast reint s_2_24 -> i64
        let s_2_25: i64 = (s_2_24 as i64);
        // C s_2_26: const #4s : i
        let s_2_26: i128 = 4;
        // D s_2_27: read-var Sinput:u64
        let s_2_27: u64 = fn_state.Sinput;
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 64u16);
        // D s_2_29: cast zx s_2_25 -> i
        let s_2_29: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_30: bit-extract s_2_28 s_2_29 s_2_26
        let s_2_30: Bits = (Bits::new(
            ((s_2_28) >> (s_2_29)).value(),
            u16::try_from(s_2_26).unwrap(),
        ));
        // D s_2_31: cast reint s_2_30 -> u8
        let s_2_31: u8 = (s_2_30.value() as u8);
        // C s_2_32: const #2s : i64
        let s_2_32: i64 = 2;
        // D s_2_33: call RotCell(s_2_31, s_2_32)
        let s_2_33: u8 = RotCell(state, tracer, s_2_31, s_2_32);
        // D s_2_34: cast zx s_2_16 -> bv
        let s_2_34: Bits = Bits::new(s_2_16 as u128, 4u16);
        // D s_2_35: cast zx s_2_33 -> bv
        let s_2_35: Bits = Bits::new(s_2_33 as u128, 4u16);
        // D s_2_36: xor s_2_34 s_2_35
        let s_2_36: Bits = ((s_2_34) ^ (s_2_35));
        // D s_2_37: cast reint s_2_36 -> u8
        let s_2_37: u8 = (s_2_36.value() as u8);
        // C s_2_38: const #0s : i
        let s_2_38: i128 = 0;
        // D s_2_39: read-var t0:u8
        let s_2_39: u8 = fn_state.t0;
        // D s_2_40: cast zx s_2_39 -> bv
        let s_2_40: Bits = Bits::new(s_2_39 as u128, 4u16);
        // D s_2_41: cast zx s_2_37 -> bv
        let s_2_41: Bits = Bits::new(s_2_37 as u128, 4u16);
        // C s_2_42: const #3s : i
        let s_2_42: i128 = 3;
        // C s_2_43: const #1u : u64
        let s_2_43: u64 = 1;
        // C s_2_44: cast zx s_2_43 -> bv
        let s_2_44: Bits = Bits::new(s_2_43 as u128, 64u16);
        // C s_2_45: lsl s_2_44 s_2_42
        let s_2_45: Bits = s_2_44 << s_2_42;
        // C s_2_46: sub s_2_45 s_2_44
        let s_2_46: Bits = ((s_2_45) - (s_2_44));
        // D s_2_47: and s_2_41 s_2_46
        let s_2_47: Bits = ((s_2_41) & (s_2_46));
        // D s_2_48: lsl s_2_47 s_2_38
        let s_2_48: Bits = s_2_47 << s_2_38;
        // C s_2_49: lsl s_2_46 s_2_38
        let s_2_49: Bits = s_2_46 << s_2_38;
        // C s_2_50: cmpl s_2_49
        let s_2_50: Bits = !s_2_49;
        // D s_2_51: and s_2_40 s_2_50
        let s_2_51: Bits = ((s_2_40) & (s_2_50));
        // D s_2_52: or s_2_51 s_2_48
        let s_2_52: Bits = ((s_2_51) | (s_2_48));
        // D s_2_53: cast reint s_2_52 -> u8
        let s_2_53: u8 = (s_2_52.value() as u8);
        // D s_2_54: write-var t0 <= s_2_53
        fn_state.t0 = s_2_53;
        // C s_2_55: const #0s : i
        let s_2_55: i128 = 0;
        // D s_2_56: cast zx s_2_53 -> bv
        let s_2_56: Bits = Bits::new(s_2_53 as u128, 4u16);
        // C s_2_57: const #1s : i64
        let s_2_57: i64 = 1;
        // C s_2_58: cast zx s_2_57 -> i
        let s_2_58: i128 = (i128::try_from(s_2_57).unwrap());
        // C s_2_59: const #3s : i
        let s_2_59: i128 = 3;
        // C s_2_60: add s_2_59 s_2_58
        let s_2_60: i128 = (s_2_59 + s_2_58);
        // D s_2_61: bit-extract s_2_56 s_2_55 s_2_60
        let s_2_61: Bits = (Bits::new(
            ((s_2_56) >> (s_2_55)).value(),
            u16::try_from(s_2_60).unwrap(),
        ));
        // D s_2_62: cast reint s_2_61 -> u8
        let s_2_62: u8 = (s_2_61.value() as u8);
        // C s_2_63: const #4s : i
        let s_2_63: i128 = 4;
        // D s_2_64: read-var i:i64
        let s_2_64: i64 = fn_state.i;
        // D s_2_65: cast zx s_2_64 -> i
        let s_2_65: i128 = (i128::try_from(s_2_64).unwrap());
        // D s_2_66: mul s_2_63 s_2_65
        let s_2_66: i128 = ((s_2_63) * (s_2_65));
        // D s_2_67: cast reint s_2_66 -> i64
        let s_2_67: i64 = (s_2_66 as i64);
        // C s_2_68: const #4s : i
        let s_2_68: i128 = 4;
        // D s_2_69: read-var Sinput:u64
        let s_2_69: u64 = fn_state.Sinput;
        // D s_2_70: cast zx s_2_69 -> bv
        let s_2_70: Bits = Bits::new(s_2_69 as u128, 64u16);
        // D s_2_71: cast zx s_2_67 -> i
        let s_2_71: i128 = (i128::try_from(s_2_67).unwrap());
        // D s_2_72: bit-extract s_2_70 s_2_71 s_2_68
        let s_2_72: Bits = (Bits::new(
            ((s_2_70) >> (s_2_71)).value(),
            u16::try_from(s_2_68).unwrap(),
        ));
        // D s_2_73: cast reint s_2_72 -> u8
        let s_2_73: u8 = (s_2_72.value() as u8);
        // C s_2_74: const #1s : i64
        let s_2_74: i64 = 1;
        // D s_2_75: call RotCell(s_2_73, s_2_74)
        let s_2_75: u8 = RotCell(state, tracer, s_2_73, s_2_74);
        // D s_2_76: cast zx s_2_62 -> bv
        let s_2_76: Bits = Bits::new(s_2_62 as u128, 4u16);
        // D s_2_77: cast zx s_2_75 -> bv
        let s_2_77: Bits = Bits::new(s_2_75 as u128, 4u16);
        // D s_2_78: xor s_2_76 s_2_77
        let s_2_78: Bits = ((s_2_76) ^ (s_2_77));
        // D s_2_79: cast reint s_2_78 -> u8
        let s_2_79: u8 = (s_2_78.value() as u8);
        // C s_2_80: const #0s : i
        let s_2_80: i128 = 0;
        // D s_2_81: cast zx s_2_53 -> bv
        let s_2_81: Bits = Bits::new(s_2_53 as u128, 4u16);
        // D s_2_82: cast zx s_2_79 -> bv
        let s_2_82: Bits = Bits::new(s_2_79 as u128, 4u16);
        // C s_2_83: const #3s : i
        let s_2_83: i128 = 3;
        // C s_2_84: const #1u : u64
        let s_2_84: u64 = 1;
        // C s_2_85: cast zx s_2_84 -> bv
        let s_2_85: Bits = Bits::new(s_2_84 as u128, 64u16);
        // C s_2_86: lsl s_2_85 s_2_83
        let s_2_86: Bits = s_2_85 << s_2_83;
        // C s_2_87: sub s_2_86 s_2_85
        let s_2_87: Bits = ((s_2_86) - (s_2_85));
        // D s_2_88: and s_2_82 s_2_87
        let s_2_88: Bits = ((s_2_82) & (s_2_87));
        // D s_2_89: lsl s_2_88 s_2_80
        let s_2_89: Bits = s_2_88 << s_2_80;
        // C s_2_90: lsl s_2_87 s_2_80
        let s_2_90: Bits = s_2_87 << s_2_80;
        // C s_2_91: cmpl s_2_90
        let s_2_91: Bits = !s_2_90;
        // D s_2_92: and s_2_81 s_2_91
        let s_2_92: Bits = ((s_2_81) & (s_2_91));
        // D s_2_93: or s_2_92 s_2_89
        let s_2_93: Bits = ((s_2_92) | (s_2_89));
        // D s_2_94: cast reint s_2_93 -> u8
        let s_2_94: u8 = (s_2_93.value() as u8);
        // D s_2_95: write-var t0 <= s_2_94
        fn_state.t0 = s_2_94;
        // C s_2_96: const #12s : i
        let s_2_96: i128 = 12;
        // D s_2_97: read-var i:i64
        let s_2_97: i64 = fn_state.i;
        // D s_2_98: cast zx s_2_97 -> i
        let s_2_98: i128 = (i128::try_from(s_2_97).unwrap());
        // D s_2_99: add s_2_98 s_2_96
        let s_2_99: i128 = (s_2_98 + s_2_96);
        // D s_2_100: cast reint s_2_99 -> i64
        let s_2_100: i64 = (s_2_99 as i64);
        // C s_2_101: const #4s : i
        let s_2_101: i128 = 4;
        // D s_2_102: cast zx s_2_100 -> i
        let s_2_102: i128 = (i128::try_from(s_2_100).unwrap());
        // D s_2_103: mul s_2_101 s_2_102
        let s_2_103: i128 = ((s_2_101) * (s_2_102));
        // D s_2_104: cast reint s_2_103 -> i64
        let s_2_104: i64 = (s_2_103 as i64);
        // C s_2_105: const #4s : i
        let s_2_105: i128 = 4;
        // D s_2_106: read-var Sinput:u64
        let s_2_106: u64 = fn_state.Sinput;
        // D s_2_107: cast zx s_2_106 -> bv
        let s_2_107: Bits = Bits::new(s_2_106 as u128, 64u16);
        // D s_2_108: cast zx s_2_104 -> i
        let s_2_108: i128 = (i128::try_from(s_2_104).unwrap());
        // D s_2_109: bit-extract s_2_107 s_2_108 s_2_105
        let s_2_109: Bits = (Bits::new(
            ((s_2_107) >> (s_2_108)).value(),
            u16::try_from(s_2_105).unwrap(),
        ));
        // D s_2_110: cast reint s_2_109 -> u8
        let s_2_110: u8 = (s_2_109.value() as u8);
        // C s_2_111: const #1s : i64
        let s_2_111: i64 = 1;
        // D s_2_112: call RotCell(s_2_110, s_2_111)
        let s_2_112: u8 = RotCell(state, tracer, s_2_110, s_2_111);
        // C s_2_113: const #4s : i
        let s_2_113: i128 = 4;
        // D s_2_114: read-var i:i64
        let s_2_114: i64 = fn_state.i;
        // D s_2_115: cast zx s_2_114 -> i
        let s_2_115: i128 = (i128::try_from(s_2_114).unwrap());
        // D s_2_116: add s_2_115 s_2_113
        let s_2_116: i128 = (s_2_115 + s_2_113);
        // D s_2_117: cast reint s_2_116 -> i64
        let s_2_117: i64 = (s_2_116 as i64);
        // C s_2_118: const #4s : i
        let s_2_118: i128 = 4;
        // D s_2_119: cast zx s_2_117 -> i
        let s_2_119: i128 = (i128::try_from(s_2_117).unwrap());
        // D s_2_120: mul s_2_118 s_2_119
        let s_2_120: i128 = ((s_2_118) * (s_2_119));
        // D s_2_121: cast reint s_2_120 -> i64
        let s_2_121: i64 = (s_2_120 as i64);
        // C s_2_122: const #4s : i
        let s_2_122: i128 = 4;
        // D s_2_123: read-var Sinput:u64
        let s_2_123: u64 = fn_state.Sinput;
        // D s_2_124: cast zx s_2_123 -> bv
        let s_2_124: Bits = Bits::new(s_2_123 as u128, 64u16);
        // D s_2_125: cast zx s_2_121 -> i
        let s_2_125: i128 = (i128::try_from(s_2_121).unwrap());
        // D s_2_126: bit-extract s_2_124 s_2_125 s_2_122
        let s_2_126: Bits = (Bits::new(
            ((s_2_124) >> (s_2_125)).value(),
            u16::try_from(s_2_122).unwrap(),
        ));
        // D s_2_127: cast reint s_2_126 -> u8
        let s_2_127: u8 = (s_2_126.value() as u8);
        // C s_2_128: const #1s : i64
        let s_2_128: i64 = 1;
        // D s_2_129: call RotCell(s_2_127, s_2_128)
        let s_2_129: u8 = RotCell(state, tracer, s_2_127, s_2_128);
        // D s_2_130: cast zx s_2_112 -> bv
        let s_2_130: Bits = Bits::new(s_2_112 as u128, 4u16);
        // D s_2_131: cast zx s_2_129 -> bv
        let s_2_131: Bits = Bits::new(s_2_129 as u128, 4u16);
        // D s_2_132: xor s_2_130 s_2_131
        let s_2_132: Bits = ((s_2_130) ^ (s_2_131));
        // D s_2_133: cast reint s_2_132 -> u8
        let s_2_133: u8 = (s_2_132.value() as u8);
        // C s_2_134: const #0s : i
        let s_2_134: i128 = 0;
        // D s_2_135: read-var t1:u8
        let s_2_135: u8 = fn_state.t1;
        // D s_2_136: cast zx s_2_135 -> bv
        let s_2_136: Bits = Bits::new(s_2_135 as u128, 4u16);
        // D s_2_137: cast zx s_2_133 -> bv
        let s_2_137: Bits = Bits::new(s_2_133 as u128, 4u16);
        // C s_2_138: const #3s : i
        let s_2_138: i128 = 3;
        // C s_2_139: const #1u : u64
        let s_2_139: u64 = 1;
        // C s_2_140: cast zx s_2_139 -> bv
        let s_2_140: Bits = Bits::new(s_2_139 as u128, 64u16);
        // C s_2_141: lsl s_2_140 s_2_138
        let s_2_141: Bits = s_2_140 << s_2_138;
        // C s_2_142: sub s_2_141 s_2_140
        let s_2_142: Bits = ((s_2_141) - (s_2_140));
        // D s_2_143: and s_2_137 s_2_142
        let s_2_143: Bits = ((s_2_137) & (s_2_142));
        // D s_2_144: lsl s_2_143 s_2_134
        let s_2_144: Bits = s_2_143 << s_2_134;
        // C s_2_145: lsl s_2_142 s_2_134
        let s_2_145: Bits = s_2_142 << s_2_134;
        // C s_2_146: cmpl s_2_145
        let s_2_146: Bits = !s_2_145;
        // D s_2_147: and s_2_136 s_2_146
        let s_2_147: Bits = ((s_2_136) & (s_2_146));
        // D s_2_148: or s_2_147 s_2_144
        let s_2_148: Bits = ((s_2_147) | (s_2_144));
        // D s_2_149: cast reint s_2_148 -> u8
        let s_2_149: u8 = (s_2_148.value() as u8);
        // D s_2_150: write-var t1 <= s_2_149
        fn_state.t1 = s_2_149;
        // C s_2_151: const #0s : i
        let s_2_151: i128 = 0;
        // D s_2_152: cast zx s_2_149 -> bv
        let s_2_152: Bits = Bits::new(s_2_149 as u128, 4u16);
        // C s_2_153: const #1s : i64
        let s_2_153: i64 = 1;
        // C s_2_154: cast zx s_2_153 -> i
        let s_2_154: i128 = (i128::try_from(s_2_153).unwrap());
        // C s_2_155: const #3s : i
        let s_2_155: i128 = 3;
        // C s_2_156: add s_2_155 s_2_154
        let s_2_156: i128 = (s_2_155 + s_2_154);
        // D s_2_157: bit-extract s_2_152 s_2_151 s_2_156
        let s_2_157: Bits = (Bits::new(
            ((s_2_152) >> (s_2_151)).value(),
            u16::try_from(s_2_156).unwrap(),
        ));
        // D s_2_158: cast reint s_2_157 -> u8
        let s_2_158: u8 = (s_2_157.value() as u8);
        // C s_2_159: const #4s : i
        let s_2_159: i128 = 4;
        // D s_2_160: read-var i:i64
        let s_2_160: i64 = fn_state.i;
        // D s_2_161: cast zx s_2_160 -> i
        let s_2_161: i128 = (i128::try_from(s_2_160).unwrap());
        // D s_2_162: mul s_2_159 s_2_161
        let s_2_162: i128 = ((s_2_159) * (s_2_161));
        // D s_2_163: cast reint s_2_162 -> i64
        let s_2_163: i64 = (s_2_162 as i64);
        // C s_2_164: const #4s : i
        let s_2_164: i128 = 4;
        // D s_2_165: read-var Sinput:u64
        let s_2_165: u64 = fn_state.Sinput;
        // D s_2_166: cast zx s_2_165 -> bv
        let s_2_166: Bits = Bits::new(s_2_165 as u128, 64u16);
        // D s_2_167: cast zx s_2_163 -> i
        let s_2_167: i128 = (i128::try_from(s_2_163).unwrap());
        // D s_2_168: bit-extract s_2_166 s_2_167 s_2_164
        let s_2_168: Bits = (Bits::new(
            ((s_2_166) >> (s_2_167)).value(),
            u16::try_from(s_2_164).unwrap(),
        ));
        // D s_2_169: cast reint s_2_168 -> u8
        let s_2_169: u8 = (s_2_168.value() as u8);
        // C s_2_170: const #2s : i64
        let s_2_170: i64 = 2;
        // D s_2_171: call RotCell(s_2_169, s_2_170)
        let s_2_171: u8 = RotCell(state, tracer, s_2_169, s_2_170);
        // D s_2_172: cast zx s_2_158 -> bv
        let s_2_172: Bits = Bits::new(s_2_158 as u128, 4u16);
        // D s_2_173: cast zx s_2_171 -> bv
        let s_2_173: Bits = Bits::new(s_2_171 as u128, 4u16);
        // D s_2_174: xor s_2_172 s_2_173
        let s_2_174: Bits = ((s_2_172) ^ (s_2_173));
        // D s_2_175: cast reint s_2_174 -> u8
        let s_2_175: u8 = (s_2_174.value() as u8);
        // C s_2_176: const #0s : i
        let s_2_176: i128 = 0;
        // D s_2_177: cast zx s_2_149 -> bv
        let s_2_177: Bits = Bits::new(s_2_149 as u128, 4u16);
        // D s_2_178: cast zx s_2_175 -> bv
        let s_2_178: Bits = Bits::new(s_2_175 as u128, 4u16);
        // C s_2_179: const #3s : i
        let s_2_179: i128 = 3;
        // C s_2_180: const #1u : u64
        let s_2_180: u64 = 1;
        // C s_2_181: cast zx s_2_180 -> bv
        let s_2_181: Bits = Bits::new(s_2_180 as u128, 64u16);
        // C s_2_182: lsl s_2_181 s_2_179
        let s_2_182: Bits = s_2_181 << s_2_179;
        // C s_2_183: sub s_2_182 s_2_181
        let s_2_183: Bits = ((s_2_182) - (s_2_181));
        // D s_2_184: and s_2_178 s_2_183
        let s_2_184: Bits = ((s_2_178) & (s_2_183));
        // D s_2_185: lsl s_2_184 s_2_176
        let s_2_185: Bits = s_2_184 << s_2_176;
        // C s_2_186: lsl s_2_183 s_2_176
        let s_2_186: Bits = s_2_183 << s_2_176;
        // C s_2_187: cmpl s_2_186
        let s_2_187: Bits = !s_2_186;
        // D s_2_188: and s_2_177 s_2_187
        let s_2_188: Bits = ((s_2_177) & (s_2_187));
        // D s_2_189: or s_2_188 s_2_185
        let s_2_189: Bits = ((s_2_188) | (s_2_185));
        // D s_2_190: cast reint s_2_189 -> u8
        let s_2_190: u8 = (s_2_189.value() as u8);
        // D s_2_191: write-var t1 <= s_2_190
        fn_state.t1 = s_2_190;
        // C s_2_192: const #12s : i
        let s_2_192: i128 = 12;
        // D s_2_193: read-var i:i64
        let s_2_193: i64 = fn_state.i;
        // D s_2_194: cast zx s_2_193 -> i
        let s_2_194: i128 = (i128::try_from(s_2_193).unwrap());
        // D s_2_195: add s_2_194 s_2_192
        let s_2_195: i128 = (s_2_194 + s_2_192);
        // D s_2_196: cast reint s_2_195 -> i64
        let s_2_196: i64 = (s_2_195 as i64);
        // C s_2_197: const #4s : i
        let s_2_197: i128 = 4;
        // D s_2_198: cast zx s_2_196 -> i
        let s_2_198: i128 = (i128::try_from(s_2_196).unwrap());
        // D s_2_199: mul s_2_197 s_2_198
        let s_2_199: i128 = ((s_2_197) * (s_2_198));
        // D s_2_200: cast reint s_2_199 -> i64
        let s_2_200: i64 = (s_2_199 as i64);
        // C s_2_201: const #4s : i
        let s_2_201: i128 = 4;
        // D s_2_202: read-var Sinput:u64
        let s_2_202: u64 = fn_state.Sinput;
        // D s_2_203: cast zx s_2_202 -> bv
        let s_2_203: Bits = Bits::new(s_2_202 as u128, 64u16);
        // D s_2_204: cast zx s_2_200 -> i
        let s_2_204: i128 = (i128::try_from(s_2_200).unwrap());
        // D s_2_205: bit-extract s_2_203 s_2_204 s_2_201
        let s_2_205: Bits = (Bits::new(
            ((s_2_203) >> (s_2_204)).value(),
            u16::try_from(s_2_201).unwrap(),
        ));
        // D s_2_206: cast reint s_2_205 -> u8
        let s_2_206: u8 = (s_2_205.value() as u8);
        // C s_2_207: const #2s : i64
        let s_2_207: i64 = 2;
        // D s_2_208: call RotCell(s_2_206, s_2_207)
        let s_2_208: u8 = RotCell(state, tracer, s_2_206, s_2_207);
        // C s_2_209: const #8s : i
        let s_2_209: i128 = 8;
        // D s_2_210: read-var i:i64
        let s_2_210: i64 = fn_state.i;
        // D s_2_211: cast zx s_2_210 -> i
        let s_2_211: i128 = (i128::try_from(s_2_210).unwrap());
        // D s_2_212: add s_2_211 s_2_209
        let s_2_212: i128 = (s_2_211 + s_2_209);
        // D s_2_213: cast reint s_2_212 -> i64
        let s_2_213: i64 = (s_2_212 as i64);
        // C s_2_214: const #4s : i
        let s_2_214: i128 = 4;
        // D s_2_215: cast zx s_2_213 -> i
        let s_2_215: i128 = (i128::try_from(s_2_213).unwrap());
        // D s_2_216: mul s_2_214 s_2_215
        let s_2_216: i128 = ((s_2_214) * (s_2_215));
        // D s_2_217: cast reint s_2_216 -> i64
        let s_2_217: i64 = (s_2_216 as i64);
        // C s_2_218: const #4s : i
        let s_2_218: i128 = 4;
        // D s_2_219: read-var Sinput:u64
        let s_2_219: u64 = fn_state.Sinput;
        // D s_2_220: cast zx s_2_219 -> bv
        let s_2_220: Bits = Bits::new(s_2_219 as u128, 64u16);
        // D s_2_221: cast zx s_2_217 -> i
        let s_2_221: i128 = (i128::try_from(s_2_217).unwrap());
        // D s_2_222: bit-extract s_2_220 s_2_221 s_2_218
        let s_2_222: Bits = (Bits::new(
            ((s_2_220) >> (s_2_221)).value(),
            u16::try_from(s_2_218).unwrap(),
        ));
        // D s_2_223: cast reint s_2_222 -> u8
        let s_2_223: u8 = (s_2_222.value() as u8);
        // C s_2_224: const #1s : i64
        let s_2_224: i64 = 1;
        // D s_2_225: call RotCell(s_2_223, s_2_224)
        let s_2_225: u8 = RotCell(state, tracer, s_2_223, s_2_224);
        // D s_2_226: cast zx s_2_208 -> bv
        let s_2_226: Bits = Bits::new(s_2_208 as u128, 4u16);
        // D s_2_227: cast zx s_2_225 -> bv
        let s_2_227: Bits = Bits::new(s_2_225 as u128, 4u16);
        // D s_2_228: xor s_2_226 s_2_227
        let s_2_228: Bits = ((s_2_226) ^ (s_2_227));
        // D s_2_229: cast reint s_2_228 -> u8
        let s_2_229: u8 = (s_2_228.value() as u8);
        // C s_2_230: const #0s : i
        let s_2_230: i128 = 0;
        // D s_2_231: read-var t2:u8
        let s_2_231: u8 = fn_state.t2;
        // D s_2_232: cast zx s_2_231 -> bv
        let s_2_232: Bits = Bits::new(s_2_231 as u128, 4u16);
        // D s_2_233: cast zx s_2_229 -> bv
        let s_2_233: Bits = Bits::new(s_2_229 as u128, 4u16);
        // C s_2_234: const #3s : i
        let s_2_234: i128 = 3;
        // C s_2_235: const #1u : u64
        let s_2_235: u64 = 1;
        // C s_2_236: cast zx s_2_235 -> bv
        let s_2_236: Bits = Bits::new(s_2_235 as u128, 64u16);
        // C s_2_237: lsl s_2_236 s_2_234
        let s_2_237: Bits = s_2_236 << s_2_234;
        // C s_2_238: sub s_2_237 s_2_236
        let s_2_238: Bits = ((s_2_237) - (s_2_236));
        // D s_2_239: and s_2_233 s_2_238
        let s_2_239: Bits = ((s_2_233) & (s_2_238));
        // D s_2_240: lsl s_2_239 s_2_230
        let s_2_240: Bits = s_2_239 << s_2_230;
        // C s_2_241: lsl s_2_238 s_2_230
        let s_2_241: Bits = s_2_238 << s_2_230;
        // C s_2_242: cmpl s_2_241
        let s_2_242: Bits = !s_2_241;
        // D s_2_243: and s_2_232 s_2_242
        let s_2_243: Bits = ((s_2_232) & (s_2_242));
        // D s_2_244: or s_2_243 s_2_240
        let s_2_244: Bits = ((s_2_243) | (s_2_240));
        // D s_2_245: cast reint s_2_244 -> u8
        let s_2_245: u8 = (s_2_244.value() as u8);
        // D s_2_246: write-var t2 <= s_2_245
        fn_state.t2 = s_2_245;
        // C s_2_247: const #0s : i
        let s_2_247: i128 = 0;
        // D s_2_248: cast zx s_2_245 -> bv
        let s_2_248: Bits = Bits::new(s_2_245 as u128, 4u16);
        // C s_2_249: const #1s : i64
        let s_2_249: i64 = 1;
        // C s_2_250: cast zx s_2_249 -> i
        let s_2_250: i128 = (i128::try_from(s_2_249).unwrap());
        // C s_2_251: const #3s : i
        let s_2_251: i128 = 3;
        // C s_2_252: add s_2_251 s_2_250
        let s_2_252: i128 = (s_2_251 + s_2_250);
        // D s_2_253: bit-extract s_2_248 s_2_247 s_2_252
        let s_2_253: Bits = (Bits::new(
            ((s_2_248) >> (s_2_247)).value(),
            u16::try_from(s_2_252).unwrap(),
        ));
        // D s_2_254: cast reint s_2_253 -> u8
        let s_2_254: u8 = (s_2_253.value() as u8);
        // C s_2_255: const #4s : i
        let s_2_255: i128 = 4;
        // D s_2_256: read-var i:i64
        let s_2_256: i64 = fn_state.i;
        // D s_2_257: cast zx s_2_256 -> i
        let s_2_257: i128 = (i128::try_from(s_2_256).unwrap());
        // D s_2_258: mul s_2_255 s_2_257
        let s_2_258: i128 = ((s_2_255) * (s_2_257));
        // D s_2_259: cast reint s_2_258 -> i64
        let s_2_259: i64 = (s_2_258 as i64);
        // C s_2_260: const #4s : i
        let s_2_260: i128 = 4;
        // D s_2_261: read-var Sinput:u64
        let s_2_261: u64 = fn_state.Sinput;
        // D s_2_262: cast zx s_2_261 -> bv
        let s_2_262: Bits = Bits::new(s_2_261 as u128, 64u16);
        // D s_2_263: cast zx s_2_259 -> i
        let s_2_263: i128 = (i128::try_from(s_2_259).unwrap());
        // D s_2_264: bit-extract s_2_262 s_2_263 s_2_260
        let s_2_264: Bits = (Bits::new(
            ((s_2_262) >> (s_2_263)).value(),
            u16::try_from(s_2_260).unwrap(),
        ));
        // D s_2_265: cast reint s_2_264 -> u8
        let s_2_265: u8 = (s_2_264.value() as u8);
        // C s_2_266: const #1s : i64
        let s_2_266: i64 = 1;
        // D s_2_267: call RotCell(s_2_265, s_2_266)
        let s_2_267: u8 = RotCell(state, tracer, s_2_265, s_2_266);
        // D s_2_268: cast zx s_2_254 -> bv
        let s_2_268: Bits = Bits::new(s_2_254 as u128, 4u16);
        // D s_2_269: cast zx s_2_267 -> bv
        let s_2_269: Bits = Bits::new(s_2_267 as u128, 4u16);
        // D s_2_270: xor s_2_268 s_2_269
        let s_2_270: Bits = ((s_2_268) ^ (s_2_269));
        // D s_2_271: cast reint s_2_270 -> u8
        let s_2_271: u8 = (s_2_270.value() as u8);
        // C s_2_272: const #0s : i
        let s_2_272: i128 = 0;
        // D s_2_273: cast zx s_2_245 -> bv
        let s_2_273: Bits = Bits::new(s_2_245 as u128, 4u16);
        // D s_2_274: cast zx s_2_271 -> bv
        let s_2_274: Bits = Bits::new(s_2_271 as u128, 4u16);
        // C s_2_275: const #3s : i
        let s_2_275: i128 = 3;
        // C s_2_276: const #1u : u64
        let s_2_276: u64 = 1;
        // C s_2_277: cast zx s_2_276 -> bv
        let s_2_277: Bits = Bits::new(s_2_276 as u128, 64u16);
        // C s_2_278: lsl s_2_277 s_2_275
        let s_2_278: Bits = s_2_277 << s_2_275;
        // C s_2_279: sub s_2_278 s_2_277
        let s_2_279: Bits = ((s_2_278) - (s_2_277));
        // D s_2_280: and s_2_274 s_2_279
        let s_2_280: Bits = ((s_2_274) & (s_2_279));
        // D s_2_281: lsl s_2_280 s_2_272
        let s_2_281: Bits = s_2_280 << s_2_272;
        // C s_2_282: lsl s_2_279 s_2_272
        let s_2_282: Bits = s_2_279 << s_2_272;
        // C s_2_283: cmpl s_2_282
        let s_2_283: Bits = !s_2_282;
        // D s_2_284: and s_2_273 s_2_283
        let s_2_284: Bits = ((s_2_273) & (s_2_283));
        // D s_2_285: or s_2_284 s_2_281
        let s_2_285: Bits = ((s_2_284) | (s_2_281));
        // D s_2_286: cast reint s_2_285 -> u8
        let s_2_286: u8 = (s_2_285.value() as u8);
        // D s_2_287: write-var t2 <= s_2_286
        fn_state.t2 = s_2_286;
        // C s_2_288: const #12s : i
        let s_2_288: i128 = 12;
        // D s_2_289: read-var i:i64
        let s_2_289: i64 = fn_state.i;
        // D s_2_290: cast zx s_2_289 -> i
        let s_2_290: i128 = (i128::try_from(s_2_289).unwrap());
        // D s_2_291: add s_2_290 s_2_288
        let s_2_291: i128 = (s_2_290 + s_2_288);
        // D s_2_292: cast reint s_2_291 -> i64
        let s_2_292: i64 = (s_2_291 as i64);
        // C s_2_293: const #4s : i
        let s_2_293: i128 = 4;
        // D s_2_294: cast zx s_2_292 -> i
        let s_2_294: i128 = (i128::try_from(s_2_292).unwrap());
        // D s_2_295: mul s_2_293 s_2_294
        let s_2_295: i128 = ((s_2_293) * (s_2_294));
        // D s_2_296: cast reint s_2_295 -> i64
        let s_2_296: i64 = (s_2_295 as i64);
        // C s_2_297: const #4s : i
        let s_2_297: i128 = 4;
        // D s_2_298: read-var Sinput:u64
        let s_2_298: u64 = fn_state.Sinput;
        // D s_2_299: cast zx s_2_298 -> bv
        let s_2_299: Bits = Bits::new(s_2_298 as u128, 64u16);
        // D s_2_300: cast zx s_2_296 -> i
        let s_2_300: i128 = (i128::try_from(s_2_296).unwrap());
        // D s_2_301: bit-extract s_2_299 s_2_300 s_2_297
        let s_2_301: Bits = (Bits::new(
            ((s_2_299) >> (s_2_300)).value(),
            u16::try_from(s_2_297).unwrap(),
        ));
        // D s_2_302: cast reint s_2_301 -> u8
        let s_2_302: u8 = (s_2_301.value() as u8);
        // C s_2_303: const #1s : i64
        let s_2_303: i64 = 1;
        // D s_2_304: call RotCell(s_2_302, s_2_303)
        let s_2_304: u8 = RotCell(state, tracer, s_2_302, s_2_303);
        // C s_2_305: const #8s : i
        let s_2_305: i128 = 8;
        // D s_2_306: read-var i:i64
        let s_2_306: i64 = fn_state.i;
        // D s_2_307: cast zx s_2_306 -> i
        let s_2_307: i128 = (i128::try_from(s_2_306).unwrap());
        // D s_2_308: add s_2_307 s_2_305
        let s_2_308: i128 = (s_2_307 + s_2_305);
        // D s_2_309: cast reint s_2_308 -> i64
        let s_2_309: i64 = (s_2_308 as i64);
        // C s_2_310: const #4s : i
        let s_2_310: i128 = 4;
        // D s_2_311: cast zx s_2_309 -> i
        let s_2_311: i128 = (i128::try_from(s_2_309).unwrap());
        // D s_2_312: mul s_2_310 s_2_311
        let s_2_312: i128 = ((s_2_310) * (s_2_311));
        // D s_2_313: cast reint s_2_312 -> i64
        let s_2_313: i64 = (s_2_312 as i64);
        // C s_2_314: const #4s : i
        let s_2_314: i128 = 4;
        // D s_2_315: read-var Sinput:u64
        let s_2_315: u64 = fn_state.Sinput;
        // D s_2_316: cast zx s_2_315 -> bv
        let s_2_316: Bits = Bits::new(s_2_315 as u128, 64u16);
        // D s_2_317: cast zx s_2_313 -> i
        let s_2_317: i128 = (i128::try_from(s_2_313).unwrap());
        // D s_2_318: bit-extract s_2_316 s_2_317 s_2_314
        let s_2_318: Bits = (Bits::new(
            ((s_2_316) >> (s_2_317)).value(),
            u16::try_from(s_2_314).unwrap(),
        ));
        // D s_2_319: cast reint s_2_318 -> u8
        let s_2_319: u8 = (s_2_318.value() as u8);
        // C s_2_320: const #2s : i64
        let s_2_320: i64 = 2;
        // D s_2_321: call RotCell(s_2_319, s_2_320)
        let s_2_321: u8 = RotCell(state, tracer, s_2_319, s_2_320);
        // D s_2_322: cast zx s_2_304 -> bv
        let s_2_322: Bits = Bits::new(s_2_304 as u128, 4u16);
        // D s_2_323: cast zx s_2_321 -> bv
        let s_2_323: Bits = Bits::new(s_2_321 as u128, 4u16);
        // D s_2_324: xor s_2_322 s_2_323
        let s_2_324: Bits = ((s_2_322) ^ (s_2_323));
        // D s_2_325: cast reint s_2_324 -> u8
        let s_2_325: u8 = (s_2_324.value() as u8);
        // C s_2_326: const #0s : i
        let s_2_326: i128 = 0;
        // D s_2_327: read-var t3:u8
        let s_2_327: u8 = fn_state.t3;
        // D s_2_328: cast zx s_2_327 -> bv
        let s_2_328: Bits = Bits::new(s_2_327 as u128, 4u16);
        // D s_2_329: cast zx s_2_325 -> bv
        let s_2_329: Bits = Bits::new(s_2_325 as u128, 4u16);
        // C s_2_330: const #3s : i
        let s_2_330: i128 = 3;
        // C s_2_331: const #1u : u64
        let s_2_331: u64 = 1;
        // C s_2_332: cast zx s_2_331 -> bv
        let s_2_332: Bits = Bits::new(s_2_331 as u128, 64u16);
        // C s_2_333: lsl s_2_332 s_2_330
        let s_2_333: Bits = s_2_332 << s_2_330;
        // C s_2_334: sub s_2_333 s_2_332
        let s_2_334: Bits = ((s_2_333) - (s_2_332));
        // D s_2_335: and s_2_329 s_2_334
        let s_2_335: Bits = ((s_2_329) & (s_2_334));
        // D s_2_336: lsl s_2_335 s_2_326
        let s_2_336: Bits = s_2_335 << s_2_326;
        // C s_2_337: lsl s_2_334 s_2_326
        let s_2_337: Bits = s_2_334 << s_2_326;
        // C s_2_338: cmpl s_2_337
        let s_2_338: Bits = !s_2_337;
        // D s_2_339: and s_2_328 s_2_338
        let s_2_339: Bits = ((s_2_328) & (s_2_338));
        // D s_2_340: or s_2_339 s_2_336
        let s_2_340: Bits = ((s_2_339) | (s_2_336));
        // D s_2_341: cast reint s_2_340 -> u8
        let s_2_341: u8 = (s_2_340.value() as u8);
        // D s_2_342: write-var t3 <= s_2_341
        fn_state.t3 = s_2_341;
        // C s_2_343: const #0s : i
        let s_2_343: i128 = 0;
        // D s_2_344: cast zx s_2_341 -> bv
        let s_2_344: Bits = Bits::new(s_2_341 as u128, 4u16);
        // C s_2_345: const #1s : i64
        let s_2_345: i64 = 1;
        // C s_2_346: cast zx s_2_345 -> i
        let s_2_346: i128 = (i128::try_from(s_2_345).unwrap());
        // C s_2_347: const #3s : i
        let s_2_347: i128 = 3;
        // C s_2_348: add s_2_347 s_2_346
        let s_2_348: i128 = (s_2_347 + s_2_346);
        // D s_2_349: bit-extract s_2_344 s_2_343 s_2_348
        let s_2_349: Bits = (Bits::new(
            ((s_2_344) >> (s_2_343)).value(),
            u16::try_from(s_2_348).unwrap(),
        ));
        // D s_2_350: cast reint s_2_349 -> u8
        let s_2_350: u8 = (s_2_349.value() as u8);
        // C s_2_351: const #4s : i
        let s_2_351: i128 = 4;
        // D s_2_352: read-var i:i64
        let s_2_352: i64 = fn_state.i;
        // D s_2_353: cast zx s_2_352 -> i
        let s_2_353: i128 = (i128::try_from(s_2_352).unwrap());
        // D s_2_354: add s_2_353 s_2_351
        let s_2_354: i128 = (s_2_353 + s_2_351);
        // D s_2_355: cast reint s_2_354 -> i64
        let s_2_355: i64 = (s_2_354 as i64);
        // C s_2_356: const #4s : i
        let s_2_356: i128 = 4;
        // D s_2_357: cast zx s_2_355 -> i
        let s_2_357: i128 = (i128::try_from(s_2_355).unwrap());
        // D s_2_358: mul s_2_356 s_2_357
        let s_2_358: i128 = ((s_2_356) * (s_2_357));
        // D s_2_359: cast reint s_2_358 -> i64
        let s_2_359: i64 = (s_2_358 as i64);
        // C s_2_360: const #4s : i
        let s_2_360: i128 = 4;
        // D s_2_361: read-var Sinput:u64
        let s_2_361: u64 = fn_state.Sinput;
        // D s_2_362: cast zx s_2_361 -> bv
        let s_2_362: Bits = Bits::new(s_2_361 as u128, 64u16);
        // D s_2_363: cast zx s_2_359 -> i
        let s_2_363: i128 = (i128::try_from(s_2_359).unwrap());
        // D s_2_364: bit-extract s_2_362 s_2_363 s_2_360
        let s_2_364: Bits = (Bits::new(
            ((s_2_362) >> (s_2_363)).value(),
            u16::try_from(s_2_360).unwrap(),
        ));
        // D s_2_365: cast reint s_2_364 -> u8
        let s_2_365: u8 = (s_2_364.value() as u8);
        // C s_2_366: const #1s : i64
        let s_2_366: i64 = 1;
        // D s_2_367: call RotCell(s_2_365, s_2_366)
        let s_2_367: u8 = RotCell(state, tracer, s_2_365, s_2_366);
        // D s_2_368: cast zx s_2_350 -> bv
        let s_2_368: Bits = Bits::new(s_2_350 as u128, 4u16);
        // D s_2_369: cast zx s_2_367 -> bv
        let s_2_369: Bits = Bits::new(s_2_367 as u128, 4u16);
        // D s_2_370: xor s_2_368 s_2_369
        let s_2_370: Bits = ((s_2_368) ^ (s_2_369));
        // D s_2_371: cast reint s_2_370 -> u8
        let s_2_371: u8 = (s_2_370.value() as u8);
        // C s_2_372: const #0s : i
        let s_2_372: i128 = 0;
        // D s_2_373: cast zx s_2_341 -> bv
        let s_2_373: Bits = Bits::new(s_2_341 as u128, 4u16);
        // D s_2_374: cast zx s_2_371 -> bv
        let s_2_374: Bits = Bits::new(s_2_371 as u128, 4u16);
        // C s_2_375: const #3s : i
        let s_2_375: i128 = 3;
        // C s_2_376: const #1u : u64
        let s_2_376: u64 = 1;
        // C s_2_377: cast zx s_2_376 -> bv
        let s_2_377: Bits = Bits::new(s_2_376 as u128, 64u16);
        // C s_2_378: lsl s_2_377 s_2_375
        let s_2_378: Bits = s_2_377 << s_2_375;
        // C s_2_379: sub s_2_378 s_2_377
        let s_2_379: Bits = ((s_2_378) - (s_2_377));
        // D s_2_380: and s_2_374 s_2_379
        let s_2_380: Bits = ((s_2_374) & (s_2_379));
        // D s_2_381: lsl s_2_380 s_2_372
        let s_2_381: Bits = s_2_380 << s_2_372;
        // C s_2_382: lsl s_2_379 s_2_372
        let s_2_382: Bits = s_2_379 << s_2_372;
        // C s_2_383: cmpl s_2_382
        let s_2_383: Bits = !s_2_382;
        // D s_2_384: and s_2_373 s_2_383
        let s_2_384: Bits = ((s_2_373) & (s_2_383));
        // D s_2_385: or s_2_384 s_2_381
        let s_2_385: Bits = ((s_2_384) | (s_2_381));
        // D s_2_386: cast reint s_2_385 -> u8
        let s_2_386: u8 = (s_2_385.value() as u8);
        // D s_2_387: write-var t3 <= s_2_386
        fn_state.t3 = s_2_386;
        // C s_2_388: const #4s : i
        let s_2_388: i128 = 4;
        // D s_2_389: read-var i:i64
        let s_2_389: i64 = fn_state.i;
        // D s_2_390: cast zx s_2_389 -> i
        let s_2_390: i128 = (i128::try_from(s_2_389).unwrap());
        // D s_2_391: mul s_2_388 s_2_390
        let s_2_391: i128 = ((s_2_388) * (s_2_390));
        // D s_2_392: cast reint s_2_391 -> i64
        let s_2_392: i64 = (s_2_391 as i64);
        // C s_2_393: const #3s : i
        let s_2_393: i128 = 3;
        // D s_2_394: cast zx s_2_392 -> i
        let s_2_394: i128 = (i128::try_from(s_2_392).unwrap());
        // D s_2_395: add s_2_394 s_2_393
        let s_2_395: i128 = (s_2_394 + s_2_393);
        // D s_2_396: cast reint s_2_395 -> i64
        let s_2_396: i64 = (s_2_395 as i64);
        // C s_2_397: const #4s : i
        let s_2_397: i128 = 4;
        // D s_2_398: read-var i:i64
        let s_2_398: i64 = fn_state.i;
        // D s_2_399: cast zx s_2_398 -> i
        let s_2_399: i128 = (i128::try_from(s_2_398).unwrap());
        // D s_2_400: mul s_2_397 s_2_399
        let s_2_400: i128 = ((s_2_397) * (s_2_399));
        // D s_2_401: cast reint s_2_400 -> i64
        let s_2_401: i64 = (s_2_400 as i64);
        // C s_2_402: const #64s : i
        let s_2_402: i128 = 64;
        // C s_2_403: const #3s : i
        let s_2_403: i128 = 3;
        // C s_2_404: const #0s : i
        let s_2_404: i128 = 0;
        // D s_2_405: read-var Soutput:u64
        let s_2_405: u64 = fn_state.Soutput;
        // D s_2_406: cast zx s_2_405 -> bv
        let s_2_406: Bits = Bits::new(s_2_405 as u128, 64u16);
        // D s_2_407: cast zx s_2_396 -> i
        let s_2_407: i128 = (i128::try_from(s_2_396).unwrap());
        // D s_2_408: cast zx s_2_401 -> i
        let s_2_408: i128 = (i128::try_from(s_2_401).unwrap());
        // D s_2_409: cast zx s_2_386 -> bv
        let s_2_409: Bits = Bits::new(s_2_386 as u128, 4u16);
        // D s_2_410: call vector_update_subrange_from_subrange(s_2_402, s_2_406, s_2_407, s_2_408, s_2_409, s_2_403, s_2_404)
        let s_2_410: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_2_402,
            s_2_406,
            s_2_407,
            s_2_408,
            s_2_409,
            s_2_403,
            s_2_404,
        );
        // D s_2_411: cast reint s_2_410 -> u64
        let s_2_411: u64 = (s_2_410.value() as u64);
        // D s_2_412: write-var Soutput <= s_2_411
        fn_state.Soutput = s_2_411;
        // C s_2_413: const #4s : i
        let s_2_413: i128 = 4;
        // D s_2_414: read-var i:i64
        let s_2_414: i64 = fn_state.i;
        // D s_2_415: cast zx s_2_414 -> i
        let s_2_415: i128 = (i128::try_from(s_2_414).unwrap());
        // D s_2_416: add s_2_415 s_2_413
        let s_2_416: i128 = (s_2_415 + s_2_413);
        // D s_2_417: cast reint s_2_416 -> i64
        let s_2_417: i64 = (s_2_416 as i64);
        // C s_2_418: const #4s : i
        let s_2_418: i128 = 4;
        // D s_2_419: cast zx s_2_417 -> i
        let s_2_419: i128 = (i128::try_from(s_2_417).unwrap());
        // D s_2_420: mul s_2_418 s_2_419
        let s_2_420: i128 = ((s_2_418) * (s_2_419));
        // D s_2_421: cast reint s_2_420 -> i64
        let s_2_421: i64 = (s_2_420 as i64);
        // C s_2_422: const #3s : i
        let s_2_422: i128 = 3;
        // D s_2_423: cast zx s_2_421 -> i
        let s_2_423: i128 = (i128::try_from(s_2_421).unwrap());
        // D s_2_424: add s_2_423 s_2_422
        let s_2_424: i128 = (s_2_423 + s_2_422);
        // D s_2_425: cast reint s_2_424 -> i64
        let s_2_425: i64 = (s_2_424 as i64);
        // C s_2_426: const #4s : i
        let s_2_426: i128 = 4;
        // D s_2_427: read-var i:i64
        let s_2_427: i64 = fn_state.i;
        // D s_2_428: cast zx s_2_427 -> i
        let s_2_428: i128 = (i128::try_from(s_2_427).unwrap());
        // D s_2_429: add s_2_428 s_2_426
        let s_2_429: i128 = (s_2_428 + s_2_426);
        // D s_2_430: cast reint s_2_429 -> i64
        let s_2_430: i64 = (s_2_429 as i64);
        // C s_2_431: const #4s : i
        let s_2_431: i128 = 4;
        // D s_2_432: cast zx s_2_430 -> i
        let s_2_432: i128 = (i128::try_from(s_2_430).unwrap());
        // D s_2_433: mul s_2_431 s_2_432
        let s_2_433: i128 = ((s_2_431) * (s_2_432));
        // D s_2_434: cast reint s_2_433 -> i64
        let s_2_434: i64 = (s_2_433 as i64);
        // C s_2_435: const #64s : i
        let s_2_435: i128 = 64;
        // C s_2_436: const #3s : i
        let s_2_436: i128 = 3;
        // C s_2_437: const #0s : i
        let s_2_437: i128 = 0;
        // D s_2_438: read-var Soutput:u64
        let s_2_438: u64 = fn_state.Soutput;
        // D s_2_439: cast zx s_2_438 -> bv
        let s_2_439: Bits = Bits::new(s_2_438 as u128, 64u16);
        // D s_2_440: cast zx s_2_425 -> i
        let s_2_440: i128 = (i128::try_from(s_2_425).unwrap());
        // D s_2_441: cast zx s_2_434 -> i
        let s_2_441: i128 = (i128::try_from(s_2_434).unwrap());
        // D s_2_442: cast zx s_2_286 -> bv
        let s_2_442: Bits = Bits::new(s_2_286 as u128, 4u16);
        // D s_2_443: call vector_update_subrange_from_subrange(s_2_435, s_2_439, s_2_440, s_2_441, s_2_442, s_2_436, s_2_437)
        let s_2_443: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_2_435,
            s_2_439,
            s_2_440,
            s_2_441,
            s_2_442,
            s_2_436,
            s_2_437,
        );
        // D s_2_444: cast reint s_2_443 -> u64
        let s_2_444: u64 = (s_2_443.value() as u64);
        // D s_2_445: write-var Soutput <= s_2_444
        fn_state.Soutput = s_2_444;
        // C s_2_446: const #8s : i
        let s_2_446: i128 = 8;
        // D s_2_447: read-var i:i64
        let s_2_447: i64 = fn_state.i;
        // D s_2_448: cast zx s_2_447 -> i
        let s_2_448: i128 = (i128::try_from(s_2_447).unwrap());
        // D s_2_449: add s_2_448 s_2_446
        let s_2_449: i128 = (s_2_448 + s_2_446);
        // D s_2_450: cast reint s_2_449 -> i64
        let s_2_450: i64 = (s_2_449 as i64);
        // C s_2_451: const #4s : i
        let s_2_451: i128 = 4;
        // D s_2_452: cast zx s_2_450 -> i
        let s_2_452: i128 = (i128::try_from(s_2_450).unwrap());
        // D s_2_453: mul s_2_451 s_2_452
        let s_2_453: i128 = ((s_2_451) * (s_2_452));
        // D s_2_454: cast reint s_2_453 -> i64
        let s_2_454: i64 = (s_2_453 as i64);
        // C s_2_455: const #3s : i
        let s_2_455: i128 = 3;
        // D s_2_456: cast zx s_2_454 -> i
        let s_2_456: i128 = (i128::try_from(s_2_454).unwrap());
        // D s_2_457: add s_2_456 s_2_455
        let s_2_457: i128 = (s_2_456 + s_2_455);
        // D s_2_458: cast reint s_2_457 -> i64
        let s_2_458: i64 = (s_2_457 as i64);
        // C s_2_459: const #8s : i
        let s_2_459: i128 = 8;
        // D s_2_460: read-var i:i64
        let s_2_460: i64 = fn_state.i;
        // D s_2_461: cast zx s_2_460 -> i
        let s_2_461: i128 = (i128::try_from(s_2_460).unwrap());
        // D s_2_462: add s_2_461 s_2_459
        let s_2_462: i128 = (s_2_461 + s_2_459);
        // D s_2_463: cast reint s_2_462 -> i64
        let s_2_463: i64 = (s_2_462 as i64);
        // C s_2_464: const #4s : i
        let s_2_464: i128 = 4;
        // D s_2_465: cast zx s_2_463 -> i
        let s_2_465: i128 = (i128::try_from(s_2_463).unwrap());
        // D s_2_466: mul s_2_464 s_2_465
        let s_2_466: i128 = ((s_2_464) * (s_2_465));
        // D s_2_467: cast reint s_2_466 -> i64
        let s_2_467: i64 = (s_2_466 as i64);
        // C s_2_468: const #64s : i
        let s_2_468: i128 = 64;
        // C s_2_469: const #3s : i
        let s_2_469: i128 = 3;
        // C s_2_470: const #0s : i
        let s_2_470: i128 = 0;
        // D s_2_471: read-var Soutput:u64
        let s_2_471: u64 = fn_state.Soutput;
        // D s_2_472: cast zx s_2_471 -> bv
        let s_2_472: Bits = Bits::new(s_2_471 as u128, 64u16);
        // D s_2_473: cast zx s_2_458 -> i
        let s_2_473: i128 = (i128::try_from(s_2_458).unwrap());
        // D s_2_474: cast zx s_2_467 -> i
        let s_2_474: i128 = (i128::try_from(s_2_467).unwrap());
        // D s_2_475: cast zx s_2_190 -> bv
        let s_2_475: Bits = Bits::new(s_2_190 as u128, 4u16);
        // D s_2_476: call vector_update_subrange_from_subrange(s_2_468, s_2_472, s_2_473, s_2_474, s_2_475, s_2_469, s_2_470)
        let s_2_476: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_2_468,
            s_2_472,
            s_2_473,
            s_2_474,
            s_2_475,
            s_2_469,
            s_2_470,
        );
        // D s_2_477: cast reint s_2_476 -> u64
        let s_2_477: u64 = (s_2_476.value() as u64);
        // D s_2_478: write-var Soutput <= s_2_477
        fn_state.Soutput = s_2_477;
        // C s_2_479: const #12s : i
        let s_2_479: i128 = 12;
        // D s_2_480: read-var i:i64
        let s_2_480: i64 = fn_state.i;
        // D s_2_481: cast zx s_2_480 -> i
        let s_2_481: i128 = (i128::try_from(s_2_480).unwrap());
        // D s_2_482: add s_2_481 s_2_479
        let s_2_482: i128 = (s_2_481 + s_2_479);
        // D s_2_483: cast reint s_2_482 -> i64
        let s_2_483: i64 = (s_2_482 as i64);
        // C s_2_484: const #4s : i
        let s_2_484: i128 = 4;
        // D s_2_485: cast zx s_2_483 -> i
        let s_2_485: i128 = (i128::try_from(s_2_483).unwrap());
        // D s_2_486: mul s_2_484 s_2_485
        let s_2_486: i128 = ((s_2_484) * (s_2_485));
        // D s_2_487: cast reint s_2_486 -> i64
        let s_2_487: i64 = (s_2_486 as i64);
        // C s_2_488: const #3s : i
        let s_2_488: i128 = 3;
        // D s_2_489: cast zx s_2_487 -> i
        let s_2_489: i128 = (i128::try_from(s_2_487).unwrap());
        // D s_2_490: add s_2_489 s_2_488
        let s_2_490: i128 = (s_2_489 + s_2_488);
        // D s_2_491: cast reint s_2_490 -> i64
        let s_2_491: i64 = (s_2_490 as i64);
        // C s_2_492: const #12s : i
        let s_2_492: i128 = 12;
        // D s_2_493: read-var i:i64
        let s_2_493: i64 = fn_state.i;
        // D s_2_494: cast zx s_2_493 -> i
        let s_2_494: i128 = (i128::try_from(s_2_493).unwrap());
        // D s_2_495: add s_2_494 s_2_492
        let s_2_495: i128 = (s_2_494 + s_2_492);
        // D s_2_496: cast reint s_2_495 -> i64
        let s_2_496: i64 = (s_2_495 as i64);
        // C s_2_497: const #4s : i
        let s_2_497: i128 = 4;
        // D s_2_498: cast zx s_2_496 -> i
        let s_2_498: i128 = (i128::try_from(s_2_496).unwrap());
        // D s_2_499: mul s_2_497 s_2_498
        let s_2_499: i128 = ((s_2_497) * (s_2_498));
        // D s_2_500: cast reint s_2_499 -> i64
        let s_2_500: i64 = (s_2_499 as i64);
        // C s_2_501: const #64s : i
        let s_2_501: i128 = 64;
        // C s_2_502: const #3s : i
        let s_2_502: i128 = 3;
        // C s_2_503: const #0s : i
        let s_2_503: i128 = 0;
        // D s_2_504: read-var Soutput:u64
        let s_2_504: u64 = fn_state.Soutput;
        // D s_2_505: cast zx s_2_504 -> bv
        let s_2_505: Bits = Bits::new(s_2_504 as u128, 64u16);
        // D s_2_506: cast zx s_2_491 -> i
        let s_2_506: i128 = (i128::try_from(s_2_491).unwrap());
        // D s_2_507: cast zx s_2_500 -> i
        let s_2_507: i128 = (i128::try_from(s_2_500).unwrap());
        // D s_2_508: cast zx s_2_94 -> bv
        let s_2_508: Bits = Bits::new(s_2_94 as u128, 4u16);
        // D s_2_509: call vector_update_subrange_from_subrange(s_2_501, s_2_505, s_2_506, s_2_507, s_2_508, s_2_502, s_2_503)
        let s_2_509: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_2_501,
            s_2_505,
            s_2_506,
            s_2_507,
            s_2_508,
            s_2_502,
            s_2_503,
        );
        // D s_2_510: cast reint s_2_509 -> u64
        let s_2_510: u64 = (s_2_509.value() as u64);
        // D s_2_511: write-var Soutput <= s_2_510
        fn_state.Soutput = s_2_510;
        // D s_2_512: read-var i:i64
        let s_2_512: i64 = fn_state.i;
        // C s_2_513: const #1s : i64
        let s_2_513: i64 = 1;
        // D s_2_514: add s_2_512 s_2_513
        let s_2_514: i64 = (s_2_512 + s_2_513);
        // D s_2_515: write-var i <= s_2_514
        fn_state.i = s_2_514;
        // N s_2_516: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var Soutput:u64
        let s_3_0: u64 = fn_state.Soutput;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
}
