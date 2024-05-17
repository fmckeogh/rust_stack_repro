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
use Q_read::*;
use SHAmajority::*;
use ROL::*;
use Elem_read::*;
use CheckCryptoEnabled32::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_SHA1M_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        y: u32,
        w: u128,
        x: u128,
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
        // S s_0_1: call CheckCryptoEnabled32(s_0_0)
        let s_0_1: () = CheckCryptoEnabled32(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i64
        let s_1_0: i64 = 1;
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // D s_1_5: write-var x <= s_1_4
        fn_state.x = s_1_4;
        // C s_1_6: const #1s : i64
        let s_1_6: i64 = 1;
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: lsr s_1_7 s_1_6
        let s_1_8: i64 = s_1_7 >> s_1_6;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: call Q_read(s_1_9)
        let s_1_10: u128 = Q_read(state, tracer, s_1_9);
        // C s_1_11: const #0s : i
        let s_1_11: i128 = 0;
        // D s_1_12: cast zx s_1_10 -> bv
        let s_1_12: Bits = Bits::new(s_1_10 as u128, 128u16);
        // C s_1_13: const #1s : i64
        let s_1_13: i64 = 1;
        // C s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // C s_1_15: const #31s : i
        let s_1_15: i128 = 31;
        // C s_1_16: add s_1_15 s_1_14
        let s_1_16: i128 = (s_1_15 + s_1_14);
        // D s_1_17: bit-extract s_1_12 s_1_11 s_1_16
        let s_1_17: Bits = (Bits::new(
            ((s_1_12) >> (s_1_11)).value(),
            u16::try_from(s_1_16).unwrap(),
        ));
        // D s_1_18: cast reint s_1_17 -> u32
        let s_1_18: u32 = (s_1_17.value() as u32);
        // D s_1_19: write-var y <= s_1_18
        fn_state.y = s_1_18;
        // C s_1_20: const #1s : i64
        let s_1_20: i64 = 1;
        // D s_1_21: read-var m:i64
        let s_1_21: i64 = fn_state.m;
        // D s_1_22: lsr s_1_21 s_1_20
        let s_1_22: i64 = s_1_21 >> s_1_20;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: call Q_read(s_1_23)
        let s_1_24: u128 = Q_read(state, tracer, s_1_23);
        // D s_1_25: write-var w <= s_1_24
        fn_state.w = s_1_24;
        // C s_1_26: const #0s : i64
        let s_1_26: i64 = 0;
        // D s_1_27: write-var e <= s_1_26
        fn_state.e = s_1_26;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i
        let s_3_0: i128 = 32;
        // D s_3_1: read-var x:u128
        let s_3_1: u128 = fn_state.x;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 128u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #31s : i
        let s_3_5: i128 = 31;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u32
        let s_3_8: u32 = (s_3_7.value() as u32);
        // C s_3_9: const #64s : i
        let s_3_9: i128 = 64;
        // D s_3_10: read-var x:u128
        let s_3_10: u128 = fn_state.x;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 128u16);
        // C s_3_12: const #1s : i64
        let s_3_12: i64 = 1;
        // C s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // C s_3_14: const #31s : i
        let s_3_14: i128 = 31;
        // C s_3_15: add s_3_14 s_3_13
        let s_3_15: i128 = (s_3_14 + s_3_13);
        // D s_3_16: bit-extract s_3_11 s_3_9 s_3_15
        let s_3_16: Bits = (Bits::new(
            ((s_3_11) >> (s_3_9)).value(),
            u16::try_from(s_3_15).unwrap(),
        ));
        // D s_3_17: cast reint s_3_16 -> u32
        let s_3_17: u32 = (s_3_16.value() as u32);
        // C s_3_18: const #96s : i
        let s_3_18: i128 = 96;
        // D s_3_19: read-var x:u128
        let s_3_19: u128 = fn_state.x;
        // D s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 128u16);
        // C s_3_21: const #1s : i64
        let s_3_21: i64 = 1;
        // C s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // C s_3_23: const #31s : i
        let s_3_23: i128 = 31;
        // C s_3_24: add s_3_23 s_3_22
        let s_3_24: i128 = (s_3_23 + s_3_22);
        // D s_3_25: bit-extract s_3_20 s_3_18 s_3_24
        let s_3_25: Bits = (Bits::new(
            ((s_3_20) >> (s_3_18)).value(),
            u16::try_from(s_3_24).unwrap(),
        ));
        // D s_3_26: cast reint s_3_25 -> u32
        let s_3_26: u32 = (s_3_25.value() as u32);
        // D s_3_27: call SHAmajority(s_3_8, s_3_17, s_3_26)
        let s_3_27: u32 = SHAmajority(state, tracer, s_3_8, s_3_17, s_3_26);
        // C s_3_28: const #0s : i
        let s_3_28: i128 = 0;
        // D s_3_29: read-var x:u128
        let s_3_29: u128 = fn_state.x;
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 128u16);
        // C s_3_31: const #1s : i64
        let s_3_31: i64 = 1;
        // C s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // C s_3_33: const #31s : i
        let s_3_33: i128 = 31;
        // C s_3_34: add s_3_33 s_3_32
        let s_3_34: i128 = (s_3_33 + s_3_32);
        // D s_3_35: bit-extract s_3_30 s_3_28 s_3_34
        let s_3_35: Bits = (Bits::new(
            ((s_3_30) >> (s_3_28)).value(),
            u16::try_from(s_3_34).unwrap(),
        ));
        // D s_3_36: cast reint s_3_35 -> u32
        let s_3_36: u32 = (s_3_35.value() as u32);
        // C s_3_37: const #5s : i
        let s_3_37: i128 = 5;
        // D s_3_38: cast zx s_3_36 -> bv
        let s_3_38: Bits = Bits::new(s_3_36 as u128, 32u16);
        // D s_3_39: call ROL(s_3_38, s_3_37)
        let s_3_39: Bits = ROL(state, tracer, s_3_38, s_3_37);
        // D s_3_40: cast reint s_3_39 -> u32
        let s_3_40: u32 = (s_3_39.value() as u32);
        // D s_3_41: read-var y:u32
        let s_3_41: u32 = fn_state.y;
        // D s_3_42: cast zx s_3_41 -> bv
        let s_3_42: Bits = Bits::new(s_3_41 as u128, 32u16);
        // D s_3_43: cast zx s_3_40 -> bv
        let s_3_43: Bits = Bits::new(s_3_40 as u128, 32u16);
        // D s_3_44: add s_3_42 s_3_43
        let s_3_44: Bits = (s_3_42 + s_3_43);
        // D s_3_45: cast reint s_3_44 -> u32
        let s_3_45: u32 = (s_3_44.value() as u32);
        // D s_3_46: cast zx s_3_45 -> bv
        let s_3_46: Bits = Bits::new(s_3_45 as u128, 32u16);
        // D s_3_47: cast zx s_3_27 -> bv
        let s_3_47: Bits = Bits::new(s_3_27 as u128, 32u16);
        // D s_3_48: add s_3_46 s_3_47
        let s_3_48: Bits = (s_3_46 + s_3_47);
        // D s_3_49: cast reint s_3_48 -> u32
        let s_3_49: u32 = (s_3_48.value() as u32);
        // C s_3_50: const #32s : i64
        let s_3_50: i64 = 32;
        // D s_3_51: read-var w:u128
        let s_3_51: u128 = fn_state.w;
        // D s_3_52: cast zx s_3_51 -> bv
        let s_3_52: Bits = Bits::new(s_3_51 as u128, 128u16);
        // D s_3_53: read-var e:i64
        let s_3_53: i64 = fn_state.e;
        // D s_3_54: cast zx s_3_53 -> i
        let s_3_54: i128 = (i128::try_from(s_3_53).unwrap());
        // C s_3_55: cast zx s_3_50 -> i
        let s_3_55: i128 = (i128::try_from(s_3_50).unwrap());
        // D s_3_56: call Elem_read(s_3_52, s_3_54, s_3_55)
        let s_3_56: Bits = Elem_read(state, tracer, s_3_52, s_3_54, s_3_55);
        // D s_3_57: cast reint s_3_56 -> u32
        let s_3_57: u32 = (s_3_56.value() as u32);
        // D s_3_58: cast zx s_3_49 -> bv
        let s_3_58: Bits = Bits::new(s_3_49 as u128, 32u16);
        // D s_3_59: cast zx s_3_57 -> bv
        let s_3_59: Bits = Bits::new(s_3_57 as u128, 32u16);
        // D s_3_60: add s_3_58 s_3_59
        let s_3_60: Bits = (s_3_58 + s_3_59);
        // D s_3_61: cast reint s_3_60 -> u32
        let s_3_61: u32 = (s_3_60.value() as u32);
        // D s_3_62: write-var y <= s_3_61
        fn_state.y = s_3_61;
        // C s_3_63: const #32s : i
        let s_3_63: i128 = 32;
        // D s_3_64: read-var x:u128
        let s_3_64: u128 = fn_state.x;
        // D s_3_65: cast zx s_3_64 -> bv
        let s_3_65: Bits = Bits::new(s_3_64 as u128, 128u16);
        // C s_3_66: const #1s : i64
        let s_3_66: i64 = 1;
        // C s_3_67: cast zx s_3_66 -> i
        let s_3_67: i128 = (i128::try_from(s_3_66).unwrap());
        // C s_3_68: const #31s : i
        let s_3_68: i128 = 31;
        // C s_3_69: add s_3_68 s_3_67
        let s_3_69: i128 = (s_3_68 + s_3_67);
        // D s_3_70: bit-extract s_3_65 s_3_63 s_3_69
        let s_3_70: Bits = (Bits::new(
            ((s_3_65) >> (s_3_63)).value(),
            u16::try_from(s_3_69).unwrap(),
        ));
        // D s_3_71: cast reint s_3_70 -> u32
        let s_3_71: u32 = (s_3_70.value() as u32);
        // C s_3_72: const #30s : i
        let s_3_72: i128 = 30;
        // D s_3_73: cast zx s_3_71 -> bv
        let s_3_73: Bits = Bits::new(s_3_71 as u128, 32u16);
        // D s_3_74: call ROL(s_3_73, s_3_72)
        let s_3_74: Bits = ROL(state, tracer, s_3_73, s_3_72);
        // D s_3_75: cast reint s_3_74 -> u32
        let s_3_75: u32 = (s_3_74.value() as u32);
        // C s_3_76: const #32s : i
        let s_3_76: i128 = 32;
        // D s_3_77: read-var x:u128
        let s_3_77: u128 = fn_state.x;
        // D s_3_78: cast zx s_3_77 -> bv
        let s_3_78: Bits = Bits::new(s_3_77 as u128, 128u16);
        // D s_3_79: cast zx s_3_75 -> bv
        let s_3_79: Bits = Bits::new(s_3_75 as u128, 32u16);
        // C s_3_80: const #31s : i
        let s_3_80: i128 = 31;
        // C s_3_81: const #1u : u64
        let s_3_81: u64 = 1;
        // C s_3_82: cast zx s_3_81 -> bv
        let s_3_82: Bits = Bits::new(s_3_81 as u128, 64u16);
        // C s_3_83: lsl s_3_82 s_3_80
        let s_3_83: Bits = s_3_82 << s_3_80;
        // C s_3_84: sub s_3_83 s_3_82
        let s_3_84: Bits = ((s_3_83) - (s_3_82));
        // D s_3_85: and s_3_79 s_3_84
        let s_3_85: Bits = ((s_3_79) & (s_3_84));
        // D s_3_86: lsl s_3_85 s_3_76
        let s_3_86: Bits = s_3_85 << s_3_76;
        // C s_3_87: lsl s_3_84 s_3_76
        let s_3_87: Bits = s_3_84 << s_3_76;
        // C s_3_88: cmpl s_3_87
        let s_3_88: Bits = !s_3_87;
        // D s_3_89: and s_3_78 s_3_88
        let s_3_89: Bits = ((s_3_78) & (s_3_88));
        // D s_3_90: or s_3_89 s_3_86
        let s_3_90: Bits = ((s_3_89) | (s_3_86));
        // D s_3_91: cast reint s_3_90 -> u128
        let s_3_91: u128 = (s_3_90.value() as u128);
        // D s_3_92: write-var x <= s_3_91
        fn_state.x = s_3_91;
        // D s_3_93: read-var y:u32
        let s_3_93: u32 = fn_state.y;
        // D s_3_94: cast zx s_3_93 -> bv
        let s_3_94: Bits = Bits::new(s_3_93 as u128, 32u16);
        // D s_3_95: read-var x:u128
        let s_3_95: u128 = fn_state.x;
        // D s_3_96: cast zx s_3_95 -> bv
        let s_3_96: Bits = Bits::new(s_3_95 as u128, 128u16);
        // D s_3_97: cast reint s_3_94 -> u128
        let s_3_97: u128 = (s_3_94.value() as u128);
        // D s_3_98: size-of s_3_94
        let s_3_98: u16 = s_3_94.length();
        // D s_3_99: cast reint s_3_96 -> u128
        let s_3_99: u128 = (s_3_96.value() as u128);
        // D s_3_100: size-of s_3_96
        let s_3_100: u16 = s_3_96.length();
        // D s_3_101: lsl s_3_97 s_3_100
        let s_3_101: u128 = s_3_97 << s_3_100;
        // D s_3_102: or s_3_101 s_3_99
        let s_3_102: u128 = ((s_3_101) | (s_3_99));
        // D s_3_103: add s_3_98 s_3_100
        let s_3_103: u16 = (s_3_98 + s_3_100);
        // D s_3_104: create-bits s_3_102 s_3_103
        let s_3_104: Bits = Bits::new(s_3_102, s_3_103);
        // D s_3_105: cast reint s_3_104 -> u160
        let s_3_105: u64 = (s_3_104.value() as u64);
        // C s_3_106: const #32s : i
        let s_3_106: i128 = 32;
        // D s_3_107: cast zx s_3_105 -> bv
        let s_3_107: Bits = Bits::new(s_3_105 as u128, 160u16);
        // D s_3_108: call ROL(s_3_107, s_3_106)
        let s_3_108: Bits = ROL(state, tracer, s_3_107, s_3_106);
        // D s_3_109: cast reint s_3_108 -> u160
        let s_3_109: u64 = (s_3_108.value() as u64);
        // C s_3_110: const #128s : i
        let s_3_110: i128 = 128;
        // D s_3_111: cast zx s_3_109 -> bv
        let s_3_111: Bits = Bits::new(s_3_109 as u128, 160u16);
        // C s_3_112: const #1s : i64
        let s_3_112: i64 = 1;
        // C s_3_113: cast zx s_3_112 -> i
        let s_3_113: i128 = (i128::try_from(s_3_112).unwrap());
        // C s_3_114: const #31s : i
        let s_3_114: i128 = 31;
        // C s_3_115: add s_3_114 s_3_113
        let s_3_115: i128 = (s_3_114 + s_3_113);
        // D s_3_116: bit-extract s_3_111 s_3_110 s_3_115
        let s_3_116: Bits = (Bits::new(
            ((s_3_111) >> (s_3_110)).value(),
            u16::try_from(s_3_115).unwrap(),
        ));
        // D s_3_117: cast reint s_3_116 -> u32
        let s_3_117: u32 = (s_3_116.value() as u32);
        // D s_3_118: write-var y <= s_3_117
        fn_state.y = s_3_117;
        // C s_3_119: const #0s : i
        let s_3_119: i128 = 0;
        // D s_3_120: cast zx s_3_109 -> bv
        let s_3_120: Bits = Bits::new(s_3_109 as u128, 160u16);
        // C s_3_121: const #1s : i64
        let s_3_121: i64 = 1;
        // C s_3_122: cast zx s_3_121 -> i
        let s_3_122: i128 = (i128::try_from(s_3_121).unwrap());
        // C s_3_123: const #127s : i
        let s_3_123: i128 = 127;
        // C s_3_124: add s_3_123 s_3_122
        let s_3_124: i128 = (s_3_123 + s_3_122);
        // D s_3_125: bit-extract s_3_120 s_3_119 s_3_124
        let s_3_125: Bits = (Bits::new(
            ((s_3_120) >> (s_3_119)).value(),
            u16::try_from(s_3_124).unwrap(),
        ));
        // D s_3_126: cast reint s_3_125 -> u128
        let s_3_126: u128 = (s_3_125.value() as u128);
        // D s_3_127: write-var x <= s_3_126
        fn_state.x = s_3_126;
        // D s_3_128: read-var e:i64
        let s_3_128: i64 = fn_state.e;
        // C s_3_129: const #1s : i64
        let s_3_129: i64 = 1;
        // D s_3_130: add s_3_128 s_3_129
        let s_3_130: i64 = (s_3_128 + s_3_129);
        // D s_3_131: write-var e <= s_3_130
        fn_state.e = s_3_130;
        // N s_3_132: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i64
        let s_4_0: i64 = 1;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: lsr s_4_1 s_4_0
        let s_4_2: i64 = s_4_1 >> s_4_0;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var x:u128
        let s_4_4: u128 = fn_state.x;
        // D s_4_5: call Q_set(s_4_3, s_4_4)
        let s_4_5: () = Q_set(state, tracer, s_4_3, s_4_4);
        // N s_4_6: return
        return;
    }
}
