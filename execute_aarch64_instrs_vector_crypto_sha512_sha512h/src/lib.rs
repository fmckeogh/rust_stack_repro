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
pub fn execute_aarch64_instrs_vector_crypto_sha512_sha512h<T: Tracer>(
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
        // D s_1_16: cast zx s_1_9 -> bv
        let s_1_16: Bits = Bits::new(s_1_9 as u128, 128u16);
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
        // C s_1_23: const #14s : i
        let s_1_23: i128 = 14;
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 64u16);
        // D s_1_25: call ROR(s_1_24, s_1_23)
        let s_1_25: Bits = ROR(state, tracer, s_1_24, s_1_23);
        // D s_1_26: cast reint s_1_25 -> u64
        let s_1_26: u64 = (s_1_25.value() as u64);
        // C s_1_27: const #64s : i
        let s_1_27: i128 = 64;
        // D s_1_28: cast zx s_1_9 -> bv
        let s_1_28: Bits = Bits::new(s_1_9 as u128, 128u16);
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
        // C s_1_35: const #18s : i
        let s_1_35: i128 = 18;
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
        // C s_1_43: const #64s : i
        let s_1_43: i128 = 64;
        // D s_1_44: cast zx s_1_9 -> bv
        let s_1_44: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_45: const #1s : i64
        let s_1_45: i64 = 1;
        // C s_1_46: cast zx s_1_45 -> i
        let s_1_46: i128 = (i128::try_from(s_1_45).unwrap());
        // C s_1_47: const #63s : i
        let s_1_47: i128 = 63;
        // C s_1_48: add s_1_47 s_1_46
        let s_1_48: i128 = (s_1_47 + s_1_46);
        // D s_1_49: bit-extract s_1_44 s_1_43 s_1_48
        let s_1_49: Bits = (Bits::new(
            ((s_1_44) >> (s_1_43)).value(),
            u16::try_from(s_1_48).unwrap(),
        ));
        // D s_1_50: cast reint s_1_49 -> u64
        let s_1_50: u64 = (s_1_49.value() as u64);
        // C s_1_51: const #41s : i
        let s_1_51: i128 = 41;
        // D s_1_52: cast zx s_1_50 -> bv
        let s_1_52: Bits = Bits::new(s_1_50 as u128, 64u16);
        // D s_1_53: call ROR(s_1_52, s_1_51)
        let s_1_53: Bits = ROR(state, tracer, s_1_52, s_1_51);
        // D s_1_54: cast reint s_1_53 -> u64
        let s_1_54: u64 = (s_1_53.value() as u64);
        // D s_1_55: cast zx s_1_42 -> bv
        let s_1_55: Bits = Bits::new(s_1_42 as u128, 64u16);
        // D s_1_56: cast zx s_1_54 -> bv
        let s_1_56: Bits = Bits::new(s_1_54 as u128, 64u16);
        // D s_1_57: xor s_1_55 s_1_56
        let s_1_57: Bits = ((s_1_55) ^ (s_1_56));
        // D s_1_58: cast reint s_1_57 -> u64
        let s_1_58: u64 = (s_1_57.value() as u64);
        // C s_1_59: const #64s : i
        let s_1_59: i128 = 64;
        // D s_1_60: cast zx s_1_9 -> bv
        let s_1_60: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_61: const #1s : i64
        let s_1_61: i64 = 1;
        // C s_1_62: cast zx s_1_61 -> i
        let s_1_62: i128 = (i128::try_from(s_1_61).unwrap());
        // C s_1_63: const #63s : i
        let s_1_63: i128 = 63;
        // C s_1_64: add s_1_63 s_1_62
        let s_1_64: i128 = (s_1_63 + s_1_62);
        // D s_1_65: bit-extract s_1_60 s_1_59 s_1_64
        let s_1_65: Bits = (Bits::new(
            ((s_1_60) >> (s_1_59)).value(),
            u16::try_from(s_1_64).unwrap(),
        ));
        // D s_1_66: cast reint s_1_65 -> u64
        let s_1_66: u64 = (s_1_65.value() as u64);
        // C s_1_67: const #0s : i
        let s_1_67: i128 = 0;
        // D s_1_68: cast zx s_1_4 -> bv
        let s_1_68: Bits = Bits::new(s_1_4 as u128, 128u16);
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
        // D s_1_75: cast zx s_1_66 -> bv
        let s_1_75: Bits = Bits::new(s_1_66 as u128, 64u16);
        // D s_1_76: cast zx s_1_74 -> bv
        let s_1_76: Bits = Bits::new(s_1_74 as u128, 64u16);
        // D s_1_77: and s_1_75 s_1_76
        let s_1_77: Bits = ((s_1_75) & (s_1_76));
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
        // D s_1_87: cast zx s_1_86 -> bv
        let s_1_87: Bits = Bits::new(s_1_86 as u128, 64u16);
        // D s_1_88: not s_1_87
        let s_1_88: Bits = !s_1_87;
        // D s_1_89: cast reint s_1_88 -> u64
        let s_1_89: u64 = (s_1_88.value() as u64);
        // C s_1_90: const #64s : i
        let s_1_90: i128 = 64;
        // D s_1_91: cast zx s_1_4 -> bv
        let s_1_91: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_92: const #1s : i64
        let s_1_92: i64 = 1;
        // C s_1_93: cast zx s_1_92 -> i
        let s_1_93: i128 = (i128::try_from(s_1_92).unwrap());
        // C s_1_94: const #63s : i
        let s_1_94: i128 = 63;
        // C s_1_95: add s_1_94 s_1_93
        let s_1_95: i128 = (s_1_94 + s_1_93);
        // D s_1_96: bit-extract s_1_91 s_1_90 s_1_95
        let s_1_96: Bits = (Bits::new(
            ((s_1_91) >> (s_1_90)).value(),
            u16::try_from(s_1_95).unwrap(),
        ));
        // D s_1_97: cast reint s_1_96 -> u64
        let s_1_97: u64 = (s_1_96.value() as u64);
        // D s_1_98: cast zx s_1_89 -> bv
        let s_1_98: Bits = Bits::new(s_1_89 as u128, 64u16);
        // D s_1_99: cast zx s_1_97 -> bv
        let s_1_99: Bits = Bits::new(s_1_97 as u128, 64u16);
        // D s_1_100: and s_1_98 s_1_99
        let s_1_100: Bits = ((s_1_98) & (s_1_99));
        // D s_1_101: cast reint s_1_100 -> u64
        let s_1_101: u64 = (s_1_100.value() as u64);
        // D s_1_102: cast zx s_1_78 -> bv
        let s_1_102: Bits = Bits::new(s_1_78 as u128, 64u16);
        // D s_1_103: cast zx s_1_101 -> bv
        let s_1_103: Bits = Bits::new(s_1_101 as u128, 64u16);
        // D s_1_104: xor s_1_102 s_1_103
        let s_1_104: Bits = ((s_1_102) ^ (s_1_103));
        // D s_1_105: cast reint s_1_104 -> u64
        let s_1_105: u64 = (s_1_104.value() as u64);
        // C s_1_106: const #64s : i
        let s_1_106: i128 = 64;
        // D s_1_107: read-var Vtmp:u128
        let s_1_107: u128 = fn_state.Vtmp;
        // D s_1_108: cast zx s_1_107 -> bv
        let s_1_108: Bits = Bits::new(s_1_107 as u128, 128u16);
        // D s_1_109: cast zx s_1_105 -> bv
        let s_1_109: Bits = Bits::new(s_1_105 as u128, 64u16);
        // C s_1_110: const #63s : i
        let s_1_110: i128 = 63;
        // C s_1_111: const #1u : u64
        let s_1_111: u64 = 1;
        // C s_1_112: cast zx s_1_111 -> bv
        let s_1_112: Bits = Bits::new(s_1_111 as u128, 64u16);
        // C s_1_113: lsl s_1_112 s_1_110
        let s_1_113: Bits = s_1_112 << s_1_110;
        // C s_1_114: sub s_1_113 s_1_112
        let s_1_114: Bits = ((s_1_113) - (s_1_112));
        // D s_1_115: and s_1_109 s_1_114
        let s_1_115: Bits = ((s_1_109) & (s_1_114));
        // D s_1_116: lsl s_1_115 s_1_106
        let s_1_116: Bits = s_1_115 << s_1_106;
        // C s_1_117: lsl s_1_114 s_1_106
        let s_1_117: Bits = s_1_114 << s_1_106;
        // C s_1_118: cmpl s_1_117
        let s_1_118: Bits = !s_1_117;
        // D s_1_119: and s_1_108 s_1_118
        let s_1_119: Bits = ((s_1_108) & (s_1_118));
        // D s_1_120: or s_1_119 s_1_116
        let s_1_120: Bits = ((s_1_119) | (s_1_116));
        // D s_1_121: cast reint s_1_120 -> u128
        let s_1_121: u128 = (s_1_120.value() as u128);
        // D s_1_122: write-var Vtmp <= s_1_121
        fn_state.Vtmp = s_1_121;
        // C s_1_123: const #64s : i
        let s_1_123: i128 = 64;
        // D s_1_124: cast zx s_1_121 -> bv
        let s_1_124: Bits = Bits::new(s_1_121 as u128, 128u16);
        // C s_1_125: const #1s : i64
        let s_1_125: i64 = 1;
        // C s_1_126: cast zx s_1_125 -> i
        let s_1_126: i128 = (i128::try_from(s_1_125).unwrap());
        // C s_1_127: const #63s : i
        let s_1_127: i128 = 63;
        // C s_1_128: add s_1_127 s_1_126
        let s_1_128: i128 = (s_1_127 + s_1_126);
        // D s_1_129: bit-extract s_1_124 s_1_123 s_1_128
        let s_1_129: Bits = (Bits::new(
            ((s_1_124) >> (s_1_123)).value(),
            u16::try_from(s_1_128).unwrap(),
        ));
        // D s_1_130: cast reint s_1_129 -> u64
        let s_1_130: u64 = (s_1_129.value() as u64);
        // D s_1_131: cast zx s_1_130 -> bv
        let s_1_131: Bits = Bits::new(s_1_130 as u128, 64u16);
        // D s_1_132: cast zx s_1_58 -> bv
        let s_1_132: Bits = Bits::new(s_1_58 as u128, 64u16);
        // D s_1_133: add s_1_131 s_1_132
        let s_1_133: Bits = (s_1_131 + s_1_132);
        // D s_1_134: cast reint s_1_133 -> u64
        let s_1_134: u64 = (s_1_133.value() as u64);
        // C s_1_135: const #64s : i
        let s_1_135: i128 = 64;
        // D s_1_136: cast zx s_1_14 -> bv
        let s_1_136: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_137: const #1s : i64
        let s_1_137: i64 = 1;
        // C s_1_138: cast zx s_1_137 -> i
        let s_1_138: i128 = (i128::try_from(s_1_137).unwrap());
        // C s_1_139: const #63s : i
        let s_1_139: i128 = 63;
        // C s_1_140: add s_1_139 s_1_138
        let s_1_140: i128 = (s_1_139 + s_1_138);
        // D s_1_141: bit-extract s_1_136 s_1_135 s_1_140
        let s_1_141: Bits = (Bits::new(
            ((s_1_136) >> (s_1_135)).value(),
            u16::try_from(s_1_140).unwrap(),
        ));
        // D s_1_142: cast reint s_1_141 -> u64
        let s_1_142: u64 = (s_1_141.value() as u64);
        // D s_1_143: cast zx s_1_134 -> bv
        let s_1_143: Bits = Bits::new(s_1_134 as u128, 64u16);
        // D s_1_144: cast zx s_1_142 -> bv
        let s_1_144: Bits = Bits::new(s_1_142 as u128, 64u16);
        // D s_1_145: add s_1_143 s_1_144
        let s_1_145: Bits = (s_1_143 + s_1_144);
        // D s_1_146: cast reint s_1_145 -> u64
        let s_1_146: u64 = (s_1_145.value() as u64);
        // C s_1_147: const #64s : i
        let s_1_147: i128 = 64;
        // D s_1_148: cast zx s_1_121 -> bv
        let s_1_148: Bits = Bits::new(s_1_121 as u128, 128u16);
        // D s_1_149: cast zx s_1_146 -> bv
        let s_1_149: Bits = Bits::new(s_1_146 as u128, 64u16);
        // C s_1_150: const #63s : i
        let s_1_150: i128 = 63;
        // C s_1_151: const #1u : u64
        let s_1_151: u64 = 1;
        // C s_1_152: cast zx s_1_151 -> bv
        let s_1_152: Bits = Bits::new(s_1_151 as u128, 64u16);
        // C s_1_153: lsl s_1_152 s_1_150
        let s_1_153: Bits = s_1_152 << s_1_150;
        // C s_1_154: sub s_1_153 s_1_152
        let s_1_154: Bits = ((s_1_153) - (s_1_152));
        // D s_1_155: and s_1_149 s_1_154
        let s_1_155: Bits = ((s_1_149) & (s_1_154));
        // D s_1_156: lsl s_1_155 s_1_147
        let s_1_156: Bits = s_1_155 << s_1_147;
        // C s_1_157: lsl s_1_154 s_1_147
        let s_1_157: Bits = s_1_154 << s_1_147;
        // C s_1_158: cmpl s_1_157
        let s_1_158: Bits = !s_1_157;
        // D s_1_159: and s_1_148 s_1_158
        let s_1_159: Bits = ((s_1_148) & (s_1_158));
        // D s_1_160: or s_1_159 s_1_156
        let s_1_160: Bits = ((s_1_159) | (s_1_156));
        // D s_1_161: cast reint s_1_160 -> u128
        let s_1_161: u128 = (s_1_160.value() as u128);
        // D s_1_162: write-var Vtmp <= s_1_161
        fn_state.Vtmp = s_1_161;
        // C s_1_163: const #64s : i
        let s_1_163: i128 = 64;
        // D s_1_164: cast zx s_1_161 -> bv
        let s_1_164: Bits = Bits::new(s_1_161 as u128, 128u16);
        // C s_1_165: const #1s : i64
        let s_1_165: i64 = 1;
        // C s_1_166: cast zx s_1_165 -> i
        let s_1_166: i128 = (i128::try_from(s_1_165).unwrap());
        // C s_1_167: const #63s : i
        let s_1_167: i128 = 63;
        // C s_1_168: add s_1_167 s_1_166
        let s_1_168: i128 = (s_1_167 + s_1_166);
        // D s_1_169: bit-extract s_1_164 s_1_163 s_1_168
        let s_1_169: Bits = (Bits::new(
            ((s_1_164) >> (s_1_163)).value(),
            u16::try_from(s_1_168).unwrap(),
        ));
        // D s_1_170: cast reint s_1_169 -> u64
        let s_1_170: u64 = (s_1_169.value() as u64);
        // C s_1_171: const #0s : i
        let s_1_171: i128 = 0;
        // D s_1_172: cast zx s_1_9 -> bv
        let s_1_172: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_173: const #1s : i64
        let s_1_173: i64 = 1;
        // C s_1_174: cast zx s_1_173 -> i
        let s_1_174: i128 = (i128::try_from(s_1_173).unwrap());
        // C s_1_175: const #63s : i
        let s_1_175: i128 = 63;
        // C s_1_176: add s_1_175 s_1_174
        let s_1_176: i128 = (s_1_175 + s_1_174);
        // D s_1_177: bit-extract s_1_172 s_1_171 s_1_176
        let s_1_177: Bits = (Bits::new(
            ((s_1_172) >> (s_1_171)).value(),
            u16::try_from(s_1_176).unwrap(),
        ));
        // D s_1_178: cast reint s_1_177 -> u64
        let s_1_178: u64 = (s_1_177.value() as u64);
        // D s_1_179: cast zx s_1_170 -> bv
        let s_1_179: Bits = Bits::new(s_1_170 as u128, 64u16);
        // D s_1_180: cast zx s_1_178 -> bv
        let s_1_180: Bits = Bits::new(s_1_178 as u128, 64u16);
        // D s_1_181: add s_1_179 s_1_180
        let s_1_181: Bits = (s_1_179 + s_1_180);
        // D s_1_182: cast reint s_1_181 -> u64
        let s_1_182: u64 = (s_1_181.value() as u64);
        // C s_1_183: const #14s : i
        let s_1_183: i128 = 14;
        // D s_1_184: cast zx s_1_182 -> bv
        let s_1_184: Bits = Bits::new(s_1_182 as u128, 64u16);
        // D s_1_185: call ROR(s_1_184, s_1_183)
        let s_1_185: Bits = ROR(state, tracer, s_1_184, s_1_183);
        // D s_1_186: cast reint s_1_185 -> u64
        let s_1_186: u64 = (s_1_185.value() as u64);
        // C s_1_187: const #18s : i
        let s_1_187: i128 = 18;
        // D s_1_188: cast zx s_1_182 -> bv
        let s_1_188: Bits = Bits::new(s_1_182 as u128, 64u16);
        // D s_1_189: call ROR(s_1_188, s_1_187)
        let s_1_189: Bits = ROR(state, tracer, s_1_188, s_1_187);
        // D s_1_190: cast reint s_1_189 -> u64
        let s_1_190: u64 = (s_1_189.value() as u64);
        // D s_1_191: cast zx s_1_186 -> bv
        let s_1_191: Bits = Bits::new(s_1_186 as u128, 64u16);
        // D s_1_192: cast zx s_1_190 -> bv
        let s_1_192: Bits = Bits::new(s_1_190 as u128, 64u16);
        // D s_1_193: xor s_1_191 s_1_192
        let s_1_193: Bits = ((s_1_191) ^ (s_1_192));
        // D s_1_194: cast reint s_1_193 -> u64
        let s_1_194: u64 = (s_1_193.value() as u64);
        // C s_1_195: const #41s : i
        let s_1_195: i128 = 41;
        // D s_1_196: cast zx s_1_182 -> bv
        let s_1_196: Bits = Bits::new(s_1_182 as u128, 64u16);
        // D s_1_197: call ROR(s_1_196, s_1_195)
        let s_1_197: Bits = ROR(state, tracer, s_1_196, s_1_195);
        // D s_1_198: cast reint s_1_197 -> u64
        let s_1_198: u64 = (s_1_197.value() as u64);
        // D s_1_199: cast zx s_1_194 -> bv
        let s_1_199: Bits = Bits::new(s_1_194 as u128, 64u16);
        // D s_1_200: cast zx s_1_198 -> bv
        let s_1_200: Bits = Bits::new(s_1_198 as u128, 64u16);
        // D s_1_201: xor s_1_199 s_1_200
        let s_1_201: Bits = ((s_1_199) ^ (s_1_200));
        // D s_1_202: cast reint s_1_201 -> u64
        let s_1_202: u64 = (s_1_201.value() as u64);
        // C s_1_203: const #64s : i
        let s_1_203: i128 = 64;
        // D s_1_204: cast zx s_1_9 -> bv
        let s_1_204: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_205: const #1s : i64
        let s_1_205: i64 = 1;
        // C s_1_206: cast zx s_1_205 -> i
        let s_1_206: i128 = (i128::try_from(s_1_205).unwrap());
        // C s_1_207: const #63s : i
        let s_1_207: i128 = 63;
        // C s_1_208: add s_1_207 s_1_206
        let s_1_208: i128 = (s_1_207 + s_1_206);
        // D s_1_209: bit-extract s_1_204 s_1_203 s_1_208
        let s_1_209: Bits = (Bits::new(
            ((s_1_204) >> (s_1_203)).value(),
            u16::try_from(s_1_208).unwrap(),
        ));
        // D s_1_210: cast reint s_1_209 -> u64
        let s_1_210: u64 = (s_1_209.value() as u64);
        // D s_1_211: cast zx s_1_182 -> bv
        let s_1_211: Bits = Bits::new(s_1_182 as u128, 64u16);
        // D s_1_212: cast zx s_1_210 -> bv
        let s_1_212: Bits = Bits::new(s_1_210 as u128, 64u16);
        // D s_1_213: and s_1_211 s_1_212
        let s_1_213: Bits = ((s_1_211) & (s_1_212));
        // D s_1_214: cast reint s_1_213 -> u64
        let s_1_214: u64 = (s_1_213.value() as u64);
        // D s_1_215: cast zx s_1_182 -> bv
        let s_1_215: Bits = Bits::new(s_1_182 as u128, 64u16);
        // D s_1_216: not s_1_215
        let s_1_216: Bits = !s_1_215;
        // D s_1_217: cast reint s_1_216 -> u64
        let s_1_217: u64 = (s_1_216.value() as u64);
        // C s_1_218: const #0s : i
        let s_1_218: i128 = 0;
        // D s_1_219: cast zx s_1_4 -> bv
        let s_1_219: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_220: const #1s : i64
        let s_1_220: i64 = 1;
        // C s_1_221: cast zx s_1_220 -> i
        let s_1_221: i128 = (i128::try_from(s_1_220).unwrap());
        // C s_1_222: const #63s : i
        let s_1_222: i128 = 63;
        // C s_1_223: add s_1_222 s_1_221
        let s_1_223: i128 = (s_1_222 + s_1_221);
        // D s_1_224: bit-extract s_1_219 s_1_218 s_1_223
        let s_1_224: Bits = (Bits::new(
            ((s_1_219) >> (s_1_218)).value(),
            u16::try_from(s_1_223).unwrap(),
        ));
        // D s_1_225: cast reint s_1_224 -> u64
        let s_1_225: u64 = (s_1_224.value() as u64);
        // D s_1_226: cast zx s_1_217 -> bv
        let s_1_226: Bits = Bits::new(s_1_217 as u128, 64u16);
        // D s_1_227: cast zx s_1_225 -> bv
        let s_1_227: Bits = Bits::new(s_1_225 as u128, 64u16);
        // D s_1_228: and s_1_226 s_1_227
        let s_1_228: Bits = ((s_1_226) & (s_1_227));
        // D s_1_229: cast reint s_1_228 -> u64
        let s_1_229: u64 = (s_1_228.value() as u64);
        // D s_1_230: cast zx s_1_214 -> bv
        let s_1_230: Bits = Bits::new(s_1_214 as u128, 64u16);
        // D s_1_231: cast zx s_1_229 -> bv
        let s_1_231: Bits = Bits::new(s_1_229 as u128, 64u16);
        // D s_1_232: xor s_1_230 s_1_231
        let s_1_232: Bits = ((s_1_230) ^ (s_1_231));
        // D s_1_233: cast reint s_1_232 -> u64
        let s_1_233: u64 = (s_1_232.value() as u64);
        // C s_1_234: const #0s : i
        let s_1_234: i128 = 0;
        // D s_1_235: cast zx s_1_161 -> bv
        let s_1_235: Bits = Bits::new(s_1_161 as u128, 128u16);
        // D s_1_236: cast zx s_1_233 -> bv
        let s_1_236: Bits = Bits::new(s_1_233 as u128, 64u16);
        // C s_1_237: const #63s : i
        let s_1_237: i128 = 63;
        // C s_1_238: const #1u : u64
        let s_1_238: u64 = 1;
        // C s_1_239: cast zx s_1_238 -> bv
        let s_1_239: Bits = Bits::new(s_1_238 as u128, 64u16);
        // C s_1_240: lsl s_1_239 s_1_237
        let s_1_240: Bits = s_1_239 << s_1_237;
        // C s_1_241: sub s_1_240 s_1_239
        let s_1_241: Bits = ((s_1_240) - (s_1_239));
        // D s_1_242: and s_1_236 s_1_241
        let s_1_242: Bits = ((s_1_236) & (s_1_241));
        // D s_1_243: lsl s_1_242 s_1_234
        let s_1_243: Bits = s_1_242 << s_1_234;
        // C s_1_244: lsl s_1_241 s_1_234
        let s_1_244: Bits = s_1_241 << s_1_234;
        // C s_1_245: cmpl s_1_244
        let s_1_245: Bits = !s_1_244;
        // D s_1_246: and s_1_235 s_1_245
        let s_1_246: Bits = ((s_1_235) & (s_1_245));
        // D s_1_247: or s_1_246 s_1_243
        let s_1_247: Bits = ((s_1_246) | (s_1_243));
        // D s_1_248: cast reint s_1_247 -> u128
        let s_1_248: u128 = (s_1_247.value() as u128);
        // D s_1_249: write-var Vtmp <= s_1_248
        fn_state.Vtmp = s_1_248;
        // C s_1_250: const #0s : i
        let s_1_250: i128 = 0;
        // D s_1_251: cast zx s_1_248 -> bv
        let s_1_251: Bits = Bits::new(s_1_248 as u128, 128u16);
        // C s_1_252: const #1s : i64
        let s_1_252: i64 = 1;
        // C s_1_253: cast zx s_1_252 -> i
        let s_1_253: i128 = (i128::try_from(s_1_252).unwrap());
        // C s_1_254: const #63s : i
        let s_1_254: i128 = 63;
        // C s_1_255: add s_1_254 s_1_253
        let s_1_255: i128 = (s_1_254 + s_1_253);
        // D s_1_256: bit-extract s_1_251 s_1_250 s_1_255
        let s_1_256: Bits = (Bits::new(
            ((s_1_251) >> (s_1_250)).value(),
            u16::try_from(s_1_255).unwrap(),
        ));
        // D s_1_257: cast reint s_1_256 -> u64
        let s_1_257: u64 = (s_1_256.value() as u64);
        // D s_1_258: cast zx s_1_257 -> bv
        let s_1_258: Bits = Bits::new(s_1_257 as u128, 64u16);
        // D s_1_259: cast zx s_1_202 -> bv
        let s_1_259: Bits = Bits::new(s_1_202 as u128, 64u16);
        // D s_1_260: add s_1_258 s_1_259
        let s_1_260: Bits = (s_1_258 + s_1_259);
        // D s_1_261: cast reint s_1_260 -> u64
        let s_1_261: u64 = (s_1_260.value() as u64);
        // C s_1_262: const #0s : i
        let s_1_262: i128 = 0;
        // D s_1_263: cast zx s_1_14 -> bv
        let s_1_263: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_264: const #1s : i64
        let s_1_264: i64 = 1;
        // C s_1_265: cast zx s_1_264 -> i
        let s_1_265: i128 = (i128::try_from(s_1_264).unwrap());
        // C s_1_266: const #63s : i
        let s_1_266: i128 = 63;
        // C s_1_267: add s_1_266 s_1_265
        let s_1_267: i128 = (s_1_266 + s_1_265);
        // D s_1_268: bit-extract s_1_263 s_1_262 s_1_267
        let s_1_268: Bits = (Bits::new(
            ((s_1_263) >> (s_1_262)).value(),
            u16::try_from(s_1_267).unwrap(),
        ));
        // D s_1_269: cast reint s_1_268 -> u64
        let s_1_269: u64 = (s_1_268.value() as u64);
        // D s_1_270: cast zx s_1_261 -> bv
        let s_1_270: Bits = Bits::new(s_1_261 as u128, 64u16);
        // D s_1_271: cast zx s_1_269 -> bv
        let s_1_271: Bits = Bits::new(s_1_269 as u128, 64u16);
        // D s_1_272: add s_1_270 s_1_271
        let s_1_272: Bits = (s_1_270 + s_1_271);
        // D s_1_273: cast reint s_1_272 -> u64
        let s_1_273: u64 = (s_1_272.value() as u64);
        // C s_1_274: const #0s : i
        let s_1_274: i128 = 0;
        // D s_1_275: cast zx s_1_248 -> bv
        let s_1_275: Bits = Bits::new(s_1_248 as u128, 128u16);
        // D s_1_276: cast zx s_1_273 -> bv
        let s_1_276: Bits = Bits::new(s_1_273 as u128, 64u16);
        // C s_1_277: const #63s : i
        let s_1_277: i128 = 63;
        // C s_1_278: const #1u : u64
        let s_1_278: u64 = 1;
        // C s_1_279: cast zx s_1_278 -> bv
        let s_1_279: Bits = Bits::new(s_1_278 as u128, 64u16);
        // C s_1_280: lsl s_1_279 s_1_277
        let s_1_280: Bits = s_1_279 << s_1_277;
        // C s_1_281: sub s_1_280 s_1_279
        let s_1_281: Bits = ((s_1_280) - (s_1_279));
        // D s_1_282: and s_1_276 s_1_281
        let s_1_282: Bits = ((s_1_276) & (s_1_281));
        // D s_1_283: lsl s_1_282 s_1_274
        let s_1_283: Bits = s_1_282 << s_1_274;
        // C s_1_284: lsl s_1_281 s_1_274
        let s_1_284: Bits = s_1_281 << s_1_274;
        // C s_1_285: cmpl s_1_284
        let s_1_285: Bits = !s_1_284;
        // D s_1_286: and s_1_275 s_1_285
        let s_1_286: Bits = ((s_1_275) & (s_1_285));
        // D s_1_287: or s_1_286 s_1_283
        let s_1_287: Bits = ((s_1_286) | (s_1_283));
        // D s_1_288: cast reint s_1_287 -> u128
        let s_1_288: u128 = (s_1_287.value() as u128);
        // D s_1_289: write-var Vtmp <= s_1_288
        fn_state.Vtmp = s_1_288;
        // C s_1_290: const #128s : i64
        let s_1_290: i64 = 128;
        // D s_1_291: read-var d:i64
        let s_1_291: i64 = fn_state.d;
        // D s_1_292: cast zx s_1_291 -> i
        let s_1_292: i128 = (i128::try_from(s_1_291).unwrap());
        // D s_1_293: cast zx s_1_288 -> bv
        let s_1_293: Bits = Bits::new(s_1_288 as u128, 128u16);
        // D s_1_294: call V_set(s_1_292, s_1_290, s_1_293)
        let s_1_294: () = V_set(state, tracer, s_1_292, s_1_290, s_1_293);
        // N s_1_295: return
        return;
    }
}
