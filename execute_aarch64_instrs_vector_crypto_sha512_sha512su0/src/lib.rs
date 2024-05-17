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
pub fn execute_aarch64_instrs_vector_crypto_sha512_sha512su0<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Vtmp: u128,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // D s_1_6: read-var d:i64
        let s_1_6: i64 = fn_state.d;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // C s_1_10: const #64s : i
        let s_1_10: i128 = 64;
        // D s_1_11: cast zx s_1_9 -> bv
        let s_1_11: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_12: const #1s : i64
        let s_1_12: i64 = 1;
        // C s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // C s_1_14: const #63s : i
        let s_1_14: i128 = 63;
        // C s_1_15: add s_1_14 s_1_13
        let s_1_15: i128 = (s_1_14 + s_1_13);
        // D s_1_16: bit-extract s_1_11 s_1_10 s_1_15
        let s_1_16: Bits = (Bits::new(
            ((s_1_11) >> (s_1_10)).value(),
            u16::try_from(s_1_15).unwrap(),
        ));
        // D s_1_17: cast reint s_1_16 -> u64
        let s_1_17: u64 = (s_1_16.value() as u64);
        // C s_1_18: const #1s : i
        let s_1_18: i128 = 1;
        // D s_1_19: cast zx s_1_17 -> bv
        let s_1_19: Bits = Bits::new(s_1_17 as u128, 64u16);
        // D s_1_20: call ROR(s_1_19, s_1_18)
        let s_1_20: Bits = ROR(state, tracer, s_1_19, s_1_18);
        // D s_1_21: cast reint s_1_20 -> u64
        let s_1_21: u64 = (s_1_20.value() as u64);
        // C s_1_22: const #64s : i
        let s_1_22: i128 = 64;
        // D s_1_23: cast zx s_1_9 -> bv
        let s_1_23: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_24: const #1s : i64
        let s_1_24: i64 = 1;
        // C s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // C s_1_26: const #63s : i
        let s_1_26: i128 = 63;
        // C s_1_27: add s_1_26 s_1_25
        let s_1_27: i128 = (s_1_26 + s_1_25);
        // D s_1_28: bit-extract s_1_23 s_1_22 s_1_27
        let s_1_28: Bits = (Bits::new(
            ((s_1_23) >> (s_1_22)).value(),
            u16::try_from(s_1_27).unwrap(),
        ));
        // D s_1_29: cast reint s_1_28 -> u64
        let s_1_29: u64 = (s_1_28.value() as u64);
        // C s_1_30: const #8s : i
        let s_1_30: i128 = 8;
        // D s_1_31: cast zx s_1_29 -> bv
        let s_1_31: Bits = Bits::new(s_1_29 as u128, 64u16);
        // D s_1_32: call ROR(s_1_31, s_1_30)
        let s_1_32: Bits = ROR(state, tracer, s_1_31, s_1_30);
        // D s_1_33: cast reint s_1_32 -> u64
        let s_1_33: u64 = (s_1_32.value() as u64);
        // D s_1_34: cast zx s_1_21 -> bv
        let s_1_34: Bits = Bits::new(s_1_21 as u128, 64u16);
        // D s_1_35: cast zx s_1_33 -> bv
        let s_1_35: Bits = Bits::new(s_1_33 as u128, 64u16);
        // D s_1_36: xor s_1_34 s_1_35
        let s_1_36: Bits = ((s_1_34) ^ (s_1_35));
        // D s_1_37: cast reint s_1_36 -> u64
        let s_1_37: u64 = (s_1_36.value() as u64);
        // C s_1_38: const #71s : i
        let s_1_38: i128 = 71;
        // D s_1_39: cast zx s_1_9 -> bv
        let s_1_39: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_40: const #1s : i64
        let s_1_40: i64 = 1;
        // C s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // C s_1_42: const #56s : i
        let s_1_42: i128 = 56;
        // C s_1_43: add s_1_42 s_1_41
        let s_1_43: i128 = (s_1_42 + s_1_41);
        // D s_1_44: bit-extract s_1_39 s_1_38 s_1_43
        let s_1_44: Bits = (Bits::new(
            ((s_1_39) >> (s_1_38)).value(),
            u16::try_from(s_1_43).unwrap(),
        ));
        // D s_1_45: cast reint s_1_44 -> u57
        let s_1_45: u64 = (s_1_44.value() as u64);
        // C s_1_46: const #0u : u8
        let s_1_46: u8 = 0;
        // C s_1_47: cast zx s_1_46 -> bv
        let s_1_47: Bits = Bits::new(s_1_46 as u128, 7u16);
        // D s_1_48: cast zx s_1_45 -> bv
        let s_1_48: Bits = Bits::new(s_1_45 as u128, 57u16);
        // C s_1_49: cast reint s_1_47 -> u128
        let s_1_49: u128 = (s_1_47.value() as u128);
        // D s_1_50: size-of s_1_47
        let s_1_50: u16 = s_1_47.length();
        // D s_1_51: cast reint s_1_48 -> u128
        let s_1_51: u128 = (s_1_48.value() as u128);
        // D s_1_52: size-of s_1_48
        let s_1_52: u16 = s_1_48.length();
        // D s_1_53: lsl s_1_49 s_1_52
        let s_1_53: u128 = s_1_49 << s_1_52;
        // D s_1_54: or s_1_53 s_1_51
        let s_1_54: u128 = ((s_1_53) | (s_1_51));
        // D s_1_55: add s_1_50 s_1_52
        let s_1_55: u16 = (s_1_50 + s_1_52);
        // D s_1_56: create-bits s_1_54 s_1_55
        let s_1_56: Bits = Bits::new(s_1_54, s_1_55);
        // D s_1_57: cast reint s_1_56 -> u64
        let s_1_57: u64 = (s_1_56.value() as u64);
        // D s_1_58: cast zx s_1_37 -> bv
        let s_1_58: Bits = Bits::new(s_1_37 as u128, 64u16);
        // D s_1_59: cast zx s_1_57 -> bv
        let s_1_59: Bits = Bits::new(s_1_57 as u128, 64u16);
        // D s_1_60: xor s_1_58 s_1_59
        let s_1_60: Bits = ((s_1_58) ^ (s_1_59));
        // D s_1_61: cast reint s_1_60 -> u64
        let s_1_61: u64 = (s_1_60.value() as u64);
        // C s_1_62: const #0s : i
        let s_1_62: i128 = 0;
        // D s_1_63: cast zx s_1_9 -> bv
        let s_1_63: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_64: const #1s : i64
        let s_1_64: i64 = 1;
        // C s_1_65: cast zx s_1_64 -> i
        let s_1_65: i128 = (i128::try_from(s_1_64).unwrap());
        // C s_1_66: const #63s : i
        let s_1_66: i128 = 63;
        // C s_1_67: add s_1_66 s_1_65
        let s_1_67: i128 = (s_1_66 + s_1_65);
        // D s_1_68: bit-extract s_1_63 s_1_62 s_1_67
        let s_1_68: Bits = (Bits::new(
            ((s_1_63) >> (s_1_62)).value(),
            u16::try_from(s_1_67).unwrap(),
        ));
        // D s_1_69: cast reint s_1_68 -> u64
        let s_1_69: u64 = (s_1_68.value() as u64);
        // D s_1_70: cast zx s_1_69 -> bv
        let s_1_70: Bits = Bits::new(s_1_69 as u128, 64u16);
        // D s_1_71: cast zx s_1_61 -> bv
        let s_1_71: Bits = Bits::new(s_1_61 as u128, 64u16);
        // D s_1_72: add s_1_70 s_1_71
        let s_1_72: Bits = (s_1_70 + s_1_71);
        // D s_1_73: cast reint s_1_72 -> u64
        let s_1_73: u64 = (s_1_72.value() as u64);
        // C s_1_74: const #0s : i
        let s_1_74: i128 = 0;
        // D s_1_75: read-var Vtmp:u128
        let s_1_75: u128 = fn_state.Vtmp;
        // D s_1_76: cast zx s_1_75 -> bv
        let s_1_76: Bits = Bits::new(s_1_75 as u128, 128u16);
        // D s_1_77: cast zx s_1_73 -> bv
        let s_1_77: Bits = Bits::new(s_1_73 as u128, 64u16);
        // C s_1_78: const #63s : i
        let s_1_78: i128 = 63;
        // C s_1_79: const #1u : u64
        let s_1_79: u64 = 1;
        // C s_1_80: cast zx s_1_79 -> bv
        let s_1_80: Bits = Bits::new(s_1_79 as u128, 64u16);
        // C s_1_81: lsl s_1_80 s_1_78
        let s_1_81: Bits = s_1_80 << s_1_78;
        // C s_1_82: sub s_1_81 s_1_80
        let s_1_82: Bits = ((s_1_81) - (s_1_80));
        // D s_1_83: and s_1_77 s_1_82
        let s_1_83: Bits = ((s_1_77) & (s_1_82));
        // D s_1_84: lsl s_1_83 s_1_74
        let s_1_84: Bits = s_1_83 << s_1_74;
        // C s_1_85: lsl s_1_82 s_1_74
        let s_1_85: Bits = s_1_82 << s_1_74;
        // C s_1_86: cmpl s_1_85
        let s_1_86: Bits = !s_1_85;
        // D s_1_87: and s_1_76 s_1_86
        let s_1_87: Bits = ((s_1_76) & (s_1_86));
        // D s_1_88: or s_1_87 s_1_84
        let s_1_88: Bits = ((s_1_87) | (s_1_84));
        // D s_1_89: cast reint s_1_88 -> u128
        let s_1_89: u128 = (s_1_88.value() as u128);
        // D s_1_90: write-var Vtmp <= s_1_89
        fn_state.Vtmp = s_1_89;
        // C s_1_91: const #0s : i
        let s_1_91: i128 = 0;
        // D s_1_92: cast zx s_1_4 -> bv
        let s_1_92: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_93: const #1s : i64
        let s_1_93: i64 = 1;
        // C s_1_94: cast zx s_1_93 -> i
        let s_1_94: i128 = (i128::try_from(s_1_93).unwrap());
        // C s_1_95: const #63s : i
        let s_1_95: i128 = 63;
        // C s_1_96: add s_1_95 s_1_94
        let s_1_96: i128 = (s_1_95 + s_1_94);
        // D s_1_97: bit-extract s_1_92 s_1_91 s_1_96
        let s_1_97: Bits = (Bits::new(
            ((s_1_92) >> (s_1_91)).value(),
            u16::try_from(s_1_96).unwrap(),
        ));
        // D s_1_98: cast reint s_1_97 -> u64
        let s_1_98: u64 = (s_1_97.value() as u64);
        // C s_1_99: const #1s : i
        let s_1_99: i128 = 1;
        // D s_1_100: cast zx s_1_98 -> bv
        let s_1_100: Bits = Bits::new(s_1_98 as u128, 64u16);
        // D s_1_101: call ROR(s_1_100, s_1_99)
        let s_1_101: Bits = ROR(state, tracer, s_1_100, s_1_99);
        // D s_1_102: cast reint s_1_101 -> u64
        let s_1_102: u64 = (s_1_101.value() as u64);
        // C s_1_103: const #0s : i
        let s_1_103: i128 = 0;
        // D s_1_104: cast zx s_1_4 -> bv
        let s_1_104: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_105: const #1s : i64
        let s_1_105: i64 = 1;
        // C s_1_106: cast zx s_1_105 -> i
        let s_1_106: i128 = (i128::try_from(s_1_105).unwrap());
        // C s_1_107: const #63s : i
        let s_1_107: i128 = 63;
        // C s_1_108: add s_1_107 s_1_106
        let s_1_108: i128 = (s_1_107 + s_1_106);
        // D s_1_109: bit-extract s_1_104 s_1_103 s_1_108
        let s_1_109: Bits = (Bits::new(
            ((s_1_104) >> (s_1_103)).value(),
            u16::try_from(s_1_108).unwrap(),
        ));
        // D s_1_110: cast reint s_1_109 -> u64
        let s_1_110: u64 = (s_1_109.value() as u64);
        // C s_1_111: const #8s : i
        let s_1_111: i128 = 8;
        // D s_1_112: cast zx s_1_110 -> bv
        let s_1_112: Bits = Bits::new(s_1_110 as u128, 64u16);
        // D s_1_113: call ROR(s_1_112, s_1_111)
        let s_1_113: Bits = ROR(state, tracer, s_1_112, s_1_111);
        // D s_1_114: cast reint s_1_113 -> u64
        let s_1_114: u64 = (s_1_113.value() as u64);
        // D s_1_115: cast zx s_1_102 -> bv
        let s_1_115: Bits = Bits::new(s_1_102 as u128, 64u16);
        // D s_1_116: cast zx s_1_114 -> bv
        let s_1_116: Bits = Bits::new(s_1_114 as u128, 64u16);
        // D s_1_117: xor s_1_115 s_1_116
        let s_1_117: Bits = ((s_1_115) ^ (s_1_116));
        // D s_1_118: cast reint s_1_117 -> u64
        let s_1_118: u64 = (s_1_117.value() as u64);
        // C s_1_119: const #7s : i
        let s_1_119: i128 = 7;
        // D s_1_120: cast zx s_1_4 -> bv
        let s_1_120: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_121: const #1s : i64
        let s_1_121: i64 = 1;
        // C s_1_122: cast zx s_1_121 -> i
        let s_1_122: i128 = (i128::try_from(s_1_121).unwrap());
        // C s_1_123: const #56s : i
        let s_1_123: i128 = 56;
        // C s_1_124: add s_1_123 s_1_122
        let s_1_124: i128 = (s_1_123 + s_1_122);
        // D s_1_125: bit-extract s_1_120 s_1_119 s_1_124
        let s_1_125: Bits = (Bits::new(
            ((s_1_120) >> (s_1_119)).value(),
            u16::try_from(s_1_124).unwrap(),
        ));
        // D s_1_126: cast reint s_1_125 -> u57
        let s_1_126: u64 = (s_1_125.value() as u64);
        // C s_1_127: const #0u : u8
        let s_1_127: u8 = 0;
        // C s_1_128: cast zx s_1_127 -> bv
        let s_1_128: Bits = Bits::new(s_1_127 as u128, 7u16);
        // D s_1_129: cast zx s_1_126 -> bv
        let s_1_129: Bits = Bits::new(s_1_126 as u128, 57u16);
        // C s_1_130: cast reint s_1_128 -> u128
        let s_1_130: u128 = (s_1_128.value() as u128);
        // D s_1_131: size-of s_1_128
        let s_1_131: u16 = s_1_128.length();
        // D s_1_132: cast reint s_1_129 -> u128
        let s_1_132: u128 = (s_1_129.value() as u128);
        // D s_1_133: size-of s_1_129
        let s_1_133: u16 = s_1_129.length();
        // D s_1_134: lsl s_1_130 s_1_133
        let s_1_134: u128 = s_1_130 << s_1_133;
        // D s_1_135: or s_1_134 s_1_132
        let s_1_135: u128 = ((s_1_134) | (s_1_132));
        // D s_1_136: add s_1_131 s_1_133
        let s_1_136: u16 = (s_1_131 + s_1_133);
        // D s_1_137: create-bits s_1_135 s_1_136
        let s_1_137: Bits = Bits::new(s_1_135, s_1_136);
        // D s_1_138: cast reint s_1_137 -> u64
        let s_1_138: u64 = (s_1_137.value() as u64);
        // D s_1_139: cast zx s_1_118 -> bv
        let s_1_139: Bits = Bits::new(s_1_118 as u128, 64u16);
        // D s_1_140: cast zx s_1_138 -> bv
        let s_1_140: Bits = Bits::new(s_1_138 as u128, 64u16);
        // D s_1_141: xor s_1_139 s_1_140
        let s_1_141: Bits = ((s_1_139) ^ (s_1_140));
        // D s_1_142: cast reint s_1_141 -> u64
        let s_1_142: u64 = (s_1_141.value() as u64);
        // C s_1_143: const #64s : i
        let s_1_143: i128 = 64;
        // D s_1_144: cast zx s_1_9 -> bv
        let s_1_144: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_145: const #1s : i64
        let s_1_145: i64 = 1;
        // C s_1_146: cast zx s_1_145 -> i
        let s_1_146: i128 = (i128::try_from(s_1_145).unwrap());
        // C s_1_147: const #63s : i
        let s_1_147: i128 = 63;
        // C s_1_148: add s_1_147 s_1_146
        let s_1_148: i128 = (s_1_147 + s_1_146);
        // D s_1_149: bit-extract s_1_144 s_1_143 s_1_148
        let s_1_149: Bits = (Bits::new(
            ((s_1_144) >> (s_1_143)).value(),
            u16::try_from(s_1_148).unwrap(),
        ));
        // D s_1_150: cast reint s_1_149 -> u64
        let s_1_150: u64 = (s_1_149.value() as u64);
        // D s_1_151: cast zx s_1_150 -> bv
        let s_1_151: Bits = Bits::new(s_1_150 as u128, 64u16);
        // D s_1_152: cast zx s_1_142 -> bv
        let s_1_152: Bits = Bits::new(s_1_142 as u128, 64u16);
        // D s_1_153: add s_1_151 s_1_152
        let s_1_153: Bits = (s_1_151 + s_1_152);
        // D s_1_154: cast reint s_1_153 -> u64
        let s_1_154: u64 = (s_1_153.value() as u64);
        // C s_1_155: const #64s : i
        let s_1_155: i128 = 64;
        // D s_1_156: cast zx s_1_89 -> bv
        let s_1_156: Bits = Bits::new(s_1_89 as u128, 128u16);
        // D s_1_157: cast zx s_1_154 -> bv
        let s_1_157: Bits = Bits::new(s_1_154 as u128, 64u16);
        // C s_1_158: const #63s : i
        let s_1_158: i128 = 63;
        // C s_1_159: const #1u : u64
        let s_1_159: u64 = 1;
        // C s_1_160: cast zx s_1_159 -> bv
        let s_1_160: Bits = Bits::new(s_1_159 as u128, 64u16);
        // C s_1_161: lsl s_1_160 s_1_158
        let s_1_161: Bits = s_1_160 << s_1_158;
        // C s_1_162: sub s_1_161 s_1_160
        let s_1_162: Bits = ((s_1_161) - (s_1_160));
        // D s_1_163: and s_1_157 s_1_162
        let s_1_163: Bits = ((s_1_157) & (s_1_162));
        // D s_1_164: lsl s_1_163 s_1_155
        let s_1_164: Bits = s_1_163 << s_1_155;
        // C s_1_165: lsl s_1_162 s_1_155
        let s_1_165: Bits = s_1_162 << s_1_155;
        // C s_1_166: cmpl s_1_165
        let s_1_166: Bits = !s_1_165;
        // D s_1_167: and s_1_156 s_1_166
        let s_1_167: Bits = ((s_1_156) & (s_1_166));
        // D s_1_168: or s_1_167 s_1_164
        let s_1_168: Bits = ((s_1_167) | (s_1_164));
        // D s_1_169: cast reint s_1_168 -> u128
        let s_1_169: u128 = (s_1_168.value() as u128);
        // D s_1_170: write-var Vtmp <= s_1_169
        fn_state.Vtmp = s_1_169;
        // C s_1_171: const #128s : i64
        let s_1_171: i64 = 128;
        // D s_1_172: read-var d:i64
        let s_1_172: i64 = fn_state.d;
        // D s_1_173: cast zx s_1_172 -> i
        let s_1_173: i128 = (i128::try_from(s_1_172).unwrap());
        // D s_1_174: cast zx s_1_169 -> bv
        let s_1_174: Bits = Bits::new(s_1_169 as u128, 128u16);
        // D s_1_175: call V_set(s_1_173, s_1_171, s_1_174)
        let s_1_175: () = V_set(state, tracer, s_1_173, s_1_171, s_1_174);
        // N s_1_176: return
        return;
    }
}
