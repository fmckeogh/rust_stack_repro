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
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3tt2a<T: Tracer>(
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
        // C s_1_22: const #32s : i
        let s_1_22: i128 = 32;
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
        // C s_1_30: const #96s : i
        let s_1_30: i128 = 96;
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
        // C s_1_38: const #64s : i
        let s_1_38: i128 = 64;
        // D s_1_39: cast zx s_1_14 -> bv
        let s_1_39: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_40: const #1s : i64
        let s_1_40: i64 = 1;
        // C s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // C s_1_42: const #31s : i
        let s_1_42: i128 = 31;
        // C s_1_43: add s_1_42 s_1_41
        let s_1_43: i128 = (s_1_42 + s_1_41);
        // D s_1_44: bit-extract s_1_39 s_1_38 s_1_43
        let s_1_44: Bits = (Bits::new(
            ((s_1_39) >> (s_1_38)).value(),
            u16::try_from(s_1_43).unwrap(),
        ));
        // D s_1_45: cast reint s_1_44 -> u32
        let s_1_45: u32 = (s_1_44.value() as u32);
        // D s_1_46: cast zx s_1_37 -> bv
        let s_1_46: Bits = Bits::new(s_1_37 as u128, 32u16);
        // D s_1_47: cast zx s_1_45 -> bv
        let s_1_47: Bits = Bits::new(s_1_45 as u128, 32u16);
        // D s_1_48: xor s_1_46 s_1_47
        let s_1_48: Bits = ((s_1_46) ^ (s_1_47));
        // D s_1_49: cast reint s_1_48 -> u32
        let s_1_49: u32 = (s_1_48.value() as u32);
        // D s_1_50: cast zx s_1_29 -> bv
        let s_1_50: Bits = Bits::new(s_1_29 as u128, 32u16);
        // D s_1_51: cast zx s_1_49 -> bv
        let s_1_51: Bits = Bits::new(s_1_49 as u128, 32u16);
        // D s_1_52: xor s_1_50 s_1_51
        let s_1_52: Bits = ((s_1_50) ^ (s_1_51));
        // D s_1_53: cast reint s_1_52 -> u32
        let s_1_53: u32 = (s_1_52.value() as u32);
        // C s_1_54: const #0s : i
        let s_1_54: i128 = 0;
        // D s_1_55: cast zx s_1_14 -> bv
        let s_1_55: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_56: const #1s : i64
        let s_1_56: i64 = 1;
        // C s_1_57: cast zx s_1_56 -> i
        let s_1_57: i128 = (i128::try_from(s_1_56).unwrap());
        // C s_1_58: const #31s : i
        let s_1_58: i128 = 31;
        // C s_1_59: add s_1_58 s_1_57
        let s_1_59: i128 = (s_1_58 + s_1_57);
        // D s_1_60: bit-extract s_1_55 s_1_54 s_1_59
        let s_1_60: Bits = (Bits::new(
            ((s_1_55) >> (s_1_54)).value(),
            u16::try_from(s_1_59).unwrap(),
        ));
        // D s_1_61: cast reint s_1_60 -> u32
        let s_1_61: u32 = (s_1_60.value() as u32);
        // D s_1_62: cast zx s_1_53 -> bv
        let s_1_62: Bits = Bits::new(s_1_53 as u128, 32u16);
        // D s_1_63: cast zx s_1_61 -> bv
        let s_1_63: Bits = Bits::new(s_1_61 as u128, 32u16);
        // D s_1_64: add s_1_62 s_1_63
        let s_1_64: Bits = (s_1_62 + s_1_63);
        // D s_1_65: cast reint s_1_64 -> u32
        let s_1_65: u32 = (s_1_64.value() as u32);
        // C s_1_66: const #96s : i
        let s_1_66: i128 = 96;
        // D s_1_67: cast zx s_1_9 -> bv
        let s_1_67: Bits = Bits::new(s_1_9 as u128, 128u16);
        // C s_1_68: const #1s : i64
        let s_1_68: i64 = 1;
        // C s_1_69: cast zx s_1_68 -> i
        let s_1_69: i128 = (i128::try_from(s_1_68).unwrap());
        // C s_1_70: const #31s : i
        let s_1_70: i128 = 31;
        // C s_1_71: add s_1_70 s_1_69
        let s_1_71: i128 = (s_1_70 + s_1_69);
        // D s_1_72: bit-extract s_1_67 s_1_66 s_1_71
        let s_1_72: Bits = (Bits::new(
            ((s_1_67) >> (s_1_66)).value(),
            u16::try_from(s_1_71).unwrap(),
        ));
        // D s_1_73: cast reint s_1_72 -> u32
        let s_1_73: u32 = (s_1_72.value() as u32);
        // D s_1_74: cast zx s_1_65 -> bv
        let s_1_74: Bits = Bits::new(s_1_65 as u128, 32u16);
        // D s_1_75: cast zx s_1_73 -> bv
        let s_1_75: Bits = Bits::new(s_1_73 as u128, 32u16);
        // D s_1_76: add s_1_74 s_1_75
        let s_1_76: Bits = (s_1_74 + s_1_75);
        // D s_1_77: cast reint s_1_76 -> u32
        let s_1_77: u32 = (s_1_76.value() as u32);
        // D s_1_78: cast zx s_1_77 -> bv
        let s_1_78: Bits = Bits::new(s_1_77 as u128, 32u16);
        // D s_1_79: cast zx s_1_21 -> bv
        let s_1_79: Bits = Bits::new(s_1_21 as u128, 32u16);
        // D s_1_80: add s_1_78 s_1_79
        let s_1_80: Bits = (s_1_78 + s_1_79);
        // D s_1_81: cast reint s_1_80 -> u32
        let s_1_81: u32 = (s_1_80.value() as u32);
        // C s_1_82: const #0s : i
        let s_1_82: i128 = 0;
        // D s_1_83: cast zx s_1_81 -> bv
        let s_1_83: Bits = Bits::new(s_1_81 as u128, 32u16);
        // C s_1_84: const #1s : i64
        let s_1_84: i64 = 1;
        // C s_1_85: cast zx s_1_84 -> i
        let s_1_85: i128 = (i128::try_from(s_1_84).unwrap());
        // C s_1_86: const #31s : i
        let s_1_86: i128 = 31;
        // C s_1_87: add s_1_86 s_1_85
        let s_1_87: i128 = (s_1_86 + s_1_85);
        // D s_1_88: bit-extract s_1_83 s_1_82 s_1_87
        let s_1_88: Bits = (Bits::new(
            ((s_1_83) >> (s_1_82)).value(),
            u16::try_from(s_1_87).unwrap(),
        ));
        // D s_1_89: cast reint s_1_88 -> u32
        let s_1_89: u32 = (s_1_88.value() as u32);
        // C s_1_90: const #32s : i
        let s_1_90: i128 = 32;
        // D s_1_91: cast zx s_1_14 -> bv
        let s_1_91: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_92: const #1s : i64
        let s_1_92: i64 = 1;
        // C s_1_93: cast zx s_1_92 -> i
        let s_1_93: i128 = (i128::try_from(s_1_92).unwrap());
        // C s_1_94: const #31s : i
        let s_1_94: i128 = 31;
        // C s_1_95: add s_1_94 s_1_93
        let s_1_95: i128 = (s_1_94 + s_1_93);
        // D s_1_96: bit-extract s_1_91 s_1_90 s_1_95
        let s_1_96: Bits = (Bits::new(
            ((s_1_91) >> (s_1_90)).value(),
            u16::try_from(s_1_95).unwrap(),
        ));
        // D s_1_97: cast reint s_1_96 -> u32
        let s_1_97: u32 = (s_1_96.value() as u32);
        // C s_1_98: const #0s : i
        let s_1_98: i128 = 0;
        // D s_1_99: read-var result:u128
        let s_1_99: u128 = fn_state.result;
        // D s_1_100: cast zx s_1_99 -> bv
        let s_1_100: Bits = Bits::new(s_1_99 as u128, 128u16);
        // D s_1_101: cast zx s_1_97 -> bv
        let s_1_101: Bits = Bits::new(s_1_97 as u128, 32u16);
        // C s_1_102: const #31s : i
        let s_1_102: i128 = 31;
        // C s_1_103: const #1u : u64
        let s_1_103: u64 = 1;
        // C s_1_104: cast zx s_1_103 -> bv
        let s_1_104: Bits = Bits::new(s_1_103 as u128, 64u16);
        // C s_1_105: lsl s_1_104 s_1_102
        let s_1_105: Bits = s_1_104 << s_1_102;
        // C s_1_106: sub s_1_105 s_1_104
        let s_1_106: Bits = ((s_1_105) - (s_1_104));
        // D s_1_107: and s_1_101 s_1_106
        let s_1_107: Bits = ((s_1_101) & (s_1_106));
        // D s_1_108: lsl s_1_107 s_1_98
        let s_1_108: Bits = s_1_107 << s_1_98;
        // C s_1_109: lsl s_1_106 s_1_98
        let s_1_109: Bits = s_1_106 << s_1_98;
        // C s_1_110: cmpl s_1_109
        let s_1_110: Bits = !s_1_109;
        // D s_1_111: and s_1_100 s_1_110
        let s_1_111: Bits = ((s_1_100) & (s_1_110));
        // D s_1_112: or s_1_111 s_1_108
        let s_1_112: Bits = ((s_1_111) | (s_1_108));
        // D s_1_113: cast reint s_1_112 -> u128
        let s_1_113: u128 = (s_1_112.value() as u128);
        // D s_1_114: write-var result <= s_1_113
        fn_state.result = s_1_113;
        // C s_1_115: const #64s : i
        let s_1_115: i128 = 64;
        // D s_1_116: cast zx s_1_14 -> bv
        let s_1_116: Bits = Bits::new(s_1_14 as u128, 128u16);
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
        // C s_1_123: const #19s : i
        let s_1_123: i128 = 19;
        // D s_1_124: cast zx s_1_122 -> bv
        let s_1_124: Bits = Bits::new(s_1_122 as u128, 32u16);
        // D s_1_125: call ROL(s_1_124, s_1_123)
        let s_1_125: Bits = ROL(state, tracer, s_1_124, s_1_123);
        // D s_1_126: cast reint s_1_125 -> u32
        let s_1_126: u32 = (s_1_125.value() as u32);
        // C s_1_127: const #32s : i
        let s_1_127: i128 = 32;
        // D s_1_128: cast zx s_1_113 -> bv
        let s_1_128: Bits = Bits::new(s_1_113 as u128, 128u16);
        // D s_1_129: cast zx s_1_126 -> bv
        let s_1_129: Bits = Bits::new(s_1_126 as u128, 32u16);
        // C s_1_130: const #31s : i
        let s_1_130: i128 = 31;
        // C s_1_131: const #1u : u64
        let s_1_131: u64 = 1;
        // C s_1_132: cast zx s_1_131 -> bv
        let s_1_132: Bits = Bits::new(s_1_131 as u128, 64u16);
        // C s_1_133: lsl s_1_132 s_1_130
        let s_1_133: Bits = s_1_132 << s_1_130;
        // C s_1_134: sub s_1_133 s_1_132
        let s_1_134: Bits = ((s_1_133) - (s_1_132));
        // D s_1_135: and s_1_129 s_1_134
        let s_1_135: Bits = ((s_1_129) & (s_1_134));
        // D s_1_136: lsl s_1_135 s_1_127
        let s_1_136: Bits = s_1_135 << s_1_127;
        // C s_1_137: lsl s_1_134 s_1_127
        let s_1_137: Bits = s_1_134 << s_1_127;
        // C s_1_138: cmpl s_1_137
        let s_1_138: Bits = !s_1_137;
        // D s_1_139: and s_1_128 s_1_138
        let s_1_139: Bits = ((s_1_128) & (s_1_138));
        // D s_1_140: or s_1_139 s_1_136
        let s_1_140: Bits = ((s_1_139) | (s_1_136));
        // D s_1_141: cast reint s_1_140 -> u128
        let s_1_141: u128 = (s_1_140.value() as u128);
        // D s_1_142: write-var result <= s_1_141
        fn_state.result = s_1_141;
        // C s_1_143: const #96s : i
        let s_1_143: i128 = 96;
        // D s_1_144: cast zx s_1_14 -> bv
        let s_1_144: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_145: const #1s : i64
        let s_1_145: i64 = 1;
        // C s_1_146: cast zx s_1_145 -> i
        let s_1_146: i128 = (i128::try_from(s_1_145).unwrap());
        // C s_1_147: const #31s : i
        let s_1_147: i128 = 31;
        // C s_1_148: add s_1_147 s_1_146
        let s_1_148: i128 = (s_1_147 + s_1_146);
        // D s_1_149: bit-extract s_1_144 s_1_143 s_1_148
        let s_1_149: Bits = (Bits::new(
            ((s_1_144) >> (s_1_143)).value(),
            u16::try_from(s_1_148).unwrap(),
        ));
        // D s_1_150: cast reint s_1_149 -> u32
        let s_1_150: u32 = (s_1_149.value() as u32);
        // C s_1_151: const #64s : i
        let s_1_151: i128 = 64;
        // D s_1_152: cast zx s_1_141 -> bv
        let s_1_152: Bits = Bits::new(s_1_141 as u128, 128u16);
        // D s_1_153: cast zx s_1_150 -> bv
        let s_1_153: Bits = Bits::new(s_1_150 as u128, 32u16);
        // C s_1_154: const #31s : i
        let s_1_154: i128 = 31;
        // C s_1_155: const #1u : u64
        let s_1_155: u64 = 1;
        // C s_1_156: cast zx s_1_155 -> bv
        let s_1_156: Bits = Bits::new(s_1_155 as u128, 64u16);
        // C s_1_157: lsl s_1_156 s_1_154
        let s_1_157: Bits = s_1_156 << s_1_154;
        // C s_1_158: sub s_1_157 s_1_156
        let s_1_158: Bits = ((s_1_157) - (s_1_156));
        // D s_1_159: and s_1_153 s_1_158
        let s_1_159: Bits = ((s_1_153) & (s_1_158));
        // D s_1_160: lsl s_1_159 s_1_151
        let s_1_160: Bits = s_1_159 << s_1_151;
        // C s_1_161: lsl s_1_158 s_1_151
        let s_1_161: Bits = s_1_158 << s_1_151;
        // C s_1_162: cmpl s_1_161
        let s_1_162: Bits = !s_1_161;
        // D s_1_163: and s_1_152 s_1_162
        let s_1_163: Bits = ((s_1_152) & (s_1_162));
        // D s_1_164: or s_1_163 s_1_160
        let s_1_164: Bits = ((s_1_163) | (s_1_160));
        // D s_1_165: cast reint s_1_164 -> u128
        let s_1_165: u128 = (s_1_164.value() as u128);
        // D s_1_166: write-var result <= s_1_165
        fn_state.result = s_1_165;
        // C s_1_167: const #9s : i
        let s_1_167: i128 = 9;
        // D s_1_168: cast zx s_1_89 -> bv
        let s_1_168: Bits = Bits::new(s_1_89 as u128, 32u16);
        // D s_1_169: call ROL(s_1_168, s_1_167)
        let s_1_169: Bits = ROL(state, tracer, s_1_168, s_1_167);
        // D s_1_170: cast reint s_1_169 -> u32
        let s_1_170: u32 = (s_1_169.value() as u32);
        // D s_1_171: cast zx s_1_89 -> bv
        let s_1_171: Bits = Bits::new(s_1_89 as u128, 32u16);
        // D s_1_172: cast zx s_1_170 -> bv
        let s_1_172: Bits = Bits::new(s_1_170 as u128, 32u16);
        // D s_1_173: xor s_1_171 s_1_172
        let s_1_173: Bits = ((s_1_171) ^ (s_1_172));
        // D s_1_174: cast reint s_1_173 -> u32
        let s_1_174: u32 = (s_1_173.value() as u32);
        // C s_1_175: const #17s : i
        let s_1_175: i128 = 17;
        // D s_1_176: cast zx s_1_89 -> bv
        let s_1_176: Bits = Bits::new(s_1_89 as u128, 32u16);
        // D s_1_177: call ROL(s_1_176, s_1_175)
        let s_1_177: Bits = ROL(state, tracer, s_1_176, s_1_175);
        // D s_1_178: cast reint s_1_177 -> u32
        let s_1_178: u32 = (s_1_177.value() as u32);
        // D s_1_179: cast zx s_1_174 -> bv
        let s_1_179: Bits = Bits::new(s_1_174 as u128, 32u16);
        // D s_1_180: cast zx s_1_178 -> bv
        let s_1_180: Bits = Bits::new(s_1_178 as u128, 32u16);
        // D s_1_181: xor s_1_179 s_1_180
        let s_1_181: Bits = ((s_1_179) ^ (s_1_180));
        // D s_1_182: cast reint s_1_181 -> u32
        let s_1_182: u32 = (s_1_181.value() as u32);
        // C s_1_183: const #96s : i
        let s_1_183: i128 = 96;
        // D s_1_184: cast zx s_1_165 -> bv
        let s_1_184: Bits = Bits::new(s_1_165 as u128, 128u16);
        // D s_1_185: cast zx s_1_182 -> bv
        let s_1_185: Bits = Bits::new(s_1_182 as u128, 32u16);
        // C s_1_186: const #31s : i
        let s_1_186: i128 = 31;
        // C s_1_187: const #1u : u64
        let s_1_187: u64 = 1;
        // C s_1_188: cast zx s_1_187 -> bv
        let s_1_188: Bits = Bits::new(s_1_187 as u128, 64u16);
        // C s_1_189: lsl s_1_188 s_1_186
        let s_1_189: Bits = s_1_188 << s_1_186;
        // C s_1_190: sub s_1_189 s_1_188
        let s_1_190: Bits = ((s_1_189) - (s_1_188));
        // D s_1_191: and s_1_185 s_1_190
        let s_1_191: Bits = ((s_1_185) & (s_1_190));
        // D s_1_192: lsl s_1_191 s_1_183
        let s_1_192: Bits = s_1_191 << s_1_183;
        // C s_1_193: lsl s_1_190 s_1_183
        let s_1_193: Bits = s_1_190 << s_1_183;
        // C s_1_194: cmpl s_1_193
        let s_1_194: Bits = !s_1_193;
        // D s_1_195: and s_1_184 s_1_194
        let s_1_195: Bits = ((s_1_184) & (s_1_194));
        // D s_1_196: or s_1_195 s_1_192
        let s_1_196: Bits = ((s_1_195) | (s_1_192));
        // D s_1_197: cast reint s_1_196 -> u128
        let s_1_197: u128 = (s_1_196.value() as u128);
        // D s_1_198: write-var result <= s_1_197
        fn_state.result = s_1_197;
        // C s_1_199: const #128s : i64
        let s_1_199: i64 = 128;
        // D s_1_200: read-var d:i64
        let s_1_200: i64 = fn_state.d;
        // D s_1_201: cast zx s_1_200 -> i
        let s_1_201: i128 = (i128::try_from(s_1_200).unwrap());
        // D s_1_202: cast zx s_1_197 -> bv
        let s_1_202: Bits = Bits::new(s_1_197 as u128, 128u16);
        // D s_1_203: call V_set(s_1_201, s_1_199, s_1_202)
        let s_1_203: () = V_set(state, tracer, s_1_201, s_1_199, s_1_202);
        // N s_1_204: return
        return;
    }
}
