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
use common::*;
pub fn reverse_endianness<T: Tracer>(state: &mut State, tracer: &T, xs: Bits) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        len: i64,
        return_value: Bits,
        xs: Bits,
    }
    let fn_state = FunctionState {
        xs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var xs:bv
        let s_0_0: Bits = fn_state.xs;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var len <= s_0_3
        fn_state.len = s_0_3;
        // C s_0_5: const #8s : i
        let s_0_5: i128 = 8;
        // D s_0_6: read-var len:i64
        let s_0_6: i64 = fn_state.len;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: cmp-eq s_0_7 s_0_5
        let s_0_8: bool = ((s_0_7) == (s_0_5));
        // N s_0_9: branch s_0_8 b9 b1
        if s_0_8 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #16s : i
        let s_1_0: i128 = 16;
        // D s_1_1: read-var len:i64
        let s_1_1: i64 = fn_state.len;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b8 b2
        if s_1_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_2_0: const #32s : i
        let s_2_0: i128 = 32;
        // D s_2_1: read-var len:i64
        let s_2_1: i64 = fn_state.len;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b7 b3
        if s_2_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // D s_3_1: read-var len:i64
        let s_3_1: i64 = fn_state.len;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b6 b4
        if s_3_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var xs:bv
        let s_4_1: Bits = fn_state.xs;
        // C s_4_2: const #1s : i64
        let s_4_2: i64 = 1;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // C s_4_4: const #7s : i
        let s_4_4: i128 = 7;
        // C s_4_5: add s_4_4 s_4_3
        let s_4_5: i128 = (s_4_4 + s_4_3);
        // D s_4_6: bit-extract s_4_1 s_4_0 s_4_5
        let s_4_6: Bits = (Bits::new(
            ((s_4_1) >> (s_4_0)).value(),
            u16::try_from(s_4_5).unwrap(),
        ));
        // D s_4_7: cast reint s_4_6 -> u8
        let s_4_7: u8 = (s_4_6.value() as u8);
        // C s_4_8: const #8s : i
        let s_4_8: i128 = 8;
        // D s_4_9: read-var xs:bv
        let s_4_9: Bits = fn_state.xs;
        // C s_4_10: const #1s : i64
        let s_4_10: i64 = 1;
        // C s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // C s_4_12: const #7s : i
        let s_4_12: i128 = 7;
        // C s_4_13: add s_4_12 s_4_11
        let s_4_13: i128 = (s_4_12 + s_4_11);
        // D s_4_14: bit-extract s_4_9 s_4_8 s_4_13
        let s_4_14: Bits = (Bits::new(
            ((s_4_9) >> (s_4_8)).value(),
            u16::try_from(s_4_13).unwrap(),
        ));
        // D s_4_15: cast reint s_4_14 -> u8
        let s_4_15: u8 = (s_4_14.value() as u8);
        // C s_4_16: const #16s : i
        let s_4_16: i128 = 16;
        // D s_4_17: read-var xs:bv
        let s_4_17: Bits = fn_state.xs;
        // C s_4_18: const #1s : i64
        let s_4_18: i64 = 1;
        // C s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // C s_4_20: const #7s : i
        let s_4_20: i128 = 7;
        // C s_4_21: add s_4_20 s_4_19
        let s_4_21: i128 = (s_4_20 + s_4_19);
        // D s_4_22: bit-extract s_4_17 s_4_16 s_4_21
        let s_4_22: Bits = (Bits::new(
            ((s_4_17) >> (s_4_16)).value(),
            u16::try_from(s_4_21).unwrap(),
        ));
        // D s_4_23: cast reint s_4_22 -> u8
        let s_4_23: u8 = (s_4_22.value() as u8);
        // C s_4_24: const #24s : i
        let s_4_24: i128 = 24;
        // D s_4_25: read-var xs:bv
        let s_4_25: Bits = fn_state.xs;
        // C s_4_26: const #1s : i64
        let s_4_26: i64 = 1;
        // C s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (i128::try_from(s_4_26).unwrap());
        // C s_4_28: const #7s : i
        let s_4_28: i128 = 7;
        // C s_4_29: add s_4_28 s_4_27
        let s_4_29: i128 = (s_4_28 + s_4_27);
        // D s_4_30: bit-extract s_4_25 s_4_24 s_4_29
        let s_4_30: Bits = (Bits::new(
            ((s_4_25) >> (s_4_24)).value(),
            u16::try_from(s_4_29).unwrap(),
        ));
        // D s_4_31: cast reint s_4_30 -> u8
        let s_4_31: u8 = (s_4_30.value() as u8);
        // C s_4_32: const #32s : i
        let s_4_32: i128 = 32;
        // D s_4_33: read-var xs:bv
        let s_4_33: Bits = fn_state.xs;
        // C s_4_34: const #1s : i64
        let s_4_34: i64 = 1;
        // C s_4_35: cast zx s_4_34 -> i
        let s_4_35: i128 = (i128::try_from(s_4_34).unwrap());
        // C s_4_36: const #7s : i
        let s_4_36: i128 = 7;
        // C s_4_37: add s_4_36 s_4_35
        let s_4_37: i128 = (s_4_36 + s_4_35);
        // D s_4_38: bit-extract s_4_33 s_4_32 s_4_37
        let s_4_38: Bits = (Bits::new(
            ((s_4_33) >> (s_4_32)).value(),
            u16::try_from(s_4_37).unwrap(),
        ));
        // D s_4_39: cast reint s_4_38 -> u8
        let s_4_39: u8 = (s_4_38.value() as u8);
        // C s_4_40: const #40s : i
        let s_4_40: i128 = 40;
        // D s_4_41: read-var xs:bv
        let s_4_41: Bits = fn_state.xs;
        // C s_4_42: const #1s : i64
        let s_4_42: i64 = 1;
        // C s_4_43: cast zx s_4_42 -> i
        let s_4_43: i128 = (i128::try_from(s_4_42).unwrap());
        // C s_4_44: const #7s : i
        let s_4_44: i128 = 7;
        // C s_4_45: add s_4_44 s_4_43
        let s_4_45: i128 = (s_4_44 + s_4_43);
        // D s_4_46: bit-extract s_4_41 s_4_40 s_4_45
        let s_4_46: Bits = (Bits::new(
            ((s_4_41) >> (s_4_40)).value(),
            u16::try_from(s_4_45).unwrap(),
        ));
        // D s_4_47: cast reint s_4_46 -> u8
        let s_4_47: u8 = (s_4_46.value() as u8);
        // C s_4_48: const #48s : i
        let s_4_48: i128 = 48;
        // D s_4_49: read-var xs:bv
        let s_4_49: Bits = fn_state.xs;
        // C s_4_50: const #1s : i64
        let s_4_50: i64 = 1;
        // C s_4_51: cast zx s_4_50 -> i
        let s_4_51: i128 = (i128::try_from(s_4_50).unwrap());
        // C s_4_52: const #7s : i
        let s_4_52: i128 = 7;
        // C s_4_53: add s_4_52 s_4_51
        let s_4_53: i128 = (s_4_52 + s_4_51);
        // D s_4_54: bit-extract s_4_49 s_4_48 s_4_53
        let s_4_54: Bits = (Bits::new(
            ((s_4_49) >> (s_4_48)).value(),
            u16::try_from(s_4_53).unwrap(),
        ));
        // D s_4_55: cast reint s_4_54 -> u8
        let s_4_55: u8 = (s_4_54.value() as u8);
        // C s_4_56: const #56s : i
        let s_4_56: i128 = 56;
        // D s_4_57: read-var xs:bv
        let s_4_57: Bits = fn_state.xs;
        // C s_4_58: const #1s : i64
        let s_4_58: i64 = 1;
        // C s_4_59: cast zx s_4_58 -> i
        let s_4_59: i128 = (i128::try_from(s_4_58).unwrap());
        // C s_4_60: const #7s : i
        let s_4_60: i128 = 7;
        // C s_4_61: add s_4_60 s_4_59
        let s_4_61: i128 = (s_4_60 + s_4_59);
        // D s_4_62: bit-extract s_4_57 s_4_56 s_4_61
        let s_4_62: Bits = (Bits::new(
            ((s_4_57) >> (s_4_56)).value(),
            u16::try_from(s_4_61).unwrap(),
        ));
        // D s_4_63: cast reint s_4_62 -> u8
        let s_4_63: u8 = (s_4_62.value() as u8);
        // C s_4_64: const #64s : i
        let s_4_64: i128 = 64;
        // D s_4_65: read-var xs:bv
        let s_4_65: Bits = fn_state.xs;
        // C s_4_66: const #1s : i64
        let s_4_66: i64 = 1;
        // C s_4_67: cast zx s_4_66 -> i
        let s_4_67: i128 = (i128::try_from(s_4_66).unwrap());
        // C s_4_68: const #7s : i
        let s_4_68: i128 = 7;
        // C s_4_69: add s_4_68 s_4_67
        let s_4_69: i128 = (s_4_68 + s_4_67);
        // D s_4_70: bit-extract s_4_65 s_4_64 s_4_69
        let s_4_70: Bits = (Bits::new(
            ((s_4_65) >> (s_4_64)).value(),
            u16::try_from(s_4_69).unwrap(),
        ));
        // D s_4_71: cast reint s_4_70 -> u8
        let s_4_71: u8 = (s_4_70.value() as u8);
        // C s_4_72: const #72s : i
        let s_4_72: i128 = 72;
        // D s_4_73: read-var xs:bv
        let s_4_73: Bits = fn_state.xs;
        // C s_4_74: const #1s : i64
        let s_4_74: i64 = 1;
        // C s_4_75: cast zx s_4_74 -> i
        let s_4_75: i128 = (i128::try_from(s_4_74).unwrap());
        // C s_4_76: const #7s : i
        let s_4_76: i128 = 7;
        // C s_4_77: add s_4_76 s_4_75
        let s_4_77: i128 = (s_4_76 + s_4_75);
        // D s_4_78: bit-extract s_4_73 s_4_72 s_4_77
        let s_4_78: Bits = (Bits::new(
            ((s_4_73) >> (s_4_72)).value(),
            u16::try_from(s_4_77).unwrap(),
        ));
        // D s_4_79: cast reint s_4_78 -> u8
        let s_4_79: u8 = (s_4_78.value() as u8);
        // C s_4_80: const #80s : i
        let s_4_80: i128 = 80;
        // D s_4_81: read-var xs:bv
        let s_4_81: Bits = fn_state.xs;
        // C s_4_82: const #1s : i64
        let s_4_82: i64 = 1;
        // C s_4_83: cast zx s_4_82 -> i
        let s_4_83: i128 = (i128::try_from(s_4_82).unwrap());
        // C s_4_84: const #7s : i
        let s_4_84: i128 = 7;
        // C s_4_85: add s_4_84 s_4_83
        let s_4_85: i128 = (s_4_84 + s_4_83);
        // D s_4_86: bit-extract s_4_81 s_4_80 s_4_85
        let s_4_86: Bits = (Bits::new(
            ((s_4_81) >> (s_4_80)).value(),
            u16::try_from(s_4_85).unwrap(),
        ));
        // D s_4_87: cast reint s_4_86 -> u8
        let s_4_87: u8 = (s_4_86.value() as u8);
        // C s_4_88: const #88s : i
        let s_4_88: i128 = 88;
        // D s_4_89: read-var xs:bv
        let s_4_89: Bits = fn_state.xs;
        // C s_4_90: const #1s : i64
        let s_4_90: i64 = 1;
        // C s_4_91: cast zx s_4_90 -> i
        let s_4_91: i128 = (i128::try_from(s_4_90).unwrap());
        // C s_4_92: const #7s : i
        let s_4_92: i128 = 7;
        // C s_4_93: add s_4_92 s_4_91
        let s_4_93: i128 = (s_4_92 + s_4_91);
        // D s_4_94: bit-extract s_4_89 s_4_88 s_4_93
        let s_4_94: Bits = (Bits::new(
            ((s_4_89) >> (s_4_88)).value(),
            u16::try_from(s_4_93).unwrap(),
        ));
        // D s_4_95: cast reint s_4_94 -> u8
        let s_4_95: u8 = (s_4_94.value() as u8);
        // C s_4_96: const #96s : i
        let s_4_96: i128 = 96;
        // D s_4_97: read-var xs:bv
        let s_4_97: Bits = fn_state.xs;
        // C s_4_98: const #1s : i64
        let s_4_98: i64 = 1;
        // C s_4_99: cast zx s_4_98 -> i
        let s_4_99: i128 = (i128::try_from(s_4_98).unwrap());
        // C s_4_100: const #7s : i
        let s_4_100: i128 = 7;
        // C s_4_101: add s_4_100 s_4_99
        let s_4_101: i128 = (s_4_100 + s_4_99);
        // D s_4_102: bit-extract s_4_97 s_4_96 s_4_101
        let s_4_102: Bits = (Bits::new(
            ((s_4_97) >> (s_4_96)).value(),
            u16::try_from(s_4_101).unwrap(),
        ));
        // D s_4_103: cast reint s_4_102 -> u8
        let s_4_103: u8 = (s_4_102.value() as u8);
        // C s_4_104: const #104s : i
        let s_4_104: i128 = 104;
        // D s_4_105: read-var xs:bv
        let s_4_105: Bits = fn_state.xs;
        // C s_4_106: const #1s : i64
        let s_4_106: i64 = 1;
        // C s_4_107: cast zx s_4_106 -> i
        let s_4_107: i128 = (i128::try_from(s_4_106).unwrap());
        // C s_4_108: const #7s : i
        let s_4_108: i128 = 7;
        // C s_4_109: add s_4_108 s_4_107
        let s_4_109: i128 = (s_4_108 + s_4_107);
        // D s_4_110: bit-extract s_4_105 s_4_104 s_4_109
        let s_4_110: Bits = (Bits::new(
            ((s_4_105) >> (s_4_104)).value(),
            u16::try_from(s_4_109).unwrap(),
        ));
        // D s_4_111: cast reint s_4_110 -> u8
        let s_4_111: u8 = (s_4_110.value() as u8);
        // C s_4_112: const #112s : i
        let s_4_112: i128 = 112;
        // D s_4_113: read-var xs:bv
        let s_4_113: Bits = fn_state.xs;
        // C s_4_114: const #1s : i64
        let s_4_114: i64 = 1;
        // C s_4_115: cast zx s_4_114 -> i
        let s_4_115: i128 = (i128::try_from(s_4_114).unwrap());
        // C s_4_116: const #7s : i
        let s_4_116: i128 = 7;
        // C s_4_117: add s_4_116 s_4_115
        let s_4_117: i128 = (s_4_116 + s_4_115);
        // D s_4_118: bit-extract s_4_113 s_4_112 s_4_117
        let s_4_118: Bits = (Bits::new(
            ((s_4_113) >> (s_4_112)).value(),
            u16::try_from(s_4_117).unwrap(),
        ));
        // D s_4_119: cast reint s_4_118 -> u8
        let s_4_119: u8 = (s_4_118.value() as u8);
        // C s_4_120: const #120s : i
        let s_4_120: i128 = 120;
        // D s_4_121: read-var xs:bv
        let s_4_121: Bits = fn_state.xs;
        // C s_4_122: const #1s : i64
        let s_4_122: i64 = 1;
        // C s_4_123: cast zx s_4_122 -> i
        let s_4_123: i128 = (i128::try_from(s_4_122).unwrap());
        // C s_4_124: const #7s : i
        let s_4_124: i128 = 7;
        // C s_4_125: add s_4_124 s_4_123
        let s_4_125: i128 = (s_4_124 + s_4_123);
        // D s_4_126: bit-extract s_4_121 s_4_120 s_4_125
        let s_4_126: Bits = (Bits::new(
            ((s_4_121) >> (s_4_120)).value(),
            u16::try_from(s_4_125).unwrap(),
        ));
        // D s_4_127: cast reint s_4_126 -> u8
        let s_4_127: u8 = (s_4_126.value() as u8);
        // D s_4_128: cast zx s_4_119 -> bv
        let s_4_128: Bits = Bits::new(s_4_119 as u128, 8u16);
        // D s_4_129: cast zx s_4_127 -> bv
        let s_4_129: Bits = Bits::new(s_4_127 as u128, 8u16);
        // D s_4_130: cast reint s_4_128 -> u128
        let s_4_130: u128 = (s_4_128.value() as u128);
        // D s_4_131: size-of s_4_128
        let s_4_131: u16 = s_4_128.length();
        // D s_4_132: cast reint s_4_129 -> u128
        let s_4_132: u128 = (s_4_129.value() as u128);
        // D s_4_133: size-of s_4_129
        let s_4_133: u16 = s_4_129.length();
        // D s_4_134: lsl s_4_130 s_4_133
        let s_4_134: u128 = s_4_130 << s_4_133;
        // D s_4_135: or s_4_134 s_4_132
        let s_4_135: u128 = ((s_4_134) | (s_4_132));
        // D s_4_136: add s_4_131 s_4_133
        let s_4_136: u16 = (s_4_131 + s_4_133);
        // D s_4_137: create-bits s_4_135 s_4_136
        let s_4_137: Bits = Bits::new(s_4_135, s_4_136);
        // D s_4_138: cast reint s_4_137 -> u16
        let s_4_138: u16 = (s_4_137.value() as u16);
        // D s_4_139: cast zx s_4_111 -> bv
        let s_4_139: Bits = Bits::new(s_4_111 as u128, 8u16);
        // D s_4_140: cast zx s_4_138 -> bv
        let s_4_140: Bits = Bits::new(s_4_138 as u128, 16u16);
        // D s_4_141: cast reint s_4_139 -> u128
        let s_4_141: u128 = (s_4_139.value() as u128);
        // D s_4_142: size-of s_4_139
        let s_4_142: u16 = s_4_139.length();
        // D s_4_143: cast reint s_4_140 -> u128
        let s_4_143: u128 = (s_4_140.value() as u128);
        // D s_4_144: size-of s_4_140
        let s_4_144: u16 = s_4_140.length();
        // D s_4_145: lsl s_4_141 s_4_144
        let s_4_145: u128 = s_4_141 << s_4_144;
        // D s_4_146: or s_4_145 s_4_143
        let s_4_146: u128 = ((s_4_145) | (s_4_143));
        // D s_4_147: add s_4_142 s_4_144
        let s_4_147: u16 = (s_4_142 + s_4_144);
        // D s_4_148: create-bits s_4_146 s_4_147
        let s_4_148: Bits = Bits::new(s_4_146, s_4_147);
        // D s_4_149: cast reint s_4_148 -> u24
        let s_4_149: u32 = (s_4_148.value() as u32);
        // D s_4_150: cast zx s_4_103 -> bv
        let s_4_150: Bits = Bits::new(s_4_103 as u128, 8u16);
        // D s_4_151: cast zx s_4_149 -> bv
        let s_4_151: Bits = Bits::new(s_4_149 as u128, 24u16);
        // D s_4_152: cast reint s_4_150 -> u128
        let s_4_152: u128 = (s_4_150.value() as u128);
        // D s_4_153: size-of s_4_150
        let s_4_153: u16 = s_4_150.length();
        // D s_4_154: cast reint s_4_151 -> u128
        let s_4_154: u128 = (s_4_151.value() as u128);
        // D s_4_155: size-of s_4_151
        let s_4_155: u16 = s_4_151.length();
        // D s_4_156: lsl s_4_152 s_4_155
        let s_4_156: u128 = s_4_152 << s_4_155;
        // D s_4_157: or s_4_156 s_4_154
        let s_4_157: u128 = ((s_4_156) | (s_4_154));
        // D s_4_158: add s_4_153 s_4_155
        let s_4_158: u16 = (s_4_153 + s_4_155);
        // D s_4_159: create-bits s_4_157 s_4_158
        let s_4_159: Bits = Bits::new(s_4_157, s_4_158);
        // D s_4_160: cast reint s_4_159 -> u32
        let s_4_160: u32 = (s_4_159.value() as u32);
        // D s_4_161: cast zx s_4_95 -> bv
        let s_4_161: Bits = Bits::new(s_4_95 as u128, 8u16);
        // D s_4_162: cast zx s_4_160 -> bv
        let s_4_162: Bits = Bits::new(s_4_160 as u128, 32u16);
        // D s_4_163: cast reint s_4_161 -> u128
        let s_4_163: u128 = (s_4_161.value() as u128);
        // D s_4_164: size-of s_4_161
        let s_4_164: u16 = s_4_161.length();
        // D s_4_165: cast reint s_4_162 -> u128
        let s_4_165: u128 = (s_4_162.value() as u128);
        // D s_4_166: size-of s_4_162
        let s_4_166: u16 = s_4_162.length();
        // D s_4_167: lsl s_4_163 s_4_166
        let s_4_167: u128 = s_4_163 << s_4_166;
        // D s_4_168: or s_4_167 s_4_165
        let s_4_168: u128 = ((s_4_167) | (s_4_165));
        // D s_4_169: add s_4_164 s_4_166
        let s_4_169: u16 = (s_4_164 + s_4_166);
        // D s_4_170: create-bits s_4_168 s_4_169
        let s_4_170: Bits = Bits::new(s_4_168, s_4_169);
        // D s_4_171: cast reint s_4_170 -> u40
        let s_4_171: u64 = (s_4_170.value() as u64);
        // D s_4_172: cast zx s_4_87 -> bv
        let s_4_172: Bits = Bits::new(s_4_87 as u128, 8u16);
        // D s_4_173: cast zx s_4_171 -> bv
        let s_4_173: Bits = Bits::new(s_4_171 as u128, 40u16);
        // D s_4_174: cast reint s_4_172 -> u128
        let s_4_174: u128 = (s_4_172.value() as u128);
        // D s_4_175: size-of s_4_172
        let s_4_175: u16 = s_4_172.length();
        // D s_4_176: cast reint s_4_173 -> u128
        let s_4_176: u128 = (s_4_173.value() as u128);
        // D s_4_177: size-of s_4_173
        let s_4_177: u16 = s_4_173.length();
        // D s_4_178: lsl s_4_174 s_4_177
        let s_4_178: u128 = s_4_174 << s_4_177;
        // D s_4_179: or s_4_178 s_4_176
        let s_4_179: u128 = ((s_4_178) | (s_4_176));
        // D s_4_180: add s_4_175 s_4_177
        let s_4_180: u16 = (s_4_175 + s_4_177);
        // D s_4_181: create-bits s_4_179 s_4_180
        let s_4_181: Bits = Bits::new(s_4_179, s_4_180);
        // D s_4_182: cast reint s_4_181 -> u48
        let s_4_182: u64 = (s_4_181.value() as u64);
        // D s_4_183: cast zx s_4_79 -> bv
        let s_4_183: Bits = Bits::new(s_4_79 as u128, 8u16);
        // D s_4_184: cast zx s_4_182 -> bv
        let s_4_184: Bits = Bits::new(s_4_182 as u128, 48u16);
        // D s_4_185: cast reint s_4_183 -> u128
        let s_4_185: u128 = (s_4_183.value() as u128);
        // D s_4_186: size-of s_4_183
        let s_4_186: u16 = s_4_183.length();
        // D s_4_187: cast reint s_4_184 -> u128
        let s_4_187: u128 = (s_4_184.value() as u128);
        // D s_4_188: size-of s_4_184
        let s_4_188: u16 = s_4_184.length();
        // D s_4_189: lsl s_4_185 s_4_188
        let s_4_189: u128 = s_4_185 << s_4_188;
        // D s_4_190: or s_4_189 s_4_187
        let s_4_190: u128 = ((s_4_189) | (s_4_187));
        // D s_4_191: add s_4_186 s_4_188
        let s_4_191: u16 = (s_4_186 + s_4_188);
        // D s_4_192: create-bits s_4_190 s_4_191
        let s_4_192: Bits = Bits::new(s_4_190, s_4_191);
        // D s_4_193: cast reint s_4_192 -> u56
        let s_4_193: u64 = (s_4_192.value() as u64);
        // D s_4_194: cast zx s_4_71 -> bv
        let s_4_194: Bits = Bits::new(s_4_71 as u128, 8u16);
        // D s_4_195: cast zx s_4_193 -> bv
        let s_4_195: Bits = Bits::new(s_4_193 as u128, 56u16);
        // D s_4_196: cast reint s_4_194 -> u128
        let s_4_196: u128 = (s_4_194.value() as u128);
        // D s_4_197: size-of s_4_194
        let s_4_197: u16 = s_4_194.length();
        // D s_4_198: cast reint s_4_195 -> u128
        let s_4_198: u128 = (s_4_195.value() as u128);
        // D s_4_199: size-of s_4_195
        let s_4_199: u16 = s_4_195.length();
        // D s_4_200: lsl s_4_196 s_4_199
        let s_4_200: u128 = s_4_196 << s_4_199;
        // D s_4_201: or s_4_200 s_4_198
        let s_4_201: u128 = ((s_4_200) | (s_4_198));
        // D s_4_202: add s_4_197 s_4_199
        let s_4_202: u16 = (s_4_197 + s_4_199);
        // D s_4_203: create-bits s_4_201 s_4_202
        let s_4_203: Bits = Bits::new(s_4_201, s_4_202);
        // D s_4_204: cast reint s_4_203 -> u64
        let s_4_204: u64 = (s_4_203.value() as u64);
        // D s_4_205: cast zx s_4_63 -> bv
        let s_4_205: Bits = Bits::new(s_4_63 as u128, 8u16);
        // D s_4_206: cast zx s_4_204 -> bv
        let s_4_206: Bits = Bits::new(s_4_204 as u128, 64u16);
        // D s_4_207: cast reint s_4_205 -> u128
        let s_4_207: u128 = (s_4_205.value() as u128);
        // D s_4_208: size-of s_4_205
        let s_4_208: u16 = s_4_205.length();
        // D s_4_209: cast reint s_4_206 -> u128
        let s_4_209: u128 = (s_4_206.value() as u128);
        // D s_4_210: size-of s_4_206
        let s_4_210: u16 = s_4_206.length();
        // D s_4_211: lsl s_4_207 s_4_210
        let s_4_211: u128 = s_4_207 << s_4_210;
        // D s_4_212: or s_4_211 s_4_209
        let s_4_212: u128 = ((s_4_211) | (s_4_209));
        // D s_4_213: add s_4_208 s_4_210
        let s_4_213: u16 = (s_4_208 + s_4_210);
        // D s_4_214: create-bits s_4_212 s_4_213
        let s_4_214: Bits = Bits::new(s_4_212, s_4_213);
        // D s_4_215: cast reint s_4_214 -> u72
        let s_4_215: u128 = (s_4_214.value() as u128);
        // D s_4_216: cast zx s_4_55 -> bv
        let s_4_216: Bits = Bits::new(s_4_55 as u128, 8u16);
        // D s_4_217: cast zx s_4_215 -> bv
        let s_4_217: Bits = Bits::new(s_4_215 as u128, 72u16);
        // D s_4_218: cast reint s_4_216 -> u128
        let s_4_218: u128 = (s_4_216.value() as u128);
        // D s_4_219: size-of s_4_216
        let s_4_219: u16 = s_4_216.length();
        // D s_4_220: cast reint s_4_217 -> u128
        let s_4_220: u128 = (s_4_217.value() as u128);
        // D s_4_221: size-of s_4_217
        let s_4_221: u16 = s_4_217.length();
        // D s_4_222: lsl s_4_218 s_4_221
        let s_4_222: u128 = s_4_218 << s_4_221;
        // D s_4_223: or s_4_222 s_4_220
        let s_4_223: u128 = ((s_4_222) | (s_4_220));
        // D s_4_224: add s_4_219 s_4_221
        let s_4_224: u16 = (s_4_219 + s_4_221);
        // D s_4_225: create-bits s_4_223 s_4_224
        let s_4_225: Bits = Bits::new(s_4_223, s_4_224);
        // D s_4_226: cast reint s_4_225 -> u80
        let s_4_226: u128 = (s_4_225.value() as u128);
        // D s_4_227: cast zx s_4_47 -> bv
        let s_4_227: Bits = Bits::new(s_4_47 as u128, 8u16);
        // D s_4_228: cast zx s_4_226 -> bv
        let s_4_228: Bits = Bits::new(s_4_226 as u128, 80u16);
        // D s_4_229: cast reint s_4_227 -> u128
        let s_4_229: u128 = (s_4_227.value() as u128);
        // D s_4_230: size-of s_4_227
        let s_4_230: u16 = s_4_227.length();
        // D s_4_231: cast reint s_4_228 -> u128
        let s_4_231: u128 = (s_4_228.value() as u128);
        // D s_4_232: size-of s_4_228
        let s_4_232: u16 = s_4_228.length();
        // D s_4_233: lsl s_4_229 s_4_232
        let s_4_233: u128 = s_4_229 << s_4_232;
        // D s_4_234: or s_4_233 s_4_231
        let s_4_234: u128 = ((s_4_233) | (s_4_231));
        // D s_4_235: add s_4_230 s_4_232
        let s_4_235: u16 = (s_4_230 + s_4_232);
        // D s_4_236: create-bits s_4_234 s_4_235
        let s_4_236: Bits = Bits::new(s_4_234, s_4_235);
        // D s_4_237: cast reint s_4_236 -> u88
        let s_4_237: u128 = (s_4_236.value() as u128);
        // D s_4_238: cast zx s_4_39 -> bv
        let s_4_238: Bits = Bits::new(s_4_39 as u128, 8u16);
        // D s_4_239: cast zx s_4_237 -> bv
        let s_4_239: Bits = Bits::new(s_4_237 as u128, 88u16);
        // D s_4_240: cast reint s_4_238 -> u128
        let s_4_240: u128 = (s_4_238.value() as u128);
        // D s_4_241: size-of s_4_238
        let s_4_241: u16 = s_4_238.length();
        // D s_4_242: cast reint s_4_239 -> u128
        let s_4_242: u128 = (s_4_239.value() as u128);
        // D s_4_243: size-of s_4_239
        let s_4_243: u16 = s_4_239.length();
        // D s_4_244: lsl s_4_240 s_4_243
        let s_4_244: u128 = s_4_240 << s_4_243;
        // D s_4_245: or s_4_244 s_4_242
        let s_4_245: u128 = ((s_4_244) | (s_4_242));
        // D s_4_246: add s_4_241 s_4_243
        let s_4_246: u16 = (s_4_241 + s_4_243);
        // D s_4_247: create-bits s_4_245 s_4_246
        let s_4_247: Bits = Bits::new(s_4_245, s_4_246);
        // D s_4_248: cast reint s_4_247 -> u96
        let s_4_248: u128 = (s_4_247.value() as u128);
        // D s_4_249: cast zx s_4_31 -> bv
        let s_4_249: Bits = Bits::new(s_4_31 as u128, 8u16);
        // D s_4_250: cast zx s_4_248 -> bv
        let s_4_250: Bits = Bits::new(s_4_248 as u128, 96u16);
        // D s_4_251: cast reint s_4_249 -> u128
        let s_4_251: u128 = (s_4_249.value() as u128);
        // D s_4_252: size-of s_4_249
        let s_4_252: u16 = s_4_249.length();
        // D s_4_253: cast reint s_4_250 -> u128
        let s_4_253: u128 = (s_4_250.value() as u128);
        // D s_4_254: size-of s_4_250
        let s_4_254: u16 = s_4_250.length();
        // D s_4_255: lsl s_4_251 s_4_254
        let s_4_255: u128 = s_4_251 << s_4_254;
        // D s_4_256: or s_4_255 s_4_253
        let s_4_256: u128 = ((s_4_255) | (s_4_253));
        // D s_4_257: add s_4_252 s_4_254
        let s_4_257: u16 = (s_4_252 + s_4_254);
        // D s_4_258: create-bits s_4_256 s_4_257
        let s_4_258: Bits = Bits::new(s_4_256, s_4_257);
        // D s_4_259: cast reint s_4_258 -> u104
        let s_4_259: u128 = (s_4_258.value() as u128);
        // D s_4_260: cast zx s_4_23 -> bv
        let s_4_260: Bits = Bits::new(s_4_23 as u128, 8u16);
        // D s_4_261: cast zx s_4_259 -> bv
        let s_4_261: Bits = Bits::new(s_4_259 as u128, 104u16);
        // D s_4_262: cast reint s_4_260 -> u128
        let s_4_262: u128 = (s_4_260.value() as u128);
        // D s_4_263: size-of s_4_260
        let s_4_263: u16 = s_4_260.length();
        // D s_4_264: cast reint s_4_261 -> u128
        let s_4_264: u128 = (s_4_261.value() as u128);
        // D s_4_265: size-of s_4_261
        let s_4_265: u16 = s_4_261.length();
        // D s_4_266: lsl s_4_262 s_4_265
        let s_4_266: u128 = s_4_262 << s_4_265;
        // D s_4_267: or s_4_266 s_4_264
        let s_4_267: u128 = ((s_4_266) | (s_4_264));
        // D s_4_268: add s_4_263 s_4_265
        let s_4_268: u16 = (s_4_263 + s_4_265);
        // D s_4_269: create-bits s_4_267 s_4_268
        let s_4_269: Bits = Bits::new(s_4_267, s_4_268);
        // D s_4_270: cast reint s_4_269 -> u112
        let s_4_270: u128 = (s_4_269.value() as u128);
        // D s_4_271: cast zx s_4_15 -> bv
        let s_4_271: Bits = Bits::new(s_4_15 as u128, 8u16);
        // D s_4_272: cast zx s_4_270 -> bv
        let s_4_272: Bits = Bits::new(s_4_270 as u128, 112u16);
        // D s_4_273: cast reint s_4_271 -> u128
        let s_4_273: u128 = (s_4_271.value() as u128);
        // D s_4_274: size-of s_4_271
        let s_4_274: u16 = s_4_271.length();
        // D s_4_275: cast reint s_4_272 -> u128
        let s_4_275: u128 = (s_4_272.value() as u128);
        // D s_4_276: size-of s_4_272
        let s_4_276: u16 = s_4_272.length();
        // D s_4_277: lsl s_4_273 s_4_276
        let s_4_277: u128 = s_4_273 << s_4_276;
        // D s_4_278: or s_4_277 s_4_275
        let s_4_278: u128 = ((s_4_277) | (s_4_275));
        // D s_4_279: add s_4_274 s_4_276
        let s_4_279: u16 = (s_4_274 + s_4_276);
        // D s_4_280: create-bits s_4_278 s_4_279
        let s_4_280: Bits = Bits::new(s_4_278, s_4_279);
        // D s_4_281: cast reint s_4_280 -> u120
        let s_4_281: u128 = (s_4_280.value() as u128);
        // D s_4_282: cast zx s_4_7 -> bv
        let s_4_282: Bits = Bits::new(s_4_7 as u128, 8u16);
        // D s_4_283: cast zx s_4_281 -> bv
        let s_4_283: Bits = Bits::new(s_4_281 as u128, 120u16);
        // D s_4_284: cast reint s_4_282 -> u128
        let s_4_284: u128 = (s_4_282.value() as u128);
        // D s_4_285: size-of s_4_282
        let s_4_285: u16 = s_4_282.length();
        // D s_4_286: cast reint s_4_283 -> u128
        let s_4_286: u128 = (s_4_283.value() as u128);
        // D s_4_287: size-of s_4_283
        let s_4_287: u16 = s_4_283.length();
        // D s_4_288: lsl s_4_284 s_4_287
        let s_4_288: u128 = s_4_284 << s_4_287;
        // D s_4_289: or s_4_288 s_4_286
        let s_4_289: u128 = ((s_4_288) | (s_4_286));
        // D s_4_290: add s_4_285 s_4_287
        let s_4_290: u16 = (s_4_285 + s_4_287);
        // D s_4_291: create-bits s_4_289 s_4_290
        let s_4_291: Bits = Bits::new(s_4_289, s_4_290);
        // D s_4_292: write-var return_value <= s_4_291
        fn_state.return_value = s_4_291;
        // N s_4_293: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var return_value:bv
        let s_5_0: Bits = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var xs:bv
        let s_6_1: Bits = fn_state.xs;
        // C s_6_2: const #1s : i64
        let s_6_2: i64 = 1;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_4: const #7s : i
        let s_6_4: i128 = 7;
        // C s_6_5: add s_6_4 s_6_3
        let s_6_5: i128 = (s_6_4 + s_6_3);
        // D s_6_6: bit-extract s_6_1 s_6_0 s_6_5
        let s_6_6: Bits = (Bits::new(
            ((s_6_1) >> (s_6_0)).value(),
            u16::try_from(s_6_5).unwrap(),
        ));
        // D s_6_7: cast reint s_6_6 -> u8
        let s_6_7: u8 = (s_6_6.value() as u8);
        // C s_6_8: const #8s : i
        let s_6_8: i128 = 8;
        // D s_6_9: read-var xs:bv
        let s_6_9: Bits = fn_state.xs;
        // C s_6_10: const #1s : i64
        let s_6_10: i64 = 1;
        // C s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // C s_6_12: const #7s : i
        let s_6_12: i128 = 7;
        // C s_6_13: add s_6_12 s_6_11
        let s_6_13: i128 = (s_6_12 + s_6_11);
        // D s_6_14: bit-extract s_6_9 s_6_8 s_6_13
        let s_6_14: Bits = (Bits::new(
            ((s_6_9) >> (s_6_8)).value(),
            u16::try_from(s_6_13).unwrap(),
        ));
        // D s_6_15: cast reint s_6_14 -> u8
        let s_6_15: u8 = (s_6_14.value() as u8);
        // C s_6_16: const #16s : i
        let s_6_16: i128 = 16;
        // D s_6_17: read-var xs:bv
        let s_6_17: Bits = fn_state.xs;
        // C s_6_18: const #1s : i64
        let s_6_18: i64 = 1;
        // C s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // C s_6_20: const #7s : i
        let s_6_20: i128 = 7;
        // C s_6_21: add s_6_20 s_6_19
        let s_6_21: i128 = (s_6_20 + s_6_19);
        // D s_6_22: bit-extract s_6_17 s_6_16 s_6_21
        let s_6_22: Bits = (Bits::new(
            ((s_6_17) >> (s_6_16)).value(),
            u16::try_from(s_6_21).unwrap(),
        ));
        // D s_6_23: cast reint s_6_22 -> u8
        let s_6_23: u8 = (s_6_22.value() as u8);
        // C s_6_24: const #24s : i
        let s_6_24: i128 = 24;
        // D s_6_25: read-var xs:bv
        let s_6_25: Bits = fn_state.xs;
        // C s_6_26: const #1s : i64
        let s_6_26: i64 = 1;
        // C s_6_27: cast zx s_6_26 -> i
        let s_6_27: i128 = (i128::try_from(s_6_26).unwrap());
        // C s_6_28: const #7s : i
        let s_6_28: i128 = 7;
        // C s_6_29: add s_6_28 s_6_27
        let s_6_29: i128 = (s_6_28 + s_6_27);
        // D s_6_30: bit-extract s_6_25 s_6_24 s_6_29
        let s_6_30: Bits = (Bits::new(
            ((s_6_25) >> (s_6_24)).value(),
            u16::try_from(s_6_29).unwrap(),
        ));
        // D s_6_31: cast reint s_6_30 -> u8
        let s_6_31: u8 = (s_6_30.value() as u8);
        // C s_6_32: const #32s : i
        let s_6_32: i128 = 32;
        // D s_6_33: read-var xs:bv
        let s_6_33: Bits = fn_state.xs;
        // C s_6_34: const #1s : i64
        let s_6_34: i64 = 1;
        // C s_6_35: cast zx s_6_34 -> i
        let s_6_35: i128 = (i128::try_from(s_6_34).unwrap());
        // C s_6_36: const #7s : i
        let s_6_36: i128 = 7;
        // C s_6_37: add s_6_36 s_6_35
        let s_6_37: i128 = (s_6_36 + s_6_35);
        // D s_6_38: bit-extract s_6_33 s_6_32 s_6_37
        let s_6_38: Bits = (Bits::new(
            ((s_6_33) >> (s_6_32)).value(),
            u16::try_from(s_6_37).unwrap(),
        ));
        // D s_6_39: cast reint s_6_38 -> u8
        let s_6_39: u8 = (s_6_38.value() as u8);
        // C s_6_40: const #40s : i
        let s_6_40: i128 = 40;
        // D s_6_41: read-var xs:bv
        let s_6_41: Bits = fn_state.xs;
        // C s_6_42: const #1s : i64
        let s_6_42: i64 = 1;
        // C s_6_43: cast zx s_6_42 -> i
        let s_6_43: i128 = (i128::try_from(s_6_42).unwrap());
        // C s_6_44: const #7s : i
        let s_6_44: i128 = 7;
        // C s_6_45: add s_6_44 s_6_43
        let s_6_45: i128 = (s_6_44 + s_6_43);
        // D s_6_46: bit-extract s_6_41 s_6_40 s_6_45
        let s_6_46: Bits = (Bits::new(
            ((s_6_41) >> (s_6_40)).value(),
            u16::try_from(s_6_45).unwrap(),
        ));
        // D s_6_47: cast reint s_6_46 -> u8
        let s_6_47: u8 = (s_6_46.value() as u8);
        // C s_6_48: const #48s : i
        let s_6_48: i128 = 48;
        // D s_6_49: read-var xs:bv
        let s_6_49: Bits = fn_state.xs;
        // C s_6_50: const #1s : i64
        let s_6_50: i64 = 1;
        // C s_6_51: cast zx s_6_50 -> i
        let s_6_51: i128 = (i128::try_from(s_6_50).unwrap());
        // C s_6_52: const #7s : i
        let s_6_52: i128 = 7;
        // C s_6_53: add s_6_52 s_6_51
        let s_6_53: i128 = (s_6_52 + s_6_51);
        // D s_6_54: bit-extract s_6_49 s_6_48 s_6_53
        let s_6_54: Bits = (Bits::new(
            ((s_6_49) >> (s_6_48)).value(),
            u16::try_from(s_6_53).unwrap(),
        ));
        // D s_6_55: cast reint s_6_54 -> u8
        let s_6_55: u8 = (s_6_54.value() as u8);
        // C s_6_56: const #56s : i
        let s_6_56: i128 = 56;
        // D s_6_57: read-var xs:bv
        let s_6_57: Bits = fn_state.xs;
        // C s_6_58: const #1s : i64
        let s_6_58: i64 = 1;
        // C s_6_59: cast zx s_6_58 -> i
        let s_6_59: i128 = (i128::try_from(s_6_58).unwrap());
        // C s_6_60: const #7s : i
        let s_6_60: i128 = 7;
        // C s_6_61: add s_6_60 s_6_59
        let s_6_61: i128 = (s_6_60 + s_6_59);
        // D s_6_62: bit-extract s_6_57 s_6_56 s_6_61
        let s_6_62: Bits = (Bits::new(
            ((s_6_57) >> (s_6_56)).value(),
            u16::try_from(s_6_61).unwrap(),
        ));
        // D s_6_63: cast reint s_6_62 -> u8
        let s_6_63: u8 = (s_6_62.value() as u8);
        // D s_6_64: cast zx s_6_55 -> bv
        let s_6_64: Bits = Bits::new(s_6_55 as u128, 8u16);
        // D s_6_65: cast zx s_6_63 -> bv
        let s_6_65: Bits = Bits::new(s_6_63 as u128, 8u16);
        // D s_6_66: cast reint s_6_64 -> u128
        let s_6_66: u128 = (s_6_64.value() as u128);
        // D s_6_67: size-of s_6_64
        let s_6_67: u16 = s_6_64.length();
        // D s_6_68: cast reint s_6_65 -> u128
        let s_6_68: u128 = (s_6_65.value() as u128);
        // D s_6_69: size-of s_6_65
        let s_6_69: u16 = s_6_65.length();
        // D s_6_70: lsl s_6_66 s_6_69
        let s_6_70: u128 = s_6_66 << s_6_69;
        // D s_6_71: or s_6_70 s_6_68
        let s_6_71: u128 = ((s_6_70) | (s_6_68));
        // D s_6_72: add s_6_67 s_6_69
        let s_6_72: u16 = (s_6_67 + s_6_69);
        // D s_6_73: create-bits s_6_71 s_6_72
        let s_6_73: Bits = Bits::new(s_6_71, s_6_72);
        // D s_6_74: cast reint s_6_73 -> u16
        let s_6_74: u16 = (s_6_73.value() as u16);
        // D s_6_75: cast zx s_6_47 -> bv
        let s_6_75: Bits = Bits::new(s_6_47 as u128, 8u16);
        // D s_6_76: cast zx s_6_74 -> bv
        let s_6_76: Bits = Bits::new(s_6_74 as u128, 16u16);
        // D s_6_77: cast reint s_6_75 -> u128
        let s_6_77: u128 = (s_6_75.value() as u128);
        // D s_6_78: size-of s_6_75
        let s_6_78: u16 = s_6_75.length();
        // D s_6_79: cast reint s_6_76 -> u128
        let s_6_79: u128 = (s_6_76.value() as u128);
        // D s_6_80: size-of s_6_76
        let s_6_80: u16 = s_6_76.length();
        // D s_6_81: lsl s_6_77 s_6_80
        let s_6_81: u128 = s_6_77 << s_6_80;
        // D s_6_82: or s_6_81 s_6_79
        let s_6_82: u128 = ((s_6_81) | (s_6_79));
        // D s_6_83: add s_6_78 s_6_80
        let s_6_83: u16 = (s_6_78 + s_6_80);
        // D s_6_84: create-bits s_6_82 s_6_83
        let s_6_84: Bits = Bits::new(s_6_82, s_6_83);
        // D s_6_85: cast reint s_6_84 -> u24
        let s_6_85: u32 = (s_6_84.value() as u32);
        // D s_6_86: cast zx s_6_39 -> bv
        let s_6_86: Bits = Bits::new(s_6_39 as u128, 8u16);
        // D s_6_87: cast zx s_6_85 -> bv
        let s_6_87: Bits = Bits::new(s_6_85 as u128, 24u16);
        // D s_6_88: cast reint s_6_86 -> u128
        let s_6_88: u128 = (s_6_86.value() as u128);
        // D s_6_89: size-of s_6_86
        let s_6_89: u16 = s_6_86.length();
        // D s_6_90: cast reint s_6_87 -> u128
        let s_6_90: u128 = (s_6_87.value() as u128);
        // D s_6_91: size-of s_6_87
        let s_6_91: u16 = s_6_87.length();
        // D s_6_92: lsl s_6_88 s_6_91
        let s_6_92: u128 = s_6_88 << s_6_91;
        // D s_6_93: or s_6_92 s_6_90
        let s_6_93: u128 = ((s_6_92) | (s_6_90));
        // D s_6_94: add s_6_89 s_6_91
        let s_6_94: u16 = (s_6_89 + s_6_91);
        // D s_6_95: create-bits s_6_93 s_6_94
        let s_6_95: Bits = Bits::new(s_6_93, s_6_94);
        // D s_6_96: cast reint s_6_95 -> u32
        let s_6_96: u32 = (s_6_95.value() as u32);
        // D s_6_97: cast zx s_6_31 -> bv
        let s_6_97: Bits = Bits::new(s_6_31 as u128, 8u16);
        // D s_6_98: cast zx s_6_96 -> bv
        let s_6_98: Bits = Bits::new(s_6_96 as u128, 32u16);
        // D s_6_99: cast reint s_6_97 -> u128
        let s_6_99: u128 = (s_6_97.value() as u128);
        // D s_6_100: size-of s_6_97
        let s_6_100: u16 = s_6_97.length();
        // D s_6_101: cast reint s_6_98 -> u128
        let s_6_101: u128 = (s_6_98.value() as u128);
        // D s_6_102: size-of s_6_98
        let s_6_102: u16 = s_6_98.length();
        // D s_6_103: lsl s_6_99 s_6_102
        let s_6_103: u128 = s_6_99 << s_6_102;
        // D s_6_104: or s_6_103 s_6_101
        let s_6_104: u128 = ((s_6_103) | (s_6_101));
        // D s_6_105: add s_6_100 s_6_102
        let s_6_105: u16 = (s_6_100 + s_6_102);
        // D s_6_106: create-bits s_6_104 s_6_105
        let s_6_106: Bits = Bits::new(s_6_104, s_6_105);
        // D s_6_107: cast reint s_6_106 -> u40
        let s_6_107: u64 = (s_6_106.value() as u64);
        // D s_6_108: cast zx s_6_23 -> bv
        let s_6_108: Bits = Bits::new(s_6_23 as u128, 8u16);
        // D s_6_109: cast zx s_6_107 -> bv
        let s_6_109: Bits = Bits::new(s_6_107 as u128, 40u16);
        // D s_6_110: cast reint s_6_108 -> u128
        let s_6_110: u128 = (s_6_108.value() as u128);
        // D s_6_111: size-of s_6_108
        let s_6_111: u16 = s_6_108.length();
        // D s_6_112: cast reint s_6_109 -> u128
        let s_6_112: u128 = (s_6_109.value() as u128);
        // D s_6_113: size-of s_6_109
        let s_6_113: u16 = s_6_109.length();
        // D s_6_114: lsl s_6_110 s_6_113
        let s_6_114: u128 = s_6_110 << s_6_113;
        // D s_6_115: or s_6_114 s_6_112
        let s_6_115: u128 = ((s_6_114) | (s_6_112));
        // D s_6_116: add s_6_111 s_6_113
        let s_6_116: u16 = (s_6_111 + s_6_113);
        // D s_6_117: create-bits s_6_115 s_6_116
        let s_6_117: Bits = Bits::new(s_6_115, s_6_116);
        // D s_6_118: cast reint s_6_117 -> u48
        let s_6_118: u64 = (s_6_117.value() as u64);
        // D s_6_119: cast zx s_6_15 -> bv
        let s_6_119: Bits = Bits::new(s_6_15 as u128, 8u16);
        // D s_6_120: cast zx s_6_118 -> bv
        let s_6_120: Bits = Bits::new(s_6_118 as u128, 48u16);
        // D s_6_121: cast reint s_6_119 -> u128
        let s_6_121: u128 = (s_6_119.value() as u128);
        // D s_6_122: size-of s_6_119
        let s_6_122: u16 = s_6_119.length();
        // D s_6_123: cast reint s_6_120 -> u128
        let s_6_123: u128 = (s_6_120.value() as u128);
        // D s_6_124: size-of s_6_120
        let s_6_124: u16 = s_6_120.length();
        // D s_6_125: lsl s_6_121 s_6_124
        let s_6_125: u128 = s_6_121 << s_6_124;
        // D s_6_126: or s_6_125 s_6_123
        let s_6_126: u128 = ((s_6_125) | (s_6_123));
        // D s_6_127: add s_6_122 s_6_124
        let s_6_127: u16 = (s_6_122 + s_6_124);
        // D s_6_128: create-bits s_6_126 s_6_127
        let s_6_128: Bits = Bits::new(s_6_126, s_6_127);
        // D s_6_129: cast reint s_6_128 -> u56
        let s_6_129: u64 = (s_6_128.value() as u64);
        // D s_6_130: cast zx s_6_7 -> bv
        let s_6_130: Bits = Bits::new(s_6_7 as u128, 8u16);
        // D s_6_131: cast zx s_6_129 -> bv
        let s_6_131: Bits = Bits::new(s_6_129 as u128, 56u16);
        // D s_6_132: cast reint s_6_130 -> u128
        let s_6_132: u128 = (s_6_130.value() as u128);
        // D s_6_133: size-of s_6_130
        let s_6_133: u16 = s_6_130.length();
        // D s_6_134: cast reint s_6_131 -> u128
        let s_6_134: u128 = (s_6_131.value() as u128);
        // D s_6_135: size-of s_6_131
        let s_6_135: u16 = s_6_131.length();
        // D s_6_136: lsl s_6_132 s_6_135
        let s_6_136: u128 = s_6_132 << s_6_135;
        // D s_6_137: or s_6_136 s_6_134
        let s_6_137: u128 = ((s_6_136) | (s_6_134));
        // D s_6_138: add s_6_133 s_6_135
        let s_6_138: u16 = (s_6_133 + s_6_135);
        // D s_6_139: create-bits s_6_137 s_6_138
        let s_6_139: Bits = Bits::new(s_6_137, s_6_138);
        // D s_6_140: write-var return_value <= s_6_139
        fn_state.return_value = s_6_139;
        // N s_6_141: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var xs:bv
        let s_7_1: Bits = fn_state.xs;
        // C s_7_2: const #1s : i64
        let s_7_2: i64 = 1;
        // C s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // C s_7_4: const #7s : i
        let s_7_4: i128 = 7;
        // C s_7_5: add s_7_4 s_7_3
        let s_7_5: i128 = (s_7_4 + s_7_3);
        // D s_7_6: bit-extract s_7_1 s_7_0 s_7_5
        let s_7_6: Bits = (Bits::new(
            ((s_7_1) >> (s_7_0)).value(),
            u16::try_from(s_7_5).unwrap(),
        ));
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: u8 = (s_7_6.value() as u8);
        // C s_7_8: const #8s : i
        let s_7_8: i128 = 8;
        // D s_7_9: read-var xs:bv
        let s_7_9: Bits = fn_state.xs;
        // C s_7_10: const #1s : i64
        let s_7_10: i64 = 1;
        // C s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // C s_7_12: const #7s : i
        let s_7_12: i128 = 7;
        // C s_7_13: add s_7_12 s_7_11
        let s_7_13: i128 = (s_7_12 + s_7_11);
        // D s_7_14: bit-extract s_7_9 s_7_8 s_7_13
        let s_7_14: Bits = (Bits::new(
            ((s_7_9) >> (s_7_8)).value(),
            u16::try_from(s_7_13).unwrap(),
        ));
        // D s_7_15: cast reint s_7_14 -> u8
        let s_7_15: u8 = (s_7_14.value() as u8);
        // C s_7_16: const #16s : i
        let s_7_16: i128 = 16;
        // D s_7_17: read-var xs:bv
        let s_7_17: Bits = fn_state.xs;
        // C s_7_18: const #1s : i64
        let s_7_18: i64 = 1;
        // C s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // C s_7_20: const #7s : i
        let s_7_20: i128 = 7;
        // C s_7_21: add s_7_20 s_7_19
        let s_7_21: i128 = (s_7_20 + s_7_19);
        // D s_7_22: bit-extract s_7_17 s_7_16 s_7_21
        let s_7_22: Bits = (Bits::new(
            ((s_7_17) >> (s_7_16)).value(),
            u16::try_from(s_7_21).unwrap(),
        ));
        // D s_7_23: cast reint s_7_22 -> u8
        let s_7_23: u8 = (s_7_22.value() as u8);
        // C s_7_24: const #24s : i
        let s_7_24: i128 = 24;
        // D s_7_25: read-var xs:bv
        let s_7_25: Bits = fn_state.xs;
        // C s_7_26: const #1s : i64
        let s_7_26: i64 = 1;
        // C s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // C s_7_28: const #7s : i
        let s_7_28: i128 = 7;
        // C s_7_29: add s_7_28 s_7_27
        let s_7_29: i128 = (s_7_28 + s_7_27);
        // D s_7_30: bit-extract s_7_25 s_7_24 s_7_29
        let s_7_30: Bits = (Bits::new(
            ((s_7_25) >> (s_7_24)).value(),
            u16::try_from(s_7_29).unwrap(),
        ));
        // D s_7_31: cast reint s_7_30 -> u8
        let s_7_31: u8 = (s_7_30.value() as u8);
        // D s_7_32: cast zx s_7_23 -> bv
        let s_7_32: Bits = Bits::new(s_7_23 as u128, 8u16);
        // D s_7_33: cast zx s_7_31 -> bv
        let s_7_33: Bits = Bits::new(s_7_31 as u128, 8u16);
        // D s_7_34: cast reint s_7_32 -> u128
        let s_7_34: u128 = (s_7_32.value() as u128);
        // D s_7_35: size-of s_7_32
        let s_7_35: u16 = s_7_32.length();
        // D s_7_36: cast reint s_7_33 -> u128
        let s_7_36: u128 = (s_7_33.value() as u128);
        // D s_7_37: size-of s_7_33
        let s_7_37: u16 = s_7_33.length();
        // D s_7_38: lsl s_7_34 s_7_37
        let s_7_38: u128 = s_7_34 << s_7_37;
        // D s_7_39: or s_7_38 s_7_36
        let s_7_39: u128 = ((s_7_38) | (s_7_36));
        // D s_7_40: add s_7_35 s_7_37
        let s_7_40: u16 = (s_7_35 + s_7_37);
        // D s_7_41: create-bits s_7_39 s_7_40
        let s_7_41: Bits = Bits::new(s_7_39, s_7_40);
        // D s_7_42: cast reint s_7_41 -> u16
        let s_7_42: u16 = (s_7_41.value() as u16);
        // D s_7_43: cast zx s_7_15 -> bv
        let s_7_43: Bits = Bits::new(s_7_15 as u128, 8u16);
        // D s_7_44: cast zx s_7_42 -> bv
        let s_7_44: Bits = Bits::new(s_7_42 as u128, 16u16);
        // D s_7_45: cast reint s_7_43 -> u128
        let s_7_45: u128 = (s_7_43.value() as u128);
        // D s_7_46: size-of s_7_43
        let s_7_46: u16 = s_7_43.length();
        // D s_7_47: cast reint s_7_44 -> u128
        let s_7_47: u128 = (s_7_44.value() as u128);
        // D s_7_48: size-of s_7_44
        let s_7_48: u16 = s_7_44.length();
        // D s_7_49: lsl s_7_45 s_7_48
        let s_7_49: u128 = s_7_45 << s_7_48;
        // D s_7_50: or s_7_49 s_7_47
        let s_7_50: u128 = ((s_7_49) | (s_7_47));
        // D s_7_51: add s_7_46 s_7_48
        let s_7_51: u16 = (s_7_46 + s_7_48);
        // D s_7_52: create-bits s_7_50 s_7_51
        let s_7_52: Bits = Bits::new(s_7_50, s_7_51);
        // D s_7_53: cast reint s_7_52 -> u24
        let s_7_53: u32 = (s_7_52.value() as u32);
        // D s_7_54: cast zx s_7_7 -> bv
        let s_7_54: Bits = Bits::new(s_7_7 as u128, 8u16);
        // D s_7_55: cast zx s_7_53 -> bv
        let s_7_55: Bits = Bits::new(s_7_53 as u128, 24u16);
        // D s_7_56: cast reint s_7_54 -> u128
        let s_7_56: u128 = (s_7_54.value() as u128);
        // D s_7_57: size-of s_7_54
        let s_7_57: u16 = s_7_54.length();
        // D s_7_58: cast reint s_7_55 -> u128
        let s_7_58: u128 = (s_7_55.value() as u128);
        // D s_7_59: size-of s_7_55
        let s_7_59: u16 = s_7_55.length();
        // D s_7_60: lsl s_7_56 s_7_59
        let s_7_60: u128 = s_7_56 << s_7_59;
        // D s_7_61: or s_7_60 s_7_58
        let s_7_61: u128 = ((s_7_60) | (s_7_58));
        // D s_7_62: add s_7_57 s_7_59
        let s_7_62: u16 = (s_7_57 + s_7_59);
        // D s_7_63: create-bits s_7_61 s_7_62
        let s_7_63: Bits = Bits::new(s_7_61, s_7_62);
        // D s_7_64: write-var return_value <= s_7_63
        fn_state.return_value = s_7_63;
        // N s_7_65: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var xs:bv
        let s_8_1: Bits = fn_state.xs;
        // C s_8_2: const #1s : i64
        let s_8_2: i64 = 1;
        // C s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: const #7s : i
        let s_8_4: i128 = 7;
        // C s_8_5: add s_8_4 s_8_3
        let s_8_5: i128 = (s_8_4 + s_8_3);
        // D s_8_6: bit-extract s_8_1 s_8_0 s_8_5
        let s_8_6: Bits = (Bits::new(
            ((s_8_1) >> (s_8_0)).value(),
            u16::try_from(s_8_5).unwrap(),
        ));
        // D s_8_7: cast reint s_8_6 -> u8
        let s_8_7: u8 = (s_8_6.value() as u8);
        // C s_8_8: const #8s : i
        let s_8_8: i128 = 8;
        // D s_8_9: read-var xs:bv
        let s_8_9: Bits = fn_state.xs;
        // C s_8_10: const #1s : i64
        let s_8_10: i64 = 1;
        // C s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // C s_8_12: const #7s : i
        let s_8_12: i128 = 7;
        // C s_8_13: add s_8_12 s_8_11
        let s_8_13: i128 = (s_8_12 + s_8_11);
        // D s_8_14: bit-extract s_8_9 s_8_8 s_8_13
        let s_8_14: Bits = (Bits::new(
            ((s_8_9) >> (s_8_8)).value(),
            u16::try_from(s_8_13).unwrap(),
        ));
        // D s_8_15: cast reint s_8_14 -> u8
        let s_8_15: u8 = (s_8_14.value() as u8);
        // D s_8_16: cast zx s_8_7 -> bv
        let s_8_16: Bits = Bits::new(s_8_7 as u128, 8u16);
        // D s_8_17: cast zx s_8_15 -> bv
        let s_8_17: Bits = Bits::new(s_8_15 as u128, 8u16);
        // D s_8_18: cast reint s_8_16 -> u128
        let s_8_18: u128 = (s_8_16.value() as u128);
        // D s_8_19: size-of s_8_16
        let s_8_19: u16 = s_8_16.length();
        // D s_8_20: cast reint s_8_17 -> u128
        let s_8_20: u128 = (s_8_17.value() as u128);
        // D s_8_21: size-of s_8_17
        let s_8_21: u16 = s_8_17.length();
        // D s_8_22: lsl s_8_18 s_8_21
        let s_8_22: u128 = s_8_18 << s_8_21;
        // D s_8_23: or s_8_22 s_8_20
        let s_8_23: u128 = ((s_8_22) | (s_8_20));
        // D s_8_24: add s_8_19 s_8_21
        let s_8_24: u16 = (s_8_19 + s_8_21);
        // D s_8_25: create-bits s_8_23 s_8_24
        let s_8_25: Bits = Bits::new(s_8_23, s_8_24);
        // D s_8_26: write-var return_value <= s_8_25
        fn_state.return_value = s_8_25;
        // N s_8_27: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var xs:bv
        let s_9_0: Bits = fn_state.xs;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
