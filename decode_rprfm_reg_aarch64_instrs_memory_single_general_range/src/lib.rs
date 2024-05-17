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
use execute_aarch64_instrs_memory_single_general_range::*;
use common::*;
pub fn decode_rprfm_reg_aarch64_instrs_memory_single_general_range<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    S: bool,
    option_name: u8,
    Rm: u8,
    opc: u8,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rt: u8,
        Rn: u8,
        S: bool,
        option_name: u8,
        Rm: u8,
        opc: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        S,
        option_name,
        Rm,
        opc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var option_name:u8
        let s_0_1: u8 = fn_state.option_name;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 3u16);
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
        // C s_0_18: const #0s : i
        let s_0_18: i128 = 0;
        // D s_0_19: read-var option_name:u8
        let s_0_19: u8 = fn_state.option_name;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 3u16);
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
        // D s_0_38: cast reint s_0_36 -> u128
        let s_0_38: u128 = (s_0_36.value() as u128);
        // D s_0_39: size-of s_0_36
        let s_0_39: u16 = s_0_36.length();
        // D s_0_40: cast reint s_0_37 -> u128
        let s_0_40: u128 = (s_0_37.value() as u128);
        // D s_0_41: size-of s_0_37
        let s_0_41: u16 = s_0_37.length();
        // D s_0_42: lsl s_0_38 s_0_41
        let s_0_42: u128 = s_0_38 << s_0_41;
        // D s_0_43: or s_0_42 s_0_40
        let s_0_43: u128 = ((s_0_42) | (s_0_40));
        // D s_0_44: add s_0_39 s_0_41
        let s_0_44: u16 = (s_0_39 + s_0_41);
        // D s_0_45: create-bits s_0_43 s_0_44
        let s_0_45: Bits = Bits::new(s_0_43, s_0_44);
        // D s_0_46: cast reint s_0_45 -> u8
        let s_0_46: u8 = (s_0_45.value() as u8);
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // D s_0_48: read-var S:u8
        let s_0_48: bool = fn_state.S;
        // D s_0_49: cast zx s_0_48 -> bv
        let s_0_49: Bits = Bits::new(s_0_48 as u128, 1u16);
        // D s_0_50: cast reint s_0_47 -> u128
        let s_0_50: u128 = (s_0_47.value() as u128);
        // D s_0_51: size-of s_0_47
        let s_0_51: u16 = s_0_47.length();
        // D s_0_52: cast reint s_0_49 -> u128
        let s_0_52: u128 = (s_0_49.value() as u128);
        // D s_0_53: size-of s_0_49
        let s_0_53: u16 = s_0_49.length();
        // D s_0_54: lsl s_0_50 s_0_53
        let s_0_54: u128 = s_0_50 << s_0_53;
        // D s_0_55: or s_0_54 s_0_52
        let s_0_55: u128 = ((s_0_54) | (s_0_52));
        // D s_0_56: add s_0_51 s_0_53
        let s_0_56: u16 = (s_0_51 + s_0_53);
        // D s_0_57: create-bits s_0_55 s_0_56
        let s_0_57: Bits = Bits::new(s_0_55, s_0_56);
        // D s_0_58: cast reint s_0_57 -> u8
        let s_0_58: u8 = (s_0_57.value() as u8);
        // C s_0_59: const #0s : i
        let s_0_59: i128 = 0;
        // D s_0_60: read-var Rt:u8
        let s_0_60: u8 = fn_state.Rt;
        // D s_0_61: cast zx s_0_60 -> bv
        let s_0_61: Bits = Bits::new(s_0_60 as u128, 5u16);
        // C s_0_62: const #1s : i64
        let s_0_62: i64 = 1;
        // C s_0_63: cast zx s_0_62 -> i
        let s_0_63: i128 = (i128::try_from(s_0_62).unwrap());
        // C s_0_64: const #2s : i
        let s_0_64: i128 = 2;
        // C s_0_65: add s_0_64 s_0_63
        let s_0_65: i128 = (s_0_64 + s_0_63);
        // D s_0_66: bit-extract s_0_61 s_0_59 s_0_65
        let s_0_66: Bits = (Bits::new(
            ((s_0_61) >> (s_0_59)).value(),
            u16::try_from(s_0_65).unwrap(),
        ));
        // D s_0_67: cast reint s_0_66 -> u8
        let s_0_67: u8 = (s_0_66.value() as u8);
        // D s_0_68: cast zx s_0_58 -> bv
        let s_0_68: Bits = Bits::new(s_0_58 as u128, 3u16);
        // D s_0_69: cast zx s_0_67 -> bv
        let s_0_69: Bits = Bits::new(s_0_67 as u128, 3u16);
        // D s_0_70: cast reint s_0_68 -> u128
        let s_0_70: u128 = (s_0_68.value() as u128);
        // D s_0_71: size-of s_0_68
        let s_0_71: u16 = s_0_68.length();
        // D s_0_72: cast reint s_0_69 -> u128
        let s_0_72: u128 = (s_0_69.value() as u128);
        // D s_0_73: size-of s_0_69
        let s_0_73: u16 = s_0_69.length();
        // D s_0_74: lsl s_0_70 s_0_73
        let s_0_74: u128 = s_0_70 << s_0_73;
        // D s_0_75: or s_0_74 s_0_72
        let s_0_75: u128 = ((s_0_74) | (s_0_72));
        // D s_0_76: add s_0_71 s_0_73
        let s_0_76: u16 = (s_0_71 + s_0_73);
        // D s_0_77: create-bits s_0_75 s_0_76
        let s_0_77: Bits = Bits::new(s_0_75, s_0_76);
        // D s_0_78: cast reint s_0_77 -> u8
        let s_0_78: u8 = (s_0_77.value() as u8);
        // D s_0_79: read-var Rn:u8
        let s_0_79: u8 = fn_state.Rn;
        // D s_0_80: cast zx s_0_79 -> bv
        let s_0_80: Bits = Bits::new(s_0_79 as u128, 5u16);
        // D s_0_81: cast zx s_0_80 -> i
        let s_0_81: i128 = (s_0_80.value() as i128);
        // D s_0_82: cast reint s_0_81 -> i64
        let s_0_82: i64 = (s_0_81 as i64);
        // D s_0_83: read-var Rm:u8
        let s_0_83: u8 = fn_state.Rm;
        // D s_0_84: cast zx s_0_83 -> bv
        let s_0_84: Bits = Bits::new(s_0_83 as u128, 5u16);
        // D s_0_85: cast zx s_0_84 -> i
        let s_0_85: i128 = (s_0_84.value() as i128);
        // D s_0_86: cast reint s_0_85 -> i64
        let s_0_86: i64 = (s_0_85 as i64);
        // D s_0_87: call execute_aarch64_instrs_memory_single_general_range(s_0_86, s_0_82, s_0_78)
        let s_0_87: () = execute_aarch64_instrs_memory_single_general_range(
            state,
            tracer,
            s_0_86,
            s_0_82,
            s_0_78,
        );
        // N s_0_88: return
        return;
    }
}
