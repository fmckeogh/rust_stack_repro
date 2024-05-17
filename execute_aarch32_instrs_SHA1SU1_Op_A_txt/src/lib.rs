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
use ROL::*;
use CheckCryptoEnabled32::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_SHA1SU1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
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
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // D s_1_6: read-var m:i64
        let s_1_6: i64 = fn_state.m;
        // D s_1_7: lsr s_1_6 s_1_5
        let s_1_7: i64 = s_1_6 >> s_1_5;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call Q_read(s_1_8)
        let s_1_9: u128 = Q_read(state, tracer, s_1_8);
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
        // C s_1_30: const #32s : i
        let s_1_30: i128 = 32;
        // D s_1_31: cast zx s_1_17 -> bv
        let s_1_31: Bits = Bits::new(s_1_17 as u128, 128u16);
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
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: cast zx s_1_37 -> bv
        let s_1_39: Bits = Bits::new(s_1_37 as u128, 32u16);
        // D s_1_40: call ROL(s_1_39, s_1_38)
        let s_1_40: Bits = ROL(state, tracer, s_1_39, s_1_38);
        // D s_1_41: cast reint s_1_40 -> u32
        let s_1_41: u32 = (s_1_40.value() as u32);
        // C s_1_42: const #64s : i
        let s_1_42: i128 = 64;
        // D s_1_43: cast zx s_1_17 -> bv
        let s_1_43: Bits = Bits::new(s_1_17 as u128, 128u16);
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
        // C s_1_50: const #1s : i
        let s_1_50: i128 = 1;
        // D s_1_51: cast zx s_1_49 -> bv
        let s_1_51: Bits = Bits::new(s_1_49 as u128, 32u16);
        // D s_1_52: call ROL(s_1_51, s_1_50)
        let s_1_52: Bits = ROL(state, tracer, s_1_51, s_1_50);
        // D s_1_53: cast reint s_1_52 -> u32
        let s_1_53: u32 = (s_1_52.value() as u32);
        // C s_1_54: const #96s : i
        let s_1_54: i128 = 96;
        // D s_1_55: cast zx s_1_17 -> bv
        let s_1_55: Bits = Bits::new(s_1_17 as u128, 128u16);
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
        // C s_1_62: const #1s : i
        let s_1_62: i128 = 1;
        // D s_1_63: cast zx s_1_61 -> bv
        let s_1_63: Bits = Bits::new(s_1_61 as u128, 32u16);
        // D s_1_64: call ROL(s_1_63, s_1_62)
        let s_1_64: Bits = ROL(state, tracer, s_1_63, s_1_62);
        // D s_1_65: cast reint s_1_64 -> u32
        let s_1_65: u32 = (s_1_64.value() as u32);
        // C s_1_66: const #0s : i
        let s_1_66: i128 = 0;
        // D s_1_67: cast zx s_1_17 -> bv
        let s_1_67: Bits = Bits::new(s_1_17 as u128, 128u16);
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
        // C s_1_74: const #2s : i
        let s_1_74: i128 = 2;
        // D s_1_75: cast zx s_1_73 -> bv
        let s_1_75: Bits = Bits::new(s_1_73 as u128, 32u16);
        // D s_1_76: call ROL(s_1_75, s_1_74)
        let s_1_76: Bits = ROL(state, tracer, s_1_75, s_1_74);
        // D s_1_77: cast reint s_1_76 -> u32
        let s_1_77: u32 = (s_1_76.value() as u32);
        // D s_1_78: cast zx s_1_65 -> bv
        let s_1_78: Bits = Bits::new(s_1_65 as u128, 32u16);
        // D s_1_79: cast zx s_1_77 -> bv
        let s_1_79: Bits = Bits::new(s_1_77 as u128, 32u16);
        // D s_1_80: xor s_1_78 s_1_79
        let s_1_80: Bits = ((s_1_78) ^ (s_1_79));
        // D s_1_81: cast reint s_1_80 -> u32
        let s_1_81: u32 = (s_1_80.value() as u32);
        // C s_1_82: const #1s : i64
        let s_1_82: i64 = 1;
        // D s_1_83: read-var d:i64
        let s_1_83: i64 = fn_state.d;
        // D s_1_84: lsr s_1_83 s_1_82
        let s_1_84: i64 = s_1_83 >> s_1_82;
        // D s_1_85: cast zx s_1_81 -> bv
        let s_1_85: Bits = Bits::new(s_1_81 as u128, 32u16);
        // D s_1_86: cast zx s_1_53 -> bv
        let s_1_86: Bits = Bits::new(s_1_53 as u128, 32u16);
        // D s_1_87: cast reint s_1_85 -> u128
        let s_1_87: u128 = (s_1_85.value() as u128);
        // D s_1_88: size-of s_1_85
        let s_1_88: u16 = s_1_85.length();
        // D s_1_89: cast reint s_1_86 -> u128
        let s_1_89: u128 = (s_1_86.value() as u128);
        // D s_1_90: size-of s_1_86
        let s_1_90: u16 = s_1_86.length();
        // D s_1_91: lsl s_1_87 s_1_90
        let s_1_91: u128 = s_1_87 << s_1_90;
        // D s_1_92: or s_1_91 s_1_89
        let s_1_92: u128 = ((s_1_91) | (s_1_89));
        // D s_1_93: add s_1_88 s_1_90
        let s_1_93: u16 = (s_1_88 + s_1_90);
        // D s_1_94: create-bits s_1_92 s_1_93
        let s_1_94: Bits = Bits::new(s_1_92, s_1_93);
        // D s_1_95: cast reint s_1_94 -> u64
        let s_1_95: u64 = (s_1_94.value() as u64);
        // D s_1_96: cast zx s_1_95 -> bv
        let s_1_96: Bits = Bits::new(s_1_95 as u128, 64u16);
        // D s_1_97: cast zx s_1_41 -> bv
        let s_1_97: Bits = Bits::new(s_1_41 as u128, 32u16);
        // D s_1_98: cast reint s_1_96 -> u128
        let s_1_98: u128 = (s_1_96.value() as u128);
        // D s_1_99: size-of s_1_96
        let s_1_99: u16 = s_1_96.length();
        // D s_1_100: cast reint s_1_97 -> u128
        let s_1_100: u128 = (s_1_97.value() as u128);
        // D s_1_101: size-of s_1_97
        let s_1_101: u16 = s_1_97.length();
        // D s_1_102: lsl s_1_98 s_1_101
        let s_1_102: u128 = s_1_98 << s_1_101;
        // D s_1_103: or s_1_102 s_1_100
        let s_1_103: u128 = ((s_1_102) | (s_1_100));
        // D s_1_104: add s_1_99 s_1_101
        let s_1_104: u16 = (s_1_99 + s_1_101);
        // D s_1_105: create-bits s_1_103 s_1_104
        let s_1_105: Bits = Bits::new(s_1_103, s_1_104);
        // D s_1_106: cast reint s_1_105 -> u96
        let s_1_106: u128 = (s_1_105.value() as u128);
        // D s_1_107: cast zx s_1_106 -> bv
        let s_1_107: Bits = Bits::new(s_1_106 as u128, 96u16);
        // D s_1_108: cast zx s_1_29 -> bv
        let s_1_108: Bits = Bits::new(s_1_29 as u128, 32u16);
        // D s_1_109: cast reint s_1_107 -> u128
        let s_1_109: u128 = (s_1_107.value() as u128);
        // D s_1_110: size-of s_1_107
        let s_1_110: u16 = s_1_107.length();
        // D s_1_111: cast reint s_1_108 -> u128
        let s_1_111: u128 = (s_1_108.value() as u128);
        // D s_1_112: size-of s_1_108
        let s_1_112: u16 = s_1_108.length();
        // D s_1_113: lsl s_1_109 s_1_112
        let s_1_113: u128 = s_1_109 << s_1_112;
        // D s_1_114: or s_1_113 s_1_111
        let s_1_114: u128 = ((s_1_113) | (s_1_111));
        // D s_1_115: add s_1_110 s_1_112
        let s_1_115: u16 = (s_1_110 + s_1_112);
        // D s_1_116: create-bits s_1_114 s_1_115
        let s_1_116: Bits = Bits::new(s_1_114, s_1_115);
        // D s_1_117: cast reint s_1_116 -> u128
        let s_1_117: u128 = (s_1_116.value() as u128);
        // D s_1_118: cast zx s_1_84 -> i
        let s_1_118: i128 = (i128::try_from(s_1_84).unwrap());
        // D s_1_119: call Q_set(s_1_118, s_1_117)
        let s_1_119: () = Q_set(state, tracer, s_1_118, s_1_117);
        // N s_1_120: return
        return;
    }
}
