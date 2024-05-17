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
use R_read::*;
use R_set::*;
use ROR::*;
use common::*;
pub fn execute_aarch32_instrs_UXTAB16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    rotation: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        n: i64,
        rotation: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        rotation,
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
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var rotation:i64
        let s_0_4: i64 = fn_state.rotation;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call ROR(s_0_3, s_0_5)
        let s_0_6: Bits = ROR(state, tracer, s_0_3, s_0_5);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: read-var d:i64
        let s_0_8: i64 = fn_state.d;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: call R_read(s_0_9)
        let s_0_10: u32 = R_read(state, tracer, s_0_9);
        // D s_0_11: read-var n:i64
        let s_0_11: i64 = fn_state.n;
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: call R_read(s_0_12)
        let s_0_13: u32 = R_read(state, tracer, s_0_12);
        // C s_0_14: const #0s : i
        let s_0_14: i128 = 0;
        // D s_0_15: cast zx s_0_13 -> bv
        let s_0_15: Bits = Bits::new(s_0_13 as u128, 32u16);
        // C s_0_16: const #1s : i64
        let s_0_16: i64 = 1;
        // C s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (i128::try_from(s_0_16).unwrap());
        // C s_0_18: const #15s : i
        let s_0_18: i128 = 15;
        // C s_0_19: add s_0_18 s_0_17
        let s_0_19: i128 = (s_0_18 + s_0_17);
        // D s_0_20: bit-extract s_0_15 s_0_14 s_0_19
        let s_0_20: Bits = (Bits::new(
            ((s_0_15) >> (s_0_14)).value(),
            u16::try_from(s_0_19).unwrap(),
        ));
        // D s_0_21: cast reint s_0_20 -> u16
        let s_0_21: u16 = (s_0_20.value() as u16);
        // C s_0_22: const #0s : i
        let s_0_22: i128 = 0;
        // D s_0_23: cast zx s_0_7 -> bv
        let s_0_23: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_24: const #1s : i64
        let s_0_24: i64 = 1;
        // C s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // C s_0_26: const #7s : i
        let s_0_26: i128 = 7;
        // C s_0_27: add s_0_26 s_0_25
        let s_0_27: i128 = (s_0_26 + s_0_25);
        // D s_0_28: bit-extract s_0_23 s_0_22 s_0_27
        let s_0_28: Bits = (Bits::new(
            ((s_0_23) >> (s_0_22)).value(),
            u16::try_from(s_0_27).unwrap(),
        ));
        // D s_0_29: cast reint s_0_28 -> u8
        let s_0_29: u8 = (s_0_28.value() as u8);
        // C s_0_30: const #16s : i
        let s_0_30: i128 = 16;
        // D s_0_31: cast zx s_0_29 -> bv
        let s_0_31: Bits = Bits::new(s_0_29 as u128, 8u16);
        // D s_0_32: bits-cast zx s_0_31 -> bv length s_0_30
        let s_0_32: Bits = s_0_31.zero_extend(s_0_30);
        // D s_0_33: cast reint s_0_32 -> u16
        let s_0_33: u16 = (s_0_32.value() as u16);
        // D s_0_34: cast zx s_0_21 -> bv
        let s_0_34: Bits = Bits::new(s_0_21 as u128, 16u16);
        // D s_0_35: cast zx s_0_33 -> bv
        let s_0_35: Bits = Bits::new(s_0_33 as u128, 16u16);
        // D s_0_36: add s_0_34 s_0_35
        let s_0_36: Bits = (s_0_34 + s_0_35);
        // D s_0_37: cast reint s_0_36 -> u16
        let s_0_37: u16 = (s_0_36.value() as u16);
        // C s_0_38: const #0s : i
        let s_0_38: i128 = 0;
        // D s_0_39: cast zx s_0_10 -> bv
        let s_0_39: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_40: cast zx s_0_37 -> bv
        let s_0_40: Bits = Bits::new(s_0_37 as u128, 16u16);
        // C s_0_41: const #15s : i
        let s_0_41: i128 = 15;
        // C s_0_42: const #1u : u64
        let s_0_42: u64 = 1;
        // C s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 64u16);
        // C s_0_44: lsl s_0_43 s_0_41
        let s_0_44: Bits = s_0_43 << s_0_41;
        // C s_0_45: sub s_0_44 s_0_43
        let s_0_45: Bits = ((s_0_44) - (s_0_43));
        // D s_0_46: and s_0_40 s_0_45
        let s_0_46: Bits = ((s_0_40) & (s_0_45));
        // D s_0_47: lsl s_0_46 s_0_38
        let s_0_47: Bits = s_0_46 << s_0_38;
        // C s_0_48: lsl s_0_45 s_0_38
        let s_0_48: Bits = s_0_45 << s_0_38;
        // C s_0_49: cmpl s_0_48
        let s_0_49: Bits = !s_0_48;
        // D s_0_50: and s_0_39 s_0_49
        let s_0_50: Bits = ((s_0_39) & (s_0_49));
        // D s_0_51: or s_0_50 s_0_47
        let s_0_51: Bits = ((s_0_50) | (s_0_47));
        // D s_0_52: cast reint s_0_51 -> u32
        let s_0_52: u32 = (s_0_51.value() as u32);
        // D s_0_53: read-var d:i64
        let s_0_53: i64 = fn_state.d;
        // D s_0_54: cast zx s_0_53 -> i
        let s_0_54: i128 = (i128::try_from(s_0_53).unwrap());
        // D s_0_55: call R_set(s_0_54, s_0_52)
        let s_0_55: () = R_set(state, tracer, s_0_54, s_0_52);
        // D s_0_56: read-var d:i64
        let s_0_56: i64 = fn_state.d;
        // D s_0_57: cast zx s_0_56 -> i
        let s_0_57: i128 = (i128::try_from(s_0_56).unwrap());
        // D s_0_58: call R_read(s_0_57)
        let s_0_58: u32 = R_read(state, tracer, s_0_57);
        // D s_0_59: read-var n:i64
        let s_0_59: i64 = fn_state.n;
        // D s_0_60: cast zx s_0_59 -> i
        let s_0_60: i128 = (i128::try_from(s_0_59).unwrap());
        // D s_0_61: call R_read(s_0_60)
        let s_0_61: u32 = R_read(state, tracer, s_0_60);
        // C s_0_62: const #16s : i
        let s_0_62: i128 = 16;
        // D s_0_63: cast zx s_0_61 -> bv
        let s_0_63: Bits = Bits::new(s_0_61 as u128, 32u16);
        // C s_0_64: const #1s : i64
        let s_0_64: i64 = 1;
        // C s_0_65: cast zx s_0_64 -> i
        let s_0_65: i128 = (i128::try_from(s_0_64).unwrap());
        // C s_0_66: const #15s : i
        let s_0_66: i128 = 15;
        // C s_0_67: add s_0_66 s_0_65
        let s_0_67: i128 = (s_0_66 + s_0_65);
        // D s_0_68: bit-extract s_0_63 s_0_62 s_0_67
        let s_0_68: Bits = (Bits::new(
            ((s_0_63) >> (s_0_62)).value(),
            u16::try_from(s_0_67).unwrap(),
        ));
        // D s_0_69: cast reint s_0_68 -> u16
        let s_0_69: u16 = (s_0_68.value() as u16);
        // C s_0_70: const #16s : i
        let s_0_70: i128 = 16;
        // D s_0_71: cast zx s_0_7 -> bv
        let s_0_71: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_72: const #1s : i64
        let s_0_72: i64 = 1;
        // C s_0_73: cast zx s_0_72 -> i
        let s_0_73: i128 = (i128::try_from(s_0_72).unwrap());
        // C s_0_74: const #7s : i
        let s_0_74: i128 = 7;
        // C s_0_75: add s_0_74 s_0_73
        let s_0_75: i128 = (s_0_74 + s_0_73);
        // D s_0_76: bit-extract s_0_71 s_0_70 s_0_75
        let s_0_76: Bits = (Bits::new(
            ((s_0_71) >> (s_0_70)).value(),
            u16::try_from(s_0_75).unwrap(),
        ));
        // D s_0_77: cast reint s_0_76 -> u8
        let s_0_77: u8 = (s_0_76.value() as u8);
        // C s_0_78: const #16s : i
        let s_0_78: i128 = 16;
        // D s_0_79: cast zx s_0_77 -> bv
        let s_0_79: Bits = Bits::new(s_0_77 as u128, 8u16);
        // D s_0_80: bits-cast zx s_0_79 -> bv length s_0_78
        let s_0_80: Bits = s_0_79.zero_extend(s_0_78);
        // D s_0_81: cast reint s_0_80 -> u16
        let s_0_81: u16 = (s_0_80.value() as u16);
        // D s_0_82: cast zx s_0_69 -> bv
        let s_0_82: Bits = Bits::new(s_0_69 as u128, 16u16);
        // D s_0_83: cast zx s_0_81 -> bv
        let s_0_83: Bits = Bits::new(s_0_81 as u128, 16u16);
        // D s_0_84: add s_0_82 s_0_83
        let s_0_84: Bits = (s_0_82 + s_0_83);
        // D s_0_85: cast reint s_0_84 -> u16
        let s_0_85: u16 = (s_0_84.value() as u16);
        // C s_0_86: const #16s : i
        let s_0_86: i128 = 16;
        // D s_0_87: cast zx s_0_58 -> bv
        let s_0_87: Bits = Bits::new(s_0_58 as u128, 32u16);
        // D s_0_88: cast zx s_0_85 -> bv
        let s_0_88: Bits = Bits::new(s_0_85 as u128, 16u16);
        // C s_0_89: const #15s : i
        let s_0_89: i128 = 15;
        // C s_0_90: const #1u : u64
        let s_0_90: u64 = 1;
        // C s_0_91: cast zx s_0_90 -> bv
        let s_0_91: Bits = Bits::new(s_0_90 as u128, 64u16);
        // C s_0_92: lsl s_0_91 s_0_89
        let s_0_92: Bits = s_0_91 << s_0_89;
        // C s_0_93: sub s_0_92 s_0_91
        let s_0_93: Bits = ((s_0_92) - (s_0_91));
        // D s_0_94: and s_0_88 s_0_93
        let s_0_94: Bits = ((s_0_88) & (s_0_93));
        // D s_0_95: lsl s_0_94 s_0_86
        let s_0_95: Bits = s_0_94 << s_0_86;
        // C s_0_96: lsl s_0_93 s_0_86
        let s_0_96: Bits = s_0_93 << s_0_86;
        // C s_0_97: cmpl s_0_96
        let s_0_97: Bits = !s_0_96;
        // D s_0_98: and s_0_87 s_0_97
        let s_0_98: Bits = ((s_0_87) & (s_0_97));
        // D s_0_99: or s_0_98 s_0_95
        let s_0_99: Bits = ((s_0_98) | (s_0_95));
        // D s_0_100: cast reint s_0_99 -> u32
        let s_0_100: u32 = (s_0_99.value() as u32);
        // D s_0_101: read-var d:i64
        let s_0_101: i64 = fn_state.d;
        // D s_0_102: cast zx s_0_101 -> i
        let s_0_102: i128 = (i128::try_from(s_0_101).unwrap());
        // D s_0_103: call R_set(s_0_102, s_0_100)
        let s_0_103: () = R_set(state, tracer, s_0_102, s_0_100);
        // N s_0_104: return
        return;
    }
}
