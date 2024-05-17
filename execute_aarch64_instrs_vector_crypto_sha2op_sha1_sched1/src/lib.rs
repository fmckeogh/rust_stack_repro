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
pub fn execute_aarch64_instrs_vector_crypto_sha2op_sha1_sched1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u128,
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
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
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
        // C s_1_10: const #32s : i
        let s_1_10: i128 = 32;
        // D s_1_11: cast zx s_1_9 -> bv
        let s_1_11: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_12: lsr s_1_11 s_1_10
        let s_1_12: Bits = s_1_11 >> s_1_10;
        // D s_1_13: cast reint s_1_12 -> u128
        let s_1_13: u128 = (s_1_12.value() as u128);
        // D s_1_14: cast zx s_1_4 -> bv
        let s_1_14: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_15: cast zx s_1_13 -> bv
        let s_1_15: Bits = Bits::new(s_1_13 as u128, 128u16);
        // D s_1_16: xor s_1_14 s_1_15
        let s_1_16: Bits = ((s_1_14) ^ (s_1_15));
        // D s_1_17: cast reint s_1_16 -> u128
        let s_1_17: u128 = (s_1_16.value() as u128);
        // C s_1_18: const #0s : i
        let s_1_18: i128 = 0;
        // D s_1_19: cast zx s_1_17 -> bv
        let s_1_19: Bits = Bits::new(s_1_17 as u128, 128u16);
        // C s_1_20: const #1s : i64
        let s_1_20: i64 = 1;
        // C s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // C s_1_22: const #31s : i
        let s_1_22: i128 = 31;
        // C s_1_23: add s_1_22 s_1_21
        let s_1_23: i128 = (s_1_22 + s_1_21);
        // D s_1_24: bit-extract s_1_19 s_1_18 s_1_23
        let s_1_24: Bits = (Bits::new(
            ((s_1_19) >> (s_1_18)).value(),
            u16::try_from(s_1_23).unwrap(),
        ));
        // D s_1_25: cast reint s_1_24 -> u32
        let s_1_25: u32 = (s_1_24.value() as u32);
        // C s_1_26: const #1s : i
        let s_1_26: i128 = 1;
        // D s_1_27: cast zx s_1_25 -> bv
        let s_1_27: Bits = Bits::new(s_1_25 as u128, 32u16);
        // D s_1_28: call ROL(s_1_27, s_1_26)
        let s_1_28: Bits = ROL(state, tracer, s_1_27, s_1_26);
        // D s_1_29: cast reint s_1_28 -> u32
        let s_1_29: u32 = (s_1_28.value() as u32);
        // C s_1_30: const #0s : i
        let s_1_30: i128 = 0;
        // D s_1_31: read-var result:u128
        let s_1_31: u128 = fn_state.result;
        // D s_1_32: cast zx s_1_31 -> bv
        let s_1_32: Bits = Bits::new(s_1_31 as u128, 128u16);
        // D s_1_33: cast zx s_1_29 -> bv
        let s_1_33: Bits = Bits::new(s_1_29 as u128, 32u16);
        // C s_1_34: const #31s : i
        let s_1_34: i128 = 31;
        // C s_1_35: const #1u : u64
        let s_1_35: u64 = 1;
        // C s_1_36: cast zx s_1_35 -> bv
        let s_1_36: Bits = Bits::new(s_1_35 as u128, 64u16);
        // C s_1_37: lsl s_1_36 s_1_34
        let s_1_37: Bits = s_1_36 << s_1_34;
        // C s_1_38: sub s_1_37 s_1_36
        let s_1_38: Bits = ((s_1_37) - (s_1_36));
        // D s_1_39: and s_1_33 s_1_38
        let s_1_39: Bits = ((s_1_33) & (s_1_38));
        // D s_1_40: lsl s_1_39 s_1_30
        let s_1_40: Bits = s_1_39 << s_1_30;
        // C s_1_41: lsl s_1_38 s_1_30
        let s_1_41: Bits = s_1_38 << s_1_30;
        // C s_1_42: cmpl s_1_41
        let s_1_42: Bits = !s_1_41;
        // D s_1_43: and s_1_32 s_1_42
        let s_1_43: Bits = ((s_1_32) & (s_1_42));
        // D s_1_44: or s_1_43 s_1_40
        let s_1_44: Bits = ((s_1_43) | (s_1_40));
        // D s_1_45: cast reint s_1_44 -> u128
        let s_1_45: u128 = (s_1_44.value() as u128);
        // D s_1_46: write-var result <= s_1_45
        fn_state.result = s_1_45;
        // C s_1_47: const #32s : i
        let s_1_47: i128 = 32;
        // D s_1_48: cast zx s_1_17 -> bv
        let s_1_48: Bits = Bits::new(s_1_17 as u128, 128u16);
        // C s_1_49: const #1s : i64
        let s_1_49: i64 = 1;
        // C s_1_50: cast zx s_1_49 -> i
        let s_1_50: i128 = (i128::try_from(s_1_49).unwrap());
        // C s_1_51: const #31s : i
        let s_1_51: i128 = 31;
        // C s_1_52: add s_1_51 s_1_50
        let s_1_52: i128 = (s_1_51 + s_1_50);
        // D s_1_53: bit-extract s_1_48 s_1_47 s_1_52
        let s_1_53: Bits = (Bits::new(
            ((s_1_48) >> (s_1_47)).value(),
            u16::try_from(s_1_52).unwrap(),
        ));
        // D s_1_54: cast reint s_1_53 -> u32
        let s_1_54: u32 = (s_1_53.value() as u32);
        // C s_1_55: const #1s : i
        let s_1_55: i128 = 1;
        // D s_1_56: cast zx s_1_54 -> bv
        let s_1_56: Bits = Bits::new(s_1_54 as u128, 32u16);
        // D s_1_57: call ROL(s_1_56, s_1_55)
        let s_1_57: Bits = ROL(state, tracer, s_1_56, s_1_55);
        // D s_1_58: cast reint s_1_57 -> u32
        let s_1_58: u32 = (s_1_57.value() as u32);
        // C s_1_59: const #32s : i
        let s_1_59: i128 = 32;
        // D s_1_60: cast zx s_1_45 -> bv
        let s_1_60: Bits = Bits::new(s_1_45 as u128, 128u16);
        // D s_1_61: cast zx s_1_58 -> bv
        let s_1_61: Bits = Bits::new(s_1_58 as u128, 32u16);
        // C s_1_62: const #31s : i
        let s_1_62: i128 = 31;
        // C s_1_63: const #1u : u64
        let s_1_63: u64 = 1;
        // C s_1_64: cast zx s_1_63 -> bv
        let s_1_64: Bits = Bits::new(s_1_63 as u128, 64u16);
        // C s_1_65: lsl s_1_64 s_1_62
        let s_1_65: Bits = s_1_64 << s_1_62;
        // C s_1_66: sub s_1_65 s_1_64
        let s_1_66: Bits = ((s_1_65) - (s_1_64));
        // D s_1_67: and s_1_61 s_1_66
        let s_1_67: Bits = ((s_1_61) & (s_1_66));
        // D s_1_68: lsl s_1_67 s_1_59
        let s_1_68: Bits = s_1_67 << s_1_59;
        // C s_1_69: lsl s_1_66 s_1_59
        let s_1_69: Bits = s_1_66 << s_1_59;
        // C s_1_70: cmpl s_1_69
        let s_1_70: Bits = !s_1_69;
        // D s_1_71: and s_1_60 s_1_70
        let s_1_71: Bits = ((s_1_60) & (s_1_70));
        // D s_1_72: or s_1_71 s_1_68
        let s_1_72: Bits = ((s_1_71) | (s_1_68));
        // D s_1_73: cast reint s_1_72 -> u128
        let s_1_73: u128 = (s_1_72.value() as u128);
        // D s_1_74: write-var result <= s_1_73
        fn_state.result = s_1_73;
        // C s_1_75: const #64s : i
        let s_1_75: i128 = 64;
        // D s_1_76: cast zx s_1_17 -> bv
        let s_1_76: Bits = Bits::new(s_1_17 as u128, 128u16);
        // C s_1_77: const #1s : i64
        let s_1_77: i64 = 1;
        // C s_1_78: cast zx s_1_77 -> i
        let s_1_78: i128 = (i128::try_from(s_1_77).unwrap());
        // C s_1_79: const #31s : i
        let s_1_79: i128 = 31;
        // C s_1_80: add s_1_79 s_1_78
        let s_1_80: i128 = (s_1_79 + s_1_78);
        // D s_1_81: bit-extract s_1_76 s_1_75 s_1_80
        let s_1_81: Bits = (Bits::new(
            ((s_1_76) >> (s_1_75)).value(),
            u16::try_from(s_1_80).unwrap(),
        ));
        // D s_1_82: cast reint s_1_81 -> u32
        let s_1_82: u32 = (s_1_81.value() as u32);
        // C s_1_83: const #1s : i
        let s_1_83: i128 = 1;
        // D s_1_84: cast zx s_1_82 -> bv
        let s_1_84: Bits = Bits::new(s_1_82 as u128, 32u16);
        // D s_1_85: call ROL(s_1_84, s_1_83)
        let s_1_85: Bits = ROL(state, tracer, s_1_84, s_1_83);
        // D s_1_86: cast reint s_1_85 -> u32
        let s_1_86: u32 = (s_1_85.value() as u32);
        // C s_1_87: const #64s : i
        let s_1_87: i128 = 64;
        // D s_1_88: cast zx s_1_73 -> bv
        let s_1_88: Bits = Bits::new(s_1_73 as u128, 128u16);
        // D s_1_89: cast zx s_1_86 -> bv
        let s_1_89: Bits = Bits::new(s_1_86 as u128, 32u16);
        // C s_1_90: const #31s : i
        let s_1_90: i128 = 31;
        // C s_1_91: const #1u : u64
        let s_1_91: u64 = 1;
        // C s_1_92: cast zx s_1_91 -> bv
        let s_1_92: Bits = Bits::new(s_1_91 as u128, 64u16);
        // C s_1_93: lsl s_1_92 s_1_90
        let s_1_93: Bits = s_1_92 << s_1_90;
        // C s_1_94: sub s_1_93 s_1_92
        let s_1_94: Bits = ((s_1_93) - (s_1_92));
        // D s_1_95: and s_1_89 s_1_94
        let s_1_95: Bits = ((s_1_89) & (s_1_94));
        // D s_1_96: lsl s_1_95 s_1_87
        let s_1_96: Bits = s_1_95 << s_1_87;
        // C s_1_97: lsl s_1_94 s_1_87
        let s_1_97: Bits = s_1_94 << s_1_87;
        // C s_1_98: cmpl s_1_97
        let s_1_98: Bits = !s_1_97;
        // D s_1_99: and s_1_88 s_1_98
        let s_1_99: Bits = ((s_1_88) & (s_1_98));
        // D s_1_100: or s_1_99 s_1_96
        let s_1_100: Bits = ((s_1_99) | (s_1_96));
        // D s_1_101: cast reint s_1_100 -> u128
        let s_1_101: u128 = (s_1_100.value() as u128);
        // D s_1_102: write-var result <= s_1_101
        fn_state.result = s_1_101;
        // C s_1_103: const #96s : i
        let s_1_103: i128 = 96;
        // D s_1_104: cast zx s_1_17 -> bv
        let s_1_104: Bits = Bits::new(s_1_17 as u128, 128u16);
        // C s_1_105: const #1s : i64
        let s_1_105: i64 = 1;
        // C s_1_106: cast zx s_1_105 -> i
        let s_1_106: i128 = (i128::try_from(s_1_105).unwrap());
        // C s_1_107: const #31s : i
        let s_1_107: i128 = 31;
        // C s_1_108: add s_1_107 s_1_106
        let s_1_108: i128 = (s_1_107 + s_1_106);
        // D s_1_109: bit-extract s_1_104 s_1_103 s_1_108
        let s_1_109: Bits = (Bits::new(
            ((s_1_104) >> (s_1_103)).value(),
            u16::try_from(s_1_108).unwrap(),
        ));
        // D s_1_110: cast reint s_1_109 -> u32
        let s_1_110: u32 = (s_1_109.value() as u32);
        // C s_1_111: const #1s : i
        let s_1_111: i128 = 1;
        // D s_1_112: cast zx s_1_110 -> bv
        let s_1_112: Bits = Bits::new(s_1_110 as u128, 32u16);
        // D s_1_113: call ROL(s_1_112, s_1_111)
        let s_1_113: Bits = ROL(state, tracer, s_1_112, s_1_111);
        // D s_1_114: cast reint s_1_113 -> u32
        let s_1_114: u32 = (s_1_113.value() as u32);
        // C s_1_115: const #0s : i
        let s_1_115: i128 = 0;
        // D s_1_116: cast zx s_1_17 -> bv
        let s_1_116: Bits = Bits::new(s_1_17 as u128, 128u16);
        // C s_1_117: const #1s : i64
        let s_1_117: i64 = 1;
        // C s_1_118: cast zx s_1_117 -> i
        let s_1_118: i128 = (i128::try_from(s_1_117).unwrap());
        // C s_1_119: const #31s : i
        let s_1_119: i128 = 31;
        // C s_1_120: add s_1_119 s_1_118
        let s_1_120: i128 = (s_1_119 + s_1_118);
        // D s_1_121: bit-extract s_1_116 s_1_115 s_1_120
        let s_1_121: Bits = (Bits::new(
            ((s_1_116) >> (s_1_115)).value(),
            u16::try_from(s_1_120).unwrap(),
        ));
        // D s_1_122: cast reint s_1_121 -> u32
        let s_1_122: u32 = (s_1_121.value() as u32);
        // C s_1_123: const #2s : i
        let s_1_123: i128 = 2;
        // D s_1_124: cast zx s_1_122 -> bv
        let s_1_124: Bits = Bits::new(s_1_122 as u128, 32u16);
        // D s_1_125: call ROL(s_1_124, s_1_123)
        let s_1_125: Bits = ROL(state, tracer, s_1_124, s_1_123);
        // D s_1_126: cast reint s_1_125 -> u32
        let s_1_126: u32 = (s_1_125.value() as u32);
        // D s_1_127: cast zx s_1_114 -> bv
        let s_1_127: Bits = Bits::new(s_1_114 as u128, 32u16);
        // D s_1_128: cast zx s_1_126 -> bv
        let s_1_128: Bits = Bits::new(s_1_126 as u128, 32u16);
        // D s_1_129: xor s_1_127 s_1_128
        let s_1_129: Bits = ((s_1_127) ^ (s_1_128));
        // D s_1_130: cast reint s_1_129 -> u32
        let s_1_130: u32 = (s_1_129.value() as u32);
        // C s_1_131: const #96s : i
        let s_1_131: i128 = 96;
        // D s_1_132: cast zx s_1_101 -> bv
        let s_1_132: Bits = Bits::new(s_1_101 as u128, 128u16);
        // D s_1_133: cast zx s_1_130 -> bv
        let s_1_133: Bits = Bits::new(s_1_130 as u128, 32u16);
        // C s_1_134: const #31s : i
        let s_1_134: i128 = 31;
        // C s_1_135: const #1u : u64
        let s_1_135: u64 = 1;
        // C s_1_136: cast zx s_1_135 -> bv
        let s_1_136: Bits = Bits::new(s_1_135 as u128, 64u16);
        // C s_1_137: lsl s_1_136 s_1_134
        let s_1_137: Bits = s_1_136 << s_1_134;
        // C s_1_138: sub s_1_137 s_1_136
        let s_1_138: Bits = ((s_1_137) - (s_1_136));
        // D s_1_139: and s_1_133 s_1_138
        let s_1_139: Bits = ((s_1_133) & (s_1_138));
        // D s_1_140: lsl s_1_139 s_1_131
        let s_1_140: Bits = s_1_139 << s_1_131;
        // C s_1_141: lsl s_1_138 s_1_131
        let s_1_141: Bits = s_1_138 << s_1_131;
        // C s_1_142: cmpl s_1_141
        let s_1_142: Bits = !s_1_141;
        // D s_1_143: and s_1_132 s_1_142
        let s_1_143: Bits = ((s_1_132) & (s_1_142));
        // D s_1_144: or s_1_143 s_1_140
        let s_1_144: Bits = ((s_1_143) | (s_1_140));
        // D s_1_145: cast reint s_1_144 -> u128
        let s_1_145: u128 = (s_1_144.value() as u128);
        // D s_1_146: write-var result <= s_1_145
        fn_state.result = s_1_145;
        // C s_1_147: const #128s : i64
        let s_1_147: i64 = 128;
        // D s_1_148: read-var d:i64
        let s_1_148: i64 = fn_state.d;
        // D s_1_149: cast zx s_1_148 -> i
        let s_1_149: i128 = (i128::try_from(s_1_148).unwrap());
        // D s_1_150: cast zx s_1_145 -> bv
        let s_1_150: Bits = Bits::new(s_1_145 as u128, 128u16);
        // D s_1_151: call V_set(s_1_149, s_1_147, s_1_150)
        let s_1_151: () = V_set(state, tracer, s_1_149, s_1_147, s_1_150);
        // N s_1_152: return
        return;
    }
}
