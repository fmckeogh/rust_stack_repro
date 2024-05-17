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
use ROL::*;
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3partw2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tmp: u128,
        result: u128,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // C s_1_10: const #128s : i64
        let s_1_10: i64 = 128;
        // D s_1_11: read-var d:i64
        let s_1_11: i64 = fn_state.d;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call V_read(s_1_12, s_1_10)
        let s_1_13: Bits = V_read(state, tracer, s_1_12, s_1_10);
        // D s_1_14: cast reint s_1_13 -> u128
        let s_1_14: u128 = (s_1_13.value() as u128);
        // C s_1_15: const #96s : i
        let s_1_15: i128 = 96;
        // D s_1_16: cast zx s_1_4 -> bv
        let s_1_16: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_17: const #1s : i64
        let s_1_17: i64 = 1;
        // C s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // C s_1_19: const #31s : i
        let s_1_19: i128 = 31;
        // C s_1_20: add s_1_19 s_1_18
        let s_1_20: i128 = (s_1_19 + s_1_18);
        // D s_1_21: bit-extract s_1_16 s_1_15 s_1_20
        let s_1_21: Bits = (Bits::new(
            ((s_1_16) >> (s_1_15)).value(),
            u16::try_from(s_1_20).unwrap(),
        ));
        // D s_1_22: cast reint s_1_21 -> u32
        let s_1_22: u32 = (s_1_21.value() as u32);
        // C s_1_23: const #7s : i
        let s_1_23: i128 = 7;
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 32u16);
        // D s_1_25: call ROL(s_1_24, s_1_23)
        let s_1_25: Bits = ROL(state, tracer, s_1_24, s_1_23);
        // D s_1_26: cast reint s_1_25 -> u32
        let s_1_26: u32 = (s_1_25.value() as u32);
        // C s_1_27: const #64s : i
        let s_1_27: i128 = 64;
        // D s_1_28: cast zx s_1_4 -> bv
        let s_1_28: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_29: const #1s : i64
        let s_1_29: i64 = 1;
        // C s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // C s_1_31: const #31s : i
        let s_1_31: i128 = 31;
        // C s_1_32: add s_1_31 s_1_30
        let s_1_32: i128 = (s_1_31 + s_1_30);
        // D s_1_33: bit-extract s_1_28 s_1_27 s_1_32
        let s_1_33: Bits = (Bits::new(
            ((s_1_28) >> (s_1_27)).value(),
            u16::try_from(s_1_32).unwrap(),
        ));
        // D s_1_34: cast reint s_1_33 -> u32
        let s_1_34: u32 = (s_1_33.value() as u32);
        // C s_1_35: const #7s : i
        let s_1_35: i128 = 7;
        // D s_1_36: cast zx s_1_34 -> bv
        let s_1_36: Bits = Bits::new(s_1_34 as u128, 32u16);
        // D s_1_37: call ROL(s_1_36, s_1_35)
        let s_1_37: Bits = ROL(state, tracer, s_1_36, s_1_35);
        // D s_1_38: cast reint s_1_37 -> u32
        let s_1_38: u32 = (s_1_37.value() as u32);
        // D s_1_39: cast zx s_1_26 -> bv
        let s_1_39: Bits = Bits::new(s_1_26 as u128, 32u16);
        // D s_1_40: cast zx s_1_38 -> bv
        let s_1_40: Bits = Bits::new(s_1_38 as u128, 32u16);
        // D s_1_41: cast reint s_1_39 -> u128
        let s_1_41: u128 = (s_1_39.value() as u128);
        // D s_1_42: size-of s_1_39
        let s_1_42: u16 = s_1_39.length();
        // D s_1_43: cast reint s_1_40 -> u128
        let s_1_43: u128 = (s_1_40.value() as u128);
        // D s_1_44: size-of s_1_40
        let s_1_44: u16 = s_1_40.length();
        // D s_1_45: lsl s_1_41 s_1_44
        let s_1_45: u128 = s_1_41 << s_1_44;
        // D s_1_46: or s_1_45 s_1_43
        let s_1_46: u128 = ((s_1_45) | (s_1_43));
        // D s_1_47: add s_1_42 s_1_44
        let s_1_47: u16 = (s_1_42 + s_1_44);
        // D s_1_48: create-bits s_1_46 s_1_47
        let s_1_48: Bits = Bits::new(s_1_46, s_1_47);
        // D s_1_49: cast reint s_1_48 -> u64
        let s_1_49: u64 = (s_1_48.value() as u64);
        // C s_1_50: const #32s : i
        let s_1_50: i128 = 32;
        // D s_1_51: cast zx s_1_4 -> bv
        let s_1_51: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_52: const #1s : i64
        let s_1_52: i64 = 1;
        // C s_1_53: cast zx s_1_52 -> i
        let s_1_53: i128 = (i128::try_from(s_1_52).unwrap());
        // C s_1_54: const #31s : i
        let s_1_54: i128 = 31;
        // C s_1_55: add s_1_54 s_1_53
        let s_1_55: i128 = (s_1_54 + s_1_53);
        // D s_1_56: bit-extract s_1_51 s_1_50 s_1_55
        let s_1_56: Bits = (Bits::new(
            ((s_1_51) >> (s_1_50)).value(),
            u16::try_from(s_1_55).unwrap(),
        ));
        // D s_1_57: cast reint s_1_56 -> u32
        let s_1_57: u32 = (s_1_56.value() as u32);
        // C s_1_58: const #7s : i
        let s_1_58: i128 = 7;
        // D s_1_59: cast zx s_1_57 -> bv
        let s_1_59: Bits = Bits::new(s_1_57 as u128, 32u16);
        // D s_1_60: call ROL(s_1_59, s_1_58)
        let s_1_60: Bits = ROL(state, tracer, s_1_59, s_1_58);
        // D s_1_61: cast reint s_1_60 -> u32
        let s_1_61: u32 = (s_1_60.value() as u32);
        // D s_1_62: cast zx s_1_49 -> bv
        let s_1_62: Bits = Bits::new(s_1_49 as u128, 64u16);
        // D s_1_63: cast zx s_1_61 -> bv
        let s_1_63: Bits = Bits::new(s_1_61 as u128, 32u16);
        // D s_1_64: cast reint s_1_62 -> u128
        let s_1_64: u128 = (s_1_62.value() as u128);
        // D s_1_65: size-of s_1_62
        let s_1_65: u16 = s_1_62.length();
        // D s_1_66: cast reint s_1_63 -> u128
        let s_1_66: u128 = (s_1_63.value() as u128);
        // D s_1_67: size-of s_1_63
        let s_1_67: u16 = s_1_63.length();
        // D s_1_68: lsl s_1_64 s_1_67
        let s_1_68: u128 = s_1_64 << s_1_67;
        // D s_1_69: or s_1_68 s_1_66
        let s_1_69: u128 = ((s_1_68) | (s_1_66));
        // D s_1_70: add s_1_65 s_1_67
        let s_1_70: u16 = (s_1_65 + s_1_67);
        // D s_1_71: create-bits s_1_69 s_1_70
        let s_1_71: Bits = Bits::new(s_1_69, s_1_70);
        // D s_1_72: cast reint s_1_71 -> u96
        let s_1_72: u128 = (s_1_71.value() as u128);
        // C s_1_73: const #0s : i
        let s_1_73: i128 = 0;
        // D s_1_74: cast zx s_1_4 -> bv
        let s_1_74: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_75: const #1s : i64
        let s_1_75: i64 = 1;
        // C s_1_76: cast zx s_1_75 -> i
        let s_1_76: i128 = (i128::try_from(s_1_75).unwrap());
        // C s_1_77: const #31s : i
        let s_1_77: i128 = 31;
        // C s_1_78: add s_1_77 s_1_76
        let s_1_78: i128 = (s_1_77 + s_1_76);
        // D s_1_79: bit-extract s_1_74 s_1_73 s_1_78
        let s_1_79: Bits = (Bits::new(
            ((s_1_74) >> (s_1_73)).value(),
            u16::try_from(s_1_78).unwrap(),
        ));
        // D s_1_80: cast reint s_1_79 -> u32
        let s_1_80: u32 = (s_1_79.value() as u32);
        // C s_1_81: const #7s : i
        let s_1_81: i128 = 7;
        // D s_1_82: cast zx s_1_80 -> bv
        let s_1_82: Bits = Bits::new(s_1_80 as u128, 32u16);
        // D s_1_83: call ROL(s_1_82, s_1_81)
        let s_1_83: Bits = ROL(state, tracer, s_1_82, s_1_81);
        // D s_1_84: cast reint s_1_83 -> u32
        let s_1_84: u32 = (s_1_83.value() as u32);
        // D s_1_85: cast zx s_1_72 -> bv
        let s_1_85: Bits = Bits::new(s_1_72 as u128, 96u16);
        // D s_1_86: cast zx s_1_84 -> bv
        let s_1_86: Bits = Bits::new(s_1_84 as u128, 32u16);
        // D s_1_87: cast reint s_1_85 -> u128
        let s_1_87: u128 = (s_1_85.value() as u128);
        // D s_1_88: size-of s_1_85
        let s_1_88: u16 = s_1_85.length();
        // D s_1_89: cast reint s_1_86 -> u128
        let s_1_89: u128 = (s_1_86.value() as u128);
        // D s_1_90: size-of s_1_86
        let s_1_90: u16 = s_1_86.length();
        // D s_1_91: lsl s_1_87 s_1_90
        let s_1_91: u128 = s_1_87 << s_1_90;
        // D s_1_92: or s_1_91 s_1_89
        let s_1_92: u128 = ((s_1_91) | (s_1_89));
        // D s_1_93: add s_1_88 s_1_90
        let s_1_93: u16 = (s_1_88 + s_1_90);
        // D s_1_94: create-bits s_1_92 s_1_93
        let s_1_94: Bits = Bits::new(s_1_92, s_1_93);
        // D s_1_95: cast reint s_1_94 -> u128
        let s_1_95: u128 = (s_1_94.value() as u128);
        // D s_1_96: cast zx s_1_9 -> bv
        let s_1_96: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_97: cast zx s_1_95 -> bv
        let s_1_97: Bits = Bits::new(s_1_95 as u128, 128u16);
        // D s_1_98: xor s_1_96 s_1_97
        let s_1_98: Bits = ((s_1_96) ^ (s_1_97));
        // D s_1_99: cast reint s_1_98 -> u128
        let s_1_99: u128 = (s_1_98.value() as u128);
        // C s_1_100: const #0s : i
        let s_1_100: i128 = 0;
        // D s_1_101: read-var tmp:u128
        let s_1_101: u128 = fn_state.tmp;
        // D s_1_102: cast zx s_1_101 -> bv
        let s_1_102: Bits = Bits::new(s_1_101 as u128, 128u16);
        // D s_1_103: cast zx s_1_99 -> bv
        let s_1_103: Bits = Bits::new(s_1_99 as u128, 128u16);
        // C s_1_104: const #127s : i
        let s_1_104: i128 = 127;
        // C s_1_105: const #1u : u64
        let s_1_105: u64 = 1;
        // C s_1_106: cast zx s_1_105 -> bv
        let s_1_106: Bits = Bits::new(s_1_105 as u128, 64u16);
        // C s_1_107: lsl s_1_106 s_1_104
        let s_1_107: Bits = s_1_106 << s_1_104;
        // C s_1_108: sub s_1_107 s_1_106
        let s_1_108: Bits = ((s_1_107) - (s_1_106));
        // D s_1_109: and s_1_103 s_1_108
        let s_1_109: Bits = ((s_1_103) & (s_1_108));
        // D s_1_110: lsl s_1_109 s_1_100
        let s_1_110: Bits = s_1_109 << s_1_100;
        // C s_1_111: lsl s_1_108 s_1_100
        let s_1_111: Bits = s_1_108 << s_1_100;
        // C s_1_112: cmpl s_1_111
        let s_1_112: Bits = !s_1_111;
        // D s_1_113: and s_1_102 s_1_112
        let s_1_113: Bits = ((s_1_102) & (s_1_112));
        // D s_1_114: or s_1_113 s_1_110
        let s_1_114: Bits = ((s_1_113) | (s_1_110));
        // D s_1_115: cast reint s_1_114 -> u128
        let s_1_115: u128 = (s_1_114.value() as u128);
        // D s_1_116: write-var tmp <= s_1_115
        fn_state.tmp = s_1_115;
        // C s_1_117: const #0s : i
        let s_1_117: i128 = 0;
        // D s_1_118: cast zx s_1_14 -> bv
        let s_1_118: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_119: const #1s : i64
        let s_1_119: i64 = 1;
        // C s_1_120: cast zx s_1_119 -> i
        let s_1_120: i128 = (i128::try_from(s_1_119).unwrap());
        // C s_1_121: const #127s : i
        let s_1_121: i128 = 127;
        // C s_1_122: add s_1_121 s_1_120
        let s_1_122: i128 = (s_1_121 + s_1_120);
        // D s_1_123: bit-extract s_1_118 s_1_117 s_1_122
        let s_1_123: Bits = (Bits::new(
            ((s_1_118) >> (s_1_117)).value(),
            u16::try_from(s_1_122).unwrap(),
        ));
        // D s_1_124: cast reint s_1_123 -> u128
        let s_1_124: u128 = (s_1_123.value() as u128);
        // C s_1_125: const #0s : i
        let s_1_125: i128 = 0;
        // D s_1_126: cast zx s_1_115 -> bv
        let s_1_126: Bits = Bits::new(s_1_115 as u128, 128u16);
        // C s_1_127: const #1s : i64
        let s_1_127: i64 = 1;
        // C s_1_128: cast zx s_1_127 -> i
        let s_1_128: i128 = (i128::try_from(s_1_127).unwrap());
        // C s_1_129: const #127s : i
        let s_1_129: i128 = 127;
        // C s_1_130: add s_1_129 s_1_128
        let s_1_130: i128 = (s_1_129 + s_1_128);
        // D s_1_131: bit-extract s_1_126 s_1_125 s_1_130
        let s_1_131: Bits = (Bits::new(
            ((s_1_126) >> (s_1_125)).value(),
            u16::try_from(s_1_130).unwrap(),
        ));
        // D s_1_132: cast reint s_1_131 -> u128
        let s_1_132: u128 = (s_1_131.value() as u128);
        // D s_1_133: cast zx s_1_124 -> bv
        let s_1_133: Bits = Bits::new(s_1_124 as u128, 128u16);
        // D s_1_134: cast zx s_1_132 -> bv
        let s_1_134: Bits = Bits::new(s_1_132 as u128, 128u16);
        // D s_1_135: xor s_1_133 s_1_134
        let s_1_135: Bits = ((s_1_133) ^ (s_1_134));
        // D s_1_136: cast reint s_1_135 -> u128
        let s_1_136: u128 = (s_1_135.value() as u128);
        // C s_1_137: const #0s : i
        let s_1_137: i128 = 0;
        // D s_1_138: read-var result:u128
        let s_1_138: u128 = fn_state.result;
        // D s_1_139: cast zx s_1_138 -> bv
        let s_1_139: Bits = Bits::new(s_1_138 as u128, 128u16);
        // D s_1_140: cast zx s_1_136 -> bv
        let s_1_140: Bits = Bits::new(s_1_136 as u128, 128u16);
        // C s_1_141: const #127s : i
        let s_1_141: i128 = 127;
        // C s_1_142: const #1u : u64
        let s_1_142: u64 = 1;
        // C s_1_143: cast zx s_1_142 -> bv
        let s_1_143: Bits = Bits::new(s_1_142 as u128, 64u16);
        // C s_1_144: lsl s_1_143 s_1_141
        let s_1_144: Bits = s_1_143 << s_1_141;
        // C s_1_145: sub s_1_144 s_1_143
        let s_1_145: Bits = ((s_1_144) - (s_1_143));
        // D s_1_146: and s_1_140 s_1_145
        let s_1_146: Bits = ((s_1_140) & (s_1_145));
        // D s_1_147: lsl s_1_146 s_1_137
        let s_1_147: Bits = s_1_146 << s_1_137;
        // C s_1_148: lsl s_1_145 s_1_137
        let s_1_148: Bits = s_1_145 << s_1_137;
        // C s_1_149: cmpl s_1_148
        let s_1_149: Bits = !s_1_148;
        // D s_1_150: and s_1_139 s_1_149
        let s_1_150: Bits = ((s_1_139) & (s_1_149));
        // D s_1_151: or s_1_150 s_1_147
        let s_1_151: Bits = ((s_1_150) | (s_1_147));
        // D s_1_152: cast reint s_1_151 -> u128
        let s_1_152: u128 = (s_1_151.value() as u128);
        // D s_1_153: write-var result <= s_1_152
        fn_state.result = s_1_152;
        // C s_1_154: const #0s : i
        let s_1_154: i128 = 0;
        // D s_1_155: cast zx s_1_115 -> bv
        let s_1_155: Bits = Bits::new(s_1_115 as u128, 128u16);
        // C s_1_156: const #1s : i64
        let s_1_156: i64 = 1;
        // C s_1_157: cast zx s_1_156 -> i
        let s_1_157: i128 = (i128::try_from(s_1_156).unwrap());
        // C s_1_158: const #31s : i
        let s_1_158: i128 = 31;
        // C s_1_159: add s_1_158 s_1_157
        let s_1_159: i128 = (s_1_158 + s_1_157);
        // D s_1_160: bit-extract s_1_155 s_1_154 s_1_159
        let s_1_160: Bits = (Bits::new(
            ((s_1_155) >> (s_1_154)).value(),
            u16::try_from(s_1_159).unwrap(),
        ));
        // D s_1_161: cast reint s_1_160 -> u32
        let s_1_161: u32 = (s_1_160.value() as u32);
        // C s_1_162: const #15s : i
        let s_1_162: i128 = 15;
        // D s_1_163: cast zx s_1_161 -> bv
        let s_1_163: Bits = Bits::new(s_1_161 as u128, 32u16);
        // D s_1_164: call ROL(s_1_163, s_1_162)
        let s_1_164: Bits = ROL(state, tracer, s_1_163, s_1_162);
        // D s_1_165: cast reint s_1_164 -> u32
        let s_1_165: u32 = (s_1_164.value() as u32);
        // C s_1_166: const #15s : i
        let s_1_166: i128 = 15;
        // D s_1_167: cast zx s_1_165 -> bv
        let s_1_167: Bits = Bits::new(s_1_165 as u128, 32u16);
        // D s_1_168: call ROL(s_1_167, s_1_166)
        let s_1_168: Bits = ROL(state, tracer, s_1_167, s_1_166);
        // D s_1_169: cast reint s_1_168 -> u32
        let s_1_169: u32 = (s_1_168.value() as u32);
        // D s_1_170: cast zx s_1_165 -> bv
        let s_1_170: Bits = Bits::new(s_1_165 as u128, 32u16);
        // D s_1_171: cast zx s_1_169 -> bv
        let s_1_171: Bits = Bits::new(s_1_169 as u128, 32u16);
        // D s_1_172: xor s_1_170 s_1_171
        let s_1_172: Bits = ((s_1_170) ^ (s_1_171));
        // D s_1_173: cast reint s_1_172 -> u32
        let s_1_173: u32 = (s_1_172.value() as u32);
        // C s_1_174: const #23s : i
        let s_1_174: i128 = 23;
        // D s_1_175: cast zx s_1_165 -> bv
        let s_1_175: Bits = Bits::new(s_1_165 as u128, 32u16);
        // D s_1_176: call ROL(s_1_175, s_1_174)
        let s_1_176: Bits = ROL(state, tracer, s_1_175, s_1_174);
        // D s_1_177: cast reint s_1_176 -> u32
        let s_1_177: u32 = (s_1_176.value() as u32);
        // D s_1_178: cast zx s_1_173 -> bv
        let s_1_178: Bits = Bits::new(s_1_173 as u128, 32u16);
        // D s_1_179: cast zx s_1_177 -> bv
        let s_1_179: Bits = Bits::new(s_1_177 as u128, 32u16);
        // D s_1_180: xor s_1_178 s_1_179
        let s_1_180: Bits = ((s_1_178) ^ (s_1_179));
        // D s_1_181: cast reint s_1_180 -> u32
        let s_1_181: u32 = (s_1_180.value() as u32);
        // C s_1_182: const #96s : i
        let s_1_182: i128 = 96;
        // D s_1_183: cast zx s_1_152 -> bv
        let s_1_183: Bits = Bits::new(s_1_152 as u128, 128u16);
        // C s_1_184: const #1s : i64
        let s_1_184: i64 = 1;
        // C s_1_185: cast zx s_1_184 -> i
        let s_1_185: i128 = (i128::try_from(s_1_184).unwrap());
        // C s_1_186: const #31s : i
        let s_1_186: i128 = 31;
        // C s_1_187: add s_1_186 s_1_185
        let s_1_187: i128 = (s_1_186 + s_1_185);
        // D s_1_188: bit-extract s_1_183 s_1_182 s_1_187
        let s_1_188: Bits = (Bits::new(
            ((s_1_183) >> (s_1_182)).value(),
            u16::try_from(s_1_187).unwrap(),
        ));
        // D s_1_189: cast reint s_1_188 -> u32
        let s_1_189: u32 = (s_1_188.value() as u32);
        // D s_1_190: cast zx s_1_189 -> bv
        let s_1_190: Bits = Bits::new(s_1_189 as u128, 32u16);
        // D s_1_191: cast zx s_1_181 -> bv
        let s_1_191: Bits = Bits::new(s_1_181 as u128, 32u16);
        // D s_1_192: xor s_1_190 s_1_191
        let s_1_192: Bits = ((s_1_190) ^ (s_1_191));
        // D s_1_193: cast reint s_1_192 -> u32
        let s_1_193: u32 = (s_1_192.value() as u32);
        // C s_1_194: const #96s : i
        let s_1_194: i128 = 96;
        // D s_1_195: cast zx s_1_152 -> bv
        let s_1_195: Bits = Bits::new(s_1_152 as u128, 128u16);
        // D s_1_196: cast zx s_1_193 -> bv
        let s_1_196: Bits = Bits::new(s_1_193 as u128, 32u16);
        // C s_1_197: const #31s : i
        let s_1_197: i128 = 31;
        // C s_1_198: const #1u : u64
        let s_1_198: u64 = 1;
        // C s_1_199: cast zx s_1_198 -> bv
        let s_1_199: Bits = Bits::new(s_1_198 as u128, 64u16);
        // C s_1_200: lsl s_1_199 s_1_197
        let s_1_200: Bits = s_1_199 << s_1_197;
        // C s_1_201: sub s_1_200 s_1_199
        let s_1_201: Bits = ((s_1_200) - (s_1_199));
        // D s_1_202: and s_1_196 s_1_201
        let s_1_202: Bits = ((s_1_196) & (s_1_201));
        // D s_1_203: lsl s_1_202 s_1_194
        let s_1_203: Bits = s_1_202 << s_1_194;
        // C s_1_204: lsl s_1_201 s_1_194
        let s_1_204: Bits = s_1_201 << s_1_194;
        // C s_1_205: cmpl s_1_204
        let s_1_205: Bits = !s_1_204;
        // D s_1_206: and s_1_195 s_1_205
        let s_1_206: Bits = ((s_1_195) & (s_1_205));
        // D s_1_207: or s_1_206 s_1_203
        let s_1_207: Bits = ((s_1_206) | (s_1_203));
        // D s_1_208: cast reint s_1_207 -> u128
        let s_1_208: u128 = (s_1_207.value() as u128);
        // D s_1_209: write-var result <= s_1_208
        fn_state.result = s_1_208;
        // C s_1_210: const #128s : i64
        let s_1_210: i64 = 128;
        // D s_1_211: read-var d:i64
        let s_1_211: i64 = fn_state.d;
        // D s_1_212: cast zx s_1_211 -> i
        let s_1_212: i128 = (i128::try_from(s_1_211).unwrap());
        // D s_1_213: cast zx s_1_208 -> bv
        let s_1_213: Bits = Bits::new(s_1_208 as u128, 128u16);
        // D s_1_214: call V_set(s_1_212, s_1_210, s_1_213)
        let s_1_214: () = V_set(state, tracer, s_1_212, s_1_210, s_1_213);
        // N s_1_215: return
        return;
    }
}
