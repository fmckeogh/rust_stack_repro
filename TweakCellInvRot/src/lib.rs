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
pub fn TweakCellInvRot<T: Tracer>(state: &mut State, tracer: &T, incell_name: u8) -> u8 {
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
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
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
        // D s_0_18: call Bit(s_0_17)
        let s_0_18: bool = Bit(state, tracer, s_0_17);
        // C s_0_19: const #3s : i
        let s_0_19: i128 = 3;
        // D s_0_20: read-var outcell:u8
        let s_0_20: u8 = fn_state.outcell;
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 4u16);
        // C s_0_22: const #1u : u64
        let s_0_22: u64 = 1;
        // D s_0_23: bit-insert s_0_21 s_0_21 s_0_19 s_0_22
        let s_0_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_21.length(),
            );
            (s_0_21 & mask) | (s_0_21 << s_0_19)
        };
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: u8 = (s_0_23.value() as u8);
        // D s_0_25: write-var outcell <= s_0_24
        fn_state.outcell = s_0_24;
        // C s_0_26: const #1s : i
        let s_0_26: i128 = 1;
        // D s_0_27: read-var incell_name:u8
        let s_0_27: u8 = fn_state.incell_name;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 4u16);
        // C s_0_29: const #1u : u64
        let s_0_29: u64 = 1;
        // D s_0_30: bit-extract s_0_28 s_0_26 s_0_29
        let s_0_30: Bits = (Bits::new(
            ((s_0_28) >> (s_0_26)).value(),
            u16::try_from(s_0_29).unwrap(),
        ));
        // D s_0_31: cast reint s_0_30 -> u8
        let s_0_31: bool = ((s_0_30.value()) != 0);
        // C s_0_32: const #0s : i
        let s_0_32: i128 = 0;
        // C s_0_33: const #0u : u64
        let s_0_33: u64 = 0;
        // D s_0_34: cast zx s_0_31 -> u64
        let s_0_34: u64 = (s_0_31 as u64);
        // C s_0_35: const #1u : u64
        let s_0_35: u64 = 1;
        // D s_0_36: and s_0_34 s_0_35
        let s_0_36: u64 = ((s_0_34) & (s_0_35));
        // D s_0_37: cmp-eq s_0_36 s_0_35
        let s_0_37: bool = ((s_0_36) == (s_0_35));
        // D s_0_38: lsl s_0_34 s_0_32
        let s_0_38: u64 = s_0_34 << s_0_32;
        // D s_0_39: or s_0_33 s_0_38
        let s_0_39: u64 = ((s_0_33) | (s_0_38));
        // D s_0_40: cmpl s_0_38
        let s_0_40: u64 = !s_0_38;
        // D s_0_41: and s_0_33 s_0_40
        let s_0_41: u64 = ((s_0_33) & (s_0_40));
        // D s_0_42: select s_0_37 s_0_39 s_0_41
        let s_0_42: u64 = if s_0_37 { s_0_39 } else { s_0_41 };
        // D s_0_43: cast trunc s_0_42 -> u8
        let s_0_43: bool = ((s_0_42) != 0);
        // D s_0_44: call Bit(s_0_43)
        let s_0_44: bool = Bit(state, tracer, s_0_43);
        // C s_0_45: const #2s : i
        let s_0_45: i128 = 2;
        // D s_0_46: cast zx s_0_24 -> bv
        let s_0_46: Bits = Bits::new(s_0_24 as u128, 4u16);
        // C s_0_47: const #1u : u64
        let s_0_47: u64 = 1;
        // D s_0_48: bit-insert s_0_46 s_0_46 s_0_45 s_0_47
        let s_0_48: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_47 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_46.length(),
            );
            (s_0_46 & mask) | (s_0_46 << s_0_45)
        };
        // D s_0_49: cast reint s_0_48 -> u8
        let s_0_49: u8 = (s_0_48.value() as u8);
        // D s_0_50: write-var outcell <= s_0_49
        fn_state.outcell = s_0_49;
        // C s_0_51: const #0s : i
        let s_0_51: i128 = 0;
        // D s_0_52: read-var incell_name:u8
        let s_0_52: u8 = fn_state.incell_name;
        // D s_0_53: cast zx s_0_52 -> bv
        let s_0_53: Bits = Bits::new(s_0_52 as u128, 4u16);
        // C s_0_54: const #1u : u64
        let s_0_54: u64 = 1;
        // D s_0_55: bit-extract s_0_53 s_0_51 s_0_54
        let s_0_55: Bits = (Bits::new(
            ((s_0_53) >> (s_0_51)).value(),
            u16::try_from(s_0_54).unwrap(),
        ));
        // D s_0_56: cast reint s_0_55 -> u8
        let s_0_56: bool = ((s_0_55.value()) != 0);
        // C s_0_57: const #0s : i
        let s_0_57: i128 = 0;
        // C s_0_58: const #0u : u64
        let s_0_58: u64 = 0;
        // D s_0_59: cast zx s_0_56 -> u64
        let s_0_59: u64 = (s_0_56 as u64);
        // C s_0_60: const #1u : u64
        let s_0_60: u64 = 1;
        // D s_0_61: and s_0_59 s_0_60
        let s_0_61: u64 = ((s_0_59) & (s_0_60));
        // D s_0_62: cmp-eq s_0_61 s_0_60
        let s_0_62: bool = ((s_0_61) == (s_0_60));
        // D s_0_63: lsl s_0_59 s_0_57
        let s_0_63: u64 = s_0_59 << s_0_57;
        // D s_0_64: or s_0_58 s_0_63
        let s_0_64: u64 = ((s_0_58) | (s_0_63));
        // D s_0_65: cmpl s_0_63
        let s_0_65: u64 = !s_0_63;
        // D s_0_66: and s_0_58 s_0_65
        let s_0_66: u64 = ((s_0_58) & (s_0_65));
        // D s_0_67: select s_0_62 s_0_64 s_0_66
        let s_0_67: u64 = if s_0_62 { s_0_64 } else { s_0_66 };
        // D s_0_68: cast trunc s_0_67 -> u8
        let s_0_68: bool = ((s_0_67) != 0);
        // D s_0_69: call Bit(s_0_68)
        let s_0_69: bool = Bit(state, tracer, s_0_68);
        // C s_0_70: const #1s : i
        let s_0_70: i128 = 1;
        // D s_0_71: cast zx s_0_49 -> bv
        let s_0_71: Bits = Bits::new(s_0_49 as u128, 4u16);
        // C s_0_72: const #1u : u64
        let s_0_72: u64 = 1;
        // D s_0_73: bit-insert s_0_71 s_0_71 s_0_70 s_0_72
        let s_0_73: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_0_72 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_0_71.length(),
            );
            (s_0_71 & mask) | (s_0_71 << s_0_70)
        };
        // D s_0_74: cast reint s_0_73 -> u8
        let s_0_74: u8 = (s_0_73.value() as u8);
        // D s_0_75: write-var outcell <= s_0_74
        fn_state.outcell = s_0_74;
        // C s_0_76: const #0s : i
        let s_0_76: i128 = 0;
        // D s_0_77: read-var incell_name:u8
        let s_0_77: u8 = fn_state.incell_name;
        // D s_0_78: cast zx s_0_77 -> bv
        let s_0_78: Bits = Bits::new(s_0_77 as u128, 4u16);
        // C s_0_79: const #1u : u64
        let s_0_79: u64 = 1;
        // D s_0_80: bit-extract s_0_78 s_0_76 s_0_79
        let s_0_80: Bits = (Bits::new(
            ((s_0_78) >> (s_0_76)).value(),
            u16::try_from(s_0_79).unwrap(),
        ));
        // D s_0_81: cast reint s_0_80 -> u8
        let s_0_81: bool = ((s_0_80.value()) != 0);
        // C s_0_82: const #0s : i
        let s_0_82: i128 = 0;
        // C s_0_83: const #0u : u64
        let s_0_83: u64 = 0;
        // D s_0_84: cast zx s_0_81 -> u64
        let s_0_84: u64 = (s_0_81 as u64);
        // C s_0_85: const #1u : u64
        let s_0_85: u64 = 1;
        // D s_0_86: and s_0_84 s_0_85
        let s_0_86: u64 = ((s_0_84) & (s_0_85));
        // D s_0_87: cmp-eq s_0_86 s_0_85
        let s_0_87: bool = ((s_0_86) == (s_0_85));
        // D s_0_88: lsl s_0_84 s_0_82
        let s_0_88: u64 = s_0_84 << s_0_82;
        // D s_0_89: or s_0_83 s_0_88
        let s_0_89: u64 = ((s_0_83) | (s_0_88));
        // D s_0_90: cmpl s_0_88
        let s_0_90: u64 = !s_0_88;
        // D s_0_91: and s_0_83 s_0_90
        let s_0_91: u64 = ((s_0_83) & (s_0_90));
        // D s_0_92: select s_0_87 s_0_89 s_0_91
        let s_0_92: u64 = if s_0_87 { s_0_89 } else { s_0_91 };
        // D s_0_93: cast trunc s_0_92 -> u8
        let s_0_93: bool = ((s_0_92) != 0);
        // C s_0_94: const #3s : i
        let s_0_94: i128 = 3;
        // D s_0_95: read-var incell_name:u8
        let s_0_95: u8 = fn_state.incell_name;
        // D s_0_96: cast zx s_0_95 -> bv
        let s_0_96: Bits = Bits::new(s_0_95 as u128, 4u16);
        // C s_0_97: const #1u : u64
        let s_0_97: u64 = 1;
        // D s_0_98: bit-extract s_0_96 s_0_94 s_0_97
        let s_0_98: Bits = (Bits::new(
            ((s_0_96) >> (s_0_94)).value(),
            u16::try_from(s_0_97).unwrap(),
        ));
        // D s_0_99: cast reint s_0_98 -> u8
        let s_0_99: bool = ((s_0_98.value()) != 0);
        // C s_0_100: const #0s : i
        let s_0_100: i128 = 0;
        // C s_0_101: const #0u : u64
        let s_0_101: u64 = 0;
        // D s_0_102: cast zx s_0_99 -> u64
        let s_0_102: u64 = (s_0_99 as u64);
        // C s_0_103: const #1u : u64
        let s_0_103: u64 = 1;
        // D s_0_104: and s_0_102 s_0_103
        let s_0_104: u64 = ((s_0_102) & (s_0_103));
        // D s_0_105: cmp-eq s_0_104 s_0_103
        let s_0_105: bool = ((s_0_104) == (s_0_103));
        // D s_0_106: lsl s_0_102 s_0_100
        let s_0_106: u64 = s_0_102 << s_0_100;
        // D s_0_107: or s_0_101 s_0_106
        let s_0_107: u64 = ((s_0_101) | (s_0_106));
        // D s_0_108: cmpl s_0_106
        let s_0_108: u64 = !s_0_106;
        // D s_0_109: and s_0_101 s_0_108
        let s_0_109: u64 = ((s_0_101) & (s_0_108));
        // D s_0_110: select s_0_105 s_0_107 s_0_109
        let s_0_110: u64 = if s_0_105 { s_0_107 } else { s_0_109 };
        // D s_0_111: cast trunc s_0_110 -> u8
        let s_0_111: bool = ((s_0_110) != 0);
        // D s_0_112: cast zx s_0_93 -> bv
        let s_0_112: Bits = Bits::new(s_0_93 as u128, 1u16);
        // D s_0_113: cast zx s_0_111 -> bv
        let s_0_113: Bits = Bits::new(s_0_111 as u128, 1u16);
        // D s_0_114: xor s_0_112 s_0_113
        let s_0_114: Bits = ((s_0_112) ^ (s_0_113));
        // D s_0_115: cast reint s_0_114 -> u8
        let s_0_115: bool = ((s_0_114.value()) != 0);
        // D s_0_116: call Bit(s_0_115)
        let s_0_116: bool = Bit(state, tracer, s_0_115);
        // C s_0_117: const #0s : i
        let s_0_117: i128 = 0;
        // D s_0_118: cast zx s_0_74 -> bv
        let s_0_118: Bits = Bits::new(s_0_74 as u128, 4u16);
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
