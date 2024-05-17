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
use u_get_RGSR_EL1_Type_SEED::*;
use u_get_GCR_EL1_Type_RRND::*;
use common::*;
pub fn AArch64_NextRandomTagBit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_15758: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_15758: (),
    }
    let fn_state = FunctionState {
        gs_15758,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #11480u : u32
        let s_0_0: u32 = 11480;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_GCR_EL1_Type_RRND(s_0_1)
        let s_0_2: bool = u_get_GCR_EL1_Type_RRND(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #20096u : u32
        let s_0_8: u32 = 20096;
        // D s_0_9: read-reg s_0_8:struct
        let s_0_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: call _get_RGSR_EL1_Type_SEED(s_0_9)
        let s_0_10: u64 = u_get_RGSR_EL1_Type_SEED(state, tracer, s_0_9);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 48u16);
        // C s_0_13: const #1s : i64
        let s_0_13: i64 = 1;
        // C s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // C s_0_15: const #15s : i
        let s_0_15: i128 = 15;
        // C s_0_16: add s_0_15 s_0_14
        let s_0_16: i128 = (s_0_15 + s_0_14);
        // D s_0_17: bit-extract s_0_12 s_0_11 s_0_16
        let s_0_17: Bits = (Bits::new(
            ((s_0_12) >> (s_0_11)).value(),
            u16::try_from(s_0_16).unwrap(),
        ));
        // D s_0_18: cast reint s_0_17 -> u16
        let s_0_18: u16 = (s_0_17.value() as u16);
        // C s_0_19: const #5s : i
        let s_0_19: i128 = 5;
        // D s_0_20: cast zx s_0_18 -> bv
        let s_0_20: Bits = Bits::new(s_0_18 as u128, 16u16);
        // C s_0_21: const #1u : u64
        let s_0_21: u64 = 1;
        // D s_0_22: bit-extract s_0_20 s_0_19 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_20) >> (s_0_19)).value(),
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
        // C s_0_36: const #3s : i
        let s_0_36: i128 = 3;
        // D s_0_37: cast zx s_0_18 -> bv
        let s_0_37: Bits = Bits::new(s_0_18 as u128, 16u16);
        // C s_0_38: const #1u : u64
        let s_0_38: u64 = 1;
        // D s_0_39: bit-extract s_0_37 s_0_36 s_0_38
        let s_0_39: Bits = (Bits::new(
            ((s_0_37) >> (s_0_36)).value(),
            u16::try_from(s_0_38).unwrap(),
        ));
        // D s_0_40: cast reint s_0_39 -> u8
        let s_0_40: bool = ((s_0_39.value()) != 0);
        // C s_0_41: const #0s : i
        let s_0_41: i128 = 0;
        // C s_0_42: const #0u : u64
        let s_0_42: u64 = 0;
        // D s_0_43: cast zx s_0_40 -> u64
        let s_0_43: u64 = (s_0_40 as u64);
        // C s_0_44: const #1u : u64
        let s_0_44: u64 = 1;
        // D s_0_45: and s_0_43 s_0_44
        let s_0_45: u64 = ((s_0_43) & (s_0_44));
        // D s_0_46: cmp-eq s_0_45 s_0_44
        let s_0_46: bool = ((s_0_45) == (s_0_44));
        // D s_0_47: lsl s_0_43 s_0_41
        let s_0_47: u64 = s_0_43 << s_0_41;
        // D s_0_48: or s_0_42 s_0_47
        let s_0_48: u64 = ((s_0_42) | (s_0_47));
        // D s_0_49: cmpl s_0_47
        let s_0_49: u64 = !s_0_47;
        // D s_0_50: and s_0_42 s_0_49
        let s_0_50: u64 = ((s_0_42) & (s_0_49));
        // D s_0_51: select s_0_46 s_0_48 s_0_50
        let s_0_51: u64 = if s_0_46 { s_0_48 } else { s_0_50 };
        // D s_0_52: cast trunc s_0_51 -> u8
        let s_0_52: bool = ((s_0_51) != 0);
        // D s_0_53: cast zx s_0_35 -> bv
        let s_0_53: Bits = Bits::new(s_0_35 as u128, 1u16);
        // D s_0_54: cast zx s_0_52 -> bv
        let s_0_54: Bits = Bits::new(s_0_52 as u128, 1u16);
        // D s_0_55: xor s_0_53 s_0_54
        let s_0_55: Bits = ((s_0_53) ^ (s_0_54));
        // D s_0_56: cast reint s_0_55 -> u8
        let s_0_56: bool = ((s_0_55.value()) != 0);
        // C s_0_57: const #2s : i
        let s_0_57: i128 = 2;
        // D s_0_58: cast zx s_0_18 -> bv
        let s_0_58: Bits = Bits::new(s_0_18 as u128, 16u16);
        // C s_0_59: const #1u : u64
        let s_0_59: u64 = 1;
        // D s_0_60: bit-extract s_0_58 s_0_57 s_0_59
        let s_0_60: Bits = (Bits::new(
            ((s_0_58) >> (s_0_57)).value(),
            u16::try_from(s_0_59).unwrap(),
        ));
        // D s_0_61: cast reint s_0_60 -> u8
        let s_0_61: bool = ((s_0_60.value()) != 0);
        // C s_0_62: const #0s : i
        let s_0_62: i128 = 0;
        // C s_0_63: const #0u : u64
        let s_0_63: u64 = 0;
        // D s_0_64: cast zx s_0_61 -> u64
        let s_0_64: u64 = (s_0_61 as u64);
        // C s_0_65: const #1u : u64
        let s_0_65: u64 = 1;
        // D s_0_66: and s_0_64 s_0_65
        let s_0_66: u64 = ((s_0_64) & (s_0_65));
        // D s_0_67: cmp-eq s_0_66 s_0_65
        let s_0_67: bool = ((s_0_66) == (s_0_65));
        // D s_0_68: lsl s_0_64 s_0_62
        let s_0_68: u64 = s_0_64 << s_0_62;
        // D s_0_69: or s_0_63 s_0_68
        let s_0_69: u64 = ((s_0_63) | (s_0_68));
        // D s_0_70: cmpl s_0_68
        let s_0_70: u64 = !s_0_68;
        // D s_0_71: and s_0_63 s_0_70
        let s_0_71: u64 = ((s_0_63) & (s_0_70));
        // D s_0_72: select s_0_67 s_0_69 s_0_71
        let s_0_72: u64 = if s_0_67 { s_0_69 } else { s_0_71 };
        // D s_0_73: cast trunc s_0_72 -> u8
        let s_0_73: bool = ((s_0_72) != 0);
        // D s_0_74: cast zx s_0_56 -> bv
        let s_0_74: Bits = Bits::new(s_0_56 as u128, 1u16);
        // D s_0_75: cast zx s_0_73 -> bv
        let s_0_75: Bits = Bits::new(s_0_73 as u128, 1u16);
        // D s_0_76: xor s_0_74 s_0_75
        let s_0_76: Bits = ((s_0_74) ^ (s_0_75));
        // D s_0_77: cast reint s_0_76 -> u8
        let s_0_77: bool = ((s_0_76.value()) != 0);
        // C s_0_78: const #0s : i
        let s_0_78: i128 = 0;
        // D s_0_79: cast zx s_0_18 -> bv
        let s_0_79: Bits = Bits::new(s_0_18 as u128, 16u16);
        // C s_0_80: const #1u : u64
        let s_0_80: u64 = 1;
        // D s_0_81: bit-extract s_0_79 s_0_78 s_0_80
        let s_0_81: Bits = (Bits::new(
            ((s_0_79) >> (s_0_78)).value(),
            u16::try_from(s_0_80).unwrap(),
        ));
        // D s_0_82: cast reint s_0_81 -> u8
        let s_0_82: bool = ((s_0_81.value()) != 0);
        // C s_0_83: const #0s : i
        let s_0_83: i128 = 0;
        // C s_0_84: const #0u : u64
        let s_0_84: u64 = 0;
        // D s_0_85: cast zx s_0_82 -> u64
        let s_0_85: u64 = (s_0_82 as u64);
        // C s_0_86: const #1u : u64
        let s_0_86: u64 = 1;
        // D s_0_87: and s_0_85 s_0_86
        let s_0_87: u64 = ((s_0_85) & (s_0_86));
        // D s_0_88: cmp-eq s_0_87 s_0_86
        let s_0_88: bool = ((s_0_87) == (s_0_86));
        // D s_0_89: lsl s_0_85 s_0_83
        let s_0_89: u64 = s_0_85 << s_0_83;
        // D s_0_90: or s_0_84 s_0_89
        let s_0_90: u64 = ((s_0_84) | (s_0_89));
        // D s_0_91: cmpl s_0_89
        let s_0_91: u64 = !s_0_89;
        // D s_0_92: and s_0_84 s_0_91
        let s_0_92: u64 = ((s_0_84) & (s_0_91));
        // D s_0_93: select s_0_88 s_0_90 s_0_92
        let s_0_93: u64 = if s_0_88 { s_0_90 } else { s_0_92 };
        // D s_0_94: cast trunc s_0_93 -> u8
        let s_0_94: bool = ((s_0_93) != 0);
        // D s_0_95: cast zx s_0_77 -> bv
        let s_0_95: Bits = Bits::new(s_0_77 as u128, 1u16);
        // D s_0_96: cast zx s_0_94 -> bv
        let s_0_96: Bits = Bits::new(s_0_94 as u128, 1u16);
        // D s_0_97: xor s_0_95 s_0_96
        let s_0_97: Bits = ((s_0_95) ^ (s_0_96));
        // D s_0_98: cast reint s_0_97 -> u8
        let s_0_98: bool = ((s_0_97.value()) != 0);
        // C s_0_99: const #20096u : u32
        let s_0_99: u32 = 20096;
        // D s_0_100: read-reg s_0_99:struct
        let s_0_100: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_99 as isize);
            tracer.read_register(s_0_99 as isize, value);
            value
        };
        // C s_0_101: const #20096u : u32
        let s_0_101: u32 = 20096;
        // N s_0_102: write-reg s_0_101 <= s_0_100
        let s_0_102: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_0_101 as isize, s_0_100);
            tracer.write_register(s_0_101 as isize, s_0_100);
        };
        // N s_0_103: return s_0_98
        return s_0_98;
    }
}
