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
use execute_aarch64_instrs_vector_fp16_movi::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn decode_fmov_advsimd_aarch64_instrs_vector_fp16_movi<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    h: bool,
    g: bool,
    f: bool,
    e: bool,
    d: bool,
    c: bool,
    b: bool,
    a: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        rd: i64,
        ga_255783: i64,
        Rd: u8,
        h: bool,
        g: bool,
        f: bool,
        e: bool,
        d: bool,
        c: bool,
        b: bool,
        a: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        h,
        g,
        f,
        e,
        d,
        c,
        b,
        a,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var rd <= s_0_3
        fn_state.rd = s_0_3;
        // D s_0_5: read-var Q:u8
        let s_0_5: bool = fn_state.Q;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // N s_0_10: branch s_0_9 b3 b1
        if s_0_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: write-var ga#255783 <= s_1_0
        fn_state.ga_255783 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#255783:i64
        let s_2_0: i64 = fn_state.ga_255783;
        // D s_2_1: read-var a:u8
        let s_2_1: bool = fn_state.a;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // D s_2_3: read-var b:u8
        let s_2_3: bool = fn_state.b;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 1u16);
        // D s_2_5: cast reint s_2_2 -> u128
        let s_2_5: u128 = (s_2_2.value() as u128);
        // D s_2_6: size-of s_2_2
        let s_2_6: u16 = s_2_2.length();
        // D s_2_7: cast reint s_2_4 -> u128
        let s_2_7: u128 = (s_2_4.value() as u128);
        // D s_2_8: size-of s_2_4
        let s_2_8: u16 = s_2_4.length();
        // D s_2_9: lsl s_2_5 s_2_8
        let s_2_9: u128 = s_2_5 << s_2_8;
        // D s_2_10: or s_2_9 s_2_7
        let s_2_10: u128 = ((s_2_9) | (s_2_7));
        // D s_2_11: add s_2_6 s_2_8
        let s_2_11: u16 = (s_2_6 + s_2_8);
        // D s_2_12: create-bits s_2_10 s_2_11
        let s_2_12: Bits = Bits::new(s_2_10, s_2_11);
        // D s_2_13: cast reint s_2_12 -> u8
        let s_2_13: u8 = (s_2_12.value() as u8);
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 2u16);
        // D s_2_15: read-var c:u8
        let s_2_15: bool = fn_state.c;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // D s_2_17: cast reint s_2_14 -> u128
        let s_2_17: u128 = (s_2_14.value() as u128);
        // D s_2_18: size-of s_2_14
        let s_2_18: u16 = s_2_14.length();
        // D s_2_19: cast reint s_2_16 -> u128
        let s_2_19: u128 = (s_2_16.value() as u128);
        // D s_2_20: size-of s_2_16
        let s_2_20: u16 = s_2_16.length();
        // D s_2_21: lsl s_2_17 s_2_20
        let s_2_21: u128 = s_2_17 << s_2_20;
        // D s_2_22: or s_2_21 s_2_19
        let s_2_22: u128 = ((s_2_21) | (s_2_19));
        // D s_2_23: add s_2_18 s_2_20
        let s_2_23: u16 = (s_2_18 + s_2_20);
        // D s_2_24: create-bits s_2_22 s_2_23
        let s_2_24: Bits = Bits::new(s_2_22, s_2_23);
        // D s_2_25: cast reint s_2_24 -> u8
        let s_2_25: u8 = (s_2_24.value() as u8);
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 3u16);
        // D s_2_27: read-var d:u8
        let s_2_27: bool = fn_state.d;
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 1u16);
        // D s_2_29: cast reint s_2_26 -> u128
        let s_2_29: u128 = (s_2_26.value() as u128);
        // D s_2_30: size-of s_2_26
        let s_2_30: u16 = s_2_26.length();
        // D s_2_31: cast reint s_2_28 -> u128
        let s_2_31: u128 = (s_2_28.value() as u128);
        // D s_2_32: size-of s_2_28
        let s_2_32: u16 = s_2_28.length();
        // D s_2_33: lsl s_2_29 s_2_32
        let s_2_33: u128 = s_2_29 << s_2_32;
        // D s_2_34: or s_2_33 s_2_31
        let s_2_34: u128 = ((s_2_33) | (s_2_31));
        // D s_2_35: add s_2_30 s_2_32
        let s_2_35: u16 = (s_2_30 + s_2_32);
        // D s_2_36: create-bits s_2_34 s_2_35
        let s_2_36: Bits = Bits::new(s_2_34, s_2_35);
        // D s_2_37: cast reint s_2_36 -> u8
        let s_2_37: u8 = (s_2_36.value() as u8);
        // D s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 4u16);
        // D s_2_39: read-var e:u8
        let s_2_39: bool = fn_state.e;
        // D s_2_40: cast zx s_2_39 -> bv
        let s_2_40: Bits = Bits::new(s_2_39 as u128, 1u16);
        // D s_2_41: cast reint s_2_38 -> u128
        let s_2_41: u128 = (s_2_38.value() as u128);
        // D s_2_42: size-of s_2_38
        let s_2_42: u16 = s_2_38.length();
        // D s_2_43: cast reint s_2_40 -> u128
        let s_2_43: u128 = (s_2_40.value() as u128);
        // D s_2_44: size-of s_2_40
        let s_2_44: u16 = s_2_40.length();
        // D s_2_45: lsl s_2_41 s_2_44
        let s_2_45: u128 = s_2_41 << s_2_44;
        // D s_2_46: or s_2_45 s_2_43
        let s_2_46: u128 = ((s_2_45) | (s_2_43));
        // D s_2_47: add s_2_42 s_2_44
        let s_2_47: u16 = (s_2_42 + s_2_44);
        // D s_2_48: create-bits s_2_46 s_2_47
        let s_2_48: Bits = Bits::new(s_2_46, s_2_47);
        // D s_2_49: cast reint s_2_48 -> u8
        let s_2_49: u8 = (s_2_48.value() as u8);
        // D s_2_50: cast zx s_2_49 -> bv
        let s_2_50: Bits = Bits::new(s_2_49 as u128, 5u16);
        // D s_2_51: read-var f:u8
        let s_2_51: bool = fn_state.f;
        // D s_2_52: cast zx s_2_51 -> bv
        let s_2_52: Bits = Bits::new(s_2_51 as u128, 1u16);
        // D s_2_53: cast reint s_2_50 -> u128
        let s_2_53: u128 = (s_2_50.value() as u128);
        // D s_2_54: size-of s_2_50
        let s_2_54: u16 = s_2_50.length();
        // D s_2_55: cast reint s_2_52 -> u128
        let s_2_55: u128 = (s_2_52.value() as u128);
        // D s_2_56: size-of s_2_52
        let s_2_56: u16 = s_2_52.length();
        // D s_2_57: lsl s_2_53 s_2_56
        let s_2_57: u128 = s_2_53 << s_2_56;
        // D s_2_58: or s_2_57 s_2_55
        let s_2_58: u128 = ((s_2_57) | (s_2_55));
        // D s_2_59: add s_2_54 s_2_56
        let s_2_59: u16 = (s_2_54 + s_2_56);
        // D s_2_60: create-bits s_2_58 s_2_59
        let s_2_60: Bits = Bits::new(s_2_58, s_2_59);
        // D s_2_61: cast reint s_2_60 -> u8
        let s_2_61: u8 = (s_2_60.value() as u8);
        // D s_2_62: cast zx s_2_61 -> bv
        let s_2_62: Bits = Bits::new(s_2_61 as u128, 6u16);
        // D s_2_63: read-var g:u8
        let s_2_63: bool = fn_state.g;
        // D s_2_64: cast zx s_2_63 -> bv
        let s_2_64: Bits = Bits::new(s_2_63 as u128, 1u16);
        // D s_2_65: cast reint s_2_62 -> u128
        let s_2_65: u128 = (s_2_62.value() as u128);
        // D s_2_66: size-of s_2_62
        let s_2_66: u16 = s_2_62.length();
        // D s_2_67: cast reint s_2_64 -> u128
        let s_2_67: u128 = (s_2_64.value() as u128);
        // D s_2_68: size-of s_2_64
        let s_2_68: u16 = s_2_64.length();
        // D s_2_69: lsl s_2_65 s_2_68
        let s_2_69: u128 = s_2_65 << s_2_68;
        // D s_2_70: or s_2_69 s_2_67
        let s_2_70: u128 = ((s_2_69) | (s_2_67));
        // D s_2_71: add s_2_66 s_2_68
        let s_2_71: u16 = (s_2_66 + s_2_68);
        // D s_2_72: create-bits s_2_70 s_2_71
        let s_2_72: Bits = Bits::new(s_2_70, s_2_71);
        // D s_2_73: cast reint s_2_72 -> u8
        let s_2_73: u8 = (s_2_72.value() as u8);
        // D s_2_74: cast zx s_2_73 -> bv
        let s_2_74: Bits = Bits::new(s_2_73 as u128, 7u16);
        // D s_2_75: read-var h:u8
        let s_2_75: bool = fn_state.h;
        // D s_2_76: cast zx s_2_75 -> bv
        let s_2_76: Bits = Bits::new(s_2_75 as u128, 1u16);
        // D s_2_77: cast reint s_2_74 -> u128
        let s_2_77: u128 = (s_2_74.value() as u128);
        // D s_2_78: size-of s_2_74
        let s_2_78: u16 = s_2_74.length();
        // D s_2_79: cast reint s_2_76 -> u128
        let s_2_79: u128 = (s_2_76.value() as u128);
        // D s_2_80: size-of s_2_76
        let s_2_80: u16 = s_2_76.length();
        // D s_2_81: lsl s_2_77 s_2_80
        let s_2_81: u128 = s_2_77 << s_2_80;
        // D s_2_82: or s_2_81 s_2_79
        let s_2_82: u128 = ((s_2_81) | (s_2_79));
        // D s_2_83: add s_2_78 s_2_80
        let s_2_83: u16 = (s_2_78 + s_2_80);
        // D s_2_84: create-bits s_2_82 s_2_83
        let s_2_84: Bits = Bits::new(s_2_82, s_2_83);
        // D s_2_85: cast reint s_2_84 -> u8
        let s_2_85: u8 = (s_2_84.value() as u8);
        // C s_2_86: const #7s : i
        let s_2_86: i128 = 7;
        // D s_2_87: cast zx s_2_85 -> bv
        let s_2_87: Bits = Bits::new(s_2_85 as u128, 8u16);
        // C s_2_88: const #1u : u64
        let s_2_88: u64 = 1;
        // D s_2_89: bit-extract s_2_87 s_2_86 s_2_88
        let s_2_89: Bits = (Bits::new(
            ((s_2_87) >> (s_2_86)).value(),
            u16::try_from(s_2_88).unwrap(),
        ));
        // D s_2_90: cast reint s_2_89 -> u8
        let s_2_90: bool = ((s_2_89.value()) != 0);
        // C s_2_91: const #0s : i
        let s_2_91: i128 = 0;
        // C s_2_92: const #0u : u64
        let s_2_92: u64 = 0;
        // D s_2_93: cast zx s_2_90 -> u64
        let s_2_93: u64 = (s_2_90 as u64);
        // C s_2_94: const #1u : u64
        let s_2_94: u64 = 1;
        // D s_2_95: and s_2_93 s_2_94
        let s_2_95: u64 = ((s_2_93) & (s_2_94));
        // D s_2_96: cmp-eq s_2_95 s_2_94
        let s_2_96: bool = ((s_2_95) == (s_2_94));
        // D s_2_97: lsl s_2_93 s_2_91
        let s_2_97: u64 = s_2_93 << s_2_91;
        // D s_2_98: or s_2_92 s_2_97
        let s_2_98: u64 = ((s_2_92) | (s_2_97));
        // D s_2_99: cmpl s_2_97
        let s_2_99: u64 = !s_2_97;
        // D s_2_100: and s_2_92 s_2_99
        let s_2_100: u64 = ((s_2_92) & (s_2_99));
        // D s_2_101: select s_2_96 s_2_98 s_2_100
        let s_2_101: u64 = if s_2_96 { s_2_98 } else { s_2_100 };
        // D s_2_102: cast trunc s_2_101 -> u8
        let s_2_102: bool = ((s_2_101) != 0);
        // C s_2_103: const #6s : i
        let s_2_103: i128 = 6;
        // D s_2_104: cast zx s_2_85 -> bv
        let s_2_104: Bits = Bits::new(s_2_85 as u128, 8u16);
        // C s_2_105: const #1u : u64
        let s_2_105: u64 = 1;
        // D s_2_106: bit-extract s_2_104 s_2_103 s_2_105
        let s_2_106: Bits = (Bits::new(
            ((s_2_104) >> (s_2_103)).value(),
            u16::try_from(s_2_105).unwrap(),
        ));
        // D s_2_107: cast reint s_2_106 -> u8
        let s_2_107: bool = ((s_2_106.value()) != 0);
        // C s_2_108: const #0s : i
        let s_2_108: i128 = 0;
        // C s_2_109: const #0u : u64
        let s_2_109: u64 = 0;
        // D s_2_110: cast zx s_2_107 -> u64
        let s_2_110: u64 = (s_2_107 as u64);
        // C s_2_111: const #1u : u64
        let s_2_111: u64 = 1;
        // D s_2_112: and s_2_110 s_2_111
        let s_2_112: u64 = ((s_2_110) & (s_2_111));
        // D s_2_113: cmp-eq s_2_112 s_2_111
        let s_2_113: bool = ((s_2_112) == (s_2_111));
        // D s_2_114: lsl s_2_110 s_2_108
        let s_2_114: u64 = s_2_110 << s_2_108;
        // D s_2_115: or s_2_109 s_2_114
        let s_2_115: u64 = ((s_2_109) | (s_2_114));
        // D s_2_116: cmpl s_2_114
        let s_2_116: u64 = !s_2_114;
        // D s_2_117: and s_2_109 s_2_116
        let s_2_117: u64 = ((s_2_109) & (s_2_116));
        // D s_2_118: select s_2_113 s_2_115 s_2_117
        let s_2_118: u64 = if s_2_113 { s_2_115 } else { s_2_117 };
        // D s_2_119: cast trunc s_2_118 -> u8
        let s_2_119: bool = ((s_2_118) != 0);
        // D s_2_120: cast zx s_2_119 -> bv
        let s_2_120: Bits = Bits::new(s_2_119 as u128, 1u16);
        // D s_2_121: not s_2_120
        let s_2_121: Bits = !s_2_120;
        // D s_2_122: cast reint s_2_121 -> u8
        let s_2_122: bool = ((s_2_121.value()) != 0);
        // D s_2_123: cast zx s_2_102 -> bv
        let s_2_123: Bits = Bits::new(s_2_102 as u128, 1u16);
        // D s_2_124: cast zx s_2_122 -> bv
        let s_2_124: Bits = Bits::new(s_2_122 as u128, 1u16);
        // D s_2_125: cast reint s_2_123 -> u128
        let s_2_125: u128 = (s_2_123.value() as u128);
        // D s_2_126: size-of s_2_123
        let s_2_126: u16 = s_2_123.length();
        // D s_2_127: cast reint s_2_124 -> u128
        let s_2_127: u128 = (s_2_124.value() as u128);
        // D s_2_128: size-of s_2_124
        let s_2_128: u16 = s_2_124.length();
        // D s_2_129: lsl s_2_125 s_2_128
        let s_2_129: u128 = s_2_125 << s_2_128;
        // D s_2_130: or s_2_129 s_2_127
        let s_2_130: u128 = ((s_2_129) | (s_2_127));
        // D s_2_131: add s_2_126 s_2_128
        let s_2_131: u16 = (s_2_126 + s_2_128);
        // D s_2_132: create-bits s_2_130 s_2_131
        let s_2_132: Bits = Bits::new(s_2_130, s_2_131);
        // D s_2_133: cast reint s_2_132 -> u8
        let s_2_133: u8 = (s_2_132.value() as u8);
        // C s_2_134: const #6s : i
        let s_2_134: i128 = 6;
        // D s_2_135: cast zx s_2_85 -> bv
        let s_2_135: Bits = Bits::new(s_2_85 as u128, 8u16);
        // C s_2_136: const #1u : u64
        let s_2_136: u64 = 1;
        // D s_2_137: bit-extract s_2_135 s_2_134 s_2_136
        let s_2_137: Bits = (Bits::new(
            ((s_2_135) >> (s_2_134)).value(),
            u16::try_from(s_2_136).unwrap(),
        ));
        // D s_2_138: cast reint s_2_137 -> u8
        let s_2_138: bool = ((s_2_137.value()) != 0);
        // C s_2_139: const #0s : i
        let s_2_139: i128 = 0;
        // C s_2_140: const #0u : u64
        let s_2_140: u64 = 0;
        // D s_2_141: cast zx s_2_138 -> u64
        let s_2_141: u64 = (s_2_138 as u64);
        // C s_2_142: const #1u : u64
        let s_2_142: u64 = 1;
        // D s_2_143: and s_2_141 s_2_142
        let s_2_143: u64 = ((s_2_141) & (s_2_142));
        // D s_2_144: cmp-eq s_2_143 s_2_142
        let s_2_144: bool = ((s_2_143) == (s_2_142));
        // D s_2_145: lsl s_2_141 s_2_139
        let s_2_145: u64 = s_2_141 << s_2_139;
        // D s_2_146: or s_2_140 s_2_145
        let s_2_146: u64 = ((s_2_140) | (s_2_145));
        // D s_2_147: cmpl s_2_145
        let s_2_147: u64 = !s_2_145;
        // D s_2_148: and s_2_140 s_2_147
        let s_2_148: u64 = ((s_2_140) & (s_2_147));
        // D s_2_149: select s_2_144 s_2_146 s_2_148
        let s_2_149: u64 = if s_2_144 { s_2_146 } else { s_2_148 };
        // D s_2_150: cast trunc s_2_149 -> u8
        let s_2_150: bool = ((s_2_149) != 0);
        // D s_2_151: cast zx s_2_150 -> bv
        let s_2_151: Bits = Bits::new(s_2_150 as u128, 1u16);
        // C s_2_152: const #2u : u64
        let s_2_152: u64 = 2;
        // D s_2_153: call replicate_bits_borealis_internal(s_2_151, s_2_152)
        let s_2_153: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_2_151,
            s_2_152,
        );
        // D s_2_154: cast reint s_2_153 -> u8
        let s_2_154: u8 = (s_2_153.value() as u8);
        // D s_2_155: cast zx s_2_133 -> bv
        let s_2_155: Bits = Bits::new(s_2_133 as u128, 2u16);
        // D s_2_156: cast zx s_2_154 -> bv
        let s_2_156: Bits = Bits::new(s_2_154 as u128, 2u16);
        // D s_2_157: cast reint s_2_155 -> u128
        let s_2_157: u128 = (s_2_155.value() as u128);
        // D s_2_158: size-of s_2_155
        let s_2_158: u16 = s_2_155.length();
        // D s_2_159: cast reint s_2_156 -> u128
        let s_2_159: u128 = (s_2_156.value() as u128);
        // D s_2_160: size-of s_2_156
        let s_2_160: u16 = s_2_156.length();
        // D s_2_161: lsl s_2_157 s_2_160
        let s_2_161: u128 = s_2_157 << s_2_160;
        // D s_2_162: or s_2_161 s_2_159
        let s_2_162: u128 = ((s_2_161) | (s_2_159));
        // D s_2_163: add s_2_158 s_2_160
        let s_2_163: u16 = (s_2_158 + s_2_160);
        // D s_2_164: create-bits s_2_162 s_2_163
        let s_2_164: Bits = Bits::new(s_2_162, s_2_163);
        // D s_2_165: cast reint s_2_164 -> u8
        let s_2_165: u8 = (s_2_164.value() as u8);
        // C s_2_166: const #0s : i
        let s_2_166: i128 = 0;
        // D s_2_167: cast zx s_2_85 -> bv
        let s_2_167: Bits = Bits::new(s_2_85 as u128, 8u16);
        // C s_2_168: const #1s : i64
        let s_2_168: i64 = 1;
        // C s_2_169: cast zx s_2_168 -> i
        let s_2_169: i128 = (i128::try_from(s_2_168).unwrap());
        // C s_2_170: const #5s : i
        let s_2_170: i128 = 5;
        // C s_2_171: add s_2_170 s_2_169
        let s_2_171: i128 = (s_2_170 + s_2_169);
        // D s_2_172: bit-extract s_2_167 s_2_166 s_2_171
        let s_2_172: Bits = (Bits::new(
            ((s_2_167) >> (s_2_166)).value(),
            u16::try_from(s_2_171).unwrap(),
        ));
        // D s_2_173: cast reint s_2_172 -> u8
        let s_2_173: u8 = (s_2_172.value() as u8);
        // D s_2_174: cast zx s_2_165 -> bv
        let s_2_174: Bits = Bits::new(s_2_165 as u128, 4u16);
        // D s_2_175: cast zx s_2_173 -> bv
        let s_2_175: Bits = Bits::new(s_2_173 as u128, 6u16);
        // D s_2_176: cast reint s_2_174 -> u128
        let s_2_176: u128 = (s_2_174.value() as u128);
        // D s_2_177: size-of s_2_174
        let s_2_177: u16 = s_2_174.length();
        // D s_2_178: cast reint s_2_175 -> u128
        let s_2_178: u128 = (s_2_175.value() as u128);
        // D s_2_179: size-of s_2_175
        let s_2_179: u16 = s_2_175.length();
        // D s_2_180: lsl s_2_176 s_2_179
        let s_2_180: u128 = s_2_176 << s_2_179;
        // D s_2_181: or s_2_180 s_2_178
        let s_2_181: u128 = ((s_2_180) | (s_2_178));
        // D s_2_182: add s_2_177 s_2_179
        let s_2_182: u16 = (s_2_177 + s_2_179);
        // D s_2_183: create-bits s_2_181 s_2_182
        let s_2_183: Bits = Bits::new(s_2_181, s_2_182);
        // D s_2_184: cast reint s_2_183 -> u10
        let s_2_184: u16 = (s_2_183.value() as u16);
        // D s_2_185: cast zx s_2_184 -> bv
        let s_2_185: Bits = Bits::new(s_2_184 as u128, 10u16);
        // C s_2_186: const #0u : u8
        let s_2_186: u8 = 0;
        // C s_2_187: cast zx s_2_186 -> bv
        let s_2_187: Bits = Bits::new(s_2_186 as u128, 6u16);
        // D s_2_188: cast reint s_2_185 -> u128
        let s_2_188: u128 = (s_2_185.value() as u128);
        // D s_2_189: size-of s_2_185
        let s_2_189: u16 = s_2_185.length();
        // C s_2_190: cast reint s_2_187 -> u128
        let s_2_190: u128 = (s_2_187.value() as u128);
        // D s_2_191: size-of s_2_187
        let s_2_191: u16 = s_2_187.length();
        // D s_2_192: lsl s_2_188 s_2_191
        let s_2_192: u128 = s_2_188 << s_2_191;
        // D s_2_193: or s_2_192 s_2_190
        let s_2_193: u128 = ((s_2_192) | (s_2_190));
        // D s_2_194: add s_2_189 s_2_191
        let s_2_194: u16 = (s_2_189 + s_2_191);
        // D s_2_195: create-bits s_2_193 s_2_194
        let s_2_195: Bits = Bits::new(s_2_193, s_2_194);
        // D s_2_196: cast reint s_2_195 -> u16
        let s_2_196: u16 = (s_2_195.value() as u16);
        // C s_2_197: const #16s : i
        let s_2_197: i128 = 16;
        // D s_2_198: cast zx s_2_0 -> i
        let s_2_198: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_199: div s_2_198 s_2_197
        let s_2_199: i128 = ((s_2_198) / (s_2_197));
        // D s_2_200: cast reint s_2_199 -> i64
        let s_2_200: i64 = (s_2_199 as i64);
        // D s_2_201: cast zx s_2_196 -> bv
        let s_2_201: Bits = Bits::new(s_2_196 as u128, 16u16);
        // D s_2_202: cast zx s_2_200 -> i
        let s_2_202: i128 = (i128::try_from(s_2_200).unwrap());
        // D s_2_203: cast reint s_2_202 -> u64
        let s_2_203: u64 = (s_2_202 as u64);
        // D s_2_204: call replicate_bits_borealis_internal(s_2_201, s_2_203)
        let s_2_204: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_2_201,
            s_2_203,
        );
        // D s_2_205: cast zx s_2_0 -> i
        let s_2_205: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_206: cast reint s_2_205 -> i64
        let s_2_206: i64 = (s_2_205 as i64);
        // D s_2_207: read-var rd:i64
        let s_2_207: i64 = fn_state.rd;
        // D s_2_208: call execute_aarch64_instrs_vector_fp16_movi(s_2_206, s_2_204, s_2_207)
        let s_2_208: () = execute_aarch64_instrs_vector_fp16_movi(
            state,
            tracer,
            s_2_206,
            s_2_204,
            s_2_207,
        );
        // N s_2_209: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: write-var ga#255783 <= s_3_0
        fn_state.ga_255783 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
