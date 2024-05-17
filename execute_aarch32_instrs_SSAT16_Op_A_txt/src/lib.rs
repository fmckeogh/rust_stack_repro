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
use SignedSatQ::*;
use common::*;
pub fn execute_aarch32_instrs_SSAT16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    saturate_to: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_347358: ProductTypef506aa96a892fe52,
        gs_301812: bool,
        ga_347363: ProductTypef506aa96a892fe52,
        sat2: bool,
        d: i64,
        n: i64,
        saturate_to: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
        saturate_to,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var saturate_to:i64
        let s_0_0: i64 = fn_state.saturate_to;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: read-var n:i64
        let s_0_3: i64 = fn_state.n;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: call R_read(s_0_4)
        let s_0_5: u32 = R_read(state, tracer, s_0_4);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // D s_0_7: cast zx s_0_5 -> bv
        let s_0_7: Bits = Bits::new(s_0_5 as u128, 32u16);
        // C s_0_8: const #1s : i64
        let s_0_8: i64 = 1;
        // C s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // C s_0_10: const #15s : i
        let s_0_10: i128 = 15;
        // C s_0_11: add s_0_10 s_0_9
        let s_0_11: i128 = (s_0_10 + s_0_9);
        // D s_0_12: bit-extract s_0_7 s_0_6 s_0_11
        let s_0_12: Bits = (Bits::new(
            ((s_0_7) >> (s_0_6)).value(),
            u16::try_from(s_0_11).unwrap(),
        ));
        // D s_0_13: cast reint s_0_12 -> u16
        let s_0_13: u16 = (s_0_12.value() as u16);
        // D s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 16u16);
        // D s_0_15: cast sx s_0_14 -> i
        let s_0_15: i128 = {
            let sign_bit = s_0_14.length() - 1;
            let mut result = s_0_14.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // D s_0_17: cast zx s_0_2 -> i
        let s_0_17: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // D s_0_19: cast zx s_0_16 -> i
        let s_0_19: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_20: cast zx s_0_18 -> i
        let s_0_20: i128 = (i128::try_from(s_0_18).unwrap());
        // D s_0_21: call SignedSatQ(s_0_19, s_0_20)
        let s_0_21: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_0_19,
            s_0_20,
        );
        // D s_0_22: write-var ga#347358 <= s_0_21
        fn_state.ga_347358 = s_0_21;
        // D s_0_23: read-var ga#347358.0:struct
        let s_0_23: Bits = fn_state.ga_347358._0;
        // D s_0_24: read-var ga#347358.1:struct
        let s_0_24: bool = fn_state.ga_347358._1;
        // D s_0_25: read-var n:i64
        let s_0_25: i64 = fn_state.n;
        // D s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (i128::try_from(s_0_25).unwrap());
        // D s_0_27: call R_read(s_0_26)
        let s_0_27: u32 = R_read(state, tracer, s_0_26);
        // C s_0_28: const #16s : i
        let s_0_28: i128 = 16;
        // D s_0_29: cast zx s_0_27 -> bv
        let s_0_29: Bits = Bits::new(s_0_27 as u128, 32u16);
        // C s_0_30: const #1s : i64
        let s_0_30: i64 = 1;
        // C s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (i128::try_from(s_0_30).unwrap());
        // C s_0_32: const #15s : i
        let s_0_32: i128 = 15;
        // C s_0_33: add s_0_32 s_0_31
        let s_0_33: i128 = (s_0_32 + s_0_31);
        // D s_0_34: bit-extract s_0_29 s_0_28 s_0_33
        let s_0_34: Bits = (Bits::new(
            ((s_0_29) >> (s_0_28)).value(),
            u16::try_from(s_0_33).unwrap(),
        ));
        // D s_0_35: cast reint s_0_34 -> u16
        let s_0_35: u16 = (s_0_34.value() as u16);
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 16u16);
        // D s_0_37: cast sx s_0_36 -> i
        let s_0_37: i128 = {
            let sign_bit = s_0_36.length() - 1;
            let mut result = s_0_36.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_38: cast reint s_0_37 -> i64
        let s_0_38: i64 = (s_0_37 as i64);
        // D s_0_39: cast zx s_0_2 -> i
        let s_0_39: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_40: cast reint s_0_39 -> i64
        let s_0_40: i64 = (s_0_39 as i64);
        // D s_0_41: cast zx s_0_38 -> i
        let s_0_41: i128 = (i128::try_from(s_0_38).unwrap());
        // D s_0_42: cast zx s_0_40 -> i
        let s_0_42: i128 = (i128::try_from(s_0_40).unwrap());
        // D s_0_43: call SignedSatQ(s_0_41, s_0_42)
        let s_0_43: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_0_41,
            s_0_42,
        );
        // D s_0_44: write-var ga#347363 <= s_0_43
        fn_state.ga_347363 = s_0_43;
        // D s_0_45: read-var ga#347363.0:struct
        let s_0_45: Bits = fn_state.ga_347363._0;
        // D s_0_46: read-var ga#347363.1:struct
        let s_0_46: bool = fn_state.ga_347363._1;
        // D s_0_47: write-var sat2 <= s_0_46
        fn_state.sat2 = s_0_46;
        // D s_0_48: read-var d:i64
        let s_0_48: i64 = fn_state.d;
        // D s_0_49: cast zx s_0_48 -> i
        let s_0_49: i128 = (i128::try_from(s_0_48).unwrap());
        // D s_0_50: call R_read(s_0_49)
        let s_0_50: u32 = R_read(state, tracer, s_0_49);
        // C s_0_51: const #16s : i
        let s_0_51: i128 = 16;
        // D s_0_52: bits-cast sx s_0_23 -> bv length s_0_51
        let s_0_52: Bits = s_0_23.sign_extend(s_0_51);
        // D s_0_53: cast reint s_0_52 -> u16
        let s_0_53: u16 = (s_0_52.value() as u16);
        // C s_0_54: const #0s : i
        let s_0_54: i128 = 0;
        // D s_0_55: cast zx s_0_50 -> bv
        let s_0_55: Bits = Bits::new(s_0_50 as u128, 32u16);
        // D s_0_56: cast zx s_0_53 -> bv
        let s_0_56: Bits = Bits::new(s_0_53 as u128, 16u16);
        // C s_0_57: const #15s : i
        let s_0_57: i128 = 15;
        // C s_0_58: const #1u : u64
        let s_0_58: u64 = 1;
        // C s_0_59: cast zx s_0_58 -> bv
        let s_0_59: Bits = Bits::new(s_0_58 as u128, 64u16);
        // C s_0_60: lsl s_0_59 s_0_57
        let s_0_60: Bits = s_0_59 << s_0_57;
        // C s_0_61: sub s_0_60 s_0_59
        let s_0_61: Bits = ((s_0_60) - (s_0_59));
        // D s_0_62: and s_0_56 s_0_61
        let s_0_62: Bits = ((s_0_56) & (s_0_61));
        // D s_0_63: lsl s_0_62 s_0_54
        let s_0_63: Bits = s_0_62 << s_0_54;
        // C s_0_64: lsl s_0_61 s_0_54
        let s_0_64: Bits = s_0_61 << s_0_54;
        // C s_0_65: cmpl s_0_64
        let s_0_65: Bits = !s_0_64;
        // D s_0_66: and s_0_55 s_0_65
        let s_0_66: Bits = ((s_0_55) & (s_0_65));
        // D s_0_67: or s_0_66 s_0_63
        let s_0_67: Bits = ((s_0_66) | (s_0_63));
        // D s_0_68: cast reint s_0_67 -> u32
        let s_0_68: u32 = (s_0_67.value() as u32);
        // D s_0_69: read-var d:i64
        let s_0_69: i64 = fn_state.d;
        // D s_0_70: cast zx s_0_69 -> i
        let s_0_70: i128 = (i128::try_from(s_0_69).unwrap());
        // D s_0_71: call R_set(s_0_70, s_0_68)
        let s_0_71: () = R_set(state, tracer, s_0_70, s_0_68);
        // D s_0_72: read-var d:i64
        let s_0_72: i64 = fn_state.d;
        // D s_0_73: cast zx s_0_72 -> i
        let s_0_73: i128 = (i128::try_from(s_0_72).unwrap());
        // D s_0_74: call R_read(s_0_73)
        let s_0_74: u32 = R_read(state, tracer, s_0_73);
        // C s_0_75: const #16s : i
        let s_0_75: i128 = 16;
        // D s_0_76: bits-cast sx s_0_45 -> bv length s_0_75
        let s_0_76: Bits = s_0_45.sign_extend(s_0_75);
        // D s_0_77: cast reint s_0_76 -> u16
        let s_0_77: u16 = (s_0_76.value() as u16);
        // C s_0_78: const #16s : i
        let s_0_78: i128 = 16;
        // D s_0_79: cast zx s_0_74 -> bv
        let s_0_79: Bits = Bits::new(s_0_74 as u128, 32u16);
        // D s_0_80: cast zx s_0_77 -> bv
        let s_0_80: Bits = Bits::new(s_0_77 as u128, 16u16);
        // C s_0_81: const #15s : i
        let s_0_81: i128 = 15;
        // C s_0_82: const #1u : u64
        let s_0_82: u64 = 1;
        // C s_0_83: cast zx s_0_82 -> bv
        let s_0_83: Bits = Bits::new(s_0_82 as u128, 64u16);
        // C s_0_84: lsl s_0_83 s_0_81
        let s_0_84: Bits = s_0_83 << s_0_81;
        // C s_0_85: sub s_0_84 s_0_83
        let s_0_85: Bits = ((s_0_84) - (s_0_83));
        // D s_0_86: and s_0_80 s_0_85
        let s_0_86: Bits = ((s_0_80) & (s_0_85));
        // D s_0_87: lsl s_0_86 s_0_78
        let s_0_87: Bits = s_0_86 << s_0_78;
        // C s_0_88: lsl s_0_85 s_0_78
        let s_0_88: Bits = s_0_85 << s_0_78;
        // C s_0_89: cmpl s_0_88
        let s_0_89: Bits = !s_0_88;
        // D s_0_90: and s_0_79 s_0_89
        let s_0_90: Bits = ((s_0_79) & (s_0_89));
        // D s_0_91: or s_0_90 s_0_87
        let s_0_91: Bits = ((s_0_90) | (s_0_87));
        // D s_0_92: cast reint s_0_91 -> u32
        let s_0_92: u32 = (s_0_91.value() as u32);
        // D s_0_93: read-var d:i64
        let s_0_93: i64 = fn_state.d;
        // D s_0_94: cast zx s_0_93 -> i
        let s_0_94: i128 = (i128::try_from(s_0_93).unwrap());
        // D s_0_95: call R_set(s_0_94, s_0_92)
        let s_0_95: () = R_set(state, tracer, s_0_94, s_0_92);
        // N s_0_96: branch s_0_24 b5 b1
        if s_0_24 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sat2:u8
        let s_1_0: bool = fn_state.sat2;
        // D s_1_1: write-var gs#301812 <= s_1_0
        fn_state.gs_301812 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#301812:u8
        let s_2_0: bool = fn_state.gs_301812;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #16988u : u32
        let s_4_1: u32 = 16988;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#301812 <= s_5_0
        fn_state.gs_301812 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
