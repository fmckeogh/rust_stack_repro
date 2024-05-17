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
pub fn execute_aarch64_instrs_vector_crypto_sha512_sha512h2<T: Tracer>(
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
        // C s_1_15: const #0s : i
        let s_1_15: i128 = 0;
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
        // C s_1_23: const #28s : i
        let s_1_23: i128 = 28;
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 64u16);
        // D s_1_25: call ROR(s_1_24, s_1_23)
        let s_1_25: Bits = ROR(state, tracer, s_1_24, s_1_23);
        // D s_1_26: cast reint s_1_25 -> u64
        let s_1_26: u64 = (s_1_25.value() as u64);
        // C s_1_27: const #0s : i
        let s_1_27: i128 = 0;
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
        // C s_1_35: const #34s : i
        let s_1_35: i128 = 34;
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
        // C s_1_43: const #0s : i
        let s_1_43: i128 = 0;
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
        // C s_1_51: const #39s : i
        let s_1_51: i128 = 39;
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
        // C s_1_59: const #0s : i
        let s_1_59: i128 = 0;
        // D s_1_60: cast zx s_1_4 -> bv
        let s_1_60: Bits = Bits::new(s_1_4 as u128, 128u16);
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
        // C s_1_67: const #64s : i
        let s_1_67: i128 = 64;
        // D s_1_68: cast zx s_1_9 -> bv
        let s_1_68: Bits = Bits::new(s_1_9 as u128, 128u16);
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
        // C s_1_79: const #0s : i
        let s_1_79: i128 = 0;
        // D s_1_80: cast zx s_1_4 -> bv
        let s_1_80: Bits = Bits::new(s_1_4 as u128, 128u16);
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
        // C s_1_87: const #0s : i
        let s_1_87: i128 = 0;
        // D s_1_88: cast zx s_1_9 -> bv
        let s_1_88: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_89: const #1s : i64
        let s_1_89: i64 = 1;
        // C s_1_90: cast zx s_1_89 -> i
        let s_1_90: i128 = (i128::try_from(s_1_89).unwrap());
        // C s_1_91: const #63s : i
        let s_1_91: i128 = 63;
        // C s_1_92: add s_1_91 s_1_90
        let s_1_92: i128 = (s_1_91 + s_1_90);
        // D s_1_93: bit-extract s_1_88 s_1_87 s_1_92
        let s_1_93: Bits = (Bits::new(
            ((s_1_88) >> (s_1_87)).value(),
            u16::try_from(s_1_92).unwrap(),
        ));
        // D s_1_94: cast reint s_1_93 -> u64
        let s_1_94: u64 = (s_1_93.value() as u64);
        // D s_1_95: cast zx s_1_86 -> bv
        let s_1_95: Bits = Bits::new(s_1_86 as u128, 64u16);
        // D s_1_96: cast zx s_1_94 -> bv
        let s_1_96: Bits = Bits::new(s_1_94 as u128, 64u16);
        // D s_1_97: and s_1_95 s_1_96
        let s_1_97: Bits = ((s_1_95) & (s_1_96));
        // D s_1_98: cast reint s_1_97 -> u64
        let s_1_98: u64 = (s_1_97.value() as u64);
        // D s_1_99: cast zx s_1_78 -> bv
        let s_1_99: Bits = Bits::new(s_1_78 as u128, 64u16);
        // D s_1_100: cast zx s_1_98 -> bv
        let s_1_100: Bits = Bits::new(s_1_98 as u128, 64u16);
        // D s_1_101: xor s_1_99 s_1_100
        let s_1_101: Bits = ((s_1_99) ^ (s_1_100));
        // D s_1_102: cast reint s_1_101 -> u64
        let s_1_102: u64 = (s_1_101.value() as u64);
        // C s_1_103: const #64s : i
        let s_1_103: i128 = 64;
        // D s_1_104: cast zx s_1_9 -> bv
        let s_1_104: Bits = Bits::new(s_1_9 as u128, 128u16);
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
        // C s_1_111: const #0s : i
        let s_1_111: i128 = 0;
        // D s_1_112: cast zx s_1_9 -> bv
        let s_1_112: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_113: const #1s : i64
        let s_1_113: i64 = 1;
        // C s_1_114: cast zx s_1_113 -> i
        let s_1_114: i128 = (i128::try_from(s_1_113).unwrap());
        // C s_1_115: const #63s : i
        let s_1_115: i128 = 63;
        // C s_1_116: add s_1_115 s_1_114
        let s_1_116: i128 = (s_1_115 + s_1_114);
        // D s_1_117: bit-extract s_1_112 s_1_111 s_1_116
        let s_1_117: Bits = (Bits::new(
            ((s_1_112) >> (s_1_111)).value(),
            u16::try_from(s_1_116).unwrap(),
        ));
        // D s_1_118: cast reint s_1_117 -> u64
        let s_1_118: u64 = (s_1_117.value() as u64);
        // D s_1_119: cast zx s_1_110 -> bv
        let s_1_119: Bits = Bits::new(s_1_110 as u128, 64u16);
        // D s_1_120: cast zx s_1_118 -> bv
        let s_1_120: Bits = Bits::new(s_1_118 as u128, 64u16);
        // D s_1_121: and s_1_119 s_1_120
        let s_1_121: Bits = ((s_1_119) & (s_1_120));
        // D s_1_122: cast reint s_1_121 -> u64
        let s_1_122: u64 = (s_1_121.value() as u64);
        // D s_1_123: cast zx s_1_102 -> bv
        let s_1_123: Bits = Bits::new(s_1_102 as u128, 64u16);
        // D s_1_124: cast zx s_1_122 -> bv
        let s_1_124: Bits = Bits::new(s_1_122 as u128, 64u16);
        // D s_1_125: xor s_1_123 s_1_124
        let s_1_125: Bits = ((s_1_123) ^ (s_1_124));
        // D s_1_126: cast reint s_1_125 -> u64
        let s_1_126: u64 = (s_1_125.value() as u64);
        // C s_1_127: const #64s : i
        let s_1_127: i128 = 64;
        // D s_1_128: read-var Vtmp:u128
        let s_1_128: u128 = fn_state.Vtmp;
        // D s_1_129: cast zx s_1_128 -> bv
        let s_1_129: Bits = Bits::new(s_1_128 as u128, 128u16);
        // D s_1_130: cast zx s_1_126 -> bv
        let s_1_130: Bits = Bits::new(s_1_126 as u128, 64u16);
        // C s_1_131: const #63s : i
        let s_1_131: i128 = 63;
        // C s_1_132: const #1u : u64
        let s_1_132: u64 = 1;
        // C s_1_133: cast zx s_1_132 -> bv
        let s_1_133: Bits = Bits::new(s_1_132 as u128, 64u16);
        // C s_1_134: lsl s_1_133 s_1_131
        let s_1_134: Bits = s_1_133 << s_1_131;
        // C s_1_135: sub s_1_134 s_1_133
        let s_1_135: Bits = ((s_1_134) - (s_1_133));
        // D s_1_136: and s_1_130 s_1_135
        let s_1_136: Bits = ((s_1_130) & (s_1_135));
        // D s_1_137: lsl s_1_136 s_1_127
        let s_1_137: Bits = s_1_136 << s_1_127;
        // C s_1_138: lsl s_1_135 s_1_127
        let s_1_138: Bits = s_1_135 << s_1_127;
        // C s_1_139: cmpl s_1_138
        let s_1_139: Bits = !s_1_138;
        // D s_1_140: and s_1_129 s_1_139
        let s_1_140: Bits = ((s_1_129) & (s_1_139));
        // D s_1_141: or s_1_140 s_1_137
        let s_1_141: Bits = ((s_1_140) | (s_1_137));
        // D s_1_142: cast reint s_1_141 -> u128
        let s_1_142: u128 = (s_1_141.value() as u128);
        // D s_1_143: write-var Vtmp <= s_1_142
        fn_state.Vtmp = s_1_142;
        // C s_1_144: const #64s : i
        let s_1_144: i128 = 64;
        // D s_1_145: cast zx s_1_142 -> bv
        let s_1_145: Bits = Bits::new(s_1_142 as u128, 128u16);
        // C s_1_146: const #1s : i64
        let s_1_146: i64 = 1;
        // C s_1_147: cast zx s_1_146 -> i
        let s_1_147: i128 = (i128::try_from(s_1_146).unwrap());
        // C s_1_148: const #63s : i
        let s_1_148: i128 = 63;
        // C s_1_149: add s_1_148 s_1_147
        let s_1_149: i128 = (s_1_148 + s_1_147);
        // D s_1_150: bit-extract s_1_145 s_1_144 s_1_149
        let s_1_150: Bits = (Bits::new(
            ((s_1_145) >> (s_1_144)).value(),
            u16::try_from(s_1_149).unwrap(),
        ));
        // D s_1_151: cast reint s_1_150 -> u64
        let s_1_151: u64 = (s_1_150.value() as u64);
        // D s_1_152: cast zx s_1_151 -> bv
        let s_1_152: Bits = Bits::new(s_1_151 as u128, 64u16);
        // D s_1_153: cast zx s_1_58 -> bv
        let s_1_153: Bits = Bits::new(s_1_58 as u128, 64u16);
        // D s_1_154: add s_1_152 s_1_153
        let s_1_154: Bits = (s_1_152 + s_1_153);
        // D s_1_155: cast reint s_1_154 -> u64
        let s_1_155: u64 = (s_1_154.value() as u64);
        // C s_1_156: const #64s : i
        let s_1_156: i128 = 64;
        // D s_1_157: cast zx s_1_14 -> bv
        let s_1_157: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_158: const #1s : i64
        let s_1_158: i64 = 1;
        // C s_1_159: cast zx s_1_158 -> i
        let s_1_159: i128 = (i128::try_from(s_1_158).unwrap());
        // C s_1_160: const #63s : i
        let s_1_160: i128 = 63;
        // C s_1_161: add s_1_160 s_1_159
        let s_1_161: i128 = (s_1_160 + s_1_159);
        // D s_1_162: bit-extract s_1_157 s_1_156 s_1_161
        let s_1_162: Bits = (Bits::new(
            ((s_1_157) >> (s_1_156)).value(),
            u16::try_from(s_1_161).unwrap(),
        ));
        // D s_1_163: cast reint s_1_162 -> u64
        let s_1_163: u64 = (s_1_162.value() as u64);
        // D s_1_164: cast zx s_1_155 -> bv
        let s_1_164: Bits = Bits::new(s_1_155 as u128, 64u16);
        // D s_1_165: cast zx s_1_163 -> bv
        let s_1_165: Bits = Bits::new(s_1_163 as u128, 64u16);
        // D s_1_166: add s_1_164 s_1_165
        let s_1_166: Bits = (s_1_164 + s_1_165);
        // D s_1_167: cast reint s_1_166 -> u64
        let s_1_167: u64 = (s_1_166.value() as u64);
        // C s_1_168: const #64s : i
        let s_1_168: i128 = 64;
        // D s_1_169: cast zx s_1_142 -> bv
        let s_1_169: Bits = Bits::new(s_1_142 as u128, 128u16);
        // D s_1_170: cast zx s_1_167 -> bv
        let s_1_170: Bits = Bits::new(s_1_167 as u128, 64u16);
        // C s_1_171: const #63s : i
        let s_1_171: i128 = 63;
        // C s_1_172: const #1u : u64
        let s_1_172: u64 = 1;
        // C s_1_173: cast zx s_1_172 -> bv
        let s_1_173: Bits = Bits::new(s_1_172 as u128, 64u16);
        // C s_1_174: lsl s_1_173 s_1_171
        let s_1_174: Bits = s_1_173 << s_1_171;
        // C s_1_175: sub s_1_174 s_1_173
        let s_1_175: Bits = ((s_1_174) - (s_1_173));
        // D s_1_176: and s_1_170 s_1_175
        let s_1_176: Bits = ((s_1_170) & (s_1_175));
        // D s_1_177: lsl s_1_176 s_1_168
        let s_1_177: Bits = s_1_176 << s_1_168;
        // C s_1_178: lsl s_1_175 s_1_168
        let s_1_178: Bits = s_1_175 << s_1_168;
        // C s_1_179: cmpl s_1_178
        let s_1_179: Bits = !s_1_178;
        // D s_1_180: and s_1_169 s_1_179
        let s_1_180: Bits = ((s_1_169) & (s_1_179));
        // D s_1_181: or s_1_180 s_1_177
        let s_1_181: Bits = ((s_1_180) | (s_1_177));
        // D s_1_182: cast reint s_1_181 -> u128
        let s_1_182: u128 = (s_1_181.value() as u128);
        // D s_1_183: write-var Vtmp <= s_1_182
        fn_state.Vtmp = s_1_182;
        // C s_1_184: const #64s : i
        let s_1_184: i128 = 64;
        // D s_1_185: cast zx s_1_182 -> bv
        let s_1_185: Bits = Bits::new(s_1_182 as u128, 128u16);
        // C s_1_186: const #1s : i64
        let s_1_186: i64 = 1;
        // C s_1_187: cast zx s_1_186 -> i
        let s_1_187: i128 = (i128::try_from(s_1_186).unwrap());
        // C s_1_188: const #63s : i
        let s_1_188: i128 = 63;
        // C s_1_189: add s_1_188 s_1_187
        let s_1_189: i128 = (s_1_188 + s_1_187);
        // D s_1_190: bit-extract s_1_185 s_1_184 s_1_189
        let s_1_190: Bits = (Bits::new(
            ((s_1_185) >> (s_1_184)).value(),
            u16::try_from(s_1_189).unwrap(),
        ));
        // D s_1_191: cast reint s_1_190 -> u64
        let s_1_191: u64 = (s_1_190.value() as u64);
        // C s_1_192: const #28s : i
        let s_1_192: i128 = 28;
        // D s_1_193: cast zx s_1_191 -> bv
        let s_1_193: Bits = Bits::new(s_1_191 as u128, 64u16);
        // D s_1_194: call ROR(s_1_193, s_1_192)
        let s_1_194: Bits = ROR(state, tracer, s_1_193, s_1_192);
        // D s_1_195: cast reint s_1_194 -> u64
        let s_1_195: u64 = (s_1_194.value() as u64);
        // C s_1_196: const #64s : i
        let s_1_196: i128 = 64;
        // D s_1_197: cast zx s_1_182 -> bv
        let s_1_197: Bits = Bits::new(s_1_182 as u128, 128u16);
        // C s_1_198: const #1s : i64
        let s_1_198: i64 = 1;
        // C s_1_199: cast zx s_1_198 -> i
        let s_1_199: i128 = (i128::try_from(s_1_198).unwrap());
        // C s_1_200: const #63s : i
        let s_1_200: i128 = 63;
        // C s_1_201: add s_1_200 s_1_199
        let s_1_201: i128 = (s_1_200 + s_1_199);
        // D s_1_202: bit-extract s_1_197 s_1_196 s_1_201
        let s_1_202: Bits = (Bits::new(
            ((s_1_197) >> (s_1_196)).value(),
            u16::try_from(s_1_201).unwrap(),
        ));
        // D s_1_203: cast reint s_1_202 -> u64
        let s_1_203: u64 = (s_1_202.value() as u64);
        // C s_1_204: const #34s : i
        let s_1_204: i128 = 34;
        // D s_1_205: cast zx s_1_203 -> bv
        let s_1_205: Bits = Bits::new(s_1_203 as u128, 64u16);
        // D s_1_206: call ROR(s_1_205, s_1_204)
        let s_1_206: Bits = ROR(state, tracer, s_1_205, s_1_204);
        // D s_1_207: cast reint s_1_206 -> u64
        let s_1_207: u64 = (s_1_206.value() as u64);
        // D s_1_208: cast zx s_1_195 -> bv
        let s_1_208: Bits = Bits::new(s_1_195 as u128, 64u16);
        // D s_1_209: cast zx s_1_207 -> bv
        let s_1_209: Bits = Bits::new(s_1_207 as u128, 64u16);
        // D s_1_210: xor s_1_208 s_1_209
        let s_1_210: Bits = ((s_1_208) ^ (s_1_209));
        // D s_1_211: cast reint s_1_210 -> u64
        let s_1_211: u64 = (s_1_210.value() as u64);
        // C s_1_212: const #64s : i
        let s_1_212: i128 = 64;
        // D s_1_213: cast zx s_1_182 -> bv
        let s_1_213: Bits = Bits::new(s_1_182 as u128, 128u16);
        // C s_1_214: const #1s : i64
        let s_1_214: i64 = 1;
        // C s_1_215: cast zx s_1_214 -> i
        let s_1_215: i128 = (i128::try_from(s_1_214).unwrap());
        // C s_1_216: const #63s : i
        let s_1_216: i128 = 63;
        // C s_1_217: add s_1_216 s_1_215
        let s_1_217: i128 = (s_1_216 + s_1_215);
        // D s_1_218: bit-extract s_1_213 s_1_212 s_1_217
        let s_1_218: Bits = (Bits::new(
            ((s_1_213) >> (s_1_212)).value(),
            u16::try_from(s_1_217).unwrap(),
        ));
        // D s_1_219: cast reint s_1_218 -> u64
        let s_1_219: u64 = (s_1_218.value() as u64);
        // C s_1_220: const #39s : i
        let s_1_220: i128 = 39;
        // D s_1_221: cast zx s_1_219 -> bv
        let s_1_221: Bits = Bits::new(s_1_219 as u128, 64u16);
        // D s_1_222: call ROR(s_1_221, s_1_220)
        let s_1_222: Bits = ROR(state, tracer, s_1_221, s_1_220);
        // D s_1_223: cast reint s_1_222 -> u64
        let s_1_223: u64 = (s_1_222.value() as u64);
        // D s_1_224: cast zx s_1_211 -> bv
        let s_1_224: Bits = Bits::new(s_1_211 as u128, 64u16);
        // D s_1_225: cast zx s_1_223 -> bv
        let s_1_225: Bits = Bits::new(s_1_223 as u128, 64u16);
        // D s_1_226: xor s_1_224 s_1_225
        let s_1_226: Bits = ((s_1_224) ^ (s_1_225));
        // D s_1_227: cast reint s_1_226 -> u64
        let s_1_227: u64 = (s_1_226.value() as u64);
        // C s_1_228: const #64s : i
        let s_1_228: i128 = 64;
        // D s_1_229: cast zx s_1_182 -> bv
        let s_1_229: Bits = Bits::new(s_1_182 as u128, 128u16);
        // C s_1_230: const #1s : i64
        let s_1_230: i64 = 1;
        // C s_1_231: cast zx s_1_230 -> i
        let s_1_231: i128 = (i128::try_from(s_1_230).unwrap());
        // C s_1_232: const #63s : i
        let s_1_232: i128 = 63;
        // C s_1_233: add s_1_232 s_1_231
        let s_1_233: i128 = (s_1_232 + s_1_231);
        // D s_1_234: bit-extract s_1_229 s_1_228 s_1_233
        let s_1_234: Bits = (Bits::new(
            ((s_1_229) >> (s_1_228)).value(),
            u16::try_from(s_1_233).unwrap(),
        ));
        // D s_1_235: cast reint s_1_234 -> u64
        let s_1_235: u64 = (s_1_234.value() as u64);
        // C s_1_236: const #0s : i
        let s_1_236: i128 = 0;
        // D s_1_237: cast zx s_1_9 -> bv
        let s_1_237: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_238: const #1s : i64
        let s_1_238: i64 = 1;
        // C s_1_239: cast zx s_1_238 -> i
        let s_1_239: i128 = (i128::try_from(s_1_238).unwrap());
        // C s_1_240: const #63s : i
        let s_1_240: i128 = 63;
        // C s_1_241: add s_1_240 s_1_239
        let s_1_241: i128 = (s_1_240 + s_1_239);
        // D s_1_242: bit-extract s_1_237 s_1_236 s_1_241
        let s_1_242: Bits = (Bits::new(
            ((s_1_237) >> (s_1_236)).value(),
            u16::try_from(s_1_241).unwrap(),
        ));
        // D s_1_243: cast reint s_1_242 -> u64
        let s_1_243: u64 = (s_1_242.value() as u64);
        // D s_1_244: cast zx s_1_235 -> bv
        let s_1_244: Bits = Bits::new(s_1_235 as u128, 64u16);
        // D s_1_245: cast zx s_1_243 -> bv
        let s_1_245: Bits = Bits::new(s_1_243 as u128, 64u16);
        // D s_1_246: and s_1_244 s_1_245
        let s_1_246: Bits = ((s_1_244) & (s_1_245));
        // D s_1_247: cast reint s_1_246 -> u64
        let s_1_247: u64 = (s_1_246.value() as u64);
        // C s_1_248: const #64s : i
        let s_1_248: i128 = 64;
        // D s_1_249: cast zx s_1_182 -> bv
        let s_1_249: Bits = Bits::new(s_1_182 as u128, 128u16);
        // C s_1_250: const #1s : i64
        let s_1_250: i64 = 1;
        // C s_1_251: cast zx s_1_250 -> i
        let s_1_251: i128 = (i128::try_from(s_1_250).unwrap());
        // C s_1_252: const #63s : i
        let s_1_252: i128 = 63;
        // C s_1_253: add s_1_252 s_1_251
        let s_1_253: i128 = (s_1_252 + s_1_251);
        // D s_1_254: bit-extract s_1_249 s_1_248 s_1_253
        let s_1_254: Bits = (Bits::new(
            ((s_1_249) >> (s_1_248)).value(),
            u16::try_from(s_1_253).unwrap(),
        ));
        // D s_1_255: cast reint s_1_254 -> u64
        let s_1_255: u64 = (s_1_254.value() as u64);
        // C s_1_256: const #64s : i
        let s_1_256: i128 = 64;
        // D s_1_257: cast zx s_1_9 -> bv
        let s_1_257: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_258: const #1s : i64
        let s_1_258: i64 = 1;
        // C s_1_259: cast zx s_1_258 -> i
        let s_1_259: i128 = (i128::try_from(s_1_258).unwrap());
        // C s_1_260: const #63s : i
        let s_1_260: i128 = 63;
        // C s_1_261: add s_1_260 s_1_259
        let s_1_261: i128 = (s_1_260 + s_1_259);
        // D s_1_262: bit-extract s_1_257 s_1_256 s_1_261
        let s_1_262: Bits = (Bits::new(
            ((s_1_257) >> (s_1_256)).value(),
            u16::try_from(s_1_261).unwrap(),
        ));
        // D s_1_263: cast reint s_1_262 -> u64
        let s_1_263: u64 = (s_1_262.value() as u64);
        // D s_1_264: cast zx s_1_255 -> bv
        let s_1_264: Bits = Bits::new(s_1_255 as u128, 64u16);
        // D s_1_265: cast zx s_1_263 -> bv
        let s_1_265: Bits = Bits::new(s_1_263 as u128, 64u16);
        // D s_1_266: and s_1_264 s_1_265
        let s_1_266: Bits = ((s_1_264) & (s_1_265));
        // D s_1_267: cast reint s_1_266 -> u64
        let s_1_267: u64 = (s_1_266.value() as u64);
        // D s_1_268: cast zx s_1_247 -> bv
        let s_1_268: Bits = Bits::new(s_1_247 as u128, 64u16);
        // D s_1_269: cast zx s_1_267 -> bv
        let s_1_269: Bits = Bits::new(s_1_267 as u128, 64u16);
        // D s_1_270: xor s_1_268 s_1_269
        let s_1_270: Bits = ((s_1_268) ^ (s_1_269));
        // D s_1_271: cast reint s_1_270 -> u64
        let s_1_271: u64 = (s_1_270.value() as u64);
        // C s_1_272: const #64s : i
        let s_1_272: i128 = 64;
        // D s_1_273: cast zx s_1_9 -> bv
        let s_1_273: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_274: const #1s : i64
        let s_1_274: i64 = 1;
        // C s_1_275: cast zx s_1_274 -> i
        let s_1_275: i128 = (i128::try_from(s_1_274).unwrap());
        // C s_1_276: const #63s : i
        let s_1_276: i128 = 63;
        // C s_1_277: add s_1_276 s_1_275
        let s_1_277: i128 = (s_1_276 + s_1_275);
        // D s_1_278: bit-extract s_1_273 s_1_272 s_1_277
        let s_1_278: Bits = (Bits::new(
            ((s_1_273) >> (s_1_272)).value(),
            u16::try_from(s_1_277).unwrap(),
        ));
        // D s_1_279: cast reint s_1_278 -> u64
        let s_1_279: u64 = (s_1_278.value() as u64);
        // C s_1_280: const #0s : i
        let s_1_280: i128 = 0;
        // D s_1_281: cast zx s_1_9 -> bv
        let s_1_281: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_282: const #1s : i64
        let s_1_282: i64 = 1;
        // C s_1_283: cast zx s_1_282 -> i
        let s_1_283: i128 = (i128::try_from(s_1_282).unwrap());
        // C s_1_284: const #63s : i
        let s_1_284: i128 = 63;
        // C s_1_285: add s_1_284 s_1_283
        let s_1_285: i128 = (s_1_284 + s_1_283);
        // D s_1_286: bit-extract s_1_281 s_1_280 s_1_285
        let s_1_286: Bits = (Bits::new(
            ((s_1_281) >> (s_1_280)).value(),
            u16::try_from(s_1_285).unwrap(),
        ));
        // D s_1_287: cast reint s_1_286 -> u64
        let s_1_287: u64 = (s_1_286.value() as u64);
        // D s_1_288: cast zx s_1_279 -> bv
        let s_1_288: Bits = Bits::new(s_1_279 as u128, 64u16);
        // D s_1_289: cast zx s_1_287 -> bv
        let s_1_289: Bits = Bits::new(s_1_287 as u128, 64u16);
        // D s_1_290: and s_1_288 s_1_289
        let s_1_290: Bits = ((s_1_288) & (s_1_289));
        // D s_1_291: cast reint s_1_290 -> u64
        let s_1_291: u64 = (s_1_290.value() as u64);
        // D s_1_292: cast zx s_1_271 -> bv
        let s_1_292: Bits = Bits::new(s_1_271 as u128, 64u16);
        // D s_1_293: cast zx s_1_291 -> bv
        let s_1_293: Bits = Bits::new(s_1_291 as u128, 64u16);
        // D s_1_294: xor s_1_292 s_1_293
        let s_1_294: Bits = ((s_1_292) ^ (s_1_293));
        // D s_1_295: cast reint s_1_294 -> u64
        let s_1_295: u64 = (s_1_294.value() as u64);
        // C s_1_296: const #0s : i
        let s_1_296: i128 = 0;
        // D s_1_297: cast zx s_1_182 -> bv
        let s_1_297: Bits = Bits::new(s_1_182 as u128, 128u16);
        // D s_1_298: cast zx s_1_295 -> bv
        let s_1_298: Bits = Bits::new(s_1_295 as u128, 64u16);
        // C s_1_299: const #63s : i
        let s_1_299: i128 = 63;
        // C s_1_300: const #1u : u64
        let s_1_300: u64 = 1;
        // C s_1_301: cast zx s_1_300 -> bv
        let s_1_301: Bits = Bits::new(s_1_300 as u128, 64u16);
        // C s_1_302: lsl s_1_301 s_1_299
        let s_1_302: Bits = s_1_301 << s_1_299;
        // C s_1_303: sub s_1_302 s_1_301
        let s_1_303: Bits = ((s_1_302) - (s_1_301));
        // D s_1_304: and s_1_298 s_1_303
        let s_1_304: Bits = ((s_1_298) & (s_1_303));
        // D s_1_305: lsl s_1_304 s_1_296
        let s_1_305: Bits = s_1_304 << s_1_296;
        // C s_1_306: lsl s_1_303 s_1_296
        let s_1_306: Bits = s_1_303 << s_1_296;
        // C s_1_307: cmpl s_1_306
        let s_1_307: Bits = !s_1_306;
        // D s_1_308: and s_1_297 s_1_307
        let s_1_308: Bits = ((s_1_297) & (s_1_307));
        // D s_1_309: or s_1_308 s_1_305
        let s_1_309: Bits = ((s_1_308) | (s_1_305));
        // D s_1_310: cast reint s_1_309 -> u128
        let s_1_310: u128 = (s_1_309.value() as u128);
        // D s_1_311: write-var Vtmp <= s_1_310
        fn_state.Vtmp = s_1_310;
        // C s_1_312: const #0s : i
        let s_1_312: i128 = 0;
        // D s_1_313: cast zx s_1_310 -> bv
        let s_1_313: Bits = Bits::new(s_1_310 as u128, 128u16);
        // C s_1_314: const #1s : i64
        let s_1_314: i64 = 1;
        // C s_1_315: cast zx s_1_314 -> i
        let s_1_315: i128 = (i128::try_from(s_1_314).unwrap());
        // C s_1_316: const #63s : i
        let s_1_316: i128 = 63;
        // C s_1_317: add s_1_316 s_1_315
        let s_1_317: i128 = (s_1_316 + s_1_315);
        // D s_1_318: bit-extract s_1_313 s_1_312 s_1_317
        let s_1_318: Bits = (Bits::new(
            ((s_1_313) >> (s_1_312)).value(),
            u16::try_from(s_1_317).unwrap(),
        ));
        // D s_1_319: cast reint s_1_318 -> u64
        let s_1_319: u64 = (s_1_318.value() as u64);
        // D s_1_320: cast zx s_1_319 -> bv
        let s_1_320: Bits = Bits::new(s_1_319 as u128, 64u16);
        // D s_1_321: cast zx s_1_227 -> bv
        let s_1_321: Bits = Bits::new(s_1_227 as u128, 64u16);
        // D s_1_322: add s_1_320 s_1_321
        let s_1_322: Bits = (s_1_320 + s_1_321);
        // D s_1_323: cast reint s_1_322 -> u64
        let s_1_323: u64 = (s_1_322.value() as u64);
        // C s_1_324: const #0s : i
        let s_1_324: i128 = 0;
        // D s_1_325: cast zx s_1_14 -> bv
        let s_1_325: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_326: const #1s : i64
        let s_1_326: i64 = 1;
        // C s_1_327: cast zx s_1_326 -> i
        let s_1_327: i128 = (i128::try_from(s_1_326).unwrap());
        // C s_1_328: const #63s : i
        let s_1_328: i128 = 63;
        // C s_1_329: add s_1_328 s_1_327
        let s_1_329: i128 = (s_1_328 + s_1_327);
        // D s_1_330: bit-extract s_1_325 s_1_324 s_1_329
        let s_1_330: Bits = (Bits::new(
            ((s_1_325) >> (s_1_324)).value(),
            u16::try_from(s_1_329).unwrap(),
        ));
        // D s_1_331: cast reint s_1_330 -> u64
        let s_1_331: u64 = (s_1_330.value() as u64);
        // D s_1_332: cast zx s_1_323 -> bv
        let s_1_332: Bits = Bits::new(s_1_323 as u128, 64u16);
        // D s_1_333: cast zx s_1_331 -> bv
        let s_1_333: Bits = Bits::new(s_1_331 as u128, 64u16);
        // D s_1_334: add s_1_332 s_1_333
        let s_1_334: Bits = (s_1_332 + s_1_333);
        // D s_1_335: cast reint s_1_334 -> u64
        let s_1_335: u64 = (s_1_334.value() as u64);
        // C s_1_336: const #0s : i
        let s_1_336: i128 = 0;
        // D s_1_337: cast zx s_1_310 -> bv
        let s_1_337: Bits = Bits::new(s_1_310 as u128, 128u16);
        // D s_1_338: cast zx s_1_335 -> bv
        let s_1_338: Bits = Bits::new(s_1_335 as u128, 64u16);
        // C s_1_339: const #63s : i
        let s_1_339: i128 = 63;
        // C s_1_340: const #1u : u64
        let s_1_340: u64 = 1;
        // C s_1_341: cast zx s_1_340 -> bv
        let s_1_341: Bits = Bits::new(s_1_340 as u128, 64u16);
        // C s_1_342: lsl s_1_341 s_1_339
        let s_1_342: Bits = s_1_341 << s_1_339;
        // C s_1_343: sub s_1_342 s_1_341
        let s_1_343: Bits = ((s_1_342) - (s_1_341));
        // D s_1_344: and s_1_338 s_1_343
        let s_1_344: Bits = ((s_1_338) & (s_1_343));
        // D s_1_345: lsl s_1_344 s_1_336
        let s_1_345: Bits = s_1_344 << s_1_336;
        // C s_1_346: lsl s_1_343 s_1_336
        let s_1_346: Bits = s_1_343 << s_1_336;
        // C s_1_347: cmpl s_1_346
        let s_1_347: Bits = !s_1_346;
        // D s_1_348: and s_1_337 s_1_347
        let s_1_348: Bits = ((s_1_337) & (s_1_347));
        // D s_1_349: or s_1_348 s_1_345
        let s_1_349: Bits = ((s_1_348) | (s_1_345));
        // D s_1_350: cast reint s_1_349 -> u128
        let s_1_350: u128 = (s_1_349.value() as u128);
        // D s_1_351: write-var Vtmp <= s_1_350
        fn_state.Vtmp = s_1_350;
        // C s_1_352: const #128s : i64
        let s_1_352: i64 = 128;
        // D s_1_353: read-var d:i64
        let s_1_353: i64 = fn_state.d;
        // D s_1_354: cast zx s_1_353 -> i
        let s_1_354: i128 = (i128::try_from(s_1_353).unwrap());
        // D s_1_355: cast zx s_1_350 -> bv
        let s_1_355: Bits = Bits::new(s_1_350 as u128, 128u16);
        // D s_1_356: call V_set(s_1_354, s_1_352, s_1_355)
        let s_1_356: () = V_set(state, tracer, s_1_354, s_1_352, s_1_355);
        // N s_1_357: return
        return;
    }
}
