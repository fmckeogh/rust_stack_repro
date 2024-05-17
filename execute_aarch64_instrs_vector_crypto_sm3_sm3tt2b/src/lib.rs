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
use Elem_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3tt2b<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    i: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u128,
        d: i64,
        i: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        i,
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
        // C s_1_15: const #32s : i64
        let s_1_15: i64 = 32;
        // D s_1_16: cast zx s_1_4 -> bv
        let s_1_16: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_17: read-var i:i64
        let s_1_17: i64 = fn_state.i;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // C s_1_19: cast zx s_1_15 -> i
        let s_1_19: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_20: call Elem_read(s_1_16, s_1_18, s_1_19)
        let s_1_20: Bits = Elem_read(state, tracer, s_1_16, s_1_18, s_1_19);
        // D s_1_21: cast reint s_1_20 -> u32
        let s_1_21: u32 = (s_1_20.value() as u32);
        // C s_1_22: const #96s : i
        let s_1_22: i128 = 96;
        // D s_1_23: cast zx s_1_14 -> bv
        let s_1_23: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_24: const #1s : i64
        let s_1_24: i64 = 1;
        // C s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // C s_1_26: const #31s : i
        let s_1_26: i128 = 31;
        // C s_1_27: add s_1_26 s_1_25
        let s_1_27: i128 = (s_1_26 + s_1_25);
        // D s_1_28: bit-extract s_1_23 s_1_22 s_1_27
        let s_1_28: Bits = (Bits::new(
            ((s_1_23) >> (s_1_22)).value(),
            u16::try_from(s_1_27).unwrap(),
        ));
        // D s_1_29: cast reint s_1_28 -> u32
        let s_1_29: u32 = (s_1_28.value() as u32);
        // C s_1_30: const #64s : i
        let s_1_30: i128 = 64;
        // D s_1_31: cast zx s_1_14 -> bv
        let s_1_31: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_32: const #1s : i64
        let s_1_32: i64 = 1;
        // C s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // C s_1_34: const #31s : i
        let s_1_34: i128 = 31;
        // C s_1_35: add s_1_34 s_1_33
        let s_1_35: i128 = (s_1_34 + s_1_33);
        // D s_1_36: bit-extract s_1_31 s_1_30 s_1_35
        let s_1_36: Bits = (Bits::new(
            ((s_1_31) >> (s_1_30)).value(),
            u16::try_from(s_1_35).unwrap(),
        ));
        // D s_1_37: cast reint s_1_36 -> u32
        let s_1_37: u32 = (s_1_36.value() as u32);
        // D s_1_38: cast zx s_1_29 -> bv
        let s_1_38: Bits = Bits::new(s_1_29 as u128, 32u16);
        // D s_1_39: cast zx s_1_37 -> bv
        let s_1_39: Bits = Bits::new(s_1_37 as u128, 32u16);
        // D s_1_40: and s_1_38 s_1_39
        let s_1_40: Bits = ((s_1_38) & (s_1_39));
        // D s_1_41: cast reint s_1_40 -> u32
        let s_1_41: u32 = (s_1_40.value() as u32);
        // C s_1_42: const #96s : i
        let s_1_42: i128 = 96;
        // D s_1_43: cast zx s_1_14 -> bv
        let s_1_43: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_44: const #1s : i64
        let s_1_44: i64 = 1;
        // C s_1_45: cast zx s_1_44 -> i
        let s_1_45: i128 = (i128::try_from(s_1_44).unwrap());
        // C s_1_46: const #31s : i
        let s_1_46: i128 = 31;
        // C s_1_47: add s_1_46 s_1_45
        let s_1_47: i128 = (s_1_46 + s_1_45);
        // D s_1_48: bit-extract s_1_43 s_1_42 s_1_47
        let s_1_48: Bits = (Bits::new(
            ((s_1_43) >> (s_1_42)).value(),
            u16::try_from(s_1_47).unwrap(),
        ));
        // D s_1_49: cast reint s_1_48 -> u32
        let s_1_49: u32 = (s_1_48.value() as u32);
        // D s_1_50: cast zx s_1_49 -> bv
        let s_1_50: Bits = Bits::new(s_1_49 as u128, 32u16);
        // D s_1_51: not s_1_50
        let s_1_51: Bits = !s_1_50;
        // D s_1_52: cast reint s_1_51 -> u32
        let s_1_52: u32 = (s_1_51.value() as u32);
        // C s_1_53: const #32s : i
        let s_1_53: i128 = 32;
        // D s_1_54: cast zx s_1_14 -> bv
        let s_1_54: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_55: const #1s : i64
        let s_1_55: i64 = 1;
        // C s_1_56: cast zx s_1_55 -> i
        let s_1_56: i128 = (i128::try_from(s_1_55).unwrap());
        // C s_1_57: const #31s : i
        let s_1_57: i128 = 31;
        // C s_1_58: add s_1_57 s_1_56
        let s_1_58: i128 = (s_1_57 + s_1_56);
        // D s_1_59: bit-extract s_1_54 s_1_53 s_1_58
        let s_1_59: Bits = (Bits::new(
            ((s_1_54) >> (s_1_53)).value(),
            u16::try_from(s_1_58).unwrap(),
        ));
        // D s_1_60: cast reint s_1_59 -> u32
        let s_1_60: u32 = (s_1_59.value() as u32);
        // D s_1_61: cast zx s_1_52 -> bv
        let s_1_61: Bits = Bits::new(s_1_52 as u128, 32u16);
        // D s_1_62: cast zx s_1_60 -> bv
        let s_1_62: Bits = Bits::new(s_1_60 as u128, 32u16);
        // D s_1_63: and s_1_61 s_1_62
        let s_1_63: Bits = ((s_1_61) & (s_1_62));
        // D s_1_64: cast reint s_1_63 -> u32
        let s_1_64: u32 = (s_1_63.value() as u32);
        // D s_1_65: cast zx s_1_41 -> bv
        let s_1_65: Bits = Bits::new(s_1_41 as u128, 32u16);
        // D s_1_66: cast zx s_1_64 -> bv
        let s_1_66: Bits = Bits::new(s_1_64 as u128, 32u16);
        // D s_1_67: or s_1_65 s_1_66
        let s_1_67: Bits = ((s_1_65) | (s_1_66));
        // D s_1_68: cast reint s_1_67 -> u32
        let s_1_68: u32 = (s_1_67.value() as u32);
        // C s_1_69: const #0s : i
        let s_1_69: i128 = 0;
        // D s_1_70: cast zx s_1_14 -> bv
        let s_1_70: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_71: const #1s : i64
        let s_1_71: i64 = 1;
        // C s_1_72: cast zx s_1_71 -> i
        let s_1_72: i128 = (i128::try_from(s_1_71).unwrap());
        // C s_1_73: const #31s : i
        let s_1_73: i128 = 31;
        // C s_1_74: add s_1_73 s_1_72
        let s_1_74: i128 = (s_1_73 + s_1_72);
        // D s_1_75: bit-extract s_1_70 s_1_69 s_1_74
        let s_1_75: Bits = (Bits::new(
            ((s_1_70) >> (s_1_69)).value(),
            u16::try_from(s_1_74).unwrap(),
        ));
        // D s_1_76: cast reint s_1_75 -> u32
        let s_1_76: u32 = (s_1_75.value() as u32);
        // D s_1_77: cast zx s_1_68 -> bv
        let s_1_77: Bits = Bits::new(s_1_68 as u128, 32u16);
        // D s_1_78: cast zx s_1_76 -> bv
        let s_1_78: Bits = Bits::new(s_1_76 as u128, 32u16);
        // D s_1_79: add s_1_77 s_1_78
        let s_1_79: Bits = (s_1_77 + s_1_78);
        // D s_1_80: cast reint s_1_79 -> u32
        let s_1_80: u32 = (s_1_79.value() as u32);
        // C s_1_81: const #96s : i
        let s_1_81: i128 = 96;
        // D s_1_82: cast zx s_1_9 -> bv
        let s_1_82: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_83: const #1s : i64
        let s_1_83: i64 = 1;
        // C s_1_84: cast zx s_1_83 -> i
        let s_1_84: i128 = (i128::try_from(s_1_83).unwrap());
        // C s_1_85: const #31s : i
        let s_1_85: i128 = 31;
        // C s_1_86: add s_1_85 s_1_84
        let s_1_86: i128 = (s_1_85 + s_1_84);
        // D s_1_87: bit-extract s_1_82 s_1_81 s_1_86
        let s_1_87: Bits = (Bits::new(
            ((s_1_82) >> (s_1_81)).value(),
            u16::try_from(s_1_86).unwrap(),
        ));
        // D s_1_88: cast reint s_1_87 -> u32
        let s_1_88: u32 = (s_1_87.value() as u32);
        // D s_1_89: cast zx s_1_80 -> bv
        let s_1_89: Bits = Bits::new(s_1_80 as u128, 32u16);
        // D s_1_90: cast zx s_1_88 -> bv
        let s_1_90: Bits = Bits::new(s_1_88 as u128, 32u16);
        // D s_1_91: add s_1_89 s_1_90
        let s_1_91: Bits = (s_1_89 + s_1_90);
        // D s_1_92: cast reint s_1_91 -> u32
        let s_1_92: u32 = (s_1_91.value() as u32);
        // D s_1_93: cast zx s_1_92 -> bv
        let s_1_93: Bits = Bits::new(s_1_92 as u128, 32u16);
        // D s_1_94: cast zx s_1_21 -> bv
        let s_1_94: Bits = Bits::new(s_1_21 as u128, 32u16);
        // D s_1_95: add s_1_93 s_1_94
        let s_1_95: Bits = (s_1_93 + s_1_94);
        // D s_1_96: cast reint s_1_95 -> u32
        let s_1_96: u32 = (s_1_95.value() as u32);
        // C s_1_97: const #0s : i
        let s_1_97: i128 = 0;
        // D s_1_98: cast zx s_1_96 -> bv
        let s_1_98: Bits = Bits::new(s_1_96 as u128, 32u16);
        // C s_1_99: const #1s : i64
        let s_1_99: i64 = 1;
        // C s_1_100: cast zx s_1_99 -> i
        let s_1_100: i128 = (i128::try_from(s_1_99).unwrap());
        // C s_1_101: const #31s : i
        let s_1_101: i128 = 31;
        // C s_1_102: add s_1_101 s_1_100
        let s_1_102: i128 = (s_1_101 + s_1_100);
        // D s_1_103: bit-extract s_1_98 s_1_97 s_1_102
        let s_1_103: Bits = (Bits::new(
            ((s_1_98) >> (s_1_97)).value(),
            u16::try_from(s_1_102).unwrap(),
        ));
        // D s_1_104: cast reint s_1_103 -> u32
        let s_1_104: u32 = (s_1_103.value() as u32);
        // C s_1_105: const #32s : i
        let s_1_105: i128 = 32;
        // D s_1_106: cast zx s_1_14 -> bv
        let s_1_106: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_107: const #1s : i64
        let s_1_107: i64 = 1;
        // C s_1_108: cast zx s_1_107 -> i
        let s_1_108: i128 = (i128::try_from(s_1_107).unwrap());
        // C s_1_109: const #31s : i
        let s_1_109: i128 = 31;
        // C s_1_110: add s_1_109 s_1_108
        let s_1_110: i128 = (s_1_109 + s_1_108);
        // D s_1_111: bit-extract s_1_106 s_1_105 s_1_110
        let s_1_111: Bits = (Bits::new(
            ((s_1_106) >> (s_1_105)).value(),
            u16::try_from(s_1_110).unwrap(),
        ));
        // D s_1_112: cast reint s_1_111 -> u32
        let s_1_112: u32 = (s_1_111.value() as u32);
        // C s_1_113: const #0s : i
        let s_1_113: i128 = 0;
        // D s_1_114: read-var result:u128
        let s_1_114: u128 = fn_state.result;
        // D s_1_115: cast zx s_1_114 -> bv
        let s_1_115: Bits = Bits::new(s_1_114 as u128, 128u16);
        // D s_1_116: cast zx s_1_112 -> bv
        let s_1_116: Bits = Bits::new(s_1_112 as u128, 32u16);
        // C s_1_117: const #31s : i
        let s_1_117: i128 = 31;
        // C s_1_118: const #1u : u64
        let s_1_118: u64 = 1;
        // C s_1_119: cast zx s_1_118 -> bv
        let s_1_119: Bits = Bits::new(s_1_118 as u128, 64u16);
        // C s_1_120: lsl s_1_119 s_1_117
        let s_1_120: Bits = s_1_119 << s_1_117;
        // C s_1_121: sub s_1_120 s_1_119
        let s_1_121: Bits = ((s_1_120) - (s_1_119));
        // D s_1_122: and s_1_116 s_1_121
        let s_1_122: Bits = ((s_1_116) & (s_1_121));
        // D s_1_123: lsl s_1_122 s_1_113
        let s_1_123: Bits = s_1_122 << s_1_113;
        // C s_1_124: lsl s_1_121 s_1_113
        let s_1_124: Bits = s_1_121 << s_1_113;
        // C s_1_125: cmpl s_1_124
        let s_1_125: Bits = !s_1_124;
        // D s_1_126: and s_1_115 s_1_125
        let s_1_126: Bits = ((s_1_115) & (s_1_125));
        // D s_1_127: or s_1_126 s_1_123
        let s_1_127: Bits = ((s_1_126) | (s_1_123));
        // D s_1_128: cast reint s_1_127 -> u128
        let s_1_128: u128 = (s_1_127.value() as u128);
        // D s_1_129: write-var result <= s_1_128
        fn_state.result = s_1_128;
        // C s_1_130: const #64s : i
        let s_1_130: i128 = 64;
        // D s_1_131: cast zx s_1_14 -> bv
        let s_1_131: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_132: const #1s : i64
        let s_1_132: i64 = 1;
        // C s_1_133: cast zx s_1_132 -> i
        let s_1_133: i128 = (i128::try_from(s_1_132).unwrap());
        // C s_1_134: const #31s : i
        let s_1_134: i128 = 31;
        // C s_1_135: add s_1_134 s_1_133
        let s_1_135: i128 = (s_1_134 + s_1_133);
        // D s_1_136: bit-extract s_1_131 s_1_130 s_1_135
        let s_1_136: Bits = (Bits::new(
            ((s_1_131) >> (s_1_130)).value(),
            u16::try_from(s_1_135).unwrap(),
        ));
        // D s_1_137: cast reint s_1_136 -> u32
        let s_1_137: u32 = (s_1_136.value() as u32);
        // C s_1_138: const #19s : i
        let s_1_138: i128 = 19;
        // D s_1_139: cast zx s_1_137 -> bv
        let s_1_139: Bits = Bits::new(s_1_137 as u128, 32u16);
        // D s_1_140: call ROL(s_1_139, s_1_138)
        let s_1_140: Bits = ROL(state, tracer, s_1_139, s_1_138);
        // D s_1_141: cast reint s_1_140 -> u32
        let s_1_141: u32 = (s_1_140.value() as u32);
        // C s_1_142: const #32s : i
        let s_1_142: i128 = 32;
        // D s_1_143: cast zx s_1_128 -> bv
        let s_1_143: Bits = Bits::new(s_1_128 as u128, 128u16);
        // D s_1_144: cast zx s_1_141 -> bv
        let s_1_144: Bits = Bits::new(s_1_141 as u128, 32u16);
        // C s_1_145: const #31s : i
        let s_1_145: i128 = 31;
        // C s_1_146: const #1u : u64
        let s_1_146: u64 = 1;
        // C s_1_147: cast zx s_1_146 -> bv
        let s_1_147: Bits = Bits::new(s_1_146 as u128, 64u16);
        // C s_1_148: lsl s_1_147 s_1_145
        let s_1_148: Bits = s_1_147 << s_1_145;
        // C s_1_149: sub s_1_148 s_1_147
        let s_1_149: Bits = ((s_1_148) - (s_1_147));
        // D s_1_150: and s_1_144 s_1_149
        let s_1_150: Bits = ((s_1_144) & (s_1_149));
        // D s_1_151: lsl s_1_150 s_1_142
        let s_1_151: Bits = s_1_150 << s_1_142;
        // C s_1_152: lsl s_1_149 s_1_142
        let s_1_152: Bits = s_1_149 << s_1_142;
        // C s_1_153: cmpl s_1_152
        let s_1_153: Bits = !s_1_152;
        // D s_1_154: and s_1_143 s_1_153
        let s_1_154: Bits = ((s_1_143) & (s_1_153));
        // D s_1_155: or s_1_154 s_1_151
        let s_1_155: Bits = ((s_1_154) | (s_1_151));
        // D s_1_156: cast reint s_1_155 -> u128
        let s_1_156: u128 = (s_1_155.value() as u128);
        // D s_1_157: write-var result <= s_1_156
        fn_state.result = s_1_156;
        // C s_1_158: const #96s : i
        let s_1_158: i128 = 96;
        // D s_1_159: cast zx s_1_14 -> bv
        let s_1_159: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_160: const #1s : i64
        let s_1_160: i64 = 1;
        // C s_1_161: cast zx s_1_160 -> i
        let s_1_161: i128 = (i128::try_from(s_1_160).unwrap());
        // C s_1_162: const #31s : i
        let s_1_162: i128 = 31;
        // C s_1_163: add s_1_162 s_1_161
        let s_1_163: i128 = (s_1_162 + s_1_161);
        // D s_1_164: bit-extract s_1_159 s_1_158 s_1_163
        let s_1_164: Bits = (Bits::new(
            ((s_1_159) >> (s_1_158)).value(),
            u16::try_from(s_1_163).unwrap(),
        ));
        // D s_1_165: cast reint s_1_164 -> u32
        let s_1_165: u32 = (s_1_164.value() as u32);
        // C s_1_166: const #64s : i
        let s_1_166: i128 = 64;
        // D s_1_167: cast zx s_1_156 -> bv
        let s_1_167: Bits = Bits::new(s_1_156 as u128, 128u16);
        // D s_1_168: cast zx s_1_165 -> bv
        let s_1_168: Bits = Bits::new(s_1_165 as u128, 32u16);
        // C s_1_169: const #31s : i
        let s_1_169: i128 = 31;
        // C s_1_170: const #1u : u64
        let s_1_170: u64 = 1;
        // C s_1_171: cast zx s_1_170 -> bv
        let s_1_171: Bits = Bits::new(s_1_170 as u128, 64u16);
        // C s_1_172: lsl s_1_171 s_1_169
        let s_1_172: Bits = s_1_171 << s_1_169;
        // C s_1_173: sub s_1_172 s_1_171
        let s_1_173: Bits = ((s_1_172) - (s_1_171));
        // D s_1_174: and s_1_168 s_1_173
        let s_1_174: Bits = ((s_1_168) & (s_1_173));
        // D s_1_175: lsl s_1_174 s_1_166
        let s_1_175: Bits = s_1_174 << s_1_166;
        // C s_1_176: lsl s_1_173 s_1_166
        let s_1_176: Bits = s_1_173 << s_1_166;
        // C s_1_177: cmpl s_1_176
        let s_1_177: Bits = !s_1_176;
        // D s_1_178: and s_1_167 s_1_177
        let s_1_178: Bits = ((s_1_167) & (s_1_177));
        // D s_1_179: or s_1_178 s_1_175
        let s_1_179: Bits = ((s_1_178) | (s_1_175));
        // D s_1_180: cast reint s_1_179 -> u128
        let s_1_180: u128 = (s_1_179.value() as u128);
        // D s_1_181: write-var result <= s_1_180
        fn_state.result = s_1_180;
        // C s_1_182: const #9s : i
        let s_1_182: i128 = 9;
        // D s_1_183: cast zx s_1_104 -> bv
        let s_1_183: Bits = Bits::new(s_1_104 as u128, 32u16);
        // D s_1_184: call ROL(s_1_183, s_1_182)
        let s_1_184: Bits = ROL(state, tracer, s_1_183, s_1_182);
        // D s_1_185: cast reint s_1_184 -> u32
        let s_1_185: u32 = (s_1_184.value() as u32);
        // D s_1_186: cast zx s_1_104 -> bv
        let s_1_186: Bits = Bits::new(s_1_104 as u128, 32u16);
        // D s_1_187: cast zx s_1_185 -> bv
        let s_1_187: Bits = Bits::new(s_1_185 as u128, 32u16);
        // D s_1_188: xor s_1_186 s_1_187
        let s_1_188: Bits = ((s_1_186) ^ (s_1_187));
        // D s_1_189: cast reint s_1_188 -> u32
        let s_1_189: u32 = (s_1_188.value() as u32);
        // C s_1_190: const #17s : i
        let s_1_190: i128 = 17;
        // D s_1_191: cast zx s_1_104 -> bv
        let s_1_191: Bits = Bits::new(s_1_104 as u128, 32u16);
        // D s_1_192: call ROL(s_1_191, s_1_190)
        let s_1_192: Bits = ROL(state, tracer, s_1_191, s_1_190);
        // D s_1_193: cast reint s_1_192 -> u32
        let s_1_193: u32 = (s_1_192.value() as u32);
        // D s_1_194: cast zx s_1_189 -> bv
        let s_1_194: Bits = Bits::new(s_1_189 as u128, 32u16);
        // D s_1_195: cast zx s_1_193 -> bv
        let s_1_195: Bits = Bits::new(s_1_193 as u128, 32u16);
        // D s_1_196: xor s_1_194 s_1_195
        let s_1_196: Bits = ((s_1_194) ^ (s_1_195));
        // D s_1_197: cast reint s_1_196 -> u32
        let s_1_197: u32 = (s_1_196.value() as u32);
        // C s_1_198: const #96s : i
        let s_1_198: i128 = 96;
        // D s_1_199: cast zx s_1_180 -> bv
        let s_1_199: Bits = Bits::new(s_1_180 as u128, 128u16);
        // D s_1_200: cast zx s_1_197 -> bv
        let s_1_200: Bits = Bits::new(s_1_197 as u128, 32u16);
        // C s_1_201: const #31s : i
        let s_1_201: i128 = 31;
        // C s_1_202: const #1u : u64
        let s_1_202: u64 = 1;
        // C s_1_203: cast zx s_1_202 -> bv
        let s_1_203: Bits = Bits::new(s_1_202 as u128, 64u16);
        // C s_1_204: lsl s_1_203 s_1_201
        let s_1_204: Bits = s_1_203 << s_1_201;
        // C s_1_205: sub s_1_204 s_1_203
        let s_1_205: Bits = ((s_1_204) - (s_1_203));
        // D s_1_206: and s_1_200 s_1_205
        let s_1_206: Bits = ((s_1_200) & (s_1_205));
        // D s_1_207: lsl s_1_206 s_1_198
        let s_1_207: Bits = s_1_206 << s_1_198;
        // C s_1_208: lsl s_1_205 s_1_198
        let s_1_208: Bits = s_1_205 << s_1_198;
        // C s_1_209: cmpl s_1_208
        let s_1_209: Bits = !s_1_208;
        // D s_1_210: and s_1_199 s_1_209
        let s_1_210: Bits = ((s_1_199) & (s_1_209));
        // D s_1_211: or s_1_210 s_1_207
        let s_1_211: Bits = ((s_1_210) | (s_1_207));
        // D s_1_212: cast reint s_1_211 -> u128
        let s_1_212: u128 = (s_1_211.value() as u128);
        // D s_1_213: write-var result <= s_1_212
        fn_state.result = s_1_212;
        // C s_1_214: const #128s : i64
        let s_1_214: i64 = 128;
        // D s_1_215: read-var d:i64
        let s_1_215: i64 = fn_state.d;
        // D s_1_216: cast zx s_1_215 -> i
        let s_1_216: i128 = (i128::try_from(s_1_215).unwrap());
        // D s_1_217: cast zx s_1_212 -> bv
        let s_1_217: Bits = Bits::new(s_1_212 as u128, 128u16);
        // D s_1_218: call V_set(s_1_216, s_1_214, s_1_217)
        let s_1_218: () = V_set(state, tracer, s_1_216, s_1_214, s_1_217);
        // N s_1_219: return
        return;
    }
}
