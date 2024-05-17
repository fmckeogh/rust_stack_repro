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
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3tt1a<T: Tracer>(
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
        // D s_1_23: cast zx s_1_9 -> bv
        let s_1_23: Bits = Bits::new(s_1_9 as u128, 128u16);
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
        // C s_1_38: const #12s : i
        let s_1_38: i128 = 12;
        // D s_1_39: cast zx s_1_37 -> bv
        let s_1_39: Bits = Bits::new(s_1_37 as u128, 32u16);
        // D s_1_40: call ROL(s_1_39, s_1_38)
        let s_1_40: Bits = ROL(state, tracer, s_1_39, s_1_38);
        // D s_1_41: cast reint s_1_40 -> u32
        let s_1_41: u32 = (s_1_40.value() as u32);
        // D s_1_42: cast zx s_1_29 -> bv
        let s_1_42: Bits = Bits::new(s_1_29 as u128, 32u16);
        // D s_1_43: cast zx s_1_41 -> bv
        let s_1_43: Bits = Bits::new(s_1_41 as u128, 32u16);
        // D s_1_44: xor s_1_42 s_1_43
        let s_1_44: Bits = ((s_1_42) ^ (s_1_43));
        // D s_1_45: cast reint s_1_44 -> u32
        let s_1_45: u32 = (s_1_44.value() as u32);
        // C s_1_46: const #32s : i
        let s_1_46: i128 = 32;
        // D s_1_47: cast zx s_1_14 -> bv
        let s_1_47: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_48: const #1s : i64
        let s_1_48: i64 = 1;
        // C s_1_49: cast zx s_1_48 -> i
        let s_1_49: i128 = (i128::try_from(s_1_48).unwrap());
        // C s_1_50: const #31s : i
        let s_1_50: i128 = 31;
        // C s_1_51: add s_1_50 s_1_49
        let s_1_51: i128 = (s_1_50 + s_1_49);
        // D s_1_52: bit-extract s_1_47 s_1_46 s_1_51
        let s_1_52: Bits = (Bits::new(
            ((s_1_47) >> (s_1_46)).value(),
            u16::try_from(s_1_51).unwrap(),
        ));
        // D s_1_53: cast reint s_1_52 -> u32
        let s_1_53: u32 = (s_1_52.value() as u32);
        // C s_1_54: const #96s : i
        let s_1_54: i128 = 96;
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
        // C s_1_62: const #64s : i
        let s_1_62: i128 = 64;
        // D s_1_63: cast zx s_1_14 -> bv
        let s_1_63: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_64: const #1s : i64
        let s_1_64: i64 = 1;
        // C s_1_65: cast zx s_1_64 -> i
        let s_1_65: i128 = (i128::try_from(s_1_64).unwrap());
        // C s_1_66: const #31s : i
        let s_1_66: i128 = 31;
        // C s_1_67: add s_1_66 s_1_65
        let s_1_67: i128 = (s_1_66 + s_1_65);
        // D s_1_68: bit-extract s_1_63 s_1_62 s_1_67
        let s_1_68: Bits = (Bits::new(
            ((s_1_63) >> (s_1_62)).value(),
            u16::try_from(s_1_67).unwrap(),
        ));
        // D s_1_69: cast reint s_1_68 -> u32
        let s_1_69: u32 = (s_1_68.value() as u32);
        // D s_1_70: cast zx s_1_61 -> bv
        let s_1_70: Bits = Bits::new(s_1_61 as u128, 32u16);
        // D s_1_71: cast zx s_1_69 -> bv
        let s_1_71: Bits = Bits::new(s_1_69 as u128, 32u16);
        // D s_1_72: xor s_1_70 s_1_71
        let s_1_72: Bits = ((s_1_70) ^ (s_1_71));
        // D s_1_73: cast reint s_1_72 -> u32
        let s_1_73: u32 = (s_1_72.value() as u32);
        // D s_1_74: cast zx s_1_53 -> bv
        let s_1_74: Bits = Bits::new(s_1_53 as u128, 32u16);
        // D s_1_75: cast zx s_1_73 -> bv
        let s_1_75: Bits = Bits::new(s_1_73 as u128, 32u16);
        // D s_1_76: xor s_1_74 s_1_75
        let s_1_76: Bits = ((s_1_74) ^ (s_1_75));
        // D s_1_77: cast reint s_1_76 -> u32
        let s_1_77: u32 = (s_1_76.value() as u32);
        // C s_1_78: const #0s : i
        let s_1_78: i128 = 0;
        // D s_1_79: cast zx s_1_14 -> bv
        let s_1_79: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_80: const #1s : i64
        let s_1_80: i64 = 1;
        // C s_1_81: cast zx s_1_80 -> i
        let s_1_81: i128 = (i128::try_from(s_1_80).unwrap());
        // C s_1_82: const #31s : i
        let s_1_82: i128 = 31;
        // C s_1_83: add s_1_82 s_1_81
        let s_1_83: i128 = (s_1_82 + s_1_81);
        // D s_1_84: bit-extract s_1_79 s_1_78 s_1_83
        let s_1_84: Bits = (Bits::new(
            ((s_1_79) >> (s_1_78)).value(),
            u16::try_from(s_1_83).unwrap(),
        ));
        // D s_1_85: cast reint s_1_84 -> u32
        let s_1_85: u32 = (s_1_84.value() as u32);
        // D s_1_86: cast zx s_1_77 -> bv
        let s_1_86: Bits = Bits::new(s_1_77 as u128, 32u16);
        // D s_1_87: cast zx s_1_85 -> bv
        let s_1_87: Bits = Bits::new(s_1_85 as u128, 32u16);
        // D s_1_88: add s_1_86 s_1_87
        let s_1_88: Bits = (s_1_86 + s_1_87);
        // D s_1_89: cast reint s_1_88 -> u32
        let s_1_89: u32 = (s_1_88.value() as u32);
        // D s_1_90: cast zx s_1_89 -> bv
        let s_1_90: Bits = Bits::new(s_1_89 as u128, 32u16);
        // D s_1_91: cast zx s_1_45 -> bv
        let s_1_91: Bits = Bits::new(s_1_45 as u128, 32u16);
        // D s_1_92: add s_1_90 s_1_91
        let s_1_92: Bits = (s_1_90 + s_1_91);
        // D s_1_93: cast reint s_1_92 -> u32
        let s_1_93: u32 = (s_1_92.value() as u32);
        // D s_1_94: cast zx s_1_93 -> bv
        let s_1_94: Bits = Bits::new(s_1_93 as u128, 32u16);
        // D s_1_95: cast zx s_1_21 -> bv
        let s_1_95: Bits = Bits::new(s_1_21 as u128, 32u16);
        // D s_1_96: add s_1_94 s_1_95
        let s_1_96: Bits = (s_1_94 + s_1_95);
        // D s_1_97: cast reint s_1_96 -> u32
        let s_1_97: u32 = (s_1_96.value() as u32);
        // C s_1_98: const #0s : i
        let s_1_98: i128 = 0;
        // D s_1_99: cast zx s_1_97 -> bv
        let s_1_99: Bits = Bits::new(s_1_97 as u128, 32u16);
        // C s_1_100: const #1s : i64
        let s_1_100: i64 = 1;
        // C s_1_101: cast zx s_1_100 -> i
        let s_1_101: i128 = (i128::try_from(s_1_100).unwrap());
        // C s_1_102: const #31s : i
        let s_1_102: i128 = 31;
        // C s_1_103: add s_1_102 s_1_101
        let s_1_103: i128 = (s_1_102 + s_1_101);
        // D s_1_104: bit-extract s_1_99 s_1_98 s_1_103
        let s_1_104: Bits = (Bits::new(
            ((s_1_99) >> (s_1_98)).value(),
            u16::try_from(s_1_103).unwrap(),
        ));
        // D s_1_105: cast reint s_1_104 -> u32
        let s_1_105: u32 = (s_1_104.value() as u32);
        // C s_1_106: const #32s : i
        let s_1_106: i128 = 32;
        // D s_1_107: cast zx s_1_14 -> bv
        let s_1_107: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_108: const #1s : i64
        let s_1_108: i64 = 1;
        // C s_1_109: cast zx s_1_108 -> i
        let s_1_109: i128 = (i128::try_from(s_1_108).unwrap());
        // C s_1_110: const #31s : i
        let s_1_110: i128 = 31;
        // C s_1_111: add s_1_110 s_1_109
        let s_1_111: i128 = (s_1_110 + s_1_109);
        // D s_1_112: bit-extract s_1_107 s_1_106 s_1_111
        let s_1_112: Bits = (Bits::new(
            ((s_1_107) >> (s_1_106)).value(),
            u16::try_from(s_1_111).unwrap(),
        ));
        // D s_1_113: cast reint s_1_112 -> u32
        let s_1_113: u32 = (s_1_112.value() as u32);
        // C s_1_114: const #0s : i
        let s_1_114: i128 = 0;
        // D s_1_115: read-var result:u128
        let s_1_115: u128 = fn_state.result;
        // D s_1_116: cast zx s_1_115 -> bv
        let s_1_116: Bits = Bits::new(s_1_115 as u128, 128u16);
        // D s_1_117: cast zx s_1_113 -> bv
        let s_1_117: Bits = Bits::new(s_1_113 as u128, 32u16);
        // C s_1_118: const #31s : i
        let s_1_118: i128 = 31;
        // C s_1_119: const #1u : u64
        let s_1_119: u64 = 1;
        // C s_1_120: cast zx s_1_119 -> bv
        let s_1_120: Bits = Bits::new(s_1_119 as u128, 64u16);
        // C s_1_121: lsl s_1_120 s_1_118
        let s_1_121: Bits = s_1_120 << s_1_118;
        // C s_1_122: sub s_1_121 s_1_120
        let s_1_122: Bits = ((s_1_121) - (s_1_120));
        // D s_1_123: and s_1_117 s_1_122
        let s_1_123: Bits = ((s_1_117) & (s_1_122));
        // D s_1_124: lsl s_1_123 s_1_114
        let s_1_124: Bits = s_1_123 << s_1_114;
        // C s_1_125: lsl s_1_122 s_1_114
        let s_1_125: Bits = s_1_122 << s_1_114;
        // C s_1_126: cmpl s_1_125
        let s_1_126: Bits = !s_1_125;
        // D s_1_127: and s_1_116 s_1_126
        let s_1_127: Bits = ((s_1_116) & (s_1_126));
        // D s_1_128: or s_1_127 s_1_124
        let s_1_128: Bits = ((s_1_127) | (s_1_124));
        // D s_1_129: cast reint s_1_128 -> u128
        let s_1_129: u128 = (s_1_128.value() as u128);
        // D s_1_130: write-var result <= s_1_129
        fn_state.result = s_1_129;
        // C s_1_131: const #64s : i
        let s_1_131: i128 = 64;
        // D s_1_132: cast zx s_1_14 -> bv
        let s_1_132: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_133: const #1s : i64
        let s_1_133: i64 = 1;
        // C s_1_134: cast zx s_1_133 -> i
        let s_1_134: i128 = (i128::try_from(s_1_133).unwrap());
        // C s_1_135: const #31s : i
        let s_1_135: i128 = 31;
        // C s_1_136: add s_1_135 s_1_134
        let s_1_136: i128 = (s_1_135 + s_1_134);
        // D s_1_137: bit-extract s_1_132 s_1_131 s_1_136
        let s_1_137: Bits = (Bits::new(
            ((s_1_132) >> (s_1_131)).value(),
            u16::try_from(s_1_136).unwrap(),
        ));
        // D s_1_138: cast reint s_1_137 -> u32
        let s_1_138: u32 = (s_1_137.value() as u32);
        // C s_1_139: const #9s : i
        let s_1_139: i128 = 9;
        // D s_1_140: cast zx s_1_138 -> bv
        let s_1_140: Bits = Bits::new(s_1_138 as u128, 32u16);
        // D s_1_141: call ROL(s_1_140, s_1_139)
        let s_1_141: Bits = ROL(state, tracer, s_1_140, s_1_139);
        // D s_1_142: cast reint s_1_141 -> u32
        let s_1_142: u32 = (s_1_141.value() as u32);
        // C s_1_143: const #32s : i
        let s_1_143: i128 = 32;
        // D s_1_144: cast zx s_1_129 -> bv
        let s_1_144: Bits = Bits::new(s_1_129 as u128, 128u16);
        // D s_1_145: cast zx s_1_142 -> bv
        let s_1_145: Bits = Bits::new(s_1_142 as u128, 32u16);
        // C s_1_146: const #31s : i
        let s_1_146: i128 = 31;
        // C s_1_147: const #1u : u64
        let s_1_147: u64 = 1;
        // C s_1_148: cast zx s_1_147 -> bv
        let s_1_148: Bits = Bits::new(s_1_147 as u128, 64u16);
        // C s_1_149: lsl s_1_148 s_1_146
        let s_1_149: Bits = s_1_148 << s_1_146;
        // C s_1_150: sub s_1_149 s_1_148
        let s_1_150: Bits = ((s_1_149) - (s_1_148));
        // D s_1_151: and s_1_145 s_1_150
        let s_1_151: Bits = ((s_1_145) & (s_1_150));
        // D s_1_152: lsl s_1_151 s_1_143
        let s_1_152: Bits = s_1_151 << s_1_143;
        // C s_1_153: lsl s_1_150 s_1_143
        let s_1_153: Bits = s_1_150 << s_1_143;
        // C s_1_154: cmpl s_1_153
        let s_1_154: Bits = !s_1_153;
        // D s_1_155: and s_1_144 s_1_154
        let s_1_155: Bits = ((s_1_144) & (s_1_154));
        // D s_1_156: or s_1_155 s_1_152
        let s_1_156: Bits = ((s_1_155) | (s_1_152));
        // D s_1_157: cast reint s_1_156 -> u128
        let s_1_157: u128 = (s_1_156.value() as u128);
        // D s_1_158: write-var result <= s_1_157
        fn_state.result = s_1_157;
        // C s_1_159: const #96s : i
        let s_1_159: i128 = 96;
        // D s_1_160: cast zx s_1_14 -> bv
        let s_1_160: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_161: const #1s : i64
        let s_1_161: i64 = 1;
        // C s_1_162: cast zx s_1_161 -> i
        let s_1_162: i128 = (i128::try_from(s_1_161).unwrap());
        // C s_1_163: const #31s : i
        let s_1_163: i128 = 31;
        // C s_1_164: add s_1_163 s_1_162
        let s_1_164: i128 = (s_1_163 + s_1_162);
        // D s_1_165: bit-extract s_1_160 s_1_159 s_1_164
        let s_1_165: Bits = (Bits::new(
            ((s_1_160) >> (s_1_159)).value(),
            u16::try_from(s_1_164).unwrap(),
        ));
        // D s_1_166: cast reint s_1_165 -> u32
        let s_1_166: u32 = (s_1_165.value() as u32);
        // C s_1_167: const #64s : i
        let s_1_167: i128 = 64;
        // D s_1_168: cast zx s_1_157 -> bv
        let s_1_168: Bits = Bits::new(s_1_157 as u128, 128u16);
        // D s_1_169: cast zx s_1_166 -> bv
        let s_1_169: Bits = Bits::new(s_1_166 as u128, 32u16);
        // C s_1_170: const #31s : i
        let s_1_170: i128 = 31;
        // C s_1_171: const #1u : u64
        let s_1_171: u64 = 1;
        // C s_1_172: cast zx s_1_171 -> bv
        let s_1_172: Bits = Bits::new(s_1_171 as u128, 64u16);
        // C s_1_173: lsl s_1_172 s_1_170
        let s_1_173: Bits = s_1_172 << s_1_170;
        // C s_1_174: sub s_1_173 s_1_172
        let s_1_174: Bits = ((s_1_173) - (s_1_172));
        // D s_1_175: and s_1_169 s_1_174
        let s_1_175: Bits = ((s_1_169) & (s_1_174));
        // D s_1_176: lsl s_1_175 s_1_167
        let s_1_176: Bits = s_1_175 << s_1_167;
        // C s_1_177: lsl s_1_174 s_1_167
        let s_1_177: Bits = s_1_174 << s_1_167;
        // C s_1_178: cmpl s_1_177
        let s_1_178: Bits = !s_1_177;
        // D s_1_179: and s_1_168 s_1_178
        let s_1_179: Bits = ((s_1_168) & (s_1_178));
        // D s_1_180: or s_1_179 s_1_176
        let s_1_180: Bits = ((s_1_179) | (s_1_176));
        // D s_1_181: cast reint s_1_180 -> u128
        let s_1_181: u128 = (s_1_180.value() as u128);
        // D s_1_182: write-var result <= s_1_181
        fn_state.result = s_1_181;
        // C s_1_183: const #96s : i
        let s_1_183: i128 = 96;
        // D s_1_184: cast zx s_1_181 -> bv
        let s_1_184: Bits = Bits::new(s_1_181 as u128, 128u16);
        // D s_1_185: cast zx s_1_105 -> bv
        let s_1_185: Bits = Bits::new(s_1_105 as u128, 32u16);
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
