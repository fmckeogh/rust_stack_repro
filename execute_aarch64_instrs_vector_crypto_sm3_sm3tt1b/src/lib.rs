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
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3tt1b<T: Tracer>(
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
        // C s_1_46: const #96s : i
        let s_1_46: i128 = 96;
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
        // C s_1_54: const #32s : i
        let s_1_54: i128 = 32;
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
        // D s_1_64: and s_1_62 s_1_63
        let s_1_64: Bits = ((s_1_62) & (s_1_63));
        // D s_1_65: cast reint s_1_64 -> u32
        let s_1_65: u32 = (s_1_64.value() as u32);
        // C s_1_66: const #96s : i
        let s_1_66: i128 = 96;
        // D s_1_67: cast zx s_1_14 -> bv
        let s_1_67: Bits = Bits::new(s_1_14 as u128, 128u16);
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
        // C s_1_74: const #64s : i
        let s_1_74: i128 = 64;
        // D s_1_75: cast zx s_1_14 -> bv
        let s_1_75: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_76: const #1s : i64
        let s_1_76: i64 = 1;
        // C s_1_77: cast zx s_1_76 -> i
        let s_1_77: i128 = (i128::try_from(s_1_76).unwrap());
        // C s_1_78: const #31s : i
        let s_1_78: i128 = 31;
        // C s_1_79: add s_1_78 s_1_77
        let s_1_79: i128 = (s_1_78 + s_1_77);
        // D s_1_80: bit-extract s_1_75 s_1_74 s_1_79
        let s_1_80: Bits = (Bits::new(
            ((s_1_75) >> (s_1_74)).value(),
            u16::try_from(s_1_79).unwrap(),
        ));
        // D s_1_81: cast reint s_1_80 -> u32
        let s_1_81: u32 = (s_1_80.value() as u32);
        // D s_1_82: cast zx s_1_73 -> bv
        let s_1_82: Bits = Bits::new(s_1_73 as u128, 32u16);
        // D s_1_83: cast zx s_1_81 -> bv
        let s_1_83: Bits = Bits::new(s_1_81 as u128, 32u16);
        // D s_1_84: and s_1_82 s_1_83
        let s_1_84: Bits = ((s_1_82) & (s_1_83));
        // D s_1_85: cast reint s_1_84 -> u32
        let s_1_85: u32 = (s_1_84.value() as u32);
        // D s_1_86: cast zx s_1_65 -> bv
        let s_1_86: Bits = Bits::new(s_1_65 as u128, 32u16);
        // D s_1_87: cast zx s_1_85 -> bv
        let s_1_87: Bits = Bits::new(s_1_85 as u128, 32u16);
        // D s_1_88: or s_1_86 s_1_87
        let s_1_88: Bits = ((s_1_86) | (s_1_87));
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
        // C s_1_98: const #64s : i
        let s_1_98: i128 = 64;
        // D s_1_99: cast zx s_1_14 -> bv
        let s_1_99: Bits = Bits::new(s_1_14 as u128, 128u16);
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
        // D s_1_106: cast zx s_1_97 -> bv
        let s_1_106: Bits = Bits::new(s_1_97 as u128, 32u16);
        // D s_1_107: cast zx s_1_105 -> bv
        let s_1_107: Bits = Bits::new(s_1_105 as u128, 32u16);
        // D s_1_108: and s_1_106 s_1_107
        let s_1_108: Bits = ((s_1_106) & (s_1_107));
        // D s_1_109: cast reint s_1_108 -> u32
        let s_1_109: u32 = (s_1_108.value() as u32);
        // D s_1_110: cast zx s_1_89 -> bv
        let s_1_110: Bits = Bits::new(s_1_89 as u128, 32u16);
        // D s_1_111: cast zx s_1_109 -> bv
        let s_1_111: Bits = Bits::new(s_1_109 as u128, 32u16);
        // D s_1_112: or s_1_110 s_1_111
        let s_1_112: Bits = ((s_1_110) | (s_1_111));
        // D s_1_113: cast reint s_1_112 -> u32
        let s_1_113: u32 = (s_1_112.value() as u32);
        // C s_1_114: const #0s : i
        let s_1_114: i128 = 0;
        // D s_1_115: cast zx s_1_14 -> bv
        let s_1_115: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_116: const #1s : i64
        let s_1_116: i64 = 1;
        // C s_1_117: cast zx s_1_116 -> i
        let s_1_117: i128 = (i128::try_from(s_1_116).unwrap());
        // C s_1_118: const #31s : i
        let s_1_118: i128 = 31;
        // C s_1_119: add s_1_118 s_1_117
        let s_1_119: i128 = (s_1_118 + s_1_117);
        // D s_1_120: bit-extract s_1_115 s_1_114 s_1_119
        let s_1_120: Bits = (Bits::new(
            ((s_1_115) >> (s_1_114)).value(),
            u16::try_from(s_1_119).unwrap(),
        ));
        // D s_1_121: cast reint s_1_120 -> u32
        let s_1_121: u32 = (s_1_120.value() as u32);
        // D s_1_122: cast zx s_1_113 -> bv
        let s_1_122: Bits = Bits::new(s_1_113 as u128, 32u16);
        // D s_1_123: cast zx s_1_121 -> bv
        let s_1_123: Bits = Bits::new(s_1_121 as u128, 32u16);
        // D s_1_124: add s_1_122 s_1_123
        let s_1_124: Bits = (s_1_122 + s_1_123);
        // D s_1_125: cast reint s_1_124 -> u32
        let s_1_125: u32 = (s_1_124.value() as u32);
        // D s_1_126: cast zx s_1_125 -> bv
        let s_1_126: Bits = Bits::new(s_1_125 as u128, 32u16);
        // D s_1_127: cast zx s_1_45 -> bv
        let s_1_127: Bits = Bits::new(s_1_45 as u128, 32u16);
        // D s_1_128: add s_1_126 s_1_127
        let s_1_128: Bits = (s_1_126 + s_1_127);
        // D s_1_129: cast reint s_1_128 -> u32
        let s_1_129: u32 = (s_1_128.value() as u32);
        // D s_1_130: cast zx s_1_129 -> bv
        let s_1_130: Bits = Bits::new(s_1_129 as u128, 32u16);
        // D s_1_131: cast zx s_1_21 -> bv
        let s_1_131: Bits = Bits::new(s_1_21 as u128, 32u16);
        // D s_1_132: add s_1_130 s_1_131
        let s_1_132: Bits = (s_1_130 + s_1_131);
        // D s_1_133: cast reint s_1_132 -> u32
        let s_1_133: u32 = (s_1_132.value() as u32);
        // C s_1_134: const #0s : i
        let s_1_134: i128 = 0;
        // D s_1_135: cast zx s_1_133 -> bv
        let s_1_135: Bits = Bits::new(s_1_133 as u128, 32u16);
        // C s_1_136: const #1s : i64
        let s_1_136: i64 = 1;
        // C s_1_137: cast zx s_1_136 -> i
        let s_1_137: i128 = (i128::try_from(s_1_136).unwrap());
        // C s_1_138: const #31s : i
        let s_1_138: i128 = 31;
        // C s_1_139: add s_1_138 s_1_137
        let s_1_139: i128 = (s_1_138 + s_1_137);
        // D s_1_140: bit-extract s_1_135 s_1_134 s_1_139
        let s_1_140: Bits = (Bits::new(
            ((s_1_135) >> (s_1_134)).value(),
            u16::try_from(s_1_139).unwrap(),
        ));
        // D s_1_141: cast reint s_1_140 -> u32
        let s_1_141: u32 = (s_1_140.value() as u32);
        // C s_1_142: const #32s : i
        let s_1_142: i128 = 32;
        // D s_1_143: cast zx s_1_14 -> bv
        let s_1_143: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_144: const #1s : i64
        let s_1_144: i64 = 1;
        // C s_1_145: cast zx s_1_144 -> i
        let s_1_145: i128 = (i128::try_from(s_1_144).unwrap());
        // C s_1_146: const #31s : i
        let s_1_146: i128 = 31;
        // C s_1_147: add s_1_146 s_1_145
        let s_1_147: i128 = (s_1_146 + s_1_145);
        // D s_1_148: bit-extract s_1_143 s_1_142 s_1_147
        let s_1_148: Bits = (Bits::new(
            ((s_1_143) >> (s_1_142)).value(),
            u16::try_from(s_1_147).unwrap(),
        ));
        // D s_1_149: cast reint s_1_148 -> u32
        let s_1_149: u32 = (s_1_148.value() as u32);
        // C s_1_150: const #0s : i
        let s_1_150: i128 = 0;
        // D s_1_151: read-var result:u128
        let s_1_151: u128 = fn_state.result;
        // D s_1_152: cast zx s_1_151 -> bv
        let s_1_152: Bits = Bits::new(s_1_151 as u128, 128u16);
        // D s_1_153: cast zx s_1_149 -> bv
        let s_1_153: Bits = Bits::new(s_1_149 as u128, 32u16);
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
        // D s_1_160: lsl s_1_159 s_1_150
        let s_1_160: Bits = s_1_159 << s_1_150;
        // C s_1_161: lsl s_1_158 s_1_150
        let s_1_161: Bits = s_1_158 << s_1_150;
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
        // C s_1_167: const #64s : i
        let s_1_167: i128 = 64;
        // D s_1_168: cast zx s_1_14 -> bv
        let s_1_168: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_169: const #1s : i64
        let s_1_169: i64 = 1;
        // C s_1_170: cast zx s_1_169 -> i
        let s_1_170: i128 = (i128::try_from(s_1_169).unwrap());
        // C s_1_171: const #31s : i
        let s_1_171: i128 = 31;
        // C s_1_172: add s_1_171 s_1_170
        let s_1_172: i128 = (s_1_171 + s_1_170);
        // D s_1_173: bit-extract s_1_168 s_1_167 s_1_172
        let s_1_173: Bits = (Bits::new(
            ((s_1_168) >> (s_1_167)).value(),
            u16::try_from(s_1_172).unwrap(),
        ));
        // D s_1_174: cast reint s_1_173 -> u32
        let s_1_174: u32 = (s_1_173.value() as u32);
        // C s_1_175: const #9s : i
        let s_1_175: i128 = 9;
        // D s_1_176: cast zx s_1_174 -> bv
        let s_1_176: Bits = Bits::new(s_1_174 as u128, 32u16);
        // D s_1_177: call ROL(s_1_176, s_1_175)
        let s_1_177: Bits = ROL(state, tracer, s_1_176, s_1_175);
        // D s_1_178: cast reint s_1_177 -> u32
        let s_1_178: u32 = (s_1_177.value() as u32);
        // C s_1_179: const #32s : i
        let s_1_179: i128 = 32;
        // D s_1_180: cast zx s_1_165 -> bv
        let s_1_180: Bits = Bits::new(s_1_165 as u128, 128u16);
        // D s_1_181: cast zx s_1_178 -> bv
        let s_1_181: Bits = Bits::new(s_1_178 as u128, 32u16);
        // C s_1_182: const #31s : i
        let s_1_182: i128 = 31;
        // C s_1_183: const #1u : u64
        let s_1_183: u64 = 1;
        // C s_1_184: cast zx s_1_183 -> bv
        let s_1_184: Bits = Bits::new(s_1_183 as u128, 64u16);
        // C s_1_185: lsl s_1_184 s_1_182
        let s_1_185: Bits = s_1_184 << s_1_182;
        // C s_1_186: sub s_1_185 s_1_184
        let s_1_186: Bits = ((s_1_185) - (s_1_184));
        // D s_1_187: and s_1_181 s_1_186
        let s_1_187: Bits = ((s_1_181) & (s_1_186));
        // D s_1_188: lsl s_1_187 s_1_179
        let s_1_188: Bits = s_1_187 << s_1_179;
        // C s_1_189: lsl s_1_186 s_1_179
        let s_1_189: Bits = s_1_186 << s_1_179;
        // C s_1_190: cmpl s_1_189
        let s_1_190: Bits = !s_1_189;
        // D s_1_191: and s_1_180 s_1_190
        let s_1_191: Bits = ((s_1_180) & (s_1_190));
        // D s_1_192: or s_1_191 s_1_188
        let s_1_192: Bits = ((s_1_191) | (s_1_188));
        // D s_1_193: cast reint s_1_192 -> u128
        let s_1_193: u128 = (s_1_192.value() as u128);
        // D s_1_194: write-var result <= s_1_193
        fn_state.result = s_1_193;
        // C s_1_195: const #96s : i
        let s_1_195: i128 = 96;
        // D s_1_196: cast zx s_1_14 -> bv
        let s_1_196: Bits = Bits::new(s_1_14 as u128, 128u16);
        // C s_1_197: const #1s : i64
        let s_1_197: i64 = 1;
        // C s_1_198: cast zx s_1_197 -> i
        let s_1_198: i128 = (i128::try_from(s_1_197).unwrap());
        // C s_1_199: const #31s : i
        let s_1_199: i128 = 31;
        // C s_1_200: add s_1_199 s_1_198
        let s_1_200: i128 = (s_1_199 + s_1_198);
        // D s_1_201: bit-extract s_1_196 s_1_195 s_1_200
        let s_1_201: Bits = (Bits::new(
            ((s_1_196) >> (s_1_195)).value(),
            u16::try_from(s_1_200).unwrap(),
        ));
        // D s_1_202: cast reint s_1_201 -> u32
        let s_1_202: u32 = (s_1_201.value() as u32);
        // C s_1_203: const #64s : i
        let s_1_203: i128 = 64;
        // D s_1_204: cast zx s_1_193 -> bv
        let s_1_204: Bits = Bits::new(s_1_193 as u128, 128u16);
        // D s_1_205: cast zx s_1_202 -> bv
        let s_1_205: Bits = Bits::new(s_1_202 as u128, 32u16);
        // C s_1_206: const #31s : i
        let s_1_206: i128 = 31;
        // C s_1_207: const #1u : u64
        let s_1_207: u64 = 1;
        // C s_1_208: cast zx s_1_207 -> bv
        let s_1_208: Bits = Bits::new(s_1_207 as u128, 64u16);
        // C s_1_209: lsl s_1_208 s_1_206
        let s_1_209: Bits = s_1_208 << s_1_206;
        // C s_1_210: sub s_1_209 s_1_208
        let s_1_210: Bits = ((s_1_209) - (s_1_208));
        // D s_1_211: and s_1_205 s_1_210
        let s_1_211: Bits = ((s_1_205) & (s_1_210));
        // D s_1_212: lsl s_1_211 s_1_203
        let s_1_212: Bits = s_1_211 << s_1_203;
        // C s_1_213: lsl s_1_210 s_1_203
        let s_1_213: Bits = s_1_210 << s_1_203;
        // C s_1_214: cmpl s_1_213
        let s_1_214: Bits = !s_1_213;
        // D s_1_215: and s_1_204 s_1_214
        let s_1_215: Bits = ((s_1_204) & (s_1_214));
        // D s_1_216: or s_1_215 s_1_212
        let s_1_216: Bits = ((s_1_215) | (s_1_212));
        // D s_1_217: cast reint s_1_216 -> u128
        let s_1_217: u128 = (s_1_216.value() as u128);
        // D s_1_218: write-var result <= s_1_217
        fn_state.result = s_1_217;
        // C s_1_219: const #96s : i
        let s_1_219: i128 = 96;
        // D s_1_220: cast zx s_1_217 -> bv
        let s_1_220: Bits = Bits::new(s_1_217 as u128, 128u16);
        // D s_1_221: cast zx s_1_141 -> bv
        let s_1_221: Bits = Bits::new(s_1_141 as u128, 32u16);
        // C s_1_222: const #31s : i
        let s_1_222: i128 = 31;
        // C s_1_223: const #1u : u64
        let s_1_223: u64 = 1;
        // C s_1_224: cast zx s_1_223 -> bv
        let s_1_224: Bits = Bits::new(s_1_223 as u128, 64u16);
        // C s_1_225: lsl s_1_224 s_1_222
        let s_1_225: Bits = s_1_224 << s_1_222;
        // C s_1_226: sub s_1_225 s_1_224
        let s_1_226: Bits = ((s_1_225) - (s_1_224));
        // D s_1_227: and s_1_221 s_1_226
        let s_1_227: Bits = ((s_1_221) & (s_1_226));
        // D s_1_228: lsl s_1_227 s_1_219
        let s_1_228: Bits = s_1_227 << s_1_219;
        // C s_1_229: lsl s_1_226 s_1_219
        let s_1_229: Bits = s_1_226 << s_1_219;
        // C s_1_230: cmpl s_1_229
        let s_1_230: Bits = !s_1_229;
        // D s_1_231: and s_1_220 s_1_230
        let s_1_231: Bits = ((s_1_220) & (s_1_230));
        // D s_1_232: or s_1_231 s_1_228
        let s_1_232: Bits = ((s_1_231) | (s_1_228));
        // D s_1_233: cast reint s_1_232 -> u128
        let s_1_233: u128 = (s_1_232.value() as u128);
        // D s_1_234: write-var result <= s_1_233
        fn_state.result = s_1_233;
        // C s_1_235: const #128s : i64
        let s_1_235: i64 = 128;
        // D s_1_236: read-var d:i64
        let s_1_236: i64 = fn_state.d;
        // D s_1_237: cast zx s_1_236 -> i
        let s_1_237: i128 = (i128::try_from(s_1_236).unwrap());
        // D s_1_238: cast zx s_1_233 -> bv
        let s_1_238: Bits = Bits::new(s_1_233 as u128, 128u16);
        // D s_1_239: call V_set(s_1_237, s_1_235, s_1_238)
        let s_1_239: () = V_set(state, tracer, s_1_237, s_1_235, s_1_238);
        // N s_1_240: return
        return;
    }
}
