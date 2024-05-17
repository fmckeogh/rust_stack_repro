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
use Bit::*;
use common::*;
pub fn TweakCellRot<T: Tracer>(state: &mut State, tracer: &T, incell_name: u8) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        outcell: u8,
        incell_name: u8,
    }
    let fn_state = FunctionState {
        incell_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var incell_name:u8
        let s_0_1: u8 = fn_state.incell_name;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 4u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_0 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // C s_0_18: const #1s : i
        let s_0_18: i128 = 1;
        // D s_0_19: read-var incell_name:u8
        let s_0_19: u8 = fn_state.incell_name;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 4u16);
        // C s_0_21: const #1u : u64
        let s_0_21: u64 = 1;
        // D s_0_22: bit-extract s_0_20 s_0_18 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_20) >> (s_0_18)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_23: cast reint s_0_22 -> u8
        let s_0_23: bool = ((s_0_22.value()) != 0);
        // C s_0_24: const #0s : i
        let s_0_24: i128 = 0;
        // C s_0_25: const #0u : u64
        let s_0_25: u64 = 0;
        // D s_0_26: cast zx s_0_23 -> u64
        let s_0_26: u64 = (s_0_23 as u64);
        // C s_0_27: const #1u : u64
        let s_0_27: u64 = 1;
        // D s_0_28: and s_0_26 s_0_27
        let s_0_28: u64 = ((s_0_26) & (s_0_27));
        // D s_0_29: cmp-eq s_0_28 s_0_27
        let s_0_29: bool = ((s_0_28) == (s_0_27));
        // D s_0_30: lsl s_0_26 s_0_24
        let s_0_30: u64 = s_0_26 << s_0_24;
        // D s_0_31: or s_0_25 s_0_30
        let s_0_31: u64 = ((s_0_25) | (s_0_30));
        // D s_0_32: cmpl s_0_30
        let s_0_32: u64 = !s_0_30;
        // D s_0_33: and s_0_25 s_0_32
        let s_0_33: u64 = ((s_0_25) & (s_0_32));
        // D s_0_34: select s_0_29 s_0_31 s_0_33
        let s_0_34: u64 = if s_0_29 { s_0_31 } else { s_0_33 };
        // D s_0_35: cast trunc s_0_34 -> u8
        let s_0_35: bool = ((s_0_34) != 0);
        // D s_0_36: cast zx s_0_17 -> bv
        let s_0_36: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_37: cast zx s_0_35 -> bv
        let s_0_37: Bits = Bits::new(s_0_35 as u128, 1u16);
        // D s_0_38: xor s_0_36 s_0_37
        let s_0_38: Bits = ((s_0_36) ^ (s_0_37));
        // D s_0_39: cast reint s_0_38 -> u8
        let s_0_39: bool = ((s_0_38.value()) != 0);
        // D s_0_40: call Bit(s_0_39)
        let s_0_40: bool = Bit(state, tracer, s_0_39);
        // C s_0_41: const #3s : i
        let s_0_41: i128 = 3;
        // D s_0_42: read-var outcell:u8
        let s_0_42: u8 = fn_state.outcell;
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 4u16);
        // C s_0_44: const #1u : u64
        let s_0_44: u64 = 1;
        // D s_0_45: bit-insert s_0_43 s_0_43 s_0_41 s_0_44
        let s_0_45: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_44 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_43.length(),
            );
            (s_0_43 & mask) | (s_0_43 << s_0_41)
        };
        // D s_0_46: cast reint s_0_45 -> u8
        let s_0_46: u8 = (s_0_45.value() as u8);
        // D s_0_47: write-var outcell <= s_0_46
        fn_state.outcell = s_0_46;
        // C s_0_48: const #3s : i
        let s_0_48: i128 = 3;
        // D s_0_49: read-var incell_name:u8
        let s_0_49: u8 = fn_state.incell_name;
        // D s_0_50: cast zx s_0_49 -> bv
        let s_0_50: Bits = Bits::new(s_0_49 as u128, 4u16);
        // C s_0_51: const #1u : u64
        let s_0_51: u64 = 1;
        // D s_0_52: bit-extract s_0_50 s_0_48 s_0_51
        let s_0_52: Bits = (Bits::new(
            ((s_0_50) >> (s_0_48)).value(),
            u16::try_from(s_0_51).unwrap(),
        ));
        // D s_0_53: cast reint s_0_52 -> u8
        let s_0_53: bool = ((s_0_52.value()) != 0);
        // C s_0_54: const #0s : i
        let s_0_54: i128 = 0;
        // C s_0_55: const #0u : u64
        let s_0_55: u64 = 0;
        // D s_0_56: cast zx s_0_53 -> u64
        let s_0_56: u64 = (s_0_53 as u64);
        // C s_0_57: const #1u : u64
        let s_0_57: u64 = 1;
        // D s_0_58: and s_0_56 s_0_57
        let s_0_58: u64 = ((s_0_56) & (s_0_57));
        // D s_0_59: cmp-eq s_0_58 s_0_57
        let s_0_59: bool = ((s_0_58) == (s_0_57));
        // D s_0_60: lsl s_0_56 s_0_54
        let s_0_60: u64 = s_0_56 << s_0_54;
        // D s_0_61: or s_0_55 s_0_60
        let s_0_61: u64 = ((s_0_55) | (s_0_60));
        // D s_0_62: cmpl s_0_60
        let s_0_62: u64 = !s_0_60;
        // D s_0_63: and s_0_55 s_0_62
        let s_0_63: u64 = ((s_0_55) & (s_0_62));
        // D s_0_64: select s_0_59 s_0_61 s_0_63
        let s_0_64: u64 = if s_0_59 { s_0_61 } else { s_0_63 };
        // D s_0_65: cast trunc s_0_64 -> u8
        let s_0_65: bool = ((s_0_64) != 0);
        // D s_0_66: call Bit(s_0_65)
        let s_0_66: bool = Bit(state, tracer, s_0_65);
        // C s_0_67: const #2s : i
        let s_0_67: i128 = 2;
        // D s_0_68: cast zx s_0_46 -> bv
        let s_0_68: Bits = Bits::new(s_0_46 as u128, 4u16);
        // C s_0_69: const #1u : u64
        let s_0_69: u64 = 1;
        // D s_0_70: bit-insert s_0_68 s_0_68 s_0_67 s_0_69
        let s_0_70: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_69 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_68.length(),
            );
            (s_0_68 & mask) | (s_0_68 << s_0_67)
        };
        // D s_0_71: cast reint s_0_70 -> u8
        let s_0_71: u8 = (s_0_70.value() as u8);
        // D s_0_72: write-var outcell <= s_0_71
        fn_state.outcell = s_0_71;
        // C s_0_73: const #2s : i
        let s_0_73: i128 = 2;
        // D s_0_74: read-var incell_name:u8
        let s_0_74: u8 = fn_state.incell_name;
        // D s_0_75: cast zx s_0_74 -> bv
        let s_0_75: Bits = Bits::new(s_0_74 as u128, 4u16);
        // C s_0_76: const #1u : u64
        let s_0_76: u64 = 1;
        // D s_0_77: bit-extract s_0_75 s_0_73 s_0_76
        let s_0_77: Bits = (Bits::new(
            ((s_0_75) >> (s_0_73)).value(),
            u16::try_from(s_0_76).unwrap(),
        ));
        // D s_0_78: cast reint s_0_77 -> u8
        let s_0_78: bool = ((s_0_77.value()) != 0);
        // C s_0_79: const #0s : i
        let s_0_79: i128 = 0;
        // C s_0_80: const #0u : u64
        let s_0_80: u64 = 0;
        // D s_0_81: cast zx s_0_78 -> u64
        let s_0_81: u64 = (s_0_78 as u64);
        // C s_0_82: const #1u : u64
        let s_0_82: u64 = 1;
        // D s_0_83: and s_0_81 s_0_82
        let s_0_83: u64 = ((s_0_81) & (s_0_82));
        // D s_0_84: cmp-eq s_0_83 s_0_82
        let s_0_84: bool = ((s_0_83) == (s_0_82));
        // D s_0_85: lsl s_0_81 s_0_79
        let s_0_85: u64 = s_0_81 << s_0_79;
        // D s_0_86: or s_0_80 s_0_85
        let s_0_86: u64 = ((s_0_80) | (s_0_85));
        // D s_0_87: cmpl s_0_85
        let s_0_87: u64 = !s_0_85;
        // D s_0_88: and s_0_80 s_0_87
        let s_0_88: u64 = ((s_0_80) & (s_0_87));
        // D s_0_89: select s_0_84 s_0_86 s_0_88
        let s_0_89: u64 = if s_0_84 { s_0_86 } else { s_0_88 };
        // D s_0_90: cast trunc s_0_89 -> u8
        let s_0_90: bool = ((s_0_89) != 0);
        // D s_0_91: call Bit(s_0_90)
        let s_0_91: bool = Bit(state, tracer, s_0_90);
        // C s_0_92: const #1s : i
        let s_0_92: i128 = 1;
        // D s_0_93: cast zx s_0_71 -> bv
        let s_0_93: Bits = Bits::new(s_0_71 as u128, 4u16);
        // C s_0_94: const #1u : u64
        let s_0_94: u64 = 1;
        // D s_0_95: bit-insert s_0_93 s_0_93 s_0_92 s_0_94
        let s_0_95: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_94 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_93.length(),
            );
            (s_0_93 & mask) | (s_0_93 << s_0_92)
        };
        // D s_0_96: cast reint s_0_95 -> u8
        let s_0_96: u8 = (s_0_95.value() as u8);
        // D s_0_97: write-var outcell <= s_0_96
        fn_state.outcell = s_0_96;
        // C s_0_98: const #1s : i
        let s_0_98: i128 = 1;
        // D s_0_99: read-var incell_name:u8
        let s_0_99: u8 = fn_state.incell_name;
        // D s_0_100: cast zx s_0_99 -> bv
        let s_0_100: Bits = Bits::new(s_0_99 as u128, 4u16);
        // C s_0_101: const #1u : u64
        let s_0_101: u64 = 1;
        // D s_0_102: bit-extract s_0_100 s_0_98 s_0_101
        let s_0_102: Bits = (Bits::new(
            ((s_0_100) >> (s_0_98)).value(),
            u16::try_from(s_0_101).unwrap(),
        ));
        // D s_0_103: cast reint s_0_102 -> u8
        let s_0_103: bool = ((s_0_102.value()) != 0);
        // C s_0_104: const #0s : i
        let s_0_104: i128 = 0;
        // C s_0_105: const #0u : u64
        let s_0_105: u64 = 0;
        // D s_0_106: cast zx s_0_103 -> u64
        let s_0_106: u64 = (s_0_103 as u64);
        // C s_0_107: const #1u : u64
        let s_0_107: u64 = 1;
        // D s_0_108: and s_0_106 s_0_107
        let s_0_108: u64 = ((s_0_106) & (s_0_107));
        // D s_0_109: cmp-eq s_0_108 s_0_107
        let s_0_109: bool = ((s_0_108) == (s_0_107));
        // D s_0_110: lsl s_0_106 s_0_104
        let s_0_110: u64 = s_0_106 << s_0_104;
        // D s_0_111: or s_0_105 s_0_110
        let s_0_111: u64 = ((s_0_105) | (s_0_110));
        // D s_0_112: cmpl s_0_110
        let s_0_112: u64 = !s_0_110;
        // D s_0_113: and s_0_105 s_0_112
        let s_0_113: u64 = ((s_0_105) & (s_0_112));
        // D s_0_114: select s_0_109 s_0_111 s_0_113
        let s_0_114: u64 = if s_0_109 { s_0_111 } else { s_0_113 };
        // D s_0_115: cast trunc s_0_114 -> u8
        let s_0_115: bool = ((s_0_114) != 0);
        // D s_0_116: call Bit(s_0_115)
        let s_0_116: bool = Bit(state, tracer, s_0_115);
        // C s_0_117: const #0s : i
        let s_0_117: i128 = 0;
        // D s_0_118: cast zx s_0_96 -> bv
        let s_0_118: Bits = Bits::new(s_0_96 as u128, 4u16);
        // C s_0_119: const #1u : u64
        let s_0_119: u64 = 1;
        // D s_0_120: bit-insert s_0_118 s_0_118 s_0_117 s_0_119
        let s_0_120: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_119 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_118.length(),
            );
            (s_0_118 & mask) | (s_0_118 << s_0_117)
        };
        // D s_0_121: cast reint s_0_120 -> u8
        let s_0_121: u8 = (s_0_120.value() as u8);
        // D s_0_122: write-var outcell <= s_0_121
        fn_state.outcell = s_0_121;
        // N s_0_123: return s_0_121
        return s_0_121;
    }
}
