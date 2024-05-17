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
use R_set::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_REV_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u32,
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
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // C s_0_11: const #24s : i
        let s_0_11: i128 = 24;
        // D s_0_12: read-var result:u32
        let s_0_12: u32 = fn_state.result;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 32u16);
        // D s_0_14: cast zx s_0_10 -> bv
        let s_0_14: Bits = Bits::new(s_0_10 as u128, 8u16);
        // C s_0_15: const #7s : i
        let s_0_15: i128 = 7;
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_15
        let s_0_18: Bits = s_0_17 << s_0_15;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_14 s_0_19
        let s_0_20: Bits = ((s_0_14) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_11
        let s_0_21: Bits = s_0_20 << s_0_11;
        // C s_0_22: lsl s_0_19 s_0_11
        let s_0_22: Bits = s_0_19 << s_0_11;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_13 s_0_23
        let s_0_24: Bits = ((s_0_13) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: write-var result <= s_0_26
        fn_state.result = s_0_26;
        // D s_0_28: read-var m:i64
        let s_0_28: i64 = fn_state.m;
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_30: call R_read(s_0_29)
        let s_0_30: u32 = R_read(state, tracer, s_0_29);
        // C s_0_31: const #8s : i
        let s_0_31: i128 = 8;
        // D s_0_32: cast zx s_0_30 -> bv
        let s_0_32: Bits = Bits::new(s_0_30 as u128, 32u16);
        // C s_0_33: const #1s : i64
        let s_0_33: i64 = 1;
        // C s_0_34: cast zx s_0_33 -> i
        let s_0_34: i128 = (i128::try_from(s_0_33).unwrap());
        // C s_0_35: const #7s : i
        let s_0_35: i128 = 7;
        // C s_0_36: add s_0_35 s_0_34
        let s_0_36: i128 = (s_0_35 + s_0_34);
        // D s_0_37: bit-extract s_0_32 s_0_31 s_0_36
        let s_0_37: Bits = (Bits::new(
            ((s_0_32) >> (s_0_31)).value(),
            u16::try_from(s_0_36).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u8
        let s_0_38: u8 = (s_0_37.value() as u8);
        // C s_0_39: const #16s : i
        let s_0_39: i128 = 16;
        // D s_0_40: cast zx s_0_26 -> bv
        let s_0_40: Bits = Bits::new(s_0_26 as u128, 32u16);
        // D s_0_41: cast zx s_0_38 -> bv
        let s_0_41: Bits = Bits::new(s_0_38 as u128, 8u16);
        // C s_0_42: const #7s : i
        let s_0_42: i128 = 7;
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // C s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: lsl s_0_44 s_0_42
        let s_0_45: Bits = s_0_44 << s_0_42;
        // C s_0_46: sub s_0_45 s_0_44
        let s_0_46: Bits = ((s_0_45) - (s_0_44));
        // D s_0_47: and s_0_41 s_0_46
        let s_0_47: Bits = ((s_0_41) & (s_0_46));
        // D s_0_48: lsl s_0_47 s_0_39
        let s_0_48: Bits = s_0_47 << s_0_39;
        // C s_0_49: lsl s_0_46 s_0_39
        let s_0_49: Bits = s_0_46 << s_0_39;
        // C s_0_50: cmpl s_0_49
        let s_0_50: Bits = !s_0_49;
        // D s_0_51: and s_0_40 s_0_50
        let s_0_51: Bits = ((s_0_40) & (s_0_50));
        // D s_0_52: or s_0_51 s_0_48
        let s_0_52: Bits = ((s_0_51) | (s_0_48));
        // D s_0_53: cast reint s_0_52 -> u32
        let s_0_53: u32 = (s_0_52.value() as u32);
        // D s_0_54: write-var result <= s_0_53
        fn_state.result = s_0_53;
        // D s_0_55: read-var m:i64
        let s_0_55: i64 = fn_state.m;
        // D s_0_56: cast zx s_0_55 -> i
        let s_0_56: i128 = (i128::try_from(s_0_55).unwrap());
        // D s_0_57: call R_read(s_0_56)
        let s_0_57: u32 = R_read(state, tracer, s_0_56);
        // C s_0_58: const #16s : i
        let s_0_58: i128 = 16;
        // D s_0_59: cast zx s_0_57 -> bv
        let s_0_59: Bits = Bits::new(s_0_57 as u128, 32u16);
        // C s_0_60: const #1s : i64
        let s_0_60: i64 = 1;
        // C s_0_61: cast zx s_0_60 -> i
        let s_0_61: i128 = (i128::try_from(s_0_60).unwrap());
        // C s_0_62: const #7s : i
        let s_0_62: i128 = 7;
        // C s_0_63: add s_0_62 s_0_61
        let s_0_63: i128 = (s_0_62 + s_0_61);
        // D s_0_64: bit-extract s_0_59 s_0_58 s_0_63
        let s_0_64: Bits = (Bits::new(
            ((s_0_59) >> (s_0_58)).value(),
            u16::try_from(s_0_63).unwrap(),
        ));
        // D s_0_65: cast reint s_0_64 -> u8
        let s_0_65: u8 = (s_0_64.value() as u8);
        // C s_0_66: const #8s : i
        let s_0_66: i128 = 8;
        // D s_0_67: cast zx s_0_53 -> bv
        let s_0_67: Bits = Bits::new(s_0_53 as u128, 32u16);
        // D s_0_68: cast zx s_0_65 -> bv
        let s_0_68: Bits = Bits::new(s_0_65 as u128, 8u16);
        // C s_0_69: const #7s : i
        let s_0_69: i128 = 7;
        // C s_0_70: const #1u : u64
        let s_0_70: u64 = 1;
        // C s_0_71: cast zx s_0_70 -> bv
        let s_0_71: Bits = Bits::new(s_0_70 as u128, 64u16);
        // C s_0_72: lsl s_0_71 s_0_69
        let s_0_72: Bits = s_0_71 << s_0_69;
        // C s_0_73: sub s_0_72 s_0_71
        let s_0_73: Bits = ((s_0_72) - (s_0_71));
        // D s_0_74: and s_0_68 s_0_73
        let s_0_74: Bits = ((s_0_68) & (s_0_73));
        // D s_0_75: lsl s_0_74 s_0_66
        let s_0_75: Bits = s_0_74 << s_0_66;
        // C s_0_76: lsl s_0_73 s_0_66
        let s_0_76: Bits = s_0_73 << s_0_66;
        // C s_0_77: cmpl s_0_76
        let s_0_77: Bits = !s_0_76;
        // D s_0_78: and s_0_67 s_0_77
        let s_0_78: Bits = ((s_0_67) & (s_0_77));
        // D s_0_79: or s_0_78 s_0_75
        let s_0_79: Bits = ((s_0_78) | (s_0_75));
        // D s_0_80: cast reint s_0_79 -> u32
        let s_0_80: u32 = (s_0_79.value() as u32);
        // D s_0_81: write-var result <= s_0_80
        fn_state.result = s_0_80;
        // D s_0_82: read-var m:i64
        let s_0_82: i64 = fn_state.m;
        // D s_0_83: cast zx s_0_82 -> i
        let s_0_83: i128 = (i128::try_from(s_0_82).unwrap());
        // D s_0_84: call R_read(s_0_83)
        let s_0_84: u32 = R_read(state, tracer, s_0_83);
        // C s_0_85: const #24s : i
        let s_0_85: i128 = 24;
        // D s_0_86: cast zx s_0_84 -> bv
        let s_0_86: Bits = Bits::new(s_0_84 as u128, 32u16);
        // C s_0_87: const #1s : i64
        let s_0_87: i64 = 1;
        // C s_0_88: cast zx s_0_87 -> i
        let s_0_88: i128 = (i128::try_from(s_0_87).unwrap());
        // C s_0_89: const #7s : i
        let s_0_89: i128 = 7;
        // C s_0_90: add s_0_89 s_0_88
        let s_0_90: i128 = (s_0_89 + s_0_88);
        // D s_0_91: bit-extract s_0_86 s_0_85 s_0_90
        let s_0_91: Bits = (Bits::new(
            ((s_0_86) >> (s_0_85)).value(),
            u16::try_from(s_0_90).unwrap(),
        ));
        // D s_0_92: cast reint s_0_91 -> u8
        let s_0_92: u8 = (s_0_91.value() as u8);
        // C s_0_93: const #0s : i
        let s_0_93: i128 = 0;
        // D s_0_94: cast zx s_0_80 -> bv
        let s_0_94: Bits = Bits::new(s_0_80 as u128, 32u16);
        // D s_0_95: cast zx s_0_92 -> bv
        let s_0_95: Bits = Bits::new(s_0_92 as u128, 8u16);
        // C s_0_96: const #7s : i
        let s_0_96: i128 = 7;
        // C s_0_97: const #1u : u64
        let s_0_97: u64 = 1;
        // C s_0_98: cast zx s_0_97 -> bv
        let s_0_98: Bits = Bits::new(s_0_97 as u128, 64u16);
        // C s_0_99: lsl s_0_98 s_0_96
        let s_0_99: Bits = s_0_98 << s_0_96;
        // C s_0_100: sub s_0_99 s_0_98
        let s_0_100: Bits = ((s_0_99) - (s_0_98));
        // D s_0_101: and s_0_95 s_0_100
        let s_0_101: Bits = ((s_0_95) & (s_0_100));
        // D s_0_102: lsl s_0_101 s_0_93
        let s_0_102: Bits = s_0_101 << s_0_93;
        // C s_0_103: lsl s_0_100 s_0_93
        let s_0_103: Bits = s_0_100 << s_0_93;
        // C s_0_104: cmpl s_0_103
        let s_0_104: Bits = !s_0_103;
        // D s_0_105: and s_0_94 s_0_104
        let s_0_105: Bits = ((s_0_94) & (s_0_104));
        // D s_0_106: or s_0_105 s_0_102
        let s_0_106: Bits = ((s_0_105) | (s_0_102));
        // D s_0_107: cast reint s_0_106 -> u32
        let s_0_107: u32 = (s_0_106.value() as u32);
        // D s_0_108: write-var result <= s_0_107
        fn_state.result = s_0_107;
        // D s_0_109: read-var d:i64
        let s_0_109: i64 = fn_state.d;
        // D s_0_110: cast zx s_0_109 -> i
        let s_0_110: i128 = (i128::try_from(s_0_109).unwrap());
        // D s_0_111: call R_set(s_0_110, s_0_107)
        let s_0_111: () = R_set(state, tracer, s_0_110, s_0_107);
        // N s_0_112: return
        return;
    }
}
