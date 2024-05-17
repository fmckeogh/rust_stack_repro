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
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use ROR::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha512_sha512su1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Vtmp: u128,
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
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var m:i64
        let s_1_6: i64 = fn_state.m;
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
        // C s_1_15: const #64s : i
        let s_1_15: i128 = 64;
        // D s_1_16: cast zx s_1_4 -> bv
        let s_1_16: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_17: const #1s : i64
        let s_1_17: i64 = 1;
        // C s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // C s_1_19: const #63s : i
        let s_1_19: i128 = 63;
        // C s_1_20: add s_1_19 s_1_18
        let s_1_20: i128 = (s_1_19 + s_1_18);
        // D s_1_21: bit-extract s_1_16 s_1_15 s_1_20
        let s_1_21: Bits = (Bits::new(
            ((s_1_16) >> (s_1_15)).value(),
            u16::try_from(s_1_20).unwrap(),
        ));
        // D s_1_22: cast reint s_1_21 -> u64
        let s_1_22: u64 = (s_1_21.value() as u64);
        // C s_1_23: const #19s : i
        let s_1_23: i128 = 19;
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 64u16);
        // D s_1_25: call ROR(s_1_24, s_1_23)
        let s_1_25: Bits = ROR(state, tracer, s_1_24, s_1_23);
        // D s_1_26: cast reint s_1_25 -> u64
        let s_1_26: u64 = (s_1_25.value() as u64);
        // C s_1_27: const #64s : i
        let s_1_27: i128 = 64;
        // D s_1_28: cast zx s_1_4 -> bv
        let s_1_28: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_29: const #1s : i64
        let s_1_29: i64 = 1;
        // C s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // C s_1_31: const #63s : i
        let s_1_31: i128 = 63;
        // C s_1_32: add s_1_31 s_1_30
        let s_1_32: i128 = (s_1_31 + s_1_30);
        // D s_1_33: bit-extract s_1_28 s_1_27 s_1_32
        let s_1_33: Bits = (Bits::new(
            ((s_1_28) >> (s_1_27)).value(),
            u16::try_from(s_1_32).unwrap(),
        ));
        // D s_1_34: cast reint s_1_33 -> u64
        let s_1_34: u64 = (s_1_33.value() as u64);
        // C s_1_35: const #61s : i
        let s_1_35: i128 = 61;
        // D s_1_36: cast zx s_1_34 -> bv
        let s_1_36: Bits = Bits::new(s_1_34 as u128, 64u16);
        // D s_1_37: call ROR(s_1_36, s_1_35)
        let s_1_37: Bits = ROR(state, tracer, s_1_36, s_1_35);
        // D s_1_38: cast reint s_1_37 -> u64
        let s_1_38: u64 = (s_1_37.value() as u64);
        // D s_1_39: cast zx s_1_26 -> bv
        let s_1_39: Bits = Bits::new(s_1_26 as u128, 64u16);
        // D s_1_40: cast zx s_1_38 -> bv
        let s_1_40: Bits = Bits::new(s_1_38 as u128, 64u16);
        // D s_1_41: xor s_1_39 s_1_40
        let s_1_41: Bits = ((s_1_39) ^ (s_1_40));
        // D s_1_42: cast reint s_1_41 -> u64
        let s_1_42: u64 = (s_1_41.value() as u64);
        // C s_1_43: const #70s : i
        let s_1_43: i128 = 70;
        // D s_1_44: cast zx s_1_4 -> bv
        let s_1_44: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_45: const #1s : i64
        let s_1_45: i64 = 1;
        // C s_1_46: cast zx s_1_45 -> i
        let s_1_46: i128 = (i128::try_from(s_1_45).unwrap());
        // C s_1_47: const #57s : i
        let s_1_47: i128 = 57;
        // C s_1_48: add s_1_47 s_1_46
        let s_1_48: i128 = (s_1_47 + s_1_46);
        // D s_1_49: bit-extract s_1_44 s_1_43 s_1_48
        let s_1_49: Bits = (Bits::new(
            ((s_1_44) >> (s_1_43)).value(),
            u16::try_from(s_1_48).unwrap(),
        ));
        // D s_1_50: cast reint s_1_49 -> u58
        let s_1_50: u64 = (s_1_49.value() as u64);
        // C s_1_51: const #0u : u8
        let s_1_51: u8 = 0;
        // C s_1_52: cast zx s_1_51 -> bv
        let s_1_52: Bits = Bits::new(s_1_51 as u128, 6u16);
        // D s_1_53: cast zx s_1_50 -> bv
        let s_1_53: Bits = Bits::new(s_1_50 as u128, 58u16);
        // C s_1_54: cast reint s_1_52 -> u128
        let s_1_54: u128 = (s_1_52.value() as u128);
        // D s_1_55: size-of s_1_52
        let s_1_55: u16 = s_1_52.length();
        // D s_1_56: cast reint s_1_53 -> u128
        let s_1_56: u128 = (s_1_53.value() as u128);
        // D s_1_57: size-of s_1_53
        let s_1_57: u16 = s_1_53.length();
        // D s_1_58: lsl s_1_54 s_1_57
        let s_1_58: u128 = s_1_54 << s_1_57;
        // D s_1_59: or s_1_58 s_1_56
        let s_1_59: u128 = ((s_1_58) | (s_1_56));
        // D s_1_60: add s_1_55 s_1_57
        let s_1_60: u16 = (s_1_55 + s_1_57);
        // D s_1_61: create-bits s_1_59 s_1_60
        let s_1_61: Bits = Bits::new(s_1_59, s_1_60);
        // D s_1_62: cast reint s_1_61 -> u64
        let s_1_62: u64 = (s_1_61.value() as u64);
        // D s_1_63: cast zx s_1_42 -> bv
        let s_1_63: Bits = Bits::new(s_1_42 as u128, 64u16);
        // D s_1_64: cast zx s_1_62 -> bv
        let s_1_64: Bits = Bits::new(s_1_62 as u128, 64u16);
        // D s_1_65: xor s_1_63 s_1_64
        let s_1_65: Bits = ((s_1_63) ^ (s_1_64));
        // D s_1_66: cast reint s_1_65 -> u64
        let s_1_66: u64 = (s_1_65.value() as u64);
        // C s_1_67: const #64s : i
        let s_1_67: i128 = 64;
        // D s_1_68: cast zx s_1_14 -> bv
        let s_1_68: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_69: const #1s : i64
        let s_1_69: i64 = 1;
        // C s_1_70: cast zx s_1_69 -> i
        let s_1_70: i128 = (i128::try_from(s_1_69).unwrap());
        // C s_1_71: const #63s : i
        let s_1_71: i128 = 63;
        // C s_1_72: add s_1_71 s_1_70
        let s_1_72: i128 = (s_1_71 + s_1_70);
        // D s_1_73: bit-extract s_1_68 s_1_67 s_1_72
        let s_1_73: Bits = (Bits::new(
            ((s_1_68) >> (s_1_67)).value(),
            u16::try_from(s_1_72).unwrap(),
        ));
        // D s_1_74: cast reint s_1_73 -> u64
        let s_1_74: u64 = (s_1_73.value() as u64);
        // D s_1_75: cast zx s_1_74 -> bv
        let s_1_75: Bits = Bits::new(s_1_74 as u128, 64u16);
        // D s_1_76: cast zx s_1_66 -> bv
        let s_1_76: Bits = Bits::new(s_1_66 as u128, 64u16);
        // D s_1_77: add s_1_75 s_1_76
        let s_1_77: Bits = (s_1_75 + s_1_76);
        // D s_1_78: cast reint s_1_77 -> u64
        let s_1_78: u64 = (s_1_77.value() as u64);
        // C s_1_79: const #64s : i
        let s_1_79: i128 = 64;
        // D s_1_80: cast zx s_1_9 -> bv
        let s_1_80: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_81: const #1s : i64
        let s_1_81: i64 = 1;
        // C s_1_82: cast zx s_1_81 -> i
        let s_1_82: i128 = (i128::try_from(s_1_81).unwrap());
        // C s_1_83: const #63s : i
        let s_1_83: i128 = 63;
        // C s_1_84: add s_1_83 s_1_82
        let s_1_84: i128 = (s_1_83 + s_1_82);
        // D s_1_85: bit-extract s_1_80 s_1_79 s_1_84
        let s_1_85: Bits = (Bits::new(
            ((s_1_80) >> (s_1_79)).value(),
            u16::try_from(s_1_84).unwrap(),
        ));
        // D s_1_86: cast reint s_1_85 -> u64
        let s_1_86: u64 = (s_1_85.value() as u64);
        // D s_1_87: cast zx s_1_78 -> bv
        let s_1_87: Bits = Bits::new(s_1_78 as u128, 64u16);
        // D s_1_88: cast zx s_1_86 -> bv
        let s_1_88: Bits = Bits::new(s_1_86 as u128, 64u16);
        // D s_1_89: add s_1_87 s_1_88
        let s_1_89: Bits = (s_1_87 + s_1_88);
        // D s_1_90: cast reint s_1_89 -> u64
        let s_1_90: u64 = (s_1_89.value() as u64);
        // C s_1_91: const #64s : i
        let s_1_91: i128 = 64;
        // D s_1_92: read-var Vtmp:u128
        let s_1_92: u128 = fn_state.Vtmp;
        // D s_1_93: cast zx s_1_92 -> bv
        let s_1_93: Bits = Bits::new(s_1_92 as u128, 128u16);
        // D s_1_94: cast zx s_1_90 -> bv
        let s_1_94: Bits = Bits::new(s_1_90 as u128, 64u16);
        // C s_1_95: const #63s : i
        let s_1_95: i128 = 63;
        // C s_1_96: const #1u : u64
        let s_1_96: u64 = 1;
        // C s_1_97: cast zx s_1_96 -> bv
        let s_1_97: Bits = Bits::new(s_1_96 as u128, 64u16);
        // C s_1_98: lsl s_1_97 s_1_95
        let s_1_98: Bits = s_1_97 << s_1_95;
        // C s_1_99: sub s_1_98 s_1_97
        let s_1_99: Bits = ((s_1_98) - (s_1_97));
        // D s_1_100: and s_1_94 s_1_99
        let s_1_100: Bits = ((s_1_94) & (s_1_99));
        // D s_1_101: lsl s_1_100 s_1_91
        let s_1_101: Bits = s_1_100 << s_1_91;
        // C s_1_102: lsl s_1_99 s_1_91
        let s_1_102: Bits = s_1_99 << s_1_91;
        // C s_1_103: cmpl s_1_102
        let s_1_103: Bits = !s_1_102;
        // D s_1_104: and s_1_93 s_1_103
        let s_1_104: Bits = ((s_1_93) & (s_1_103));
        // D s_1_105: or s_1_104 s_1_101
        let s_1_105: Bits = ((s_1_104) | (s_1_101));
        // D s_1_106: cast reint s_1_105 -> u128
        let s_1_106: u128 = (s_1_105.value() as u128);
        // D s_1_107: write-var Vtmp <= s_1_106
        fn_state.Vtmp = s_1_106;
        // C s_1_108: const #0s : i
        let s_1_108: i128 = 0;
        // D s_1_109: cast zx s_1_4 -> bv
        let s_1_109: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_110: const #1s : i64
        let s_1_110: i64 = 1;
        // C s_1_111: cast zx s_1_110 -> i
        let s_1_111: i128 = (i128::try_from(s_1_110).unwrap());
        // C s_1_112: const #63s : i
        let s_1_112: i128 = 63;
        // C s_1_113: add s_1_112 s_1_111
        let s_1_113: i128 = (s_1_112 + s_1_111);
        // D s_1_114: bit-extract s_1_109 s_1_108 s_1_113
        let s_1_114: Bits = (Bits::new(
            ((s_1_109) >> (s_1_108)).value(),
            u16::try_from(s_1_113).unwrap(),
        ));
        // D s_1_115: cast reint s_1_114 -> u64
        let s_1_115: u64 = (s_1_114.value() as u64);
        // C s_1_116: const #19s : i
        let s_1_116: i128 = 19;
        // D s_1_117: cast zx s_1_115 -> bv
        let s_1_117: Bits = Bits::new(s_1_115 as u128, 64u16);
        // D s_1_118: call ROR(s_1_117, s_1_116)
        let s_1_118: Bits = ROR(state, tracer, s_1_117, s_1_116);
        // D s_1_119: cast reint s_1_118 -> u64
        let s_1_119: u64 = (s_1_118.value() as u64);
        // C s_1_120: const #0s : i
        let s_1_120: i128 = 0;
        // D s_1_121: cast zx s_1_4 -> bv
        let s_1_121: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_122: const #1s : i64
        let s_1_122: i64 = 1;
        // C s_1_123: cast zx s_1_122 -> i
        let s_1_123: i128 = (i128::try_from(s_1_122).unwrap());
        // C s_1_124: const #63s : i
        let s_1_124: i128 = 63;
        // C s_1_125: add s_1_124 s_1_123
        let s_1_125: i128 = (s_1_124 + s_1_123);
        // D s_1_126: bit-extract s_1_121 s_1_120 s_1_125
        let s_1_126: Bits = (Bits::new(
            ((s_1_121) >> (s_1_120)).value(),
            u16::try_from(s_1_125).unwrap(),
        ));
        // D s_1_127: cast reint s_1_126 -> u64
        let s_1_127: u64 = (s_1_126.value() as u64);
        // C s_1_128: const #61s : i
        let s_1_128: i128 = 61;
        // D s_1_129: cast zx s_1_127 -> bv
        let s_1_129: Bits = Bits::new(s_1_127 as u128, 64u16);
        // D s_1_130: call ROR(s_1_129, s_1_128)
        let s_1_130: Bits = ROR(state, tracer, s_1_129, s_1_128);
        // D s_1_131: cast reint s_1_130 -> u64
        let s_1_131: u64 = (s_1_130.value() as u64);
        // D s_1_132: cast zx s_1_119 -> bv
        let s_1_132: Bits = Bits::new(s_1_119 as u128, 64u16);
        // D s_1_133: cast zx s_1_131 -> bv
        let s_1_133: Bits = Bits::new(s_1_131 as u128, 64u16);
        // D s_1_134: xor s_1_132 s_1_133
        let s_1_134: Bits = ((s_1_132) ^ (s_1_133));
        // D s_1_135: cast reint s_1_134 -> u64
        let s_1_135: u64 = (s_1_134.value() as u64);
        // C s_1_136: const #6s : i
        let s_1_136: i128 = 6;
        // D s_1_137: cast zx s_1_4 -> bv
        let s_1_137: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_138: const #1s : i64
        let s_1_138: i64 = 1;
        // C s_1_139: cast zx s_1_138 -> i
        let s_1_139: i128 = (i128::try_from(s_1_138).unwrap());
        // C s_1_140: const #57s : i
        let s_1_140: i128 = 57;
        // C s_1_141: add s_1_140 s_1_139
        let s_1_141: i128 = (s_1_140 + s_1_139);
        // D s_1_142: bit-extract s_1_137 s_1_136 s_1_141
        let s_1_142: Bits = (Bits::new(
            ((s_1_137) >> (s_1_136)).value(),
            u16::try_from(s_1_141).unwrap(),
        ));
        // D s_1_143: cast reint s_1_142 -> u58
        let s_1_143: u64 = (s_1_142.value() as u64);
        // C s_1_144: const #0u : u8
        let s_1_144: u8 = 0;
        // C s_1_145: cast zx s_1_144 -> bv
        let s_1_145: Bits = Bits::new(s_1_144 as u128, 6u16);
        // D s_1_146: cast zx s_1_143 -> bv
        let s_1_146: Bits = Bits::new(s_1_143 as u128, 58u16);
        // C s_1_147: cast reint s_1_145 -> u128
        let s_1_147: u128 = (s_1_145.value() as u128);
        // D s_1_148: size-of s_1_145
        let s_1_148: u16 = s_1_145.length();
        // D s_1_149: cast reint s_1_146 -> u128
        let s_1_149: u128 = (s_1_146.value() as u128);
        // D s_1_150: size-of s_1_146
        let s_1_150: u16 = s_1_146.length();
        // D s_1_151: lsl s_1_147 s_1_150
        let s_1_151: u128 = s_1_147 << s_1_150;
        // D s_1_152: or s_1_151 s_1_149
        let s_1_152: u128 = ((s_1_151) | (s_1_149));
        // D s_1_153: add s_1_148 s_1_150
        let s_1_153: u16 = (s_1_148 + s_1_150);
        // D s_1_154: create-bits s_1_152 s_1_153
        let s_1_154: Bits = Bits::new(s_1_152, s_1_153);
        // D s_1_155: cast reint s_1_154 -> u64
        let s_1_155: u64 = (s_1_154.value() as u64);
        // D s_1_156: cast zx s_1_135 -> bv
        let s_1_156: Bits = Bits::new(s_1_135 as u128, 64u16);
        // D s_1_157: cast zx s_1_155 -> bv
        let s_1_157: Bits = Bits::new(s_1_155 as u128, 64u16);
        // D s_1_158: xor s_1_156 s_1_157
        let s_1_158: Bits = ((s_1_156) ^ (s_1_157));
        // D s_1_159: cast reint s_1_158 -> u64
        let s_1_159: u64 = (s_1_158.value() as u64);
        // C s_1_160: const #0s : i
        let s_1_160: i128 = 0;
        // D s_1_161: cast zx s_1_14 -> bv
        let s_1_161: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_162: const #1s : i64
        let s_1_162: i64 = 1;
        // C s_1_163: cast zx s_1_162 -> i
        let s_1_163: i128 = (i128::try_from(s_1_162).unwrap());
        // C s_1_164: const #63s : i
        let s_1_164: i128 = 63;
        // C s_1_165: add s_1_164 s_1_163
        let s_1_165: i128 = (s_1_164 + s_1_163);
        // D s_1_166: bit-extract s_1_161 s_1_160 s_1_165
        let s_1_166: Bits = (Bits::new(
            ((s_1_161) >> (s_1_160)).value(),
            u16::try_from(s_1_165).unwrap(),
        ));
        // D s_1_167: cast reint s_1_166 -> u64
        let s_1_167: u64 = (s_1_166.value() as u64);
        // D s_1_168: cast zx s_1_167 -> bv
        let s_1_168: Bits = Bits::new(s_1_167 as u128, 64u16);
        // D s_1_169: cast zx s_1_159 -> bv
        let s_1_169: Bits = Bits::new(s_1_159 as u128, 64u16);
        // D s_1_170: add s_1_168 s_1_169
        let s_1_170: Bits = (s_1_168 + s_1_169);
        // D s_1_171: cast reint s_1_170 -> u64
        let s_1_171: u64 = (s_1_170.value() as u64);
        // C s_1_172: const #0s : i
        let s_1_172: i128 = 0;
        // D s_1_173: cast zx s_1_9 -> bv
        let s_1_173: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_174: const #1s : i64
        let s_1_174: i64 = 1;
        // C s_1_175: cast zx s_1_174 -> i
        let s_1_175: i128 = (i128::try_from(s_1_174).unwrap());
        // C s_1_176: const #63s : i
        let s_1_176: i128 = 63;
        // C s_1_177: add s_1_176 s_1_175
        let s_1_177: i128 = (s_1_176 + s_1_175);
        // D s_1_178: bit-extract s_1_173 s_1_172 s_1_177
        let s_1_178: Bits = (Bits::new(
            ((s_1_173) >> (s_1_172)).value(),
            u16::try_from(s_1_177).unwrap(),
        ));
        // D s_1_179: cast reint s_1_178 -> u64
        let s_1_179: u64 = (s_1_178.value() as u64);
        // D s_1_180: cast zx s_1_171 -> bv
        let s_1_180: Bits = Bits::new(s_1_171 as u128, 64u16);
        // D s_1_181: cast zx s_1_179 -> bv
        let s_1_181: Bits = Bits::new(s_1_179 as u128, 64u16);
        // D s_1_182: add s_1_180 s_1_181
        let s_1_182: Bits = (s_1_180 + s_1_181);
        // D s_1_183: cast reint s_1_182 -> u64
        let s_1_183: u64 = (s_1_182.value() as u64);
        // C s_1_184: const #0s : i
        let s_1_184: i128 = 0;
        // D s_1_185: cast zx s_1_106 -> bv
        let s_1_185: Bits = Bits::new(s_1_106 as u128, 128u16);
        // D s_1_186: cast zx s_1_183 -> bv
        let s_1_186: Bits = Bits::new(s_1_183 as u128, 64u16);
        // C s_1_187: const #63s : i
        let s_1_187: i128 = 63;
        // C s_1_188: const #1u : u64
        let s_1_188: u64 = 1;
        // C s_1_189: cast zx s_1_188 -> bv
        let s_1_189: Bits = Bits::new(s_1_188 as u128, 64u16);
        // C s_1_190: lsl s_1_189 s_1_187
        let s_1_190: Bits = s_1_189 << s_1_187;
        // C s_1_191: sub s_1_190 s_1_189
        let s_1_191: Bits = ((s_1_190) - (s_1_189));
        // D s_1_192: and s_1_186 s_1_191
        let s_1_192: Bits = ((s_1_186) & (s_1_191));
        // D s_1_193: lsl s_1_192 s_1_184
        let s_1_193: Bits = s_1_192 << s_1_184;
        // C s_1_194: lsl s_1_191 s_1_184
        let s_1_194: Bits = s_1_191 << s_1_184;
        // C s_1_195: cmpl s_1_194
        let s_1_195: Bits = !s_1_194;
        // D s_1_196: and s_1_185 s_1_195
        let s_1_196: Bits = ((s_1_185) & (s_1_195));
        // D s_1_197: or s_1_196 s_1_193
        let s_1_197: Bits = ((s_1_196) | (s_1_193));
        // D s_1_198: cast reint s_1_197 -> u128
        let s_1_198: u128 = (s_1_197.value() as u128);
        // D s_1_199: write-var Vtmp <= s_1_198
        fn_state.Vtmp = s_1_198;
        // C s_1_200: const #128s : i64
        let s_1_200: i64 = 128;
        // D s_1_201: read-var d:i64
        let s_1_201: i64 = fn_state.d;
        // D s_1_202: cast zx s_1_201 -> i
        let s_1_202: i128 = (i128::try_from(s_1_201).unwrap());
        // D s_1_203: cast zx s_1_198 -> bv
        let s_1_203: Bits = Bits::new(s_1_198 as u128, 128u16);
        // D s_1_204: call V_set(s_1_202, s_1_200, s_1_203)
        let s_1_204: () = V_set(state, tracer, s_1_202, s_1_200, s_1_203);
        // N s_1_205: return
        return;
    }
}
